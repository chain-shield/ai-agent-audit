# Benchmark Ground Truth: Tapioca Invitational 

## Accepted H/M Findings

# Accepted H/M Findings: Tapioca Invitational 

# [H-01] MagnetarMintXChainModule.sol : mintBBLendXChainSGL can be used to manipulate user positions by abusing whitelist privileges

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

MagnetarMintXChainModule.sol:

mintBBLendXChainSGL can be used to manipulate user positions by abusing whitelist privileges Submitted by carrotsmuggler, also found by carrotsmuggler, GalloDaSballo, and cccz ( 1, 2 ) The Magnetar functions use _checkSender function to check if the caller should be allowed to perform operations on the account. The function allows operations if the caller is the owner, or if the caller is a whitelisted trusted address.

function _checkSender ( address _from ) internal view { if ( _from != msg.

sender && !

cluster.

isWhitelisted ( 0, msg.

sender )) { revert Magnetar_NotAuthorized ( msg.

sender, _from ); } However, this means that if a malicious user is able to make a whitelisted contract call magnetar functions with their own payload, they can steal tokens and wreak havoc on other user’s accounts!

The function depositYBLendSGLLockXchainTOLP in the MagnetarAssetXChainModule contract uses a similar check. This function deposits and lends into markets, for the account passed in as data.user. Crucially, it also extracts tokens from data.user for these operations. So if a malicious user was able to get this function called by a whitelisted contract and pass in a malicious data.user, they can cause the target user to lose tokens and manipulate their market positions. This is a high severity issue and the path to attack is demonstrated below.

## Recommended Mitigation Steps

The architecture of this crosschain call is quite vulnerable. Due to the whitelist, any function call that can be done via USDO contract is risky since it can override the Magnetar checks. The mintBBLendXChainSGL function on chainA should make sure the lzcompose data.user is the same as the current data.user, but this only blocks a single attack vector.

USDO contract is crosschain compatible and allows lzcompose message, so any other methods which can be used to trigger such a cross chain call can abuse the whitelist.

0xRektora (Tapioca) confirmed via duplicate Issue #124 cryptotechmaker (Tapioca) commented via duplicate Issue #124:

PR here.

# [H-02] Missing check on helper contract allows arbitrary actions and theft of assets

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by carrotsmuggler, also found by ladboy233 The MagnetarOptionModule contract implements the exitPositionAndRemoveCollateral function which allows users to do a series of operations which is irrelevant to the issue. The user passes in the variable data, and later, data.externalData is used to extract out relevant contract addresses. These are then checked against a whitelist.

if ( data.

externalData.

bigBang != address ( 0 )) { if (!

cluster.

isWhitelisted ( 0, data.

externalData.

bigBang )) { revert Magnetar_TargetNotWhitelisted ( data.

externalData.

bigBang ); } if ( data.

externalData.

singularity != address ( 0 )) { if (!

cluster.

isWhitelisted ( 0, data.

externalData.

singularity )) { revert Magnetar_TargetNotWhitelisted ( data.

externalData.

singularity ); } The main issue is that the data.externalData also has a marketHelper field which is not checked against a whitelist and ends up being used.

( Module [] memory modules, bytes [] memory calls ) = IMarketHelper ( data.

externalData.

marketHelper ).

repay ( address ( this ), data.

user, false, data.

removeAndRepayData.

repayAmount ); ( bool [] memory successes, bytes [] memory results ) = bigBang_.

execute ( modules, calls, true ); The helper contracts are used to construct the calldata for market operations. In the above snippet, the helper contract is passed in some data, and it is expected to create a calldata out of the passed in data. The expected output is the repay module and a call value which when executed, will repay for the data.user ’s account.

However, since the marketHelper contract is never checked against a whitelist, malicious user can pass in any address in that place. So the above call can return any data payload, and the bigBang_.execute will execute it without any checks. This means the malicious helper contract can return a borrow payload of some random user, and the contract will end up borrowing USDO against that user’s position. The Magnetar contract is assumed to have approval for market operations, and thus the Magnetar’s approval is essentially exploited by the attacker to perform arbitrary actions on any user’s account.

This can be used by any user to steal collateral from other user’s bigbang position, or borrow out usdo tokens on their position. Since this is direct theft, this is a high severity issue.

## Recommended Mitigation Steps

Check the helper contract against a whitelist.

cryptotechmaker (Tapioca) disagreed with severity and commented:

Low/Invalid; even if the market helper is not checked (and I agree it’s ok to add that verification) the module which is going to be executed is checked on the BB/SGL side and the action that’s being performed also checks the allowances ladboy233 (warden) commented:

I think the severity is not inflated and the severity is high and the issue clearly leads to theft of fund.

Magnatar is a like a router contract and help user compose multicall.

User calls magnetar function -> delegate calls Option Module.

/// @dev Modules will not return result data.

if ( _action.

id == MagnetarAction.

OptionModule ) { _executeModule ( MagnetarModule.

OptionModule, _action.

call ); continue; // skip the rest of the loop } User needs to give a lot of approve for magnetar contract to allow magnetar contract pull fund out of user’s account to complete transaction.

To prevent abuse of allowance, this check is made in-place.

function exitPositionAndRemoveCollateral ( ExitPositionAndRemoveCollateralData memory data ) public payable { // Check sender _checkSender ( data.

user ); Which calls:

function _checkSender ( address _from ) internal view { if ( _from != msg.

sender && !

cluster.

isWhitelisted ( 0, msg.

sender )) { revert Magnetar_NotAuthorized ( msg.

sender, _from ); } The from != msg.sender is super important, otherwise.

If user A gives allowance to magnetar contract, user B can set data.user to user A and steal fund from user A directly.

Lack of validation of market helper allows malicious actor executes arbitrary multicall. See here.

( Module [] memory modules, bytes [] memory calls ) = IMarketHelper ( data.

externalData.

marketHelper ).

repay ( address ( this ), data.

user, false, data.

removeAndRepayData.

repayAmount ); ( bool [] memory successes, bytes [] memory results ) = bigBang_.

execute ( modules, calls, true ); As for sponsor comments:

The module which is going to be executed is checked on the BB/SGL side and the action that’s being performed also checks the allowances.

This is the code in BBCollateral module:

bigBang_.execute multicall to bigBang module and one of the module is BBCollateral module:

function removeCollateral(address from, address to, uint256 share) external optionNotPaused(PauseType.RemoveCollateral) solvent(from, false) notSelf(to) allowedBorrow(from, share) { _removeCollateral(from, to, share); } The validation that sponsor mentions is in the modifier:

allowedBorrow ( from, share ) Which calls:

function _allowedBorrow ( address from, uint256 share ) internal virtual override { if ( from != msg.

sender ) { // TODO review risk of using this ( uint256 pearlmitAllowed,) = penrose.

pearlmit ().

allowance ( from, msg.

sender, address ( yieldBox ), collateralId ); require ( allowanceBorrow [ from ][ msg.

sender ] >= share || pearlmitAllowed >= share, "Market: not approved" ); if ( allowanceBorrow [ from ][ msg.

sender ] != type ( uint256 ).

max ) { allowanceBorrow [ from ][ msg.

sender ] -= share; } Obviously “from” is not msg.sender, but msg.sender is the magnetar contract that hold user’s allowance.

Protocol fix the lack of market helper validation in the other part of the codebase, see here. The exact same issue should be fixed in Option module as well.

Other way to abuse pending allowance is marked as high severity here.

Abuse this issue is not fixed here.

This type of exploit can occur:

User approves spending allowance to sushi router.

Funds sit idle in users wallet.

Attacker triggers transferFrom from victim address to hacker address -> exploit.

In this case:

User approves spending allowance to magnetar.

Funds sit idle in users wallet.

Attacker bypasses the _checkSender and constructs multicall to remove collateral from user’s account directly.

carrotsmuggler (warden) commented:

This should be valid. According to the sponsor, even if the market helper is not checked the module which is going to be executed is checked on the BB/SGL side.

This is true. However the bigbang/sgl markets do the check on msg.sender, which is the magnetar contract itself, which is expected to have allowance from the users. Checks are not done on the initiator of this transaction. This is highlighted here and below.

function _allowedBorrow ( address from, uint256 share ) internal virtual override { if ( from != msg.

sender ) { if ( share == 0 ) revert AllowanceNotValid (); // TODO review risk of using this ( uint256 pearlmitAllowed,) = penrose.

pearlmit ().

allowance ( from, msg.

sender, address ( yieldBox ), collateralId ); require ( allowanceBorrow [ from ][ msg.

sender ] >= share || pearlmitAllowed >= share, "Market: not approved" ); if ( allowanceBorrow [ from ][ msg.

sender ] != type ( uint256 ).

max ) { allowanceBorrow [ from ][ msg.

sender ] -= share; } Magnetar is a privileged contract, and this function allows other users to abuse this privilege. This is basically approval hijacking, and so is high severity.

0xRektora (Tapioca) commented:

@LSDan, this can be approved as a high risk.

While we switched the model to use “atomic” approvals using Pearlmit, it’s better to be safe than sorry. The reviewed code also still has an obsolete allowanceBorrow which could help initiate this attack.

0xWeiss (Tapioca) confirmed

# [H-03] Absence of restrictions on the sender of the twTAP.claimsReward() function could enable attackers to freeze reward tokens within the Tap token contract

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

twTAP.claimsReward() function could enable attackers to freeze reward tokens within the Tap token contract Submitted by KIntern_NA, also found by KIntern_NA, immeas, carrotsmuggler, ronnyx2017, GalloDaSballo ( 1, 2, 3 ), cccz, and ladboy233

- https://github.com/Tapioca-DAO/tap-token/blob/20a83b1d2d5577653610a6c3879dff9df4968345/contracts/governance/twTAP.sol#L396-L404
The function twTAP.claimRewards() is utilized to claim the reward distributed to the position identified by _tokenId.

function claimRewards ( uint256 _tokenId, address _to ) external nonReentrant whenNotPaused returns ( uint256 [] memory amounts_ ) { _requireClaimPermission ( _to, _tokenId ); amounts_ = _claimRewards ( _tokenId, _to ); } This function can be triggered by anyone, provided that the receiver of the claimed reward _to is either the owner of the position or an address approved by the position’s owner.

In the function TapTokenReceiver._claimTwpTapRewardsReceiver(), the twTAP.claimRewards() function is invoked at line 156 to calculate the reward assigned to _tokenId and claim the reward to this contract before transferring it to the receiver on another chain. To achieve this, the position’s owner must first approve this contract to access the position before executing the function.

function _claimTwpTapRewardsReceiver ( bytes memory _data ) internal virtual twTapExists { ClaimTwTapRewardsMsg memory claimTwTapRewardsMsg_ = TapTokenCodec.

decodeClaimTwTapRewardsMsg ( _data ); uint256 [] memory claimedAmount_ = twTap.

claimRewards ( claimTwTapRewardsMsg_.

tokenId, address ( this ));...

} However, between the call to grant approval to the contract and the execution of the _claimTwpTapRewardsReceiver() function, an attacker can insert a transaction calling twTAP.claimRewards(_tokenId, TapTokenReceiver). By doing so, the rewards will be claimed to the TapTokenReceiver contract before the _claimTwpTapRewardsReceiver() function is invoked. Consequently, the return value of claimedAmount_ = twTap.claimRewards(claimTwTapRewardsMsg_.tokenId, address(this)) within the function will be 0 for all elements, resulting in no rewards being claimed for the receiver. As a result, the reward tokens will become trapped in the contract.

In the event that the sender utilizes multiple LayerZero composed messages containing two messages:

Permit message: to approve permission of _tokenId to the TapTokenReceiver contract.

Claim reward message: to trigger the _claimTwpTapRewardsReceiver() function and claim the reward.

The attacker cannot insert any twTAP.claimRewards() between these two messages, as they are executed within the same transaction on the destination chain. However, the permit message can be triggered by anyone, not just the contract TapTokenReceiver. The attacker can thus trigger the permit message on the destination chain and subsequently call the twTAP.claimRewards() function before the _claimTwpTapRewardsReceiver() message is delivered on the destination chain.

## Impact

The reward tokens will become trapped within the TapTokenReceiver contract.

## Recommended Mitigation Steps

Consider updating the function twTAP.claimRewards() as depicted below to impose restrictions on who can invoke this function:

function claimRewards ( uint256 _tokenId, address _to ) external nonReentrant whenNotPaused returns ( uint256 [] memory amounts_ ) { _requireClaimPermission ( msg.

sender, _tokenId ); _requireClaimPermission ( _to, _tokenId ); amounts_ = _claimRewards ( _tokenId, _to ); } 0xRektora (Tapioca) confirmed via duplicate Issue #120 0xRektora (Tapioca) commented:

Just as reference, the proposed mitigation will not work, because in this context msg.sender == _to.

# [H-04] Incorrect approval mechanism breaks all Magnetar functionality

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by carrotsmuggler, also found by KIntern_NA The Magnetar contract hands out approvals to various contracts so that the target contracts can use any tokens held currently by the Magnetar contract.

The issue is that at some point of time, all the target contracts were refactored to use permitC to handle token transfers. However, this change wasn’t reflected in the Magnetar contracts. Thus, instead of handing out permitC approvals, Magnetar hands out normal ERC20 approvals or yieldbox approvals. This essentially breaks the whole system.

There are numerous instances of this in the codebase. Essentially, almost every approval in the Magnetar contract is incorrect. Below are some examples, however the entire codebase needs to be checked for approvals and corrected.

The _depositYBLendSGL function in MagnetarAssetCommonModule.sol contract gives approval to the singularity contract via yieldbox. However, if we check the _addTokens function in the singularity contract below, we see the token transfers actually take place via pearlmit / permitC.

_setApprovalForYieldBox ( singularityAddress, yieldBox_ );

- https://github.com/Tapioca-DAO/Tapioca-bar/blob/9d76b2fc7e2752ca8a816af2d748a0259af5ea42/contracts/markets/singularity/SGLCommon.sol#L165-L177
Since the Magnetar contract does not give permitC approval to the singularity contract, and instead only gives yieldbox approval, the singularity contract is unable to transfer tokens from the Magnetar contract.

Similarly, in the _wrapSglReceipt function, the Magnetar gives approval to the TOFT contract vie ERC20 approval:

IERC20 ( sgl ).

approve ( tReceiptAddress, fraction ); But if we check the TOFT contract, we see the tokens are transferred via permitC and not with the raw tokens:

- https://github.com/Tapioca-DAO/TapiocaZ/blob/57750b7e997e5a1654651af9b413bbd5ea508f59/contracts/tOFT/BaseTOFT.sol#L73
Since the Magnetar contract does not hand out the permitC approvals, most of the token transfers via Magnetar will fail.

## Recommended Mitigation Steps

Refactor Magnetar to give approvals via permitC throughout.

cryptotechmaker (Tapioca) confirmed

# [H-05] _vested() claimable amount calculation error

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

_vested() claimable amount calculation error Submitted by bin2chen, also found by bin2chen, immeas, KIntern_NA, ronnyx2017, and deadrxsezzz The Vesting._vested() method is used to calculate the maximum claimable amount for the current user. The calculation formula is as follows:

(_totalAmount * (block.timestamp - _start)) / _duration. If there is an __initialUnlockTimeOffset, it needs to be subtracted from _start before performing the calculation, i.e., _start = _start - __initialUnlockTimeOffset.

function _vested ( uint256 _totalAmount ) internal view returns ( uint256 ) { uint256 _cliff = cliff; uint256 _start = start; uint256 _duration = duration; if ( _start == 0 ) return 0; // Not started if ( _cliff > 0 ) { _start = _start + _cliff; // Apply cliff offset if ( block.

timestamp < _start ) return 0; // Cliff not reached } @> if ( block.

timestamp >= _start + _duration ) return _totalAmount; // Fully vested _start = _start - __initialUnlockTimeOffset; // Offset initial unlock so it's claimable immediately return ( _totalAmount * ( block.

timestamp - _start )) / _duration; // Partially vested } The issue with the code snippet above is that the check for being “Fully vested” is incorrect; it does not take into account the __initialUnlockTimeOffset. The correct approach should be:

if (block.timestamp >= _start - __initialUnlockTimeOffset + _duration) return _totalAmount;// Fully vested. Resulting in calculations that may be greater than the maximum number _totalAmount Example:

_totalAmount = 500, duration = 1000 __initialUnlockTimeOffset = 100 start = 1000 block.timestamp= 1999 because block.timestamp < start + duration ( 1999 < 1000 + 1000 ) it will not return Fully vested.

Final calculation result: start = start - __initialUnlockTimeOffset = 1000 - 100 = 900. return = (_totalAmount * (block.timestamp - _start)) / _duration = 500 * (1999 - 900) / 1000 = 549.5.

It is greater 49.5 than the maximum _totalAmount=500.

## Impact

Users can `claim’ more than they should.

## Recommended Mitigation

function _vested(uint256 _totalAmount) internal view returns (uint256) { uint256 _cliff = cliff; uint256 _start = start; uint256 _duration = duration; if (_start == 0) return 0; // Not started if (_cliff > 0) { _start = _start + _cliff; // Apply cliff offset if (block.timestamp < _start) return 0; // Cliff not reached } - if (block.timestamp >= _start + _duration) return _totalAmount; // Fully vested + if (block.timestamp >= _start - __initialUnlockTimeOffset + _duration) return _totalAmount; // Fully vested _start = _start - __initialUnlockTimeOffset; // Offset initial unlock so it's claimable immediately return (_totalAmount * (block.timestamp - _start)) / _duration; // Partially vested } cryptotechmaker (Tapioca) confirmed, but disagreed with severity and commented via duplicate Issue #167

PR here.

# [H-06] Attacker can use MagnetarAction.OFT action of the Magnet to perform operations as any user including directly stealing user tokens

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

MagnetarAction.OFT action of the Magnet to perform operations as any user including directly stealing user tokens Submitted by ronnyx2017, also found by immeas, carrotsmuggler ( 1, 2 ), GalloDaSballo ( 1, 2, 3 ), rvierdiiev, deadrxsezzz, cccz, and ladboy233

- https://github.com/Tapioca-DAO/tapioca-periph/blob/032396f701be935b04a7e5cf3cb40a0136259dbc/contracts/Magnetar/Magnetar.sol#L153-L156
- https://github.com/Tapioca-DAO/tapioca-periph/blob/032396f701be935b04a7e5cf3cb40a0136259dbc/contracts/Magnetar/Magnetar.sol#L325-L333
- https://github.com/Tapioca-DAO/tapioca-periph/blob/032396f701be935b04a7e5cf3cb40a0136259dbc/contracts/Magnetar/MagnetarStorage.sol#L93-L97
This issue requires the combination of two vulnerabilities to achieve the impact described in the title. The first vulnerability is that the Magnetar._processOFTOperation function doesn’t check the function sigs in the action calldata with the the target addresses. It only ensures the calling target addresses are in the Whitelist of the Cluster. So an attacker can use this vuln to call any whitelist target address from the Magnetar.

The second vulnerability is that the Magnetar contract address itself will also be added to the Cluster whitelist. It can be found in the following integration test here. If the attacker can let the Magnetar call itself, the msg.sender in the sub-call will be in the whitelist. It will bypass the _checkSender check:

function _checkSender ( address _from ) internal view { if ( _from != msg.

sender && !

cluster.

isWhitelisted ( 0, msg.

sender )) { revert Magnetar_NotAuthorized ( msg.

sender, _from ); }

## Impact

Combining the two issues mentioned above, we can carry out the following exploitation.

Call Magnetar.burst function with _action.id == MagnetarAction.OFT, which will call _processOFTOperation function. The _target is the Magnetar contract itself, and the _actionCalldata is still an encoded calldata to call the Magnetar.burst function with _action.id == MagnetarAction.OFT again.

In the second call, the msg.sender will be the Magnetar itself, so the from address check in the _checkSender function will be skipped directly because the msg.sender is in the whitelist.

Now the attacker can pretend to be anyone and call any contract through the Magnetar. Please note that users approved their tokens to Magnetar if they used it.

# [H-07] Incorrect math means data.removeAndRepayData.removeAssetFromSGL will never work once SGL has accrued interest

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

data.removeAndRepayData.removeAssetFromSGL will never work once SGL has accrued interest Submitted by GalloDaSballo, also found by KIntern_NA and bin2chen The code to remove shares from Singularity is as follows:

- https://github.com/Tapioca-DAO/tapioca-periph/blob/2ddbcb1cde03b548e13421b2dba66435d2ac8eb5/contracts/Magnetar/modules/MagnetarOptionModule.sol#L158-L159
singularity_.

removeAsset ( data.

user, removeAssetTo, share ); Where share is computed in this way:

- https://github.com/Tapioca-DAO/tapioca-periph/blob/2ddbcb1cde03b548e13421b2dba66435d2ac8eb5/contracts/Magnetar/modules/MagnetarOptionModule.sol#L153
uint256 share = yieldBox_.

toShare ( _assetId, _removeAmount, false ); The line is calculating: The (incorrectly rounded down) amount of shares of Yieldbox to burn in order to withdraw from Yieldbox the _removeAmount.

But the code is calling:

singularity_.removeAsset(data.user, removeAssetTo, share); This is asking Singularity to remove a % (part) of the total assets in Singularity. Due to this, the line will stop working as soon as singularity has had any operation that generated interest.

## Mitigation

The unused function getFractionForAmount should help, minus some possible rounding considerations.

cryptotechmaker (Tapioca) confirmed, but disagreed with severity and commented via duplicate Issue #159:

PR here.

# [H-08] IMarket.execute.selector , _checkSender bypass allows to execute arbitrary operations

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

IMarket.execute.selector, _checkSender bypass allows to execute arbitrary operations Submitted by GalloDaSballo Because of an incorrect interpretation of calldata for the execute signature, we are able to bypass the _checkSender and perform arbitrary execute operations as Magnetar.

## Impact

Market.execute uses the following signature:

function execute ( Module [] calldata modules, bytes [] calldata calls, bool revertOnFail ) For Calldata variables, the size 4:36 is going to be the length of the calldata. We can specify an arbitrary length that matches the value of any address that is whitelisted, or any address that we’re able to generate. This will allow us to bypass the check and perform arbitrary execution in the market.

After forging our length, we have bypassed the check, allowing us to execute, while having permissions/allowances from other users:

- https://github.com/Tapioca-DAO/tapioca-periph/blob/2ddbcb1cde03b548e13421b2dba66435d2ac8eb5/contracts/Magnetar/Magnetar.sol#L256-L281
function _processMarketOperation ( address _target, bytes calldata _actionCalldata, uint256 _actionValue, bool _allowFailure ) private { if (!

cluster.

isWhitelisted ( 0, _target )) revert Magnetar_NotAuthorized ( _target, _target ); /// @dev owner address should always be first param.

// addCollateral(address from,...) // borrow(address from,...) // addAsset(address from,...) // repay(address _from,...) // buyCollateral(address from,...) // sellCollateral(address from,...) bytes4 funcSig = bytes4 ( _actionCalldata [:

4 ]); if ( funcSig == IMarket.

execute.

selector || funcSig == ISingularity.

addAsset.

selector /// @audit ??????

|| funcSig == ISingularity.

removeAsset.

selector ) { /// @dev Owner param check. See Warning above.

_checkSender ( abi.

decode ( _actionCalldata [ 4:

36 ], ( address ))); /// @audit we can forge this 80% _executeCall ( _target, _actionCalldata, _actionValue, _allowFailure ); return; }

## Mitigation

It may be necessary to remove execute from available commands as all commands will be performed by Magnetar.

## Assessed type

en/de-code 0xWeiss (Tapioca) confirmed

# [H-09] Funds can be stolen through remote transfer functionality

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-09
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by rvierdiiev User can send LZ message through any Oft token using the TapiocaOmnichainSender.sendPacket function. User provides params that should be used and also provides composed message if he needs to send it.

What is important for composed message is during crafting message, msg.sender is stored as srcChainSender_. In this way we know who have triggered composed call.

function encode ( bytes32 _sendTo, uint64 _amountShared, bytes memory _composeMsg ) internal view returns ( bytes memory _msg, bool hasCompose ) { hasCompose = _composeMsg.

length > 0; // @dev Remote chains will want to know the composed function caller ie. msg.sender on the src.

_msg = hasCompose ?

abi.

encodePacked ( _sendTo, _amountShared, addressToBytes32 ( msg.

sender ), _composeMsg ):

abi.

encodePacked ( _sendTo, _amountShared ); } The amount that should be sent to other chain is burnt (if any) and LZ call is sent. On another chain, the call will be handled by TapiocaOmnichainReceiver._lzReceive function. This function will mint tokens to recipient. If the composed message was included, then it will sent it to endpoint, so it can be triggered later.

When composed message is triggered, then lzCompose function handles it. As you can see, the function retrieves srcChainSender_ to know who was initiator of compose call on source chain. Then _lzCompose function continue processing of message.

Using msgType user can provide operation he wants to execute on target chain. One of operations is MSG_REMOTE_TRANSFER that allows to remotely send tokens to another chain. The flow is next: on chain A user initiates compose call to chain B, that will send his tokens on chain B to chain A, or will use allowance to send tokens of other user on chain B to chain A. Let’s check how it works.

First, the function should transfer tokens from owner to address(this). This function receives owner of funds and _srcChainSender as inputs to check allowance. As you can see, in the case of if _srcChainSender is owner then we don’t need any approve.

After transfer is done to address(this) then the contract can send them back to chain A. So the function burns tokens and crafts message to another chain and it can have composed call again; which means that it will include _srcChainSender, so the contract on chain A knows who initiated the call.

The problem is that _srcChainSender that will be included is owner of funds on chain B, which is incorrect.

Here’s the described attack flow:

Victim has funds on chain A, that attacker is going to steal to chain B.

Attacker on chain A initiates compose call with victim as owner of funds and provides amount 0 as amount to transfer of chain B.

Compose call succeed on chain B as it is possible to transfer 0 tokens and then another compose message was included, which transfers all tokens from victim to attacker on chain B.

Because _srcChainSender was set to victim on first compose call. Then the next compose call on chain A will think that victim is initiator of remote transfer, which means that no allowance will be checked.

Funds are stolen to attacker address on chain B.

## Impact

Possible to steal funds.

Tools Used VsCode

## Recommended Mitigation Steps

Provide _srcChainSender as initiator of compose call.

_internalRemoteTransferSendPacket( _srcChainSender, remoteTransferMsg_.lzSendParam, remoteTransferMsg_.composeMsg );

## Assessed type

Error 0xWeiss (Tapioca) confirmed

# [H-10] Adversary can steal approved tOLP s to Magnetar via _paricipateOnTOLP

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-10
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

tOLP s to Magnetar via _paricipateOnTOLP Submitted by deadrxsezzz Any user could steal any approved tOLP to Magnetar. This is because within the Magnetar call, if the user has not minted a tOLP NFT, they can participate with any id they wish, by inputting it in participateData.

function _participateOnTOLP ( IOptionsParticipateData memory participateData, address user, address lockDataTarget, uint256 tOLPTokenId ) internal { if (!

cluster.

isWhitelisted ( 0, participateData.

target )) { revert Magnetar_TargetNotWhitelisted ( participateData.

target ); } // Check tOLPTokenId if ( participateData.

tOLPTokenId != 0 ) { if ( participateData.

tOLPTokenId != tOLPTokenId && tOLPTokenId != 0 ) { revert Magnetar_tOLPTokenMismatch (); } tOLPTokenId = participateData.

tOLPTokenId; // @audit - does not verify sender owns that token } if ( tOLPTokenId == 0 ) revert Magnetar_ActionParamsMismatch (); IERC721 ( lockDataTarget ).

approve ( participateData.

target, tOLPTokenId ); uint256 oTAPTokenId = ITapiocaOptionBroker ( participateData.

target ).

participate ( tOLPTokenId ); address oTapAddress = ITapiocaOptionBroker ( participateData.

target ).

oTAP (); IERC721 ( oTapAddress ).

safeTransferFrom ( address ( this ), user, oTAPTokenId, "0x" ); } The only thing to consider is that the following line, must not revert:

IERC721 ( lockDataTarget ).

approve ( participateData.

target, tOLPTokenId ); Since the contract will not be an owner of tOLPTokenId, we’ll need to input a custom malicious lockDataTarget address, for which the approve will not revert. The lockDataTarget is not used at any other place within that function, so there’ll be no problem inputting a malicious address here.

After doing the described steps above, the attacker will lock the innocent user’s tOLP and get the oTAP NFT minted to themselves, effectively stealing the innocent user’s NFT.

## Recommended Mitigation Steps

Verify that the sender owns that tOLP id.

## Assessed type

ERC721 0xWeiss (Tapioca) confirmed

# [H-11] Adversary can utilise approved to Magnetar oTAP and tOLP NFTs

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-11
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

oTAP and tOLP NFTs Submitted by deadrxsezzz The idea of Magnetar is to allow users to batch transactions towards certain contract within the Tapioca contracts, including TapiocaOptionBroker and TapiocaOptionLiquidityProvision. In order to do so, users will have to give oTAP and tOLP allowance to the Magnetar contract.

The problem is that within the _processTapTokenOperation function, any user could make a call for another’s user’s approved NFT, as there are no checks that the msg.sender is the owner of the NFT.

function _processTapTokenOperation ( address _target, bytes calldata _actionCalldata, uint256 _actionValue, bool _allowFailure ) private { if (!

cluster.

isWhitelisted ( 0, _target )) revert Magnetar_NotAuthorized ( _target, _target ); bytes4 funcSig = bytes4 ( _actionCalldata [:

4 ]); if ( funcSig == ITapiocaOptionBroker.

exerciseOption.

selector || funcSig == ITapiocaOptionBroker.

participate.

selector || funcSig == ITapiocaOptionBroker.

exitPosition.

selector || funcSig == ITapiocaOptionLiquidityProvision.

lock.

selector || funcSig == ITapiocaOptionLiquidityProvision.

unlock.

selector ) { _executeCall ( _target, _actionCalldata, _actionValue, _allowFailure ); return; } revert Magnetar_ActionNotValid ( MagnetarAction.

TapToken, _actionCalldata ); } Example: A user can call exerciseOption for another person’s approved oTAP and exercise their option:

function exerciseOption ( uint256 _oTAPTokenID, ERC20 _paymentToken, uint256 _tapAmount ) external whenNotPaused { // Load data (, TapOption memory oTAPPosition ) = oTAP.

attributes ( _oTAPTokenID ); LockPosition memory tOLPLockPosition = tOLP.

getLock ( oTAPPosition.

tOLP ); bool isPositionActive = _isPositionActive ( tOLPLockPosition ); if (!

isPositionActive ) revert OptionExpired (); uint256 cachedEpoch = epoch; PaymentTokenOracle memory paymentTokenOracle = paymentTokens [ _paymentToken ]; // Check requirements if ( paymentTokenOracle.

oracle == ITapiocaOracle ( address ( 0 ))) { revert PaymentTokenNotSupported (); } if (!

oTAP.

isApprovedOrOwner ( msg.

sender, _oTAPTokenID )) { revert NotAuthorized (); } if ( block.

timestamp < oTAPPosition.

entry + EPOCH_DURATION ) { revert OneEpochCooldown (); } // Can only exercise after 1 epoch duration // Get eligible OTC amount uint256 gaugeTotalForEpoch = singularityGauges [ cachedEpoch ][ tOLPLockPosition.

sglAssetID ]; uint256 netAmount = uint256 ( netDepositedForEpoch [ cachedEpoch ][ tOLPLockPosition.

sglAssetID ]); if ( netAmount == 0 ) revert NoLiquidity (); uint256 eligibleTapAmount = muldiv ( tOLPLockPosition.

ybShares, gaugeTotalForEpoch, netAmount ); eligibleTapAmount -= oTAPCalls [ _oTAPTokenID ][ cachedEpoch ]; // Subtract already exercised amount if ( eligibleTapAmount < _tapAmount ) revert TooHigh (); uint256 chosenAmount = _tapAmount == 0 ?

eligibleTapAmount:

_tapAmount; if ( chosenAmount < 1e18 ) revert TooLow (); oTAPCalls [ _oTAPTokenID ][ cachedEpoch ] += chosenAmount; // Adds up exercised amount to current epoch // Finalize the deal _processOTCDeal ( _paymentToken, paymentTokenOracle, chosenAmount, oTAPPosition.

discount ); emit ExerciseOption ( cachedEpoch, msg.

sender, _paymentToken, _oTAPTokenID, chosenAmount ); }

## Recommended Mitigation Steps

Add checks within Magnetar that the user owns the NFT on which they’re making a call.

## Assessed type

Access Control 0xWeiss (Tapioca) confirmed

# [H-12] Adversary can steal user’s NFT’s if they have set Magnetar as isApprovedForAll == true

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** H-12
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

isApprovedForAll == true Submitted by deadrxsezzz Since Magnetar is supposed to be used as a router for multiple operations, it can be expected that user will have it pre-approved for their NFTs (such as tOLP as oTAP ones, as they’ll be the ones primarily used).

The Magnetar contract allows for any user to make a ERC721.approve, via _processPermitOperation:

function _processPermitOperation ( address _target, bytes calldata _actionCalldata, bool _allowFailure ) private { if (!

cluster.

isWhitelisted ( 0, _target )) revert Magnetar_NotAuthorized ( _target, _target ); /// @dev owner address should always be first param.

// permitAction(bytes,uint16) // permit(address owner...) // revoke(address owner...) // permitAll(address from,..) // permit(address from,...) // setApprovalForAll(address from,...) // setApprovalForAsset(address from,...) bytes4 funcSig = bytes4 ( _actionCalldata [:

4 ]); if ( funcSig == IPermitAll.

permitAll.

selector || funcSig == IPermitAll.

revokeAll.

selector || funcSig == IPermit.

permit.

selector || funcSig == IPermit.

revoke.

selector || funcSig == IYieldBox.

setApprovalForAll.

selector || funcSig == IYieldBox.

setApprovalForAsset.

selector || funcSig == IERC20.

approve.

selector || funcSig == IPearlmit.

approve.

selector || funcSig == IERC721.

approve.

selector ) { /// @dev Owner param check. See Warning above.

_checkSender ( abi.

decode ( _actionCalldata [ 4:

36 ], ( address ))); // No need to send value on permit _executeCall ( _target, _actionCalldata, 0, _allowFailure ); return; } revert Magnetar_ActionNotValid ( MagnetarAction.

Permit, _actionCalldata ); } The problem is that for OZ ERC721s (such as oTAP and tOLP ), if an NFT owner has approved a spender as isApprovedForAll, the spender can call approve for any NFTs belonging to the owner.

In other words, if user A has set Magnetar as approvedForAll, user B can call NFT.approve(userB, id) and get access to user A’s NFT:

function approve ( address to, uint256 tokenId ) public virtual override { address owner = ERC721.

ownerOf ( tokenId ); require ( to != owner, "ERC721: approval to current owner" ); require ( _msgSender () == owner || isApprovedForAll ( owner, _msgSender ()), "ERC721: approve caller is not token owner or approved for all" ); _approve ( to, tokenId ); }

## Recommended Mitigation Steps

Do not allow users to make a call with ERC721.approve selector.

0xWeiss (Tapioca) confirmed Medium Risk Findings (32)

# [M-01] Missing unwrap configuration when withdrawing cross-chain in the depositYBLendSGLLockXchainTOLP() function of MagnetarAssetXChainModule results in being unable to lock and participate on the destination chain

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

depositYBLendSGLLockXchainTOLP() function of MagnetarAssetXChainModule results in being unable to lock and participate on the destination chain Submitted by KIntern_NA, also found by carrotsmuggler, deadrxsezzz, and cccz ( 1, 2 ) The depositYBLendSGLLockXchainTOLP() function attempts to lend into Singularity, then withdraws the Singularity tokens cross-chain to lock and participate on the destination chain. The Singularity tokens are wrapped as TOFT tokens to facilitate cross-chain transfer.

uint256 fraction = _depositYBLendSGL ( data.

depositData, data.

singularity, IYieldBox ( yieldBox ), data.

user, data.

lendAmount ); // wrap SGL receipt into tReceipt // ! User should approve `address(this)` for `IERC20(data.singularity)` !

uint256 toftAmount = _wrapSglReceipt ( IYieldBox ( yieldBox ), data.

singularity, data.

user, fraction, data.

assetId ); This function calls _withdrawToChain() with the unwrap parameter set to false, indicating that TOFT-wrapped Singularity tokens will not be unwrapped upon receipt on the destination chain.

_withdrawToChain ( MagnetarWithdrawData ({ yieldBox:

yieldBox, assetId:

data.

assetId, unwrap:

false, lzSendParams:

data.

lockAndParticipateSendParams.

lzParams, sendGas:

data.

lockAndParticipateSendParams.

lzSendGas, composeGas:

data.

lockAndParticipateSendParams.

lzComposeGas, sendVal:

data.

lockAndParticipateSendParams.

lzSendVal, composeVal:

data.

lockAndParticipateSendParams.

lzComposeVal, composeMsg:

data.

lockAndParticipateSendParams.

lzParams.

sendParam.

composeMsg, composeMsgType:

data.

lockAndParticipateSendParams.

lzComposeMsgType, withdraw:

true }) ); However, the TapiocaOptionLiquidityProvision.lock() function attempts to acquire YieldBox’s shares of the original Singularity tokens. Therefore, upon receiving wrapped Singularity tokens on the destination chain, it should unwrap these tokens to facilitate the execution of subsequent actions.

## Impact

depositYBLendSGLLockXchainTOLP() will fail to execute the locking process after receiving wrapped Singularity tokens cross-chain.

## Recommended Mitigation Steps

depositYBLendSGLLockXchainTOLP() should call _withdrawToChain() with unwrap set to true.

## Assessed type

Context cryptotechmaker (Tapioca) confirmed and commented:

Fixed here.

# [M-02] tOLP positions created through MagnetarAction.Permit can be stolen

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

tOLP positions created through MagnetarAction.Permit can be stolen Submitted by immeas, also found by immeas

- https://github.com/Tapioca-DAO/tapioca-periph/blob/032396f701be935b04a7e5cf3cb40a0136259dbc/contracts/Magnetar/Magnetar.sol#L199-L212
- https://github.com/Tapioca-DAO/tapioca-periph/blob/032396f701be935b04a7e5cf3cb40a0136259dbc/contracts/Magnetar/Magnetar.sol#L304-L305
Description This issue is a combination of three other issues:

MagnetarAction.TapToken integration will leave tokens stuck in Magnetar contract.

A lot of calls in MagnetarAction.Permit enables anyone to steal whitelisted tokens held by Magnetar.

Anyone can take any whitelisted tokens approved to Magnetar.

In short, the first issue describes that, due to how MagnetarAction.TapToken is setup, it will leave the tOLP position a user creates through TapiocaOptionBroker::participate stuck in the Magnetar contract. As it mints the position to msg.sender which will be the Magnetar contract:

File:

tap - token / contracts / options / TapiocaOptionBroker.

sol 301:

oTAPTokenID = oTAP.

mint ( msg.

sender, lockExpiry, uint128 ( target ), _tOLPTokenID ); The two following ones:

Firstly, describes how anyone can take any tokens that are in the Magnetar contract by granting themselves permissions to transfer any whitelisted tokens in using MagnetarAction.Permit. Because of how MagnetarStorage::_checkSender validates that the first argument in the calldata to MagnetarAction.Permit is the same as msg.sender. This together with that a lot of the calls allowed through MagnetarAction.Permit ( IYieldBox::setApprovalForAll, IYieldBox::setApprovalForAsset, IERC20::approve, and IERC721::approve ) have address to; i.e. the operator/approvee as the first argument. Hence, any user can approve themselves to transfer tokens out of the contract.

Secondly, by using MagnetarAction.OFT; which allows anyone to transfer tokens out of Magnetar (and steal any approved tokens to Magnetar ) since there is an unvalidated call done to any whitelisted contract in MagnetarAction.OFT.

The contract is not in itself supposed to hold any tokens; in itself these issues are not that severe by themselves. However, these issues combined allows an attacker to steal the position completely. Since the first one makes the position be stuck in Magnetar and the two last ones makes it not actually stuck, but retrievable by anyone.

## Impact

If a user uses MagnetarAction.TapToken to create their position they can have their position and/or rewards stolen.

## Recommended Mitigation Steps

Consider implementing the mitigations described in the three mentioned referenced issues:

Adding the ability for the caller to declare a receiver in TapiocaOptionBroker::participate and exerciseOption. Similar to how it’s done in TapiocaOptionLiquidityProvision::lock.

Rethinking how MagnetarAction.Permit should work. As there is support in modules for a lot of the calls it is used for perhaps it is unnecessary to have.

Removing the general MagnetarAction.OFT call. Most of the interactions with the contracts in the Tapioca ecosystem is already handled in the Magnetar module system which handles approvals and transfers.

## Assessed type

Invalid Validation cryptotechmaker (Tapioca) disagreed with severity and commented:

The last 2 seems possible only if you approve the attacker, which is valid even for any random ERC20.

The first one can be an issue, but I don’t think the severity should be H. Maybe Medium or Low. I’ll let @0xRektora confirm as well.

0xRektora (Tapioca) confirmed and commented:

While this is a valid finding, the probability of it happening are very low. Will still put it as medium due to the nature of the issue.

MagnetarAction.TapToken integration will leave tokens stuck in Magnetar contract. A lot of calls in MagnetarAction. Permit enables anyone to steal whitelisted tokens held by Magnetar.

We don’t actually use TapToken singular actions, instead we use the MagnetarOptionModule. As for permits, we use TapiocaOmnichainEngine/OFT for that.

The probability are very low because the flow of action taken to the casual user is dictated by a different code. While this is possible to happen it’d have to be an advanced user, who probably will batch the actions together of permitting/locking/sending the token to his address. The Tx will revert if those are not done in the same Tx using Magnetar batching.

LSDan (judge) decreased severity to Medium

# [M-03] Spearbit finding “It is possible to exercise TAP option an extra time compared to lock duration” not fixed

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by immeas

- https://github.com/Tapioca-DAO/tap-token/blob/20a83b1d2d5577653610a6c3879dff9df4968345/contracts/options/TapiocaOptionBroker.sol#L408-L414
- https://github.com/Tapioca-DAO/tap-token/blob/20a83b1d2d5577653610a6c3879dff9df4968345/contracts/options/TapiocaOptionBroker.sol#L292-L298
Description The finding itself is a follow up on the C4 finding #189. The description of the issue can be found in the Spearbit report.

The mitigation introduced in the PR seems lost in the code base at audit here.

## Impact

Quoting the impact from the Spearbit report:

Attacker has removed one epoch of rewards from the long term stakers, receiving 2 tapOFT payoffs for 1 epoch long staking. More generally, an attacker can add 1 epoch of option rewards in excess to their actual locking time (as epsilon can be made minutes long and not significant position locking wise). This is a violation of base protocol token economy:

Lenders with active oTAP positions will receive oTAP shares from the DSO program every week that their position remains locked, proportional to their positions share of the total supplied locked liquidity in the respective market

## Recommended Mitigation Steps

Consider re-applying the same fix that was acknowledged in the Spearbit audit.

## Assessed type

Timing 0xRektora (Tapioca) confirmed cryptotechmaker (Tapioca) commented:

Fixed here.

# [M-04] anyone with a Pearlmit approval to transfer TapToken can have their funds stolen

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Pearlmit approval to transfer TapToken can have their funds stolen Submitted by immeas, also found by KIntern_NA, carrotsmuggler, and GalloDaSballo When transferring TapToken there is an extra check done in BaseTapiocaOmnichainEngine::transferFrom:

File:

tapioca - periph / contracts / tapiocaOmnichainEngine / BaseTapiocaOmnichainEngine.

sol 63:

if ( allowance ( from, spender ) < value ) { 64:

// _transfer(from, to, value); 65:

bool isErr = pearlmit.

transferFromERC20 ( from, to, address ( this ), value ); 66:

if ( isErr ) revert BaseTapiocaOmnichainEngine_NotValid (); 67: } else { Here there’s if the spender is not allowed, the allowance is checked in Pearlmit. The issue is that, Pearlmit checks the allowance against msg.sender which in this case will be the TapToken contract. Hence, any user with an allowance to the TapToken contract in Pearlmit can have their TAP stolen.

## Impact

If a user has allowed TapToken to transfer TapToken through Pearlmit, they can have all they have approved stolen. Since TapToken does a lot of token handling in composed messages, this is likely to happen.

## Recommended Mitigation Steps

Consider not doing the fallback to Pearlmit in transferFrom.

## Assessed type

Access Control cryptotechmaker (Tapioca) commented:

Low/Invalid; The issue seems a bit out of context. The link provided is from BaseTapiocaOmnichainEngine and there’s no context provided from which part this is triggered on TapToken.

Also this would be possible only if the approve in pearlmit is done with a large enough deadline and amount someone else can exploit. All pearlmit approvals have a deadline associated with them.

0xRektora (Tapioca) confirmed and commented:

I’d keep it as medium. Potential side effects might happen on current/future TOE tokens.

# [M-05] depositRepayAndRemoveCollateralFromMarket function of MagnetarAssetModule can’t be used on behalf of user

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

depositRepayAndRemoveCollateralFromMarket function of MagnetarAssetModule can’t be used on behalf of user Submitted by KIntern_NA, also found by GalloDaSballo Functions of Magnetar are intended to be callable from whitelisted addresses on behalf of users. This serves purposes such as allowing TOFT or USDT contracts to execute Magnetar functions during lzReceive (receiving tokens cross-chain).

All Magnetar functions use _checkSender to allow whitelisted sender:

function _checkSender ( address _from ) internal view { if ( _from != msg.

sender && !

cluster.

isWhitelisted ( 0, msg.

sender )) { revert Magnetar_NotAuthorized ( msg.

sender, _from ); } However, in the depositRepayAndRemoveCollateralFromMarket function of MagnetarAssetModule, it calls _extractTokens with the msg.sender address to pull tokens from the sender. Therefore, if msg.sender is different from data.user (the sender calling on behalf of the user), this function still pulls tokens from the sender. This behavior is incorrect, resulting in pulling tokens from the wrong address or reverting due to insufficient balance and allowance of msg.sender.

function depositRepayAndRemoveCollateralFromMarket ( DepositRepayAndRemoveCollateralFromMarketData memory data ) public payable { // Check sender _checkSender ( data.

user );...

// @dev deposit to YieldBox if ( data.

depositAmount > 0 ) { data.

depositAmount = _extractTokens ( msg.

sender, assetAddress, data.

depositAmount ); IERC20 ( assetAddress ).

approve ( address ( _yieldBox ), 0 ); IERC20 ( assetAddress ).

approve ( address ( _yieldBox ), data.

depositAmount ); _yieldBox.

depositAsset ( assetId, address ( this ), address ( this ), data.

depositAmount, 0 ); }...

## Impact

Senders will be at risk of losses due to mistakenly pulling tokens or facing a DOS attack when calling depositRepayAndRemoveCollateralFromMarket on behalf of the user. This intended functionality will be broken.

## Recommended Mitigation Steps

Should _extractTokens from data.user instead of msg.sender.

data.

depositAmount = _extractTokens ( msg.

sender, assetAddress, data.

depositAmount ); cryptotechmaker (Tapioca) confirmed and commented:

Fixed here.

# [M-06] depositYBLendSGLLockXchainTOLP function of the MagnetarAssetXChainModule will not work because it transfers Singularity tokens to the user before _withdrawToChain

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

depositYBLendSGLLockXchainTOLP function of the MagnetarAssetXChainModule will not work because it transfers Singularity tokens to the user before _withdrawToChain Submitted by KIntern_NA

- https://github.com/Tapioca-DAO/tapioca-periph/blob/032396f701be935b04a7e5cf3cb40a0136259dbc/contracts/Magnetar/modules/MagnetarAssetXChainModule.sol#L85-L114
- https://github.com/Tapioca-DAO/tapioca-periph/blob/032396f701be935b04a7e5cf3cb40a0136259dbc/contracts/Magnetar/modules/MagnetarAssetCommonModule.sol#L52-L63
Description In depositYBLendSGLLockXchainTOLP function of the MagnetarAssetXChainModule, after lending to SGL, it will wrap the received Singularity tokens into the TOFT wrapping token of it. Afterward, it attempts to transfer those wrapped tokens cross-chain, then unwrap them in the destination chain to lock and participate.

function depositYBLendSGLLockXchainTOLP ( DepositAndSendForLockingData memory data ) public payable {...

uint256 fraction = _depositYBLendSGL ( data.

depositData, data.

singularity, IYieldBox ( yieldBox ), data.

user, data.

lendAmount ); // wrap SGL receipt into tReceipt // ! User should approve `address(this)` for `IERC20(data.singularity)` !

uint256 toftAmount = _wrapSglReceipt ( IYieldBox ( yieldBox ), data.

singularity, data.

user, fraction, data.

assetId ); data.

lockAndParticipateSendParams.

lzParams.

sendParam.

amountLD = toftAmount; // decode `composeMsg` and re-encode it with updated params ( uint16 msgType_,, uint16 msgIndex_, bytes memory tapComposeMsg_, bytes memory nextMsg_ ) = TapiocaOmnichainEngineCodec.

decodeToeComposeMsg ( data.

lockAndParticipateSendParams.

lzParams.

sendParam.

composeMsg ); LockAndParticipateData memory lockData = abi.

decode ( tapComposeMsg_, ( LockAndParticipateData )); lockData.

fraction = toftAmount; data.

lockAndParticipateSendParams.

lzParams.

sendParam.

composeMsg = TapiocaOmnichainEngineCodec.

encodeToeComposeMsg ( abi.

encode ( lockData ), msgType_, msgIndex_, nextMsg_ ); // send on another layer for lending _withdrawToChain ( MagnetarWithdrawData ({ yieldBox:

yieldBox, assetId:

data.

assetId, unwrap:

false, lzSendParams:

data.

lockAndParticipateSendParams.

lzParams, sendGas:

data.

lockAndParticipateSendParams.

lzSendGas, composeGas:

data.

lockAndParticipateSendParams.

lzComposeGas, sendVal:

data.

lockAndParticipateSendParams.

lzSendVal, composeVal:

data.

lockAndParticipateSendParams.

lzComposeVal, composeMsg:

data.

lockAndParticipateSendParams.

lzParams.

sendParam.

composeMsg, composeMsgType:

data.

lockAndParticipateSendParams.

lzComposeMsgType, withdraw:

true }) ); } After lending into the Singularity contract, _wrapSglReceipt is used to wrap the received Singularity tokens into TOFT tokens. Thus, they will be able to be sent cross-chain by using _withdrawToChain with composed messages including the lockAndParticipate option to perform these actions after receiving tokens The _withdrawToChain function attempts to withdraw YieldBox shares of this contract (address(this)) to obtain tokens before sending them cross-chain (see this code snippet ). However, _wrapSglReceipt has sent tokens to the user after wrapping. Therefore, there are no tokens or YieldBox shares existing in this contract, resulting in _withdrawToChain reverting afterward.

function _wrapSglReceipt ( IYieldBox yieldBox, address sgl, address user, uint256 fraction, uint256 assetId ) internal returns ( uint256 toftAmount ) { IERC20 ( sgl ).

safeTransferFrom ( user, address ( this ), fraction ); (, address tReceiptAddress,,) = yieldBox.

assets ( assetId ); IERC20 ( sgl ).

approve ( tReceiptAddress, fraction ); toftAmount = ITOFT ( tReceiptAddress ).

wrap ( address ( this ), address ( this ), fraction ); IERC20 ( tReceiptAddress ).

safeTransfer ( user, toftAmount ); }

## Impact

depositYBLendSGLLockXchainTOLP function of MagnetarAssetXChainModule will be broken.

## Recommended Mitigation Steps

Should deposit into YieldBox for address(this) after wrapping Singularity tokens in the _wrapSglReceipt function as following:

function _wrapSglReceipt ( IYieldBox yieldBox, address sgl, address user, uint256 fraction, uint256 assetId ) internal returns ( uint256 toftAmount ) { IERC20 ( sgl ).

safeTransferFrom ( user, address ( this ), fraction ); (, address tReceiptAddress,,) = yieldBox.

assets ( assetId ); IERC20 ( sgl ).

approve ( tReceiptAddress, fraction ); toftAmount = ITOFT ( tReceiptAddress ).

wrap ( address ( this ), address ( this ), fraction ); //deposit to YieldBox for this IERC20 ( tReceiptAddress ).

safeApprove ( address ( yieldBox ), toftAmount ); yieldBox.

depositAsset ( assetId, address ( this ), address ( this ), toftAmount, 0 ); } cryptotechmaker (Tapioca) confirmed and commented:

PR here.

# [M-07] _lockOnTOB function of MagnetarMintCommonModule will not work due to the missing approved asset for YieldBox before depositing

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

_lockOnTOB function of MagnetarMintCommonModule will not work due to the missing approved asset for YieldBox before depositing Submitted by KIntern_NA, also found by carrotsmuggler In MagnetarMintCommonModule, the _lockOnTOB function is used to pull the singularity tokens from the user and lock them into the TapiocaOptionBroker contract.

function _lockOnTOB ( IOptionsLockData memory lockData, IYieldBox yieldBox_, uint256 fraction, bool participate, address user, address singularityAddress ) internal returns ( uint256 tOLPTokenId ) { tOLPTokenId = 0; if ( lockData.

lock ) { if (!

cluster.

isWhitelisted ( 0, lockData.

target )) { revert Magnetar_TargetNotWhitelisted ( lockData.

target ); } if ( lockData.

fraction > 0 ) fraction = lockData.

fraction; // retrieve and deposit SGLAssetId registered in tOLP ( uint256 tOLPSglAssetId,,) = ITapiocaOptionLiquidityProvision ( lockData.

target ).

activeSingularities ( singularityAddress ); if ( fraction == 0 ) revert Magnetar_ActionParamsMismatch (); //deposit to YieldBox _extractTokens ( user, singularityAddress, fraction ); yieldBox_.

depositAsset ( tOLPSglAssetId, address ( this ), address ( this ), fraction, 0 );...

} In the above code snippet, _extractTokens is used to pull singularity tokens from the user to this contract. Afterward, it will deposit these tokens into YieldBox to get YieldBox shares and then lock them in the TOB contract. However, it misses approving Singularity tokens before depositing them into YieldBox. YieldBox will attempt to pull tokens from this contract (from == address(this) ), so it will revert as YieldBox can’t transfer tokens due to insufficient allowance during yieldBox_.depositAsset().

## Impact

The functions of Magnetar which call _lockOnTOB will be broken, including the mintBBLendSGLLockTOLP function of MagnetarMintModule and the lockAndParticipate function of MagnetarMintXChainModule.

## Recommended Mitigation Steps

Should approve Singularity tokens before depositing them into YieldBox:

//deposit to YieldBox _extractTokens ( user, singularityAddress, fraction ); singularityAddress.

safeApprove ( address ( yieldBox_ ), fraction ); yieldBox_.

depositAsset ( tOLPSglAssetId, address ( this ), address ( this ), fraction, 0 ); cryptotechmaker (Tapioca) confirmed and commented:

PR here.

# [M-08] Incorrect return value of function BaseTapiocaOmnichainEngine._payNative()

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

BaseTapiocaOmnichainEngine._payNative() Submitted by KIntern_NA According to the function _payNative(_nativeFee) described in the LayerZero codebase, it is designed to return the native fee associated with the message. However, when a contract intends to initiate multiple LayerZero messages within a single transaction, more than just _nativeFee may be required from the sender to execute such messages.

The contract BaseTapiocaOmnichainEngine() facilitates multiple LayerZero messages within the Magnetar contract and the Tap token contract. Therefore, the function _payNativeFee() needs to be overridden to return an amount of native tokens greater than just _nativeFee. However, in the current implementation of the function BaseTapiocaOmnichainEngine._payNative(), it still returns the value of the input _nativeFee.

/** * @inheritdoc OAppSender * @dev Overwrite to check for < values.

*/ function _payNative ( uint256 _nativeFee ) internal override returns ( uint256 nativeFee ) { if ( msg.

value < _nativeFee ) revert NotEnoughNative ( msg.

value ); return _nativeFee; }

## Impact

As only _nativeFee will be sent along with the cross-chain message, the remaining amount msg.value - _nativeFee will become trapped in the BaseTapiocaOmnichainEngine contract. This amount can be larger than just the fee to execute the transaction since the Magnetar also supports the LzComposeOption, which defines the msg.value used to execute the compose option.

Due to the insufficient native tokens provided for the multiple LayerZero messages, certain functions cannot be executed (e.g., MagnetarBaseModule._lzCustomWithdraw(), TapTokenReceiver._claimTwpTapRewardsReceiver(), …).

## Recommended Mitigation Steps

Consider modifying function BaseTapiocaOmnichainEngine._payNative() as follows:

function _payNative ( uint256 _nativeFee ) internal override returns ( uint256 nativeFee ) { if ( msg.

value < _nativeFee ) revert NotEnoughNative ( msg.

value ); return msg.

value; }

## Assessed type

Context LSDan (judge) decreased severity to Medium 0xWeiss (Tapioca) confirmed

# [M-09] Magnetar’s mintBBLendSGLLockTOLP reverts when lock is set to false

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

mintBBLendSGLLockTOLP reverts when lock is set to false Submitted by carrotsmuggler, also found by KIntern_NA The mintBBLendSGLLockTOLP function in Magnetar is designed to mint USDO tokens from bigbang, deposit them to singularity, lock the liquidity in the TOLP contract and then participate in the TOB contract.

The function is designed to be modular, so the user can choose to skip any of the steps and still have the other execute. The issue is that if the user decides not lock in the TOLP contract, the function will revert since it is pulling tokens from the wrong address.

After the market operations, the function does two operations for locking, as shown below:

uint256 tOLPTokenId = _lockOnTOB ( data.

lockData, yieldBox_, fraction, data.

participateData.

participate, data.

user, data.

externalContracts.

singularity ); if ( data.

participateData.

participate ) { _participateOnTOLP ( data.

participateData, data.

user, data.

lockData.

target, tOLPTokenId ); } In the _lockOnTOB function, there is a check which allows the user to skip this step based on their input:

if ( lockData.

lock ) { //...

} However, if this step is skipped, then the TOLP NFT position will still be with the user; thus, needs to be pulled from the user for the participate step. But in the participate step, we see that the code expects the token to be with the Magnetar contract already.

IERC721 ( lockDataTarget ).

approve ( participateData.

target, tOLPTokenId ); uint256 oTAPTokenId = ITapiocaOptionBroker ( participateData.

target ).

participate ( tOLPTokenId ); address oTapAddress = ITapiocaOptionBroker ( participateData.

target ).

oTAP (); IERC721 ( oTapAddress ).

safeTransferFrom ( address ( this ), user, oTAPTokenId, "0x" ); This would only be true if the previous step’s lock function had run. This is because when locking the tokens, the Magnetar contract receives the NFT position if participate function is set to be run. If the lock function is skipped, the participate function will revert since the NFT position is still with the user.

## Recommended Mitigation Steps

If lockData.lock was true, the _participateOnTOLP function should pull the NFT positions from the user to the Magnetsar contract.

cryptotechmaker (Tapioca) confirmed and commented:

PR here.

# [M-10] Magnetar unwrap operations broken due to bad ownership and checks

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by carrotsmuggler The _processWrapOperation function can be triggered on the Magnetar contract to wrap/unwrap the user’s tokens into or out of the TOFT contracts. This function allows 2 selectors, wrap and unwrap to be called.

The issue is that unwrap function has 2 issues which prevent it from working properly. Below is the code from the BaseTOFT.sol contract, showing the implementation of the unwrap: function.

function _unwrap ( address _toAddress, uint256 _amount ) internal virtual { _burn ( msg.

sender, _amount ); vault.

withdraw ( _toAddress, _amount ); } As seen here, the token is burnt from the msg.sender address. However, the _processWrapOperation function in the Magnetar contract does not transfer out the token from the user’s address to itself before calling _unwrap.

if ( funcSig == ITOFT.

wrap.

selector || funcSig == ITOFT.

unwrap.

selector ) { /// @dev Owner param check. See Warning above.

_checkSender ( abi.

decode ( _actionCalldata [ 4:

36 ], ( address ))); _executeCall ( _target, _actionCalldata, _actionValue, _allowFailure ); return; } So the Magnetar contract isn’t in possession of the token that the TOFT contract is trying to burn. The Magnetar contract itself is not designed to hold user tokens, since anyone can claim them. Users should not send their tokens to the Magnetar contract manually and then call this function, since MEV bots can steal tokens from this contract by just calling the unwrap function before them. Due to this, there is no way for Magnetar to actually unwrap the tokens.

Secondly, the _checkSender function is used to check the first passed address against msg.sender. the issue is that the first address passed to the unwrap function is the destination address, not the owner’s address. The owner is assumed to be msg.sender. So this contract essentially only makes sure that the msg.sender matches the destination of the unwrapped tokens, which is not a useful check.

Since these two issues break the functionality of this function, this is a medium severity issue.

## Recommended Mitigation Steps

The unwrap functionality should transfer the token from the msg.sender to itself first. This will ensure both that the caller owns the token and that the token is in the possession of the Magnetar contract. The _checkSender check for the unwrap case is unnecessary and prevents users from choosing a destination address.

## Assessed type

Invalid Validation cryptotechmaker (Tapioca) confirmed and commented:

PR here.

# [M-11] twAML weights can be griefed by burning tokens

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

twAML weights can be griefed by burning tokens Submitted by carrotsmuggler Users can lock their liquidity in the TOLP contract and mint OTAP tokens in the TOB contract. The TOB contract has a special mechanism called twAML to balance out how much rewards they emit over time.

Basically, if a user commits OTAP tokens worth more than a minimum amount of shares, they are eligible to sway the votes:

bool hasVotingPower = lock.

ybShares >= computeMinWeight ( pool.

totalDeposited + VIRTUAL_TOTAL_AMOUNT, MIN_WEIGHT_FACTOR ); This allows them to influence the magnitude, divergence as well as the twAML value of this asset id.

pool.

averageMagnitude = ( pool.

averageMagnitude + magnitude ) / pool.

totalParticipants; // compute new average magnitude // Compute and save new cumulative divergenceForce = lock.

lockDuration >= pool.

cumulative; if ( divergenceForce ) { pool.

cumulative += pool.

averageMagnitude; } else { if ( pool.

cumulative > pool.

averageMagnitude ) { pool.

cumulative -= pool.

averageMagnitude; } else { pool.

cumulative = 0; } // Save new weight pool.

totalDeposited += lock.

ybShares; twAML [ lock.

sglAssetID ] = pool These values determine how large of a discount the users can get when exercising their options.

Similarly, when users decide to exit their position, or if their lock has expired, either they themselves or other users can kick them out of the TOB system and reset the twAML values to the values it was before.

if (!

isSGLInRescueMode && participation.

hasVotingPower ) { TWAMLPool memory pool = twAML [ lock.

sglAssetID ]; if ( participation.

divergenceForce ) { //...

So the twAML change a single user can cause is limited to their lock duration. However, users also have another option: they can directly burn their OTAP token after participating. This is because the OTAP contract has an open burn function.

function burn ( uint256 _tokenId ) external { if (!

_isApprovedOrOwner ( msg.

sender, _tokenId )) revert NotAuthorized (); _burn ( _tokenId ); emit Burn ( msg.

sender, _tokenId, options [ _tokenId ]); } Now, these user’s contributions to the twAML calculations cannot be wiped out after their lock expires. This is because the exitPosition function calls otap.burn which will fail since the user has already burnt their tokens.

So users can affect the twAML calculations for an infinite amount of time by burning tokens. This scenario is specifically prevented in the TOLP contract which has a max lock duration enforced with MAX_LOCK_DURATION.

Since users can permanently affect the twAML calculations, this is a medium severity issue.

## Recommended Mitigation Steps

Disable the open burn function in the OTAP contract. Only allows selected contracts such as the TOB contract to call it.

## Assessed type

Math 0xRektora (Tapioca) confirmed

# [M-12] A single second in an epoch makes an user eligible for the entire epoch’s rewards

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by carrotsmuggler, also found by KIntern_NA, deadrxsezzz ( 1, 2 ), and GalloDaSballo A user can lock their LP tokens into the TOLP contract, and then put their TOLP into the TOB contract to be eligible for option rewards. These option rewards are paid out every epoch and distributed proportionally to all the participants in that epoch. When locking LP tokens in TOLP, the user can choose their own _lockDuration.

The TOB contract has 2 important functions. One is the exitPosition, which allows users to take out their TOLP token, and the other is the exerciseOption function, which allows the users to collect the option rewards. The issue is that the TOLP contract records lock duration in seconds, while the TOB contract processes them in epochs.

So if a user creates a lock position where the lock end time is a single second after the epoch change, the epoch when the lock is due to expire in the TOB contract will be calculated as shown.

uint128 lockExpiry = lock.

lockTime + lock.

lockDuration; uint256 lastEpoch = _timestampToWeek ( lockExpiry ); So if a user lock expires a single second into an epoch, the lastEpoch will be calculated as that epoch and the user will be eligible for that entire epoch’s rewards.

Moreover, this also allows a peculiar situation where a user can take out their TOLP token and exit their position and then come back and collect their token rewards. This is because when users want to exit their position, the contract checks if the lock has expired.

if (!

isSGLInRescueMode ) { if ( block.

timestamp < lock.

lockTime + lock.

lockDuration ) { revert LockNotExpired (); } But when a user wants to exercise their options, the contract checks for the liveness of the position in different terms, the epoch.

uint256 expiryWeek = _timestampToWeek ( _lock.

lockTime + _lock.

lockDuration ); isPositionActive = epoch <= expiryWeek; The first method and the second method can disagree if a position is active. If the expiry time is in the middle of an epoch, for the entire second half of the epoch, the exitPosition function will treat the position as expired and will allow the user to take out their TOLP token, but the exerciseOption contract will treat the position as active and will allow the user to collect rewards as well. Due to the discussion above, the user can have only a single second of lock time remaining in this second epoch; so for the entire duration of 7 days, the user can have no TOLP locked in the contract but still be eligible to withdraw rewards.

## Recommended Mitigation Steps

The different contracts should use the same mechanism to define locks. Either use epochs in both TOLP and TOB contracts, or use seconds in both. The current situation is a bit confusing and can lead to unexpected behavior.

## Assessed type

Invalid Validation 0xWeiss (Tapioca) confirmed

# [M-13] Rescue request timestamp not reset in TapiocaOptionLiquidityProvision.sol contract

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

TapiocaOptionLiquidityProvision.sol contract Submitted by carrotsmuggler, also found by immeas

## Impact

In order to put an asset in rescue mode, the admin first needs to call requestSglPoolRescue. This will start a cooldown period and the rescue can only be triggered after the cooldown period has passed. This is evident from the code in the activateSglPoolRescue function.

if ( block.

timestamp < sglRescueRequest [ sgl.

sglAssetID ] + rescueCooldown ) revert RescueCooldownNotReached (); The issue is that once the rescue has been carried out, the value in sglRescueRequest[sgl.sglAssetID] is not reset. This means if the same asset is added back in, it will now not have any cooldown anymore. The contract will treat the asset as if the rescue request is still ongoing.

The requestSglPoolRescue cannot be called to reset the cooldown, since the value stored is non 0. Since this bypasses a protection mechanism put in place by the devs, this is a medium severity issue.

## Recommended Mitigation Steps

Delete the entry in the sglRescueRequest mapping when the asset is rescued via the activateSglPoolRescue function.

0xRektora (Tapioca) confirmed, but disagreed with severity and commented:

Informational.

# [M-14] Options can be exercised preemptively due to timing delays

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by carrotsmuggler, also found by GalloDaSballo and deadrxsezzz When a user participates in the options system via TapiocaOptionBroker.sol, they mint an otap position in exchange for their tolp position. The time of creation is recorded, and the user is allowed to exercise the option only after a full epoch duration, which is a week.

if ( block.

timestamp < oTAPPosition.

entry + EPOCH_DURATION ) { revert OneEpochCooldown (); } The variable netDepositedForEpoch records how much liquidity the system expects to hold in any given epoch, and is used to calculate the rewards for the users. On calling participate, this variable is increased for the next epoch, and decreased on the expiration epoch.

netDepositedForEpoch [ epoch + 1 ][ lock.

sglAssetID ] += int256 ( uint256 ( lock.

ybShares )); netDepositedForEpoch [ lastEpoch + 1 ][ lock.

sglAssetID ] -= int256 ( uint256 ( lock.

ybShares )); The issue is this accounting is done via epoch, while the cooldown is accounted via block.timestamp. So if a user participates at the very beginning of an epoch, their option will be exercisable after 7 days, which is very close to the time when the epoch number can be incremented. If the user is able to call exerciseOption before the epoch number is incremented, they can even participate in the current epoch! This is because the epoch number has not yet been updated, but the EPOCH_DURATION has already passed.

In this scenario, the user gets access to the rewards of the very same epoch they participated in, which should be impossible. They can also cause the contract to run out of rewards, as they are claiming rewards they shouldn’t be owed. The contract calculates rewards based on the current net deposit, which hasn’t been updated yet since the epoch number hasn’t been updated yet.

uint256 eligibleTapAmount = muldiv ( tOLPLockPosition.

ybShares, gaugeTotalForEpoch, netAmount ); eligibleTapAmount -= oTAPCalls [ _oTAPTokenID ][ cachedEpoch ]; Thus the user can take out rewards based on the wrong netAmount, which can lead to other users not getting their rewards, since the eligibleAmount is decreased by users not owed any rewards.

## Recommended Mitigation Steps

The main issue is that the OneEpochCooldown is not measured in epochs and is measured in time instead, which will not necessarily always coincide. Can be fixed in a variety of methods:

Check OneEpochCooldown based on the epoch number, not the timestamp.

When calling exerciseOption, check if the epoch has completed. Similar to what is done in the participate function if ( _timestampToWeek ( block.

timestamp ) > epoch ) revert AdvanceEpochFirst ();

## Assessed type

Invalid Validation 0xRektora (Tapioca) confirmed via duplicate Issue #35 LSDan (judge) decreased severity to Medium cryptotechmaker (Tapioca) commented via duplicate Issue #35:

PR here.

# [M-15] _internalRemoteTransferSendPacket() can’t send the difference back to the user

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

_internalRemoteTransferSendPacket() can’t send the difference back to the user Submitted by bin2chen, also found by cccz In TapiocaOmnichainReceiver, when the user executes MSG_REMOTE_TRANSFER, if the srcChain amount request is bigger than the debited one, it overwrites the amount to credit with the amount debited and send the difference back to the user.

lzCompose() -> _remoteTransferReceiver() -> _internalRemoteTransferSendPacket().

function _internalRemoteTransferSendPacket ( address _srcChainSender, LZSendParam memory _lzSendParam, bytes memory _composeMsg ) internal returns ( MessagingReceipt memory msgReceipt, OFTReceipt memory oftReceipt ) {...

// If the srcChain amount request is bigger than the debited one, overwrite the amount to credit with the amount debited and send the difference back to the user.

if ( _lzSendParam.

sendParam.

amountLD > amountDebitedLD_ ) { // Overwrite the amount to credit with the amount debited @> _lzSendParam.

sendParam.

amountLD = amountDebitedLD_; _lzSendParam.

sendParam.

minAmountLD = amountDebitedLD_; // Send the difference back to the user @> _transfer ( address ( this ), _srcChainSender, _lzSendParam.

sendParam.

amountLD - amountDebitedLD_ ); } The above code, first modify _lzSendParam.sendParam.amountLD = amountDebitedLD_ Then call _transfer(address(this), _srcChainSender, _lzSendParam.sendParam.amountLD - amountDebitedLD_);. This way the difference is always 0.

Correctly transfer() the difference first, and then modify _lzSendParam.sendParam.amountLD.

## Impact

The difference token be left in the contract.

## Recommended Mitigation

function _internalRemoteTransferSendPacket( address _srcChainSender, LZSendParam memory _lzSendParam, bytes memory _composeMsg ) internal returns (MessagingReceipt memory msgReceipt, OFTReceipt memory oftReceipt) {...

// If the srcChain amount request is bigger than the debited one, overwrite the amount to credit with the amount debited and send the difference back to the user.

if (_lzSendParam.sendParam.amountLD > amountDebitedLD_) { + _transfer(address(this), _srcChainSender, _lzSendParam.sendParam.amountLD - amountDebitedLD_); // Overwrite the amount to credit with the amount debited _lzSendParam.sendParam.amountLD = amountDebitedLD_; _lzSendParam.sendParam.minAmountLD = amountDebitedLD_; // Send the difference back to the user - _transfer(address(this), _srcChainSender, _lzSendParam.sendParam.amountLD - amountDebitedLD_); }

## Assessed type

Context LSDan (judge) decreased severity to Medium cryptotechmaker (Tapioca) confirmed and commented:

PR here.

# [M-16] burst() does not return eth when action fails

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

burst() does not return eth when action fails Submitted by bin2chen, also found by ronnyx2017, rvierdiiev, and deadrxsezzz ( 1, 2 ) When executing Magnetar.burst(), the user can specify allowFailure = true. If it fails, it ignores the current _action and execute another _action.

burst() -> _executeCall(_allowFailure).

function _executeCall ( address _target, bytes calldata _actionCalldata, uint256 _actionValue, bool _allowFailure ) private { bool success; bytes memory returnData; if ( _actionValue > 0 ) { ( success, returnData ) = _target.

call {value:

_actionValue }( _actionCalldata ); } else { ( success, returnData ) = _target.

call ( _actionCalldata ); } @> if (!

success && !

_allowFailure ) { _getRevertMsg ( returnData ); } But with the current implementation, if the user also passes _action.value> 0 and _action.allowFailure = true, if the action fails, the _action.value is not returned to the user, it stays in the contract.

Although the owner can retrieve the eth left in the contract via rescueEth(), the eth is not returned to the user until the owner executes rescueEth(). However, before the owner executes rescueEth(), a malicious user can use the contract’s eth directly to execute an action.

For example, by using MagnetarAssetModule.depositRepayAndRemoveCollateralFromMarket() -> _withdrawToChain() -> _lzWithdraw() -> sendPacket{value:??} () to use up eth.

## Impact

If the action fails, the eth left in the contract can be stolen by malicious people.

## Recommended Mitigation

Recommends the final return of all eth:

function burst(MagnetarCall[] calldata calls) external payable { uint256 valAccumulator; uint256 length = calls.length; for (uint256 i; i < length; i++) {.....

if (msg.value != valAccumulator) revert Magnetar_ValueMismatch(msg.value, valAccumulator); + if(address(this).balance>0){ + msg.sender.call{value:address(this).balance}(""); // return all eth + } }

## Assessed type

Context cryptotechmaker (Tapioca) confirmed and commented:

PR here.

# [M-17] After unregisterSingularity() , position has not been unlocked and will be locked in the contract

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

unregisterSingularity(), position has not been unlocked and will be locked in the contract Submitted by bin2chen, also found by deadrxsezzz and cccz When singularity.rescue==true, owner can execute unregisterSingularity(). This method will delete activeSingularities[]/sglAssetIDToAddress[].

function unregisterSingularity ( IERC20 singularity ) external onlyOwner updateTotalSGLPoolWeights { uint256 sglAssetID = activeSingularities [ singularity ].

sglAssetID; if ( sglAssetID == 0 ) revert NotRegistered (); if (!

activeSingularities [ singularity ].

rescue ) revert NotInRescueMode (); unchecked { uint256 [] memory _singularities = singularities; uint256 sglLength = _singularities.

length; uint256 sglLastIndex = sglLength - 1; for ( uint256 i; i < sglLength; i ++) { if ( _singularities [ i ] == sglAssetID ) { // If in the middle, copy last element on deleted element, then pop @> delete activeSingularities [ singularity ]; @> delete sglAssetIDToAddress [ sglAssetID ]; if ( i != sglLastIndex ) { singularities [ i ] = _singularities [ sglLastIndex ]; } singularities.

pop (); emit UnregisterSingularity ( address ( singularity ), sglAssetID ); break; } emit UnregisterSingularity ( address ( singularity ), sglAssetID ); } However, this method does not determine whether there are still has locked positions. In this way, if the user wants to unlock(), it will not be executed and the token will be locked in the contract.

activeSingularities[_singularity] has been deleted.

function unlock ( uint256 _tokenId, IERC20 _singularity, address _to ) external {...

@> SingularityPool memory sgl = activeSingularities [ _singularity ]; yieldBox.

transfer ( address ( this ), _to, lockPosition.

sglAssetID, lockPosition.

ybShares ); @> activeSingularities [ _singularity ].

totalDeposited -= lockPosition.

ybShares; emit Burn ( _to, lockPosition.

sglAssetID, _tokenId ); }

## Recommended Mitigation

function unregisterSingularity(IERC20 singularity) external onlyOwner updateTotalSGLPoolWeights {...

for (uint256 i; i < sglLength; i++) { if (_singularities[i] == sglAssetID) { + //check totalDeposited + require(activeSingularities[singularity].totalDeposited == 0, "invalid"); // If in the middle, copy last element on deleted element, then pop delete activeSingularities[singularity]; delete sglAssetIDToAddress[sglAssetID]; if (i != sglLastIndex) { singularities[i] = _singularities[sglLastIndex]; } singularities.pop(); emit UnregisterSingularity(address(singularity), sglAssetID); break; }

## Assessed type

Context cryptotechmaker (Tapioca) confirmed and commented:

PR here.

0xRektora (Tapioca) commented:

@LSDan, This one should be QA.

Although it makes sense to add this change, but there’ll never realistically be 0 total deposited in a pool, leaving some dust. Furthermore, it opens up an attack vector to DoS this functionality. I’d say it’s the responsibility of the DAO to make sane decision about when to unregister the pool. There’s already a 2 day cooldown period as a safeguard.

# [M-18] TapiocaOptionBroker.participate() with the approve authorization, it still cannot be executed

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-18
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

TapiocaOptionBroker.participate() with the approve authorization, it still cannot be executed Submitted by bin2chen, also found by carrotsmuggler and cccz If the user isApproved(), the user can call TapiocaOptionBroker.participate(). The code is as follows:

function participate ( uint256 _tOLPTokenID ) external whenNotPaused nonReentrant returns ( uint256 oTAPTokenID ) {...

TWAMLPool memory pool = twAML [ lock.

sglAssetID ]; if ( pool.

cumulative == 0 ) { pool.

cumulative = EPOCH_DURATION; } @> if (!

tOLP.

isApprovedOrOwner ( msg.

sender, _tOLPTokenID )) { revert NotAuthorized (); } { @> bool isErr = pearlmit.

transferFromERC721 ( msg.

sender, address ( this ), address ( tOLP ), _tOLPTokenID ); if ( isErr ) revert TransferFailed (); } However, in the current implementation, even though msg.sender has obtained authorization ( isApprovedOrOwner(msg.sender) == true ), it is still unable to execute due to the incorrect usage of:

pearlmit.transferFromERC721(msg.sender, address(this), address(tOLP), _tOLPTokenID); Passing msg.sender as the owner will result in failure to execute.

pearlmit.transferFromERC721(msg.sener,to) -> IERC721(token).transferFrom(owner, to) -> _transfer(owner,to) -> require(ERC721.ownerOf(tokenId) == owner.

It will check the first parameter is the owner of NFT. It should use:

pearlmit.transferFromERC721(tOLP.ownerOf(_tOLPTokenID), address(this), address(tOLP), _tOLPTokenID).

## Impact

Although msg.sender has been granted authorization, it is still unable to execute participate().

## Recommended Mitigation

function participate(uint256 _tOLPTokenID) external whenNotPaused nonReentrant returns (uint256 oTAPTokenID) {...

if (!tOLP.isApprovedOrOwner(msg.sender, _tOLPTokenID)) { revert NotAuthorized(); } // Transfer tOLP position to this contract // tOLP.transferFrom(msg.sender, address(this), _tOLPTokenID); { - bool isErr = pearlmit.transferFromERC721(msg.sender, address(this), address(tOLP), _tOLPTokenID); + bool isErr = pearlmit.transferFromERC721(tOLP.ownerOf(_tOLPTokenID), address(this), address(tOLP), _tOLPTokenID); if (isErr) revert TransferFailed(); }

## Assessed type

Context 0xRektora (Tapioca) confirmed cryptotechmaker (Tapioca) commented:

PR here.

# [M-19] Adding reward tokens to twTap could cause pending cross-chain claim rewards msg to be stuck

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-19
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

twTap could cause pending cross-chain claim rewards msg to be stuck Submitted by ronnyx2017 There is a check in the TapTokenReceiver._claimTwpTapRewardsReceiver to ensure the claimed types amount is equal to the token types amount of sendParam.

if ( claimedAmount_.

length - 1 ) // Remove 1 because the first index doesn't count.

!= claimTwTapRewardsMsg_.

sendParam.

length ) { revert The process of cross-chain claim is as follows: The B chain is the main chain, which the twTap is deployed. And the user is on the chain A.

The user calls TapOFT.sendPacket with MSG_CLAIM_REWARDS message on the chain A.

The LZ replays the message on the chain B, which will call the TapTokenReceiver._claimTwpTapRewardsReceiver function.

TapTokenReceiver._claimTwpTapRewardsReceiver function will send every reward token to the chain A by LZ TapTokenSender(rewardToken_).sendPacket.

## Impact

Every LZ message sent by TapToken.sendPacket can carry cross-chain TAP tokens, which means the amountToSendLD of the message is not zero. It will burn the TAP token of the source chain and mint them on the target chain. If the message is stuck, these TAP token will be lost forever.

## Assessed type

Context 0xWeiss (Tapioca) disputed Note: For full discussion, see here.

# [M-20] Max magnitude lock check will lead to a DoS and possible monopolization of gov/Option on twTAP / TapiocaOptionBroker

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-20
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

gov/Option on twTAP / TapiocaOptionBroker Submitted by ronnyx2017

- https://github.com/Tapioca-DAO/tap-token/blob/20a83b1d2d5577653610a6c3879dff9df4968345/contracts/governance/twTAP.sol#L315
- https://github.com/Tapioca-DAO/tap-token/blob/20a83b1d2d5577653610a6c3879dff9df4968345/contracts/options/TapiocaOptionBroker.sol#L261

## Vulnerability details

There is a max magnitude lock check in the twTAP.participate function, it can’t be greater than 4x the cumulative:

if (magnitude >= pool.cumulative * 4) revert NotValid(); Because the param _duration can’t be less than EPOCH_DURATION = 7 days = 604800, lets assume pool.cumulative is x, and solve for its minimum value without DoS:

(604800 * 604800 + x * x)**0.5 - x < 4 * x Solution derived:

the x, pool.cumulative, must be >= 123455.

The main issue is that, when a user participates the lock with divergenceForce = false, his participation will decrease pool.cumulative. Meanwhile, if there are positions previously entered with divergenceForce = true that are now exiting, then pool.cumulative will be reduced to below 123455.

## Impact

TAP cannot be staked until pool.cumulative rises to 123455, and the system will remain under the DoS for even months until positions with divergenceForce = false can exit (arrive at the unlocking time). During this DoS period, the attacker has the potential to monopolize gov.

The same issue is also in the TapiocaOptionBroker

## Assessed type

Context 0xWeiss (Tapioca) acknowledged Note: For full discussion, see here.

# [M-21] Gov ( twTAP ) and Tapioca Option can be monopolized by an attacker

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-21
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

twTAP ) and Tapioca Option can be monopolized by an attacker Submitted by ronnyx2017, also found by immeas and GalloDaSballo

- https://github.com/Tapioca-DAO/tap-token/blob/20a83b1d2d5577653610a6c3879dff9df4968345/contracts/governance/twTAP.sol#L592-L594

## Vulnerability details

The function twTAP._releaseTap updates only --twAML.totalParticipants and neglected to update twAML.averageMagnitude. This will result in twAML.averageMagnitude accumulating every time a new position participates, without ever decreasing.

pool.

averageMagnitude = ( pool.

averageMagnitude + magnitude ) / pool.

totalParticipants; // compute new average magnitude

## Impact

An attacker can monopolize the gov by precisely controlling the number of tokens entering and exiting to ensure that only their own position remains in the current twTap.

On the other hand, there is another exploit way here. If pool.cumulative < pool.averageMagnitude, then pool.cumulative will be set to 0. But the greater position.averageMagnitude will be added to pool.cumulative when the position exits. This will cause pool.cumulative to continuously increase, which will result in the efficiency of twTAP / TapiocaOptionBroker becoming increasingly lower. Because the multiplier of the twTAP or target of the TapiocaOptionBroker will always be dMIN.

The same issue is also in the TapiocaOptionBroker. The only difference is that the pool.cumulative can’t be decreased to zero, because it will be reset to EPOCH_DURATION if it’s zero here

## Assessed type

Context LSDan (judge) decreased severity to Medium 0xRektora (Tapioca) confirmed, but disagreed with severity and commented via duplicate Issue #165:

Low. While true, the probabilities of this happening are very thin.

Note: For full discussion, see here.

# [M-22] MagnetarHelper.getFractionForAmount uses the wrong rounding and will yield the wrong result

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-22
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

MagnetarHelper.getFractionForAmount uses the wrong rounding and will yield the wrong result Submitted by GalloDaSballo, also found by GalloDaSballo MagnetarHelper.getFractionForAmount is used to determine the fraction (shares) that will be received when an amount is deposited. The code needs to compute the totalShares and then determine what the fraction is going to be.

The logic is used in Singularity._removeAsset and since it’s tied to a withdrawal, the rounding will be down. However, in MagnetarHelper the round is up.

- https://github.com/Tapioca-DAO/tapioca-periph/blob/2ddbcb1cde03b548e13421b2dba66435d2ac8eb5/contracts/Magnetar/MagnetarHelper.sol#L208-L219
function getFractionForAmount ( ISingularity singularity, uint256 amount ) external view returns ( uint256 fraction ) { ( uint128 totalAssetShare, uint128 totalAssetBase ) = singularity.

totalAsset (); ( uint128 totalBorrowElastic,) = singularity.

totalBorrow (); uint256 assetId = singularity.

assetId (); IYieldBox yieldBox = IYieldBox ( singularity.

yieldBox ()); uint256 share = yieldBox.

toShare ( assetId, amount, false ); uint256 allShare = totalAssetShare + yieldBox.

toShare ( assetId, totalBorrowElastic, true ); /// @audit Round UP for Debt fraction = allShare == 0 ?

share: ( share * totalAssetBase ) / allShare; } See the actual implementation in Singularity:

- https://github.com/Tapioca-DAO/Tapioca-bar/blob/c2031ac2e2667ac8f9ac48eaedae3dd52abef559/contracts/markets/singularity/SGLCommon.sol#L199-L216
function _removeAsset ( address from, address to, uint256 fraction ) internal returns ( uint256 share ) { if ( totalAsset.

base == 0 ) { return 0; } Rebase memory _totalAsset = totalAsset; uint256 allShare = _totalAsset.

elastic + yieldBox.

toShare ( assetId, totalBorrow.

elastic, false ); share = ( fraction * allShare ) / _totalAsset.

base; } This will cause issues when calculating how much fraction to withdraw based on the amount required by the withdrawer.

## Mitigation

Change the rounding direction:

uint256 allShare = totalAssetShare + yieldBox.

toShare ( assetId, totalBorrowElastic, down );

## Assessed type

Math cryptotechmaker (Tapioca) confirmed and commented:

PR here.

# [M-23] Sending more TAP tokens to the TapToken.sol contract does not actually increase the total amount to be distributed

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-23
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

TapToken.sol contract does not actually increase the total amount to be distributed Submitted by deadrxsezzz Within the TapToken contract, the to-be-distributed balance is tracked by the dso_supply. Every week a percentage of is distributed to the users via oTAP emissions.

function emitForWeek () external onlyMinter returns ( uint256 ) { if ( _getChainId () != governanceEid ) revert NotValid (); uint256 week = _timestampToWeek ( block.

timestamp ); if ( emissionForWeek [ week ] > 0 ) return 0; // Compute unclaimed emission from last week and add it to the current week emission uint256 unclaimed; if ( week > 0 ) { // Update DSO supply from last minted emissions dso_supply -= mintedInWeek [ week - 1 ]; // Push unclaimed emission from last week to the current week unclaimed = emissionForWeek [ week - 1 ] - mintedInWeek [ week - 1 ]; } uint256 emission = _computeEmission (); emission += unclaimed; // Boosted TAP is burned and added to the emission to be minted on demand later on in `extractTAP()` uint256 boostedTAP = balanceOf ( address ( this )); if ( boostedTAP > 0 ) { _burn ( address ( this ), boostedTAP ); emission += boostedTAP; // Add TAP in the contract as boosted TAP emit BoostedTAP ( boostedTAP ); } emissionForWeek [ week ] = emission; emit Emitted ( week, emission ); return emission; } If we look at the code, we’ll see that there’s a check if there’s extra tokens within the contract and distribute them if that’s the case. The problem is that if such funds are distributed, they’ll still be deducted from the dso_supply which will mean that these tokens will not be treated as an extra.

What they’ll do is actually speed up the regular distribution and break the linearly decaying curve of weekly emissions.

Note: From the same core of the problem, there could be an underflow in dso_supply (if there’s enough Boost Tap sent when the weekly emissions have become low enough). This would block rolling over a week, within the TapToken and TapiocaOptionBroker contracts, though no funds will be locked.

## Recommended Mitigation Steps

Fix accounting for extra tap tokens sent.

## Assessed type

Context 0xRektora (Tapioca) confirmed and commented:

Nice catch, mintedInWeek is increased in extractTAP(). If boosted with TAP sent from outside the DSO it actually reduces its supply.

# [M-24] Unordered Nonces open up to further MEV risks

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-24
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by GalloDaSballo While a great effort was made to mitigate out of order nonce execution, the reality is that since PermitC allows unordered nonces, then nonces could be executed out of order. This opens up more problems than the original Permit Exploit. The original Permit exploit allowed to effectively always make multi-operations revert. This new scenario instead allows to use the order of nonces that will cause the most damage.

The most basic example would be using the reverse order to keep allowances to be non-zero. This may open up to more exploits, an example being claiming TAP, twTAP or rewards to router addresses by keeping the allowance as non-zero. Additionally, any time more than one Permit is issued, a race condition is available. This race condition would allow an exploiter to consume the nonces out of order, and then causes the transaction to revert. This, in contrast to the previous iteration, allows all possible sequences of combinations of allowances to determine the final result, while the original permit operation would allow only the proper sequence to be executed (with pauses between each step).

This additional risk can create scenarios in which outstanding allowance is left, which could incorrectly signal the willingness to claim tokens or the willingness to allow a target to bridge funds at a different time than intended. It’s worth reiterating how Signatures can be executed as soon as made available on any mempool, whether a L2 or LayerZeros system.

## Mitigation

The only solution I have found is to ensure that all “Macros” in Magnetar always approve a specific contract at most once. Revokes are done without signatures, either by consuming the allowance via a transferFrom (which may not be possible) or by introducing a function that allows an operator to reset their allowance.

## Assessed type

MEV 0xRektora (Tapioca) acknowledged and commented:

If I remember correctly, the team from Limit Break did not build PermitC to have batched transaction, but it can be dangerous if implemented wrong. Will forward the message.

As for Tapioca, we do have a batch function in Pearlmit that will force the signatures to be verified against the order they were sent to.

We believe the solution to the griefing scenario would be to use something like permitTransferFromWithAdditionalDataERC20 on said batch, by effectively binding the whole Tx (approvals paired with compose messages) and force the approval to be executed only in the case, which would nullify the front-running incentives.

0xRektora (Tapioca) commented:

Limit Break answer — Yes - as designed it’s explicitly allowed to use unordered nonces, so this would be valid. A modification could be made to the _checkAndInvalidateNonce function if you want to do ordered where you have a tracker by account and make sure that nonce == nextNonce[owner] but we don’t want that for our base case as unordered nonces are required to execute orders out of sequence.

# [M-25] Same contract multi permits fundamentally cannot be solved via the chosen standards

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-25
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by GalloDaSballo, also found by GalloDaSballo ( 1, 2 ), KIntern_NA, carrotsmuggler, bin2chen, and cccz The finding is a direct follow up to:

All cross-chain USDO and TOFT flows using approvals may be susceptible to permit-based DoS griefing from the Spearbit report.

Since permits are signatures, they will be available to anyone monitoring the chain. They will be usable by anyone, since the goal of permit is allowing some other msg.sender to broadcast the signature and have it work.

## Mitigation

The only solution I have found at this time would be to use Permits to grant approvals (with try-catch) and not using permits to revoke approvals, as the revoke permit call could be front-run causing all xChain calls to revert.

If you wish to use exact approvals xChain (which I recommend), you’d have to solve for rounding errors when converting shares vs amounts. Due to this, you may recommend people to grant higher allowances, and then change all toft tokens to have a renounceAllowance function, which would re-set the allowance on behalf of the operator, enforcing a strict 0 -> X -> 0 allowance pattern while avoiding front-run griefs.

This would ensure that trusted Tapioca Operators receive allowances, and re-set them at the end of all of their operations, which gives a stronger security guarantee. We built something similar for eBTC, with PositionMangers here.

## Assessed type

MEV cryptotechmaker (Tapioca) confirmed and commented via duplicate Issue #83:

PR here.

# [M-26] Incorrect decoding in decodeLockTwpTapDstMsg

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-26
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

decodeLockTwpTapDstMsg Submitted by GalloDaSballo, also found by KIntern_NA The decoding applied in decodeLockTwpTapDstMsg is incorrect as there are more than one combination of bytes that would result in the same result.

This is due to:

uint96 duration = BytesLib.toUint96(BytesLib.slice(_msg, userOffset_, durationOffset_), 0);, which uses length == durationOffset_ which is 32 instead of 12.

uint256 amount = BytesLib.toUint256(BytesLib.slice(_msg, durationOffset_, _msg.length - durationOffset_), 0); uses the length of the message, instead of 32 which would be the maximum size of a u256.

## Mitigation

Change:

uint96 duration = BytesLib.toUint96(BytesLib.slice(_msg, userOffset_, durationOffset_), 0); to uint96 duration = BytesLib.toUint96(BytesLib.slice(_msg, userOffset_, 12), 0);, which will prevent reading the wrong area of memory.

Change:

uint256 amount = BytesLib.toUint256(BytesLib.slice(_msg, durationOffset_, _msg.length - durationOffset_), 0); to uint256 amount = BytesLib.toUint256(BytesLib.slice(_msg, durationOffset_, 32), 0);, which will ensure that the bytes being read are the length of the message, instead of 32 which would be the maximum size of a u256.

## Assessed type

en/de-code cryptotechmaker (Tapioca) confirmed via duplicate Issue #144

# [M-27] Cross chain messages in MagnetarAssetXChainModule and MagnetarMintXChainModule will not work

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-27
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

MagnetarAssetXChainModule and MagnetarMintXChainModule will not work Submitted by rvierdiiev, also found by KIntern_NA In order to send LZ message MagnetarBaseModule._withdrawToChain function is called. This function allows to include composed message only if data.unwrap is set to true. In this case _lzCustomWithdraw function will be used, which will include composed message.

In case if !data.unwrap, then _lzWithdraw function is called, which calls _prepareLzSend function, which includes empty composed message. If you want to include composed message, then you should set data.unwrap as true.

Now, let’s look into MagnetarMintXChainModule.mintBBLendXChainSGL function, which passes false. Then look into MagnetarAssetXChainModule.depositYBLendSGLLockXchainTOLP function, which passes false.

As both of them pass data.unwrap as false, it means that compose message will not be crafted and this cross chain functionality will not work.

## Impact

It will be not possible to min usdo on one chain and lend it to singularity on another chain.

Tools Used VsCode

## Recommended Mitigation Steps

Pass data.unwrap` as true.

## Assessed type

Error cryptotechmaker (Tapioca) confirmed and commented:

PR here.

# [M-28] MagnetarMintXChainModule will not work as msg type is not allowed

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-28
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

MagnetarMintXChainModule will not work as msg type is not allowed Submitted by rvierdiiev Users can mint USDO on chain A, then lend this USDO to singularity on chain B and lock singularity tokens to be able to get tap options on chain C.

The first step of this flow is inside MagnetarMintXChainModule.mintBBLendXChainSGL function. This function mints USDO and then initiates LZ message to chain B to process second step using _withdrawToChain function.

When message will be received by USDO on chain B, then receiver will handle compose message and MSG_DEPOSIT_LEND_AND_SEND_FOR_LOCK msg type should be provided to process it correctly. The problem is that USDO will never receive such msg type as it is not allowed.

Let’s check how _withdrawToChain function works on chain A. In order to request compose message it should call _lzCustomWithdraw. This function then creates instance of TapiocaOmnichainEngineHelper contract, which will be used to prepare LZ message.

When TapiocaOmnichainEngineHelper will build compose message it will call _sanitizeMsgType function with msg type that is going to be sent.

- https://github.com/Tapioca-DAO/tapioca-periph/blob/032396f701be935b04a7e5cf3cb40a0136259dbc/contracts/tapiocaOmnichainEngine/extension/TapiocaOmnichainEngineHelper.sol#L333-L346
function _sanitizeMsgType ( uint16 _msgType ) internal pure { if ( // LZ _msgType == MSG_SEND // Tapioca msg types || _msgType == MSG_APPROVALS || _msgType == MSG_NFT_APPROVALS || _msgType == MSG_PEARLMIT_APPROVAL || _msgType == MSG_REMOTE_TRANSFER || _msgType == MSG_YB_APPROVE_ASSET || _msgType == MSG_YB_APPROVE_ALL || _msgType == MSG_MARKET_PERMIT ) { return; } else if (!

_sanitizeMsgTypeExtended ( _msgType )) { revert InvalidMsgType ( _msgType ); } As you can see this function allows only some types and other should be handled by _sanitizeMsgTypeExtended function and this function is empty and returns false. It is designed to be extended by other helpers, such as UsdoHelper.

But as _lzCustomWithdraw always creates instance of TapiocaOmnichainEngineHelper it means that some messages will be not allowed and will not work. Thus, whole minting flow that I have described on the beginning won’t work.

## Impact

Users can’t mint on one chain and deposit on another.

Tools Used VsCode

## Recommended Mitigation Steps

For different oft token you should use different helper. For example, if message is going to be sent to USDO, then UsdoHelper should be used; if message comes to tOft, then ToftHelper is needed.

Or you can set all approved messaged in TapiocaOmnichainEngineHelper instead, then you can leave current design of MagnetarBaseModule.

## Assessed type

Error cryptotechmaker (Tapioca) disputed and commented:

Invalid. The following extends the default behavior, see here.

rvierdiiev (warden) commented:

Sponsor said that issue is invalid, because TapiocaOmnichainReceiver has _toeComposeReceiver function to handle this. This is incorrect.

TapiocaOmnichainReceiver.sol is the contract that is responsible for receiving message on another chain. But in this issue I have described the fact that initiating of request on source chain will fail, so destination chain and TapiocaOmnichainReceiver will not even receive it, because the source chain will not allow to send such message.

I ask the judge and sponsor to go through the issue one more time with the whole message flow that I have described to see, that call will not be executed.

# [M-29] AirdropBroker : Airdrops in epoch 4 can participate and exercise options in subsequent epochs

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-29
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

AirdropBroker: Airdrops in epoch 4 can participate and exercise options in subsequent epochs Submitted by cccz In AirdropBroker, the owner calls registerUsersForPhase with _phase = 4 to airdrop to users in epoch 4 to 8.

else if ( _phase == 4 ) { for ( uint256 i; i < _users.

length; i ++) { phase4Users [ _users [ i ]] = _amounts [ i ]; } And the documentation says:

Phase Four will have five sub phases, each bearing one week epochs. Four of the sub phases will be rewarded to twTAP lockers each weekly epoch over four weeks. Unclaimed aoTAP in each epoch will roll over to the next epoch, until the final sub phase.

The problem here is that phase4Users is not cleared before the start of the 5 sub-phases of phase 4, which results in users being able to participate and exercise the epoch 4 options in epoch 5.

} else if ( cachedEpoch >= 4 ) { aoTAPTokenID = _participatePhase4 (); }...

function _participatePhase4 () internal returns ( uint256 oTAPTokenID ) { uint256 _eligibleAmount = phase4Users [ msg.

sender ]; if ( _eligibleAmount == 0 ) revert NotEligible (); // Close eligibility phase4Users [ msg.

sender ] = 0; Consider the following scenario. There are 10,000 aoTAPs in phase 4, which means that epoch 4, 5, 6, and 7 will receive 2,500 aoTAPs respectively, and the remaining unclaimed aoTAPs will be rolled over to the next epoch.

In epoch 4, Alice received 1000 airdrops, but Alice did not participate.

At epoch 5, the total airdrop will become 2500 + 1000 = 3500 and distributed.

However. Alice can participate and exercise the epoch 4 options in epoch 5, and making the total airdrop in epoch 5 become 3500 + 1000 = 4500.

## Recommended Mitigation Steps

It is recommended to delete phase4Users in newEpoch().

- https://ethereum.stackexchange.com/questions/15553/how-to-delete-a-mapping
Or, add phase4Users1 / phase4Users2 / phase4Users3 / phase4Users4 / phase4Users5 mappings to store the airdrops for epochs 4 through 8, respectively.

## Assessed type

Context LSDan (judge) decreased severity to Low cccz (warden) commented:

I believe this is a valid M. It’s not any admin error stuff. It’s a code level error, the code uses the same variables to manage airdrops at different epochs.

The only way to get rid of it is if the admin calls newEpoch() to enter a new epoch while resetting all the phase4Users variables, but newEpoch() is a public function and anyone can call it, which makes it hard for the admin to get rid of it.

function newEpoch () external tapExists { if ( block.

timestamp < lastEpochUpdate + EPOCH_DURATION ) { revert TooSoon (); } // Update epoch info lastEpochUpdate = uint64 ( block.

timestamp ); epoch ++; // At epoch 4, change the epoch duration to 7 days if ( epoch == 4 ) { EPOCH_DURATION = 7 days; } // Get epoch TAP valuation ( bool success, uint256 _epochTAPValuation ) = tapOracle.

get ( tapOracleData ); if (!

success ) revert Failed (); epochTAPValuation = uint128 ( _epochTAPValuation ); emit NewEpoch ( epoch, epochTAPValuation ); } And as the report says, using different variables to manage airdrops for different epochs is the right way.

LSDan (judge) increased severity to Medium and commented:

Agreed. This makes sense as a Medium.

0xWeiss (Tapioca) confirmed Note: For full discussion, see here.

# [M-30] AirdropBroker : When block.timestamp == lastEpochUpdate + EPOCH_DURATION , users can exercise options in the new epoch.

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-30
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

AirdropBroker: When block.timestamp == lastEpochUpdate + EPOCH_DURATION, users can exercise options in the new epoch.

Submitted by cccz, also found by bin2chen In AirdropBroker, when a user receives an airdrop, it should only be able to exercise the options in the current epoch.

function _participatePhase1 () internal returns ( uint256 oTAPTokenID ) { uint256 _eligibleAmount = phase1Users [ msg.

sender ]; if ( _eligibleAmount == 0 ) revert NotEligible (); // Close eligibility phase1Users [ msg.

sender ] = 0; // Mint aoTAP uint128 expiry = uint128 ( lastEpochUpdate + EPOCH_DURATION ); // Set expiry to the end of the epoch oTAPTokenID = aoTAP.

mint ( msg.

sender, expiry, uint128 ( PHASE_1_DISCOUNT ), _eligibleAmount ); }...

function exerciseOption ( uint256 _aoTAPTokenID, ERC20 _paymentToken, uint256 _tapAmount ) external whenNotPaused tapExists { // Load data (, AirdropTapOption memory aoTapOption ) = aoTAP.

attributes ( _aoTAPTokenID ); if ( aoTapOption.

expiry < block.

timestamp ) revert OptionExpired (); For example, Alice received the airdrop for epoch 1, then Alice can participate in epoch 1 and exercise the options in epoch 1. When Alice exercises her option, the TAP price is determined at the beginning of epoch 1.

The problem here is that when block.timestamp == lastEpochUpdate + EPOCH_DURATION, newEpoch() can be called to enter epoch 2, and exerciseOption() can also be called to exercise the option of epoch 1. This allows the user to exercise the epoch 1 option at the epoch 2 TAP price.

function newEpoch () external tapExists { if ( block.

timestamp < lastEpochUpdate + EPOCH_DURATION ) { revert TooSoon (); }...

function exerciseOption ( uint256 _aoTAPTokenID, ERC20 _paymentToken, uint256 _tapAmount ) external whenNotPaused tapExists { // Load data (, AirdropTapOption memory aoTapOption ) = aoTAP.

attributes ( _aoTAPTokenID ); if ( aoTapOption.

expiry < block.

timestamp ) revert OptionExpired (); Consider the following scenario, Alice receives a 1000 options airdrop for epoch 1.

Epoch 1 starts, lastEpochUpdate = day 0, TAP price is 5 USD, and TAP price is in a downward trend. If Alice exercises the option in epoch 1, she needs to pay 5000 * 0.5 = 2500 USD.

However, when block.timestamp == lastEpochUpdate + EPOCH_DURATION, the current TAP price is 2 USD, Alice can call newEpoch() and exerciseOption() in one transaction, Alice will exercise the option at the price of epoch 2, and only needs to pay 1000 USD to get 1000 TAP.

## Recommended Mitigation Steps

It is recommended that exercise is not allowed when block.timestamp == lastEpochUpdate + EPOCH_DURATION.

function exerciseOption(uint256 _aoTAPTokenID, ERC20 _paymentToken, uint256 _tapAmount) external whenNotPaused tapExists { // Load data (, AirdropTapOption memory aoTapOption) = aoTAP.attributes(_aoTAPTokenID); - if (aoTapOption.expiry < block.timestamp) revert OptionExpired(); + if (aoTapOption.expiry <= block.timestamp) revert OptionExpired();

## Assessed type

Context 0xRektora (Tapioca) confirmed, but disagreed with severity and commented:

it’s a good catch, however this is informational. Probability of this happening are very low. The epoch is called with an only owner function, so it can’t be done within the same Tx, as for the same block, since this happens on Arbitrum with the fair sequencing and low latency, the chances are close to 0. On top of the incentives are extremely low for the effort being made.

LSDan (judge) decreased severity to Low cryptotechmaker (Tapioca) commented:

PR here.

cccz (warden) commented:

@0xRektora - I disagree.

newEpoch() doesn’t have any modifiers like onlyOwner, i.e.

newEpoch() can be called by anyone and immediately follow the exerciseOption() call.

function newEpoch () external tapExists { if ( block.

timestamp < lastEpochUpdate + EPOCH_DURATION ) { revert TooSoon (); } // Update epoch info lastEpochUpdate = uint64 ( block.

timestamp ); epoch ++; // At epoch 4, change the epoch duration to 7 days if ( epoch == 4 ) { EPOCH_DURATION = 7 days; } // Get epoch TAP valuation ( bool success, uint256 _epochTAPValuation ) = tapOracle.

get ( tapOracleData ); if (!

success ) revert Failed (); epochTAPValuation = uint128 ( _epochTAPValuation ); emit NewEpoch ( epoch, epochTAPValuation ); }

# [M-31] sendParam.minAmountLD slippage setting is too strict

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-31
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

sendParam.minAmountLD slippage setting is too strict Submitted by ladboy233 Across the codebase, the minAmonutLD and amountLD is set to equal value:

if (data.collateralAmount > 0) { address collateralWithdrawReceiver = data.withdrawCollateralParams.withdraw ? address(this): data.user; uint256 collateralShare = _yieldBox.toShare(_market.collateralId(), data.collateralAmount, false); (Module[] memory modules, bytes[] memory calls) = IMarketHelper(data.marketHelper).removeCollateral( data.user, collateralWithdrawReceiver, collateralShare ); _market.execute(modules, calls, true); //withdraw if (data.withdrawCollateralParams.withdraw) { uint256 collateralId = _market.collateralId(); if (data.withdrawCollateralParams.assetId != collateralId) revert Magnetar_WithdrawParamsMismatch(); // @dev re-calculate amount if (collateralShare > 0) { uint256 computedCollateral = _yieldBox.toAmount(collateralId, collateralShare, false);

if (computedCollateral == 0) revert Magnetar_WithdrawParamsMismatch(); data.withdrawCollateralParams.lzSendParams.sendParam.amountLD = computedCollateral; data.withdrawCollateralParams.lzSendParams.sendParam.minAmountLD = computedCollateral; _withdrawToChain(data.withdrawCollateralParams); } However, minAmonutLD served as a slippage control on layerzero v2 side, and 0% slippage is not always possible. The cross-chain transaction will always reverted in too strict slippage contract and block asset transfer.

- https://github.com/LayerZero-Labs/LayerZero-v2/blob/142846c3d6d51e3c2a0852c41b4c2b63fcda5a0a/oapp/contracts/oft/OFTCore.sol#L345
function _debitView( uint256 _amountLD, uint256 _minAmountLD, uint32 /*_dstEid*/ ) internal view virtual returns (uint256 amountSentLD, uint256 amountReceivedLD) { // @dev Remove the dust so nothing is lost on the conversion between chains with different decimals for the token.

amountSentLD = _removeDust(_amountLD); // @dev The amount to send is the same as amount received in the default implementation.

amountReceivedLD = amountSentLD; // @dev Check for slippage.

if (amountReceivedLD < _minAmountLD) { revert SlippageExceeded(amountReceivedLD, _minAmountLD); }

## Recommended Mitigation Steps

Let user input a percentage and compute minAmountLD based on a percentage of slippage user is willing to take risk of.

## Assessed type

Token-Transfer cryptotechmaker (Tapioca) confirmed, but disagreed with severity and commented:

Slippage is only for EVM to Non-EVM transfers. We’re good for now.

# [M-32] Layerzero fee refund address is not handled correctly

- **Contest:** Tapioca Invitational 
- **Slug:** 2024-02-tapioca-invitational
- **Finding ID:** M-32
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-tapioca-invitational
- **Source snapshot:** competitions/2024-02-tapioca-invitational/final_report.html

Submitted by ladboy233 The protocol aims to integrate with layerzero v2. To leverage layerzero infrastructure to send out cross-chain message, the user has to pay the native fee. If the user underpays the message fee, transaction reverts. If the user overpays the message fee, the excessive fee is refunded back.

However, in the current implementation, the refund address is not compose correctly.

In MagnetarBaseModule.sol, there is a function:

function _lzCustomWithdraw( address _asset, LZSendParam memory _lzSendParam, uint128 _lzSendGas, uint128 _lzSendVal, uint128 _lzComposeGas, uint128 _lzComposeVal, uint16 _lzComposeMsgType ) private { PrepareLzCallReturn memory prepareLzCallReturn = _prepareLzSend(_asset, _lzSendParam, _lzSendGas, _lzSendVal); TapiocaOmnichainEngineHelper _toeHelper = new TapiocaOmnichainEngineHelper(); PrepareLzCallReturn memory prepareLzCallReturn2 = _toeHelper.prepareLzCall( ITapiocaOmnichainEngine(_asset), PrepareLzCallData({ dstEid: _lzSendParam.sendParam.dstEid, recipient: _lzSendParam.sendParam.to, amountToSendLD: 0, minAmountToCreditLD: 0, msgType: _lzComposeMsgType, composeMsgData: ComposeMsgData({ index: 0,

gas: _lzComposeGas, value: prepareLzCallReturn.msgFee.nativeFee.toUint128(), data: _lzSendParam.sendParam.composeMsg, prevData: bytes(""), prevOptionsData: bytes("") }), lzReceiveGas: _lzSendGas + _lzComposeGas, lzReceiveValue: _lzComposeVal }) ); if (msg.value < prepareLzCallReturn2.msgFee.nativeFee) { revert Magnetar_GasMismatch(prepareLzCallReturn2.msgFee.nativeFee, msg.value); } IOftSender(_asset).sendPacket{value: prepareLzCallReturn2.msgFee.nativeFee}( prepareLzCallReturn2.lzSendParam, prepareLzCallReturn2.composeMsg ); } First, we are creating a temp TapiocaOmnichainEngineHelper contract, then calling prepareLzCall to compose the data type. The function is long, but the important thing is

that the refundAddress is set to address(msg.sender).

lzSendParam_ = LZSendParam({ sendParam: sendParam_, fee: msgFee_, extraOptions: oftMsgOptions_, refundAddress: address(msg.sender) }); prepareLzCallReturn_ = PrepareLzCallReturn({ composeMsg: composeMsg_, composeOptions: composeOptions_, sendParam: sendParam_, msgFee: msgFee_, lzSendParam: lzSendParam_, oftMsgOptions: oftMsgOptions_ }); Who is msg.sender ?

In this case:

User calls contract A, Contract A creates contract B, Contract A calls contract B prepareLzCall method.

Inside the prepareLzCall function call, msg.sender will be address contract A.

However, that is not what we want. The refunded fee should go to original msg.sender who triggered the withdraw and paid the native fee.

## Recommended Mitigation Steps

Set the layerzero refund address to a user input address:

lzSendParam_ = LZSendParam({ sendParam: sendParam_, fee: msgFee_, extraOptions: oftMsgOptions_, refundAddress: refundAddress // change here }); prepareLzCallReturn_ = PrepareLzCallReturn({ composeMsg: composeMsg_, composeOptions: composeOptions_, sendParam: sendParam_, msgFee: msgFee_, lzSendParam: lzSendParam_, oftMsgOptions: oftMsgOptions_ });

## Assessed type

Token-Transfer 0xRektora (Tapioca) confirmed cryptotechmaker (Tapioca) commented:

Main PR here.

Secondary PRs here, here and here.

## Rejected Primary Findings

# Rejected Primary Findings: Tapioca Invitational 

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
