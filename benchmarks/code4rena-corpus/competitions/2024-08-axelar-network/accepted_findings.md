# Accepted H/M Findings: Axelar Network

# [H-01] Bridge requests to remote chains where interchain tokens are not deployed can result in DoS attacks

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-axelar-network
- **Source snapshot:** competitions/2024-08-axelar-network/final_report.html

Submitted by klau5, also found by gjaldon and 0x007

- https://github.com/code-423n4/2024-08-axelar-network/blob/4572617124bed39add9025317d2c326acfef29f1/axelar-amplifier/interchain-token-service/src/contract/execute.rs#L112-L135
- https://github.com/code-423n4/2024-08-axelar-network/blob/4572617124bed39add9025317d2c326acfef29f1/axelar-amplifier/interchain-token-service/src/state.rs#L192

## Impact

Insufficient balance in the ITSHub prevents bridging, leading to a DoS attack on the bridge.

## Recommended Mitigation Steps

The problem is allowing bridging to chains where the remote interchain token is not deployed. The implementation of update_token_balance should be changed to allow bridging only to chains where the balance has been initialized.

If you don’t want to track the balance of the source chain, instead of judging by balance initialization, you should accurately identify the source chain and handle it.

## Assessed type

DoS milapsheth (Axelar) confirmed and commented:

The report is valid. We agree with the Medium severity since assets can’t be stolen, and ITS hub can be upgraded easily to fix the balance invariant. We do plan to restrict transfers if the token isn’t initialized yet to handle this scenario. As mentioned, it requires more careful tracking of the source chain of the token.

0xsomeone (judge) increased severity to High and commented:

The Warden outlines a discrepancy in the token accounting system of the ITS service that would permit a chain that does not yet have an interchain token deployment in it to process messages for it incorrectly.

The vulnerability stems from the same root cause as submission #77, and I believe both descriptions merit a high severity rating. This particular submission directly outlines how funds could be stolen by front-running the deployment of an interchain token and bridging back, thereby causing funds bridged by other users to become “locked” in the attacked chain.

# [H-02] Can block bridge or limit the bridgeable amount by initializing the ITSHub balance of the original chain

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-axelar-network
- **Source snapshot:** competitions/2024-08-axelar-network/final_report.html

Submitted by klau5

- https://github.com/code-423n4/2024-08-axelar-network/blob/4572617124bed39add9025317d2c326acfef29f1/interchain-token-service/contracts/InterchainTokenFactory.sol#L176
- https://github.com/code-423n4/2024-08-axelar-network/blob/4572617124bed39add9025317d2c326acfef29f1/interchain-token-service/contracts/InterchainTokenService.sol#L342
- https://github.com/code-423n4/2024-08-axelar-network/blob/4572617124bed39add9025317d2c326acfef29f1/interchain-token-service/contracts/InterchainTokenFactory.sol#L269

## Impact

Token deployers can block the token bridge or limit the bridgeable amount without having Operator or FlowLimiter permissions. Especially, Canonical tokens can be attacked by anyone, not just the token register.

## Recommended Mitigation Steps

In InterchainTokenFactory.deployRemoteInterchainToken, check if destinationChain is the same as originalChainName to prevent remote deployment requests to the original chain.

