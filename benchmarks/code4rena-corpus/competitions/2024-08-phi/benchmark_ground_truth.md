# Benchmark Ground Truth: Phi

## Accepted H/M Findings

# Accepted H/M Findings: Phi

# [H-01] Signature replay in signatureClaim results in unauthorized claiming of rewards

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

signatureClaim results in unauthorized claiming of rewards Submitted by kuprum, also found by Evo, McToady, and rbserver Loss of funds due to unauthorized and multiple claiming of art rewards: signature obtained from the Phi signer on one chain can be employed to claim NFT rewards on any other chain.

Summary Contract PhiFactory includes the function signatureClaim, which is intended for claiming art rewards using a signature obtained from the Phi signing authority. Unfortunately, the signature validation in this function is faulty, as it doesn’t validate the chain id for which the signature has been issued:

( uint256 expiresIn_, address minter_, address ref_, address verifier_, uint256 artId_,, bytes32 data_ ) = abi.

decode ( encodeData_, ( uint256, address, address, address, uint256, uint256, bytes32 )); if ( expiresIn_ <= block.

timestamp ) revert SignatureExpired (); if ( _recoverSigner ( keccak256 ( encodeData_ ), signature_ ) != phiSignerAddress ) revert AddressNotSigned (); Notice the fragment uint256 artId_,, bytes32 data_: in between of artId and data_ there is the encoded chainId, which is ignored.

There is a correct path for signature claims, which goes via PhiNFT1155 -> Claimable::signatureClaim, which unpacks the supplied data, but submits to PhiFactory::signatureClaim the data with the chain id substituted with block.chainid. Unfortunately, PhiFactory::signatureClaim can be called directly, thus bypassing this crucial step. As a result, the signature obtained for one chain can be reused for multiple chains. As PhiFactory and art contracts on multiple chains are independent, the artId referring to some cheap artwork on one chain, may refer on another chain to a very valuable artwork; claiming this artwork without proper authorization leads to direct loss of funds. Here is the scenario:

An attacker observes that a valuable art piece exists on chain A with artId.

They find a chain B where such artId is not yet used, and create a bunch of arts till they reach this artId.

They correctly claim artId on chain B.

They reuse the obtained signature to claim the art piece with artId on chain A, thus effectively stealing it.

## Recommended Mitigation Steps

We recommend to apply in PhiFactory::signatureClaim the same approach as in Claimable::signatureClaim, i.e., to repack the signature data substituting the chainId with block.chainid.

## Assessed type

Invalid Validation ZaK3939 (Phi) confirmed via duplicate Issue #59

# [H-02] Signature replay in createArt allows to impersonate artist and steal royalties

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

createArt allows to impersonate artist and steal royalties Submitted by kuprum, also found by rscodes, petarP1998, Tigerfrake, joicygiore, Jorgect, MrValioBg, OpaBatyo, smaul, willycode20, McToady, hail_the_lord, 0xNirix, pep7siup, enami_el, Sparrow, rbserver, and 0xCiphky Loss of funds as anyone can frontrun the createArt transaction, reusing the original signature but supplying their own config. As a result, the artist, the royalties recipient, as well the the royalty BPS can be set arbitrarily, leading to stealing the royalties from the artist and achieving other impacts.

Summary Function PhiFactory::createArt() doesn’t limit the signature to either the specific submitter, nor does it include into the signed data the CreateConfig parameters; which in particular include the artist, the royalties receiver, as well as other parameters. Other impacts are possible but here is one specific scenario:

The legitimate party creates a valid signature, config, and submits the createArt transaction.

An attacker observes createArt transaction in the mempool, and frontruns it, reusing the signature, but with their own config where they are the royalties recipient.

Attacker’s transaction succeeds, setting them as the royalties recipient.

The original transaction gets executed, and also succeeds, because createERC1155Internal succeeds both when a new PhiNFT1155 contract is created, as well as when it exists already.

The legitimate user doesn’t notice the difference: both ArtContractCreated and NewArtCreated events are emitted correctly (only NewArtCreated is emitted twice).

As a result, the attacker gets all rewards sent to the PhiRewards contract from PhiNFT1155 when the legitimate party claims an NFT token (see PhiNFT1155::claimFromFactory() ).

Other possible impacts (a non-exclusive list):

The attacker sets themselves as an artist, and gets the possibility to call the PhiNFT1155::updateRoyalties() function, thus setting the royaltyBPS to arbitrary value.

The attacker sets themselves as an artist, and gets the possibility to call the PhiFactory::updateArtSettings() function, thus setting the parameters to arbitrary values, e.g.

maxSupply, endTime, or mintFee.

## Recommended Mitigation Steps

We recommend the following steps:

In PhiFactory::createArt():

Include into the signed data the CreateConfig parameters.

Limit transaction execution to a specific submitter.

In PhiFactory::createERC1155Internal() fail the transaction if the contract already exists.

## Assessed type

Invalid Validation ZaK3939 (Phi) confirmed, but disagreed with severity and commented:

We will disregard the proposed mitigation, because it’s incorrect. Instead, we will extend the signature by introducing nonce, creator, and executor. However, I believe this issue should be classified as Medium rather than High. Addresses that appear to be invalid artist addresses are easily verifiable on the frontend (for example, by checking for lack of prior history). Additionally, artists have certification badges through EAS (Ethereum Attestation Service).

So, we don’t necessarily see a problem with multiple arts being created, but let me improve our logic by this:

struct SignatureData { uint256 expiresIn; uint256 nonce; address creator; address executor; string uri; bytes credData; } function createArt( bytes calldata signedData_, bytes calldata signature_, CreateConfig memory createConfig_ ) external payable nonReentrant whenNotPaused returns (address) { _validateArtCreationSignature(signedData_, signature_); SignatureData memory data = abi.decode(signedData_, (SignatureData)); ERC1155Data memory erc1155Data = _createERC1155Data(artIdCounter, createConfig_, data.uri, data.credData); address artAddress = createERC1155Internal(artIdCounter, erc1155Data); nonces[_msgSender()]++; artIdCounter++; return artAddress; } function _validateArtCreationSignature(bytes memory signedData_, bytes calldata signature_) private view {

SignatureData memory data = abi.decode(signedData_, (SignatureData)); if (_recoverSigner(keccak256(signedData_), signature_) != phiSignerAddress) revert AddressNotSigned(); if (data.expiresIn <= block.timestamp) revert SignatureExpired(); if (data.creator != address(0) && data.creator != _msgSender()) revert InvalidCreator(); if (data.executor != address(0) && data.executor != _msgSender()) revert InvalidExecutor(); if (data.nonce != nonces[_msgSender()]) revert InvalidNonce(); } 0xDjango (judge) commented:

High. This vulnerability would undermine the entire art creation process.

# [H-03] shareBalance bloating eventually blocks curator rewards distribution

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

shareBalance bloating eventually blocks curator rewards distribution Submitted by CAUsr, also found by CAUsr, pipidu83, typicalHuman, kuprum, DigiSafe, joicygiore, DanielArmstrong, rare_one, Jorgect, 0xrafaelnicolau, federodes, 0xmuxyz, robertodf99, OpaBatyo, rscodes, MrValioBg, hgrano, pep7siup, and 0xrex Cred curator share balances are tracked by the following data structure in the Cred:

mapping ( uint256 credId => EnumerableMap.

AddressToUintMap balance ) private shareBalance; There may be MAX_SUPPLY = 999 maximum curators of a cred. However, when a curator sells all their shares, the curator entry still remains in the data structure with a zero shares number. Because of that the mapping grows unlimited. As soon as about 4000 different addresses owned (for any time period) a cred’s share, the current shareholders, regardless of their current number, will be no longer enumerable by CuratorRewardsDistributor. Rewards distribution for this cred will be blocked because gas usage hits the block gas limit.

Have a look at the curator share balance update code:

