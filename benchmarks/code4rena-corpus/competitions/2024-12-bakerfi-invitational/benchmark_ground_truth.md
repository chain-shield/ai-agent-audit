# Benchmark Ground Truth: BakerFi Invitational

## Accepted H/M Findings

# Accepted H/M Findings: BakerFi Invitational

# [H-01] Users may encounter losses on assets deposited through StrategySupplyERC4626

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

StrategySupplyERC4626 Submitted by 0xpiken, also found by 0xlemon, klau5, klau5, MrPotatoMagic, and shaflow2

- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/strategies/StrategySupplyERC4626.sol#L44
- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/strategies/StrategySupplyERC4626.sol#L51
- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/strategies/StrategySupplyERC4626.sol#L58

## Finding description and impact

The _deploy(), _undeploy(), and _getBalance() functions of StrategySupplyERC4626 currently return the amount of shares instead of the amount of the underlying asset. This mistake leads to incorrect calculations of user assets within any BakerFi Vault that utilizes StrategySupplyERC4626.

## Recommended mitigation steps

Update StrategySupplyERC4626 to return correct value:

function _deploy(uint256 amount) internal override returns (uint256) { - return _vault.deposit(amount, address(this)); + _vault.deposit(amount, address(this)); + return amount; } /** * @inheritdoc StrategySupplyBase */ function _undeploy(uint256 amount) internal override returns (uint256) { - return _vault.withdraw(amount, address(this), address(this)); + _vault.withdraw(amount, address(this), address(this)); + return amount; } /** * @inheritdoc StrategySupplyBase */ function _getBalance() internal view override returns (uint256) { - return _vault.balanceOf(address(this)); + return _vault.convertToAssets(_vault.balanceOf(address(this))); } chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-17 Status:

Mitigation confirmed. Full details in reports from shaflow2 and 0xlemon.

# [H-02] Anyone can call StrategySupplyBase.harvest , allowing users to avoid paying performance fees on interest

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

StrategySupplyBase.harvest, allowing users to avoid paying performance fees on interest Submitted by klau5, also found by 0xlemon, 0xpiken, and shaflow2

- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/strategies/StrategySupplyBase.sol#L90

## Finding description and impact

Since StrategySupplyBase.harvest can be called by anyone, users can front-run the rebalance call or regularly call harvest to avoid paying protocol fees on interest. This allows users to receive more interest than they should.

## Recommended Mitigation Steps

Add the onlyOwner modifier to StrategySupplyBase.harvest to restrict access.

chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-15 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [H-03] _deployedAmount not updated on StrategySupplyBase.undeploy , preventing performance fees from being collected

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

_deployedAmount not updated on StrategySupplyBase.undeploy, preventing performance fees from being collected Submitted by klau5, also found by 0xlemon, 0xpiken, MrPotatoMagic, pfapostol, and shaflow2

- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/strategies/StrategySupplyBase.sol#L110

## Finding description and impact

StrategySupplyBase.undeploy does not update _deployedAmount. As a result, if a withdrawal occurs, even if interest is generated, the protocol cannot collect performance fees through rebalance.

## Recommended Mitigation Steps

Update _deployedAmount by the withdrawal amount in StrategySupplyBase.undeploy.

chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-12 Status:

Mitigation confirmed. Full details in reports from shaflow2 and 0xlemon.

# [H-04] There are multiple issues with the decimal conversions between the vault and the strategy

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

Submitted by shaflow2, also found by 0xlemon, 0xpiken, ABAIKUNANBAEV, klau5, and shaflow2

- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/strategies/StrategyLeverage.sol#L234
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/strategies/StrategyLeverage.sol#L347
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/strategies/StrategyLeverage.sol#L359
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/strategies/StrategyLeverage.sol#L673
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/strategies/StrategyLeverage.sol#L640
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/strategies/StrategySupplyBase.sol#L110
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/strategies/StrategySupplyBase.sol#L69

## Finding description and impact

The StrategyLeverage contract has multiple incorrect decimal handling issues, causing the system to not support tokens with decimals other than 18.

## Recommended mitigation steps

It is recommended to align the vault’s decimals with the underlying token’s decimals instead of using 18 decimals. This alignment can significantly reduce the complexity of decimal conversions throughout the system.

chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-24 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [H-05] The implementation of pullTokensWithPermit poses a risk, allowing malicious actors to steal tokens

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

pullTokensWithPermit poses a risk, allowing malicious actors to steal tokens Submitted by shaflow2, also found by 0xlemon and MrPotatoMagic

- https://github.com/code-423n4/2024-12-bakerfi/blob/3873b82ae8b321473f3afaf08727e97be0635be9/contracts/core/hooks/UsePermitTransfers.sol#L31

## Finding description and impact

In batch operations interacting with the router, users are allowed to input tokens into the router using the permit method. This approach may be vulnerable to frontrunning attacks, allowing malicious actors to steal the user’s tokens.

## Recommended mitigation

The current router is not suitable for integrating permit to handle token input.

chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-23 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [H-06] Malicious actors can exploit user-approved allowances on VaultRouter to drain their ERC20 tokens

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

VaultRouter to drain their ERC20 tokens Submitted by 0xpiken, also found by 0xlemon, MrPotatoMagic, and shaflow2

- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/VaultRouter.sol#L186-L202
- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/VaultRouter.sol#L234-L252

## Finding description and impact

Once a user approves VaultRouter to spend their ERC20 tokens, anyone could call VaultRouter#execute() to drain the user’s ERC20 assets.

## Recommended mitigation steps

To protect users from potential exploitation, the PULL_TOKEN_FROM and PUSH_TOKEN_FROM commands should be executed only when msg.sender is from.

chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-20 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [H-07] Malicious actors can exploit user-approved allowances on VaultRouter to drain their ERC4626 tokens

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

VaultRouter to drain their ERC4626 tokens Submitted by 0xpiken, also found by MrPotatoMagic

- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/VaultRouter.sol#L120
- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/VaultRouter.sol#L122

## Finding description and impact

Once a user approves VaultRouter to spend their ERC4626 shares, anyone could call VaultRouter#execute() to drain the user’s ERC4626 shares.

## Recommended mitigation steps

Both ERC4626_VAULT_REDEEM and ERC4626_VAULT_WITHDRAW commands should only handle the caller’s ERC4626 shares:

function _handleVaultRedeem( bytes calldata data, uint256[] memory callStack, uint32 inputMapping, uint32 outputMapping ) private returns (bytes memory) { IERC4626 vault; uint256 shares; address receiver; address owner; assembly { vault:= calldataload(data.offset) shares:= calldataload(add(data.offset, 0x20)) receiver:= calldataload(add(data.offset, 0x40)) - owner:= calldataload(add(data.offset, 0x60)) } + owner = msg.sender; shares = Commands.pullInputParam(callStack, shares, inputMapping, 1); uint256 assets = redeemVault(vault, shares, receiver, owner); Commands.pushOutputParam(callStack, assets, outputMapping, 1); return abi.encodePacked(assets); } function _handleVaultWithdraw( bytes calldata data,

uint256[] memory callStack, uint32 inputMapping, uint32 outputMapping ) private returns (bytes memory) { IERC4626 vault; uint256 assets; address receiver; address owner; assembly { vault:= calldataload(data.offset) assets:= calldataload(add(data.offset, 0x20)) receiver:= calldataload(add(data.offset, 0x40)) - owner:= calldataload(add(data.offset, 0x60)) } + owner = msg.sender; assets = Commands.pullInputParam(callStack, assets, inputMapping, 1); uint256 shares = withdrawVault(vault, assets, receiver, owner); Commands.pushOutputParam(callStack, shares, outputMapping, 1); return abi.encodePacked(shares); } chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-19 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

Medium Risk Findings (16)

# [M-01] VaultBase is not ERC4626 compliant

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

Submitted by shaflow2 Original issue M-01:

- https://code4rena.com/evaluate/2024-12-bakerfi-invitational/findings/F-3
Mitigation Issues Ineffective final statement in maxMint function:

The last statement in the maxMint function has no effect. When maxAssets equals type(uint256).max, maxShares might incorrectly return type(uint256).max, which can lead to unintended behavior.

function maxMint(address receiver) external view override returns (uint256 maxShares) { uint256 maxAssets = _maxDepositFor(receiver); maxShares = this.convertToShares(maxAssets); maxAssets == 0 || maxAssets == type(uint256).max ? maxAssets: _convertToShares(maxAssets, false); } Lack of special case handling in maxMint and maxDeposit:

The maxMint and maxDeposit functions do not account for special conditions within the system. For example, if there is an Aave strategy involved, the functions should consider Aave’s maximum supply cap limits for assets to prevent exceeding protocol constraints.

require( supplyCap == 0 || ((IAToken(reserveCache.aTokenAddress).scaledTotalSupply() + uint256(reserve.accruedToTreasury)).rayMul(reserveCache.nextLiquidityIndex) + amount) <= supplyCap * (10 ** reserveCache.reserveConfiguration.getDecimals()), Errors.SUPPLY_CAP_EXCEEDED ); Does not account for third-party strategy pauses or asset deposit rejections:

The system does not consider situations where third-party strategies are paused or reject asset deposits.

## Links to affected code

VaultBase.sol#L186

# [M-02] New strategy can not work due to insufficient allowance

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

Submitted by 0xpiken, also found by 0xlemon, klau5, and shaflow2 When a new strategy is added through MultiStrategy#addStrategy(), it was not approved to spend the asset in MultiStrategyVault. Any functions that call newStrategy#deploy() may revert and result in MultiStrategyVault being DoS’ed.

## Recommended mitigation steps

The new strategy should be approved with max allowance when added:

function addStrategy(IStrategy strategy) external onlyRole(VAULT_MANAGER_ROLE) { if (address(strategy) == address(0)) revert InvalidStrategy(); _strategies.push(strategy); _weights.push(0); + IERC20(strategy.asset()).approve(address(strategy), type(uint256).max); emit AddStrategy(address(strategy)); } chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-13 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [M-03] MultiStrategy#removeStrategy() cannot remove leverage strategies that still have deployed assets

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

MultiStrategy#removeStrategy() cannot remove leverage strategies that still have deployed assets Submitted by 0xpiken, also found by 0xlemon, pfapostol, and shaflow2

- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/MultiStrategy.sol#L265

## Finding description and impact

A leverage strategy with deployed assets can not be removed from MultiStrategyVault due to insufficient assets

## Recommended mitigation steps

When removing a strategy from MultiStrategyVault, ensure the amount of assets to be re-allocated is same as the received amount:

function removeStrategy(uint256 index) external onlyRole(VAULT_MANAGER_ROLE) { // Validate the index to ensure it is within bounds if (index >= _strategies.length) revert InvalidStrategyIndex(index); // Retrieve the total assets managed by the strategy to be removed uint256 strategyAssets = _strategies[index].totalAssets(); // Update the total weight and mark the weight of the removed strategy as zero _totalWeight -= _weights[index]; _weights[index] = 0; // If the strategy has assets, undeploy them and allocate accordingly if (strategyAssets > 0) { - IStrategy(_strategies[index]).undeploy(strategyAssets); - _allocateAssets(strategyAssets); + _allocateAssets(IStrategy(_strategies[index]).undeploy(strategyAssets));

} // Move the last strategy to the index of the removed strategy to maintain array integrity uint256 lastIndex = _strategies.length - 1; if (index < lastIndex) { _strategies[index] = _strategies[lastIndex]; _weights[index] = _weights[lastIndex]; } emit RemoveStrategy(address(_strategies[lastIndex])); // Remove the last strategy and weight from the arrays _strategies.pop(); _weights.pop(); } chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-16 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [M-04] Sending tokens to a Strategy when totalSupply is 0 can permanently make the Vault unavailable

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

totalSupply is 0 can permanently make the Vault unavailable Submitted by klau5, also found by 0xlemon, MrPotatoMagic, and shaflow2

- https://github.com/code-423n4/2024-12-bakerfi/blob/3873b82ae8b321473f3afaf08727e97be0635be9/contracts/core/VaultBase.sol#L244

## Finding description and impact

Before the first deposit or when all shares have been withdrawn making totalSupply zero, an attacker can manipulate totalAssets by directly sending tokens to the Strategy, making the Vault permanently unusable. Additionally, in normal usage scenarios, small amounts of assets remaining in the Strategy can cause the same issue.

## Recommended Mitigation Steps

Create a function that can withdraw assets when totalSupply is zero but totalAssets is non-zero. Call this function in _depositInternal to clean up the Strategy.

chefkenji (BakerFi) acknowledged and commented:

This issue was already reported in previous audits. We have decided to seed the vaults to now allow the minimum number of shares to be achieved and prevent first depositor attacks,

- https://github.com/code-423n4/2024-05-bakerfi-findings/issues/39

# [M-05] Permit doesn’t work with DAI

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

Permit doesn’t work with DAI Submitted by 0xlemon, also found by MrPotatoMagic

- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/VaultRouter.sol#L114

## Vulnerability Details

VaultRouter allows users to use permit transactions for convenience. This router is supposed to work with any ERC20 tokens. We can see in pullTokensWithPermit how the permit is utilized:

function pullTokensWithPermit ( IERC20Permit token, uint256 amount, address owner, uint256 deadline, uint8 v, bytes32 r, bytes32 s ) internal virtual { // Permit the VaultRouter to spend tokens on behalf of the owner @-> IERC20Permit ( token ).

permit ( owner, address ( this ), amount, deadline, v, r, s ); // Transfer the tokens from the owner to this contract IERC20 ( address ( token )).

safeTransferFrom ( owner, address ( this ), amount ); } However this.permit doesn’t work with DAI tokens because DAI token’s permit signature is different. From the contract at address 0x6B175474E89094C44Da98b954EedeAC495271d0F, we see the permit function:

function permit ( address holder, address spender, uint256 nonce, uint256 expiry, bool allowed, uint8 v, bytes32 r, bytes32 s ) external The nonce and allowed arguments are added to DAI’s permit that means calling pullTokensWithPermit where DAI is the token will revert.

## Impact

Permit cannot be used with DAI tokens

## Recommended mitigation steps

For the special case of DAI token, allow a different implementation of the permit function which allows nonce and allowed variables.

chefkenji (BakerFi) acknowledged

# [M-06] Even when the Vault contract is paused, the rebalance function is not paused

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

rebalance function is not paused Submitted by klau5

- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/Vault.sol#L177
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/MultiStrategyVault.sol#L174

## Finding description and impact

When the contract is paused, rebalance is not paused. While users cannot withdraw, performance fees can still be collected from interest.

## Recommended Mitigation Steps

Add the whenNotPaused modifier to the rebalance function.

chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-3 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [M-07] Depositor can bypass the max deposit limit

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

Submitted by shaflow2 Original issue M-07:

- https://code4rena.com/evaluate/2024-12-bakerfi-invitational/findings/F-16
Mitigation issue The mitigation measures described in the report were not successfully implemented. The receiver only needs to transfer out the shares, and they can continue minting.

function _maxDepositFor(address receiver) internal view returns (uint256) { uint256 maxDepositLocal = getMaxDeposit(); uint256 depositInAssets = _convertToAssets(balanceOf(receiver), false); if (paused()) return 0; if (maxDepositLocal > 0) { return depositInAssets > maxDepositLocal ? 0: maxDepositLocal - depositInAssets; } return type(uint256).max; } The report suggests creating a mapping for each whitelisted receiver to store the deposit amount. This approach should be implemented accordingly.