function deployRemoteInterchainToken( string calldata originalChainName, bytes32 salt, address minter, string memory destinationChain, uint256 gasValue ) external payable returns (bytes32 tokenId) { string memory tokenName; string memory tokenSymbol; uint8 tokenDecimals; bytes memory minter_ = new bytes(0); { bytes32 chainNameHash_; if (bytes(originalChainName).length == 0) { chainNameHash_ = chainNameHash; } else { chainNameHash_ = keccak256(bytes(originalChainName)); } + require(chainNameHash_ != keccak256(bytes(destinationChain)), "Cannot remote deploy on original chain"); address sender = msg.sender; salt = interchainTokenSalt(chainNameHash_, sender, salt); tokenId = interchainTokenService.interchainTokenId(TOKEN_FACTORY_DEPLOYER, salt);

IInterchainToken token = IInterchainToken(interchainTokenService.interchainTokenAddress(tokenId)); tokenName = token.name(); tokenSymbol = token.symbol(); tokenDecimals = token.decimals(); if (minter != address(0)) { if (!token.isMinter(minter)) revert NotMinter(minter); minter_ = minter.toBytes(); } tokenId = _deployInterchainToken(salt, destinationChain, tokenName, tokenSymbol, tokenDecimals, minter_, gasValue); } Check originalChainName and destinationChain in InterchainTokenFactory.deployRemoteCanonicalInterchainToken to ensure that a remote deploy cannot be requested to the original chain.

When deploying by InterchainTokenService.deployInterchainToken directly, it’s not possible to check if the original chain and destination chain are the same. Store information about the original chain for each tokenId in ITSHub, and ensure that the balance of the original chain is not initialized.

## Assessed type

Invalid Validation milapsheth (Axelar) confirmed and commented:

The report is valid. We consider this a Medium severity issue, however, since assets can’t be stolen. It’s a DOS issue that prevents canonical tokens from being transferred from the origin chain to other chains. ITS hub can be upgraded easily to handle the scenario where source_chain == destination_chain and fix the balance for the source chain, so the impact is low.

0xsomeone (judge) commented:

The Warden has identified a mechanism via which bridging of canonical tokens can be permanently DoSd.

Upgrades of contract logic are not considered appropriate mitigations for vulnerabilities such as the one described (as we could effectively resolve any and all vulnerabilities identified via upgrades), so I believe that a high-risk severity rating is appropriate for this submission.

milapsheth (Axelar) commented:

@0xsomeone - In the case of a bug that allows stealing user funds, an upgrade could be too late if the issue is already exploited. Whereas if the DoS issue reported here is triggered, an upgrade can fix the issue without loss of funds. Furthermore, ITS Hub is designed to be upgradable (a non upgradable contract would have made this issue more severe). For this reason, we consider this to be a Medium severity issue since the impact is much lower than a compromise of user funds.

0xsomeone (judge) commented:

@milapsheth - While from a practical perspective you might consider this to be a medium-severity issue, C4 guidelines are clear that contract upgrades cannot be utilized to mitigate the severity of a vulnerability (i.e. we consider this DoS irrecoverable). As such, this vulnerability’s severity will remain from a C4 audit perspective.

Medium Risk Findings (2)

# [M-01] Axelar cross chain token transfers balance tracking logic is completely broken for rebasing tokens and the transfers of these type of tokens can be exploited

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-axelar-network
- **Source snapshot:** competitions/2024-08-axelar-network/final_report.html

Submitted by Bauchibred, also found by mxuse, trachev, grearlake, ayden, ZanyBonzy, gjaldon, and jasonxiale NB: This report is a 2 in 1, first sub section shows how the transfers would be exploited when there is a rebase (focusing on the solidity implementation of the Hub) and the second sub section showcases how the balance tracking logic would be completely broken for rebasing tokens (focusing on the Rust implementation of the ITS Hub).

First, it would be key to note that per the scope of the audit we should count rebasing tokens (whose balance change outside of transfers) in scope. See what’s been stated in the readMe.

Question Answer ERC20 used by the protocol Any (all possible ERC20s) Balance changes outside of transfers In scope Sub-section 1 There are different token manager types that take care of the different types of tokens that get integrated to the system, i.e., here.

enum TokenManagerType { NATIVE_INTERCHAIN_TOKEN, // This type is reserved for interchain tokens deployed by ITS, and can't be used by custom token managers.

MINT_BURN_FROM, // The token will be minted/burned on transfers. The token needs to give mint permission to the token manager, but burning happens via an approval.

LOCK_UNLOCK, // The token will be locked/unlocked at the token manager.

LOCK_UNLOCK_FEE, // The token will be locked/unlocked at the token manager, which will account for any fee-on-transfer behaviour.

MINT_BURN, // The token will be minted/burned on transfers. The token needs to give mint and burn permission to the token manager.

GATEWAY // The token will be sent throught the gateway via callContractWithToken } Now whereas this logic includes a type for fee-on-transfer tokens, there is no logic for supporting rebasing tokens.

Now from the readMe, we can see that protocol plans to integrate rebasing tokens. Since these tokens do not charge fees during transfers, the method as to which their integration would be done, would be via LOCK_UNLOCK.

However, the problem is that after the initial lock at the token manager, there could be a positive/negative rebase of the token before the unlock which would then mean that the amount of tokens transferred in via takeToken() would have changed by the time giveToken() is to be called to send out these tokens:

- https://github.com/code-423n4/2024-08-axelar-network/blob/69c4f2c3fcefb1b8eb2129af9c3685a44ae5b6fe/interchain-token-service/contracts/TokenHandler.sol#L91-L123
function takeToken ( bytes32 tokenId, bool tokenOnly, address from, uint256 amount ) external payable returns ( uint256, string memory symbol ) { address tokenManager = _create3Address ( tokenId ); ( uint256 tokenManagerType, address tokenAddress ) = ITokenManagerProxy ( tokenManager ).

getImplementationTypeAndTokenAddress (); if ( tokenOnly && msg.

sender != tokenAddress ) revert NotToken ( msg.

sender, tokenAddress );..

snip } else if ( tokenManagerType == uint256 ( TokenManagerType.

LOCK_UNLOCK )) { _transferTokenFrom ( tokenAddress, from, tokenManager, amount );..

snip /// @dev Track the flow amount being sent out as a message ITokenManager ( tokenManager ).

addFlowOut ( amount ); return ( amount, symbol ); }

- https://github.com/code-423n4/2024-08-axelar-network/blob/69c4f2c3fcefb1b8eb2129af9c3685a44ae5b6fe/interchain-token-service/contracts/TokenHandler.sol#L177-L180
function _transferTokenFrom ( address tokenAddress, address from, address to, uint256 amount ) internal { // slither-disable-next-line arbitrary-send-erc20 IERC20 ( tokenAddress ).

safeTransferFrom ( from, to, amount ); }

- https://github.com/code-423n4/2024-08-axelar-network/blob/69c4f2c3fcefb1b8eb2129af9c3685a44ae5b6fe/interchain-token-service/contracts/TokenHandler.sol#L45-L80
function giveToken ( bytes32 tokenId, address to, uint256 amount ) external payable returns ( uint256, address ) {..

snip if ( tokenManagerType == uint256 ( TokenManagerType.

LOCK_UNLOCK )) { _transferTokenFrom ( tokenAddress, tokenManager, to, amount ); return ( amount, tokenAddress ); }..

snip } Now this would be problematic when we consider the integration in the InterchainTokenService, when processing interchain transfers, since the payload attached to the execution would include the right amount of tokens to be sent:

- https://github.com/code-423n4/2024-08-axelar-network/blob/69c4f2c3fcefb1b8eb2129af9c3685a44ae5b6fe/interchain-token-service/contracts/InterchainTokenService.sol#L733-L739
function _processInterchainTransferPayload ( bytes32 commandId, address expressExecutor, string memory sourceChain, bytes memory payload ) internal { bytes32 tokenId; bytes memory sourceAddress; address destinationAddress; uint256 amount; bytes memory data; { bytes memory destinationAddressBytes; (, tokenId, sourceAddress, destinationAddressBytes, amount, data ) = abi.

decode ( payload, ( uint256, bytes32, bytes, bytes, uint256, bytes ) ); destinationAddress = destinationAddressBytes.

toAddress (); } // Return token to the existing express caller if ( expressExecutor != address ( 0 )) { // slither-disable-next-line unused-return _giveToken ( tokenId, expressExecutor, amount ); return; } address tokenAddress; ( amount, tokenAddress ) = _giveToken ( tokenId, destinationAddress, amount );..

snip } But during the time frame where the transfer gets initiated to the time it gets processed, the amount specified on that asset might have changed; which would then cause for either the wrong amount of tokens to be given out or for the attempt at transfer to revert when being attempted to be given out here.

function _giveToken ( bytes32 tokenId, address to, uint256 amount ) internal returns ( uint256, address tokenAddress ) { ( bool success, bytes memory data ) = tokenHandler.

delegatecall ( abi.

encodeWithSelector ( ITokenHandler.

giveToken.

selector, tokenId, to, amount ) ); if (!

success ) revert GiveTokenFailed ( data ); ( amount, tokenAddress ) = abi.

decode ( data, ( uint256, address )); return ( amount, tokenAddress ); } Sub-section 2 When executing a message, there is a need to apply the balance tracking logic, see here.

pub fn execute_message ( deps: DepsMut, cc_id: CrossChainId, source_address: Address, payload: HexBinary, ) -> Result <Response, Error> { //..snip match its_hub_message { ItsHubMessage::SendToHub { destination_chain, message: its_message, } => { apply_balance_tracking ( //@audit deps.storage, source_chain.

clone (), destination_chain.

clone (), &its_message, )?; //..snip } _ => Err( report!

(Error::InvalidPayload)), } While applying the balance logic, this is documented pattern for interchainTransfers.