function _updateCuratorShareBalance ( uint256 credId_, address sender_, uint256 amount_, bool isBuy ) internal { (, uint256 currentNum ) = shareBalance [ credId_ ].

tryGet ( sender_ ); if ( isBuy ) { if ( currentNum == 0 && !

_credIdExistsPerAddress [ sender_ ][ credId_ ]) { _addCredIdPerAddress ( credId_, sender_ ); _credIdExistsPerAddress [ sender_ ][ credId_ ] = true; } shareBalance [ credId_ ].

set ( sender_, currentNum + amount_ ); } else { if (( currentNum - amount_ ) == 0 ) { _removeCredIdPerAddress ( credId_, sender_ ); _credIdExistsPerAddress [ sender_ ][ credId_ ] = false; } shareBalance [ credId_ ].

set ( sender_, currentNum - amount_ ); } When all shares are sold by the curator, its balance is set to 0 rather than deleted from the EnumerableMap.

During rewards distribution, a call to getCuratorAddresses is made. The control goes to _getCuratorData function which enumerates all entries of shareBalance[credId]:

uint256 stopIndex; if ( stop_ == 0 || stop_ > shareBalance [ credId_ ].

length ()) { stopIndex = shareBalance [ credId_ ].

length (); } else { stopIndex = stop_; } //...

for ( uint256 i = start_; i < stopIndex; ++ i ) { ( address key, uint256 shareAmount ) = shareBalance [ credId_ ].

at ( i ); Reading storage slots is quite an expensive operation nowadays. The execution will eventually hit the block gas limit and the transaction will be reverted. Details of sload costs and block gas limit may vary from chain to chain, but the DoS possibility never goes away.

Besides, take into account that storage access costs may increase significantly, rendering the discussed code unexecutable. Such a change may be introduced in a hardfork, e.g., the cost of the sload opcode was increased 10 times recently.

## Impact

If a cred is used and traded long enough (which is the intended state of the matter, presumably), the rewards of such a cred will be undistributable. Besides that, a griefing may be executed to block existing rewards (please see discussion in the PoC). Assets (rewards) of curators can be lost in that case until a code upgrade is made. If the contract was not upgradeable, that would be a high-risk situation in my opinion.

## Recommended Mitigation Steps

Remove zero entries from the map:

function _updateCuratorShareBalance ( uint256 credId_, address sender_, uint256 amount_, bool isBuy ) internal { (, uint256 currentNum ) = shareBalance [ credId_ ].

tryGet ( sender_ ); if ( isBuy ) { if ( currentNum == 0 && !

_credIdExistsPerAddress [ sender_ ][ credId_ ]) { _addCredIdPerAddress ( credId_, sender_ ); _credIdExistsPerAddress [ sender_ ][ credId_ ] = true; } shareBalance [ credId_ ].

set ( sender_, currentNum + amount_ ); } else { if (( currentNum - amount_ ) == 0 ) { _removeCredIdPerAddress ( credId_, sender_ ); _credIdExistsPerAddress [ sender_ ][ credId_ ] = false; + shareBalance [ credId_ ].

remove ( sender_ ); } + else shareBalance [ credId_ ].

set ( sender_, currentNum - amount_ ); }

## Assessed type

DoS ZaK3939 (Phi) confirmed and commented:

In addition to addressing #188, we will add recommended steps and remove zero entries.

0xDjango (judge) increased severity to High and commented:

Upgrading to HIGH, for permanent freezing of rewards.

# [H-04] Forced endTime extension in updateArtSettings() allows attacker to mint more tokens

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

endTime extension in updateArtSettings() allows attacker to mint more tokens Submitted by MrPotatoMagic The function updateArtSettings() allows an artist to update the art settings for their artwork at any time. Specifically, it allows updating the receiver, maxSupply, mintFee, startTime, endTime, soulBounded and uri of the PhiArt struct as well as the royalty configuration.

Issue The issue is that if the artist wants to update the art settings after the claiming/minting event from startTime to endTime is over, the artist is forced to extend the endTime to atleast the current block.timestamp. The artist would be wanting to update the uri or the soulBounded value (in case it might be the artist’s idea to keep or not keep soulbounded initially and then later on change the value to make it not soulbounded or soulbounded) or even updating the configuration for royalties for secondary sales post the event.

## Impact

Since the artist is forced to extend the endTime to block.timestamp, a malicious user keeping track of such calls to updateArtSettings() can backrun the artist’s transaction with a merkleClaim() call to mint any amount of tokens for that artId. Since the validation here would not cause a revert, the call goes through successfully.

## Recommended Mitigation Steps

Consider providing a separate function to update the uri, soulBounded and royalty config settings since these values are most likely to be updated post an event. Additionally, as a safety measure use >= instead of > in the check here.

## Assessed type

Timing ZaK3939 (Phi) commented:

In addressing Issue 70, we will make it impossible to update the URI and soulbound status. Therefore, when reopening, it seems better for the endtime to be automatically extended. We do not consider the observation about excessive minting to be a problem in any way.

ZaK3939 (Phi) confirmed and commented:

As for this one, I re-labeled it as a bug, since I will delete update feature of soulbound and uri.

However, I will leave a comment regarding the number of mints, which we don’t have a problem with and we will tell artist this feature. I will take a consistent approach to other issues regarding mints quantity.

I agree that this can be classified as a medium severity bug. However, assuming the soulbound feature and URI have been removed, if we’re going to update the contract, it should be properly reopened.

What’s more concerning is the ability to manipulate the mintfee and maxSupply while leaving the blocktime in the past. This scenario presents a far more serious vulnerability.

0xDjango (judge) increased severity to High and commented:

This report highlights an interesting issue with the updateArtSettings() function, though I agree with the sponsor that I don’t see extra minting to impose a problem for the artist or the protocol. I don’t agree with ruling this as a Medium. I will dupe to #14 as this report states:

The function updateArtSettings() allows an artist to update the art settings for their artwork at any time. Specifically, it allows updating the receiver, maxSupply, mintFee, startTime, endTime, soulBounded and uri of the PhiArt struct as well as the royalty configuration.” MrPotatoMagic (warden) commented:

@0xDjango, I believe this issue should be treated separately as a valid High-severity issue.

The reason why I think excessive minting is an issue is since once NFTs are being traded on external marketplaces, there are only limited amount of tokens in circulation (that have monetary value attached to them). If an artist updates the art settings and is forced to extend the endTime to block.timestamp, an attacker could use this opportunity to mint more tokens into circulation (indirectly receiving more monetary value as well as diluting the value of NFTs in the external market over time). Since there wasn’t an ongoing event during which the minting occurs, this would fall under unauthorized minting that an artist wouldn’t have expected after the original event ended.

The primary issue is viewing it from the viewpoint of an artist being malicious. Here this report is viewing it from the point of an artist being a normal honest actor who is wanting to update either the uri, soulbounded value or royalty config.

To note, post an event, The URI can be updated for several reasons such as in case the IPFS/image hosting provider has changed but the media is kept the same.

The soulBounded value can be updated in case it’s the artist idea (which has been communicated with users) to keep or not keep the NFTs soulbounded initially and then later on change the value to make it not soulbounded or soulbounded.

The royalty config percentage can clearly be updated by an artist post an event for secondary sale royalties, in case the artist wants to charge lesser or higher royalties.

Lastly, I also disagree with the comment that it does not affect the artist. It does affect the artist, since if this unauthorized minting occurs, it dilutes the value of the NFT in the external marketplace. This means the price per NFT drops and thus the royalties the artist receives are lesser since royalties are based on sale price as per EIP-2981, which is utilized by the protocol currently in CreatorRoyaltiesControl.sol.

0xDjango (judge) commented:

Thank you for re-surfacing this. My current thinking:

BAYC originally minted for 0.08 ETH.

The floor price is currently ~12 ETH.

This endTime extension reopens minting. It negatively affects the users who already hold the NFT and intend to sell in on the open market for financial gain.

MrPotatoMagic (warden) commented:

Adding onto the impact mentioned:

It negatively affects the users who already hold the NFT and intend to sell in on the open market for financial gain.

It also affects the artist since the royalties received would be lesser as pointed out in the last paragraph in my comment. The existing users and artist have everything to lose while the attacker mints more tokens to receive free monetary value.

0xDjango (judge) commented:

I have de-duped this report from #14. This report was the only one that mentioned the forced update to endTime, which can negatively affect token holders by allowing more minting after the NFTs have already been trading on the open market without the ability to mint fresh.

Note, this can be mitigated (on the frontend) by forcing the call to updateArtSettings() to provide a maxSupply equal to the current total number of NFTs that have already been minted. A janky mitigation though and could lead to frontrunning issues.

# [H-05] Exposed _removeCredIdPerAddress & _addCredIdPerAddress allows anyone to cause issues to current holders as well as upcoming ones

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

_removeCredIdPerAddress & _addCredIdPerAddress allows anyone to cause issues to current holders as well as upcoming ones Submitted by 0xrex, also found by 0xrex, debo, JustUzair, CodeCop, NerfZeri, 0xkaox ( 1, 2 ), 0xc0ffEE, th3l1ghtd3m0n, kn0t, TopStar, waydou, eLSeR17 ( 1, 2 ), 0xrafaelnicolau, onthehunt11, mjcpwns, nikolap, lu1gi, rscodes, EaglesSecurity, CAUsr, OpaBatyo ( 1, 2 ), kuprum ( 1, 2 ), Inspecktor, avoloder, McToady, unnamed, hail_the_lord, 54thMass, sweetbluesugarlatte, 41uve12, Taiger, Falendar, pfapostol, 0xAkira, timatc, Monstitude, 20centclub, NexusAudits, 10ap17, MrPotatoMagic, KupiaSec, and JanuaryPersimmon2024 ( 1, 2 ) The _addCredIdPerAddress

and _removeCredIdPerAddress allows anyone to interrupt users who are attempting to hold cred shares & sell already held cred shares.

## Recommended Mitigation Steps

Make both functions internal rather than public.

## Assessed type

DoS ZaK3939 (Phi) confirmed 0xDjango (judge) increased severity to High

# [H-06] Reentrancy in creating Creds allows an attacker to steal all Ether from the Cred contract

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

Submitted by CAUsr, also found by 0xCiphky, 0xc0ffEE, smaul ( 1, 2 ), hearmen, farismaulana, aldarion, Decap, rscodes, lu1gi, Jorgect, CAUsr ( 2 ), pfapostol, McToady, hail_the_lord, Ruhum, IzuMan, hgrano, MrPotatoMagic, Agontuk, 0xrex ( 1, 2 ), KupiaSec, and JanuaryPersimmon2024 Note: the submission from warden 0xCiphky was originally featured as H-06 in this report; however, after further discussion between the judge and sponsor, it was determined that a different submission demonstrates the largest impact of the common issue. Per direction from the judge, this audit report was updated on October 16, 2024 to highlight the submission below from warden CAUsr.

## Vulnerability Details

During cred creation, the sender automatically buys 1 share of the cred. During share purchases, a refund is issued if Ether provided by the sender exceeds the value needed. It allows an attacker to reenter both cred creation and share trading functions. Which, in turn, allows the attacker to overwrite cred data before the credIdCounter is incremented. This fact introduces a critical vulnerability in the case when there is more than one whitelisted bonding curve. The attacker can purchase shares cheaply, overwrite the cred, and sell the shares expensively, almost depleting the entire contract balance.

Let’s discuss a possible attack vector in detail.

Preparations. The attacker prepares two sets of cred creation data which may be quite legit. The most important aspect is that the first set uses a “cheaper” bonding curve and the other uses a more expensive one (see the Impact section for more details). The sender’s address would be a helper contract address. However, the attacker doesn’t need to deploy the contract beforehand to know its address, since there are several ways to know a to-be-deployed contract address without exposing its bytecode (e.g. by deployer address and nonce or with the help of the CREATE2 opcode). To sum up, at the moment of checking cred creation data and signing there are no signs of malignancy.

Then, the attacker deploys the helper contract and invokes an attack. Further actions are executed by the contract in a single transaction as follows.

The attacker creates the first cred, with the cheap IBondingCurve used. The cred creation code invokes buyShareCred(credIdCounter, 1, 0); as part of its job. The attacker provides excess Ether, so a refund is triggered and the control goes back to the attacker. Note that the counter increment is not reached yet.

function _createCredInternal ( address creator_, string memory credURL_, string memory credType_, string memory verificationType_, address bondingCurve_, uint16 buyShareRoyalty_, uint16 sellShareRoyalty_ ) internal whenNotPaused returns ( uint256 ) { if ( creator_ == address ( 0 )) { revert InvalidAddressZero (); } creds [ credIdCounter ].

creator = creator_; creds [ credIdCounter ].

credURL = credURL_; creds [ credIdCounter ].

credType = credType_; creds [ credIdCounter ].

verificationType = verificationType_; creds [ credIdCounter ].

bondingCurve = bondingCurve_; creds [ credIdCounter ].

createdAt = uint40 ( block.

timestamp ); creds [ credIdCounter ].

buyShareRoyalty = buyShareRoyalty_; creds [ credIdCounter ].

sellShareRoyalty = sellShareRoyalty_; buyShareCred ( credIdCounter, 1, 0 ); emit CredCreated ( creator_, credIdCounter, credURL_, credType_, verificationType_ ); credIdCounter += 1; return credIdCounter - 1; } The attacker buys some amount of shares of the almost created cred by invoking buyShareCred. Note that there are no significant obstacles to doing that as the attacker can employ a flash loan. Again, the attacker provides excess Ether, so a refund is triggered and the control goes back to the attacker.

Remember that the control is still nested in buyShareCred nested in the first createCred. This time the attacker invokes the createCred again, with the second cred creation data, with a more expensive bonding curve used. Note that there are no reentrancy guards in createCred, buyShareCred, or sellShareCred. The data is passed to _createCredInternal where it overwrites the first cred contents in the creds[credIdCounter] (remember, the counter is not incremented yet by the first createCred ).

Here is the most important part of the overwriting for the attacker: they just changed the bonding curve.

creds [ credIdCounter ].

bondingCurve = bondingCurve_; A couple of lines below buyShareCred will be hit again and the attacker grabs back the control in the usual manner.

Now the attacker calls sellShareCred and dumps as many purchased shares as possible until depleting the Cred balance or his own share balance. The “more expensive” bonding curve gives the attacker more Ether than they spend while buying on step 2. Note that the share lock time also won’t help as the timestamp is not yet set, which is the subject of another finding.

if ( isBuy ) { cred.

currentSupply += amount_; uint256 excessPayment = msg.

value - price - protocolFee - creatorFee; if ( excessPayment > 0 ) { _msgSender ().

safeTransferETH ( excessPayment ); } lastTradeTimestamp [ credId_ ][ curator_ ] = block.

timestamp; } else { cred.

currentSupply -= amount_; curator_.

safeTransferETH ( price - protocolFee - creatorFee ); } Even if the SHARE_LOCK_PERIOD was honored, that would mean that the attacker has to wait 10 minutes before pocketing the profit. That would make the situation a bit more difficult for the attacker, but still highly dangerous for the protocol.

At this moment almost all Ether from the Cred contract is transferred to the attacker’s contract. The attacker has to keep some of the Ether intact since now the unwinding of the nested calls is happening (and some fees are paid): the second createCred, buyShareCred, and the first createCred.

## Impact

As soon as there is more than one bonding curve whitelisted in the protocol, the vulnerability allows to drain almost entire protocol funds. Whitelisting more than one curve seems to be an expected behavior.

Fine-tuning the numbers to exploit the bonding curve price differences is a mere technicality, e.g. if there was just a 10% price difference that would be more than enough to perform an attack. Also note that on the current codebase, the attack may be repeated numerously and instantly.

## Recommended Mitigation Steps

To mitigate this attack vector it would be enough to add a reentrancy guard here:

function createCred ( address creator_, bytes calldata signedData_, bytes calldata signature_, uint16 buyShareRoyalty_, uint16 sellShareRoyalty_ ) public payable whenNotPaused + nonReentrant { However, I recommend protecting with reentrancy guards all protocol functions directly or indirectly dealing with assets or important state.

Another important measure to consider is adhering to the Checks-Effects-Interactions Pattern.

ZaK3939 (Phi) confirmed via issue #25 0xDjango (judge) commented:

After further discussion, this submission should be selected for the public report. It shares the same root cause and fix as #25, though highlights a much more severe impact. Sponsor indicated that there is a possibility that multiple price curves may be selectable in the future.

Please note: the above re-assessment took place approximately 3 weeks after judging and awarding were finalized.

# [H-07] Unrestricted changes to token settings allow artists to alter critical features

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

Submitted by 0xCiphky, also found by MrPotatoMagic, rbserver ( 1, 2 ), petarP1998, pipidu83, kuprum ( 1, 2 ), ironside, onthehunt11, OpaBatyo, 0xNirix ( 1, 2 ), eierina, hail_the_lord, Trooper, 0xBeastBoy ( 1, 2 ), and NexusAudits The updateArtSettings function in the PhiFactory contract allows artists to modify several key settings of their art at any time. These settings include the URI link, royalty fee and soulBounded (non-transferable) feature.

function updateArtSettings ( uint256 artId_, string memory url_, address receiver_, uint256 maxSupply_, uint256 mintFee_, uint256 startTime_, uint256 endTime_, bool soulBounded_, IPhiNFT1155Ownable.RoyaltyConfiguration memory configuration ) external onlyArtCreator ( artId_ ) {...

art.

receiver = receiver_; art.

maxSupply = maxSupply_; art.

mintFee = mintFee_; art.

startTime = startTime_; art.

endTime = endTime_; art.

soulBounded = soulBounded_; art.

uri = url_; uint256 tokenId = IPhiNFT1155Ownable ( art.

artAddress ).

getTokenIdFromFactoryArtId ( artId_ ); IPhiNFT1155Ownable ( art.

artAddress ).

updateRoyalties ( tokenId, configuration ); emit ArtUpdated ( artId_, url_, receiver_, maxSupply_, mintFee_, startTime_, endTime_, soulBounded_ ); } The problem is that there are no restrictions or limitations on how these settings can be changed after the art has been created and minted. This flexibility allows artists to alter critical functionalities in ways that could negatively impact existing holders:

Changing Soulbound Status: Artists can toggle the soulBounded status to lock transfers, effectively trapping users who purchased NFTs under the assumption they were transferable.

Modifying Royalties: Artists can set royalty fees to extremely high values after initial sales, forcing holders to pay unexpected fees upon resale.

Updating URLs: Artists can change the linkURI, potentially misleading users or affecting the NFT’s perceived value by altering the associated content. In the worst case, the URL could be changed to a malicious link, posing security risks to users who interact with it.

These changes can be made at any time, without prior notice to holders, leaving users vulnerable to unfavourable adjustments.

## Impact

Allowing unrestricted changes to critical token settings poses a significant risk to the stability and trustworthiness of the NFTs. Users who have already minted or purchased NFTs could be adversely affected by changes they did not agree to, such as increased fees or transfer restrictions.

Recommendation Implement limits on how and when critical settings can be changed, such as capping royalty rates. This would help protect users while maintaining some flexibility for artists.

ZaK3939 (Phi) confirmed and commented:

Given the specifications of our NFT, which is relatively inexpensive and doesn’t have strict conditions regarding the number of mints, we feel that classifying this as a High severity issue is inappropriate.

We will implement the fix. Regarding updates, we will remove the Soulbound feature and the URI update functionality.

As for royalties, some other platforms allow up to 100% royalties. While we’re using EIP-2981, which is not mandatory, we plan to implement a maximum royalty of around 20%.

0xDjango (judge) commented:

I am going to rule HIGH on this one. All three conditions outlined by the warden are medium/high, and the accumulation of all three makes this a clear high in my opinion.

Medium Risk Findings (8)

# [M-01] Contract PhiNFT1155 can’t be paused

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

PhiNFT1155 can’t be paused Submitted by kuprum, also found by MrPotatoMagic, Rhaydden, hail_the_lord, Gosho, inh3l, and DigiSafe The pausing mechanism of PhiNFT1155 contract is implemented incorrectly and doesn’t work. Users are still able to transfer NFTs in paused state.

Summary Contract PhiNFT1155 inherits from the following parent contracts:

contract PhiNFT1155 is Initializable, UUPSUpgradeable, ERC1155SupplyUpgradeable, ReentrancyGuardUpgradeable, PausableUpgradeable, Ownable2StepUpgradeable, IPhiNFT1155, Claimable, CreatorRoyaltiesControl { The problem with the above is that inheriting from PausableUpgradeable is not effective in the scope of OZ ERC1155 contracts. As a result, users are able to transfer NFT tokens even when the contract is paused, as the below PoC demonstrates.

## Recommended Mitigation Steps

We recommend to inherit from ERC1155PausableUpgradeable instead of PausableUpgradeable; this enforces the paused state on the NFT transfer functions. Apply the following diff to PhiNFT1155.sol:

diff --git a/src/art/PhiNFT1155.sol b/src/art/PhiNFT1155.sol index a280d05..258cb5a 100644 --- a/src/art/PhiNFT1155.sol +++ b/src/art/PhiNFT1155.sol @@ -8,7 +8,8 @@ import { Claimable } from "../abstract/Claimable.sol"; import { ERC1155Upgradeable } from "@openzeppelin/contracts-upgradeable/token/ERC1155/ERC1155Upgradeable.sol"; import { ERC1155SupplyUpgradeable } from "@openzeppelin/contracts-upgradeable/token/ERC1155/extensions/ERC1155SupplyUpgradeable.sol"; -import { PausableUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol"; +import { ERC1155PausableUpgradeable } from + "@openzeppelin/contracts-upgradeable/token/ERC1155/extensions/ERC1155PausableUpgradeable.sol";

import { ReentrancyGuardUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol"; import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol"; import { SafeTransferLib } from "solady/utils/SafeTransferLib.sol"; @@ -23,12 +24,21 @@ contract PhiNFT1155 is UUPSUpgradeable, ERC1155SupplyUpgradeable, ReentrancyGuardUpgradeable, - PausableUpgradeable, + ERC1155PausableUpgradeable, Ownable2StepUpgradeable, IPhiNFT1155, Claimable, CreatorRoyaltiesControl { + // The following functions are overrides required by Solidity.

+ + function _update(address from, address to, uint256[] memory ids, uint256[] memory values) + internal + override(ERC1155PausableUpgradeable, ERC1155SupplyUpgradeable) + { + super._update(from, to, ids, values); + } + /*////////////////////////////////////////////////////////////// USING //////////////////////////////////////////////////////////////*/

## Assessed type

Library ZaK3939 (Phi) confirmed via duplicate Issue #61 0xDjango (judge) commented:

Given the sponsor’s acceptance of the issue, it can be concluded that omitting the whenNotPaused function on the transfer functions was indeed an oversight.

Note: For full discussion, see here.

# [M-02] Incorrect fee handling prevents protocol from updating fees

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

Submitted by kn0t, also found by 0xc0ffEE, 0xrafaelnicolau, Decap, federodes, Ruhum ( 1, 2 ), CAUsr, OpaBatyo ( 1, 2 ), 0xNirix, 0xBugSlayer, 20centclub ( 1, 2 ), MrPotatoMagic, rbserver ( 1, 2 ), and 0xCiphky The primary impact of this bug is that the protocol cannot change the mint and art creation fees as intended. This limitation has the following consequences:

The protocol is unable to set the correct mint fee of 0.00005 ETH per NFT minted, as documented in the specifications:

Mint Fee This fee is applied when a Minter wants to mint a Cred NFT.

Protocol will have a mint price/fee of 0.00005 ETH.

function setProtocolFee ( uint256 protocolFee_ ) external onlyOwner { if ( protocolFee_ > 10_000 ) revert ProtocolFeeTooHigh (); // @audit <== This check is incorrect mintProtocolFee = protocolFee_; emit ProtocolFeeSet ( protocolFee_ ); } function setArtCreatFee ( uint256 artCreateFee_ ) external onlyOwner { if ( artCreateFee_ > 10_000 ) revert ArtCreatFeeTooHigh (); // @audit <== This check is incorrect artCreateFee = artCreateFee_; emit ArtCreatFeeSet ( artCreateFee_ ); } This discrepancy between the intended fee (0.00005 ETH) and the maximum settable fee (10,000 wei) renders the fee adjustment mechanism ineffective.

The protocol loses the ability to adjust its revenue model in response to market conditions or strategic decisions, as both the mint fee and art creation fee are effectively locked at their initial values.

Below lines demonstrate that the fees are being applied as direct values rather than percentages, contrary to the setter functions’ assumptions.

From src/PhiFactory.sol:

function _processClaim ( uint256 artId_, address minter_, address ref_, address verifier_, uint256 quantity_, bytes32 data_, string memory imageURI_, uint256 etherValue_ ) private { PhiArt storage art = arts [ artId_ ]; // Handle refund uint256 mintFee = getArtMintFee ( artId_, quantity_ ); if (( etherValue_ - mintFee ) > 0 ) { _msgSender ().

safeTransferETH ( etherValue_ - mintFee ); } protocolFeeDestination.

safeTransferETH ( mintProtocolFee * quantity_ ); // @audit From src/art/PhiNFT1155.sol:

function createArtFromFactory ( uint256 artId_ ) external payable onlyPhiFactory whenNotPaused returns ( uint256 ) { _artIdToTokenId [ artId_ ] = tokenIdCounter; _tokenIdToArtId [ tokenIdCounter ] = artId_; uint256 artFee = phiFactoryContract.

artCreateFee (); protocolFeeDestination.

safeTransferETH ( artFee ); // @audit It’s important to note that while the immediate financial impact may be limited if the initial fees were set correctly, the long-term implications of this inflexibility could be significant for the protocol’s adaptability and financial management.

## Recommended Mitigation Steps

+ uint constant MAX_PROTOCOL_FEE = 0.00005 ether; + uint constant MAX_ART_CREATE_FEE = 0.00005 ether; function setProtocolFee(uint256 protocolFee_) external onlyOwner { - if (protocolFee_ > 10_000) revert ProtocolFeeTooHigh(); + if (protocolFee_ > MAX_PROTOCOL_FEE) revert ProtocolFeeTooHigh(); mintProtocolFee = protocolFee_; emit ProtocolFeeSet(protocolFee_); } function setArtCreatFee(uint256 artCreateFee_) external onlyOwner { - if (artCreateFee_ > 10_000) revert ArtCreatFeeTooHigh(); + if (artCreateFee_ > MAX_ART_CREATE_FEE) revert ArtCreatFeeTooHigh(); artCreateFee = artCreateFee_; emit ArtCreatFeeSet(artCreateFee_); } ZaK3939 (Phi) confirmed and commented via duplicate Issue #13 To address this, we will modify the set function to use absolute values instead of percentages.

# [M-03] PhiFactory:claim potentially causing loss of funds if mintFee changed beforehand

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

PhiFactory:claim potentially causing loss of funds if mintFee changed beforehand Submitted by pep7siup, also found by 0xNirix ( 1, 2 ), TECHFUND-inc, Tigerfrake, 13u9, DigiSafe, federodes, Decap, waydou, EaglesSecurity, smaul, rscodes, willycode20, Gosho, 20centclub, Kaysoft, MrPotatoMagic, rbserver, 0xCiphky, and biakia If the mintFee was lowered by the Art creator via updateArtSettings before the claim was executed, the msg.value from user would become larger than the updated value of mintFee. As the function does not validate msg.value and consumes the mintFee directly, the excess ETH sent by the user will not be refunded, causing a loss of funds for user.

If PhiFactory still holds positive ETH balance, a malicious user can exploit this by sending less msg.value than expected (or no ETH at all). This results in PhiFactory ’s ETH being used for the mintFee payment instead, causing a loss for the protocol.

## Recommended Mitigation Steps

Validate msg.value against mintFee to ensure the caller sent in the correct amount of ETH. Or, forward msg.value directly to subsequent claim process since the _processClaim will eventually handle the refund.

## Assessed type

Payable 0xDjango (judge) decreased severity to Medium ZaK3939 (Phi) confirmed

# [M-04] Cred creator could cause stuck funds

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

Submitted by petarP1998 Update of cred can be triggered by the cred creator. If there are already issued shares for users, and the cred creator decides to update the sellShareRoyalty_ this will lead users to pay up to 50% fee depending on the value. And users will be in situation that to withdraw their funds, they need to pay 50% to the cred creator and additionally some fee to the protocol.

## Recommended Mitigation Steps

The protocol should implement a mechanism where this type of updates, will be applied after some period. I will explain my idea in the below lines:

If cred creator wants to increase the sellShareRoyalty_, it should first create a request for updating which can be applied after 2 days, for example.

In these two days users who are not okay with the new value of sellShareRoyalty_ can withdraw their funds from the protocol.

My proposal is to have two functions:

updateSellShareRoyaltyRequest which will create this request and this request can be approved after some period.

approveUpdateSellShareRoyaltyRequest if the period already passed the cred creator can apply the change.

## Assessed type

Access Control ZaK3939 (Phi) confirmed and commented:

The observation is valid. However, I believe the main issue is the ability to update the royalty percentage. The high percentage is deliberately set to allow creators to set up creds where they prefer to minimize sharing with others. Therefore, my plan is to keep the current high percentage limit, but remove the update functionality altogether. This vulnerability should be rated medium severity in my opinion.

0xDjango (judge) decreased severity to Medium and commented:

I agree that the functionality should be removed or placed behind a timelock. Medium.

MrPotatoMagic (warden) commented:

Hi @0xDjango, I believe this issue should be of QA-severity.

The main intention behind creds is for users to form, visualize, showcase their onchain identity. When a cred is created, the creator (i.e., the onchain identity) is the source of trust when users buy up their shares. This means, the more social, reputed and trusted an identity is, the lower are the chances of a creator wanting to perform such an attack.

It is highly unlikely a high reputed creator would want to perform such an attack. When it comes to low reputed creators, who are more likely to perform this attack, the impact would be low because the shares being bought for such untrusted entities would not be high (Note: This claim is based on onchain data I’ve observed from a similar protocol). If we look at the price curve, even if 5-10 shares were bought by users, the overall loss would be minimum and bearable, considering that users know the risk when buying shares of untrusted onchain identities.

I think the above points along with the fact that 0-50% is an acceptable range defined by the protocol as per sponsor’s comments above, it seems that this issue is rather suited for QA-severity.

0xDjango (judge) commented:

This is a valid medium severity issue. Such a function that can directly affect user funds, especially through frontrunning, should be mitigated by a timelock or similar.

# [M-05] Lack of data validation when users are claiming their art allows malicious user to bypass signature/merkle hash to provide unapproved ref_ , artId_ and imageURI

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

ref_, artId_ and imageURI Submitted by rscodes, also found by gregom, MrValioBg, McToady, and MrPotatoMagic In the file PhiFactory.sol, the lack of checks in merkleClaim allows malicious users to bypass the hash check to pass in their own parameter of choice. The 3 variables that could possibly be manipulated ( referral, artId and imageURI ) and their impact will be each discussed below.

Missing ref_ and artId_ in validation.

As we can see in merkleClaim, address ref_ and uint256 artId_ is decoded from encodeData_ which is simply a bytes data type passed into the function parameter by the user.

( address minter_, address ref_, uint256 artId_ ) = abi.

decode ( encodeData_, ( address, address, uint256 )); PhiArt storage art = arts [ artId_ ];......

if ( !

MerkleProofLib.

verifyCalldata ( proof_, credMerkleRootHash, keccak256 ( bytes.

concat ( keccak256 ( abi.

encode ( minter_, leafPart_ )))) ) ) { revert InvalidMerkleProof (); }......

Observing the line where the merkle hash is cross-checked:

!

MerkleProofLib.

verifyCalldata ( proof_, credMerkleRootHash, keccak256 ( bytes.

concat ( keccak256 ( abi.

encode ( minter_, leafPart_ )))) ) It becomes clear that only minter_ from encodeData_ is checked, leaving ref_ and artId_ to whatever values passed by the user.

By the way, both ref_ and artId_ are explicitly checked in the signatureClaim function:

function signatureClaim (.....) external payable whenNotPaused { ( uint256 expiresIn_, address minter_, address ref_, address verifier_, uint256 artId_,, bytes32 data_ ) = abi.

decode ( encodeData_, ( uint256, address, address, address, uint256, uint256, bytes32 )); if ( expiresIn_ <= block.

timestamp ) revert SignatureExpired (); if ( _recoverSigner ( keccak256 ( encodeData_ ), signature_ ) != phiSignerAddress ) revert AddressNotSigned ();.....

} Hence, to forge ref_ and artId_, malicious users will have to use merkleClaim instead of signatureClaim.

Impact of forging ref_ Malicious users can pass in their own other account address as ref_, successfully stealing the referral fee from the real ref_ or the artist(who will receive the referral fee if ref_ is set to null).

Suppose Alice has 2 accounts, when she calls merkleClaim to claim on account 1, she can pass in account 2 as the ref_ to illegally steal the referral fees to offset the cost of minting.

The reason why Alice has to pass in account 2 instead of account 1 as the ref_ is because in PhiRewards.sol ’s depositRewards, referral has to be different from minter address in order for Alice to receive the referral fees.

Impact of forging artId_ Unlike signatureClaim which checks artId_ explicitly, merkleClaim only verifies minter_ and leafPart_ to the credId’s merkle root hash.

Since a Cred can have multiple art linked to it, this becomes a problem.

Suppose Cred A has Art 1 and Art 2.

Now, Alice got approval to claim Art 1.

If the art’s verification type was set to “SIGNATURE” and Alice had to call signatureClaim, Alice would only be able to claim Art 1 with that approval signature.

However if the art’s verification type was “MERKLE” instead, when she calls merkleClaim, she will be able to claim Art 2 with only approval for Art 1 by passing the artId_ of Art 2.

Missing imageURI in validation We can see in both signatureClaim Line 330 and merkleClaim Line 355, imageURI is passed into the function parameter rather than extracted from the signature and is not involved in the merkle hash check as well.

Since imageURI contains important information about the art details (such as location/directory). This could have a severe impact. When the malicious user is trying to claim an art that he has permission to, he could pass in an imageURI with the location directory of the picture image file pointing to another art which he does not have permission for.

The line advancedTokenURI[tokenId_][to_] = imageURI_; in PhiNFT1155.sol will also be set to an unapproved value. This will lead to undefined/unsupported behaviours when interacting with the smart contract regarding trying to render an user’s art URI.

## Recommended Mitigation Steps

In function merkleClaim:

if ( !MerkleProofLib.verifyCalldata( - proof_, credMerkleRootHash, keccak256(bytes.concat(keccak256(abi.encode(minter_, leafPart_)))) + proof_, credMerkleRootHash, keccak256(bytes.concat(keccak256(abi.encode(minter_, ref_, artId_, mintArgs_.imageURI, leafPart_)))) ) ) { revert InvalidMerkleProof(); } In function signatureClaim:

- (uint256 expiresIn_, address minter_, address ref_, address verifier_, uint256 artId_,, bytes32 data_) = abi.decode(encodeData_, (uint256, address, address, address, uint256, uint256, bytes32)); + (uint256 expiresIn_, address minter_, address ref_, address verifier_, uint256 artId_,, bytes32 data_, string memory uri) = abi.decode(encodeData_, (uint256, address, address, address, uint256, uint256, bytes32, string)); + mintArgs_.imageURI = uri; Tools Used Foundry, VSCode ZaK3939 (Phi) confirmed and commented:

Regarding the issue with ref_ (referrer address): As you pointed out, the manipulation of referrer addresses is a phenomenon seen on other platforms as well, and may not be considered a significant problem.

Incorrect assertion about artId_: The report’s claim that “Art 2 can be claimed with only approval for Art 1” misunderstands the actual operation of the code. This assertion is not accurate for the following reasons:

PhiArt storage art = arts[artId_]; bytes32 credMerkleRootHash = credMerkleRoot[art.credChainId][art.credId]; if (minter_ == address(0) || !art.verificationType.eq("MERKLE") || credMerkleRootHash == bytes32(0)) { revert InvalidMerkleProof(); } if ( !MerkleProofLib.verifyCalldata( proof_, credMerkleRootHash, keccak256(bytes.concat(keccak256(abi.encode(minter_, leafPart_)))) ) ) { revert InvalidMerkleProof(); } This code checks the verification type and credMerkleRootHash corresponding to the specified artId_. In other words, even if a different art ID is used, the correct verification information for that art is required. Therefore, it is not possible to claim Art 2 with only the approval for Art 1.

Issue with imageURI: The lack of verification for the imageURI parameter remains a potential problem. This could allow unapproved image URLs to be set.

Main issue after correction: lack of verification for imageURI:

Attackers may be able to specify unapproved image URLs. This could result in inappropriate images or unintended content being associated with the NFT.

0xDjango (judge) decreased severity to Medium and commented:

Valid medium. The user has the ability to provide any imageURI, potentially to the detriment of the protocol and its users.

# [M-06] PhiNFT1155 contracts continue sending fees/royalties to old protocol destination address

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

Submitted by MrPotatoMagic, also found by hgrano, 20centclub, 0xNirix, and 0xCiphky When a PhiNFT1155.sol contract instance is created from the PhiFactory contract, the initialize() function is called to setup it. One of the parameters passed along is the protocolFeeDestination address, which is used as the default royalty recipient (see here ) and the address that will receive the artCreate fees (see here ).

Issue The issue is that if the protocolFeeDestinationAddress contract is changed in the PhiFactory contract by the owner using function setProtocolFeeDestination(), the existing PhiNFT1155.sol contract instances created would still be using the old protocol fee destination address stored in them since it is not being dynamically retrieved from the PhiFactory contract. The protocolFeeDestination address could be updated for several reasons such as in case it is compromised, becomes malicious or there has been a decision made to make the address non-privileged.

## Impact

Due to this, the old protocol fee destination address would still be receiving all the royalties (i.e., 500 bps = 5% ) from all PhiNFT1155.sol contract instances. It would also be receiving all the artCreateFee amount of tokens when art is created through the createArtFromFactory() function.

## Recommended Mitigation Steps

To solve the issue in CreatorRoyaltiesControl, consider retrieving the default royalty recipient variable dynamically from the phiFactory contract. The same would also need to be applied to the protocolFeeDestination address currently stored in the PhiNFT1155.sol contract.

Using dynamic retrieval from the factory would solve the issue for all PhiNFT1155.sol contract instances that were created.

## Assessed type

Error ZaK3939 (Phi) confirmed and commented:

This is a good observation. I appreciate it.

MrPotatoMagic (warden) commented:

@0xDjango - Could you please re-consider this issue and its dups for High-severity? It seems like the impact is rather suited as a high-severity bug since the protocol would be losing all the fees and royalties from all the PhiNFT1155.sol instances ever created as well as considering that the accumulated amount would become large over time.

bronze_pickaxe (warden) commented:

This has been correctly identified as a Medium severity issue. The preconditions are very unlikely - project changing their protocolFeeDestination address is extremely unlikely. Low likelihood + High impact = Medium severity.

0xDjango (judge) commented:

I agree that this issue has high impact, but requires that the fee destination address be changed, which is rather unlikely in practice. Even in the case of a fee address update, the older fee address may still be functional and the desired location for the older fee accrual. Sticking with medium for these reasons.

# [M-07] Attacker can DOS user from selling shares of a credId

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

credId Submitted by MrPotatoMagic, also found by 0xCiphky, 0xNirix, Tigerfrake, typicalHuman, 0xc0ffEE, DigiSafe, eLSeR17, onthehunt11, rscodes, Jorgect, 20centclub, dimah7, Gosho, and Beosin The Cred.sol contract provides a buyShareCredFor() function which anyone can use to buy shares for another curator. We also know that once a share is bought, the curator needs to serve a 10 minute share lock period before being able to sell those tokens.

## Impact

The issue is that an attacker can purchase 1 share every 10 minutes on behalf of the curator to DOS them from selling. As we can see on the bonding share price curve docs here, the price to buy 1 share starts from 2.46 USD and barely increases. Each share bought = 10 minutes of delay for the victim user.

Delaying the user from selling is not the only problem though. During these delays, the price of the share can drop, which could ruin a user’s trade strategy causing to incur more loss.

## Recommended Mitigation Steps

Consider removing the buyShareCredFor() function since it does more harm than good.

## Assessed type

Timing ZaK3939 (Phi) confirmed and commented:

Thank you for this submit. I like this PR.

cats (warden) commented:

This scenario works only in a sandbox environment where users A and C are aware of the trading intentions of user B. The gradual increase/decrease of signal prices is a protocol design as per the curation price spreadsheet, meaning users engaging in signal trading are aware that their signals can appreciate/depreciate with each transaction. User B’s signals depreciating in price due to A and C’s sells is an intended design, regardless if a timelock is imposed or not. One could argue that a whale signal holder could dump their signals when other curators buy from the same cred - this is still not a vulnerability, it’s rules of the game and it also relies on the prerequisite that users willingly bought signals in a cred where there is a whale holder.

The issue is entirely speculative, requires information that is inaccessible in a real world scenario and requires the attacker to just gift free tokens to his “victim”. Also, what if user B front-runs their attack with one of his own to purchase a ton of signals to make the attacker’s signal purchase costly?

This issue is QA at best.

MrPotatoMagic (warden) commented:

@cats, I believe you’re misunderstanding the issue at hand would appreciate if you understand the root cause of the issue first.

regardless if a timelock is imposed or not.

The issue isn’t talking about the intended design of a timelock being imposed. The whole point of the issue is that other users should not be able to extend someone else’s existing share lock period. Surely users have to do their due diligence while taking a trade knowing that there would be the original timelock of 10 minutes imposed by their “own trade”. But they’re not aware beforehand that after those 10 minutes, they won’t be able to sell those shares and perform their expected trade. Fundamentally, other users should not be allowed to manipulate another user’s trade.

The issue isn’t that complicated:

User buys shares.

Waits for 10 minutes.

User tries to sell shares for profit but fails.

Market dumps on the user.

The user not only loses their profit but would be in net loss.

this is still not a vulnerability, it’s rules of the game and it also relies on the prerequisite that users willingly bought signals in a cred where there is a whale holder.

Users are aware of this risk like in any other market. It’s like you’re saying I won’t buy BTC cause Blackrock can dump on me. A user is looking to profit from future user buys. If someone can prevent a user from selling at any time by extending their original share lock period, that’s a bug.

kuprum (warden) commented:

@0xDjango, I would kindly ask you to take a second look at this finding.

I was considering this when I was performing the audit, and decided against submitting this, because it’s not a vulnerability. Let’s consider the implications of this “attack”, specifically of what profit someone would extract from the attack?

In the example, users C and A seem to be profiting by preventing B from selling their shares. But for that:

They would need to know that B wants to sell their shares, which is already unrealistic.

Even if they know B’s intentions: instead of running to sell their shares directly; i.e., extracting the profits immediately, they for some reason frontrun B by buying shares for them; i.e., they run to experience a net loss first instead of immediately extracting profit.

Overall, it’s just an unrealistic assumption that someone would organize a DoS via buying shares at increasing prices, for someone else, only to be able to delay them for 10 minutes from selling their shares.

Finally, if you disregard the unrealistic assumption above that someone knew someone else’s intention to sell their shares, what is the net effect of this “attack”?

The attacker spends funds to buy shares for the victim… Each time investing more and more money to delay the victim for another 10 minutes (!)… While at the end the victim, who now owns substantially more shares, is able to sell them at increased prices, which increased specifically due to the executed attack.

With all due respect, this is a bunch of unrealistic assumptions stitched together.

0xDjango (judge) commented:

This is a valid griefing scenario. While it does involve the attacker “gifting” a share of increasing value, it temporarily locks someone’s funds. Higher level, this verges into “market manipulation” as anyone is able to prop up the market by prohibiting sales. Also, imagine if an exploit occurs or a zero-day vulnerability is leaked. Blocking user withdrawals for even 10 minutes will have severe impact. Given that such an example is low likelihood, this will remain as Medium.

Note: For full discussion, see here.

# [M-08] Refunds sent to incorrect addresses in certain cases

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-phi
- **Source snapshot:** competitions/2024-08-phi/final_report.html

Submitted by 0xCiphky, also found by MrPotatoMagic, pep7siup, aldarion, hgrano, McToady, TECHFUND-inc ( 1, 2 ), pipidu83, 13u9, typicalHuman ( 1, 2 ), 0xc0ffEE ( 1, 2 ), DanielArmstrong ( 1, 2 ), Fitro, th3l1ghtd3m0n, ironside, farismaulana ( 1, 2 ), smaul, eLSeR17, federodes, robertodf99, CAUsr ( 1, 2 ), OpaBatyo ( 1, 2 ), kn0t, 0xNirix, avoloder, DigiSafe, jesusrod15, hail_the_lord, pfapostol, 0xAkira, KupiaSec, 0xrex ( 1, 2 ), and rbserver ( 1, 2 ) The _processClaim internal function is responsible for calling the claimFromFactory function in the art contract, transferring the protocol fees to the protocolFeeDestination, and handling any applicable refunds.

function _processClaim (...

) private { PhiArt storage art = arts [ artId_ ]; // Handle refund uint256 mintFee = getArtMintFee ( artId_, quantity_ ); if (( etherValue_ - mintFee ) > 0 ) { _msgSender ().

safeTransferETH ( etherValue_ - mintFee ); } protocolFeeDestination.

safeTransferETH ( mintProtocolFee * quantity_ ); ( bool success_,) = art.

artAddress.

call { value:

mintFee - mintProtocolFee * quantity_ }( abi.

encodeWithSignature ( "claimFromFactory(uint256,address,address,address,uint256,bytes32,string)", artId_, minter_, ref_, verifier_, quantity_, data_, imageURI_ ) ); if (!

success_ ) revert ClaimFailed (); } When a user sends excess funds ( msg.value ) over the required mintFee, the difference is intended to be refunded to the user. This works as expected when the signatureClaim or merkleClaim functions are called directly in the PhiFactory contract by the user since msg.sender is the correct address (User). However, in some cases refunds are sent to the wrong address.

Case 1:

Claim function and batchClaim function In the batchClaim function, multiple claims are processed by calling the claim function with this (the current contract) as the caller, resulting in the msg.sender being the PhiFactory contract rather than the actual user.

function batchClaim ( bytes [] calldata encodeDatas_, uint256 [] calldata ethValue_ ) external payable { if ( encodeDatas_.

length != ethValue_.

length ) revert ArrayLengthMismatch (); // calc claim fee uint256 totalEthValue; for ( uint256 i = 0; i < encodeDatas_.

length; i ++) { totalEthValue = totalEthValue + ethValue_ [ i ]; } if ( msg.

value != totalEthValue ) revert InvalidEthValue (); for ( uint256 i = 0; i < encodeDatas_.

length; i ++) { this.

claim { value:

ethValue_ [ i ] }( encodeDatas_ [ i ]); } Similarly, the claim function also uses this when calling either the signatureClaim or merkleClaim functions, further propagating the incorrect msg.sender value. Consequently, any refunds are sent to the PhiFactory contract instead of the actual user, causing an incorrect refund.

Case 2:

signatureClaim function and merkleClaim function in the Claimable contract The signatureClaim and merkleClaim functions are also callable through the Claimable contract, which is part of the PhiNFT1155 contract. However, like in the previous case, the msg.sender will be incorrect being the PhiNFT1155 contract itself, rather than the actual user who initiated the call. This results in refunds being directed to the PhiNFT1155 contract rather than the user. In this case, funds are also not able to be pulled out by the owner, effectively locking them in the contract.

## Impact

Incorrect handling of msg.sender results in refunds being sent to contracts (PhiFactory or PhiNFT1155) rather than the intended users.

While the loss of funds due to overpayment is more of a user error, the primary concern here is that the intended logic for refunding users is incorrect which was specifically mentioned by the protocol to look out for.

Function Reliability:

Ensure the contract behaves consistently under various conditions.

Function Correctness:

Ensure the smart contract logic correctly implements the intended functionality without errors.

Reference

## Rejected Primary Findings

# Rejected Primary Findings: Phi

# Centralization risk by giving a single user the privilege of pause/unpause certain operations.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-471
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/471
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-471.md

## Brief Summary

Only the address who own the smart contract can `pause` and `unpause` operations; but what if this address is no longer accessible? The Openzeppelin `PausableUpgradeable` librairie is use in the `Cred`, `PhiFactory` smart contracts; and the ability to pause and unpause some operations is often controlled by a single privileged account, `the owner`. But if, after pausing one or more operations, the private key of that account is corrupted or lost, no other address will be able to remedy the situation and the paused operations will remain there and can no longer be accessed by any users.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing receive function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-563
- **Submitter:** 0xAkira
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/563
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-563.md

## Brief Summary

The `PhiFactory` contract does not have a function to **receive** ether. Thus the contract cannot **receive** ether if there is a success check after a low-level `call` function or if `safeTransferETH` is used. Since the contract has the withdraw function it should have the function of accepting ether, because the contract, for example, will not be able to transfer ether using the withdraw function of `WETH` and similar tokens.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Reentrancy Vulnerability in `_handleTrade` Function due to Missing `nonReentrant` Modifier

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-516
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/516
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-516.md

## Brief Summary

The `_handleTrade` function is responsible for managing trading operations within the contract. It is a critical function that likely involves transferring tokens, updating balances, and interacting with other contracts. However, the absence of the `nonReentrant` modifier in this function opens up the possibility of a reentrancy attack, where a malicious actor could exploit the function to repeatedly call itself (or another vulnerable function) before the initial execution completes. In a typical reentrancy attack, an attacker can manipulate the contract's state or balances by reentering the function, leading to multiple unintended executions of critical operations, such as token transfers...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Inconsistent Function Signatures in `Claimable` Contract Leading to Execution Failures

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-575
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/575
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-575.md

## Brief Summary

The `Claimable` contract contains two external functions, `signatureClaim` and `merkleClaim`, intended to handle different types of claims within the system. These functions are supposed to interact with the `IPhiFactory` contract to process claims. However, there is a critical mismatch between the function signatures in the `Claimable` contract and the corresponding functions defined in the `IPhiFactory` interface. The `signatureClaim` function in the `Claimable` contract does not accept any parameters explicitly. Instead, it decodes the required data from the transaction's calldata within the function itself. This differs from the `signatureClaim` function in the `IPhiFactory` interface,...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Issue in `getTotalMintFee` Function Due to Missing Array Length Validation

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-594
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/594
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-594.md

## Brief Summary

The `getTotalMintFee` function is responsible for calculating the total mint fee for multiple art items. It takes in two arrays as input: `artId_` (an array of art IDs) and `quantitys_` (an array of corresponding quantities). The function then iterates over these arrays to calculate the total mint fee. However, the function does not check whether the lengths of the two input arrays are equal before starting the loop. If the `quantitys_` array is shorter than the `artId_` array, accessing an element of quantitys_ beyond its length could cause an out-of-bounds error. If the `quantitys_` array is longer than the `artId_` array, the extra quantities would be ignored, potentially resulting in an...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validation in `createArtFromFactory` Function of `PhiNFT1155` Contract

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-596
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/596
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-596.md

## Brief Summary

The `createArtFromFactory` function is responsible for creating a new art token within the `PhiNFT1155` contract. However, it does not perform any validation on the `artId_` parameter, which introduces several risks. The function does not check whether the `artId_` has already been used. This omission allows for the potential reuse of `artId_`, leading to inconsistencies in the mappings `_artIdToTokenId` and `_tokenIdToArtId`. Without validation, there is a risk that `artId_` could be reused accidentally or maliciously, resulting in overwriting or hijacking another art’s data. This could lead to severe issues, such as the incorrect assignment of tokens or loss of data integrity. Since there...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# Ponteial Race Condition in `createArtFromFactory` Function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-601
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/601
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-601.md

## Brief Summary

The `createArtFromFactory` function in the `PhiNFT1155` contract is susceptible to a potential race condition, especially on high-throughput and parallel-processing blockchains like Base, Optimism, BeraChain, and other EVM-compatible chains. This vulnerability arises due to the way the `tokenIdCounter` is incremented after it has been used in critical operations. If exploited, this issue could lead to the assignment of the same `tokenId` to multiple art pieces, resulting in data corruption, loss of uniqueness, and potential disruption of the system's integrity. The createArtFromFactory function is responsible for creating a new art token based on a given artId_. It maps this artId_ to a tok...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Zero Address Issue in `claimFromFactory`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-628
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/628
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-628.md

## Brief Summary

The claimFromFactory function is designed to allow users to claim art tokens from the Phi Factory contract. The function accepts several parameters, including the minter_, ref_, and verifier_ addresses, which play important roles in the minting and referral process. However, the function lacks proper validation for these addresses, particularly the zero address (address(0)), which represents an invalid or uninitialized address in Solidity. If `minter_` is set to address(0), the function would attempt to mint tokens to this invalid address. Since address(0) cannot hold tokens, these tokens would effectively be burned or lost, resulting in an irreversible loss for the user. If `ref_` or `veri...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing validation in `safeBatchTransferFrom` Function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-630
- **Submitter:** 0xBeastBoy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/630
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-630.md

## Brief Summary

The `safeBatchTransferFrom` function allows for the transfer of multiple tokens in a single transaction. This function takes in several parameters, including arrays of token IDs (`ids_`) and the corresponding quantities to be transferred (`values_`). According to the ERC-1155 standard, these two arrays must have the same length, with each element in ids_ corresponding to the same index element in values_. This function lacks a critical validation check for ensuring that the lengths of the ids_ and values_ arrays are equal. This omission can lead to various issues, including unexpected behavior, potential loss of tokens, or contract misbehavior. The function is responsible for transferring m...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Issue with Fee-on-Transfer and Blacklisted Tokens in `deposit` Function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-633
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/633
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-633.md

## Brief Summary

The `deposit` function is responsible for accepting a deposit in Ether for a specific credId, updating the contract's internal balance for that `credId`, and emitting a Deposit event. The function checks whether the `msg.value` (the Ether sent with the transaction) matches the amount specified in the function call. If these values do not match, the transaction is reverted. It might be vulnerable to issues related to fee-on-transfer tokens and blacklisted tokens. Specifically, the function assumes that the amount of tokens sent in a transaction will always match the msg.value specified by the sender. However, if the deposited tokens have transfer fees or are subject to blacklisting (e.g., US...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Division by Zero Vulnerability in the `_curve` Function of the `BondingCurve` Contract

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-638
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/638
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-638.md

## Brief Summary

The `_curve` function is responsible for calculating the curve price of a specific target amount based on the bonding curve logic. The calculation involves a division operation where the denominator is derived from subtracting targetAmount_ from a constant TOTAL_SUPPLY_FACTOR. The relevant line of code is: This issue arises when the `targetAmount_` parameter equals the `TOTAL_SUPPLY_FACTOR`. If this condition is met, the denominator in the division operation becomes zero, causing the entire transaction to revert. This issue can have serious implications for the contract's functionality, potentially leading to failed operations and loss of funds. Impact If this condition is met during a tran...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Strict Check used with `msg.value`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-648
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/648
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-648.md

## Brief Summary

The functions `handleRewardsAndGetValueSent`, `batchClaim`, and `depositBatch` across different contracts use a strict equality (`!=`) check to compare the `msg.value` (the amount of ETH sent by the caller) against the expected amount required by the function logic. This approach can cause unnecessary transaction reverts due to minor discrepancies in the transferred ETH amount, especially when interacting with fee-on-transfer tokens or dealing with imprecise calculations. Exact matching of `msg.value` can be challenging, especially when dealing with varying gas fees or slight calculation differences. This may cause transactions to revert unnecessarily, frustrating users. In cases where fee-...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Signature Malleability in the `_verifySignature` Function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-654
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/654
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-654.md

## Brief Summary

The `_verifySignature` function in your `RewardControl` contract is responsible for verifying EIP-712 signatures, primarily for enabling gasless withdrawals. While this function leverages the `SignatureCheckerLib` from Solady for signature verification, it is vulnerable to two critical issues: Signature Malleability and Front-Running Attacks. These vulnerabilities could undermine the integrity of the contract's operations, potentially allowing for replay attacks, unauthorized actions, and denial-of-service conditions. Signature malleability occurs when a cryptographic signature can be modified without altering the underlying message, resulting in multiple valid signatures for the same messa...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Sandwich Attacks in BondingCurve's Price Calculation Functions

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-658
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/658
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-658.md

## Brief Summary

In `BondingCurve` contract, the `getPrice`, `getBuyPrice`, and `getSellPrice` functions calculate the price of shares based on the current supply (`supply_`) and the amount being bought or sold (`amount_`). The calculation is deterministic based on the current state of the contract, particularly the supply_. An attacker can observe a pending transaction on the network where a user intends to buy or sell shares. The attacker can then send a transaction that changes the supply of shares before the user's transaction is processed. This manipulation affects the price calculation in the user's transaction. For example, if a user intends to buy shares, the attacker could buy shares first, increas...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_19_group

# User can prevent the protocol fee from going to the `protocolFeeDestination` address via reeentrancy possibility in `Cred::_handleTrade()` function`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-408
- **Submitter:** 0xBugSlayer
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/408
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-408.md

## Brief Summary

User can prevent the protocol fee from going where it's supposed to go, by reentering the `Cred::_handleTrade()` function

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `claim()` function calls functions with `external` visibility in `PhiFactory`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-410
- **Submitter:** 0xBugSlayer
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/410
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-410.md

## Brief Summary

This leads to the full crash of the system, since functions with `external` visibility can't be called by internally in the contract

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Missing whenNotPaused modifier in safeBatchTransferFrom and safeTransferFrom

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-201
- **Submitter:** 0xastronatey
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/201
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-201.md

## Brief Summary

All the vital external function of the PhiNFT1155 have whenNotPasued modifier. However, the safeTransferFrom and safeBatchTransferFrom functions do not have the whenNotPaused modifier. Impact Even if governance decides to halt all activity, they cannot prevent the transfer of profile NFTs. Here’s an example where stopping token transfers was actually beneficial:https://x.com/flashfish0x/status/1466369783016869892

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing pause modifier on claim and batchClaim()

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-202
- **Submitter:** 0xastronatey
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/202
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-202.md

## Brief Summary

The PhiFactory contract implements a pause mechanism, intended to halt operations during emergencies or maintenance. However, two core functions lack the whenNotPaused modifier: claim() batchClaim() These functions can be executed even when the contract is in a paused state. Impact Users would be able to claim or batch claim rewards during a pause, potentially draining funds or minting tokens when it's not intended.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_177_group

# Anyone can create art with expired signatures, bypassing the intended time restriction.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-25
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/25
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-25.md

## Brief Summary

In the [_validateArtCreationSignature](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/PhiFactory.sol#L589-L593) function in `PhiFactory.so`l, the signature expiration check is performed against the `expiresIn_` value decoded from `signedData_` > However, the actual expiration time for the art creation is specified in the `endTime` field of the `CreateConfig` struct, which is not checked in this function. Impact The art creator suffers from the inability to enforce the intended signature expiration time for art creation. An attacker can create art using an expired signature, bypassing the intended time restriction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_135_group

# Creator will be unable to create new creds when contract is not paused

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-45
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/45
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-45.md

## Brief Summary

Insufficient payment validation in `createCred` will cause the function to revert for creators as they will be unable to create new creds even when the contract is not paused. Root Cause In Cred.sol, the [createCred](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L232-L242) function calls [buyShareCred(credIdCounter, 1, 0)](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L570) after creating a new cred, without ensuring that sufficient `msg.value` is provided to cover the price, protocol fee, and creator fee for this initial share purchase. After creating a new cred, `createCred` calls...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_92_group

# Creator will be unable to set maximum royalty for their creds

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-47
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/47
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-47.md

## Brief Summary

An incorrect royalty range check will cause the [createCred](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L232-L271) function to revert for creators as they will be unable to set the maximum allowed royalty values for their creds. **Root Cause** In Cred.sol:158-160, there is an incorrect royalty range check: [Cred.sol#L269-L270](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L269-L271) > The condition checks if either `buyShareRoyalty_` or `sellShareRoyalty_` is strictly greater than `MAX_ROYALTY_RANGE`. However, it doesn't allow the royalty values to be equal to `MAX_ROYALTY_RANG`E...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_104_group

# Inconsistent `credId` emitted in `CredCreated` event

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-48
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/48
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-48.md

## Brief Summary

The incorrect incrementation of `credIdCounter` will cause an inconsistency between the `credId` emitted in the `CredCreated` event and the actual `credId` assigned to the new cred, leading to confusion and difficulty in tracking cred creation for users and external systems relying on the `CredCreated` event. Root Cause In Cred.sol, the [_createCredInternal](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L572-L577) function incorrectly increments the `credIdCounter` after emitting the `CredCreated` event but before returning the new `credId`. Impact The inconsistency between the emitted `credId` in the `CredCreated` event and the actual...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Spamming creating Cred for arbitrary creator by replay signature

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-88
- **Submitter:** 0xc0ffEE
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/88
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-88.md

## Brief Summary

An user can create Cred given that a valid signature from signer is not expired. This allows a malicious user to spam creating Cred for arbitrary set of `(creator_, buyShareRoyalty_, sellShareRoyalty_)`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# Updating phiSignerAddress invalidates all pending signatures causing art creation failures

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-546
- **Submitter:** 20centclub
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/546
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-546.md

## Brief Summary

The `phiSignerAddress` in the contract is used to verify that signatures received from the backend are valid. The signatures are checked against this address in the `_validateArtCreationSignature()` function. However, if the `phiSignerAddress` is updated using the `setPhiSignerAddress()` function, all previously issued signatures that are still pending will become invalid. This is because the signer address stored in the smart contract is no longer the same as the address that generated the signatures. As a result, all pending art creation transactions that rely on these signatures will fail. Impact When the `phiSignerAddress` is changed, it causes all pending signatures to become invalid....

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_102_group

# Large Input Array can DOS depositBatch function(RewardControl.sol::Line 54)

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-427
- **Submitter:** 54thMass
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/427
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-427.md

## Brief Summary

The depositBatch function allows a user to deposit ETH on behalf of others (RewardControl.sol::Line 54) by providing an address[] of recipient addresses and a uint256[] of amounts to deposit. depositBatch can be DOS'd if unusually large address arrays are passed during the function call accompanied with zero deposit amounts. A very large number of addresses can be created by the malicious actor and 0 amounts to be passed to the uint256[].This attack renders the function unusable to other users without having to deposit any ETH. Impact: Prevents other users from calling depositBatch. Solution: 1) Implement a check that restricts array lengths of recipients and amounts to reasonable number (a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_143_group

# Users will overpay for minting due to double counting of `mintFee_` (`PhiRewards::handleRewardsAndGetValueSent`)

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-114
- **Submitter:** Agontuk
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/114
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-114.md

## Brief Summary

The `PhiRewards` contract manages deposits and withdrawals for art rewards in the system. The [`handleRewardsAndGetValueSent()` function](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/reward/PhiRewards.sol#L123-L148) is responsible for processing rewards when a user mints an art piece. This function relies on the [`computeMintReward()` function](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/reward/PhiRewards.sol#L153-L155) to calculate the total reward amount and verify that the sent value matches the expected reward. However, there is a flaw in the reward calculation logic that leads to the `mintFee...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Underflow vulnerability in the createArtFromFactory() function will result in loss of protocol funds

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-350
- **Submitter:** BajagaSec
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/350
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-350.md

## Brief Summary

Summary: A malicious user can exploit the underflow vulnerability in the `createArtFromFactory()` function to steal funds from the protocol. The exploit is highly probable since it can be carried out at any time by an attacker. Vulnerability details The `createArtFromFactory()` function in the `PhiNFT1155.sol` contract lacks proper checks to prevent an underflow when calculating the difference between `msg.value` and `artFee`. Specifically, if `msg.value` is less than `artFee`, the subtraction will underflow which will bypassing the check, resulting in an unintended large amount being transferred to the `msg.sender`. This occurs in the following section of the code: Link: https://github.com...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Front-Running Vulnerability in Cred Buying Mechanism: A High-Severity Threat to Fairness and Security

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-353
- **Submitter:** Bz
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/353
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-353.md

## Brief Summary

The attack scenario involves an attacker front-running a buyer's transaction to buy a cred. The attacker exploits the lack of proper synchronization between the cred buying mechanism and the bonding curve to manipulate the price of the cred. **Here's a step-by-step breakdown:** - The buyer initiates a transaction to buy a cred at the current price. - The attacker front-runs the buyer's transaction, buying the cred at the current price. - The attacker's transaction is executed before the buyer's transaction, causing the price of the cred to increase. - The buyer's transaction is executed at the new, higher price, resulting in the buyer paying more for the cred than they intended. **Root Caus...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_190_group

# Unprotected Zero-Value Transactions in phiRewards Contract Enable Denial of Service (DoS) and Other Attacks

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-612
- **Submitter:** Bz
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/612
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-612.md

## Brief Summary

The phiRewards contract is vulnerable to a denial of service (DoS) attack and other potential issues due to its acceptance of zero-value deposits and withdrawals. This allows an attacker to repeatedly call the contract with zero-value transactions, potentially leading to a denial of service and other negative consequences. Detail of Vulnerability: The vulnerability arises from the fact that the contract does not validate or restrict transactions with a value of zero ether. This allows an attacker to repeatedly call the deposit and withdraw functions with a value of zero ether, potentially causing a denial of service and other issues. Impact If exploited, - this vulnerability could lead to a...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Artist can change url of art without signature.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-488
- **Submitter:** DanielArmstrong
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/488
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-488.md

## Brief Summary

It can damage the buyer of that art because the `url` can be changed after they buy the art.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# PhiNFT1155.sol does not inherit from IPhiNFT1155Ownable interface

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-236
- **Submitter:** Decap
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/236
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-236.md

## Brief Summary

Contract `PhiNFT1155.sol` does not inherit from interface `IPhiNFT1155Ownable`. However contract `PhiFactory.sol` makes multiple calls to this interface assuming that `artAddress` (PhiNFT1155 contract) is indeed `IPhiNFT1155Ownable` which is not the case. This can lead to unexpected reverts and behaviours.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# [M-3] User can set his other address as a referral and claim NFT with a discount

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-195
- **Submitter:** DigiSafe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/195
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-195.md

## Brief Summary

The protocol supports referrals which can get rewards when the user claims an NFT with them. There is a check for users to set them as a referral directly, but the user setting his other address as a referral cannot be stopped. This gives him a chance to claim the NFT with a discount Users can claim NFTs with a discount

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# You can block sale/purchase through the condition if (block.timestamp <= lastTradeTimestamp[credId_][curator_] + SHARE_LOCK_PERIOD)

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-450
- **Submitter:** Inspecktor
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/450
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-450.md

## Brief Summary

In the Cred._handleTrade() function, the following condition is checked before trading: if (block.timestamp <= lastTradeTimestamp[credId_][curator_] + SHARE_LOCK_PERIOD) { revert ShareLockPeriodNotPassed( block.timestamp, lastTradeTimestamp[credId_][curator_] + SHARE_LOCK_PERIOD ); } However, this condition can be used to permanently block trading.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_38_group

# Under certain conditions getPriceData() may return 0 price and fee due to lack of validation at 0

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-451
- **Submitter:** Inspecktor
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/451
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-451.md

## Brief Summary

In the _handleTrade() function (https://github.com/code-423n4/2024-08-phi/blob/main/src/Cred.sol#L612) and _validateAndCalculateBatch() (https://github.com/code-423n4/2024-08-phi/blob/main/src/Cred.sol#L858), the getPriceData() function from the BondingCurve.sol contract is called. The getPriceData() function returns the uint256 price, uint256 protocolFee, uint256 creatorFee parameters. Under certain conditions, getPriceData() may return 0 price and fee due to lack of validation at 0.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# Art creation may be subject to reorg attack and DOS

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-453
- **Submitter:** Inspecktor
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/453
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-453.md

## Brief Summary

In the PhiFactory._createNewNFTContract() function, a new art is created (https://github.com/code-423n4/2024-08-phi/blob/main/src/PhiFactory.sol#L631-L632): address payable newArt = payable(erc1155ArtAddress.cloneDeterministic(keccak256(abi.encodePacked(block.chainid, newArtId, credId)))); The parameters received as salt are: block.chainid, newArtId, credId

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# Possible theft of funds in the RewardControl.sol contract

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-454
- **Submitter:** Inspecktor
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/454
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-454.md

## Brief Summary

The RewardControl.sol contract has a withdrawWithSig() function, which executes a withdrawal of protocol rewards via signature. This function can be used to steal funds.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_55_group

# Typo in RewardControl.withdrawFor() function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-455
- **Submitter:** Inspecktor
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/455
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-455.md

## Brief Summary

Typo in RewardControl.withdrawFor() function

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_93_group

# Rounding to zero can allow an attacker exploit the `curatorRewardsDistributor.sol`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-294
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/294
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-294.md

## Brief Summary

Solidity does not have float number so if you divide a divisor major than the numerator, the operation will round to zero. This is a common problem in smart contracts and in some cases attackers could exploit this rounding issues. In the context of the Phi system there is a instance where an attacker can make loss of funds to the project and users see the `distribute` function: [[Link]](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/reward/CuratorRewardsDistributor.sol#L77) As you can see the ` uint256 userRewards = (distributeAmount * userAmounts) / totalNum; ` can round to zero if the conditions met. If `userRewards ` round to zero then the tot...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Invalid distribution in `PhiRewards.sol`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-304
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/304
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-304.md

## Brief Summary

As you can see the distribution of rewards in the `PhiRewards.sol` is the following : If we sum this values we get 0.0025 if we go to the Docs we see that the fees in the Docs are the next one: As you can see the all the different set up the total fee is 0.00030 in the contract the total fee amount is 0.00025 Impact The phi protocol is not being consistent with the docs and the code, if this is a error in the docs then this should be a low, but if the error is in the code then this will be a medium because the functions are not working how the projects is expecting.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Return Value of Low-Level Call Leading to Potential Data Inconsistency

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-126
- **Submitter:** JuggerNaut63
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/126
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-126.md

## Brief Summary

The unchecked return value of low-level `call()` in `_createNewNFTContract` and `_useExistingNFTContract` can lead to data inconsistency and incorrect assumptions about the state of NFT creation. This can result in the system believing that an NFT has been successfully created when it has not, potentially leading to further errors in business logic and financial discrepancies.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Precision loss in share lock period by an off-by-one error, which enables early trading

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-104
- **Submitter:** K42
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/104
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-104.md

## Brief Summary

Impact is off-by-one error in the share lock period check (using `<=` instead of `<`) allows trades at exactly `t+600` seconds, violating the intended `t+600+ε` lock duration. This precision loss creates a exploitable time-of-check to time-of-use `TOCTOU` bug, enabling microsecond-level front-running and disrupting the protocol's temporal invariants in cross-contract interactions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# When creating a new cred the caller is limited to purchasing only one share, meaning popular creds risk being sniped

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-464
- **Submitter:** McToady
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/464
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-464.md

## Brief Summary

When a cred is created the caller is minted only the first `credShare`, shown here: After this anyone is free to buy any quantity of shares they wish via `Cred::buyShareCred`. This causes a problem where should a cred that is deemed to be popular is created a sniper (typically a bot) can jump in and instantly buy a large amount of the shares for the newly created `credId` at the cheapest possible price. This is a scenario that has been a common problem for other popular projects that offer shares priced on a bonding curve such as Friend Tech which had significant issues with botting of newly created tokens. [Here](https://x.com/tomwanhh/status/1694715993992700061/photo/1) is a write-up docu...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of validation of `ref_` in `PhiFactory::merkleClaim` means user can effectively claim art without paying the referral fee

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-593
- **Submitter:** McToady
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/593
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-593.md

## Brief Summary

Within the protocol a user can encourage others to mint tokens by sharing a referral link, when using this link the referrer earns a small fee per art piece claimed via their link. At the smart contract level the referrer's address is passed as part of the `encodeData_` to one of the `PhiFactory` `claim`/`signatureClaim`/`merkleClaim` functions. In the case of signature claims the `ref_` provided is part of the signed data ensuring that only the correct referrer receives the mint reward. However for the case of `merkleClaim` there is no such validation meaning the person claiming is free to set an address of their own as the `ref_` meaning they are able to mint without paying this referral...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# Centralized Pause Function Enables Owner to Freeze NFT Operations, Potentially Causing User Losses

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-600
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/600
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-600.md

## Brief Summary

The Phi Protocol is susceptible to significant disruption and potential abuse due to the owner's ability to unilaterally pause the PhiNFT1155 contract. If an NFT contract owner acts maliciously or their account is compromised, pausing the contract can halt core functionalities and negatively impact users. Details - **PhiNFT1155.sol**: - `pause()`: This function allows the contract owner to pause the contract. - `createArtFromFactory`, `claimFromFactory`: These functions are indirectly affected as they revert when the contract is paused. - **Cred.sol**: - Several functions indirectly depend on the NFT contract being unpaused. Malicious Owner Scenarios - **Rug Pull**: An artist could create a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Signature Replay Vulnerability in RewardControl Contract Allows Potential Balance Draining

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-604
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/604
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-604.md

## Brief Summary

The `RewardControl` contract is vulnerable to signature replay attacks due to the implementation of its `withdrawWithSig` function. This vulnerability allows an attacker to potentially drain a user's balance by repeatedly using a single valid signature with a far-future deadline. Vulnerability Details The vulnerability is present in the `withdrawWithSig` function of the `RewardControl` contract. The root cause is a combination of two factors: 1. The ability to set arbitrarily long deadlines for signatures 2. The nonce system only invalidating a signature after it has been successfully used

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Block Timestamp Manipulation in PhiFactory Contract Affects NFT Minting Timing

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-605
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/605
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-605.md

## Brief Summary

The PhiFactory contract uses `block.timestamp` to enforce time-based restrictions on NFT minting and to validate signature expiration. This reliance on `block.timestamp` can potentially be manipulated by miners, leading to security vulnerabilities. Vulnerability Details In Ethereum, `block.timestamp` is set by miners and can be manipulated within a small range (typically up to 15 seconds into the future). While this manipulation is limited, it can still impact time-sensitive operations in smart contracts, especially those requiring precise timing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Array Length Manipulation in Cred Contract Enables Data Corruption and Denial of Service

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-607
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/607
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-607.md

## Brief Summary

The Cred contract utilizes inline assembly to resize arrays within the `getPositionsForCurator` and `_getCuratorData` functions. This approach bypasses Solidity's inherent safety checks, exposing the contract to critical vulnerabilities. Details The contract's manual manipulation of array lengths using assembly can lead to severe security issues, including data corruption and denial of service.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unaccounted ETH Deposits Lead to Balance Inconsistency and Potential ETH Lock-up

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-609
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/609
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-609.md

## Brief Summary

The RewardControl contract manages deposits of ETH, tracking individual balances using the `balanceOf` mapping. However, the contract's actual ETH balance and the sum of all user balances can become inconsistent. Vulnerability Details The deposit functions (`deposit` and `depositBatch`) increase user balances in the `balanceOf` mapping when ETH is sent to the contract. However, there's no mechanism to handle ETH sent directly to the contract address without calling these functions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# Share Lock Period Bypass in Batch Operations

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-650
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/650
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-650.md

## Brief Summary

The Cred contract implements a share trading system with a lock period to prevent rapid buying and selling of shares. However, this lock mechanism is not consistently applied across all trading functions, potentially leading to a vulnerability. The contract implements a share lock period in the `sellShareCred` function: However, this check is missing in the `batchSellShareCred` function, which could allow users to bypass the lock period when selling shares in batch.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_86_group

# Once a Cred's bonding curve is removed from the whitelist, a new one cannot be set and trades will be executed on outdated pricing logic

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-424
- **Submitter:** OpaBatyo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/424
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-424.md

## Brief Summary

Once a Cred's bonding curve is removed from the whitelist, there is no way to assign the Cred a new one so it will continue to execute all trades on outdated pricing logic which has been prohibited.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_82_group

# Possible underflow could miscalculate fees and revert

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-661
- **Submitter:** Parkeq
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/661
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-661.md

## Brief Summary

if the **mintFee** parameter is less than the result of **mintProtocolFee** * **quantity_** we risk the possibility of an underflow resulting on undesired reverts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_71_group

# Incorrect `supportsInterface` implementation in `PhiNFT1155.sol` may lead to loss of royalty fees

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-303
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/303
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-303.md

## Brief Summary

The `supportsInterface` function in the `PhiNFT1155` contract does not explicitly check for the `IERC2981` interface, which is the standard interface for NFT royalties. This could lead to a situation where marketplaces or other contracts that rely on `supportsInterface` to determine royalty support may incorrectly conclude that the contract does not support royalties. As a result, they may not collect or forward royalty fees to the creators, leading to a potential loss of revenue. The implementation of the `supportsInterface` (EIP-165) within the contract is incorrect, potentially leading to a loss of assets, as shown in the scenario below: >A marketplace wants to determine if the PhiNFT115...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Integer Overflow and Underflow

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-435
- **Submitter:** Rikka
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/435
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-435.md

## Brief Summary

The identified issue can lead to integer underflow in the smart contract. Specifically, when the amount_ or amount variables are subtracted from currentSupply, if these values are greater than the currentSupply, it will result in an underflow. This can potentially allow an attacker to manipulate the currentSupply to negative values, leading to unexpected behavior in the contract. This vulnerability can compromise the integrity of the contract, affecting token balances and supply calculations, which can have serious financial implications.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Denial of Service (DoS) Attacks

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-436
- **Submitter:** Rikka
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/436
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-436.md

## Brief Summary

-Denial of Service (DoS) attacks can significantly impact the functionality and reliability of smart contracts. The primary concerns associated with the identified issues are: -Increased Gas Costs: The loops identified in the contract code iterate over potentially large arrays or mappings, which can result in high gas consumption. If an attacker is able to trigger these loops with large inputs, they can cause transactions to fail or exceed the gas limit, effectively rendering the contract functions unusable. -Contract Unavailability: By exploiting the high gas consumption of these loops, an attacker could prevent legitimate users from interacting with the contract. This can lead to situatio...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_91_group

# Potential JSON Injection Vulnerability in Metadata Construction

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-644
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/644
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-644.md

## Brief Summary

The `PhiFactory.sol` constructs JSON metadata strings using user-supplied data without proper sanitization. This occurs in the `contractURI` and `_buildDescription` functions. User-controlled input, particularly the `verificationType` field, is directly incorporated into the JSON string without any sanitization or encoding. Impact An attacker could potentially inject malicious JSON content by manipulating the `verificationType` or other user-supplied fields. This could lead to: - Malformed JSON that breaks client-side parsing - Injection of unexpected data into the NFT metadata - Potential XSS attacks if the metadata is rendered in a web interface without proper escaping While not as severe...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users can lose their funds when they call the `CuratorRewardsDistributor::deposit` with some ether

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-442
- **Submitter:** Spomaria
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/442
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-442.md

## Brief Summary

The `CuratorRewardsDistributor::deposit` function is a payable function that allows ether to be sent to the contract for a particular cred by specifiying its id, parsing it as an argument in the function call. The function has no access control and can therefore be called by any user on the platform. Any amount of ether sent to the contract using the `CuratorRewardsDistributor::deposit` function will be shared to the users who are share holders of that cred. Vulnerability Details The vulnerability lies in the fact that 1. the `CuratorRewardsDistributor::deposit` function lacks access control, enabling just any user make a function call and send ether in the process which cannot be retrieved...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_36_group

# Incorrect `endTime` validation during art creation

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-167
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/167
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-167.md

## Brief Summary

These use of `<=` operator in `endTime` validation results in the function reverting before `endtime` has passed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Invalid time range set in `updateArtSettings()`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-168
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/168
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-168.md

## Brief Summary

Without checking if `endTime_` is equal to `startTime_`, the whole logic will result in an invalid time range. The claim period would be effectively non-existent. The art would be available for claiming only at the exact moment when `startTime` equals `endTime`. This would likely result in no successful claims, as the window is too narrow for practical use.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_43_group

# Missing `verificationType` validation in `signatureClaim()`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-183
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/183
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-183.md

## Brief Summary

The lack of a `verificationType` check in the `signatureClaim()` function allows for unauthorized claims to be processed. This means that users could potentially claim art pieces that were not intended to be verified through `signatures`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_58_group

# Lack `credData` validation during Art initialization may result in potential issues

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-259
- **Submitter:** Tigerfrake
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/259
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-259.md

## Brief Summary

Without `credData` validation, mismatched data can lead to inconsistencies in the system, where the art's attributes do not accurately reflect accurate credentials.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_75_group

# Premature `signature` expiration

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-321
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/321
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-321.md

## Brief Summary

Signatures may become invalid earlier than users anticipate, leading to failed transactions

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_57_group

# Potential overflow risk in `RewardControl::deposit()`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-54
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/54
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-54.md

## Brief Summary

If an `overflow` occurs, the `balance` of the affected address could wrap around to a much smaller value or zero, effectively corrupting the balance tracking. This could lead to significant discrepancies in the contract's accounting.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Denial of Service in RewardControl.sol contract, leads to locking of withdrawal and deposit functionality.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-606
- **Submitter:** Tonchi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/606
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-606.md

## Brief Summary

An attacker can push enormous amounts of withdrawals with amount = 0 in the RewardControl::_withdraw function, there is no minimum limit to the amount to withdraw. The governance or owner needs to spend as much gas as the attacker to stop the DoS attack and process the withdrawal. The withdrawals can't be processed if the governance or owner doesn't have enough money to pay for the gas. Same for RewardControl::deposit and RewardControl::depositBatch function. there is no minimum amount of value to deposit, an attacker can call with amount = 0 and it causes a Dos to deposit. To stop the DoS attack and process the deposit, the governance needs to spend as much gas as the attacker. The deposit...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# changes balance of all intended actors in RewardControl contract but the actual money not deposited to Rewardcontrol contract

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-627
- **Submitter:** Tonchi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/627
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-627.md

## Brief Summary

Only changes balances of actors, no actual money is sent from PhiRewards to RewardControl. So withdrawal doesn't happen in RewardControl if this contract has less money than expected.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# excessive gas uses unnecessarily, lead to revert in trasaction due to high gas cost with large numRecipients

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-631
- **Submitter:** Tonchi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/631
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-631.md

## Brief Summary

Excessive Gas Usage and denial of service in depositBatch function in RewardControl contract, this function changing state with emitting events in a for loop in one transaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# Buying price is greater than selling price, no one buys a depreciating asset and leads to dead protocol

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-639
- **Submitter:** Tonchi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/639
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-639.md

## Brief Summary

In the BondingCurve contract, getBuyPrice for a particular supply and amount is greater than getSellPrice even if we instantly sell after a buy. It means this asset's value gets dropped always and no one wants any depreciating asset which leads to this protocol being abandoned.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `protocolFee` deduction prevents NFT claim despite sufficient ETH sent

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-139
- **Submitter:** TopStar
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/139
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-139.md

## Brief Summary

The `PhiFactory::_processClaim()` function contains a potential vulnerability where the provided `etherValue_` might be insufficient to cover all required transfers and calls within the function. Specifically, if `etherValue_` is less than the `mintFee` calculated by `getArtMintFee(artId_, quantity_)`, the function may fail when attempting to transfer the remaining funds to the artist's contract address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_76_group

# Potential Out-of-Bounds Error in `_removeCredIdPerAddress` Due to Missing Validation

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-414
- **Submitter:** VulnViper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/414
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-414.md

## Brief Summary

The `_removeCredIdPerAddress` function does not check whether `credId_` is a valid index within `_credIdsPerAddressCredIdIndex[sender_]`. If `credId_` is invalid, the function could produce incorrect results or revert unexpectedly due to out-of-bounds array access. An attacker or a malfunctioning contract can exploit this issue to cause unexpected behaviors, potentially corrupting the internal state. This can result in incorrect balances, unauthorized deletions, or contracts becoming unusable.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Distribute Function Fails Entirely if Any Recipient Reverts During Batch Distribution

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-415
- **Submitter:** VulnViper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/415
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-415.md

## Brief Summary

The current implementation of the `distribute` function in `CuratorRewardsDistributor.sol` depends on the `depositBatch` function from `RewardControl.sol` for batch distribution. If any recipient within the batch causes the `depositBatch` function to revert (e.g., due to an invalid address or insufficient balance), the entire `distribute` function will revert, causing no recipients to receive their rewards. Impact A single invalid recipient address or other issue within the batch can prevent all recipients from receiving their rewards. This can lead to significant disruption, particularly if distributing rewards to a large number of recipients.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `batchClaim` Function Reverts Entire Operation on Single Claim Failure

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-501
- **Submitter:** VulnViper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/501
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-501.md

## Brief Summary

In `PhiFactory.sol`, the `batchClaim` function calls the `claim` function multiple times using a loop. If any single call to `claim` within this loop reverts (e.g., due to an invalid Merkle proof or insufficient fee), the entire `batchClaim` operation will fail. This undesired behavior prevents the successful claims from completing. Impact When a batch claim process fails due to an issue with a single claim, it disrupts the entire batch, causing potential inconvenience and inefficiency for users attempting to claim multiple rewards at once. This is particularly problematic for high-volume transactions where the probability of encountering a failed claim increases.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# Caller will receive excess ETH from `batchSellShareCred` function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-137
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/137
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-137.md

## Brief Summary

The caller of the `batchSellShareCred` function will receive excess ETH that should not be transferred. This violates the expected behavior of the function and can lead to unintended consequences and potential vulnerabilities.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_144_group

# Curator will receive incorrect amount of Ether when selling share cred

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-152
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/152
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-152.md

## Brief Summary

In the `sellShareCred` function in [Cred.sol:L182-L183](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L182-L184) This function calls the internal `_handleTrade` function, which handles the trade logic for buying and selling share cred. In the `_handleTrade` function (Cred.sol:588-659), the issue lies in the following lines. When selling share cred, the curator is transferred the price minus the protocol fee and creator fee. However, this does not take into account the actual amount of Ether received by the contract during the sell operation. The curator suffers a loss when selling share cred due to receiving an incorrect amount of Ether...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Expiration Handling: Claims processed after expiration could lead to unauthorized minting.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-140
- **Submitter:** abi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/140
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-140.md

## Brief Summary

The signatureClaim function decodes the `expiresIn_` parameter from calldata but fails to verify if the current block timestamp surpasses this expiration time. Consequently, expired claims may still be processed, enabling unauthorized token minting. This flaw poses significant economic and security risks, as it could be exploited by malicious actors to mint tokens beyond the designated claim period.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Without proper signature validation, unauthorized claims might be processed.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-141
- **Submitter:** abi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/141
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-141.md

## Brief Summary

In the signatureClaim function, the signature is decoded and passed to the `IPhiFactory` contract without any validation to ensure its authenticity. This lack of verification means that an attacker could potentially craft a malicious signature and claim tokens without proper authorization. The contract should implement signature validation, typically using `ecrecover`, to ensure that the signature is indeed from a trusted source and corresponds to the intended message. Without this, the contract is susceptible to unauthorized claims, leading to potential loss or misallocation of tokens.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Rewards can be sweeped pre-maturely because of missing check with closing the reward period.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-177
- **Submitter:** abi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/177
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-177.md

## Brief Summary

This vulnerability refers to a potential issue in the `closeAndSweep` function of the `ContributeRewards` contract. This function allows the `rewardSetter` to close a reward and sweep any remaining tokens back to their address. However, the function does not enforce that the reward period must be closed before sweeping. This means that a `rewardSetter` could potentially sweep the remaining rewards before the claim period has ended, denying legitimate claimants their rewards. To mitigate this, the contract should ensure that the claim period has ended before allowing the sweeping of remaining rewards.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked output of the ECDSA recover function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-257
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/257
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-257.md

## Brief Summary

Detailed description of the impact of this finding. The ECDSA.recover function (in version 2.5.1) returns address(0) if the signature provided is invalid. This function is used in the Cred.sol code: If the oracle signature was invalid, the oracleAddress is set to address(0). Similarly, if the user’s signature is invalid, then the userMessage.signer or the withDrawer is set to address(0). This can result in unintended behavior. For example, it allows users to perform some interactions on behalf of the zero address,

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No arraay length check in _executeBatchTrade.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-263
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/263
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-263.md

## Brief Summary

Detailed description of the impact of this finding. No array length check in _executeBatchTrade.No check for array length check of protocolFees and creatorFees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# wrong implement of getCredBuyPrice

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-266
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/266
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-266.md

## Brief Summary

Detailed description of the impact of this finding. wrong implement of getCredBuyPrice.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# use address(this) instead of "this" in claim.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-268
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/268
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-268.md

## Brief Summary

Detailed description of the impact of this finding. we should use address(this) instead of this.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# PhiRewards should have a widhraw function as its payable.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-270
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/270
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-270.md

## Brief Summary

Detailed description of the impact of this finding. PhiRewards should have a widhraw function as its payable unless amount will not be widhraw.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Cred contract has Multiple Calls in a Single Transaction in function getBatchSellPrice

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-15
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/15
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-15.md

## Brief Summary

Detailed description of the impact of this finding. Contract: Cred Function Name: getBatchSellPrice(uint256[] credIds_, uint256[] amounts_) PC Address: 12031 Estimated Gas Usage: 8420 - 79910 The getBatchSellPrice function in the Cred contract makes multiple external calls within a single transaction. Specifically, the function iterates over a list of credIds_ and invokes the getSellPriceAfterFee method from the external IBondingCurve contract for each item in the list. This pattern introduces a reentrancy risk: 1. Multiple External Calls: If the external IBondingCurve contract is untrusted or compromised, it could re-enter the Cred contract during the execution of the getBatchSellPrice fun...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_48_group

# Validation in _updateRoyalties function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-173
- **Submitter:** dreamcoder
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/173
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-173.md

## Brief Summary

function _updateRoyalties(uint256 tokenId, RoyaltyConfiguration memory configuration) internal { if (configuration.royaltyRecipient == address(0) && configuration.royaltyBPS > 0) { revert InvalidRoyaltyRecipient(); } ... } The _updateRoyalties function checks if royaltyRecipient is a zero address, but it only reverts if royaltyBPS is greater than 0. This might occur problem. You could either always disallow a zero address or handle it in a specific way.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_30_group

# EIP-1155 standard

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-186
- **Submitter:** dreamcoder
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/186
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-186.md

## Brief Summary

You must emit the following events as part of ERC-1155 compliance: TransferSingle, TransferBatch, ApprovalForAll, URI You must implement the following functions as part of ERC-1155 compliance: balanceOf, balanceOfBatch, setApprovalForAll, isApprovedForAll

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# PhiFactory.signatureClaim() can be called multiple times with the same parameters

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-301
- **Submitter:** eLSeR17
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/301
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-301.md

## Brief Summary

When signatureClaim() is invoked, _recoverSigner() is used to check if phiSignerAddress signed encodedData_ and prevent executing the function with unsigned parameters. The problem is that the function does not implement a mechanism that prevents from being called once and again with same parameters, leading to a signature replay vulnerability. The function can be called once and again until expiresIn blockstamp is reached.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_87_group

# Improper Validation in `batchClaim` Function Allows NFT Minting Bypass

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-231
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/231
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-231.md

## Brief Summary

Loss of minting fees and the intended minting verification process are bypassed. An attacker can claim multiple NFTs without meeting the required conditions, potentially gaining undeserved rewards or benefits.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# Curator can buy shares for themselves without updating their share balance

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-232
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/232
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-232.md

## Brief Summary

The curator will an approximate loss of the share amount they intended to buy. The attacker (curator) loses the payment for the shares but doesn't gain the corresponding share balance.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_178_group

# Minter status can be set without minting any tokens

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-291
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/291
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-291.md

## Brief Summary

An attacker can set their minter status for an art without actually minting any tokens or increasing the total minted count. **This breaks a key invariant in the system:** _if an address is marked as having minted an art, then either it was already marked before, or the total minted quantity for that art has increased. Violating this invariant can lead to various inconsistencies and incorrect assumptions in other parts of the system that rely on accurate minter status._ While this issue doesn't directly lead to any loss of funds, it can confuse and undermine the reliability and integrity of the minting process and the data it generates.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Sellers may not receive the expected amount when selling their shares, especially when the supply reaches 0.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-299
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/299
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-299.md

## Brief Summary

The sellers of shares in the bonding curve are directly affected when the supply of shares reaches zero after a sell operation, the `getPrice` function in BondingCurve.sol returns a sell price of zero. Consequently, the sellers will receive zero funds for their shares, suffering a complete loss of their investment. The impact of this issue is severe, as it can result in substantial financial losses for sellers and erode trust in the bonding curve mechanism.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# Incorrect Share Balance Update During Sell Operations

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-306
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/306
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-306.md

## Brief Summary

The bug occurs in the [sellShareCred](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L182-L184) function in Cred.sol: The `_handleTrade` function is called with `isBuy` set to `false` for a sell operation. However, upon further inspection of the `_handleTrade` function, there is no update to the `lastTradeTimestamp` for the trader when selling shares: The `lastTradeTimestamp` is only updated during a buy operation, but not during a sell operation. Impact Traders may experience an approximate loss of their share balance during sell operations because the protocol fails to accurately update the `lastTradeTimestamp` for traders when they se...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_81_group

# Curator could sell shares even when their balance becomes zero

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-309
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/309
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-309.md

## Brief Summary

The issue start with the following lines of code: The problem is that the `_removeCredIdPerAddress` function is called before updating the `shareBalance` mapping. If the sell operation reduces the curator's balance to zero, the `_removeCredIdPerAddress` function removes the curator's entry from the `_credIdsPerAddress` mapping. However, the `shareBalance` mapping is updated afterwards, which means that the curator's balance is not actually zero at the time of removal. This allows a curator to sell shares even when their balance becomes zero, violating the expected behavior. The `_removeCredIdPerAddress` function should be called after updating the `shareBalance` mapping to ensure that the c...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# The contract's view of user balances is inconsistent with the actual ETH balances.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-344
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/344
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-344.md

## Brief Summary

Sellers inconsistency between their actual ETH balance and the contract's view of their balance after a successful sell operation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Total fees (protocol fee + creator fee) can exceed the sent value (`msg.value`) in a buy operation.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-345
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/345
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-345.md

## Brief Summary

[getPriceDate function in the BondingCurve](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/curve/BondingCurve.sol#L51-L72) incorrectly calculates the creator fee when the supply is zero. This can lead to a scenario where the total fees (protocol fee + creator fee) exceed the value sent by the buyer during a buy operation. The bug occurs when `supply_ == 0`. In this case, the function sets `creatorFee = 0` and immediately returns without considering the `royaltyRate`. This leads to a scenario where the total fees (protocol fee + creator fee) can exceed the sent value (`msg.value`) in a buy operation. Impact In certain scenarios, the total fees cha...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_67_group

# No Token Recovery Mechanism

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-381
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/381
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-381.md

## Brief Summary

The contract does not include a mechanism to recover tokens that are accidentally sent to the contract address. Without such a mechanism, tokens that are mistakenly transferred to the contract could be lost permanently. Code

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Function to Withdraw Stuck ETH

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-382
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/382
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-382.md

## Brief Summary

The contract does not provide a function to withdraw ETH that may be unintentionally sent to the contract address. Without such a function, any ETH sent to the contract could be permanently stuck, leading to potential financial losses. Code

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Front-running Risk

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-384
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/384
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-384.md

## Brief Summary

The `priceLimit` checks (`priceLimit` in buy and sell scenarios) can be exploited by front-running attacks. An attacker could monitor pending transactions and front-run them by submitting a transaction with a slightly better price limit, causing the original transaction to fail.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_33_group

# Oracle Manipulation

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-386
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/386
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-386.md

## Brief Summary

The function relies on `IBondingCurve(cred.bondingCurve).getPriceData` to get price information. If the bonding curve contract or the data it relies on can be manipulated (e.g., through a compromised oracles), the price data could be skewed, leading to incorrect pricing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Centralization Risk in Signature Verification

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-584
- **Submitter:** enami_el
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/584
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-584.md

## Brief Summary

The contract relies heavily on a single `phiSignerAddress` for signature verification. If this address is compromised, it could lead to unauthorized art creation and claims. **Vulnerable Code:**

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `curatorData` indexing within `getCuratorAddresses` is flawed

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-668
- **Submitter:** escrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/668
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-668.md

## Brief Summary

When executing `getCuratorAddresses`, we retrieve `curatorData` by executing `_getCuratorData`. Within `_getCuratorData` we have a check which checks array[n] however if starting index == n, and stop index == n (meaning we want to check nth slot) then we won't be able to as `start_ >= stopindex` meaning we revert if `n >= n`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Anyone can set themselves as the 'royaltyRecipient' and allow them to manipulate and diverting funds away from the recipient

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-505
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/505
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-505.md

## Brief Summary

Anyone could call the 'initializeRoyalties' function and set themselves as the 'royaltyRecipient'. This would allow them to receive royalties from the contract, and diverting funds away from the rightful recipient.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# The '_initializePhiArt' function is not protected causing unauthorized access to the contract

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-509
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/509
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-509.md

## Brief Summary

The `_initializePhiArt` function is missing appropriate access control measures, allowing any user to execute it and modify credentials, the Merkle root, and other critical data. This vulnerability could lead to unauthorized or repeated invocations, potentially compromising the integrity of the `PhiArt` structure.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Immediate Credential Creation and Share Purchase Vulnerability Enables Flash Loan Exploits for Governance Manipulation

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-615
- **Submitter:** gregom
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/615
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-615.md

## Brief Summary

The identified vulnerability in the Phi Protocol smart contracts revolves around the handling of immediate state changes post credential creation. In the current setup, particularly within the `createCred()` function in the `Cred.sol` contract, there is no sufficient delay or cooldown period between creating a credential and purchasing shares of it. This lack of delay allows an attacker to manipulate their holdings rapidly, thereby influencing governance outcomes. By leveraging flash loans, an attacker could contribute minimal initial funds to create a credential and then, within the same block, temporarily acquire a majority of shares to execute governance actions or capture rewards immedi...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Incomplete Reward Claims Due to Insufficient Balance and Misleading Event Emissions

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-647
- **Submitter:** gregom
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/647
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-647.md

## Brief Summary

The identified vulnerability in the rewards management lies within the `PhiRewards` contract, particularly in its approach to handling reward claims. The contract applies a strategy where the rewards to be distributed are dependent on the contract’s current balance. If the claimant's rewards exceed the available balance, the excess claims are unfulfilled. Furthermore, the contract fails to properly track or handle these unclaimed rewards, which could ultimately mislead users through inaccurate event emissions, suggesting that full rewards processing took place irrespective of actual payments made. RELEVANT CODE VULNERABILITY DETAILS The vulnerability stems from the `handleRewardsAndGetValue...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_42_group

# Unsafe Token Transfer Due to Lack of Address Validation in Smart Contracts

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-666
- **Submitter:** gregom
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/666
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-666.md

## Brief Summary

The identified vulnerability within the smart contracts involves the unsafe use of the `SafeTransferLib` library for transferring tokens without validating the token address. Specifically, this vulnerability arises from the absence of checks that confirm whether a specified token address corresponds to a valid and deployed contract. Functions such as `buyShareCred`, `mint`, and `claimFromFactory` utilize this library to handle token transfers, but they lack the necessary validation step. This omission can be exploited by attackers to perform unauthorized operations using invalid or non-existent addresses, bypassing intended security measures. The exploitation allows potentially harmful acti...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Lack of domain separation for signatures used in PhiFactory

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-540
- **Submitter:** hgrano
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/540
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-540.md

## Brief Summary

Within the `PhiFactory` contract the data signed does not include fields to prevent replay attacks. This includes: - Lack of chain ID - so the same signature could be replayed on multiple chains. - No signing domain / application name / application version to separate the signatures used for Phi DApp from other DApps or other versions of Phi. - No separation from other verifying contracts. Note - the cred data does include the chain ID of the credential but this may not be the same as the chain on which the contract is deployed. Another chain ID should be included to specify which chain the signature is intended for. The impact of this is that users could create or claim art that they are n...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `Cred.sol::createCred` permits cross-chain signature replay

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-507
- **Submitter:** kuprum
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/507
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-507.md

## Brief Summary

Unauthorized creation of credentials on another chain, replaying the signature obtained for a specific chain. May lead to unauthorized minting of NFTs, or users buying shares/NFTs from wrong creators. Summary [Cred.sol::createCred](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L231-L280) ignores the chain id embedded into the supplied signature: While all other parameters of the signature are checked for validity, `chainId` is ignored; this allows to replay on any other chain the signature obtained from the Phi signing authority for a specific chain. Unauthorized creation of credentials may lead to other impacts: e.g. unauthorized minti...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# Exploitation of Bonding Curve - Underpricing Due to Low Initial Supply.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-622
- **Submitter:** la-arana-inteligente
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/622
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-622.md

## Brief Summary

The vulnerability arises from the bonding curve mechanism used to calculate the price of a credential in scenarios where the supply is low. When a new credential is issued with a low initial supply, the price determined by the bonding curve may be significantly undervalued. Malicious actors can exploit this by purchasing large quantities of the credential at these artificially low prices. As the supply increases and demand grows, the price naturally rises according to the bonding curve, allowing these actors to sell their holdings at a much higher price, leading to disproportionate profits. This can result in financial losses for other participants who buy at the inflated prices and destabi...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_95_group

# The adversary can steal the royalty fee by using the credId

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-166
- **Submitter:** lanyi2023
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/166
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-166.md

## Brief Summary

The adversary can steal the royalty fee by using the credId

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `_executeBatchTrade` function doesn't check if the batch operation would exceed the `MAX_SUPPLY`

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-480
- **Submitter:** minato7namikazi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/480
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-480.md

## Brief Summary

in the `_executeBatchTrade` function of the `Cred` contract The function doesn't check if the batch operation would exceed the `MAX_SUPPLY` for each credential. This could lead to a situation where the total supply of a credential exceeds the intended maximum, which goes against the design principles of the system. 1. In the documentation, it's mentioned that there's a `MAX_SUPPLY` constant set to 999: 2. For individual buy operations, there's a check to ensure the supply doesn't exceed this maximum: 3. but in the `_executeBatchTrade` function, this check is missing for buy operations. It only updates the supply without checking if it exceeds the maximum : This oversight could allow a malic...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# The system design could lead to incorrect supply calculations and pricing

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-484
- **Submitter:** minato7namikazi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/484
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-484.md

## Brief Summary

there is a potential issue in the overall system design that could lead to incorrect supply calculations and pricing. This issue stems from the interaction between the `Cred` and `PhiFactory` contracts. Let's break it down: 1. The `Cred` contract manages the buying and selling of shares for credentials. 2. The `PhiFactory` contract manages the creation and minting of NFTs associated with these credentials. The potential issue arises because these two processes are not directly linked, which could lead to a mismatch between the number of shares and the number of minted NFTs. Here's how this could manifest: Potential Impact: 1. If the current supply in the `Cred` contract doesn't accurately r...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# handleRewardsAndGetValueSent external allows other to set their own address and get rewards

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-334
- **Submitter:** mjcpwns
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/334
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-334.md

## Brief Summary

[H-3] Anyone can call depositRewards function depositRewards is now internal. However, handleRewardsAndGetValueSent is external and allows anyone to call it, which means the issue still remains. Thus users can freely call this function and update their balances with arbitrary values, which can then be withdrawn for ETH.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_40_group

# Wrongful calculation in _validateAndCalculateBatch due to using isBuy

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-400
- **Submitter:** mjcpwns
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/400
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-400.md

## Brief Summary

Leads to loss of profit for people who are selling shares in a batch due to wrongful calculation

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_112_group

# Malicious user can alter the reporting of getPositionsForCurator

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-355
- **Submitter:** nikolap
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/355
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-355.md

## Brief Summary

A malicious user can manipulate other curators' positions by calling `_addCredIdPerAddress`, which breaks the reporting logic.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_126_group

# use of msgSender instead of msgsender

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-154
- **Submitter:** nnamdi0482
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/154
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-154.md

## Brief Summary

Detailed description of the impact of this finding. If there’s a mistake in the logic (e.g., a comparison or validation error), a bad actor might exploit this by calling functions in a specific order or with specific parameters that bypass the intended checks. For instance, if the validation logic incorrectly handles contract-to-contract calls or fails to verify addresses properly, it could be exploited or If _msgSender() is not behaving as expected and allows unintended interactions, it could lead to security vulnerabilities or incorrect function execution

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# User can front-run and block someone from selling and dropping the price down of the cred.

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-323
- **Submitter:** onthehunt11
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/323
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-323.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Zero Amount Transfers Full Balance to Recipient In `RewardControl::_withdraw` Function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-331
- **Submitter:** pro_king
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/331
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-331.md

## Brief Summary

The `RewardControl::_withdraw` function contains a vulnerability where if the `amount` parameter is passed as zero, it triggers a withdrawal of the `sender's` entire balance. This happens because the condition `if (amount == FULL_BALANCE)` evaluates as `true` when `amount` is zero. The variable `FULL_BALANCE` is set to zero, so when `amount` is zero, it causes the entire balance of the from address to be transferred to the to address. This behavior is unintended and can lead to incorrect transfers, potentially draining all funds from the `from`(`sender`) address. Impact An attacker could exploit this vulnerability by passing a zero value for the `amount` parameter, resulting in the transfer...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# Conditions without Proper Validation

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-310
- **Submitter:** rare_one
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/310
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-310.md

## Brief Summary

Conditions such as curator_ == address(0) and credId_ != credIdToRemove are used without comprehensive validation. Risk Rating: Medium: Insufficient condition checks may lead to logic flaws or unintended behavior. Impacts: Logic Flaws: Conditions might not handle all edge cases, leading to faulty logic. Unintended Behavior: Incorrect conditions can cause the contract to behave unexpectedly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_88_group

# Unchecked Parameters in Batch Operations

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-311
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/311
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-311.md

## Brief Summary

Parameters like arrays of credIds_, amounts_, and maxPrices_ are used but not validated for consistency. Risk Rating: High: Batch operations can have significant impacts if parameters are incorrect or inconsistent. Impacts: Inconsistent State: Mismatched array lengths can lead to incorrect batch processing. Operational Failures: Inconsistent data may cause batch operations to fail or behave unpredictably.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Signature and Data Validation

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-312
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/312
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-312.md

## Brief Summary

Parameters related to signatures like signedData_ and signature_ are used without thorough validation. Risk Rating: High: Inadequate signature validation can lead to unauthorized actions and security breaches. Impacts: Unauthorized Actions: Invalid or forged signatures may allow unauthorized actions. Security Breaches: Weak signature validation can be exploited to bypass security measures.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent Token URI Handling in PhiFactory Smart Contract

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-379
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/379
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-379.md

## Brief Summary

The PhiFactory smart contract's "getTokenURI function" exhibits inconsistent behavior, particularly when retrieving URIs for valid token IDs. This issue was identified during testing, where the function failed to return the correct URIs for both standard and edge cases. These discrepancies pose a significant risk to the integrity of the contract, potentially leading to user dissatisfaction, loss of trust, and financial consequences. Affected Function: Function: getTokenURI(uint256 artId) public view returns (string memory) Impact: User Experience and Trust: Incorrect Token URIs: Users relying on the getTokenURI function to retrieve the metadata associated with their tokens will receive inco...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# High Gas Consumption Leading to Out-of-Gas Errors

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-393
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/393
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-393.md

## Brief Summary

During the execution of a DoS (Denial of Service) test on the PhiNFT1155 contract, it was discovered that increasing the batch size and number of iterations resulted in excessive gas consumption, leading to an out-of-gas error. This indicates that the contract might not handle large-scale operations efficiently, which could be exploited in a real-world scenario to disrupt the contract's functionality. Observed Gas Consumption: The test with the high batch size and iterations consumed significantly more gas than the typical Ethereum block gas limit (around 8 million gas). The excessive gas consumption in this test suggests that the contract is vulnerable to DoS attacks that exploit gas limit...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# unvalidated external call

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-394
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/394
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-394.md

## Brief Summary

The createArtFromFactory function transfers ETH to protocolFeeDestination without validating whether the destination address is a contract or if it can handle ETH.The line protocolFeeDestination.safeTransferETH(artFee) is an external call to the protocolFeeDestination address. While safeTransferLib is used to provide some level of security, it doesn't guarantee that the protocolFeeDestination address is valid or that the contract at that address is trustworthy. Invalid Address: If protocolFeeDestination is set to an invalid address, the transaction will fail, and the funds will be lost. Malicious Contract: If protocolFeeDestination points to a malicious contract, it could exploit the reentr...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Cred trades succeeds despite priceLimit being 0

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-224
- **Submitter:** roland
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/224
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-224.md

## Brief Summary

In in [Cred.sol](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L826), there is no check for whether the priceLimit, maxPrice_ for buys and minPrice_ for sells is equal 0. The function only checks whether the priceLimit is enough to cover price and fees while it is larger than 0. Because of this, if msg.value is enough to cover the cost, trades succeed even when their priceLimit is obviously insufficient, being 0. An analogous issue occurs for batch trades in [ ](https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/Cred.sol#L861).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_66_group

# BatchTrade processing can be blocked

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-517
- **Submitter:** saneryee
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/517
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-517.md

## Brief Summary

The function `_executeBatchTrade` makes external calls to `safeTransferETH` and `deposit` in a loop to handle funds. If `protocolFeeDestination` or `phiRewardsAddress` is a malicious contract, it may intentionally fail or consume a large amount of gas, resulting in the failure of the entire transaction and causing a denial of service (DOS). Gas optimization If `credIds` is very large, such calls will also lead to excessive gas costs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Performance Degradation and Inefficient Duplicate Check in Batch Validation Process

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-522
- **Submitter:** saneryee
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/522
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-522.md

## Brief Summary

1. **Performance Degradation Due to Nested Loops** The `_validateAndCalculateBatch` function contains nested loops, which increases the algorithm's time complexity to O(n²). This could lead to performance issues when processing large amounts of data. 2. **Inefficient Duplicate Check Logic** In the original code, when a duplicate `credId` is encountered, the function immediately triggers a `revert DuplicateCredId()` error. Current Behavior: - The code reverts as soon as it detects the first duplicate `credId`. - It does not continue to check for additional duplicates, nor does it report multiple occurrences. - The error message does not specify which `credId` is duplicated. Potential Issues:...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential DOS Vulnerability and Excessive Gas Consumption Due to Unrestricted Array Length in `_validateAndCalculateBatch` Function

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-524
- **Submitter:** saneryee
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/524
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-524.md

## Brief Summary

The `_validateAndCalculateBatch` function uses nested loops to validate `credIds` and calculate fees. The function does not impose a limit on the length of the input arrays `credIds`, `amounts`, and `priceLimits_`. If these arrays are excessively long, several issues may arise: 1. **Excessive Gas Consumption** If the array length is too long, processing this data might exceed the block's gas limit, leading to transaction failure. 2. **Potential DOS Attack** A malicious user could deliberately pass excessively long arrays, attempting to launch a Denial of Service (DOS) attack by consuming large amounts of gas or causing the function to run for an extended time, thereby disrupting the contrac...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# Falsification of Cred Data by Bypassing Verification Process while updating cred information

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-560
- **Submitter:** sangal0810
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/560
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-560.md

## Brief Summary

The identified vulnerability allows the curator to update the credURL associated with a credential (cred) without a subsequent verification process, which could lead to the potential falsification of the entire credential. 1. Lack of Verification: After the initial verification and creation of the NFT linked to a credential, the curator has the ability to update the credURL. However, this update process does not require any re-verification by the original artist or a third party. This means that after an NFT is created and users have claimed it, the underlying data associated with the NFT can be altered without their knowledge or consent. 2. Potential for Fraud: This loophole can be exploit...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Risk of Infinite Art Creation Leading to Spam and Harm to Genuine Artists

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-566
- **Submitter:** sangal0810
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/566
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-566.md

## Brief Summary

This vulnerability allows an artist to create an unlimited number of artworks on the platform, potentially leading to spam and overwhelming the ecosystem. The ability to generate infinite art without restrictions can harm genuine artists by flooding the platform with low-quality or repetitive content, reducing visibility for high-quality, original works, and devaluing the overall artistic contributions. Platform Spam: An artist could exploit the ability to create unlimited artworks by flooding the platform with a large volume of content. This can overwhelm the system and clutter the marketplace, making it difficult for users to find and appreciate high-quality, original art. Reduced Visibil...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# contractURI in PhiFactory.sol will revert when nftAddress is equal to zero

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-261
- **Submitter:** tmotfl
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/261
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-261.md

## Brief Summary

contract will revert when call `contractURI`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Malicious actor can frontrun call to distribute(credId) in CuratorRewardsDistributor.sol and claim other curators' rewards

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-426
- **Submitter:** udayjjw
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/426
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-426.md

## Brief Summary

The distribute(credId) function in CuratorRewardsDistributor.sol gets the list of curators for a Cred ref https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/reward/CuratorRewardsDistributor.sol#L84 and simply distributes the rewards to current share holders of a cred ref https://github.com/code-423n4/2024-08-phi/blob/8c0985f7a10b231f916a51af5d506dd6b0c54120/src/reward/CuratorRewardsDistributor.sol#L110 without taking into account who were the share holders (and thus legitimate claimers) at the time the rewards were deposited. The documentation clearly states that curators should earn a percentage of the minting fee when a user mints a CRED for which...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_192_group

# Potential for Reward Misattribution in `handleRewardsAndGetValueSent` due to lack of access control and artId and credId correlation check

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-269
- **Submitter:** waydou
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/269
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-269.md

## Brief Summary

The `handleRewardsAndGetValueSent` function in the `PhiRewards` contract lacks access control This oversight allows malicious actors to inflate their reward balance, while the malicious actor still needs to provide `msg.value`, this is still a vulnerability as rewards balance can be used for potential airdrops. I should also note that `handleRewardsAndGetValueSent` doesn't check the relation between `artId` and `credId` The platform's reward system might not accurately reflect the intended relationship between art pieces and their associated credentials.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# possible re-entrancy in the [PhiFactory:signatureClaim()] allowing a malicious user to steal funds from the protoco

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-474
- **Submitter:** willycode20
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/474
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-474.md

## Brief Summary

Even when following CEI pattern, if an external call(sending of ether) is made to a malicious user of the flow and execute a re-entrancy attack. See detailed explanation on how this can happen in your function If you send Ether to an address controlled by a malicious user (e.g during the refund process) before performing the action of sending `mintProtocolFee` to its destination. If the refund process is executed first the fallback funcion can immediately callback into the vulnerable function `signatureClaim` opening a window for re-entrancy. Because the state have not fully been updated or some critical operation have not been made (sending `mintProtocolFee`), they can trigger unintended b...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_61_group

# abi.encode should be used instead of abi.encodePacked

- **Contest:** Phi
- **Slug:** 2024-08-phi
- **Submission:** V-320
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-phi-validation/issues/320
- **Source snapshot:** competitions/2024-08-phi/submissions/raw/V-320.md

## Brief Summary

The encoding of string or bytes does not apply padding at the end, unless it is part of an array or struct (then it is padded to a multiple of 32 bytes). In general, the encoding is ambiguous as soon as there are two dynamically-sized elements, because of the missing length field.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_65_group
