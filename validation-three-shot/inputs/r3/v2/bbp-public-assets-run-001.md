# bbp-public-assets Round Input

Source report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/bbp-public-assets/report/audit-report.md`
Finding count: `3`

## Findings

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