/// # Behavior for different ITS message types /// /// 1. InterchainTransfer:

/// - Decreases the token balance on the source chain.

/// - Increases the token balance on the destination chain.

/// - If the balance becomes insufficient on the source chain, an error is returned.

/// We can confirm this in the code snippets:

- https://github.com/code-423n4/2024-08-axelar-network/blob/69c4f2c3fcefb1b8eb2129af9c3685a44ae5b6fe/axelar-amplifier/interchain-token-service/src/contract/execute.rs#L101-L132
fn apply_balance_tracking ( storage: & mut dyn Storage, source_chain: ChainName, destination_chain: ChainName, message: &ItsMessage, ) -> Result <(), Error> { match message { ItsMessage::InterchainTransfer { token_id, amount,..

} => { // Update the balance on the source chain update_token_balance ( storage, token_id.

clone (), source_chain.

clone (), *amount, false, ).

change_context_lazy (|| Error::

BalanceUpdateFailed (source_chain, token_id.

clone ()))?; // Update the balance on the destination chain update_token_balance ( storage, token_id.

clone (), destination_chain.

clone (), *amount, true, ).

change_context_lazy (|| { Error::

BalanceUpdateFailed (destination_chain, token_id.

clone ()) })?

} //..snip } Evidently the update_token_balance() is being queried and in our case would be to the source chain and we’d have our is_deposited bool to be false since we are withdrawing, going to the implementation of update_token_balance() we can see here.