## Links to affected code

VaultBase.sol#L312

# [M-08] The dispatch function of the VaultRouter , does not work as intended, with PULL_TOKEN action

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

dispatch function of the VaultRouter, does not work as intended, with PULL_TOKEN action Submitted by pfapostol, also found by MrPotatoMagic

- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/VaultRouter.sol#L99

## Finding description and impact

The dispatch function of the VaultRouter contract handles the execution of actions from the MultiCommand.execute call.

Actions are encoded as follows (Right-to-Left):

Bits (0-32): The action.

Bits (32-63): The input mapping.

Bits (64-95): The output mapping.

While most commands are handled using the corresponding action stored in the actionToExecute variable:

Normal logic example uint32 actionToExecute = uint32 ( action & Commands.

THIRTY_TWO_BITS_MASK ); // Extract input mapping from bits 32-63 by right shifting 32 bits and masking uint32 inputMapping = uint16 (( action >> 32 ) & Commands.

THIRTY_TWO_BITS_MASK ); // Extract output mapping from bits 64-95 by right shifting 64 bits and masking uint32 outputMapping = uint16 ((( action >> 64 ) & Commands.

THIRTY_TWO_BITS_MASK ));...

} else if ( actionToExecute == Commands.

PULL_TOKEN_FROM ) { output = _handlePullTokenFrom ( data, callStack, inputMapping ); } else if ( actionToExecute == Commands.

PUSH_TOKEN ) { output = _handlePushToken ( data, callStack, inputMapping ); } else if ( actionToExecute == Commands.

PUSH_TOKEN_FROM ) { output = _handlePushTokenFrom ( data, callStack, inputMapping ); } else if ( actionToExecute == Commands.

SWEEP_TOKENS ) {...

There is one exception. Likely due to a typo, the action “tuple” is used instead of actionToExecute:

Vulnerable logic:

} else if (action == Commands.PULL_TOKEN) {

## Impact

If an inputMapping is supplied with actionToExecute equal PULL_TOKEN, the execution will revert with InvalidCommand(uint256 action).

## Recommended mitigation steps

Use correct variable:

} else if ( actionToExecute == Commands.

PULL_TOKEN ) { chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-11 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [M-09] Non-whitelisted recipient can receive shares

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

Submitted by 0xlemon, also found by 0xlemon and MrPotatoMagic

- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/VaultBase.sol#L237-L271
Summary The recipient of the vault shares isn’t checked to be in the whitelist. This means that a non-whitelisted user can receive shares and then withdraw/redeem them throught the VaultRouter.

## Vulnerability Details

If we look at VaultBase deposit/mint/withdraw/redeem functions have a onlyWhiteListed modifier that means they can only be called by someone who is within the _enabledAccounts. However the protocol doesn’t check if the receiver is included in that whitelist. This allows non-whitelisted people to receive shares and they can later easily withdraw them through the VaultRouter.

## Impact

Bypass of the whitelist

## Recommended mitigation steps

Check if the receiver of the vault shares is whitelisted chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-26 Status:

Mitigation confirmed. Full details in reports from shaflow2 and 0xlemon.

# [M-10] The withdrawal of Multi strategies vault could be DoSed while asset deposits remain unaffected

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

Submitted by 0xpiken, also found by klau5

- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/MultiStrategy.sol#L173

## Finding description and impact

The MultiStrategy#_deallocateAssets() function will be DoSed if IStrategy#undeploy(0) is called.

## Recommended mitigation steps

Check if the amount is 0 before undeploying it:

function _deallocateAssets(uint256 amount) internal returns (uint256 totalUndeployed) { uint256[] memory currentAssets = new uint256[](_strategies.length); uint256 totalAssets = 0; uint256 strategiesLength = _strategies.length; for (uint256 i = 0; i < strategiesLength; i++) { currentAssets[i] = IStrategy(_strategies[i]).totalAssets(); totalAssets += currentAssets[i]; } totalUndeployed = 0; for (uint256 i = 0; i < strategiesLength; i++) { uint256 fractAmount = (amount * currentAssets[i]) / totalAssets; + if (fractAmount == 0) continue; totalUndeployed += IStrategy(_strategies[i]).undeploy(fractAmount); } chefkenji (BakerFi) confirmed MrPotatoMagic (warden) commented:

There is no DOS issue here. The VAULT_MANAGER can simply remove the strategy in that case. Having a strategy without any assets deposited means the strategy is unused and should be removed.

0xpiken (warden) commented:

VAULT_MANAGER may add a new strategy with no assets allocated yet. All withdrawals since then will be DoS’ed.

MrPotatoMagic (warden) commented:

add a new strategy with no assets allocated yet.

- You’ve added it to allow users to deposit. I do not see this being any more than Low/Info finding. The availability of the protocol is only impacted till the time you deposit assets into the new strategy.

Dravee (judge) commented:

First and foremost: this was confirmed by the sponsor. Let’s now discuss about the severity.

The availability of the protocol is only impacted till the time you deposit assets into the new strategy.

The protocol’s availability and functionality is indeed impacted unexpectedly. But there exist a workaround for this not to be permanent. Still, users’ assets can be affected quite badly (all withdrawals). This is an edge case, but it indeed qualifies as a Medium.

BakerFi mitigated:

PR-5 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [M-11] The calculation of assetsMax is incorrect.

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

assetsMax is incorrect.

Submitted by shaflow2

- https://github.com/code-423n4/2024-12-bakerfi/blob/3873b82ae8b321473f3afaf08727e97be0635be9/contracts/core/strategies/StrategySupplyMorpho.sol#L78

## Finding description and impact

In the _undeploy function, assetsMax is incorrectly calculated because the contract directly retrieves totalSupplyAssets and totalSupplyShares from _morpho storage without accounting for the accrued interest over time. This leads to an underestimation of assetsMax, which may allow users to withdraw more assets than they should, causing losses to other users.

## Recommended mitigation steps

When calculating assetsMax, consider the accrued interest and fees that have not been updated.

function _undeploy(uint256 amount) internal override returns (uint256) { Id id = _marketParams.id(); uint256 assetsWithdrawn = 0; - uint256 totalSupplyAssets = _morpho.totalSupplyAssets(id); - uint256 totalSupplyShares = _morpho.totalSupplyShares(id); - uint256 shares = _morpho.supplyShares(id, address(this)); - uint256 assetsMax = shares.toAssetsDown(totalSupplyAssets, totalSupplyShares); + uint256 assetsMax = _morpho.expectedSupplyAssets(_marketParams, address(this)); if (amount >= assetsMax) { (assetsWithdrawn, ) = _morpho.withdraw(_marketParams, 0, shares, address(this), address(this)); } else { (assetsWithdrawn, ) = _morpho.withdraw(_marketParams, amount, 0, address(this), address(this));

} return assetsWithdrawn; } chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-22 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [M-12] Cannot withdraw tokens from all strategies in MultiStrategyVault when one third party is paused

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

Submitted by klau5

- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/MultiStrategy.sol#L148
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/MultiStrategy.sol#L173
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/MultiStrategy.sol#L226
- https://github.com/code-423n4/2024-12-bakerfi/blob/0daf8a0547b6245faed5b6cd3f5daf44d2ea7c9a/contracts/core/MultiStrategy.sol#L264

## Finding description and impact

When even one third party integrated with MultiStrategyVault is paused, withdrawals become impossible from all Strategies. There is no way to remove the paused third party (Strategy).

## Recommended Mitigation Steps

We need a way to exclude third parties (Strategies) from withdrawals if they are unavailable. We need to be able to exclude a strategy without making a withdrawal request.

chefkenji (BakerFi) acknowledged

# [M-13] The Vault Manager is unable to delete the last strategy from MultiStrategyVault

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

MultiStrategyVault Submitted by pfapostol The removeStrategy function in the MultiStrategy contract allows the removal of a strategy and redistributes the withdrawn funds among the remaining strategies.

Refered code:

if ( strategyAssets > 0 ) { IStrategy ( _strategies [ index ]).

undeploy ( strategyAssets ); _allocateAssets ( strategyAssets ); } The issue arises when the last strategy is removed. The weight ( _weights[index] ) of the last strategy is first subtracted from _totalWeight, which results in _totalWeight being zero, and it is then set to zero.

Vulnerable logic:

_totalWeight -= _weights [ index ]; _weights [ index ] = 0; Later, when _allocateAssets is called: for each of the active _strategies (the last strategy has not yet been removed), it attempts to calculate the fraction of the input amount. However, since _totalWeight is zero, the execution is reverted with a “panic: division or modulo by zero” error.

Vulnerable logic:

function _allocateAssets ( uint256 amount ) internal returns ( uint256 totalDeployed ) { totalDeployed = 0; for ( uint256 i = 0; i < _strategies.

length; ) { uint256 fractAmount = ( amount * _weights [ i ]) / _totalWeight; if ( fractAmount > 0 ) { totalDeployed += IStrategy ( _strategies [ i ]).

deploy ( fractAmount ); } unchecked { i ++; }

## Impact

The VAULT_MANAGER_ROLE would be unable to delete the last strategy.

## Recommended Mitigation Steps

There are several ways to improve the code to fix the issue (such as adding a check for zero, etc.). However, the most straightforward and direct approach is to remove the strategy from _strategies before calling _allocateAssets:

function removeStrategy ( uint256 index ) external onlyRole ( VAULT_MANAGER_ROLE ) { // Validate the index to ensure it is within bounds if ( index >= _strategies.

length ) revert InvalidStrategyIndex ( index ); // Retrieve the total assets managed by the strategy to be removed uint256 strategyAssets = _strategies [ index ].

totalAssets (); // Update the total weight and mark the weight of the removed strategy as zero _totalWeight -= _weights [ index ]; _weights [ index ] = 0; IStrategy cache_strategy = _strategies [ index ]; // Move the last strategy to the index of the removed strategy to maintain array integrity uint256 lastIndex = _strategies.

length - 1; if ( index < lastIndex ) { _strategies [ index ] = _strategies [ lastIndex ]; _weights [ index ] = _weights [ lastIndex ]; } emit RemoveStrategy ( address ( _strategies [ lastIndex ])); // Remove the last strategy and weight from the arrays _strategies.

pop (); _weights.

pop (); // If the strategy has assets, undeploy them and allocate accordingly if ( strategyAssets > 0 ) { IStrategy ( cache_strategy ).

undeploy ( strategyAssets ); _allocateAssets ( strategyAssets ); } 3 chefkenji (BakerFi) confirmed 0xpiken (warden) commented:

The last strategy should not be allowed to be removed since MultiStrategyVault allocates assets accordingly to its strategies. Removing the last strategy will leads DoS on MultiStrategyVault. The worse is that no user can withdraw their assets since _deallocateAssets() will return totalUndeployed as 0.

pfapostol (warden) commented:

Yes, but that sounds more like a second problem, unrelated to this one. The order of operations in the function is clearly incorrect.

Dravee (judge) commented:

Per the sponsor:

I understand both points but for me is an issue because it leaves the vault on a weird state (funds are waiting for a rebalance) that could only be unlocked by the vault manager with a rebalance This finding is still valid.

BakerFi mitigated:

PR-18 Status:

Mitigation confirmed. Full details in reports from shaflow2 and 0xlemon.

# [M-14] The StrategySupplyMorpho allow to use wrong token in _asset

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

StrategySupplyMorpho allow to use wrong token in _asset Submitted by pfapostol The StrategySupplyMorpho is designed to supply tokens to a specific market in MorphoBlue. To achieve this, it defines a _marketParams structure that stores all the necessary information for the target market, including loanToken, which is transferred from the strategy during the IMorpho.supply call.

However, the strategy also allows the deployer to specify a different token via the _asset variable. This token is used to pull tokens from users during deposits and transfer tokens to users during withdrawals.

It is unclear whether this behavior is intended by design (e.g., treating _asset as a “collateral” token) or if _asset is always meant to be the same as loanToken in _marketParams. Regardless, the protocol will not function if _asset is different from loanToken.

The main issue is that the _asset variable is used to approve MorphoBlue:

Vulnerable Logic ( StrategySupplyMorpho ) constructor ( address initialOwner, address asset_, address morphoBlue, Id morphoMarketId ) StrategySupplyBase ( initialOwner, asset_ ) {...

if (!

ERC20 ( asset_ ).

approve ( morphoBlue, type ( uint256 ).

max )) { Vulnerable Logic ( StrategySupplyBase ) constructor ( address initialOwner, address asset_ ) ReentrancyGuard () Ownable () {...

_asset = asset_; But later, in the _deploy call, the loanToken is used to supply to a Morpho position:

Vulnerable Logic function _deploy ( uint256 amount ) internal override returns ( uint256 ) { ( uint256 deployedAmount, ) = _morpho.

supply ( _marketParams, amount, 0, address ( this ), hex "" ); Morpho Blue Supply function supply ( MarketParams memory marketParams, uint256 assets, uint256 shares, address onBehalf, bytes calldata data ) external returns ( uint256, uint256 ) {...

IERC20 ( marketParams.

loanToken ).

safeTransferFrom ( msg.

sender, address ( this ), assets );

## Impact

If these two tokens are different, the strategy will be unusable. Even if the design intends for _asset to act as collateral for loanToken s supplied externally (e.g., an initial supply), the protocol will still fail due to the absence of allowance for loanToken in the Morpho market.

## Recommended mitigation steps

In the constructor, either:

Set the _asset token to match the loanToken from MarketParams:

_marketParams = _morpho.

idToMarketParams ( morphoMarketId ); _asset = _marketParams.

loanToken; // Allowance approval if (!

ERC20 ( _marketParams.

loanToken ).

approve ( morphoBlue, type ( uint256 ).

max )) { revert FailedToApproveAllowanceForMorpho (); } Validate that _asset matches the loanToken:

if ( asset_ != _marketParams.

loanToken ) revert (); chefkenji (BakerFi) confirmed BakerFi mitigated:

PR-6 Status:

Mitigation confirmed. Full details in reports from 0xlemon and shaflow2.

# [M-15] VaultRouter cannot be used for deposits when it reaches the maximum deposit limit

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

VaultRouter cannot be used for deposits when it reaches the maximum deposit limit Submitted by 0xlemon

- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/VaultBase.sol#L251
- https://github.com/code-423n4/2024-12-bakerfi/blob/main/contracts/core/VaultRouter.sol#L116
Summary VaultRouter cannot be used for deposits when it reaches the maximum deposit limit because this contract is the msg.sender to the vault and it is treated as a depositor who has a limit.

## Vulnerability Details

When doing deposits to a vault from the VaultRouter the router does an external call to the vault meaning that in Vault’s case msg.sender will be the router itself. The protocol, however, enforces a max deposit limit for depositors. This means that after the VaultRouter reaches the vault’s getMaxDeposit() no one will be able to deposit to the vault using the router.

Since the vault looks at balanceOf(msg.sender) for the deposit limit, an attacker can use the router to deposit to the vault specifying the recipient to be the router itself and then immediately withdrawing in the same transaction so that his tokens won’t be stolen. He can do that to reach VaultRouter deposit limit and now no one will be able to deposit through the router.

function _depositInternal ( uint256 assets, address receiver ) private returns ( uint256 shares ) { //...

// Check if deposit exceeds the maximum allowed per wallet uint256 maxDepositLocal = getMaxDeposit (); if ( maxDepositLocal > 0 ) { @-> uint256 depositInAssets = ( balanceOf ( msg.

sender ) * _ONE ) / tokenPerAsset (); uint256 newBalance = assets + depositInAssets; if ( newBalance > maxDepositLocal ) revert MaxDepositReached (); } //...

}

## Impact

DoS of the router’s deposit functionality

## Recommended mitigation steps

You can try to enforce the same deposit limit on the router level and give the router unlimited deposit limit klau5 (warden) commented:

Same root cause different impact.

S-2 mitigation would work.

Dravee (judge) commented:

While the finding here is interesting, according to the Supreme Court decisions, this should be a duplicate.

0xlemon (warden) commented:

@Dravee - the recommended mitigation in S-2 is absolutely wrong. As I said previously it is a deposit limit so taking the balanceOf (receiver) does absolutely nothing but limit the amount a single user can “hold”.

The correct way to fix S-2 would be to implement some kind of a mapping to account for any deposit and limit the amount msg.sender can deposit. This, however, doesn’t solve this finding. The root cause is not the same because here it is shown the interactions between VaultRouter and the Vault and it blocks the VaultRouter from depositing.

Dravee (judge) commented:

0xlemon, I agree. Additionally, making S-15 as primary instead of S-2 due to the wrong mitigation.

shaflow2 (warden) commented:

The current implementation of the deposit limit in the system is incorrect. This report is based on the issue that the sponsor implemented an erroneous mitigation measure. Therefore, this issue should be categorized under implementation problems related to the deposit limit. Relevant historical judgments:

- https://github.com/code-423n4/2024-08-chakra-findings/issues/33
In report s-2, it was mentioned:

“Additionally, considering the whitelist mechanism, if you want to limit the deposit amount for whitelisted addresses, it is recommended to create a mapping to store the deposit amount for each address, rather than checking the balanceOf.” This point was not elaborated further because managing the deposit limit mapping is challenging. For example, if an address has a deposit limit of 1000, and it deposits 1000, increasing depositLimit[addr1] by 1000. If this address then transfers shares to another address addr2, and addr2 withdraws, with depositLimit[addr2] = 0, how should the deposit limit be deducted? If the deposit limit is not reduced, under the condition that the whitelist is not increased, the total assets in the system will only decrease over time.

0xlemon (warden) commented:

shaflow2 - the problem in this issue is that the VaultRouter is the msg.sender and since we are limiting msg.sender it will get blocked after it reaches the limit. The problem in S-15 is that balanceOf was used to account for the deposit limit. Do you see the difference? In my problem it doesn’t matter how this deposit limit is implemented when it limits msg.sender as it can be seen.

For the mapping part first of all it would be highly unlikely for a user to just give (transfer) his tokens away to someone but even if they do why do we need to deduct the limit when transfering? As I said it doesn’t matter how much a single user holds, we only need to make sure he doesn’t deposit more than maxDepositLocal MrPotatoMagic (warden) commented:

@Dravee, I agree with klau5 and shaflow2. This issue should not be considered as separate.

According to the SC ruling here, if fixing the root cause (in a reasonable manner) resolves the issue, they’re dups. It is important to focus on the point of fixing the issue in reasonable manner here.

How can the issue actually be mitigated?

If msg.sender is the router, take in another parameter routerMsgSender to the deposit() / _depositInternal() function that stores the msg.sender of the router contract.

Implement an if check in deposit() / _depositInternal() that when msg.sender == router is true, we utilize the parameter routerMsgSender to check the limit as per the method suggested by S-2. I.e., “it is recommended to create a mapping to store the deposit amount for each address”.

Both the issues are two sides of the same coin. A universal mitigation as mentioned above resolves both of them.

Dravee (judge) commented:

I’m agreeing with 0xlemon on this one, as reasonably fixing the other issue (by strictly fixing the other issue) would leave this current issue unfixed (unless already aware of this finding, which would be unreasonable).

chefkenji (BakerFi) disputed and commented:

Duplicate issue BakerFi mitigated:

PR-4 Status:

Mitigation confirmed. Full details in the report from 0xlemon.

# [M-16] Unmitigated Setting _performanceFee will result in inaccurate fees calculation In StrategySupplyBase::undeploy() subtracting the withdrawalValue from _deployedAmount

- **Contest:** BakerFi Invitational
- **Slug:** 2024-12-bakerfi-invitational
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-bakerfi-invitational
- **Source snapshot:** competitions/2024-12-bakerfi-invitational/final_report.html

Submitted by shaflow2 Original issue M-16:

- https://code4rena.com/evaluate/2024-12-bakerfi-invitational/findings/F-43
Mitigation issue The function will return the actual amount of tokens withdrawn, which has been fixed. However, the balance value calculation is incorrect, causing the contract to emit events with incorrect parameters.

- https://github.com/baker-fi/bakerfi-contracts/blob/42eb8e7a09022e0ab4007d768f068874b02c8a50/contracts/core/strategies/StrategySupplyBase.sol#L129
// Check withdrawal value matches the initial amount // Transfer assets to user ERC20(_asset).safeTransfer(msg.sender, withdrawalValue); balance -= amount; emit StrategyUndeploy(msg.sender, withdrawalValue); emit StrategyAmountUpdate(balance); The actual remaining balance should be balance - withdrawalValue, where withdrawalValue is the actual amount of tokens withdrawn.

Dravee (judge) commented:

Warden noted that balance is still substracting amount instead of withdrawalValue, which means the mitigation is incomplete. The original issue is indeed fixed though.

Setting _performanceFee will result in inaccurate fees calculation Submitted by 0xlemon Severity: Medium

## Vulnerability details

The ADMIN_ROLE can set a new performance fee or enable/disable it; which will be used when calculating the fees for the protocol in _harvestAndMintFees. The VaultSettings::setPerformanceFee() function doesn’t call the harvest function, which would lead to the newly set performance fee to be used for calculation of previous rewards.

We can see the function for setting a fee:

In VaultSettings.sol:

function setPerformanceFee(uint256 fee) external onlyRole(ADMIN_ROLE) { if (fee >= PERCENTAGE_PRECISION) revert InvalidPercentage(); _performanceFee = fee; emit PerformanceFeeChanged(_performanceFee); } This performance fee is later used in VaultBase::_harvestAndMintFees:

function _harvestAndMintFees() internal { uint256 currentPosition = _totalAssets(); if (currentPosition == 0) { return; } int256 balanceChange = _harvest(); if (balanceChange > 0) { address feeReceiver = getFeeReceiver(); @-> uint256 performanceFee = getPerformanceFee(); if (feeReceiver != address(this) && feeReceiver != address(0) && performanceFee > 0) { uint256 feeInEth = uint256(balanceChange) * performanceFee; uint256 sharesToMint = feeInEth.mulDivUp( totalSupply(), currentPosition * PERCENTAGE_PRECISION ); _mint(feeReceiver, sharesToMint); } Consider the following scenario:

The default performance fee is 1% and the vault currently has 1000 tokens and 100 tokens accrued interest that hasn’t been applied yet because the _harvestAndMintFees hasn’t been called yet.

The admin sets this performance fee to 10%.

Now the VAULT_MANAGER_ROLE calls Vault::rebalance that calls the harvest of the strategy and when calculating the performance fee it will be performanceFee = 10% * 100 tokens = 10 tokens.

In this case users lose funds and didn’t agree to stake when the performance fee is 10%.

The opposite scenario can happen as well. For example, considering the above scenario, if the admin disables the performance fee, no fees will be minted for the protocol causing a loss for the protocol.

## Impact

Accrued fees will be incorrectly calculated.

## Recommended mitigation steps

Override the setPerformanceFee function in VaultBase and call _harvestAndMintFees() before setting the new performance fee.

## Links to affected code

VaultSettings.sol#L173-L176 In StrategySupplyBase::undeploy() subtracting the withdrawalValue from _deployedAmount might lead to an underflow Submitted by 0xlemon Severity: Medium

## Vulnerability Details

In StrategySupplyBase::undeploy() subtracting the withdrawalValue from _deployedAmount might lead to an underflow because the _deployedAmount variable doesn’t account for the latest accrued interest and, therefore, can be lower than the amount a user is trying to withdraw.

By introducing the following line the protocol correctly mitigated the original issue; however, a new problem has appeared:

function undeploy( uint256 amount ) external nonReentrant onlyOwner returns (uint256 undeployedAmount) { if (amount == 0) revert ZeroAmount(); // Get Balance uint256 balance = getBalance(); if (amount > balance) revert InsufficientBalance(); // Transfer assets back to caller uint256 withdrawalValue = _undeploy(amount); // Update the deployed amount @-> _deployedAmount -= withdrawalValue; //...

} This _deployedAmount gets updated when a harvest is called so that it accounts for the latest accrued interest. However, since we do not call the harvest function before withdrawing/redeeming, the withdrawalValue can be higher than the _deployedAmount, which would result in reverting the withdrawal transaction.

Consider the following case:

Bob deposits 100 tokens to the vault that immediately get deployed to the strategy of the vault and now _deployedAmount = 100 tokens.

After some time these deposited tokens accrue interest and now let’s say the interest is 2 tokens so in the strategy we have 102 tokens in total.

The protocol calls _harvestAndMintFees which updates the _deployedAmount = 102 tokens and mints performance fees.

After some more time these tokens accrue 3 more tokens so now in total we have 105 tokens.

Bob calls Vault::withdraw (105 tokens) which would try to undeploy 105 tokens from the strategy. Since getBalance() of the strategy is 105 tokens it should be possible for Bob to withdraw that amount. However, since the _deployedAmount isn’t updated we get 102 tokens - 105 tokens which would underflow and revert the transaction.

We can see how a case can occur where a user cannot withdraw his tokens.

## Impact

Temporary DoS and stuck funds until _harvestAndMintFees is called.

## Recommended mitigation steps

_harvestAndMintFees should be called before withdrawing/redeeming from a vault.

## Links to affected code

StrategySupplyBase.sol#L123

## Rejected Primary Findings

# Rejected Primary Findings: BakerFi Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** captured_from_authenticated_browser
