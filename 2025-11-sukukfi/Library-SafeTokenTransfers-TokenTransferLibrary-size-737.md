
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @title SafeTokenTransfers
 * @notice Library for safe token transfers with strict balance validation
 *
 * This library enforces exact balance changes to prevent fee-on-transfer exploits,
 * accounting mismatches, and silent value leakage in vault systems.
 *
 * COMPATIBLE TOKENS (Standard ERC20):
 * - USDC, DAI, USDT (without fees enabled)
 * - Standard wrapped tokens (WETH, WBTC)
 * - Most ERC20 tokens that transfer exact amounts
 *
 * INCOMPATIBLE TOKENS (will revert with TransferAmountMismatch):
 * - Fee-on-transfer tokens (SAFEMOON, USDT with fees, etc.)
 * - Rebase tokens (stETH, aTokens, AMPL)
 * - Elastic supply tokens
 * - Tokens with transfer hooks that modify balances
 * - Any token that doesn't deliver exact transfer amounts
 *
 * USAGE WARNING:
 * Before deploying a vault with a new token, verify that the token:
 * 1. Transfers exactly the specified amount (no fees)
 * 2. Does not rebase or change balances automatically
 * 3. Does not have transfer hooks that modify amounts
 *
 * Test with small amounts first to ensure compatibility.
 *
 * @dev The balance validation check will reject any token where
 * recipientBalanceAfter != recipientBalanceBefore + amount
 */
library SafeTokenTransfers {
    using SafeERC20 for IERC20Metadata;

    /// @dev Transfer amount mismatch (fee-on-transfer or rebase token detected)
    error TransferAmountMismatch();

    /**
     * @dev Safely transfer tokens with balance validation to protect against fee-on-transfer tokens
     * @param token The token contract address
     * @param recipient The recipient address
     * @param amount The amount to transfer
     */
    function safeTransfer(address token, address recipient, uint256 amount) internal {
        uint256 balanceBefore = IERC20Metadata(token).balanceOf(recipient);
        IERC20Metadata(token).safeTransfer(recipient, amount);
        uint256 balanceAfter = IERC20Metadata(token).balanceOf(recipient);
        if (balanceAfter != balanceBefore + amount) revert TransferAmountMismatch();
    }

    /**
     * @dev Safely transfer tokens from sender to recipient with balance validation
     * @param token The token contract address
     * @param sender The sender address
     * @param recipient The recipient address
     * @param amount The amount to transfer
     */
    function safeTransferFrom(address token, address sender, address recipient, uint256 amount) internal {
        uint256 balanceBefore = IERC20Metadata(token).balanceOf(recipient);
        IERC20Metadata(token).safeTransferFrom(sender, recipient, amount);
        uint256 balanceAfter = IERC20Metadata(token).balanceOf(recipient);
        if (balanceAfter != balanceBefore + amount) revert TransferAmountMismatch();
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