pub fn update_token_balance ( storage: & mut dyn Storage, token_id: TokenId, chain: ChainName, amount: Uint256, is_deposit:

bool, ) -> Result <(), Error> { let key = TokenChainPair { token_id, chain }; let token_balance = TOKEN_BALANCES.

may_load (storage, key.

clone ())?; match token_balance { Some(TokenBalance::

Tracked (balance)) => { let token_balance = if is_deposit { balance.

checked_add (amount).

map_err (|_| Error::MissingConfig)?

} else { balance.

checked_sub (amount) //@audit.

map_err (|_| Error::MissingConfig)?

}.

then (TokenBalance::Tracked); TOKEN_BALANCES.

save (storage, key.

clone (), &token_balance)?; } Some(_) | None => (), } Ok(()) } Evidently, there is a query of the last stored tracked balance, and an attempt to withdraw more than was last stored, would fail, due to the revert that occurs in rust’s checked_sub() implementation:

pub fn checked_sub ( self, other:

Self ) -> Result < Self, OverflowError> { self.

0.

checked_sub (other.

0 ).

map ( Self ).

ok_or_else (|| OverflowError::

new (OverflowOperation::Sub, self, other)) } However, this would be wrong for rebasing tokens, cause throughout the duration the token is existing, the tracked balance != the real balance this is because these tokens update their balances outside transfers and in the case where they’ve been multiple positive rebases. Then, we effectively have funds locked in the source chain, because attempting to withdraw what has been tracked, would always revert here

## Impact

Sub-section 1:

As hinted under Proof of Concept, in the case where a rebasing token is integrated then the amount specified in the payload is not necessarily the amount that needs to end up being sent to the user by the time the interchain attempt gets processed; which is because within the time frame the token might have rebased, positively or negatively:

If a positive rebase, this then means that the interchain transfers would unfairly leak value from the recipient, since they are to receive more than is specified in the payload.

Alternatively, if a negative rebase, this then means that the recipient is leaking value from the protocol/other users who are attempting to process interchain transfers on the same token.

Which could even open up a sneaky window for tech savvy users to steal value from protocol. This is because, for example, the LIDO stETH token normally rebases around 12pm, so an attacker can always watch the mempool on the rebase from LIDO around the time and in the case where the rebase is going to skim off or heavily increase the balances they can position themselves to make the most out of the transaction by frontrunning the rebase update with requesting an interchain. By the time this is executed, the balance would have already changed for the asset, effectively tricking the system and stealing value from other users.

Sub-section 2:

Asides the hinted cases above, if a token’s balance is tracked, then there is also no active method to ensure the right tracking is being done for this token, considering positive and negative rebases would occur. This would then mean that during interchain transfers we can have reverts when we should not, essentially meaning a chunk of the assets are going to be stuck/untransferrable from the source chain.

## Recommended Mitigation Steps

Sub-section 1:

Do not support these type of tokens, or integrate a new type and then have a way to check the amount of rebase that has happened since the users specified their transfer request.

enum TokenManagerType { NATIVE_INTERCHAIN_TOKEN, // This type is reserved for interchain tokens deployed by ITS, and can't be used by custom token managers.

MINT_BURN_FROM, // The token will be minted/burned on transfers. The token needs to give mint permission to the token manager, but burning happens via an approval.

LOCK_UNLOCK, // The token will be locked/unlocked at the token manager.

LOCK_UNLOCK_FEE, // The token will be locked/unlocked at the token manager, which will account for any fee-on-transfer behaviour.

+ LOCK_UNLOCK_REBASE, // The token will be locked/unlocked at the token manager, which will account for any positive/negative rebasing behaviour.

MINT_BURN, // The token will be minted/burned on transfers. The token needs to give mint and burn permission to the token manager.

GATEWAY // The token will be sent throught the gateway via callContractWithToken } Sub-section 2:

Do not track the balances for rebasing tokens and instead have them naturally revert here if not enough balance is available.

## Assessed type

ERC20 milapsheth (Axelar) disputed and commented via duplicate Issue #42:

ITS only supports tokens that can adhere to the requirements of the supported token managers. It’s known that rebasing tokens are not supported by bridging protocols directly. Their wrapped versions should be used (e.g., wstETH ). So, this is an out of scope requirement and we don’t really consider it an issue. It’s also been discussed during prior ITS audits.

0xsomeone (judge) decreased severity to Medium and commented:

The Warden has outlined that the system is inherently incompatible with rebasing tokens.

The audit’s README outlines that tokens with balance changes outside of transfers are in the scope of the audit. Even though it might be obvious that such tokens are not supported by bridges directly, the list of token behaviors in scope needs to align with those intentions.

I am inclined to retain a medium-risk severity rating for this submission due to outlining that the system is inherently incompatible with rebasing tokens even though it purported that it can support balance changes outside of transfers. I would like to note that this submission is considered valid from a C4 audit perspective rather than a technical perspective.

milapsheth (Axelar) commented:

ITS intentionally allows linking tokens with custom logic but it’s the user’s responsibility to ensure that the token’s custom logic is compatible with ITS’s token bridging model. ITS aims to be a permissionless and flexible protocol, but this requires users to have a good understanding when using it with non-standard ERC20s. This holds for any custom logic and not just rebasing tokens. A user can use ITS with rebasing tokens by registering it under the MINT_BURN token type on all chains with the token giving a mint role to the corresponding ITS token managers. What ITS guarantees is that one invalid token link doesn’t affect another correctly setup link, thus isolating any issues to the incorrectly setup link. So we think this is a QA report given the model of ITS.

0xsomeone (judge) commented:

@milapsheth - While the submission might be invalid from a practical perspective, a medium-risk severity rating was awarded due to the submission’s validity from a C4 perspective. The audit’s README explicitly includes balance changes outside of transfers as in-scope and this error in the setup of the C4 audit renders this submission to be valid.

# [M-02] TokenBalance limit could be bypassed by deploying TokenManager

- **Contest:** Axelar Network
- **Slug:** 2024-08-axelar-network
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-axelar-network
- **Source snapshot:** competitions/2024-08-axelar-network/final_report.html

TokenBalance limit could be bypassed by deploying TokenManager Submitted by 0x007 If a chain is compromised, it can not transfer beyond its balance. However, there’s a clever way to increase its balance arbitrarily.

A compromised chain can deploy token manager to a token controlled by the attacker on a different chain (or in the future, deploy ITS with their minter address). The balance on that chain would be untracked and the attacker can bridge an unlimited amount to the compromised chain to increase its balance. This is very severe because the main purpose of the hub and audit is to reduce damage to the wider ecosystem when a chain is compromised.

## Recommended Mitigation Steps

Token hub should store the original chain of each ITS tokenId and allow token or manager deployment from the original chain only. This would limit the access of remote chains to transfers.

## Assessed type

Access Control milapsheth (Axelar) confirmed and commented:

The report is valid, although we consider the severity to be Medium since the issue can’t be exploited on it’s own and requires a severe chain compromise to occur (since ITS by itself doesn’t allow deploying token manager for trustless factory tokens). While ITS hub isn’t meant to protect against all possible scenarios, it could handle this case by storing the original chain and restricting deployments from the origin chain as the report suggests. This is the same issue discussed in #43 and #77, although the exploit here is different.

0xsomeone (judge) commented:

The Warden specifies a potential scenario in which a blockchain compromise could affect a balance greater than the original one that was bridged to it.

I do not believe that a blockchain compromise is meant to reduce the severity of the vulnerability as the ITS system is meant to integrate with as many chains as possible in a “permissionless” manner. As tokens can be directly affected by the described vulnerability, I believe a high-risk severity rating is appropriate. To note, the root cause is different from #77 and #43 and thus, merits its own submission.

milapsheth (Axelar) commented:

@0xsomeone - Our reply here is relevant for this report, as well. ITS Hub is meant to limit damage in certain scenarios for chains connected to Axelar that have ITS deployed. But chains added by Axelar governance have to still meet a quality and security standard. Furthermore, ITS Hub explicitly whitelists ITS addresses by chain that it trusts. A compromised chain is inherently risky to all connected chains and apps, so ITS hub doesn’t allow arbitrary permissionless connections. So without a concrete exploit that can steal user funds, we consider this report to be a Medium severity issue.

0xsomeone (judge) decreased severity to Medium and commented:

@milapsheth - After reviewing the codebase’s documentation, I am inclined to agree that a blockchain compromise is considered a low-likelihood event rendering this submission to be of medium severity.
