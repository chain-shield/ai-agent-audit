
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {NATIVE_TOKEN_ADDRESS} from "../math/constants.sol";
import {IFlashAccountant} from "../interfaces/IFlashAccountant.sol";
import {Locker} from "../types/locker.sol";

/// @title FlashAccountant
/// @notice Abstract contract that provides flash loan accounting functionality using transient storage
/// @dev This contract manages debt tracking for flash loans, allowing users to borrow tokens temporarily
///      and ensuring all debts are settled before the transaction completes. Uses transient storage
///      for gas-efficient temporary state management within a single transaction.
abstract contract FlashAccountant is IFlashAccountant {
    // These offsets are selected so that they do not accidentally overlap with any other base contract's use of transient storage

    /// @dev Transient storage slot for tracking the current locker ID and address
    /// @dev The stored ID is kept as id + 1 to facilitate the NotLocked check (zero means unlocked)
    /// @dev Generated using: cast keccak "FlashAccountant#CURRENT_LOCKER_SLOT"
    uint256 private constant _CURRENT_LOCKER_SLOT = 0x07cc7f5195d862f505d6b095c82f92e00cfc1766f5bca4383c28dc5fca1555fd;

    /// @dev Transient storage offset for tracking token debts for each locker
    /// @dev Generated using: cast keccak "FlashAccountant#_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET"
    uint256 private constant _DEBT_LOCKER_TOKEN_ADDRESS_OFFSET =
        0x753dfe4b4dfb3ff6c11bbf6a97f3c094e91c003ce904a55cc5662fbad220f599;

    /// @dev Transient storage offset for tracking the count of tokens with non-zero debt for each locker
    /// @dev Generated using: cast keccak "FlashAccountant#NONZERO_DEBT_COUNT_OFFSET"
    uint256 private constant _NONZERO_DEBT_COUNT_OFFSET =
        0x7772acfd7e0f66ebb20a058830296c3dc1301b111d23348e1c961d324223190d;

    /// @dev Transient storage offset for tracking token balances during payment operations
    /// @dev Generated using: cast keccak "FlashAccountant#_PAYMENT_TOKEN_ADDRESS_OFFSET"
    uint256 private constant _PAYMENT_TOKEN_ADDRESS_OFFSET =
        0x6747da56dbd05b26a7ecd2a0106781585141cf07098ad54c0e049e4e86dccb8c;

    /// @notice Gets the current locker information from transient storage
    /// @dev Reverts with NotLocked() if no lock is currently active
    /// @return locker The current locker containing both id and address
    function _getLocker() internal view returns (Locker locker) {
        assembly ("memory-safe") {
            locker := tload(_CURRENT_LOCKER_SLOT)

            if iszero(locker) {
                // cast sig "NotLocked()"
                mstore(0, shl(224, 0x1834e265))
                revert(0, 4)
            }
        }
    }

    /// @notice Gets the current locker information and ensures the caller is the locker
    /// @dev Reverts with LockerOnly() if the caller is not the current locker
    /// @return locker The current locker containing both id and address (address must be msg.sender)
    function _requireLocker() internal view returns (Locker locker) {
        locker = _getLocker();
        if (locker.addr() != msg.sender) revert LockerOnly();
    }

    /// @notice Updates the debt tracking for a specific locker and token
    /// @dev We assume debtChange cannot exceed a 128 bits value, even though it uses a int256 container.
    ///      This must be enforced at the places it is called for this contract's safety.
    ///      Negative values erase debt, positive values add debt.
    ///      Updates the non-zero debt count when debt transitions between zero and non-zero states.
    /// @param id The locker ID to update debt for
    /// @param token The token address to update debt for
    /// @param debtChange The change in debt (negative to reduce, positive to increase)
    function _accountDebt(uint256 id, address token, int256 debtChange) internal {
        assembly ("memory-safe") {
            let deltaSlot := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), token))
            let current := tload(deltaSlot)

            // we know this never overflows because debtChange is only ever derived from 128 bit values in inheriting contracts
            let next := add(current, debtChange)

            let countChange := sub(iszero(current), iszero(next))

            if countChange {
                let nzdCountSlot := add(id, _NONZERO_DEBT_COUNT_OFFSET)
                tstore(nzdCountSlot, add(tload(nzdCountSlot), countChange))
            }

            tstore(deltaSlot, next)
        }
    }

    /// @notice Updates the debt tracking for a specific locker and pair of tokens in a single operation
    /// @dev Optimized version that updates both tokens' debts and performs a single tload/tstore on the non-zero debt count.
    ///      Individual token debt slots are still updated separately, but the non-zero debt count is only loaded/stored once.
    ///      We assume debtChange values cannot exceed 128 bits. This must be enforced at the places it is called for this contract's safety.
    ///      Negative values erase debt, positive values add debt.
    /// @param id The locker ID to update debt for
    /// @param tokenA The first token address
    /// @param tokenB The second token address
    /// @param debtChangeA The change in debt for tokenA (negative to reduce, positive to increase)
    /// @param debtChangeB The change in debt for tokenB (negative to reduce, positive to increase)
    function _updatePairDebt(uint256 id, address tokenA, address tokenB, int256 debtChangeA, int256 debtChangeB)
        internal
    {
        assembly ("memory-safe") {
            let nzdCountChange := 0

            // Update token0 debt if there's a change
            if debtChangeA {
                let deltaSlotA := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), tokenA))
                let currentA := tload(deltaSlotA)
                let nextA := add(currentA, debtChangeA)

                nzdCountChange := sub(iszero(currentA), iszero(nextA))

                tstore(deltaSlotA, nextA)
            }

            if debtChangeB {
                let deltaSlotB := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), tokenB))
                let currentB := tload(deltaSlotB)
                let nextB := add(currentB, debtChangeB)

                nzdCountChange := add(nzdCountChange, sub(iszero(currentB), iszero(nextB)))

                tstore(deltaSlotB, nextB)
            }

            // Update non-zero debt count only if it changed
            if nzdCountChange {
                let nzdCountSlot := add(id, _NONZERO_DEBT_COUNT_OFFSET)
                tstore(nzdCountSlot, add(tload(nzdCountSlot), nzdCountChange))
            }
        }
    }

    /// @inheritdoc IFlashAccountant
    function updateDebt() external {
        if (msg.data.length != 20) {
            revert UpdateDebtMessageLength();
        }

        uint256 id = _getLocker().id();
        int256 delta;
        assembly ("memory-safe") {
            delta := signextend(15, shr(128, calldataload(4)))
        }
        _accountDebt(id, msg.sender, delta);
    }

    /// @inheritdoc IFlashAccountant
    function lock() external {
        assembly ("memory-safe") {
            let current := tload(_CURRENT_LOCKER_SLOT)

            let id := shr(160, current)

            // store the count
            tstore(_CURRENT_LOCKER_SLOT, or(shl(160, add(id, 1)), caller()))

            let free := mload(0x40)
            // Prepare call to locked_(uint256) -> selector 0
            mstore(free, 0)
            mstore(add(free, 4), id) // ID argument

            calldatacopy(add(free, 36), 4, sub(calldatasize(), 4))

            // Call the original caller with the packed data
            let success := call(gas(), caller(), 0, free, add(calldatasize(), 32), 0, 0)

            // Pass through the error on failure
            if iszero(success) {
                returndatacopy(free, 0, returndatasize())
                revert(free, returndatasize())
            }

            // Undo the "locker" state changes
            tstore(_CURRENT_LOCKER_SLOT, current)

            // Check if something is nonzero
            let nonzeroDebtCount := tload(add(_NONZERO_DEBT_COUNT_OFFSET, id))
            if nonzeroDebtCount {
                // cast sig "DebtsNotZeroed(uint256)"
                mstore(0x00, 0x9731ba37)
                mstore(0x20, id)
                revert(0x1c, 0x24)
            }

            // Directly return whatever the subcall returned
            returndatacopy(free, 0, returndatasize())
            return(free, returndatasize())
        }
    }

    /// @inheritdoc IFlashAccountant
    function forward(address to) external {
        Locker locker = _requireLocker();

        // update this lock's locker to the forwarded address for the duration of the forwarded
        // call, meaning only the forwarded address can update state
        assembly ("memory-safe") {
            tstore(_CURRENT_LOCKER_SLOT, or(shl(160, shr(160, locker)), to))

            let free := mload(0x40)

            // Prepare call to forwarded_2374103877(bytes32) -> selector 0x01
            mstore(free, shl(224, 1))
            mstore(add(free, 4), locker)

            calldatacopy(add(free, 36), 36, sub(calldatasize(), 36))

            // Call the forwardee with the packed data
            let success := call(gas(), to, 0, free, calldatasize(), 0, 0)

            // Pass through the error on failure
            if iszero(success) {
                returndatacopy(free, 0, returndatasize())
                revert(free, returndatasize())
            }

            tstore(_CURRENT_LOCKER_SLOT, locker)

            // Directly return whatever the subcall returned
            returndatacopy(free, 0, returndatasize())
            return(free, returndatasize())
        }
    }

    /// @inheritdoc IFlashAccountant
    function startPayments() external {
        assembly ("memory-safe") {
            // 0-52 are used for the balanceOf calldata
            mstore(20, address()) // Store the `account` argument.
            mstore(0, 0x70a08231000000000000000000000000) // `balanceOf(address)`.

            let free := mload(0x40)

            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 32) } {
                // clean upper 96 bits of the token argument at i
                let token := shr(96, shl(96, calldataload(i)))

                let returnLocation := add(free, sub(i, 4))

                let success := staticcall(gas(), token, 0x10, 0x24, returnLocation, 0x20)

                let tokenBalance :=
                    mul(
                        mload(returnLocation),
                        and(
                            gt(returndatasize(), 0x1f), // At least 32 bytes returned.
                            success
                        )
                    )

                tstore(add(_PAYMENT_TOKEN_ADDRESS_OFFSET, token), add(tokenBalance, success))
            }

            return(free, sub(calldatasize(), 4))
        }
    }

    /// @inheritdoc IFlashAccountant
    function completePayments() external {
        uint256 id = _getLocker().id();

        assembly ("memory-safe") {
            let paymentAmounts := mload(0x40)
            let nzdCountChange := 0

            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 32) } {
                let token := shr(96, shl(96, calldataload(i)))

                let offset := add(_PAYMENT_TOKEN_ADDRESS_OFFSET, token)
                let lastBalance := tload(offset)
                tstore(offset, 0)

                mstore(20, address()) // Store the `account` argument.
                mstore(0, 0x70a08231000000000000000000000000) // `balanceOf(address)`.

                let currentBalance :=
                    mul( // The arguments of `mul` are evaluated from right to left.
                        mload(0),
                        and( // The arguments of `and` are evaluated from right to left.
                            gt(returndatasize(), 0x1f), // At least 32 bytes returned.
                            staticcall(gas(), token, 0x10, 0x24, 0, 0x20)
                        )
                    )

                let payment :=
                    mul(
                        and(gt(lastBalance, 0), not(lt(currentBalance, lastBalance))),
                        sub(currentBalance, sub(lastBalance, 1))
                    )

                // We never expect tokens to have this much total supply
                if shr(128, payment) {
                    // cast sig "PaymentOverflow()"
                    mstore(0x00, 0x9cac58ca)
                    revert(0x1c, 4)
                }

                mstore(add(paymentAmounts, mul(16, div(i, 32))), shl(128, payment))

                if payment {
                    let deltaSlot := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), token))
                    let current := tload(deltaSlot)

                    // never overflows because of the payment overflow check that bounds payment to 128 bits
                    let next := sub(current, payment)

                    nzdCountChange := add(nzdCountChange, sub(iszero(current), iszero(next)))

                    tstore(deltaSlot, next)
                }
            }

            // Update nzdCountSlot only once if there were any changes
            if nzdCountChange {
                let nzdCountSlot := add(id, _NONZERO_DEBT_COUNT_OFFSET)
                tstore(nzdCountSlot, add(tload(nzdCountSlot), nzdCountChange))
            }

            return(paymentAmounts, mul(16, div(calldatasize(), 32)))
        }
    }

    /// @inheritdoc IFlashAccountant
    function withdraw() external {
        uint256 id = _requireLocker().id();

        assembly ("memory-safe") {
            let nzdCountChange := 0

            // Process each withdrawal entry
            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 56) } {
                let token := shr(96, calldataload(i))
                let recipient := shr(96, calldataload(add(i, 20)))
                let amount := shr(128, calldataload(add(i, 40)))

                if amount {
                    // Update debt tracking without updating nzdCountSlot yet
                    let deltaSlot := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), token))
                    let current := tload(deltaSlot)
                    let next := add(current, amount)

                    nzdCountChange := add(nzdCountChange, sub(iszero(current), iszero(next)))

                    tstore(deltaSlot, next)

                    // Perform the transfer of the withdrawn asset
                    // Note that these calls can re-enter and even relock with the same ID
                    // However the nzdCountChange is always applied as a delta at the end, meaning we load the latest value before updating it,
                    // so it's safe from re-entry
                    switch token
                    case 0 {
                        let success := call(gas(), recipient, amount, 0, 0, 0, 0)
                        if iszero(success) {
                            // cast sig "ETHTransferFailed()"
                            mstore(0x00, 0xb12d13eb)
                            revert(0x1c, 4)
                        }
                    }
                    default {
                        mstore(0x14, recipient)
                        mstore(0x34, amount)
                        mstore(0x00, 0xa9059cbb000000000000000000000000)
                        let success := call(gas(), token, 0, 0x10, 0x44, 0x00, 0x20)
                        if iszero(and(eq(mload(0x00), 1), success)) {
                            if iszero(lt(or(iszero(extcodesize(token)), returndatasize()), success)) {
                                mstore(0x00, 0x90b8ec18) // `TransferFailed()`.
                                revert(0x1c, 0x04)
                            }
                        }
                    }
                }
            }

            // Update nzdCountSlot only once if there were any changes
            if nzdCountChange {
                let nzdCountSlot := add(id, _NONZERO_DEBT_COUNT_OFFSET)
                tstore(nzdCountSlot, add(tload(nzdCountSlot), nzdCountChange))
            }

            // we return from assembly so as to prevent solidity from accessing the free memory pointer after we have written into it
            return(0, 0)
        }
    }

    /// @inheritdoc IFlashAccountant
    receive() external payable {
        uint256 id = _getLocker().id();

        // Note because we use msg.value here, this contract can never be multicallable, i.e. it should never expose the ability
        //      to delegatecall itself more than once in a single call
        unchecked {
            // We assume msg.value will never exceed type(uint128).max, so this should never cause an overflow/underflow of debt
            _accountDebt(id, NATIVE_TOKEN_ADDRESS, -int256(msg.value));
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PayableMulticallable} from "./base/PayableMulticallable.sol";
import {BaseLocker} from "./base/BaseLocker.sol";
import {UsesCore} from "./base/UsesCore.sol";
import {ICore} from "./interfaces/ICore.sol";
import {PoolKey} from "./types/poolKey.sol";
import {NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";
import {SqrtRatio} from "./types/sqrtRatio.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {FlashAccountantLib} from "./libraries/FlashAccountantLib.sol";
import {CoreLib} from "./libraries/CoreLib.sol";
import {PoolState} from "./types/poolState.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "./types/poolBalanceUpdate.sol";
import {SwapParameters, createSwapParameters} from "./types/swapParameters.sol";

/// @notice Represents a single hop in a multi-hop swap route
/// @dev Contains pool information and swap parameters for one step
struct RouteNode {
    /// @notice Pool key identifying the pool for this hop
    PoolKey poolKey;
    /// @notice Price limit for this hop (0 for no limit)
    SqrtRatio sqrtRatioLimit;
    /// @notice Number of ticks to skip ahead for gas optimization
    uint256 skipAhead;
}

/// @notice Represents a token and amount pair
/// @dev Used to specify input/output tokens and amounts in swaps
struct TokenAmount {
    /// @notice Address of the token
    address token;
    /// @notice Amount of the token (positive or negative)
    int128 amount;
}

/// @notice Represents a multi-hop swap with route and initial token amount
/// @dev Contains the complete path and starting point for a swap
struct Swap {
    /// @notice Array of route nodes defining the swap path
    RouteNode[] route;
    /// @notice Initial token and amount for the swap
    TokenAmount tokenAmount;
}

/// @title Ekubo Protocol Router
/// @author Moody Salem <moody@ekubo.org>
/// @notice Enables swapping and quoting against pools in Ekubo Protocol
/// @dev Provides high-level swap functionality including single-hop, multi-hop, and batch swaps
contract Router is UsesCore, PayableMulticallable, BaseLocker {
    using FlashAccountantLib for *;
    using CoreLib for *;

    uint256 private constant CALL_TYPE_SINGLE_SWAP = 0;
    uint256 private constant CALL_TYPE_MULTIHOP_SWAP = 1;
    uint256 private constant CALL_TYPE_MULTI_MULTIHOP_SWAP = 3; // == 1 | 2
    uint256 private constant CALL_TYPE_QUOTE = 4;

    /// @notice Thrown when a swap doesn't consume the full input amount
    error PartialSwapsDisallowed();

    /// @notice Thrown when the calculated amount doesn't meet the minimum threshold
    /// @param expectedAmount The minimum expected amount
    /// @param calculatedAmount The actual calculated amount
    error SlippageCheckFailed(int256 expectedAmount, int256 calculatedAmount);

    /// @notice Thrown when tokens don't match across multiple swaps
    /// @param index The index of the mismatched swap
    error TokensMismatch(uint256 index);

    /// @notice Constructs the Router contract
    /// @param core The core contract instance
    constructor(ICore core) BaseLocker(core) UsesCore(core) {}

    /// @notice Internal function to execute a swap against the core contract
    /// @dev Virtual function that can be overridden by derived contracts
    /// @param value Native token value to send with the swap
    /// @param poolKey Pool key identifying the pool
    /// @param params The parameters of the swap to make
    /// @return balanceUpdate Change in token0 and token1 balances of the pool
    function _swap(uint256 value, PoolKey memory poolKey, SwapParameters params)
        internal
        virtual
        returns (PoolBalanceUpdate balanceUpdate, PoolState stateAfter)
    {
        (balanceUpdate, stateAfter) = CORE.swap(value, poolKey, params.withDefaultSqrtRatioLimit());
    }

    function handleLockData(uint256, bytes memory data) internal override returns (bytes memory result) {
        uint256 callType = abi.decode(data, (uint256));

        if (callType == CALL_TYPE_SINGLE_SWAP) {
            // swap
            (
                ,
                address swapper,
                PoolKey memory poolKey,
                SwapParameters params,
                int256 calculatedAmountThreshold,
                address recipient
            ) = abi.decode(data, (uint256, address, PoolKey, SwapParameters, int256, address));

            unchecked {
                uint256 value = FixedPointMathLib.ternary(
                    !params.isToken1() && !params.isExactOut() && poolKey.token0 == NATIVE_TOKEN_ADDRESS,
                    uint128(params.amount()),
                    0
                );

                bool increasing = params.isPriceIncreasing();

                (PoolBalanceUpdate balanceUpdate,) = _swap(value, poolKey, params);

                int128 amountCalculated = params.isToken1() ? -balanceUpdate.delta0() : -balanceUpdate.delta1();
                if (amountCalculated < calculatedAmountThreshold) {
                    revert SlippageCheckFailed(calculatedAmountThreshold, amountCalculated);
                }

                if (increasing) {
                    if (balanceUpdate.delta0() != 0) {
                        ACCOUNTANT.withdraw(poolKey.token0, recipient, uint128(-balanceUpdate.delta0()));
                    }
                    if (balanceUpdate.delta1() != 0) {
                        ACCOUNTANT.payFrom(swapper, poolKey.token1, uint128(balanceUpdate.delta1()));
                    }
                } else {
                    if (balanceUpdate.delta1() != 0) {
                        ACCOUNTANT.withdraw(poolKey.token1, recipient, uint128(-balanceUpdate.delta1()));
                    }

                    if (balanceUpdate.delta0() != 0) {
                        if (poolKey.token0 == NATIVE_TOKEN_ADDRESS) {
                            int256 valueDifference = int256(value) - int256(balanceUpdate.delta0());

                            // refund the overpaid ETH to the swapper
                            if (valueDifference > 0) {
                                ACCOUNTANT.withdraw(NATIVE_TOKEN_ADDRESS, swapper, uint128(uint256(valueDifference)));
                            } else if (valueDifference < 0) {
                                SafeTransferLib.safeTransferETH(address(ACCOUNTANT), uint128(uint256(-valueDifference)));
                            }
                        } else {
                            ACCOUNTANT.payFrom(swapper, poolKey.token0, uint128(balanceUpdate.delta0()));
                        }
                    }
                }

                result = abi.encode(balanceUpdate);
            }
        } else if ((callType & CALL_TYPE_MULTIHOP_SWAP) != 0) {
            address swapper;
            Swap[] memory swaps;
            int256 calculatedAmountThreshold;

            if (callType == CALL_TYPE_MULTIHOP_SWAP) {
                Swap memory s;
                // multihopSwap
                (, swapper, s, calculatedAmountThreshold) = abi.decode(data, (uint256, address, Swap, int256));

                swaps = new Swap[](1);
                swaps[0] = s;
            } else {
                // multiMultihopSwap
                (, swapper, swaps, calculatedAmountThreshold) = abi.decode(data, (uint256, address, Swap[], int256));
            }

            PoolBalanceUpdate[][] memory results = new PoolBalanceUpdate[][](swaps.length);

            unchecked {
                int256 totalCalculated;
                int256 totalSpecified;
                address specifiedToken;
                address calculatedToken;

                for (uint256 i = 0; i < swaps.length; i++) {
                    Swap memory s = swaps[i];
                    results[i] = new PoolBalanceUpdate[](s.route.length);

                    TokenAmount memory tokenAmount = s.tokenAmount;
                    totalSpecified += tokenAmount.amount;

                    for (uint256 j = 0; j < s.route.length; j++) {
                        RouteNode memory node = s.route[j];

                        bool isToken1 = tokenAmount.token == node.poolKey.token1;
                        require(isToken1 || tokenAmount.token == node.poolKey.token0);

                        (PoolBalanceUpdate update,) = _swap(
                            0,
                            node.poolKey,
                            createSwapParameters({
                                _amount: tokenAmount.amount,
                                _isToken1: isToken1,
                                _sqrtRatioLimit: node.sqrtRatioLimit,
                                _skipAhead: node.skipAhead
                            })
                        );
                        results[i][j] = update;

                        if (isToken1) {
                            if (update.delta1() != tokenAmount.amount) revert PartialSwapsDisallowed();
                            tokenAmount = TokenAmount({token: node.poolKey.token0, amount: -update.delta0()});
                        } else {
                            if (update.delta0() != tokenAmount.amount) revert PartialSwapsDisallowed();
                            tokenAmount = TokenAmount({token: node.poolKey.token1, amount: -update.delta1()});
                        }
                    }

                    totalCalculated += tokenAmount.amount;

                    if (i == 0) {
                        specifiedToken = s.tokenAmount.token;
                        calculatedToken = tokenAmount.token;
                    } else {
                        if (specifiedToken != s.tokenAmount.token || calculatedToken != tokenAmount.token) {
                            revert TokensMismatch(i);
                        }
                    }
                }

                if (totalCalculated < calculatedAmountThreshold) {
                    revert SlippageCheckFailed(calculatedAmountThreshold, totalCalculated);
                }

                if (totalSpecified < 0) {
                    ACCOUNTANT.withdraw(specifiedToken, swapper, uint128(uint256(-totalSpecified)));
                } else if (totalSpecified > 0) {
                    if (specifiedToken == NATIVE_TOKEN_ADDRESS) {
                        SafeTransferLib.safeTransferETH(address(ACCOUNTANT), uint128(uint256(totalSpecified)));
                    } else {
                        ACCOUNTANT.payFrom(swapper, specifiedToken, uint128(uint256(totalSpecified)));
                    }
                }

                if (totalCalculated > 0) {
                    ACCOUNTANT.withdraw(calculatedToken, swapper, uint128(uint256(totalCalculated)));
                } else if (totalCalculated < 0) {
                    if (calculatedToken == NATIVE_TOKEN_ADDRESS) {
                        SafeTransferLib.safeTransferETH(address(ACCOUNTANT), uint128(uint256(-totalCalculated)));
                    } else {
                        ACCOUNTANT.payFrom(swapper, calculatedToken, uint128(uint256(-totalCalculated)));
                    }
                }
            }

            if (callType == CALL_TYPE_MULTIHOP_SWAP) {
                result = abi.encode(results[0]);
            } else {
                result = abi.encode(results);
            }
        } else if (callType == CALL_TYPE_QUOTE) {
            (, PoolKey memory poolKey, SwapParameters params) = abi.decode(data, (uint256, PoolKey, SwapParameters));

            (PoolBalanceUpdate balanceUpdate, PoolState stateAfter) = _swap(0, poolKey, params);

            revert QuoteReturnValue(balanceUpdate, stateAfter);
        }
    }

    /// @notice Executes a single-hop swap with a specified recipient
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param params The swap parameters to execute
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(PoolKey memory poolKey, SwapParameters params, int256 calculatedAmountThreshold)
        public
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        balanceUpdate = swap(poolKey, params, calculatedAmountThreshold, msg.sender);
    }

    /// @notice Executes a single-hop swap with a specified recipient
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param params The swap parameters to execute
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @param recipient Address to receive the output tokens
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(PoolKey memory poolKey, SwapParameters params, int256 calculatedAmountThreshold, address recipient)
        public
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        (balanceUpdate) = abi.decode(
            lock(abi.encode(CALL_TYPE_SINGLE_SWAP, msg.sender, poolKey, params, calculatedAmountThreshold, recipient)),
            (PoolBalanceUpdate)
        );
    }

    /// @notice Executes a single-hop swap with a specified recipient
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param isToken1 True if swapping token1, false if swapping token0
    /// @param amount Amount to swap (positive for exact input, negative for exact output)
    /// @param sqrtRatioLimit Price limit for the swap (0 for no limit)
    /// @param skipAhead Number of ticks to skip ahead for gas optimization
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @param recipient Address to receive the output tokens
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(
        PoolKey memory poolKey,
        bool isToken1,
        int128 amount,
        SqrtRatio sqrtRatioLimit,
        uint256 skipAhead,
        int256 calculatedAmountThreshold,
        address recipient
    ) public payable returns (PoolBalanceUpdate balanceUpdate) {
        balanceUpdate = swap(
            poolKey,
            createSwapParameters({
                _isToken1: isToken1, _amount: amount, _sqrtRatioLimit: sqrtRatioLimit, _skipAhead: skipAhead
            }),
            calculatedAmountThreshold,
            recipient
        );
    }

    /// @notice Executes a single-hop swap with msg.sender as recipient
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param isToken1 True if swapping token1, false if swapping token0
    /// @param amount Amount to swap (positive for exact input, negative for exact output)
    /// @param sqrtRatioLimit Price limit for the swap (0 for no limit)
    /// @param skipAhead Number of ticks to skip ahead for gas optimization
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(
        PoolKey memory poolKey,
        bool isToken1,
        int128 amount,
        SqrtRatio sqrtRatioLimit,
        uint256 skipAhead,
        int256 calculatedAmountThreshold
    ) external payable returns (PoolBalanceUpdate balanceUpdate) {
        balanceUpdate = swap(
            poolKey, isToken1, amount, sqrtRatioLimit, skipAhead, calculatedAmountThreshold, msg.sender
        );
    }

    /// @notice Executes a single-hop swap with no slippage protection
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param isToken1 True if swapping token1, false if swapping token0
    /// @param amount Amount to swap (positive for exact input, negative for exact output)
    /// @param sqrtRatioLimit Price limit for the swap (0 for no limit)
    /// @param skipAhead Number of ticks to skip ahead for gas optimization
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(PoolKey memory poolKey, bool isToken1, int128 amount, SqrtRatio sqrtRatioLimit, uint256 skipAhead)
        external
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        balanceUpdate = swap(poolKey, isToken1, amount, sqrtRatioLimit, skipAhead, type(int256).min, msg.sender);
    }

    /// @notice Executes a single-hop swap using RouteNode and TokenAmount structs
    /// @param node Route node containing pool and swap parameters
    /// @param tokenAmount Token and amount to swap
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(RouteNode memory node, TokenAmount memory tokenAmount, int256 calculatedAmountThreshold)
        public
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        balanceUpdate = swap(
            node.poolKey,
            node.poolKey.token1 == tokenAmount.token,
            tokenAmount.amount,
            node.sqrtRatioLimit,
            node.skipAhead,
            calculatedAmountThreshold,
            msg.sender
        );
    }

    /// @notice Executes a multi-hop swap through multiple pools
    /// @param s Swap struct containing the route and initial token amount
    /// @param calculatedAmountThreshold Minimum final amount to receive (for slippage protection)
    /// @return result Array of deltas for each hop in the swap
    function multihopSwap(Swap memory s, int256 calculatedAmountThreshold)
        external
        payable
        returns (PoolBalanceUpdate[] memory result)
    {
        result = abi.decode(
            lock(abi.encode(CALL_TYPE_MULTIHOP_SWAP, msg.sender, s, calculatedAmountThreshold)), (PoolBalanceUpdate[])
        );
    }

    /// @notice Executes multiple multi-hop swaps in a single transaction
    /// @param swaps Array of swap structs, each containing a route and initial token amount
    /// @param calculatedAmountThreshold Minimum total final amount to receive (for slippage protection)
    /// @return results Array of delta arrays, one for each swap
    function multiMultihopSwap(Swap[] memory swaps, int256 calculatedAmountThreshold)
        external
        payable
        returns (PoolBalanceUpdate[][] memory results)
    {
        results = abi.decode(
            lock(abi.encode(CALL_TYPE_MULTI_MULTIHOP_SWAP, msg.sender, swaps, calculatedAmountThreshold)),
            (PoolBalanceUpdate[][])
        );
    }

    /// @notice Error used to return quote values from the quote function
    /// @param balanceUpdate Change in token0 and token1 balance of the pool
    /// @param poolState The state after the swap
    error QuoteReturnValue(PoolBalanceUpdate balanceUpdate, PoolState poolState);

    /// @notice Quotes the result of a swap without executing it
    /// @dev Uses a revert-based mechanism to return the quote without state changes
    /// @param poolKey Pool key identifying the pool to quote against
    /// @param isToken1 True if swapping token1, false if swapping token0
    /// @param amount Amount to swap (positive for exact input, negative for exact output)
    /// @param sqrtRatioLimit Price limit for the swap (0 for no limit)
    /// @param skipAhead Number of ticks to skip ahead for gas optimization
    /// @return balanceUpdate The change in pool balances resulting from the swap
    /// @return stateAfter The state of the pool after the swap is complete
    function quote(PoolKey memory poolKey, bool isToken1, int128 amount, SqrtRatio sqrtRatioLimit, uint256 skipAhead)
        external
        returns (PoolBalanceUpdate balanceUpdate, PoolState stateAfter)
    {
        bytes memory revertData = lockAndExpectRevert(
            abi.encode(
                CALL_TYPE_QUOTE,
                poolKey,
                createSwapParameters({
                    _isToken1: isToken1, _amount: amount, _sqrtRatioLimit: sqrtRatioLimit, _skipAhead: skipAhead
                })
            )
        );

        // check that the sig matches the error data

        bytes4 sig;
        assembly ("memory-safe") {
            sig := mload(add(revertData, 32))
        }
        if (sig == QuoteReturnValue.selector && revertData.length == 68) {
            assembly ("memory-safe") {
                balanceUpdate := mload(add(revertData, 36))
                stateAfter := mload(add(revertData, 68))
            }
        } else {
            assembly ("memory-safe") {
                revert(add(revertData, 32), mload(revertData))
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints, addressToCallPoints} from "./types/callPoints.sol";
import {PoolKey} from "./types/poolKey.sol";
import {PoolConfig} from "./types/poolConfig.sol";
import {PositionId} from "./types/positionId.sol";
import {FeesPerLiquidity, feesPerLiquidityFromAmounts} from "./types/feesPerLiquidity.sol";
import {Position} from "./types/position.sol";
import {tickToSqrtRatio, sqrtRatioToTick} from "./math/ticks.sol";
import {CoreStorageLayout} from "./libraries/CoreStorageLayout.sol";
import {ExtensionCallPointsLib} from "./libraries/ExtensionCallPointsLib.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {ExposedStorage} from "./base/ExposedStorage.sol";
import {liquidityDeltaToAmountDelta, addLiquidityDelta} from "./math/liquidity.sol";
import {findNextInitializedTick, findPrevInitializedTick, flipTick} from "./math/tickBitmap.sol";
import {ICore, IExtension} from "./interfaces/ICore.sol";
import {FlashAccountant} from "./base/FlashAccountant.sol";
import {MIN_TICK, MAX_TICK, NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";
import {MIN_SQRT_RATIO, MAX_SQRT_RATIO, SqrtRatio} from "./types/sqrtRatio.sol";
import {PoolState, createPoolState} from "./types/poolState.sol";
import {SwapParameters} from "./types/swapParameters.sol";
import {TickInfo, createTickInfo} from "./types/tickInfo.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "./types/poolBalanceUpdate.sol";
import {PoolId} from "./types/poolId.sol";
import {Locker} from "./types/locker.sol";
import {computeFee, amountBeforeFee} from "./math/fee.sol";
import {nextSqrtRatioFromAmount0, nextSqrtRatioFromAmount1} from "./math/sqrtRatio.sol";
import {
    amount0Delta,
    amount1Delta,
    amount0DeltaSorted,
    amount1DeltaSorted,
    sortAndConvertToFixedSqrtRatios
} from "./math/delta.sol";
import {StorageSlot} from "./types/storageSlot.sol";
import {LibBit} from "solady/utils/LibBit.sol";

/// @title Ekubo Protocol Core
/// @author Moody Salem <moody@ekubo.org>
/// @notice Singleton contract holding all tokens and containing all possible operations in Ekubo Protocol
/// @dev Implements the core AMM functionality including pools, positions, swaps, and fee collection
/// @dev Note this code is under the terms of the Ekubo DAO Shared Revenue License 1.0.
/// @dev The full terms of the license can be found at the contenthash specified at ekubo-license-v1.eth.
contract Core is ICore, FlashAccountant, ExposedStorage {
    using ExtensionCallPointsLib for *;

    /// @inheritdoc ICore
    function registerExtension(CallPoints memory expectedCallPoints) external {
        CallPoints memory computed = addressToCallPoints(msg.sender);
        if (!computed.eq(expectedCallPoints) || !computed.isValid()) {
            revert FailedRegisterInvalidCallPoints();
        }
        StorageSlot isExtensionRegisteredSlot = CoreStorageLayout.isExtensionRegisteredSlot(msg.sender);
        if (isExtensionRegisteredSlot.load() != bytes32(0)) revert ExtensionAlreadyRegistered();

        isExtensionRegisteredSlot.store(bytes32(LibBit.rawToUint(true)));

        emit ExtensionRegistered(msg.sender);
    }

    function readPoolState(PoolId poolId) internal view returns (PoolState state) {
        state = PoolState.wrap(CoreStorageLayout.poolStateSlot(poolId).load());
    }

    function writePoolState(PoolId poolId, PoolState state) internal {
        CoreStorageLayout.poolStateSlot(poolId).store(PoolState.unwrap(state));
    }

    /// @inheritdoc ICore
    function initializePool(PoolKey memory poolKey, int32 tick) external returns (SqrtRatio sqrtRatio) {
        poolKey.validate();

        address extension = poolKey.config.extension();
        if (extension != address(0)) {
            StorageSlot isExtensionRegisteredSlot = CoreStorageLayout.isExtensionRegisteredSlot(extension);

            if (isExtensionRegisteredSlot.load() == bytes32(0)) {
                revert ExtensionNotRegistered();
            }

            IExtension(extension).maybeCallBeforeInitializePool(msg.sender, poolKey, tick);
        }

        PoolId poolId = poolKey.toPoolId();
        PoolState state = readPoolState(poolId);
        if (state.isInitialized()) revert PoolAlreadyInitialized();

        sqrtRatio = tickToSqrtRatio(tick);
        writePoolState(poolId, createPoolState({_sqrtRatio: sqrtRatio, _tick: tick, _liquidity: 0}));

        // initialize these slots so the first swap or deposit on the pool is the same cost as any other swap
        StorageSlot fplSlot0 = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
        fplSlot0.store(bytes32(uint256(1)));
        fplSlot0.next().store(bytes32(uint256(1)));

        emit PoolInitialized(poolId, poolKey, tick, sqrtRatio);

        IExtension(extension).maybeCallAfterInitializePool(msg.sender, poolKey, tick, sqrtRatio);
    }

    /// @inheritdoc ICore
    function prevInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized)
    {
        (tick, isInitialized) =
            findPrevInitializedTick(CoreStorageLayout.tickBitmapsSlot(poolId), fromTick, tickSpacing, skipAhead);
    }

    /// @inheritdoc ICore
    function nextInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized)
    {
        (tick, isInitialized) =
            findNextInitializedTick(CoreStorageLayout.tickBitmapsSlot(poolId), fromTick, tickSpacing, skipAhead);
    }

    /// @inheritdoc ICore
    function updateSavedBalances(
        address token0,
        address token1,
        bytes32,
        // positive is saving, negative is loading
        int256 delta0,
        int256 delta1
    )
        external
        payable
    {
        if (token0 >= token1) revert SavedBalanceTokensNotSorted();

        (uint256 id, address lockerAddr) = _requireLocker().parse();

        assembly ("memory-safe") {
            function addDelta(u, i) -> result {
                // full‐width sum mod 2^256
                let sum := add(u, i)
                // 1 if i<0 else 0
                let sign := shr(255, i)
                // if sum > type(uint128).max || (i>=0 && sum<u) || (i<0 && sum>u) ⇒ 256-bit wrap or underflow
                if or(shr(128, sum), or(and(iszero(sign), lt(sum, u)), and(sign, gt(sum, u)))) {
                    mstore(0x00, 0x1293d6fa) // `SavedBalanceOverflow()`
                    revert(0x1c, 0x04)
                }
                result := sum
            }

            // we can cheaply calldatacopy the arguments into memory, hence no call to CoreStorageLayout#savedBalancesSlot
            let free := mload(0x40)
            mstore(free, lockerAddr)
            // copy the first 3 arguments in the same order
            calldatacopy(add(free, 0x20), 4, 96)
            let slot := keccak256(free, 128)
            let balances := sload(slot)

            let b0 := shr(128, balances)
            let b1 := shr(128, shl(128, balances))

            let b0Next := addDelta(b0, delta0)
            let b1Next := addDelta(b1, delta1)

            sstore(slot, add(shl(128, b0Next), b1Next))
        }

        _updatePairDebtWithNative(id, token0, token1, delta0, delta1);
    }

    /// @notice Returns the pool fees per liquidity inside the given bounds
    /// @dev Internal function that calculates fees per liquidity within position bounds
    /// @param poolId Unique identifier for the pool
    /// @param tick Current tick of the pool
    /// @param tickLower Lower tick of the price range to get the snapshot of
    /// @param tickLower Upper tick of the price range to get the snapshot of
    /// @return feesPerLiquidityInside Accumulated fees per liquidity snapshot inside the bounds. Note this is a relative value.
    function _getPoolFeesPerLiquidityInside(PoolId poolId, int32 tick, int32 tickLower, int32 tickUpper)
        internal
        view
        returns (FeesPerLiquidity memory feesPerLiquidityInside)
    {
        uint256 lower0;
        uint256 lower1;
        uint256 upper0;
        uint256 upper1;
        {
            (StorageSlot l0, StorageSlot l1) = CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, tickLower);
            (lower0, lower1) = (uint256(l0.load()), uint256(l1.load()));

            (StorageSlot u0, StorageSlot u1) = CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, tickUpper);
            (upper0, upper1) = (uint256(u0.load()), uint256(u1.load()));
        }

        unchecked {
            if (tick < tickLower) {
                feesPerLiquidityInside.value0 = lower0 - upper0;
                feesPerLiquidityInside.value1 = lower1 - upper1;
            } else if (tick < tickUpper) {
                uint256 global0;
                uint256 global1;
                {
                    (bytes32 g0, bytes32 g1) = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).loadTwo();
                    (global0, global1) = (uint256(g0), uint256(g1));
                }

                feesPerLiquidityInside.value0 = global0 - upper0 - lower0;
                feesPerLiquidityInside.value1 = global1 - upper1 - lower1;
            } else {
                feesPerLiquidityInside.value0 = upper0 - lower0;
                feesPerLiquidityInside.value1 = upper1 - lower1;
            }
        }
    }

    /// @inheritdoc ICore
    function getPoolFeesPerLiquidityInside(PoolId poolId, int32 tickLower, int32 tickUpper)
        external
        view
        returns (FeesPerLiquidity memory)
    {
        return _getPoolFeesPerLiquidityInside(poolId, readPoolState(poolId).tick(), tickLower, tickUpper);
    }

    /// @inheritdoc ICore
    function accumulateAsFees(PoolKey memory poolKey, uint128 _amount0, uint128 _amount1) external payable {
        (uint256 id, address lockerAddr) = _requireLocker().parse();
        require(lockerAddr == poolKey.config.extension());

        PoolId poolId = poolKey.toPoolId();

        uint256 amount0;
        uint256 amount1;
        assembly ("memory-safe") {
            amount0 := _amount0
            amount1 := _amount1
        }

        // Note we do not check pool is initialized. If the extension calls this for a pool that does not exist,
        //  the fees are simply burned since liquidity is 0.

        if (amount0 != 0 || amount1 != 0) {
            uint256 liquidity;
            {
                uint128 _liquidity = readPoolState(poolId).liquidity();
                assembly ("memory-safe") {
                    liquidity := _liquidity
                }
            }

            unchecked {
                if (liquidity != 0) {
                    StorageSlot slot0 = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);

                    if (amount0 != 0) {
                        slot0.store(
                            bytes32(uint256(slot0.load()) + FixedPointMathLib.rawDiv(amount0 << 128, liquidity))
                        );
                    }
                    if (amount1 != 0) {
                        StorageSlot slot1 = slot0.next();
                        slot1.store(
                            bytes32(uint256(slot1.load()) + FixedPointMathLib.rawDiv(amount1 << 128, liquidity))
                        );
                    }
                }
            }
        }

        // whether the fees are actually accounted to any position, the caller owes the debt
        _updatePairDebtWithNative(id, poolKey.token0, poolKey.token1, int256(amount0), int256(amount1));

        emit FeesAccumulated(poolId, _amount0, _amount1);
    }

    /// @notice Updates tick information when liquidity is added or removed
    /// @dev Private function that handles tick initialization and liquidity tracking
    /// @param poolId Unique identifier for the pool
    /// @param tick Tick to update
    /// @param poolConfig Pool configuration containing tick spacing
    /// @param liquidityDelta Change in liquidity
    /// @param isUpper Whether this is the upper bound of a position
    function _updateTick(PoolId poolId, int32 tick, PoolConfig poolConfig, int128 liquidityDelta, bool isUpper)
        private
    {
        StorageSlot tickInfoSlot = CoreStorageLayout.poolTicksSlot(poolId, tick);

        (int128 currentLiquidityDelta, uint128 currentLiquidityNet) = TickInfo.wrap(tickInfoSlot.load()).parse();
        uint128 liquidityNetNext = addLiquidityDelta(currentLiquidityNet, liquidityDelta);
        // this is checked math
        int128 liquidityDeltaNext =
            isUpper ? currentLiquidityDelta - liquidityDelta : currentLiquidityDelta + liquidityDelta;

        // Check that liquidityNet doesn't exceed max liquidity per tick
        uint128 maxLiquidity = poolConfig.concentratedMaxLiquidityPerTick();
        if (liquidityNetNext > maxLiquidity) {
            revert MaxLiquidityPerTickExceeded(tick, liquidityNetNext, maxLiquidity);
        }

        if ((currentLiquidityNet == 0) != (liquidityNetNext == 0)) {
            flipTick(CoreStorageLayout.tickBitmapsSlot(poolId), tick, poolConfig.concentratedTickSpacing());

            (StorageSlot fplSlot0, StorageSlot fplSlot1) =
                CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, tick);

            bytes32 v;
            assembly ("memory-safe") {
                v := gt(liquidityNetNext, 0)
            }

            // initialize the storage slots for the fees per liquidity outside to non-zero so tick crossing is cheaper
            fplSlot0.store(v);
            fplSlot1.store(v);
        }

        tickInfoSlot.store(TickInfo.unwrap(createTickInfo(liquidityDeltaNext, liquidityNetNext)));
    }

    /// @notice Updates debt for a token pair, handling native token payments for token0
    /// @dev Optimized version that updates both tokens' debts in a single operation when possible.
    ///      Assumes token0 < token1 (tokens are sorted).
    /// @param id Lock ID for debt tracking
    /// @param token0 Address of token0 (must be < token1)
    /// @param token1 Address of token1 (must be > token0)
    /// @param debtChange0 Change in debt amount for token0
    /// @param debtChange1 Change in debt amount for token1
    function _updatePairDebtWithNative(
        uint256 id,
        address token0,
        address token1,
        int256 debtChange0,
        int256 debtChange1
    ) private {
        if (msg.value == 0) {
            // No native token payment included in the call, so use optimized pair update
            _updatePairDebt(id, token0, token1, debtChange0, debtChange1);
        } else {
            if (token0 == NATIVE_TOKEN_ADDRESS) {
                unchecked {
                    // token0 is native, so we can still use pair update with adjusted debtChange0
                    // Subtraction is safe because debtChange0 and msg.value are both bounded by int128/uint128
                    _updatePairDebt(id, token0, token1, debtChange0 - int256(msg.value), debtChange1);
                }
            } else {
                // token0 is not native, and since token0 < token1, token1 cannot be native either
                // Update the token0, token1 debt and then update native token debt separately
                unchecked {
                    _updatePairDebt(id, token0, token1, debtChange0, debtChange1);
                    _accountDebt(id, NATIVE_TOKEN_ADDRESS, -int256(msg.value));
                }
            }
        }
    }

    /// @inheritdoc ICore
    function updatePosition(PoolKey memory poolKey, PositionId positionId, int128 liquidityDelta)
        external
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        positionId.validate(poolKey.config);

        Locker locker = _requireLocker();

        IExtension(poolKey.config.extension())
            .maybeCallBeforeUpdatePosition(locker, poolKey, positionId, liquidityDelta);

        PoolId poolId = poolKey.toPoolId();
        PoolState state = readPoolState(poolId);
        if (!state.isInitialized()) revert PoolNotInitialized();

        if (liquidityDelta != 0) {
            (SqrtRatio sqrtRatioLower, SqrtRatio sqrtRatioUpper) =
                (tickToSqrtRatio(positionId.tickLower()), tickToSqrtRatio(positionId.tickUpper()));

            (int128 delta0, int128 delta1) =
                liquidityDeltaToAmountDelta(state.sqrtRatio(), liquidityDelta, sqrtRatioLower, sqrtRatioUpper);

            StorageSlot positionSlot = CoreStorageLayout.poolPositionsSlot(poolId, locker.addr(), positionId);
            Position storage position;
            assembly ("memory-safe") {
                position.slot := positionSlot
            }

            uint128 liquidityNext = addLiquidityDelta(position.liquidity, liquidityDelta);

            FeesPerLiquidity memory feesPerLiquidityInside;

            if (poolKey.config.isConcentrated()) {
                // the position is fully withdrawn
                if (liquidityNext == 0) {
                    // we need to fetch it before the tick fees per liquidity outside is deleted
                    feesPerLiquidityInside = _getPoolFeesPerLiquidityInside(
                        poolId, state.tick(), positionId.tickLower(), positionId.tickUpper()
                    );
                }

                _updateTick(poolId, positionId.tickLower(), poolKey.config, liquidityDelta, false);
                _updateTick(poolId, positionId.tickUpper(), poolKey.config, liquidityDelta, true);

                if (liquidityNext != 0) {
                    feesPerLiquidityInside = _getPoolFeesPerLiquidityInside(
                        poolId, state.tick(), positionId.tickLower(), positionId.tickUpper()
                    );
                }

                if (state.tick() >= positionId.tickLower() && state.tick() < positionId.tickUpper()) {
                    state = createPoolState({
                        _sqrtRatio: state.sqrtRatio(),
                        _tick: state.tick(),
                        _liquidity: addLiquidityDelta(state.liquidity(), liquidityDelta)
                    });
                    writePoolState(poolId, state);
                }
            } else {
                // we store the active liquidity in the liquidity slot for stableswap pools
                state = createPoolState({
                    _sqrtRatio: state.sqrtRatio(),
                    _tick: state.tick(),
                    _liquidity: addLiquidityDelta(state.liquidity(), liquidityDelta)
                });
                writePoolState(poolId, state);
                StorageSlot fplFirstSlot = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
                feesPerLiquidityInside.value0 = uint256(fplFirstSlot.load());
                feesPerLiquidityInside.value1 = uint256(fplFirstSlot.next().load());
            }

            if (liquidityNext == 0) {
                position.liquidity = 0;
                position.feesPerLiquidityInsideLast = FeesPerLiquidity(0, 0);
            } else {
                (uint128 fees0, uint128 fees1) = position.fees(feesPerLiquidityInside);
                position.liquidity = liquidityNext;
                position.feesPerLiquidityInsideLast =
                    feesPerLiquidityInside.sub(feesPerLiquidityFromAmounts(fees0, fees1, liquidityNext));
            }

            _updatePairDebtWithNative(locker.id(), poolKey.token0, poolKey.token1, delta0, delta1);

            balanceUpdate = createPoolBalanceUpdate(delta0, delta1);
            emit PositionUpdated(locker.addr(), poolId, positionId, liquidityDelta, balanceUpdate, state);
        }

        IExtension(poolKey.config.extension())
            .maybeCallAfterUpdatePosition(locker, poolKey, positionId, liquidityDelta, balanceUpdate, state);
    }

    /// @inheritdoc ICore
    function setExtraData(PoolId poolId, PositionId positionId, bytes16 _extraData) external {
        StorageSlot firstSlot = CoreStorageLayout.poolPositionsSlot(poolId, msg.sender, positionId);

        bytes32 extraData;
        assembly ("memory-safe") {
            extraData := _extraData
        }

        firstSlot.store(((firstSlot.load() >> 128) << 128) | (extraData >> 128));
    }

    /// @inheritdoc ICore
    function collectFees(PoolKey memory poolKey, PositionId positionId)
        external
        returns (uint128 amount0, uint128 amount1)
    {
        Locker locker = _requireLocker();

        IExtension(poolKey.config.extension()).maybeCallBeforeCollectFees(locker, poolKey, positionId);

        PoolId poolId = poolKey.toPoolId();

        Position storage position;
        StorageSlot positionSlot = CoreStorageLayout.poolPositionsSlot(poolId, locker.addr(), positionId);
        assembly ("memory-safe") {
            position.slot := positionSlot
        }

        FeesPerLiquidity memory feesPerLiquidityInside;
        if (poolKey.config.isStableswap()) {
            // Stableswap pools: use global fees per liquidity
            StorageSlot fplFirstSlot = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
            feesPerLiquidityInside.value0 = uint256(fplFirstSlot.load());
            feesPerLiquidityInside.value1 = uint256(fplFirstSlot.next().load());
        } else {
            // Concentrated pools: calculate fees per liquidity inside the position bounds
            feesPerLiquidityInside = _getPoolFeesPerLiquidityInside(
                poolId, readPoolState(poolId).tick(), positionId.tickLower(), positionId.tickUpper()
            );
        }

        (amount0, amount1) = position.fees(feesPerLiquidityInside);

        position.feesPerLiquidityInsideLast = feesPerLiquidityInside;

        _updatePairDebt(
            locker.id(), poolKey.token0, poolKey.token1, -int256(uint256(amount0)), -int256(uint256(amount1))
        );

        emit PositionFeesCollected(locker.addr(), poolId, positionId, amount0, amount1);

        IExtension(poolKey.config.extension()).maybeCallAfterCollectFees(locker, poolKey, positionId, amount0, amount1);
    }

    /// @inheritdoc ICore
    function swap_6269342730() external payable {
        unchecked {
            PoolKey memory poolKey;
            address token0;
            address token1;
            PoolConfig config;

            SwapParameters params;

            assembly ("memory-safe") {
                token0 := calldataload(4)
                token1 := calldataload(36)
                config := calldataload(68)
                params := calldataload(100)
                calldatacopy(poolKey, 4, 96)
            }

            SqrtRatio sqrtRatioLimit = params.sqrtRatioLimit();
            if (!sqrtRatioLimit.isValid()) revert InvalidSqrtRatioLimit();

            Locker locker = _requireLocker();

            IExtension(config.extension()).maybeCallBeforeSwap(locker, poolKey, params);

            PoolId poolId = poolKey.toPoolId();

            PoolState stateAfter = readPoolState(poolId);

            if (!stateAfter.isInitialized()) revert PoolNotInitialized();

            int256 amountRemaining = params.amount();

            PoolBalanceUpdate balanceUpdate;

            // 0 swap amount or sqrt ratio limit == sqrt ratio is no-op
            if (amountRemaining != 0 && stateAfter.sqrtRatio() != sqrtRatioLimit) {
                (SqrtRatio sqrtRatio, int32 tick, uint128 liquidity) = stateAfter.parse();

                bool isToken1 = params.isToken1();

                bool isExactOut = amountRemaining < 0;
                bool increasing;
                assembly ("memory-safe") {
                    increasing := xor(isToken1, isExactOut)
                }

                if ((sqrtRatioLimit < sqrtRatio) == increasing) {
                    revert SqrtRatioLimitWrongDirection();
                }

                int256 calculatedAmount;

                // fees per liquidity only for the input token
                uint256 inputTokenFeesPerLiquidity;

                // 0 = not loaded & not updated, 1 = loaded & not updated, 2 = loaded & updated
                uint256 feesAccessed;

                while (true) {
                    int32 nextTick;
                    bool isInitialized;
                    SqrtRatio nextTickSqrtRatio;

                    // For stableswap pools, determine active liquidity for this step
                    uint128 stepLiquidity = liquidity;

                    if (config.isStableswap()) {
                        if (config.isFullRange()) {
                            // special case since we don't need to compute min/max tick sqrt ratio
                            (nextTick, nextTickSqrtRatio) =
                                increasing ? (MAX_TICK, MAX_SQRT_RATIO) : (MIN_TICK, MIN_SQRT_RATIO);
                        } else {
                            (int32 lower, int32 upper) = config.stableswapActiveLiquidityTickRange();

                            bool inRange;
                            assembly ("memory-safe") {
                                inRange := and(slt(tick, upper), iszero(slt(tick, lower)))
                            }
                            if (inRange) {
                                nextTick = increasing ? upper : lower;
                                nextTickSqrtRatio = tickToSqrtRatio(nextTick);
                            } else {
                                if (tick < lower) {
                                    (nextTick, nextTickSqrtRatio) =
                                        increasing ? (lower, tickToSqrtRatio(lower)) : (MIN_TICK, MIN_SQRT_RATIO);
                                } else {
                                    // tick >= upper implied
                                    (nextTick, nextTickSqrtRatio) =
                                        increasing ? (MAX_TICK, MAX_SQRT_RATIO) : (upper, tickToSqrtRatio(upper));
                                }
                                stepLiquidity = 0;
                            }
                        }
                    } else {
                        // concentrated liquidity pools use the tick bitmaps
                        (nextTick, isInitialized) = increasing
                            ? findNextInitializedTick(
                                CoreStorageLayout.tickBitmapsSlot(poolId),
                                tick,
                                config.concentratedTickSpacing(),
                                params.skipAhead()
                            )
                            : findPrevInitializedTick(
                                CoreStorageLayout.tickBitmapsSlot(poolId),
                                tick,
                                config.concentratedTickSpacing(),
                                params.skipAhead()
                            );

                        nextTickSqrtRatio = tickToSqrtRatio(nextTick);
                    }

                    SqrtRatio limitedNextSqrtRatio =
                        increasing ? nextTickSqrtRatio.min(sqrtRatioLimit) : nextTickSqrtRatio.max(sqrtRatioLimit);

                    SqrtRatio sqrtRatioNext;

                    if (stepLiquidity == 0) {
                        // if the pool is empty, the swap will always move all the way to the limit price
                        sqrtRatioNext = limitedNextSqrtRatio;
                    } else {
                        // this amount is what moves the price
                        int128 priceImpactAmount;
                        if (isExactOut) {
                            assembly ("memory-safe") {
                                priceImpactAmount := amountRemaining
                            }
                        } else {
                            uint128 amountU128;
                            assembly ("memory-safe") {
                                // cast is safe because amountRemaining is g.t. 0 and fits in int128
                                amountU128 := amountRemaining
                            }
                            uint128 feeAmount = computeFee(amountU128, config.fee());
                            assembly ("memory-safe") {
                                // feeAmount will never exceed amountRemaining since fee is < 100%
                                priceImpactAmount := sub(amountRemaining, feeAmount)
                            }
                        }

                        SqrtRatio sqrtRatioNextFromAmount = isToken1
                            ? nextSqrtRatioFromAmount1(sqrtRatio, stepLiquidity, priceImpactAmount)
                            : nextSqrtRatioFromAmount0(sqrtRatio, stepLiquidity, priceImpactAmount);

                        bool hitLimit;
                        assembly ("memory-safe") {
                            // Branchless limit check: (increasing && next > limit) || (!increasing && next < limit)
                            let exceedsUp := and(increasing, gt(sqrtRatioNextFromAmount, limitedNextSqrtRatio))
                            let exceedsDown :=
                                and(iszero(increasing), lt(sqrtRatioNextFromAmount, limitedNextSqrtRatio))
                            hitLimit := or(exceedsUp, exceedsDown)
                        }

                        // the change in fees per liquidity for this step of the iteration
                        uint256 stepFeesPerLiquidity;

                        if (hitLimit) {
                            (uint256 sqrtRatioLower, uint256 sqrtRatioUpper) =
                                sortAndConvertToFixedSqrtRatios(limitedNextSqrtRatio, sqrtRatio);
                            (uint128 limitSpecifiedAmountDelta, uint128 limitCalculatedAmountDelta) = isToken1
                                ? (
                                    amount1DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, !isExactOut),
                                    amount0DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, isExactOut)
                                )
                                : (
                                    amount0DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, !isExactOut),
                                    amount1DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, isExactOut)
                                );

                            if (isExactOut) {
                                uint128 beforeFee = amountBeforeFee(limitCalculatedAmountDelta, config.fee());
                                assembly ("memory-safe") {
                                    calculatedAmount := add(calculatedAmount, beforeFee)
                                    amountRemaining := add(amountRemaining, limitSpecifiedAmountDelta)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(beforeFee, limitCalculatedAmountDelta)),
                                        stepLiquidity
                                    )
                                }
                            } else {
                                uint128 beforeFee = amountBeforeFee(limitSpecifiedAmountDelta, config.fee());
                                assembly ("memory-safe") {
                                    calculatedAmount := sub(calculatedAmount, limitCalculatedAmountDelta)
                                    amountRemaining := sub(amountRemaining, beforeFee)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(beforeFee, limitSpecifiedAmountDelta)),
                                        stepLiquidity
                                    )
                                }
                            }

                            sqrtRatioNext = limitedNextSqrtRatio;
                        } else if (sqrtRatioNextFromAmount != sqrtRatio) {
                            uint128 calculatedAmountWithoutFee = isToken1
                                ? amount0Delta(sqrtRatioNextFromAmount, sqrtRatio, stepLiquidity, isExactOut)
                                : amount1Delta(sqrtRatioNextFromAmount, sqrtRatio, stepLiquidity, isExactOut);

                            if (isExactOut) {
                                uint128 includingFee = amountBeforeFee(calculatedAmountWithoutFee, config.fee());
                                assembly ("memory-safe") {
                                    calculatedAmount := add(calculatedAmount, includingFee)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(includingFee, calculatedAmountWithoutFee)),
                                        stepLiquidity
                                    )
                                }
                            } else {
                                assembly ("memory-safe") {
                                    calculatedAmount := sub(calculatedAmount, calculatedAmountWithoutFee)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(amountRemaining, priceImpactAmount)),
                                        stepLiquidity
                                    )
                                }
                            }

                            amountRemaining = 0;
                            sqrtRatioNext = sqrtRatioNextFromAmount;
                        } else {
                            // for an exact output swap, the price should always move since we have to round away from the current price
                            assert(!isExactOut);

                            // consume the entire input amount as fees since the price did not move
                            assembly ("memory-safe") {
                                stepFeesPerLiquidity := div(shl(128, amountRemaining), stepLiquidity)
                            }
                            amountRemaining = 0;
                            sqrtRatioNext = sqrtRatio;
                        }

                        // only if fees per liquidity was updated in this swap iteration
                        if (stepFeesPerLiquidity != 0) {
                            if (feesAccessed == 0) {
                                // this loads only the input token fees per liquidity
                                inputTokenFeesPerLiquidity = uint256(
                                    CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(increasing))
                                        .load()
                                ) + stepFeesPerLiquidity;
                            } else {
                                inputTokenFeesPerLiquidity += stepFeesPerLiquidity;
                            }

                            feesAccessed = 2;
                        }
                    }

                    if (sqrtRatioNext == nextTickSqrtRatio) {
                        sqrtRatio = sqrtRatioNext;
                        assembly ("memory-safe") {
                            // no overflow danger because nextTick is always inside the valid tick bounds
                            tick := sub(nextTick, iszero(increasing))
                        }

                        if (isInitialized) {
                            bytes32 tickValue = CoreStorageLayout.poolTicksSlot(poolId, nextTick).load();
                            assembly ("memory-safe") {
                                // if increasing, we add the liquidity delta, otherwise we subtract it
                                let liquidityDelta :=
                                    mul(signextend(15, tickValue), sub(increasing, iszero(increasing)))
                                liquidity := add(liquidity, liquidityDelta)
                            }

                            (StorageSlot tickFplFirstSlot, StorageSlot tickFplSecondSlot) =
                                CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, nextTick);

                            if (feesAccessed == 0) {
                                inputTokenFeesPerLiquidity = uint256(
                                    CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(increasing))
                                        .load()
                                );
                                feesAccessed = 1;
                            }

                            uint256 globalFeesPerLiquidityOther = uint256(
                                CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(!increasing))
                                    .load()
                            );

                            // if increasing, it means the pool is receiving token1 so the input fees per liquidity is token1
                            if (increasing) {
                                tickFplFirstSlot.store(
                                    bytes32(globalFeesPerLiquidityOther - uint256(tickFplFirstSlot.load()))
                                );
                                tickFplSecondSlot.store(
                                    bytes32(inputTokenFeesPerLiquidity - uint256(tickFplSecondSlot.load()))
                                );
                            } else {
                                tickFplFirstSlot.store(
                                    bytes32(inputTokenFeesPerLiquidity - uint256(tickFplFirstSlot.load()))
                                );
                                tickFplSecondSlot.store(
                                    bytes32(globalFeesPerLiquidityOther - uint256(tickFplSecondSlot.load()))
                                );
                            }
                        }
                    } else if (sqrtRatio != sqrtRatioNext) {
                        sqrtRatio = sqrtRatioNext;
                        tick = sqrtRatioToTick(sqrtRatio);
                    }

                    if (amountRemaining == 0 || sqrtRatio == sqrtRatioLimit) {
                        break;
                    }
                }

                int128 calculatedAmountDelta =
                    SafeCastLib.toInt128(FixedPointMathLib.max(type(int128).min, calculatedAmount));

                int128 specifiedAmountDelta;
                int128 specifiedAmount = params.amount();
                assembly ("memory-safe") {
                    specifiedAmountDelta := sub(specifiedAmount, amountRemaining)
                }

                balanceUpdate = isToken1
                    ? createPoolBalanceUpdate(calculatedAmountDelta, specifiedAmountDelta)
                    : createPoolBalanceUpdate(specifiedAmountDelta, calculatedAmountDelta);

                stateAfter = createPoolState({_sqrtRatio: sqrtRatio, _tick: tick, _liquidity: liquidity});

                writePoolState(poolId, stateAfter);

                if (feesAccessed == 2) {
                    // this stores only the input token fees per liquidity
                    CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(increasing))
                        .store(bytes32(inputTokenFeesPerLiquidity));
                }

                _updatePairDebtWithNative(locker.id(), token0, token1, balanceUpdate.delta0(), balanceUpdate.delta1());

                assembly ("memory-safe") {
                    let o := mload(0x40)
                    mstore(o, shl(96, locker))
                    mstore(add(o, 20), poolId)
                    mstore(add(o, 52), balanceUpdate)
                    mstore(add(o, 84), stateAfter)
                    log0(o, 116)
                }
            }

            IExtension(config.extension()).maybeCallAfterSwap(locker, poolKey, params, balanceUpdate, stateAfter);

            assembly ("memory-safe") {
                mstore(0, balanceUpdate)
                mstore(32, stateAfter)
                return(0, 64)
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {Router} from "./Router.sol";
import {ICore, PoolKey} from "./interfaces/ICore.sol";
import {PoolState} from "./types/poolState.sol";
import {PoolBalanceUpdate} from "./types/poolBalanceUpdate.sol";
import {SwapParameters} from "./types/swapParameters.sol";
import {FlashAccountantLib} from "./libraries/FlashAccountantLib.sol";
import {CoreLib} from "./libraries/CoreLib.sol";

/// @title Ekubo MEV Capture Router
/// @author Moody Salem <moody@ekubo.org>
/// @notice Enables swapping and quoting against pools in Ekubo Protocol including the MEV capture extension pools
contract MEVCaptureRouter is Router {
    using FlashAccountantLib for *;
    using CoreLib for *;

    address public immutable MEV_CAPTURE;

    constructor(ICore core, address _mevCapture) Router(core) {
        MEV_CAPTURE = _mevCapture;
    }

    function _swap(uint256 value, PoolKey memory poolKey, SwapParameters params)
        internal
        override
        returns (PoolBalanceUpdate balanceUpdate, PoolState stateAfter)
    {
        if (poolKey.config.extension() != MEV_CAPTURE) {
            (balanceUpdate, stateAfter) = CORE.swap(value, poolKey, params.withDefaultSqrtRatioLimit());
        } else {
            (balanceUpdate, stateAfter) = abi.decode(
                CORE.forward(MEV_CAPTURE, abi.encode(poolKey, params.withDefaultSqrtRatioLimit())),
                (PoolBalanceUpdate, PoolState)
            );
            if (value != 0) {
                SafeTransferLib.safeTransferETH(address(CORE), value);
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity ^0.8.30;

import {ICore, PoolKey, PositionId, CallPoints} from "../interfaces/ICore.sol";
import {IMEVCapture} from "../interfaces/extensions/IMEVCapture.sol";
import {IExtension} from "../interfaces/ICore.sol";
import {BaseExtension} from "../base/BaseExtension.sol";
import {BaseForwardee} from "../base/BaseForwardee.sol";
import {amountBeforeFee, computeFee} from "../math/fee.sol";
import {ExposedStorage} from "../base/ExposedStorage.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ExposedStorageLib} from "../libraries/ExposedStorageLib.sol";
import {CoreStorageLayout} from "../libraries/CoreStorageLayout.sol";
import {PoolState} from "../types/poolState.sol";
import {MEVCapturePoolState, createMEVCapturePoolState} from "../types/mevCapturePoolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {PoolId} from "../types/poolId.sol";
import {Locker} from "../types/locker.sol";
import {StorageSlot} from "../types/storageSlot.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

function mevCaptureCallPoints() pure returns (CallPoints memory) {
    return CallPoints({
        // to store the initial tick
        beforeInitializePool: true,
        afterInitializePool: false,
        // so that we can prevent swaps that are not made via forward
        beforeSwap: true,
        afterSwap: false,
        // in order to accumulate any collected fees
        beforeUpdatePosition: true,
        afterUpdatePosition: false,
        // in order to accumulate any collected fees
        beforeCollectFees: true,
        afterCollectFees: false
    });
}

/// @notice Charges additional fees based on the relative size of the priority fee
contract MEVCapture is IMEVCapture, BaseExtension, BaseForwardee, ExposedStorage {
    using CoreLib for *;
    using ExposedStorageLib for *;

    constructor(ICore core) BaseExtension(core) BaseForwardee(core) {}

    function getPoolState(PoolId poolId) private view returns (MEVCapturePoolState state) {
        assembly ("memory-safe") {
            state := sload(poolId)
        }
    }

    function setPoolState(PoolId poolId, MEVCapturePoolState state) private {
        assembly ("memory-safe") {
            sstore(poolId, state)
        }
    }

    function getCallPoints() internal pure override returns (CallPoints memory) {
        return mevCaptureCallPoints();
    }

    function beforeInitializePool(address, PoolKey memory poolKey, int32 tick)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (poolKey.config.isStableswap()) {
            revert ConcentratedLiquidityPoolsOnly();
        }
        if (poolKey.config.fee() == 0) {
            // nothing to multiply == no-op extension
            revert NonzeroFeesOnly();
        }

        setPoolState({
            poolId: poolKey.toPoolId(),
            state: createMEVCapturePoolState({_lastUpdateTime: uint32(block.timestamp), _tickLast: tick})
        });
    }

    /// @notice We only allow swapping via forward to this extension
    function beforeSwap(Locker, PoolKey memory, SwapParameters) external pure override(BaseExtension, IExtension) {
        revert SwapMustHappenThroughForward();
    }

    // Allows users to collect pending fees before the first swap in the block happens
    function beforeCollectFees(Locker, PoolKey memory poolKey, PositionId)
        external
        override(BaseExtension, IExtension)
    {
        accumulatePoolFees(poolKey);
    }

    /// Prevents new liquidity from collecting on fees
    function beforeUpdatePosition(Locker, PoolKey memory poolKey, PositionId, int128)
        external
        override(BaseExtension, IExtension)
    {
        accumulatePoolFees(poolKey);
    }

    /// @inheritdoc IMEVCapture
    function accumulatePoolFees(PoolKey memory poolKey) public {
        PoolId poolId = poolKey.toPoolId();
        MEVCapturePoolState state = getPoolState(poolId);

        // the only thing we lock for is accumulating fees when the pool has not been updated in this block
        if (state.lastUpdateTime() != uint32(block.timestamp)) {
            address target = address(CORE);
            assembly ("memory-safe") {
                let o := mload(0x40)
                mstore(o, shl(224, 0xf83d08ba))
                mcopy(add(o, 4), poolKey, 96)
                mstore(add(o, 100), poolId)

                // If the call failed, pass through the revert
                if iszero(call(gas(), target, 0, o, 132, 0, 0)) {
                    returndatacopy(o, 0, returndatasize())
                    revert(o, returndatasize())
                }
            }
        }
    }

    function locked_6416899205(uint256) external onlyCore {
        PoolKey memory poolKey;
        PoolId poolId;
        assembly ("memory-safe") {
            // copy the poolkey out of calldata
            calldatacopy(poolKey, 36, 96)
            poolId := calldataload(132)
        }

        (int32 tick, uint128 fees0, uint128 fees1) = loadCoreState(poolId, poolKey.token0, poolKey.token1);

        if (fees0 != 0 || fees1 != 0) {
            CORE.accumulateAsFees(poolKey, fees0, fees1);
            unchecked {
                CORE.updateSavedBalances(
                    poolKey.token0,
                    poolKey.token1,
                    PoolId.unwrap(poolId),
                    -int256(uint256(fees0)),
                    -int256(uint256(fees1))
                );
            }
        }

        setPoolState({
            poolId: poolId,
            state: createMEVCapturePoolState({_lastUpdateTime: uint32(block.timestamp), _tickLast: tick})
        });
    }

    function loadCoreState(PoolId poolId, address token0, address token1)
        private
        view
        returns (int32 tick, uint128 fees0, uint128 fees1)
    {
        StorageSlot stateSlot = CoreStorageLayout.poolStateSlot(poolId);
        StorageSlot feesSlot = CoreStorageLayout.savedBalancesSlot(address(this), token0, token1, PoolId.unwrap(poolId));

        (bytes32 v0, bytes32 v1) = CORE.sload(stateSlot, feesSlot);
        tick = PoolState.wrap(v0).tick();

        assembly ("memory-safe") {
            fees0 := shr(128, v1)
            fees0 := sub(fees0, gt(fees0, 0))

            fees1 := shr(128, shl(128, v1))
            fees1 := sub(fees1, gt(fees1, 0))
        }
    }

    function handleForwardData(Locker, bytes memory data) internal override returns (bytes memory result) {
        unchecked {
            (PoolKey memory poolKey, SwapParameters params) = abi.decode(data, (PoolKey, SwapParameters));

            PoolId poolId = poolKey.toPoolId();
            MEVCapturePoolState state = getPoolState(poolId);
            uint32 lastUpdateTime = state.lastUpdateTime();
            int32 tickLast = state.tickLast();

            uint32 currentTime = uint32(block.timestamp);

            int256 saveDelta0;
            int256 saveDelta1;

            if (lastUpdateTime != currentTime) {
                (int32 tick, uint128 fees0, uint128 fees1) =
                    loadCoreState({poolId: poolId, token0: poolKey.token0, token1: poolKey.token1});

                if (fees0 != 0 || fees1 != 0) {
                    CORE.accumulateAsFees(poolKey, fees0, fees1);
                    // never overflows int256 container
                    saveDelta0 -= int256(uint256(fees0));
                    saveDelta1 -= int256(uint256(fees1));
                }

                tickLast = tick;
                setPoolState({
                    poolId: poolId,
                    state: createMEVCapturePoolState({_lastUpdateTime: currentTime, _tickLast: tickLast})
                });
            }

            (PoolBalanceUpdate balanceUpdate, PoolState stateAfter) = CORE.swap(0, poolKey, params);

            // however many tick spacings were crossed is the fee multiplier
            uint256 feeMultiplierX64 =
                (FixedPointMathLib.abs(stateAfter.tick() - tickLast) << 64) / poolKey.config.concentratedTickSpacing();
            uint64 poolFee = poolKey.config.fee();
            uint64 additionalFee = uint64(FixedPointMathLib.min(type(uint64).max, (feeMultiplierX64 * poolFee) >> 64));

            if (additionalFee != 0) {
                if (params.isExactOut()) {
                    // take an additional fee from the calculated input amount equal to the `additionalFee - poolFee`
                    if (balanceUpdate.delta0() > 0) {
                        uint128 inputAmount = uint128(uint256(int256(balanceUpdate.delta0())));
                        // first remove the fee to get the original input amount before we compute the additional fee
                        inputAmount -= computeFee(inputAmount, poolFee);
                        int128 fee = SafeCastLib.toInt128(amountBeforeFee(inputAmount, additionalFee) - inputAmount);

                        saveDelta0 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0() + fee, balanceUpdate.delta1());
                    } else if (balanceUpdate.delta1() > 0) {
                        uint128 inputAmount = uint128(uint256(int256(balanceUpdate.delta1())));
                        // first remove the fee to get the original input amount before we compute the additional fee
                        inputAmount -= computeFee(inputAmount, poolFee);
                        int128 fee = SafeCastLib.toInt128(amountBeforeFee(inputAmount, additionalFee) - inputAmount);

                        saveDelta1 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0(), balanceUpdate.delta1() + fee);
                    }
                } else {
                    if (balanceUpdate.delta0() < 0) {
                        uint128 outputAmount = uint128(uint256(-int256(balanceUpdate.delta0())));
                        int128 fee = SafeCastLib.toInt128(computeFee(outputAmount, additionalFee));

                        saveDelta0 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0() + fee, balanceUpdate.delta1());
                    } else if (balanceUpdate.delta1() < 0) {
                        uint128 outputAmount = uint128(uint256(-int256(balanceUpdate.delta1())));
                        int128 fee = SafeCastLib.toInt128(computeFee(outputAmount, additionalFee));

                        saveDelta1 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0(), balanceUpdate.delta1() + fee);
                    }
                }
            }

            if (saveDelta0 != 0 || saveDelta1 != 0) {
                CORE.updateSavedBalances(poolKey.token0, poolKey.token1, PoolId.unwrap(poolId), saveDelta0, saveDelta1);
            }

            result = abi.encode(balanceUpdate, stateAfter);
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {BaseLocker} from "./base/BaseLocker.sol";
import {UsesCore} from "./base/UsesCore.sol";
import {ICore} from "./interfaces/ICore.sol";
import {IOrders} from "./interfaces/IOrders.sol";
import {PayableMulticallable} from "./base/PayableMulticallable.sol";
import {TWAMMLib} from "./libraries/TWAMMLib.sol";
import {ITWAMM} from "./interfaces/extensions/ITWAMM.sol";
import {OrderKey} from "./types/orderKey.sol";
import {computeSaleRate} from "./math/twamm.sol";
import {BaseNonfungibleToken} from "./base/BaseNonfungibleToken.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";
import {FlashAccountantLib} from "./libraries/FlashAccountantLib.sol";

/// @title Ekubo Protocol Orders
/// @author Moody Salem <moody@ekubo.org>
/// @notice Tracks TWAMM (Time-Weighted Average Market Maker) orders in Ekubo Protocol as NFTs
/// @dev Manages long-term orders that execute over time through the TWAMM extension
contract Orders is IOrders, UsesCore, PayableMulticallable, BaseLocker, BaseNonfungibleToken {
    using TWAMMLib for *;
    using FlashAccountantLib for *;

    uint256 private constant CALL_TYPE_CHANGE_SALE_RATE = 0;
    uint256 private constant CALL_TYPE_COLLECT_PROCEEDS = 1;

    /// @notice The TWAMM extension contract that handles order execution
    ITWAMM public immutable TWAMM_EXTENSION;

    /// @notice Constructs the Orders contract
    /// @param core The core contract instance
    /// @param _twamm The TWAMM extension contract
    /// @param owner The owner of the contract (for access control)
    constructor(ICore core, ITWAMM _twamm, address owner) BaseNonfungibleToken(owner) BaseLocker(core) UsesCore(core) {
        TWAMM_EXTENSION = _twamm;
    }

    /// @inheritdoc IOrders
    function mintAndIncreaseSellAmount(OrderKey memory orderKey, uint112 amount, uint112 maxSaleRate)
        public
        payable
        returns (uint256 id, uint112 saleRate)
    {
        id = mint();
        saleRate = increaseSellAmount(id, orderKey, amount, maxSaleRate);
    }

    /// @inheritdoc IOrders
    function increaseSellAmount(uint256 id, OrderKey memory orderKey, uint128 amount, uint112 maxSaleRate)
        public
        payable
        authorizedForNft(id)
        returns (uint112 saleRate)
    {
        uint256 realStart = FixedPointMathLib.max(block.timestamp, orderKey.config.startTime());

        unchecked {
            if (orderKey.config.endTime() <= realStart) {
                revert OrderAlreadyEnded();
            }

            saleRate = uint112(computeSaleRate(amount, uint32(orderKey.config.endTime() - realStart)));

            if (saleRate > maxSaleRate) {
                revert MaxSaleRateExceeded();
            }
        }

        lock(abi.encode(CALL_TYPE_CHANGE_SALE_RATE, msg.sender, id, orderKey, saleRate));
    }

    /// @inheritdoc IOrders
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease, address recipient)
        public
        payable
        authorizedForNft(id)
        returns (uint112 refund)
    {
        refund = uint112(
            uint256(
                -abi.decode(
                    lock(
                        abi.encode(
                            CALL_TYPE_CHANGE_SALE_RATE, recipient, id, orderKey, -int256(uint256(saleRateDecrease))
                        )
                    ),
                    (int256)
                )
            )
        );
    }

    /// @inheritdoc IOrders
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease)
        external
        payable
        returns (uint112 refund)
    {
        refund = decreaseSaleRate(id, orderKey, saleRateDecrease, msg.sender);
    }

    /// @inheritdoc IOrders
    function collectProceeds(uint256 id, OrderKey memory orderKey, address recipient)
        public
        payable
        authorizedForNft(id)
        returns (uint128 proceeds)
    {
        proceeds = abi.decode(lock(abi.encode(CALL_TYPE_COLLECT_PROCEEDS, id, orderKey, recipient)), (uint128));
    }

    /// @inheritdoc IOrders
    function collectProceeds(uint256 id, OrderKey memory orderKey) external payable returns (uint128 proceeds) {
        proceeds = collectProceeds(id, orderKey, msg.sender);
    }

    /// @inheritdoc IOrders
    function executeVirtualOrdersAndGetCurrentOrderInfo(uint256 id, OrderKey memory orderKey)
        external
        returns (uint112 saleRate, uint256 amountSold, uint256 remainingSellAmount, uint128 purchasedAmount)
    {
        (saleRate, amountSold, remainingSellAmount, purchasedAmount) =
            TWAMM_EXTENSION.executeVirtualOrdersAndGetCurrentOrderInfo(address(this), bytes32(id), orderKey);
    }

    /// @notice Handles lock callback data for order operations
    /// @dev Internal function that processes different types of order operations
    /// @param data Encoded operation data
    /// @return result Encoded result data
    function handleLockData(uint256, bytes memory data) internal override returns (bytes memory result) {
        uint256 callType = abi.decode(data, (uint256));

        if (callType == CALL_TYPE_CHANGE_SALE_RATE) {
            (, address recipientOrPayer, uint256 id, OrderKey memory orderKey, int256 saleRateDelta) =
                abi.decode(data, (uint256, address, uint256, OrderKey, int256));

            int256 amount =
                CORE.updateSaleRate(TWAMM_EXTENSION, bytes32(id), orderKey, SafeCastLib.toInt112(saleRateDelta));

            if (amount != 0) {
                address sellToken = orderKey.sellToken();
                if (saleRateDelta > 0) {
                    if (sellToken == NATIVE_TOKEN_ADDRESS) {
                        SafeTransferLib.safeTransferETH(address(ACCOUNTANT), uint256(amount));
                    } else {
                        ACCOUNTANT.payFrom(recipientOrPayer, sellToken, uint256(amount));
                    }
                } else {
                    unchecked {
                        // we know amount will never exceed the uint128 type because of limitations on sale rate (fixed point 80.32) and duration (uint32)
                        ACCOUNTANT.withdraw(sellToken, recipientOrPayer, uint128(uint256(-amount)));
                    }
                }
            }

            result = abi.encode(amount);
        } else if (callType == CALL_TYPE_COLLECT_PROCEEDS) {
            (, uint256 id, OrderKey memory orderKey, address recipient) =
                abi.decode(data, (uint256, uint256, OrderKey, address));

            uint128 proceeds = CORE.collectProceeds(TWAMM_EXTENSION, bytes32(id), orderKey);

            if (proceeds != 0) {
                ACCOUNTANT.withdraw(orderKey.buyToken(), recipient, proceeds);
            }

            result = abi.encode(proceeds);
        } else {
            revert();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ICore} from "../interfaces/ICore.sol";
import {CoreStorageLayout} from "./CoreStorageLayout.sol";
import {ExposedStorageLib} from "./ExposedStorageLib.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {Position} from "../types/position.sol";
import {PoolState} from "../types/poolState.sol";
import {PositionId} from "../types/positionId.sol";
import {PoolId} from "../types/poolId.sol";
import {PoolKey} from "../types/poolKey.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {SwapParameters, createSwapParameters} from "../types/swapParameters.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";
import {StorageSlot} from "../types/storageSlot.sol";

/// @title Core Library
/// @notice Library providing common storage getters for external contracts
/// @dev These functions access Core contract storage directly for gas efficiency
library CoreLib {
    using ExposedStorageLib for *;

    /// @notice Checks if an extension is registered with the core contract
    /// @param core The core contract instance
    /// @param extension The extension address to check
    /// @return registered True if the extension is registered
    function isExtensionRegistered(ICore core, address extension) internal view returns (bool registered) {
        registered = uint256(core.sload(CoreStorageLayout.isExtensionRegisteredSlot(extension))) != 0;
    }

    /// @notice Gets the current state of a pool
    /// @param core The core contract instance
    /// @param poolId The unique identifier for the pool
    /// @return state The current state of the pool
    function poolState(ICore core, PoolId poolId) internal view returns (PoolState state) {
        state = PoolState.wrap(core.sload(CoreStorageLayout.poolStateSlot(poolId)));
    }

    /// @notice Gets the current global fees per liquidity for a pool
    /// @param core The core contract instance
    /// @param poolId The unique identifier for the pool
    /// @return feesPerLiquidity The current global fees per liquidtiy of the pool
    function getPoolFeesPerLiquidity(ICore core, PoolId poolId)
        internal
        view
        returns (FeesPerLiquidity memory feesPerLiquidity)
    {
        StorageSlot fplFirstSlot = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
        (bytes32 value0, bytes32 value1) = core.sload(fplFirstSlot, fplFirstSlot.next());

        feesPerLiquidity.value0 = uint256(value0);
        feesPerLiquidity.value1 = uint256(value1);
    }

    /// @notice Gets position data for a specific position in a pool
    /// @param core The core contract instance
    /// @param poolId The unique identifier for the pool
    /// @param positionId The unique identifier for the position
    /// @return position The position data including liquidity, extraData, and fees
    function poolPositions(ICore core, PoolId poolId, address owner, PositionId positionId)
        internal
        view
        returns (Position memory position)
    {
        StorageSlot firstSlot = CoreStorageLayout.poolPositionsSlot(poolId, owner, positionId);
        (bytes32 v0, bytes32 v1, bytes32 v2) = core.sload(firstSlot, firstSlot.add(1), firstSlot.add(2));

        assembly ("memory-safe") {
            mstore(position, shl(128, v0))
            mstore(add(position, 0x20), shr(128, v0))
        }

        position.feesPerLiquidityInsideLast = FeesPerLiquidity(uint256(v1), uint256(v2));
    }

    /// @notice Gets saved balances for a specific owner and token pair
    /// @param core The core contract instance
    /// @param owner The owner of the saved balances
    /// @param token0 The first token address
    /// @param token1 The second token address
    /// @param salt The salt used for the saved balance key
    /// @return savedBalance0 The saved balance of token0
    /// @return savedBalance1 The saved balance of token1
    function savedBalances(ICore core, address owner, address token0, address token1, bytes32 salt)
        internal
        view
        returns (uint128 savedBalance0, uint128 savedBalance1)
    {
        uint256 value = uint256(core.sload(CoreStorageLayout.savedBalancesSlot(owner, token0, token1, salt)));

        savedBalance0 = uint128(value >> 128);
        savedBalance1 = uint128(value);
    }

    /// @notice Gets tick information for a specific tick in a pool
    /// @param core The core contract instance
    /// @param poolId The unique identifier for the pool
    /// @param tick The tick to query
    /// @return liquidityDelta The liquidity change when crossing this tick
    /// @return liquidityNet The net liquidity above this tick
    function poolTicks(ICore core, PoolId poolId, int32 tick)
        internal
        view
        returns (int128 liquidityDelta, uint128 liquidityNet)
    {
        bytes32 data = core.sload(CoreStorageLayout.poolTicksSlot(poolId, tick));

        // takes only least significant 128 bits
        liquidityDelta = int128(uint128(uint256(data)));
        // takes only most significant 128 bits
        liquidityNet = uint128(bytes16(data));
    }

    /// @notice Executes a swap against the core contract using assembly optimization
    /// @dev Uses assembly to make direct call to core contract for gas efficiency
    /// @param core The core contract instance
    /// @param value Native token value to send with the swap
    /// @param poolKey Pool key identifying the pool
    /// @param params The swap parameters to use
    /// @return balanceUpdate Change to the pool balances that resulted from the swap
    /// @return stateAfter The pool state after the swap
    function swap(ICore core, uint256 value, PoolKey memory poolKey, SwapParameters params)
        internal
        returns (PoolBalanceUpdate balanceUpdate, PoolState stateAfter)
    {
        assembly ("memory-safe") {
            let free := mload(0x40)

            // the function selector of swap is 0
            mstore(free, 0)

            // Copy PoolKey
            mcopy(add(free, 4), poolKey, 96)

            // Add SwapParameters
            mstore(add(free, 100), params)

            if iszero(call(gas(), core, value, free, 132, free, 64)) {
                returndatacopy(free, 0, returndatasize())
                revert(free, returndatasize())
            }

            // Extract return values - balanceUpdate is packed (delta1 << 128 | delta0)
            balanceUpdate := mload(free)
            stateAfter := mload(add(free, 32))
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ICore} from "../interfaces/ICore.sol";

/// @title Uses Core
/// @notice Abstract base contract for contracts that need to interact with the Ekubo Protocol core
/// @dev Provides core contract reference and access control functionality
abstract contract UsesCore {
    /// @notice Thrown when a function restricted to the core contract is called by another address
    error CoreOnly();

    /// @notice The core contract instance that this contract interacts with
    ICore internal immutable CORE;

    /// @notice Constructs the UsesCore contract with a core contract reference
    /// @param _core The core contract instance to use
    constructor(ICore _core) {
        CORE = _core;
    }

    /// @notice Restricts function access to only the core contract
    /// @dev Reverts with CoreOnly if called by any address other than the core contract
    modifier onlyCore() {
        if (msg.sender != address(CORE)) revert CoreOnly();
        _;
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {DropKey} from "../types/dropKey.sol";
import {ClaimKey} from "../types/claimKey.sol";
import {IExposedStorage} from "./IExposedStorage.sol";

/// @title Incentives Interface
/// @notice Interface for the Incentives contract that manages airdrops
/// @dev Inherits from IExposedStorage to allow direct storage access
interface IIncentives is IExposedStorage {
    /// @notice Emitted when a drop is funded
    /// @param key The drop key that was funded
    /// @param amountNext The new total funded amount
    event Funded(DropKey key, uint128 amountNext);

    /// @notice Emitted when a drop is refunded
    /// @param key The drop key that was refunded
    /// @param refundAmount The amount that was refunded
    event Refunded(DropKey key, uint128 refundAmount);

    /// @notice Thrown if the claim has already happened for this drop
    error AlreadyClaimed();

    /// @notice Thrown if the merkle proof does not correspond to the root
    error InvalidProof();

    /// @notice Thrown if the drop is not sufficiently funded for the claim
    error InsufficientFunds();

    /// @notice Only the drop owner may call this function
    error DropOwnerOnly();

    /// @notice Funds a drop to a minimum amount
    /// @param key The drop key to fund
    /// @param minimum The minimum amount to fund to
    /// @return fundedAmount The amount that was actually funded
    function fund(DropKey memory key, uint128 minimum) external returns (uint128 fundedAmount);

    /// @notice Refunds the remaining amount from a drop to the owner
    /// @param key The drop key to refund
    /// @return refundAmount The amount that was refunded
    function refund(DropKey memory key) external returns (uint128 refundAmount);

    /// @notice Claims tokens from a drop using a merkle proof
    /// @param key The drop key to claim from
    /// @param c The claim details
    /// @param proof The merkle proof for the claim
    function claim(DropKey memory key, ClaimKey memory c, bytes32[] calldata proof) external;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ICore, IExtension} from "../interfaces/ICore.sol";
import {CallPoints} from "../types/callPoints.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {PoolState} from "../types/poolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {UsesCore} from "./UsesCore.sol";
import {Locker} from "../types/locker.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

/// @title Base Extension
/// @notice Abstract base contract for creating extensions to the Ekubo Protocol
/// @dev Extensions can hook into various pool operations to add custom functionality
///      Derived contracts must implement getCallPoints() and the specific hook functions they want to use
abstract contract BaseExtension is IExtension, UsesCore {
    /// @notice Thrown when a call point is not implemented by the extension
    error CallPointNotImplemented();

    /// @notice Constructs the BaseExtension and optionally registers it with the core
    /// @param core The core contract instance
    constructor(ICore core) UsesCore(core) {
        if (_registerInConstructor()) core.registerExtension(getCallPoints());
    }

    /// @notice Determines whether the extension should register itself in the constructor
    /// @dev Can be overridden by derived contracts to control registration timing
    /// @return True if the extension should register in the constructor
    function _registerInConstructor() internal pure virtual returns (bool) {
        return true;
    }

    /// @notice Returns the call points configuration for this extension
    /// @dev Must be implemented by derived contracts to specify which hooks they use
    /// @return The call points configuration
    function getCallPoints() internal virtual returns (CallPoints memory);

    /// @inheritdoc IExtension
    function beforeInitializePool(address, PoolKey calldata, int32) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterInitializePool(address, PoolKey calldata, int32, SqrtRatio) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function beforeUpdatePosition(Locker, PoolKey memory, PositionId, int128) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterUpdatePosition(Locker, PoolKey memory, PositionId, int128, PoolBalanceUpdate, PoolState)
        external
        virtual
    {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function beforeSwap(Locker, PoolKey memory, SwapParameters) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterSwap(Locker, PoolKey memory, SwapParameters, PoolBalanceUpdate, PoolState) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function beforeCollectFees(Locker, PoolKey memory, PositionId) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterCollectFees(Locker, PoolKey memory, PositionId, uint128, uint128) external virtual {
        revert CallPointNotImplemented();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ERC721} from "solady/tokens/ERC721.sol";
import {LibString} from "solady/utils/LibString.sol";
import {Ownable} from "solady/auth/Ownable.sol";

import {IBaseNonfungibleToken} from "../interfaces/IBaseNonfungibleToken.sol";

/// @title Base Nonfungible Token
/// @notice Abstract NFT contract where tokens can be minted and burned freely, and the owner can change the metadata
/// @dev This contract provides a base implementation for NFTs with deterministic ID generation based on minter and salt.
///      Token IDs are generated using keccak256(minter, salt, chainid, contract_address) to ensure uniqueness.
///      The contract allows free minting and burning, with metadata management restricted to the owner.
abstract contract BaseNonfungibleToken is IBaseNonfungibleToken, Ownable, ERC721 {
    /// @notice Thrown when a caller is not authorized to perform an action on a specific token
    /// @param caller The address that attempted the unauthorized action
    /// @param id The token ID for which authorization was required
    error NotUnauthorizedForToken(address caller, uint256 id);

    /// @dev The name of the NFT collection
    string private _name;

    /// @dev The symbol of the NFT collection
    string private _symbol;

    /// @notice The base URL used for constructing token URIs
    /// @dev Token URIs are constructed by concatenating this base URL with the token ID
    string public baseUrl;

    /// @notice Initializes the contract with the specified owner
    /// @param owner The address that will be set as the owner of the contract
    constructor(address owner) {
        _initializeOwner(owner);
    }

    /// @notice Updates the metadata for the NFT collection
    /// @dev Only the contract owner can call this function
    /// @param newName The new name for the NFT collection
    /// @param newSymbol The new symbol for the NFT collection
    /// @param newBaseUrl The new base URL for token metadata
    function setMetadata(string memory newName, string memory newSymbol, string memory newBaseUrl) external onlyOwner {
        _name = newName;
        _symbol = newSymbol;
        baseUrl = newBaseUrl;
    }

    /// @notice Returns the name of the NFT collection
    /// @return The name of the collection
    function name() public view override returns (string memory) {
        return _name;
    }

    /// @notice Returns the symbol of the NFT collection
    /// @return The symbol of the collection
    function symbol() public view override returns (string memory) {
        return _symbol;
    }

    /// @notice Returns the URI for a given token ID
    /// @dev Constructs the URI by concatenating the base URL with the token ID.
    ///      If baseUrl is empty, returns just the stringified token ID.
    /// @param id The token ID to get the URI for
    /// @return The complete URI for the token
    function tokenURI(uint256 id) public view virtual override returns (string memory) {
        return string(
            abi.encodePacked(
                baseUrl,
                LibString.toString(block.chainid),
                "/",
                LibString.toHexStringChecksummed(address(this)),
                "/",
                LibString.toString(id)
            )
        );
    }

    /// @notice Modifier to ensure the caller is authorized to perform actions on a specific token
    /// @dev Checks if the caller is the owner or approved for the token
    /// @param id The token ID to check authorization for
    modifier authorizedForNft(uint256 id) {
        if (!_isApprovedOrOwner(msg.sender, id)) {
            revert NotUnauthorizedForToken(msg.sender, id);
        }
        _;
    }

    /// @inheritdoc IBaseNonfungibleToken
    /// @dev Uses keccak256 hash of minter, salt, chain ID, and contract address to generate unique IDs.
    ///      IDs are deterministic per (minter, salt, chainId, contract) tuple; the same pair on a
    ///      different chain or contract yields a different ID.
    function saltToId(address minter, bytes32 salt) public view returns (uint256 result) {
        assembly ("memory-safe") {
            let free := mload(0x40)
            mstore(free, minter)
            mstore(add(free, 32), salt)
            mstore(add(free, 64), chainid())
            mstore(add(free, 96), address())

            result := keccak256(free, 128)
        }
    }

    /// @inheritdoc IBaseNonfungibleToken
    /// @dev Generates a salt using prevrandao() and gas() for pseudorandomness.
    ///      Note: This can encounter conflicts if a sender sends two identical transactions
    ///      in the same block that consume exactly the same amount of gas.
    ///      No fees are collected; any msg.value sent is ignored.
    function mint() public payable returns (uint256 id) {
        bytes32 salt;
        assembly ("memory-safe") {
            mstore(0, prevrandao())
            mstore(32, gas())
            salt := keccak256(0, 64)
        }
        id = mint(salt);
    }

    /// @inheritdoc IBaseNonfungibleToken
    /// @dev The token ID is generated using saltToId(msg.sender, salt). This prevents the need
    ///      to store a counter of how many tokens were minted, as IDs are deterministic.
    ///      No fees are collected; any msg.value sent is ignored.
    function mint(bytes32 salt) public payable returns (uint256 id) {
        id = saltToId(msg.sender, salt);
        _mint(msg.sender, id);
    }

    /// @inheritdoc IBaseNonfungibleToken
    /// @dev Can be used to refund some gas after the NFT is no longer needed.
    ///      The same ID can be recreated by the original minter by reusing the salt.
    ///      Only the token owner or approved addresses can burn the token.
    ///      No fees are collected; any msg.value sent is ignored.
    function burn(uint256 id) external payable authorizedForNft(id) {
        _burn(id);
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IFlashAccountant} from "../interfaces/IFlashAccountant.sol";

/// @title Flash Accountant Library
/// @notice Provides helper functions for interacting with the Flash Accountant
/// @dev Contains optimized assembly implementations for token payments to the accountant
library FlashAccountantLib {
    /// @notice Pays tokens directly to the flash accountant
    /// @dev Uses assembly for gas optimization and handles the payment flow with start/complete calls
    /// @param accountant The flash accountant contract to pay
    /// @param token The token address to pay
    /// @param amount The amount of tokens to pay
    function pay(IFlashAccountant accountant, address token, uint256 amount) internal {
        assembly ("memory-safe") {
            mstore(0x00, 0xf9b6a796)
            mstore(0x20, token)

            // accountant.startPayments()
            // this is expected to never revert
            pop(call(gas(), accountant, 0, 0x1c, 36, 0x00, 0x00))

            // token#transfer
            mstore(0x14, accountant) // Store the `to` argument.
            mstore(0x34, amount) // Store the `amount` argument.
            mstore(0x00, 0xa9059cbb000000000000000000000000) // `transfer(address,uint256)`.
            // Perform the transfer, reverting upon failure.
            let success := call(gas(), token, 0, 0x10, 0x44, 0x00, 0x20)
            if iszero(and(eq(mload(0x00), 1), success)) {
                if iszero(lt(or(iszero(extcodesize(token)), returndatasize()), success)) {
                    mstore(0x00, 0x90b8ec18) // `TransferFailed()`.
                    revert(0x1c, 0x04)
                }
            }
            mstore(0x34, 0) // Restore the part of the free memory pointer that was overwritten.

            // accountant.completePayments()
            mstore(0x00, 0x12e103f1)
            mstore(0x20, token)
            // we ignore the potential reverts in this case because it will almost always result in nonzero debt when the lock returns
            pop(call(gas(), accountant, 0, 0x1c, 36, 0x00, 0x00))
        }
    }

    /// @notice Pays tokens from a specific address to the flash accountant
    /// @dev Uses assembly for gas optimization and handles transferFrom with start/complete payment calls
    /// @param accountant The flash accountant contract to pay
    /// @param from The address to transfer tokens from
    /// @param token The token address to pay
    /// @param amount The amount of tokens to pay
    function payFrom(IFlashAccountant accountant, address from, address token, uint256 amount) internal {
        assembly ("memory-safe") {
            mstore(0, 0xf9b6a796)
            mstore(32, token)

            // accountant.startPayments()
            // this is expected to never revert
            pop(call(gas(), accountant, 0, 0x1c, 36, 0x00, 0x00))

            // token#transferFrom
            let m := mload(0x40)
            mstore(0x60, amount)
            mstore(0x40, accountant)
            mstore(0x2c, shl(96, from))
            mstore(0x0c, 0x23b872dd000000000000000000000000) // `transferFrom(address,address,uint256)`.
            let success := call(gas(), token, 0, 0x1c, 0x64, 0x00, 0x20)
            if iszero(and(eq(mload(0x00), 1), success)) {
                if iszero(lt(or(iszero(extcodesize(token)), returndatasize()), success)) {
                    mstore(0x00, 0x7939f424) // `TransferFromFailed()`.
                    revert(0x1c, 0x04)
                }
            }
            mstore(0x60, 0)
            mstore(0x40, m)

            // accountant.completePayments()
            mstore(0x00, 0x12e103f1)
            mstore(0x20, token)
            // we ignore the potential reverts in this case because it will almost always result in nonzero debt when the lock returns
            pop(call(gas(), accountant, 0, 0x1c, 36, 0x00, 0x00))
        }
    }

    /// @notice Withdraws a single token using the old withdraw interface
    /// @dev Provides backward compatibility for the old withdraw(token, recipient, amount) signature
    /// @param accountant The flash accountant contract instance
    /// @param token The token address to withdraw
    /// @param recipient The address to receive the tokens
    /// @param amount The amount to withdraw
    function withdraw(IFlashAccountant accountant, address token, address recipient, uint128 amount) internal {
        assembly ("memory-safe") {
            let free := mload(0x40)

            // cast sig "withdraw()"
            mstore(free, shl(224, 0x3ccfd60b))

            // Pack: token (20 bytes) + recipient (20 bytes) + amount (16 bytes)
            mstore(add(free, 4), shl(96, token))
            mstore(add(free, 24), shl(96, recipient))
            mstore(add(free, 44), shl(128, amount))

            if iszero(call(gas(), accountant, 0, free, 60, 0, 0)) {
                returndatacopy(free, 0, returndatasize())
                revert(free, returndatasize())
            }
        }
    }

    /// @notice Pays two tokens from a specific address to the flash accountant in a single operation
    /// @dev Uses assembly for gas optimization and handles both tokens in a single startPayments/completePayments cycle
    /// @param accountant The flash accountant contract to pay
    /// @param from The address to transfer tokens from
    /// @param token0 The first token address to pay
    /// @param token1 The second token address to pay
    /// @param amount0 The amount of token0 to pay
    /// @param amount1 The amount of token1 to pay
    function payTwoFrom(
        IFlashAccountant accountant,
        address from,
        address token0,
        address token1,
        uint256 amount0,
        uint256 amount1
    ) internal {
        assembly ("memory-safe") {
            // Save free memory pointer before using 0x40
            let free := mload(0x40)

            // accountant.startPayments() with both tokens
            mstore(0x00, 0xf9b6a796) // startPayments selector
            mstore(0x20, token0) // first token
            mstore(0x40, token1) // second token

            // Call startPayments with both tokens (4 + 32 + 32 = 68 bytes)
            pop(call(gas(), accountant, 0, 0x1c, 68, 0x00, 0x00))

            // Restore free memory pointer
            mstore(0x40, free)

            // Transfer token0 from caller to accountant
            if amount0 {
                let m := mload(0x40)
                mstore(0x60, amount0)
                mstore(0x40, accountant)
                mstore(0x2c, shl(96, from))
                mstore(0x0c, 0x23b872dd000000000000000000000000) // transferFrom selector
                let success := call(gas(), token0, 0, 0x1c, 0x64, 0x00, 0x20)
                if iszero(and(eq(mload(0x00), 1), success)) {
                    if iszero(lt(or(iszero(extcodesize(token0)), returndatasize()), success)) {
                        mstore(0x00, 0x7939f424) // TransferFromFailed()
                        revert(0x1c, 0x04)
                    }
                }
                mstore(0x60, 0)
                mstore(0x40, m)
            }

            // Transfer token1 from caller to accountant
            if amount1 {
                let m := mload(0x40)
                mstore(0x60, amount1)
                mstore(0x40, accountant)
                mstore(0x2c, shl(96, from))
                mstore(0x0c, 0x23b872dd000000000000000000000000) // transferFrom selector
                let success := call(gas(), token1, 0, 0x1c, 0x64, 0x00, 0x20)
                if iszero(and(eq(mload(0x00), 1), success)) {
                    if iszero(lt(or(iszero(extcodesize(token1)), returndatasize()), success)) {
                        mstore(0x00, 0x7939f424) // TransferFromFailed()
                        revert(0x1c, 0x04)
                    }
                }
                mstore(0x60, 0)
                mstore(0x40, m)
            }

            // accountant.completePayments() with both tokens
            let free2 := mload(0x40)
            mstore(0x00, 0x12e103f1) // completePayments selector
            mstore(0x20, token0) // first token
            mstore(0x40, token1) // second token

            // Call completePayments with both tokens (4 + 32 + 32 = 68 bytes)
            pop(call(gas(), accountant, 0, 0x1c, 68, 0x00, 0x00))

            // Restore free memory pointer
            mstore(0x40, free2)
        }
    }

    /// @notice Withdraws two tokens using assembly to call withdraw with packed calldata
    /// @dev Uses assembly and packed calldata for gas efficiency, optimized for positions contract
    /// @param accountant The flash accountant contract instance
    /// @param token0 The first token address to withdraw
    /// @param token1 The second token address to withdraw
    /// @param recipient The address to receive both tokens
    /// @param amount0 The amount of token0 to withdraw
    /// @param amount1 The amount of token1 to withdraw
    function withdrawTwo(
        IFlashAccountant accountant,
        address token0,
        address token1,
        address recipient,
        uint128 amount0,
        uint128 amount1
    ) internal {
        assembly ("memory-safe") {
            let free := mload(0x40)

            // cast sig "withdraw()"
            mstore(free, shl(224, 0x3ccfd60b))

            // Pack first withdrawal: token0 (20 bytes) + recipient (20 bytes) + amount0 (16 bytes)
            mstore(add(free, 4), shl(96, token0))
            mstore(add(free, 24), shl(96, recipient))
            mstore(add(free, 44), shl(128, amount0))

            // Pack second withdrawal: token1 (20 bytes) + recipient (20 bytes) + amount1 (16 bytes)
            mstore(add(free, 60), shl(96, token1))
            mstore(add(free, 80), shl(96, recipient))
            mstore(add(free, 100), shl(128, amount1))

            if iszero(call(gas(), accountant, 0, free, 116, 0, 0)) {
                returndatacopy(free, 0, returndatasize())
                revert(free, returndatasize())
            }
        }
    }

    /// @notice Forwards a call to another contract through the accountant
    /// @dev Used to call other contracts while maintaining the lock context
    /// @param accountant The flash accountant contract to forward through
    /// @param to The address to forward the call to
    /// @param data The call data to forward
    /// @return result The result of the forwarded call
    function forward(IFlashAccountant accountant, address to, bytes memory data)
        internal
        returns (bytes memory result)
    {
        assembly ("memory-safe") {
            // We will store result where the free memory pointer is now, ...
            result := mload(0x40)

            // But first use it to store the calldata

            // Selector of forward(address)
            mstore(result, shl(224, 0x101e8952))
            mstore(add(result, 4), to)

            // We only copy the data, not the length, because the length is read from the calldata size
            let len := mload(data)
            mcopy(add(result, 36), add(data, 32), len)

            // If the call failed, pass through the revert
            if iszero(call(gas(), accountant, 0, result, add(36, len), 0, 0)) {
                returndatacopy(result, 0, returndatasize())
                revert(result, returndatasize())
            }

            // Copy the entire return data into the space where the result is pointing
            mstore(result, returndatasize())
            returndatacopy(add(result, 32), 0, returndatasize())

            // Update the free memory pointer to be after the end of the data, aligned to the next 32 byte word
            mstore(0x40, and(add(add(result, add(32, returndatasize())), 31), not(31)))
        }
    }

    /// @notice Calls updateDebt optimally
    /// @param accountant The flash accountant contract to forward through
    /// @param delta The change in delta for the caller token to effect on the accountant
    function updateDebt(IFlashAccountant accountant, int128 delta) internal {
        assembly ("memory-safe") {
            // cast sig "updateDebt()"
            mstore(0, 0x17c5da6a)
            mstore(32, shl(128, delta))

            if iszero(call(gas(), accountant, 0, 28, 20, 0, 0)) {
                returndatacopy(0, 0, returndatasize())
                revert(0, returndatasize())
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IOrders} from "./IOrders.sol";
import {BuybacksState} from "../types/buybacksState.sol";
import {IExposedStorage} from "./IExposedStorage.sol";

/// @title Revenue Buybacks Interface
/// @notice Interface for automated revenue buyback orders using TWAMM (Time-Weighted Average Market Maker)
/// @dev Defines the interface for managing buyback orders for protocol revenue
interface IRevenueBuybacks is IExposedStorage {
    /// @notice Thrown when minimum order duration exceeds target order duration
    /// @dev This would prevent order creation since the condition order duration >= min order duration would not always be met
    error MinOrderDurationGreaterThanTargetOrderDuration();

    /// @notice Thrown when minimum order duration is set to zero
    /// @dev Orders cannot have zero duration, so this prevents invalid configurations
    error MinOrderDurationMustBeGreaterThanZero();

    /// @notice Thrown when roll is called and a token is not configured
    error TokenNotConfigured(address token);

    /// @notice Emitted when a token's buyback configuration is updated
    /// @param token The token being configured for buybacks
    /// @param state The state after configuring the token
    event Configured(address token, BuybacksState state);

    /// @notice The Orders contract used to create and manage TWAMM orders
    /// @dev All buyback orders are created through this contract
    function ORDERS() external view returns (IOrders);

    /// @notice The NFT token ID that represents all buyback orders created by this contract
    /// @dev A single NFT is minted and reused for all buyback orders to simplify management
    function NFT_ID() external view returns (uint256);

    /// @notice The token that is purchased with collected revenue
    /// @dev This is typically the protocol's governance or utility token
    function BUY_TOKEN() external view returns (address);

    /// @notice Approves the Orders contract to spend unlimited amounts of a token
    /// @dev Must be called at least once for each revenue token before creating buyback orders
    /// @param token The token to approve for spending by the Orders contract
    function approveMax(address token) external;

    /// @notice Withdraws leftover tokens from the contract (only callable by owner)
    /// @dev Used to recover tokens that may be stuck in the contract or to withdraw excess funds
    /// @param token The address of the token to withdraw
    /// @param amount The amount of tokens to withdraw
    function take(address token, uint256 amount) external;

    /// @notice Collects the proceeds from a completed buyback order
    /// @dev Can be called by anyone at any time to collect proceeds from orders that have finished
    /// @param token The revenue token that was sold in the order
    /// @param fee The fee tier of the pool where the order was executed
    /// @param endTime The end time of the order to collect proceeds from
    /// @return proceeds The amount of buyToken received from the completed order
    function collect(address token, uint64 fee, uint64 endTime) external returns (uint128 proceeds);

    /// @notice Creates a new buyback order or extends an existing one with available revenue
    /// @dev Can be called by anyone to trigger the creation of buyback orders using collected revenue
    /// This function will either extend the current order (if conditions are met) or create a new order
    /// @param token The revenue token to use for creating the buyback order
    /// @return endTime The end time of the order that was created or extended
    /// @return saleRate The sale rate of the order (amount of token sold per second)
    function roll(address token) external returns (uint64 endTime, uint112 saleRate);

    /// @notice Configures buyback parameters for a revenue token (only callable by owner)
    /// @dev Sets the timing and fee parameters for automated buyback order creation
    /// @param token The revenue token to configure
    /// @param targetOrderDuration The target duration for new orders (in seconds)
    /// @param minOrderDuration The minimum duration threshold for creating new orders (in seconds)
    /// @param fee The fee tier for the buyback pool
    function configure(address token, uint32 targetOrderDuration, uint32 minOrderDuration, uint64 fee) external;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ILocker, IFlashAccountant} from "../interfaces/IFlashAccountant.sol";

/// @title Base Locker
/// @notice Abstract base contract for contracts that need to interact with the flash accountant
/// @dev Provides locking functionality and token transfer utilities
abstract contract BaseLocker is ILocker {
    /// @notice Thrown when a function is called by an address other than the accountant
    error BaseLockerAccountantOnly();

    /// @notice The flash accountant contract that manages locks and token transfers
    IFlashAccountant internal immutable ACCOUNTANT;

    /// @notice Constructs the BaseLocker with a flash accountant
    /// @param _accountant The flash accountant contract
    constructor(IFlashAccountant _accountant) {
        ACCOUNTANT = _accountant;
    }

    /// CALLBACK HANDLERS

    /// @inheritdoc ILocker
    function locked_6416899205(uint256 id) external {
        if (msg.sender != address(ACCOUNTANT)) revert BaseLockerAccountantOnly();

        bytes memory data = msg.data[36:];

        bytes memory result = handleLockData(id, data);

        assembly ("memory-safe") {
            // raw return whatever the handler sent
            return(add(result, 32), mload(result))
        }
    }

    /// INTERNAL FUNCTIONS

    /// @notice Acquires a lock and executes the provided data
    /// @dev Internal function that calls the accountant's lock function
    /// @param data The data to execute within the lock
    /// @return result The result of the lock execution
    function lock(bytes memory data) internal returns (bytes memory result) {
        address target = address(ACCOUNTANT);

        assembly ("memory-safe") {
            // We will store result where the free memory pointer is now, ...
            result := mload(0x40)

            // But first use it to store the calldata

            // Selector of lock()
            mstore(result, shl(224, 0xf83d08ba))

            // We only copy the data, not the length, because the length is read from the calldata size
            let len := mload(data)
            mcopy(add(result, 4), add(data, 32), len)

            // If the call failed, pass through the revert
            if iszero(call(gas(), target, 0, result, add(len, 4), 0, 0)) {
                returndatacopy(result, 0, returndatasize())
                revert(result, returndatasize())
            }

            // Copy the entire return data into the space where the result is pointing
            mstore(result, returndatasize())
            returndatacopy(add(result, 32), 0, returndatasize())

            // Update the free memory pointer to be after the end of the data, aligned to the next 32 byte word
            mstore(0x40, and(add(add(result, add(32, returndatasize())), 31), not(31)))
        }
    }

    /// @notice Thrown when a lock was expected to revert but didn't
    error ExpectedRevertWithinLock();

    /// @notice Acquires a lock expecting it to revert and returns the revert data
    /// @dev Used for quote functions that use reverts to return data
    /// @param data The data to execute within the lock
    /// @return result The revert data from the lock execution
    function lockAndExpectRevert(bytes memory data) internal returns (bytes memory result) {
        address target = address(ACCOUNTANT);

        assembly ("memory-safe") {
            // We will store result where the free memory pointer is now, ...
            result := mload(0x40)

            // But first use it to store the calldata

            // Selector of lock()
            mstore(result, shl(224, 0xf83d08ba))

            // We only copy the data, not the length, because the length is read from the calldata size
            let len := mload(data)
            mcopy(add(result, 4), add(data, 32), len)

            // If the call succeeded, revert with ExpectedRevertWithinLock.selector
            if call(gas(), target, 0, result, add(len, 4), 0, 0) {
                mstore(0, shl(224, 0x4c816e2b))
                revert(0, 4)
            }

            // Copy the entire revert data into the space where the result is pointing
            mstore(result, returndatasize())
            returndatacopy(add(result, 32), 0, returndatasize())

            // Update the free memory pointer to be after the end of the data, aligned to the next 32 byte word
            mstore(0x40, and(add(add(result, add(32, returndatasize())), 31), not(31)))
        }
    }

    /// @notice Handles the execution of lock data
    /// @dev Must be implemented by derived contracts to define lock behavior
    /// @param id The lock ID
    /// @param data The data to process within the lock
    /// @return result The result of processing the lock data
    function handleLockData(uint256 id, bytes memory data) internal virtual returns (bytes memory result);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IIncentives} from "../interfaces/IIncentives.sol";
import {ExposedStorageLib} from "./ExposedStorageLib.sol";
import {DropKey, toDropId} from "../types/dropKey.sol";
import {DropState} from "../types/dropState.sol";
import {Bitmap} from "../types/bitmap.sol";

/// @title Incentives Library
/// @notice Library providing common storage getters for the Incentives contract
/// @dev These functions access Incentives contract storage directly for gas efficiency
library IncentivesLib {
    using ExposedStorageLib for *;
    using {toDropId} for DropKey;

    /// @notice Converts an index to word and bit position for bitmap storage
    /// @param index The index to convert
    /// @return word The word position in the bitmap
    /// @return bit The bit position within the word
    function claimIndexToStorageIndex(uint256 index) internal pure returns (uint256 word, uint8 bit) {
        (word, bit) = (index >> 8, uint8(index % 256));
    }

    /// @notice Gets the drop state for a given drop key
    /// @dev Accesses the incentives contract's storage directly for gas efficiency
    /// @param incentives The incentives contract instance
    /// @param key The drop key to get state for
    /// @return state The drop state containing funded and claimed amounts
    function getDropState(IIncentives incentives, DropKey memory key) internal view returns (DropState state) {
        bytes32 dropId = key.toDropId();
        state = DropState.wrap(incentives.sload(dropId));
    }

    /// @notice Gets the claimed bitmap for a specific drop and word
    /// @dev Accesses the incentives contract's storage directly for gas efficiency
    /// @param incentives The incentives contract instance
    /// @param key The drop key
    /// @param word The word index in the bitmap
    /// @return bitmap The claimed bitmap for the specified word
    function getClaimedBitmap(IIncentives incentives, DropKey memory key, uint256 word)
        internal
        view
        returns (Bitmap bitmap)
    {
        bytes32 dropId = key.toDropId();
        // Bitmaps are stored starting from drop id + 1 + word
        bytes32 slot;
        unchecked {
            slot = bytes32(uint256(dropId) + 1 + word);
        }
        bitmap = Bitmap.wrap(uint256(incentives.sload(slot)));
    }

    /// @notice Checks if a specific index has been claimed for a drop
    /// @param incentives The incentives contract instance
    /// @param key The drop key to check
    /// @param index The index to check
    /// @return True if the index has been claimed
    function isClaimed(IIncentives incentives, DropKey memory key, uint256 index) internal view returns (bool) {
        (uint256 word, uint8 bit) = claimIndexToStorageIndex(index);
        Bitmap bitmap = getClaimedBitmap(incentives, key, word);
        return bitmap.isSet(bit);
    }

    /// @notice Checks if a claim is available (not claimed and sufficient funds)
    /// @param incentives The incentives contract instance
    /// @param key The drop key to check
    /// @param index The index to check
    /// @param amount The amount to check availability for
    /// @return True if the claim is available
    function isAvailable(IIncentives incentives, DropKey memory key, uint256 index, uint128 amount)
        internal
        view
        returns (bool)
    {
        if (isClaimed(incentives, key, index)) return false;

        DropState state = getDropState(incentives, key);
        return state.getRemaining() >= amount;
    }

    /// @notice Gets the remaining amount available for claims in a drop
    /// @param incentives The incentives contract instance
    /// @param key The drop key to check
    /// @return The remaining amount available
    function getRemaining(IIncentives incentives, DropKey memory key) internal view returns (uint128) {
        DropState state = getDropState(incentives, key);
        return state.getRemaining();
    }

    /// @notice Gets the funded amount for a drop
    /// @param incentives The incentives contract instance
    /// @param key The drop key to check
    /// @return The funded amount
    function getFunded(IIncentives incentives, DropKey memory key) internal view returns (uint128) {
        DropState state = getDropState(incentives, key);
        return state.funded();
    }

    /// @notice Gets the claimed amount for a drop
    /// @param incentives The incentives contract instance
    /// @param key The drop key to check
    /// @return The claimed amount
    function getClaimed(IIncentives incentives, DropKey memory key) internal view returns (uint128) {
        DropState state = getDropState(incentives, key);
        return state.claimed();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "../../types/poolKey.sol";
import {IExtension} from "../ICore.sol";
import {IExposedStorage} from "../IExposedStorage.sol";
import {Snapshot} from "../../types/snapshot.sol";
import {Observation} from "../../types/observation.sol";

/// @title Oracle Interface
/// @notice Interface for the Ekubo Oracle Extension
/// @dev Records price and liquidity into accumulators enabling a separate contract to compute a manipulation resistant average price and liquidity
interface IOracle is IExposedStorage, IExtension {
    /// @notice Thrown when trying to create a pool that doesn't pair with the native token
    error PairsWithNativeTokenOnly();

    /// @notice Thrown when the pool fee is not zero
    error FeeMustBeZero();

    /// @notice Thrown when the tick spacing is not the maximum allowed value
    error FullRangePoolOnly();

    /// @notice Thrown when querying data for a future timestamp
    error FutureTime();

    /// @notice Thrown when no previous snapshot exists for the given time
    /// @param token The token address
    /// @param time The requested timestamp
    error NoPreviousSnapshotExists(address token, uint256 time);

    /// @notice Thrown when the end time is less than the start time
    error EndTimeLessThanStartTime();

    /// @notice Thrown when timestamps are not provided in sorted order
    error TimestampsNotSorted();

    /// @notice Thrown when zero timestamps are provided to a function that requires at least one
    error ZeroTimestampsProvided();

    /// @notice Gets the pool key for the given token
    /// @dev The only allowed pool key for the given token pairs with the native token
    /// @param token The token address
    /// @return The pool key for the token paired with native token
    function getPoolKey(address token) external view returns (PoolKey memory);

    /// @notice Expands the capacity of the list of snapshots for the given token
    /// @param token The token address
    /// @param minCapacity The minimum capacity required
    /// @return capacity The actual capacity after expansion
    function expandCapacity(address token, uint32 minCapacity) external returns (uint32 capacity);

    /// @notice Finds the snapshot with the greatest timestamp ≤ the given time
    /// @param token The token address
    /// @param time The target timestamp
    /// @return count The total number of snapshots for the token
    /// @return logicalIndex The logical index of the found snapshot
    /// @return snapshot The snapshot data
    function findPreviousSnapshot(address token, uint256 time)
        external
        view
        returns (uint256 count, uint256 logicalIndex, Snapshot snapshot);

    /// @notice Returns cumulative snapshot values at a specific time
    /// @dev Extrapolates values from the most recent snapshot before the given time
    /// @param token The token address
    /// @param atTime The target timestamp
    /// @return secondsPerLiquidityCumulative The cumulative seconds per liquidity at the target time
    /// @return tickCumulative The cumulative tick value at the target time
    function extrapolateSnapshot(address token, uint256 atTime)
        external
        view
        returns (uint160 secondsPerLiquidityCumulative, int64 tickCumulative);

    /// @notice Returns extrapolated snapshots at each of the provided sorted timestamps
    /// @dev Efficiently computes observations for multiple timestamps in a single call
    /// @param token The token address
    /// @param timestamps Array of timestamps in ascending order
    /// @return observations Array of observations corresponding to each timestamp
    function getExtrapolatedSnapshotsForSortedTimestamps(address token, uint256[] memory timestamps)
        external
        view
        returns (Observation[] memory observations);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {BasePositions} from "./base/BasePositions.sol";
import {ICore} from "./interfaces/ICore.sol";
import {PoolKey} from "./types/poolKey.sol";
import {computeFee} from "./math/fee.sol";

/// @title Ekubo Protocol Positions
/// @author Moody Salem <moody@ekubo.org>
/// @notice Tracks liquidity positions in Ekubo Protocol as NFTs
/// @dev Manages liquidity positions, fee collection, and protocol fees
contract Positions is BasePositions {
    /// @notice Protocol fee rate for swaps (as a fraction of 2^64)
    uint64 public immutable SWAP_PROTOCOL_FEE_X64;

    /// @notice Denominator for withdrawal protocol fee calculation
    uint64 public immutable WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR;

    /// @notice Constructs the Positions contract
    /// @param core The core contract instance
    /// @param owner The owner of the contract (for access control)
    /// @param _swapProtocolFeeX64 Protocol fee rate for swaps
    /// @param _withdrawalProtocolFeeDenominator Denominator for withdrawal protocol fee
    constructor(ICore core, address owner, uint64 _swapProtocolFeeX64, uint64 _withdrawalProtocolFeeDenominator)
        BasePositions(core, owner)
    {
        SWAP_PROTOCOL_FEE_X64 = _swapProtocolFeeX64;
        WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR = _withdrawalProtocolFeeDenominator;
    }

    /// @notice Handles protocol fee collection during fee collection
    /// @dev Implements the abstract method from BasePositions
    /// @param amount0 The amount of token0 fees collected before protocol fee deduction
    /// @param amount1 The amount of token1 fees collected before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeSwapProtocolFees(PoolKey memory, uint128 amount0, uint128 amount1)
        internal
        view
        override
        returns (uint128 protocolFee0, uint128 protocolFee1)
    {
        if (SWAP_PROTOCOL_FEE_X64 != 0) {
            protocolFee0 = computeFee(amount0, SWAP_PROTOCOL_FEE_X64);
            protocolFee1 = computeFee(amount1, SWAP_PROTOCOL_FEE_X64);
        }
    }

    /// @notice Handles protocol fee collection during liquidity withdrawal
    /// @dev Implements the abstract method from BasePositions
    /// @param poolKey The pool key for the position
    /// @param amount0 The amount of token0 being withdrawn before protocol fee deduction
    /// @param amount1 The amount of token1 being withdrawn before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeWithdrawalProtocolFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1)
        internal
        view
        override
        returns (uint128 protocolFee0, uint128 protocolFee1)
    {
        uint64 fee = poolKey.config.fee();
        if (fee != 0 && WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR != 0) {
            protocolFee0 = computeFee(amount0, fee / WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR);
            protocolFee1 = computeFee(amount1, fee / WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR);
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IExposedStorage} from "../interfaces/IExposedStorage.sol";
import {StorageSlot} from "../types/storageSlot.sol";

/// @dev This library includes some helper functions for calling IExposedStorage#sload and IExposedStorage#tload.
library ExposedStorageLib {
    function sload(IExposedStorage target, bytes32 slot) internal view returns (bytes32 result) {
        assembly ("memory-safe") {
            mstore(0, shl(224, 0x380eb4e0))
            mstore(4, slot)

            if iszero(staticcall(gas(), target, 0, 36, 0, 32)) { revert(0, 0) }

            result := mload(0)
        }
    }

    function sload(IExposedStorage target, bytes32 slot0, bytes32 slot1)
        internal
        view
        returns (bytes32 result0, bytes32 result1)
    {
        assembly ("memory-safe") {
            let o := mload(0x40)
            mstore(o, shl(224, 0x380eb4e0))
            mstore(add(o, 4), slot0)
            mstore(add(o, 36), slot1)

            if iszero(staticcall(gas(), target, o, 68, o, 64)) { revert(0, 0) }

            result0 := mload(o)
            result1 := mload(add(o, 32))
        }
    }

    function sload(IExposedStorage target, bytes32 slot0, bytes32 slot1, bytes32 slot2)
        internal
        view
        returns (bytes32 result0, bytes32 result1, bytes32 result2)
    {
        assembly ("memory-safe") {
            let o := mload(0x40)
            mstore(o, shl(224, 0x380eb4e0))
            mstore(add(o, 4), slot0)
            mstore(add(o, 36), slot1)
            mstore(add(o, 68), slot2)

            if iszero(staticcall(gas(), target, o, 100, o, 96)) { revert(0, 0) }

            result0 := mload(o)
            result1 := mload(add(o, 32))
            result2 := mload(add(o, 64))
        }
    }

    function tload(IExposedStorage target, bytes32 slot) internal view returns (bytes32 result) {
        assembly ("memory-safe") {
            mstore(0, shl(224, 0xed832830))
            mstore(4, slot)

            if iszero(staticcall(gas(), target, 0, 36, 0, 32)) { revert(0, 0) }

            result := mload(0)
        }
    }

    // Overloads for StorageSlot type

    function sload(IExposedStorage target, StorageSlot slot) internal view returns (bytes32 result) {
        return sload(target, StorageSlot.unwrap(slot));
    }

    function sload(IExposedStorage target, StorageSlot slot0, StorageSlot slot1)
        internal
        view
        returns (bytes32 result0, bytes32 result1)
    {
        return sload(target, StorageSlot.unwrap(slot0), StorageSlot.unwrap(slot1));
    }

    function sload(IExposedStorage target, StorageSlot slot0, StorageSlot slot1, StorageSlot slot2)
        internal
        view
        returns (bytes32 result0, bytes32 result1, bytes32 result2)
    {
        return sload(target, StorageSlot.unwrap(slot0), StorageSlot.unwrap(slot1), StorageSlot.unwrap(slot2));
    }

    function tload(IExposedStorage target, StorageSlot slot) internal view returns (bytes32 result) {
        return tload(target, StorageSlot.unwrap(slot));
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

import {ITWAMM} from "../interfaces/extensions/ITWAMM.sol";
import {ICore} from "../interfaces/ICore.sol";
import {ExposedStorageLib} from "./ExposedStorageLib.sol";
import {TWAMMStorageLayout} from "./TWAMMStorageLayout.sol";
import {FlashAccountantLib} from "./FlashAccountantLib.sol";
import {TwammPoolState} from "../types/twammPoolState.sol";
import {OrderState} from "../types/orderState.sol";
import {OrderKey} from "../types/orderKey.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PoolId} from "../types/poolId.sol";
import {OrderId} from "../types/orderId.sol";
import {computeAmountFromSaleRate, computeRewardAmount} from "../math/twamm.sol";
import {StorageSlot} from "../types/storageSlot.sol";

/// @title TWAMM Library
/// @notice Helper methods for interacting with the TWAMM extension
library TWAMMLib {
    using ExposedStorageLib for *;
    using FlashAccountantLib for *;

    function poolState(ITWAMM twamm, PoolId poolId) internal view returns (TwammPoolState twammPoolState) {
        twammPoolState = TwammPoolState.wrap(twamm.sload(TWAMMStorageLayout.twammPoolStateSlot(poolId)));
    }

    function orderState(ITWAMM twamm, address owner, bytes32 salt, OrderId orderId)
        internal
        view
        returns (OrderState state)
    {
        state = OrderState.wrap(
            twamm.sload(
                StorageSlot.unwrap(
                    TWAMMStorageLayout.orderStateSlotFollowedByOrderRewardRateSnapshotSlot(owner, salt, orderId)
                )
            )
        );
    }

    function rewardRateSnapshot(ITWAMM twamm, address owner, bytes32 salt, OrderId orderId)
        internal
        view
        returns (uint256)
    {
        return uint256(
            twamm.sload(
                StorageSlot.unwrap(
                    TWAMMStorageLayout.orderStateSlotFollowedByOrderRewardRateSnapshotSlot(owner, salt, orderId).next()
                )
            )
        );
    }

    function executeVirtualOrdersAndGetCurrentOrderInfo(
        ITWAMM twamm,
        address owner,
        bytes32 salt,
        OrderKey memory orderKey
    ) internal returns (uint112 saleRate, uint256 amountSold, uint256 remainingSellAmount, uint128 purchasedAmount) {
        unchecked {
            PoolKey memory poolKey = orderKey.toPoolKey(address(twamm));
            twamm.lockAndExecuteVirtualOrders(poolKey);

            uint32 lastUpdateTime;
            OrderId orderId = orderKey.toOrderId();

            (lastUpdateTime, saleRate, amountSold) = orderState(twamm, owner, salt, orderId).parse();

            uint256 _rewardRateSnapshot = rewardRateSnapshot(twamm, owner, salt, orderId);

            if (saleRate != 0) {
                (uint64 startTime, uint64 endTime) = (orderKey.config.startTime(), orderKey.config.endTime());

                uint256 rewardRateInside = twamm.getRewardRateInside(poolKey.toPoolId(), orderKey.config);

                purchasedAmount = computeRewardAmount(rewardRateInside - _rewardRateSnapshot, saleRate);

                if (block.timestamp > startTime) {
                    uint32 secondsSinceLastUpdate = uint32(block.timestamp) - lastUpdateTime;

                    uint32 secondsSinceOrderStart = uint32(uint64(block.timestamp) - startTime);

                    uint32 totalOrderDuration = uint32(endTime - startTime);

                    uint32 remainingTimeSinceLastUpdate = uint32(endTime) - lastUpdateTime;

                    uint32 saleDuration = uint32(
                        FixedPointMathLib.min(
                            remainingTimeSinceLastUpdate,
                            FixedPointMathLib.min(
                                FixedPointMathLib.min(secondsSinceLastUpdate, secondsSinceOrderStart),
                                totalOrderDuration
                            )
                        )
                    );

                    amountSold += computeAmountFromSaleRate({
                        saleRate: saleRate, duration: saleDuration, roundUp: false
                    });
                }
                if (block.timestamp < endTime) {
                    remainingSellAmount = computeAmountFromSaleRate({
                        saleRate: saleRate,
                        duration: uint32(endTime - FixedPointMathLib.max(startTime, block.timestamp)),
                        roundUp: true
                    });
                }
            }
        }
    }

    /// @notice Updates the sale rate for a TWAMM order using FlashAccountantLib.forward
    /// @dev Uses FlashAccountantLib.forward to make the necessary call to update the sale rate and parse the result
    /// @param core The core contract to forward through
    /// @param twamm The TWAMM extension contract
    /// @param salt Unique salt for the order
    /// @param orderKey Order key identifying the order
    /// @param saleRateDelta Change in sale rate (positive to increase, negative to decrease)
    /// @return amount The amount delta resulting from the sale rate update
    function updateSaleRate(ICore core, ITWAMM twamm, bytes32 salt, OrderKey memory orderKey, int112 saleRateDelta)
        internal
        returns (int256 amount)
    {
        amount =
            abi.decode(core.forward(address(twamm), abi.encode(uint256(0), salt, orderKey, saleRateDelta)), (int256));
    }

    /// @notice Collects proceeds from a TWAMM order using FlashAccountantLib.forward
    /// @dev Uses FlashAccountantLib.forward to make the necessary call to collect proceeds and parse the result
    /// @param core The core contract to forward through
    /// @param twamm The TWAMM extension contract
    /// @param salt Unique salt for the order
    /// @param orderKey Order key identifying the order
    /// @return proceeds The amount of proceeds collected
    function collectProceeds(ICore core, ITWAMM twamm, bytes32 salt, OrderKey memory orderKey)
        internal
        returns (uint128 proceeds)
    {
        proceeds = abi.decode(core.forward(address(twamm), abi.encode(uint256(1), salt, orderKey)), (uint128));
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ILocker, IForwardee} from "../IFlashAccountant.sol";
import {IExtension} from "../ICore.sol";
import {IExposedStorage} from "../IExposedStorage.sol";
import {PoolKey} from "../../types/poolKey.sol";
import {OrderKey} from "../../types/orderKey.sol";
import {OrderConfig} from "../../types/orderConfig.sol";
import {PoolId} from "../../types/poolId.sol";

/// @title TWAMM Interface
/// @notice Interface for the Ekubo TWAMM Extension
/// @dev Extension for Ekubo Protocol that enables creation of DCA orders that are executed over time
interface ITWAMM is IExposedStorage, IExtension, ILocker, IForwardee {
    /// @notice Emitted when an order is updated
    /// @param owner Address of the order owner
    /// @param salt Unique salt for the order
    /// @param orderKey Order key identifying the order
    /// @param saleRateDelta Change in sale rate applied
    event OrderUpdated(address owner, bytes32 salt, OrderKey orderKey, int112 saleRateDelta);

    /// @notice Emitted when proceeds are withdrawn from an order
    /// @param owner Address of the order owner
    /// @param salt Unique salt for the order
    /// @param orderKey Order key identifying the order
    /// @param amount Amount of tokens withdrawn
    event OrderProceedsWithdrawn(address owner, bytes32 salt, OrderKey orderKey, uint128 amount);

    /// @notice Thrown when the number of orders at a time would overflow
    error TimeNumOrdersOverflow();

    /// @notice Thrown when tick spacing is not the maximum allowed value
    error FullRangePoolOnly();

    /// @notice Thrown when trying to modify an order that has already ended
    error OrderAlreadyEnded();

    /// @notice Thrown when order timestamps are invalid
    error InvalidTimestamps();

    /// @notice Thrown when sale rate delta exceeds maximum allowed value
    error MaxSaleRateDeltaPerTime();

    /// @notice Thrown when trying to operate on an uninitialized pool
    error PoolNotInitialized();

    /// @notice Gets the reward rate inside a time range for a specific token
    /// @dev Used to calculate how much of the buy token an order has earned
    /// @param poolId Unique identifier for the pool
    /// @param config The order config that is being checked
    /// @return result The reward rate inside the specified range
    function getRewardRateInside(PoolId poolId, OrderConfig config) external view returns (uint256 result);

    /// @notice Locks core and executes virtual orders for the given pool key
    /// @dev The pool key must use this extension, which is checked in the locked callback
    /// @param poolKey Pool key identifying the pool
    function lockAndExecuteVirtualOrders(PoolKey memory poolKey) external;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints} from "../types/callPoints.sol";
import {PoolKey, PoolConfig} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {ICore, IExtension} from "../interfaces/ICore.sol";
import {IOracle} from "../interfaces/extensions/IOracle.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ExposedStorage} from "../base/ExposedStorage.sol";
import {BaseExtension} from "../base/BaseExtension.sol";
import {NATIVE_TOKEN_ADDRESS} from "../math/constants.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {Snapshot, createSnapshot} from "../types/snapshot.sol";
import {Counts, createCounts} from "../types/counts.sol";
import {Observation, createObservation} from "../types/observation.sol";
import {PoolId} from "../types/poolId.sol";
import {PoolState} from "../types/poolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {Locker} from "../types/locker.sol";

/// @notice Returns the call points configuration for the Oracle extension
/// @dev Specifies which hooks the Oracle needs to capture price and liquidity data
/// @return The call points configuration for Oracle functionality
function oracleCallPoints() pure returns (CallPoints memory) {
    return CallPoints({
        beforeInitializePool: true,
        afterInitializePool: false,
        beforeUpdatePosition: true,
        afterUpdatePosition: false,
        beforeSwap: true,
        afterSwap: false,
        beforeCollectFees: false,
        afterCollectFees: false
    });
}

/// @notice Converts a logical index to a storage index for circular snapshot array
/// @dev Because the snapshots array is circular, the storage index of the most recently written snapshot can be any value in [0,c.count).
///      To simplify the code, we operate on the logical indices, rather than the storage indices.
///      For logical indices, the most recently written value is always at logicalIndex = c.count-1 and the earliest snapshot is always at logicalIndex = 0.
/// @param index The index of the most recently written snapshot
/// @param count The total number of snapshots that have been written
/// @param logicalIndex The index of the snapshot for which to compute the storage index
/// @return The storage index corresponding to the logical index
function logicalIndexToStorageIndex(uint256 index, uint256 count, uint256 logicalIndex) pure returns (uint256) {
    // We assume index < count and logicalIndex < count
    unchecked {
        return (index + 1 + logicalIndex) % count;
    }
}

/// @title Ekubo Oracle Extension
/// @author Moody Salem <moody@ekubo.org>
/// @notice Records price and liquidity into accumulators enabling a separate contract to compute a manipulation resistant average price and liquidity
contract Oracle is IOracle, ExposedStorage, BaseExtension {
    using CoreLib for ICore;

    constructor(ICore core) BaseExtension(core) {}

    /// @notice Emits a snapshot event for off-chain indexing
    /// @dev Uses assembly for gas-efficient event emission
    /// @param token The token address for the snapshot
    /// @param snapshot The snapshot that was just written
    function _emitSnapshotEvent(address token, Snapshot snapshot) private {
        unchecked {
            assembly ("memory-safe") {
                mstore(0, shl(96, token))
                mstore(20, snapshot)
                log0(0, 52)
            }
        }
    }

    /// @inheritdoc IOracle
    function getPoolKey(address token) public view returns (PoolKey memory) {
        PoolConfig config;
        assembly ("memory-safe") {
            config := shl(96, address())
        }
        return PoolKey({token0: NATIVE_TOKEN_ADDRESS, token1: token, config: config});
    }

    /// @notice Returns the call points configuration for this extension
    /// @dev Overrides the base implementation to return Oracle-specific call points
    /// @return The call points configuration
    function getCallPoints() internal pure override returns (CallPoints memory) {
        return oracleCallPoints();
    }

    /// @notice Inserts a new snapshot if enough time has passed since the last one
    /// @dev Only inserts if block.timestamp > lastTimestamp to avoid duplicate snapshots
    /// @param poolId The unique identifier for the pool
    /// @param token The token address for the oracle data
    function maybeInsertSnapshot(PoolId poolId, address token) private {
        unchecked {
            Counts c;
            assembly ("memory-safe") {
                c := sload(token)
            }

            uint32 timePassed = uint32(block.timestamp) - c.lastTimestamp();
            if (timePassed == 0) return;

            uint32 index = c.index();

            // we know count is always g.t. 0 in the places this is called
            Snapshot last;
            assembly ("memory-safe") {
                last := sload(or(shl(32, token), index))
            }

            PoolState state = CORE.poolState(poolId);

            uint128 liquidity = state.liquidity();
            uint256 nonZeroLiquidity;
            assembly ("memory-safe") {
                nonZeroLiquidity := add(liquidity, iszero(liquidity))
            }

            Snapshot snapshot = createSnapshot({
                _timestamp: uint32(block.timestamp),
                _secondsPerLiquidityCumulative: last.secondsPerLiquidityCumulative()
                    + uint160(FixedPointMathLib.rawDiv(uint256(timePassed) << 128, nonZeroLiquidity)),
                _tickCumulative: last.tickCumulative() + int64(uint64(timePassed)) * state.tick()
            });

            uint32 count = c.count();
            uint32 capacity = c.capacity();

            bool isLastIndex = index == count - 1;
            bool incrementCount = isLastIndex && capacity > count;

            if (incrementCount) count++;
            index = (index + 1) % count;
            uint32 lastTimestamp = uint32(block.timestamp);

            c = createCounts({_index: index, _count: count, _capacity: capacity, _lastTimestamp: lastTimestamp});
            assembly ("memory-safe") {
                sstore(token, c)
                sstore(or(shl(32, token), index), snapshot)
            }

            _emitSnapshotEvent(token, snapshot);
        }
    }

    /// @notice Called before a pool is initialized to set up Oracle tracking
    /// @dev Validates pool configuration and initializes the first snapshot
    function beforeInitializePool(address, PoolKey calldata key, int32)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (key.token0 != NATIVE_TOKEN_ADDRESS) revert PairsWithNativeTokenOnly();
        if (key.config.fee() != 0) revert FeeMustBeZero();
        if (!key.config.isFullRange()) revert FullRangePoolOnly();

        address token = key.token1;

        // in case expandCapacity is called before the pool is initialized:
        //  remember we have the capacity since the snapshot storage has been initialized
        uint32 lastTimestamp = uint32(block.timestamp);

        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }

        c = createCounts({
            _index: 0,
            _count: 1,
            _capacity: uint32(FixedPointMathLib.max(1, c.capacity())),
            _lastTimestamp: lastTimestamp
        });

        Snapshot snapshot =
            createSnapshot({_timestamp: lastTimestamp, _secondsPerLiquidityCumulative: 0, _tickCumulative: 0});

        assembly ("memory-safe") {
            sstore(token, c)
            sstore(shl(32, token), snapshot)
        }

        _emitSnapshotEvent(token, snapshot);
    }

    /// @notice Called before a position is updated to capture price/liquidity snapshot
    /// @dev Inserts a new snapshot if liquidity is changing
    function beforeUpdatePosition(Locker, PoolKey memory poolKey, PositionId, int128 liquidityDelta)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (liquidityDelta != 0) {
            maybeInsertSnapshot(poolKey.toPoolId(), poolKey.token1);
        }
    }

    /// @notice Called before a swap to capture price/liquidity snapshot
    /// @dev Inserts a new snapshot if a swap is occurring
    function beforeSwap(Locker, PoolKey memory poolKey, SwapParameters params)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (params.amount() != 0) {
            maybeInsertSnapshot(poolKey.toPoolId(), poolKey.token1);
        }
    }

    /// @inheritdoc IOracle
    function expandCapacity(address token, uint32 minCapacity) external returns (uint32 capacity) {
        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }

        if (c.capacity() < minCapacity) {
            for (uint256 i = c.capacity(); i < minCapacity; i++) {
                assembly ("memory-safe") {
                    // Simply initialize the slot, it will be overwritten when the index is reached
                    sstore(or(shl(32, token), i), 1)
                }
            }
            c = createCounts({
                _index: c.index(), _count: c.count(), _capacity: minCapacity, _lastTimestamp: c.lastTimestamp()
            });
            assembly ("memory-safe") {
                sstore(token, c)
            }
        }

        capacity = c.capacity();
    }

    /// @notice Searches for the latest snapshot with timestamp <= time within a logical range
    /// @dev Searches the logical range [min, maxExclusive) for the latest snapshot with timestamp <= time.
    ///      See logicalIndexToStorageIndex for an explanation of logical indices.
    ///      We make the assumption that all snapshots for the token were written within (2**32 - 1) seconds of the current block timestamp
    /// @param c The counts containing metadata about the snapshots array
    /// @param token The token address to search snapshots for
    /// @param time The target timestamp to search for
    /// @param logicalMin The minimum logical index to search from
    /// @param logicalMaxExclusive The maximum logical index to search to (exclusive)
    /// @return logicalIndex The logical index of the found snapshot
    /// @return snapshot The snapshot data at the found index
    function searchRangeForPrevious(
        Counts c,
        address token,
        uint256 time,
        uint256 logicalMin,
        uint256 logicalMaxExclusive
    ) private view returns (uint256 logicalIndex, Snapshot snapshot) {
        unchecked {
            if (logicalMin >= logicalMaxExclusive) {
                revert NoPreviousSnapshotExists(token, time);
            }

            uint32 current = uint32(block.timestamp);
            uint32 targetDiff = current - uint32(time);

            uint256 left = logicalMin;
            uint256 right = logicalMaxExclusive - 1;
            while (left < right) {
                uint256 mid = (left + right + 1) >> 1;
                uint256 storageIndex = logicalIndexToStorageIndex(c.index(), c.count(), mid);
                Snapshot midSnapshot;
                assembly ("memory-safe") {
                    midSnapshot := sload(or(shl(32, token), storageIndex))
                }
                if (current - midSnapshot.timestamp() >= targetDiff) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }

            uint256 resultIndex = logicalIndexToStorageIndex(c.index(), c.count(), left);
            assembly ("memory-safe") {
                snapshot := sload(or(shl(32, token), resultIndex))
            }
            if (current - snapshot.timestamp() < targetDiff) {
                revert NoPreviousSnapshotExists(token, time);
            }
            return (left, snapshot);
        }
    }

    /// @inheritdoc IOracle
    function findPreviousSnapshot(address token, uint256 time)
        public
        view
        returns (uint256 count, uint256 logicalIndex, Snapshot snapshot)
    {
        if (time > block.timestamp) revert FutureTime();

        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }
        count = c.count();
        (logicalIndex, snapshot) = searchRangeForPrevious(c, token, time, 0, count);
    }

    /// @notice Computes cumulative values at a given time by extrapolating from a previous snapshot
    /// @dev Uses linear interpolation between snapshots or current pool state for extrapolation
    /// @param c The counts containing metadata about the snapshots array
    /// @param token The token address to extrapolate for
    /// @param atTime The timestamp to extrapolate to
    /// @param logicalIndex The logical index of the base snapshot
    /// @param snapshot The base snapshot to extrapolate from
    /// @return secondsPerLiquidityCumulative The extrapolated seconds per liquidity cumulative
    /// @return tickCumulative The extrapolated tick cumulative
    function extrapolateSnapshotInternal(
        Counts c,
        address token,
        uint256 atTime,
        uint256 logicalIndex,
        Snapshot snapshot
    ) private view returns (uint160 secondsPerLiquidityCumulative, int64 tickCumulative) {
        unchecked {
            secondsPerLiquidityCumulative = snapshot.secondsPerLiquidityCumulative();
            tickCumulative = snapshot.tickCumulative();
            uint32 timePassed = uint32(atTime) - snapshot.timestamp();
            if (timePassed != 0) {
                if (logicalIndex == c.count() - 1) {
                    // Use current pool state.
                    PoolId poolId = getPoolKey(token).toPoolId();
                    PoolState state = CORE.poolState(poolId);

                    tickCumulative += int64(state.tick()) * int64(uint64(timePassed));
                    secondsPerLiquidityCumulative += uint160(
                        FixedPointMathLib.rawDiv(
                            uint256(timePassed) << 128, FixedPointMathLib.max(1, state.liquidity())
                        )
                    );
                } else {
                    // Use the next snapshot.
                    uint256 logicalIndexNext = logicalIndexToStorageIndex(c.index(), c.count(), logicalIndex + 1);
                    Snapshot next;
                    assembly ("memory-safe") {
                        next := sload(or(shl(32, token), logicalIndexNext))
                    }

                    uint32 timestampDifference = next.timestamp() - snapshot.timestamp();

                    tickCumulative += int64(
                        FixedPointMathLib.rawSDiv(
                            int256(uint256(timePassed)) * (next.tickCumulative() - snapshot.tickCumulative()),
                            int256(uint256(timestampDifference))
                        )
                    );
                    secondsPerLiquidityCumulative += uint160(
                        (uint256(timePassed)
                                * (next.secondsPerLiquidityCumulative() - snapshot.secondsPerLiquidityCumulative()))
                            / timestampDifference
                    );
                }
            }
        }
    }

    /// @inheritdoc IOracle
    function extrapolateSnapshot(address token, uint256 atTime)
        public
        view
        returns (uint160 secondsPerLiquidityCumulative, int64 tickCumulative)
    {
        if (atTime > block.timestamp) revert FutureTime();

        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }
        (uint256 logicalIndex, Snapshot snapshot) = searchRangeForPrevious(c, token, atTime, 0, c.count());
        (secondsPerLiquidityCumulative, tickCumulative) =
            extrapolateSnapshotInternal(c, token, atTime, logicalIndex, snapshot);
    }

    /// @inheritdoc IOracle
    function getExtrapolatedSnapshotsForSortedTimestamps(address token, uint256[] memory timestamps)
        public
        view
        returns (Observation[] memory observations)
    {
        unchecked {
            if (timestamps.length == 0) revert ZeroTimestampsProvided();
            uint256 startTime = timestamps[0];
            uint256 endTime = timestamps[timestamps.length - 1];
            if (endTime < startTime) revert EndTimeLessThanStartTime();

            Counts c;
            assembly ("memory-safe") {
                c := sload(token)
            }
            (uint256 indexFirst,) = searchRangeForPrevious(c, token, startTime, 0, c.count());
            (uint256 indexLast,) = searchRangeForPrevious(c, token, endTime, indexFirst, c.count());

            observations = new Observation[](timestamps.length);
            uint256 lastTimestamp;
            for (uint256 i = 0; i < timestamps.length; i++) {
                uint256 timestamp = timestamps[i];

                if (timestamp < lastTimestamp) {
                    revert TimestampsNotSorted();
                } else if (timestamp > block.timestamp) {
                    revert FutureTime();
                }

                (uint256 logicalIndex, Snapshot snapshot) =
                    searchRangeForPrevious(c, token, timestamp, indexFirst, indexLast + 1);
                (uint160 spcCumulative, int64 tcCumulative) =
                    extrapolateSnapshotInternal(c, token, timestamp, logicalIndex, snapshot);
                observations[i] = createObservation(spcCumulative, tcCumulative);
                indexFirst = logicalIndex;
                lastTimestamp = timestamp;
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {OrderKey} from "../types/orderKey.sol";
import {IBaseNonfungibleToken} from "./IBaseNonfungibleToken.sol";

/// @title Orders Interface
/// @notice Interface for managing TWAMM (Time-Weighted Average Market Maker) orders as NFTs
/// @dev Defines the interface for creating, modifying, and collecting proceeds from long-term orders
interface IOrders is IBaseNonfungibleToken {
    /// @notice Thrown when trying to modify an order that has already ended
    error OrderAlreadyEnded();

    /// @notice Thrown when the calculated sale rate exceeds the maximum allowed
    error MaxSaleRateExceeded();

    /// @notice Mints a new NFT and creates a TWAMM order
    /// @param orderKey Key identifying the order parameters
    /// @param amount Amount of tokens to sell over the order duration
    /// @param maxSaleRate Maximum acceptable sale rate (for slippage protection)
    /// @return id The newly minted NFT token ID
    /// @return saleRate The calculated sale rate for the order
    function mintAndIncreaseSellAmount(OrderKey memory orderKey, uint112 amount, uint112 maxSaleRate)
        external
        payable
        returns (uint256 id, uint112 saleRate);

    /// @notice Increases the sell amount for an existing TWAMM order
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @param amount Additional amount of tokens to sell
    /// @param maxSaleRate Maximum acceptable sale rate (for slippage protection)
    /// @return saleRate The calculated sale rate for the additional amount
    function increaseSellAmount(uint256 id, OrderKey memory orderKey, uint128 amount, uint112 maxSaleRate)
        external
        payable
        returns (uint112 saleRate);

    /// @notice Decreases the sale rate for an existing TWAMM order
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @param saleRateDecrease Amount to decrease the sale rate by
    /// @param recipient Address to receive the refunded tokens
    /// @return refund Amount of tokens refunded
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease, address recipient)
        external
        payable
        returns (uint112 refund);

    /// @notice Decreases the sale rate for an existing TWAMM order (refund to msg.sender)
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @param saleRateDecrease Amount to decrease the sale rate by
    /// @return refund Amount of tokens refunded
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease)
        external
        payable
        returns (uint112 refund);

    /// @notice Collects the proceeds from a TWAMM order
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @param recipient Address to receive the proceeds
    /// @return proceeds Amount of tokens collected as proceeds
    function collectProceeds(uint256 id, OrderKey memory orderKey, address recipient)
        external
        payable
        returns (uint128 proceeds);

    /// @notice Collects the proceeds from a TWAMM order (to msg.sender)
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @return proceeds Amount of tokens collected as proceeds
    function collectProceeds(uint256 id, OrderKey memory orderKey) external payable returns (uint128 proceeds);

    /// @notice Executes virtual orders and returns current order information
    /// @dev Updates the order state by executing any pending virtual orders
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @return saleRate Current sale rate of the order
    /// @return amountSold Total amount sold so far
    /// @return remainingSellAmount Amount remaining to be sold
    /// @return purchasedAmount Amount of tokens purchased (proceeds available)
    function executeVirtualOrdersAndGetCurrentOrderInfo(uint256 id, OrderKey memory orderKey)
        external
        returns (uint112 saleRate, uint256 amountSold, uint256 remainingSellAmount, uint128 purchasedAmount);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "../types/poolId.sol";
import {PositionId} from "../types/positionId.sol";
import {StorageSlot} from "../types/storageSlot.sol";

/// @title Core Storage Layout
/// @notice Library providing functions to compute the storage locations for the Core contract
/// @dev Core uses a custom storage layout to avoid keccak's where possible.
///      For certain storage values, the pool id is used as a base offset and
///      we allocate the following relative offsets (starting from the pool id) as:
///        0: pool state
///        [FPL_OFFSET, FPL_OFFSET + 1]: fees per liquidity
///        [TICKS_OFFSET + MIN_TICK, TICKS_OFFSET + MAX_TICK]: tick info
///        [FPL_OUTSIDE_OFFSET_VALUE0 + MIN_TICK, FPL_OUTSIDE_OFFSET_VALUE0 + MAX_TICK]: fees per liquidity outside (value0)
///        [FPL_OUTSIDE_OFFSET_VALUE0 + FPL_OUTSIDE_OFFSET_VALUE1 + MIN_TICK, FPL_OUTSIDE_OFFSET_VALUE0 + FPL_OUTSIDE_OFFSET_VALUE1 + MAX_TICK]: fees per liquidity outside (value1)
///        [BITMAPS_OFFSET + FIRST_BITMAP_WORD, BITMAPS_OFFSET + LAST_BITMAP_WORD]: tick bitmaps
library CoreStorageLayout {
    /// @dev Generated using: cast keccak "CoreStorageLayout#FPL_OFFSET"
    uint256 internal constant FPL_OFFSET = 0xb09b03866d96933565a9435bfb511c8ac5b2be454285ca331201452704799f72;
    /// @dev Generated using: cast keccak "CoreStorageLayout#TICKS_OFFSET"
    uint256 internal constant TICKS_OFFSET = 0x435a5eb89a296820174331cf5a3902d9fca683928d56726d8e7acd6efb28c568;
    /// @dev Generated using: cast keccak "CoreStorageLayout#FPL_OUTSIDE_OFFSET_VALUE0"
    uint256 internal constant FPL_OUTSIDE_OFFSET_VALUE0 =
        0x5695060fdb9cfea656f872ae4887221aff7dbfefc45eaf753e4e70cdfb5cd19c;
    /// @dev Generated using: cast keccak "CoreStorageLayout#FPL_OUTSIDE_OFFSET_VALUE1"
    uint256 internal constant FPL_OUTSIDE_OFFSET_VALUE1 =
        0x7a2a03fc08af3dae7869678617dc8abe8f15a3b719b37ba108dba879571f8b02;
    /// @dev Generated using: cast keccak "CoreStorageLayout#BITMAPS_OFFSET"
    uint256 internal constant BITMAPS_OFFSET = 0x3def450d0010a2fef515ce5eba4b363b5a0f42fdd4c53e1c737975db05a2e3a5;

    /// @notice Computes the storage slot containing information on whether an extension is registered
    /// @param extension The extension address to check
    /// @return slot The storage slot in the Core contract
    function isExtensionRegisteredSlot(address extension) internal pure returns (StorageSlot slot) {
        assembly ("memory-safe") {
            mstore(0, extension)
            mstore(32, 0)
            slot := keccak256(0, 64)
        }
    }

    /// @notice Computes the storage slot of the current state of a pool
    /// @param poolId The unique identifier for the pool
    /// @return slot The storage slot in the Core contract
    function poolStateSlot(PoolId poolId) internal pure returns (StorageSlot slot) {
        slot = StorageSlot.wrap(PoolId.unwrap(poolId));
    }

    /// @notice Computes the storage slots of the current global fees per liquidity of a pool
    /// @param poolId The unique identifier for the pool
    /// @return firstSlot The first of two consecutive storage slots in the Core contract
    function poolFeesPerLiquiditySlot(PoolId poolId) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, FPL_OFFSET)
        }
    }

    /// @notice Computes the storage slot of tick information for a specific tick in a pool
    /// @param poolId The unique identifier for the pool
    /// @param tick The tick to query
    /// @return slot The storage slot in the Core contract
    function poolTicksSlot(PoolId poolId, int32 tick) internal pure returns (StorageSlot slot) {
        assembly ("memory-safe") {
            slot := add(poolId, add(tick, TICKS_OFFSET))
        }
    }

    /// @notice Computes the storage slots of the outside fees of a pool for a given tick
    /// @param poolId The unique identifier for the pool
    /// @param tick The tick to query
    /// @return firstSlot The first storage slot in the Core contract
    /// @return secondSlot The second storage slot in the Core contract
    function poolTickFeesPerLiquidityOutsideSlot(PoolId poolId, int32 tick)
        internal
        pure
        returns (StorageSlot firstSlot, StorageSlot secondSlot)
    {
        assembly ("memory-safe") {
            firstSlot := add(poolId, add(FPL_OUTSIDE_OFFSET_VALUE0, tick))
            secondSlot := add(firstSlot, FPL_OUTSIDE_OFFSET_VALUE1)
        }
    }

    /// @notice Computes the first storage slot of the tick bitmaps for a specific pool
    /// @param poolId The unique identifier for the pool
    /// @return firstSlot The first storage slot in the Core contract
    function tickBitmapsSlot(PoolId poolId) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, BITMAPS_OFFSET)
        }
    }

    /// @notice Computes the first storage slot of the position data for a specific position in a pool
    /// @param poolId The unique identifier for the pool
    /// @param owner The position owner
    /// @param positionId The unique identifier for the position
    /// @return firstSlot The first of three consecutive storage slots in the Core contract
    function poolPositionsSlot(PoolId poolId, address owner, PositionId positionId)
        internal
        pure
        returns (StorageSlot firstSlot)
    {
        assembly ("memory-safe") {
            let free := mload(0x40)
            mstore(free, positionId)
            mstore(add(free, 0x20), poolId)
            mstore(add(free, 0x40), owner)
            mstore(0, keccak256(free, 0x60))
            mstore(32, 1)
            firstSlot := keccak256(0, 64)
        }
    }

    /// @notice Computes the storage slot for saved balances
    /// @param owner The owner of the saved balances
    /// @param token0 The first token address
    /// @param token1 The second token address
    /// @param salt The salt used for the saved balance key
    /// @return slot The storage slot in the Core contract
    function savedBalancesSlot(address owner, address token0, address token1, bytes32 salt)
        internal
        pure
        returns (StorageSlot slot)
    {
        assembly ("memory-safe") {
            let free := mload(0x40)
            mstore(free, owner)
            mstore(add(free, 0x20), token0)
            mstore(add(free, 0x40), token1)
            mstore(add(free, 0x60), salt)
            slot := keccak256(free, 128)
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {Bitmap} from "./types/bitmap.sol";
import {DropKey} from "./types/dropKey.sol";
import {ClaimKey} from "./types/claimKey.sol";
import {DropState} from "./types/dropState.sol";
import {StorageSlot} from "./types/storageSlot.sol";
import {IIncentives} from "./interfaces/IIncentives.sol";
import {IncentivesLib} from "./libraries/IncentivesLib.sol";
import {ExposedStorage} from "./base/ExposedStorage.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {Multicallable} from "solady/utils/Multicallable.sol";
import {MerkleProofLib} from "solady/utils/MerkleProofLib.sol";

/// @author Moody Salem
/// @notice A singleton contract for making many airdrops
contract Incentives is IIncentives, ExposedStorage, Multicallable {
    /// @inheritdoc IIncentives
    function fund(DropKey memory key, uint128 minimum) external override returns (uint128 fundedAmount) {
        bytes32 id = key.toDropId();

        // Load drop state from storage slot: drop id
        DropState dropState;
        assembly ("memory-safe") {
            dropState := sload(id)
        }

        uint128 currentFunded = dropState.funded();
        if (currentFunded < minimum) {
            fundedAmount = minimum - currentFunded;
            dropState = dropState.setFunded(minimum);

            // Store updated drop state
            assembly ("memory-safe") {
                sstore(id, dropState)
            }

            SafeTransferLib.safeTransferFrom(key.token, msg.sender, address(this), fundedAmount);
            emit Funded(key, minimum);
        }
    }

    /// @inheritdoc IIncentives
    function refund(DropKey memory key) external override returns (uint128 refundAmount) {
        if (msg.sender != key.owner) {
            revert DropOwnerOnly();
        }

        bytes32 id = key.toDropId();

        // Load drop state from storage slot: drop id
        DropState dropState;
        assembly ("memory-safe") {
            dropState := sload(id)
        }

        refundAmount = dropState.getRemaining();
        if (refundAmount > 0) {
            // Set funded amount to claimed amount (no remaining funds)
            dropState = dropState.setFunded(dropState.claimed());

            // Store updated drop state
            assembly ("memory-safe") {
                sstore(id, dropState)
            }

            SafeTransferLib.safeTransfer(key.token, key.owner, refundAmount);
        }
        emit Refunded(key, refundAmount);
    }

    /// @inheritdoc IIncentives
    function claim(DropKey memory key, ClaimKey memory c, bytes32[] calldata proof) external override {
        bytes32 id = key.toDropId();

        // Check that it is not claimed
        (uint256 word, uint8 bit) = IncentivesLib.claimIndexToStorageIndex(c.index);
        StorageSlot bitmapSlot;
        unchecked {
            bitmapSlot = StorageSlot.wrap(bytes32(uint256(id) + 1 + word));
        }
        Bitmap bitmap = Bitmap.wrap(uint256(bitmapSlot.load()));
        if (bitmap.isSet(bit)) revert AlreadyClaimed();

        // Check the proof is valid
        bytes32 leaf = c.toClaimId();
        if (!MerkleProofLib.verify(proof, key.root, leaf)) revert InvalidProof();

        // Load drop state from storage slot: drop id
        DropState dropState;
        assembly ("memory-safe") {
            dropState := sload(id)
        }

        // Check sufficient funds
        uint128 remaining = dropState.getRemaining();
        if (remaining < c.amount) {
            revert InsufficientFunds();
        }

        // Update claimed amount
        dropState = dropState.setClaimed(dropState.claimed() + c.amount);

        // Store updated drop state
        assembly ("memory-safe") {
            sstore(id, dropState)
        }

        // Update claimed bitmap
        bitmap = bitmap.toggle(bit);
        assembly ("memory-safe") {
            sstore(bitmapSlot, bitmap)
        }

        SafeTransferLib.safeTransfer(key.token, c.account, c.amount);
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IExtension} from "../interfaces/ICore.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {PoolState} from "../types/poolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {Locker} from "../types/locker.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

/// @dev Contains methods for determining whether an extension should be called
library ExtensionCallPointsLib {
    function shouldCallBeforeInitializePool(IExtension extension, address initializer)
        internal
        pure
        returns (bool yes)
    {
        assembly ("memory-safe") {
            yes := and(shr(152, extension), iszero(eq(initializer, extension)))
        }
    }

    function maybeCallBeforeInitializePool(
        IExtension extension,
        address initializer,
        PoolKey memory poolKey,
        int32 tick
    ) internal {
        bool needCall = shouldCallBeforeInitializePool(extension, initializer);
        assembly ("memory-safe") {
            if needCall {
                let freeMem := mload(0x40)
                // cast sig "beforeInitializePool(address, (address, address, bytes32), int32)"
                mstore(freeMem, shl(224, 0x1fbbb462))
                mstore(add(freeMem, 4), initializer)
                mcopy(add(freeMem, 36), poolKey, 96)
                mstore(add(freeMem, 132), tick)
                // bubbles up the revert
                if iszero(call(gas(), extension, 0, freeMem, 164, 0, 0)) {
                    returndatacopy(freeMem, 0, returndatasize())
                    revert(freeMem, returndatasize())
                }
            }
        }
    }

    function shouldCallAfterInitializePool(IExtension extension, address initializer) internal pure returns (bool yes) {
        assembly ("memory-safe") {
            yes := and(shr(159, extension), iszero(eq(initializer, extension)))
        }
    }

    function maybeCallAfterInitializePool(
        IExtension extension,
        address initializer,
        PoolKey memory poolKey,
        int32 tick,
        SqrtRatio sqrtRatio
    ) internal {
        bool needCall = shouldCallAfterInitializePool(extension, initializer);
        assembly ("memory-safe") {
            if needCall {
                let freeMem := mload(0x40)
                // cast sig "afterInitializePool(address, (address, address, bytes32), int32, uint96)"
                mstore(freeMem, shl(224, 0x948374ff))
                mstore(add(freeMem, 4), initializer)
                mcopy(add(freeMem, 36), poolKey, 96)
                mstore(add(freeMem, 132), tick)
                mstore(add(freeMem, 164), sqrtRatio)
                // bubbles up the revert
                if iszero(call(gas(), extension, 0, freeMem, 196, 0, 0)) {
                    returndatacopy(freeMem, 0, returndatasize())
                    revert(freeMem, returndatasize())
                }
            }
        }
    }

    function shouldCallBeforeSwap(IExtension extension, Locker locker) internal pure returns (bool yes) {
        assembly ("memory-safe") {
            yes := and(shr(158, extension), iszero(eq(shl(96, locker), shl(96, extension))))
        }
    }

    function maybeCallBeforeSwap(IExtension extension, Locker locker, PoolKey memory poolKey, SwapParameters params)
        internal
    {
        bool needCall = shouldCallBeforeSwap(extension, locker);
        assembly ("memory-safe") {
            if needCall {
                let freeMem := mload(0x40)
                // cast sig "beforeSwap(bytes32,(address,address,bytes32),bytes32)"
                mstore(freeMem, shl(224, 0xca11dba7))
                mstore(add(freeMem, 4), locker)
                mcopy(add(freeMem, 36), poolKey, 96)
                mstore(add(freeMem, 132), params)
                // bubbles up the revert
                if iszero(call(gas(), extension, 0, freeMem, 164, 0, 0)) {
                    returndatacopy(freeMem, 0, returndatasize())
                    revert(freeMem, returndatasize())
                }
            }
        }
    }

    function shouldCallAfterSwap(IExtension extension, Locker locker) internal pure returns (bool yes) {
        assembly ("memory-safe") {
            yes := and(shr(157, extension), iszero(eq(shl(96, locker), shl(96, extension))))
        }
    }

    function maybeCallAfterSwap(
        IExtension extension,
        Locker locker,
        PoolKey memory poolKey,
        SwapParameters params,
        PoolBalanceUpdate balanceUpdate,
        PoolState stateAfter
    ) internal {
        bool needCall = shouldCallAfterSwap(extension, locker);
        assembly ("memory-safe") {
            if needCall {
                let freeMem := mload(0x40)
                // cast sig "afterSwap(bytes32,(address,address,bytes32),bytes32,bytes32,bytes32)"
                mstore(freeMem, shl(224, 0xa4e8f288))
                mstore(add(freeMem, 4), locker)
                mcopy(add(freeMem, 36), poolKey, 96)
                mstore(add(freeMem, 132), params)
                mstore(add(freeMem, 164), balanceUpdate)
                mstore(add(freeMem, 196), stateAfter)
                // bubbles up the revert
                if iszero(call(gas(), extension, 0, freeMem, 228, 0, 0)) {
                    returndatacopy(freeMem, 0, returndatasize())
                    revert(freeMem, returndatasize())
                }
            }
        }
    }

    function shouldCallBeforeUpdatePosition(IExtension extension, Locker locker) internal pure returns (bool yes) {
        assembly ("memory-safe") {
            yes := and(shr(156, extension), iszero(eq(shl(96, locker), shl(96, extension))))
        }
    }

    function maybeCallBeforeUpdatePosition(
        IExtension extension,
        Locker locker,
        PoolKey memory poolKey,
        PositionId positionId,
        int128 liquidityDelta
    ) internal {
        bool needCall = shouldCallBeforeUpdatePosition(extension, locker);
        assembly ("memory-safe") {
            if needCall {
                let freeMem := mload(0x40)
                // cast sig "beforeUpdatePosition(bytes32, (address,address,bytes32), bytes32, int128)"
                mstore(freeMem, shl(224, 0x0035c723))
                mstore(add(freeMem, 4), locker)
                mcopy(add(freeMem, 36), poolKey, 96)
                mstore(add(freeMem, 132), positionId)
                mstore(add(freeMem, 164), liquidityDelta)
                // bubbles up the revert
                if iszero(call(gas(), extension, 0, freeMem, 196, 0, 0)) {
                    returndatacopy(freeMem, 0, returndatasize())
                    revert(freeMem, returndatasize())
                }
            }
        }
    }

    function shouldCallAfterUpdatePosition(IExtension extension, Locker locker) internal pure returns (bool yes) {
        assembly ("memory-safe") {
            yes := and(shr(155, extension), iszero(eq(shl(96, locker), shl(96, extension))))
        }
    }

    function maybeCallAfterUpdatePosition(
        IExtension extension,
        Locker locker,
        PoolKey memory poolKey,
        PositionId positionId,
        int128 liquidityDelta,
        PoolBalanceUpdate balanceUpdate,
        PoolState stateAfter
    ) internal {
        bool needCall = shouldCallAfterUpdatePosition(extension, locker);
        assembly ("memory-safe") {
            if needCall {
                let freeMem := mload(0x40)
                // cast sig "afterUpdatePosition(bytes32,(address,address,bytes32),bytes32,int128,bytes32,bytes32)"
                mstore(freeMem, shl(224, 0x25fa4e69))
                mstore(add(freeMem, 4), locker)
                mcopy(add(freeMem, 36), poolKey, 96)
                mstore(add(freeMem, 132), positionId)
                mstore(add(freeMem, 164), liquidityDelta)
                mstore(add(freeMem, 196), balanceUpdate)
                mstore(add(freeMem, 228), stateAfter)
                // bubbles up the revert
                if iszero(call(gas(), extension, 0, freeMem, 260, 0, 0)) {
                    returndatacopy(freeMem, 0, returndatasize())
                    revert(freeMem, returndatasize())
                }
            }
        }
    }

    function shouldCallBeforeCollectFees(IExtension extension, Locker locker) internal pure returns (bool yes) {
        assembly ("memory-safe") {
            yes := and(shr(154, extension), iszero(eq(shl(96, locker), shl(96, extension))))
        }
    }

    function maybeCallBeforeCollectFees(
        IExtension extension,
        Locker locker,
        PoolKey memory poolKey,
        PositionId positionId
    ) internal {
        bool needCall = shouldCallBeforeCollectFees(extension, locker);
        assembly ("memory-safe") {
            if needCall {
                let freeMem := mload(0x40)
                // cast sig "beforeCollectFees(bytes32, (address,address,bytes32), bytes32)"
                mstore(freeMem, shl(224, 0xdf65d8d1))
                mstore(add(freeMem, 4), locker)
                mcopy(add(freeMem, 36), poolKey, 96)
                mstore(add(freeMem, 132), positionId)
                // bubbles up the revert
                if iszero(call(gas(), extension, 0, freeMem, 164, 0, 0)) {
                    returndatacopy(freeMem, 0, returndatasize())
                    revert(freeMem, returndatasize())
                }
            }
        }
    }

    function shouldCallAfterCollectFees(IExtension extension, Locker locker) internal pure returns (bool yes) {
        assembly ("memory-safe") {
            yes := and(shr(153, extension), iszero(eq(shl(96, locker), shl(96, extension))))
        }
    }

    function maybeCallAfterCollectFees(
        IExtension extension,
        Locker locker,
        PoolKey memory poolKey,
        PositionId positionId,
        uint128 amount0,
        uint128 amount1
    ) internal {
        bool needCall = shouldCallAfterCollectFees(extension, locker);
        assembly ("memory-safe") {
            if needCall {
                let freeMem := mload(0x40)
                // cast sig "afterCollectFees(bytes32, (address,address,bytes32), bytes32, uint128, uint128)"
                mstore(freeMem, shl(224, 0x751fd5df))
                mstore(add(freeMem, 4), locker)
                mcopy(add(freeMem, 36), poolKey, 96)
                mstore(add(freeMem, 132), positionId)
                mstore(add(freeMem, 164), amount0)
                mstore(add(freeMem, 196), amount1)
                // bubbles up the revert
                if iszero(call(gas(), extension, 0, freeMem, 228, 0, 0)) {
                    returndatacopy(freeMem, 0, returndatasize())
                    revert(freeMem, returndatasize())
                }
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints} from "../types/callPoints.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {SqrtRatio, MIN_SQRT_RATIO, MAX_SQRT_RATIO} from "../types/sqrtRatio.sol";
import {ICore, IExtension} from "../interfaces/ICore.sol";
import {ITWAMM} from "../interfaces/extensions/ITWAMM.sol";
import {TimeInfo, createTimeInfo} from "../types/timeInfo.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ExposedStorage} from "../base/ExposedStorage.sol";
import {TWAMMStorageLayout} from "../libraries/TWAMMStorageLayout.sol";
import {StorageSlot} from "../types/storageSlot.sol";
import {BaseExtension} from "../base/BaseExtension.sol";
import {BaseForwardee} from "../base/BaseForwardee.sol";
import {PoolState} from "../types/poolState.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";
import {TwammPoolState, createTwammPoolState} from "../types/twammPoolState.sol";
import {OrderKey} from "../types/orderKey.sol";
import {OrderConfig} from "../types/orderConfig.sol";
import {OrderState, createOrderState} from "../types/orderState.sol";
import {searchForNextInitializedTime, flipTime} from "../math/timeBitmap.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {computeFee} from "../math/fee.sol";
import {
    computeNextSqrtRatio,
    computeAmountFromSaleRate,
    computeRewardAmount,
    addSaleRateDelta
} from "../math/twamm.sol";
import {isTimeValid, MAX_ABS_VALUE_SALE_RATE_DELTA} from "../math/time.sol";
import {PoolId} from "../types/poolId.sol";
import {OrderId} from "../types/orderId.sol";
import {SwapParameters, createSwapParameters} from "../types/swapParameters.sol";
import {Locker} from "../types/locker.sol";
import {LibBit} from "solady/utils/LibBit.sol";

/// @notice Returns the call points configuration for the TWAMM extension
/// @dev Specifies which hooks TWAMM needs to execute virtual orders and manage DCA functionality
/// @return The call points configuration for TWAMM functionality
function twammCallPoints() pure returns (CallPoints memory) {
    return CallPoints({
        beforeInitializePool: false,
        afterInitializePool: true,
        beforeUpdatePosition: true,
        afterUpdatePosition: false,
        beforeSwap: true,
        afterSwap: false,
        beforeCollectFees: true,
        afterCollectFees: false
    });
}

/// @title Ekubo TWAMM (Time-Weighted Average Market Maker)
/// @author Moody Salem <moody@ekubo.org>
/// @notice Extension for Ekubo Protocol that enables creation of DCA (Dollar Cost Averaging) orders that are executed over time
/// @dev Implements virtual order execution that spreads trades over time periods to reduce price impact and provide better execution
contract TWAMM is ITWAMM, ExposedStorage, BaseExtension, BaseForwardee {
    using CoreLib for *;

    constructor(ICore core) BaseExtension(core) BaseForwardee(core) {}

    /// @notice Emits an event for virtual order execution
    /// @dev Emits an event for the virtual order execution. Assumes that saleRateToken0 and saleRateToken1 are <= type(uint112).max
    /// @param poolId The unique identifier for the pool
    /// @param saleRateToken0 The sale rate for token0 orders
    /// @param saleRateToken1 The sale rate for token1 orders
    function _emitVirtualOrdersExecuted(PoolId poolId, uint256 saleRateToken0, uint256 saleRateToken1) internal {
        assembly ("memory-safe") {
            // by writing it backwards, we overwrite only the empty bits with each subsequent write
            // 28-60, only 46-60 can be non-zero
            mstore(28, saleRateToken1)
            // 14-46, only 32-46 can be non-zero
            mstore(14, saleRateToken0)
            mstore(0, poolId)

            log0(0, 60)
        }
    }

    /// @inheritdoc ITWAMM
    function getRewardRateInside(PoolId poolId, OrderConfig config) public view returns (uint256 result) {
        if (block.timestamp >= config.endTime()) {
            uint256 offset = LibBit.rawToUint(!config.isToken1());
            uint256 rewardRateStart =
                uint256(TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, config.startTime()).add(offset).load());

            uint256 rewardRateEnd =
                uint256(TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, config.endTime()).add(offset).load());

            unchecked {
                result = rewardRateEnd - rewardRateStart;
            }
        } else if (block.timestamp > config.startTime()) {
            uint256 offset = LibBit.rawToUint(!config.isToken1());

            //  note that we check gt because if it's equal to start time, then the reward rate inside is necessarily 0
            uint256 rewardRateStart =
                uint256(TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, config.startTime()).add(offset).load());
            uint256 rewardRateCurrent = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).add(offset).load());

            unchecked {
                result = rewardRateCurrent - rewardRateStart;
            }
        } else {
            // less than or equal to start time
            // returns 0
        }
    }

    /// @notice Safely adds a change to a sale rate delta with overflow protection
    /// @dev Ensures the resulting sale rate delta doesn't exceed the maximum allowed value
    /// @param saleRateDelta The current sale rate delta
    /// @param saleRateDeltaChange The change to apply to the sale rate delta
    /// @return saleRateDeltaNext The new sale rate delta after applying the change
    function _addConstrainSaleRateDelta(int112 saleRateDelta, int256 saleRateDeltaChange)
        internal
        pure
        returns (int112 saleRateDeltaNext)
    {
        int256 result = int256(saleRateDelta) + saleRateDeltaChange;

        // checked addition, no overflow of int112 type
        if (FixedPointMathLib.abs(result) > MAX_ABS_VALUE_SALE_RATE_DELTA) {
            revert MaxSaleRateDeltaPerTime();
        }

        // we know cast is safe because abs(result) is less than MAX_ABS_VALUE_SALE_RATE_DELTA which fits in a int112
        saleRateDeltaNext = int112(result);
    }

    /// @notice Updates time-specific information for TWAMM orders
    /// @dev Manages the sale rate deltas and order counts for a specific time point
    /// @param poolId The unique identifier for the pool
    /// @param time The timestamp to update
    /// @param saleRateDelta The change in sale rate for this time
    /// @param isToken1 True if updating token1 sale rate, false for token0
    /// @param numOrdersChange The change in number of orders referencing this time
    function _updateTime(PoolId poolId, uint256 time, int256 saleRateDelta, bool isToken1, int256 numOrdersChange)
        internal
    {
        TimeInfo timeInfo = TimeInfo.wrap(TWAMMStorageLayout.poolTimeInfosSlot(poolId, time).load());
        (uint32 numOrders, int112 saleRateDeltaToken0, int112 saleRateDeltaToken1) = timeInfo.parse();

        // note we assume this will never overflow, since it would require 2**32 separate orders to be placed
        uint32 numOrdersNext;
        assembly ("memory-safe") {
            numOrdersNext := add(numOrders, numOrdersChange)
            if gt(numOrdersNext, 0xffffffff) {
                // cast sig "TimeNumOrdersOverflow()"
                mstore(0, shl(224, 0x6916a952))
                revert(0, 4)
            }
        }

        bool flip = (numOrders == 0) != (numOrdersNext == 0);

        // write the poolRewardRatesBefore[poolId][time] = (1,1) if any orders still reference the time, or write (0,0) otherwise
        // we assume `_updateTime` is being called only for times that are greater than block.timestamp, i.e. have not been crossed yet
        // this reduces the cost of crossing that timestamp to a warm write instead of a cold write
        if (flip) {
            bytes32 zeroNumOrders = bytes32(LibBit.rawToUint(numOrders == 0));

            TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, time).storeTwo(zeroNumOrders, zeroNumOrders);

            flipTime(TWAMMStorageLayout.poolInitializedTimesBitmapSlot(poolId), time);
        }

        if (isToken1) {
            saleRateDeltaToken1 = _addConstrainSaleRateDelta(saleRateDeltaToken1, saleRateDelta);
        } else {
            saleRateDeltaToken0 = _addConstrainSaleRateDelta(saleRateDeltaToken0, saleRateDelta);
        }

        TWAMMStorageLayout.poolTimeInfosSlot(poolId, time)
            .store(TimeInfo.unwrap(createTimeInfo(numOrdersNext, saleRateDeltaToken0, saleRateDeltaToken1)));
    }

    /// @notice Returns the call points configuration for this extension
    /// @dev Overrides the base implementation to return TWAMM-specific call points
    /// @return The call points configuration
    function getCallPoints() internal pure override returns (CallPoints memory) {
        return twammCallPoints();
    }

    ///////////////////////// Callbacks /////////////////////////

    function handleForwardData(Locker original, bytes memory data) internal override returns (bytes memory result) {
        unchecked {
            uint256 callType = abi.decode(data, (uint256));
            address owner = original.addr();

            if (callType == 0) {
                (, bytes32 salt, OrderKey memory orderKey, int112 saleRateDelta) =
                    abi.decode(data, (uint256, bytes32, OrderKey, int112));

                (uint64 startTime, uint64 endTime) = (orderKey.config.startTime(), orderKey.config.endTime());

                if (endTime <= block.timestamp) revert OrderAlreadyEnded();

                if (
                    !isTimeValid(block.timestamp, startTime) || !isTimeValid(block.timestamp, endTime)
                        || startTime >= endTime
                ) {
                    revert InvalidTimestamps();
                }

                PoolKey memory poolKey = orderKey.toPoolKey(address(this));
                PoolId poolId = poolKey.toPoolId();
                _executeVirtualOrdersFromWithinLock(poolKey, poolId);

                OrderId orderId = orderKey.toOrderId();

                StorageSlot orderStateSlot =
                    TWAMMStorageLayout.orderStateSlotFollowedByOrderRewardRateSnapshotSlot(owner, salt, orderId);

                StorageSlot orderRewardRateSnapshotSlot = orderStateSlot.next();

                OrderState order = OrderState.wrap(orderStateSlot.load());
                uint256 rewardRateSnapshot = uint256(orderRewardRateSnapshotSlot.load());

                uint256 rewardRateInside = getRewardRateInside(poolId, orderKey.config);

                (uint32 lastUpdateTime, uint112 saleRate, uint112 amountSold) = order.parse();

                uint256 purchasedAmount = computeRewardAmount(rewardRateInside - rewardRateSnapshot, saleRate);

                uint256 saleRateNext = addSaleRateDelta(saleRate, saleRateDelta);

                uint256 rewardRateSnapshotAdjusted;
                int256 numOrdersChange;
                assembly ("memory-safe") {
                    rewardRateSnapshotAdjusted := mul(
                        sub(rewardRateInside, div(shl(128, purchasedAmount), saleRateNext)),
                        // if saleRateNext is zero, write 0 for the reward rate snapshot adjusted
                        iszero(iszero(saleRateNext))
                    )

                    // if current is zero, and next is zero, then 1-1 = 0
                    // if current is nonzero, and next is nonzero, then 0-0 = 0
                    // if current is zero, and next is nonzero, then we get 1-0 = 1
                    // if current is nonzero, and next is zero, then we get 0-1 = -1 = (type(uint256).max)
                    numOrdersChange := sub(iszero(saleRate), iszero(saleRateNext))
                }

                orderStateSlot.store(
                    OrderState.unwrap(
                        createOrderState({
                            _lastUpdateTime: uint32(block.timestamp),
                            _saleRate: uint112(saleRateNext),
                            _amountSold: uint112(
                                amountSold
                                    + computeAmountFromSaleRate({
                                        saleRate: saleRate,
                                        duration: FixedPointMathLib.min(
                                            uint32(block.timestamp) - lastUpdateTime,
                                            uint32(uint64(block.timestamp) - startTime)
                                        ),
                                        roundUp: false
                                    })
                            )
                        })
                    )
                );
                orderRewardRateSnapshotSlot.store(bytes32(rewardRateSnapshotAdjusted));

                bool isToken1 = orderKey.config.isToken1();

                if (block.timestamp < startTime) {
                    _updateTime(poolId, startTime, saleRateDelta, isToken1, numOrdersChange);
                    _updateTime(poolId, endTime, -int256(saleRateDelta), isToken1, numOrdersChange);
                } else {
                    // we know block.timestamp < orderKey.endTime because we validate that first
                    // and we know the order is active, so we have to apply its delta to the current pool state
                    StorageSlot currentStateSlot = TWAMMStorageLayout.twammPoolStateSlot(poolId);
                    TwammPoolState currentState = TwammPoolState.wrap(currentStateSlot.load());
                    (uint32 lastTime, uint112 rate0, uint112 rate1) = currentState.parse();

                    if (isToken1) {
                        currentState = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: lastTime,
                            _saleRateToken0: rate0,
                            _saleRateToken1: uint112(addSaleRateDelta(rate1, saleRateDelta))
                        });
                    } else {
                        currentState = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: lastTime,
                            _saleRateToken0: uint112(addSaleRateDelta(rate0, saleRateDelta)),
                            _saleRateToken1: rate1
                        });
                    }

                    currentStateSlot.store(TwammPoolState.unwrap(currentState));

                    // only update the end time
                    _updateTime(poolId, endTime, -int256(saleRateDelta), isToken1, numOrdersChange);
                }

                // we know this will fit in a uint32 because otherwise isValidTime would fail for the end time
                uint256 durationRemaining = endTime - FixedPointMathLib.max(block.timestamp, startTime);

                // the amount required for executing at the next sale rate for the remaining duration of the order
                uint256 amountRequired =
                    computeAmountFromSaleRate({saleRate: saleRateNext, duration: durationRemaining, roundUp: true});

                // subtract the remaining sell amount to get the delta
                int256 amountDelta;

                uint256 remainingSellAmount =
                    computeAmountFromSaleRate({saleRate: saleRate, duration: durationRemaining, roundUp: true});

                assembly ("memory-safe") {
                    amountDelta := sub(amountRequired, remainingSellAmount)
                }

                // user is withdrawing tokens, so they need to pay a fee to the liquidity providers
                if (amountDelta < 0) {
                    // negation and downcast will never overflow, since max sale rate times max duration is at most type(uint112).max
                    uint128 fee = computeFee(uint128(uint256(-amountDelta)), poolKey.config.fee());
                    if (isToken1) {
                        CORE.accumulateAsFees(poolKey, 0, fee);
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), 0, amountDelta);
                    } else {
                        CORE.accumulateAsFees(poolKey, fee, 0);
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), amountDelta, 0);
                    }

                    amountDelta += int128(fee);
                } else {
                    if (isToken1) {
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), 0, amountDelta);
                    } else {
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), amountDelta, 0);
                    }
                }

                emit OrderUpdated(owner, salt, orderKey, saleRateDelta);

                result = abi.encode(amountDelta);
            } else if (callType == 1) {
                (, bytes32 salt, OrderKey memory orderKey) = abi.decode(data, (uint256, bytes32, OrderKey));

                PoolKey memory poolKey = orderKey.toPoolKey(address(this));
                PoolId poolId = poolKey.toPoolId();
                _executeVirtualOrdersFromWithinLock(poolKey, poolId);

                OrderId orderId = orderKey.toOrderId();

                StorageSlot orderStateSlot =
                    TWAMMStorageLayout.orderStateSlotFollowedByOrderRewardRateSnapshotSlot(owner, salt, orderId);

                StorageSlot orderRewardRateSnapshotSlot = orderStateSlot.next();

                OrderState order = OrderState.wrap(orderStateSlot.load());
                uint256 rewardRateSnapshot = uint256(orderRewardRateSnapshotSlot.load());

                uint256 rewardRateInside = getRewardRateInside(poolId, orderKey.config);

                uint256 purchasedAmount = computeRewardAmount(rewardRateInside - rewardRateSnapshot, order.saleRate());

                orderRewardRateSnapshotSlot.store(bytes32(rewardRateInside));

                if (purchasedAmount != 0) {
                    if (orderKey.config.isToken1()) {
                        CORE.updateSavedBalances(
                            poolKey.token0, poolKey.token1, bytes32(0), -int256(purchasedAmount), 0
                        );
                    } else {
                        CORE.updateSavedBalances(
                            poolKey.token0, poolKey.token1, bytes32(0), 0, -int256(purchasedAmount)
                        );
                    }
                }

                emit OrderProceedsWithdrawn(original.addr(), salt, orderKey, uint128(purchasedAmount));

                result = abi.encode(purchasedAmount);
            } else {
                revert();
            }
        }
    }

    function _executeVirtualOrdersFromWithinLock(PoolKey memory poolKey, PoolId poolId) internal {
        unchecked {
            StorageSlot stateSlot = TWAMMStorageLayout.twammPoolStateSlot(poolId);
            TwammPoolState state = TwammPoolState.wrap(stateSlot.load());

            // we only conditionally load this if the state is coincidentally zero,
            // in order to not lock the pool if state is 0 but the pool _is_ initialized
            // this can only happen iff a pool has zero sale rates **and** an execution of virtual orders
            // happens on the uint32 boundary
            if (TwammPoolState.unwrap(state) == bytes32(0)) {
                if (poolKey.config.extension() != address(this) || !CORE.poolState(poolId).isInitialized()) {
                    revert PoolNotInitialized();
                }
            }

            uint256 realLastVirtualOrderExecutionTime = state.realLastVirtualOrderExecutionTime();

            // no-op if already executed in this block
            if (realLastVirtualOrderExecutionTime != block.timestamp) {
                // initialize the values that are handled once per execution
                FeesPerLiquidity memory rewardRates;

                // 0 = not loaded & not updated, 1 = loaded & not updated, 2 = loaded & updated
                uint256 rewardRate0Access;
                uint256 rewardRate1Access;

                int256 saveDelta0;
                int256 saveDelta1;
                PoolState corePoolState;
                uint256 time = realLastVirtualOrderExecutionTime;

                while (time != block.timestamp) {
                    StorageSlot initializedTimesBitmapSlot = TWAMMStorageLayout.poolInitializedTimesBitmapSlot(poolId);

                    (uint256 nextTime, bool initialized) = searchForNextInitializedTime({
                        slot: initializedTimesBitmapSlot,
                        lastVirtualOrderExecutionTime: realLastVirtualOrderExecutionTime,
                        fromTime: time,
                        untilTime: block.timestamp
                    });

                    // it is assumed that this will never return a value greater than type(uint32).max
                    uint256 timeElapsed = nextTime - time;

                    uint256 amount0 = computeAmountFromSaleRate({
                        saleRate: state.saleRateToken0(), duration: timeElapsed, roundUp: false
                    });

                    uint256 amount1 = computeAmountFromSaleRate({
                        saleRate: state.saleRateToken1(), duration: timeElapsed, roundUp: false
                    });

                    int256 rewardDelta0;
                    int256 rewardDelta1;
                    // if both sale rates are non-zero but amounts are zero, we will end up doing the math for no reason since we swap 0
                    if (amount0 != 0 && amount1 != 0) {
                        if (!corePoolState.isInitialized()) {
                            corePoolState = CORE.poolState(poolId);
                        }
                        SqrtRatio sqrtRatioNext = computeNextSqrtRatio({
                            sqrtRatio: corePoolState.sqrtRatio(),
                            liquidity: corePoolState.liquidity(),
                            saleRateToken0: state.saleRateToken0(),
                            saleRateToken1: state.saleRateToken1(),
                            timeElapsed: timeElapsed,
                            fee: poolKey.config.fee()
                        });

                        PoolBalanceUpdate swapBalanceUpdate;
                        if (sqrtRatioNext > corePoolState.sqrtRatio()) {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: sqrtRatioNext,
                                    _amount: int128(uint128(amount1)),
                                    _isToken1: true,
                                    _skipAhead: 0
                                })
                            );
                        } else if (sqrtRatioNext < corePoolState.sqrtRatio()) {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: sqrtRatioNext,
                                    _amount: int128(uint128(amount0)),
                                    _isToken1: false,
                                    _skipAhead: 0
                                })
                            );
                        }

                        saveDelta0 -= swapBalanceUpdate.delta0();
                        saveDelta1 -= swapBalanceUpdate.delta1();

                        // this cannot overflow or underflow because swapDelta0 is constrained to int128,
                        // and amounts computed from uint112 sale rates cannot exceed uint112.max
                        rewardDelta0 = swapBalanceUpdate.delta0() - int256(uint256(amount0));
                        rewardDelta1 = swapBalanceUpdate.delta1() - int256(uint256(amount1));
                    } else if (amount0 != 0 || amount1 != 0) {
                        PoolBalanceUpdate swapBalanceUpdate;
                        if (amount0 != 0) {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: MIN_SQRT_RATIO,
                                    _amount: int128(uint128(amount0)),
                                    _isToken1: false,
                                    _skipAhead: 0
                                })
                            );
                        } else {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: MAX_SQRT_RATIO,
                                    _amount: int128(uint128(amount1)),
                                    _isToken1: true,
                                    _skipAhead: 0
                                })
                            );
                        }

                        (rewardDelta0, rewardDelta1) = (swapBalanceUpdate.delta0(), swapBalanceUpdate.delta1());
                        saveDelta0 -= rewardDelta0;
                        saveDelta1 -= rewardDelta1;
                    }

                    if (rewardDelta0 < 0) {
                        if (rewardRate0Access == 0) {
                            rewardRates.value0 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).load());
                        }
                        rewardRate0Access = 2;
                        rewardRates.value0 += FixedPointMathLib.rawDiv(
                            uint256(-rewardDelta0) << 128, state.saleRateToken1()
                        );
                    }

                    if (rewardDelta1 < 0) {
                        if (rewardRate1Access == 0) {
                            rewardRates.value1 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).next().load());
                        }
                        rewardRate1Access = 2;
                        rewardRates.value1 += FixedPointMathLib.rawDiv(
                            uint256(-rewardDelta1) << 128, state.saleRateToken0()
                        );
                    }

                    if (initialized) {
                        if (rewardRate0Access == 0) {
                            rewardRates.value0 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).load());
                            rewardRate0Access = 1;
                        }
                        if (rewardRate1Access == 0) {
                            rewardRates.value1 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).next().load());
                            rewardRate1Access = 1;
                        }

                        TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, nextTime)
                            .storeTwo(bytes32(rewardRates.value0), bytes32(rewardRates.value1));

                        StorageSlot timeInfoSlot = TWAMMStorageLayout.poolTimeInfosSlot(poolId, nextTime);
                        (, int112 saleRateDeltaToken0, int112 saleRateDeltaToken1) =
                            TimeInfo.wrap(timeInfoSlot.load()).parse();

                        state = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: uint32(nextTime),
                            _saleRateToken0: uint112(addSaleRateDelta(state.saleRateToken0(), saleRateDeltaToken0)),
                            _saleRateToken1: uint112(addSaleRateDelta(state.saleRateToken1(), saleRateDeltaToken1))
                        });

                        // this time is _consumed_, will never be crossed again, so we delete the info we no longer need.
                        // this helps reduce the cost of executing virtual orders.
                        timeInfoSlot.store(0);

                        flipTime(initializedTimesBitmapSlot, nextTime);
                    } else {
                        state = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: uint32(nextTime),
                            _saleRateToken0: state.saleRateToken0(),
                            _saleRateToken1: state.saleRateToken1()
                        });
                    }

                    time = nextTime;
                }

                if (saveDelta0 != 0 || saveDelta1 != 0) {
                    CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), saveDelta0, saveDelta1);
                }

                if (rewardRate0Access == 2) {
                    TWAMMStorageLayout.poolRewardRatesSlot(poolId).store(bytes32(rewardRates.value0));
                }
                if (rewardRate1Access == 2) {
                    TWAMMStorageLayout.poolRewardRatesSlot(poolId).next().store(bytes32(rewardRates.value1));
                }

                stateSlot.store(TwammPoolState.unwrap(state));

                _emitVirtualOrdersExecuted(poolId, state.saleRateToken0(), state.saleRateToken1());
            }
        }
    }

    // Executes virtual orders for the specified initialized pool key. Protected because it is only called by core.
    function locked_6416899205(uint256) external override onlyCore {
        PoolKey memory poolKey;
        assembly ("memory-safe") {
            // copy the poolkey out of calldata at the solidity-allocated address
            calldatacopy(poolKey, 36, 96)
        }
        _executeVirtualOrdersFromWithinLock(poolKey, poolKey.toPoolId());
    }

    /// @inheritdoc ITWAMM
    function lockAndExecuteVirtualOrders(PoolKey memory poolKey) public {
        // the only thing we lock for is executing virtual orders, so all we need to encode is the pool key
        // so we call lock on the core contract with the pool key after it
        address target = address(CORE);
        assembly ("memory-safe") {
            let o := mload(0x40)
            mstore(o, shl(224, 0xf83d08ba))
            mcopy(add(o, 4), poolKey, 96)

            // If the call failed, pass through the revert
            if iszero(call(gas(), target, 0, o, 100, 0, 0)) {
                returndatacopy(o, 0, returndatasize())
                revert(o, returndatasize())
            }
        }
    }

    ///////////////////////// Extension call points /////////////////////////

    // This method must be protected because it sets state directly
    function afterInitializePool(address, PoolKey memory key, int32, SqrtRatio)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (!key.config.isFullRange()) revert FullRangePoolOnly();

        PoolId poolId = key.toPoolId();

        TWAMMStorageLayout.twammPoolStateSlot(poolId)
            .store(
                TwammPoolState.unwrap(
                    createTwammPoolState({
                        _lastVirtualOrderExecutionTime: uint32(block.timestamp), _saleRateToken0: 0, _saleRateToken1: 0
                    })
                )
            );

        _emitVirtualOrdersExecuted({poolId: poolId, saleRateToken0: 0, saleRateToken1: 0});
    }

    // Since anyone can call the method `#lockAndExecuteVirtualOrders`, the method is not protected
    function beforeSwap(Locker, PoolKey memory poolKey, SwapParameters) external override(BaseExtension, IExtension) {
        lockAndExecuteVirtualOrders(poolKey);
    }

    // Since anyone can call the method `#lockAndExecuteVirtualOrders`, the method is not protected
    function beforeUpdatePosition(Locker, PoolKey memory poolKey, PositionId, int128)
        external
        override(BaseExtension, IExtension)
    {
        lockAndExecuteVirtualOrders(poolKey);
    }

    // Since anyone can call the method `#lockAndExecuteVirtualOrders`, the method is not protected
    function beforeCollectFees(Locker, PoolKey memory poolKey, PositionId)
        external
        override(BaseExtension, IExtension)
    {
        lockAndExecuteVirtualOrders(poolKey);
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "../../types/poolKey.sol";
import {ILocker, IForwardee} from "../IFlashAccountant.sol";
import {IExtension} from "../ICore.sol";
import {IExposedStorage} from "../IExposedStorage.sol";

/// @title MEV Capture Interface
/// @notice Interface for the Ekubo MEV Capture Extension
/// @dev Extension that charges additional fees based on the relative size of the priority fee and tick movement during swaps
interface IMEVCapture is IExposedStorage, ILocker, IForwardee, IExtension {
    /// @notice Thrown when trying to use MEV capture on a full-range-only pool
    /// @dev MEV capture only works with concentrated liquidity pools that have discrete tick spacing
    error ConcentratedLiquidityPoolsOnly();

    /// @notice Thrown when trying to use MEV capture on a pool with zero fees
    /// @dev MEV capture multiplies the base fee, so a zero fee would result in no additional fees
    error NonzeroFeesOnly();

    /// @notice Thrown when attempting to swap directly without using the forward mechanism
    /// @dev All swaps must go through the forward mechanism to ensure proper MEV fee calculation
    error SwapMustHappenThroughForward();

    /// @notice Accumulates any pending pool fees from past blocks
    /// @dev This function can be called by anyone to trigger fee accumulation for a pool
    /// @dev Fees are accumulated when the pool hasn't been updated in the current block
    /// @param poolKey The pool key identifying the pool to accumulate fees for
    function accumulatePoolFees(PoolKey memory poolKey) external;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {Multicallable} from "solady/utils/Multicallable.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

/// @title Payable Multicallable
/// @notice Abstract contract that extends Multicallable to support payable multicalls and ETH refunds
/// @dev Provides functionality for batching multiple calls with native token support
///      Derived contracts can use this to enable efficient batch operations with ETH payments
abstract contract PayableMulticallable is Multicallable {
    /// @notice Executes multiple calls in a single transaction with native token support
    /// @dev Overrides the base multicall function to make it payable, allowing ETH to be sent
    ///      Uses direct return to avoid unnecessary memory copying for gas efficiency
    /// @param data Array of encoded function call data to execute
    /// @return results Array of return data from each function call
    function multicall(bytes[] calldata data) public payable override returns (bytes[] memory) {
        _multicallDirectReturn(_multicall(data));
    }

    /// @notice Refunds any remaining native token balance to the caller
    /// @dev Allows callers to recover ETH that was sent for transient payments but not fully consumed
    ///      This is useful when exact payment amounts are difficult to calculate in advance
    ///      Only refunds if there is a non-zero balance to avoid unnecessary gas costs
    function refundNativeToken() external payable {
        if (address(this).balance != 0) {
            SafeTransferLib.safeTransferETH(msg.sender, address(this).balance);
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {BaseLocker} from "./BaseLocker.sol";
import {UsesCore} from "./UsesCore.sol";
import {ICore} from "../interfaces/ICore.sol";
import {IPositions} from "../interfaces/IPositions.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {FlashAccountantLib} from "../libraries/FlashAccountantLib.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId, createPositionId} from "../types/positionId.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {Position} from "../types/position.sol";
import {tickToSqrtRatio} from "../math/ticks.sol";
import {maxLiquidity, liquidityDeltaToAmountDelta} from "../math/liquidity.sol";
import {PayableMulticallable} from "./PayableMulticallable.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {BaseNonfungibleToken} from "./BaseNonfungibleToken.sol";
import {NATIVE_TOKEN_ADDRESS} from "../math/constants.sol";
import {PoolId} from "../types/poolId.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

/// @title Base Positions Contract
/// @author Moody Salem <moody@ekubo.org>
/// @notice Abstract base contract for tracking liquidity positions in Ekubo Protocol as NFTs
/// @dev Provides core position management functionality with abstract protocol fee collection methods
abstract contract BasePositions is IPositions, UsesCore, PayableMulticallable, BaseLocker, BaseNonfungibleToken {
    using CoreLib for *;
    using FlashAccountantLib for *;

    /// @notice Constructs the BasePositions contract
    /// @param core The core contract instance
    /// @param owner The owner of the contract (for access control)
    constructor(ICore core, address owner) BaseNonfungibleToken(owner) BaseLocker(core) UsesCore(core) {}

    uint256 private constant CALL_TYPE_DEPOSIT = 0;
    uint256 private constant CALL_TYPE_WITHDRAW = 1;
    uint256 private constant CALL_TYPE_WITHDRAW_PROTOCOL_FEES = 2;

    /// @inheritdoc IPositions
    function getPositionFeesAndLiquidity(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        external
        view
        returns (uint128 liquidity, uint128 principal0, uint128 principal1, uint128 fees0, uint128 fees1)
    {
        PoolId poolId = poolKey.toPoolId();
        SqrtRatio sqrtRatio = CORE.poolState(poolId).sqrtRatio();
        PositionId positionId =
            createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper});
        Position memory position = CORE.poolPositions(poolId, address(this), positionId);

        liquidity = position.liquidity;

        // the sqrt ratio may be 0 (because the pool is uninitialized) but this is
        // fine since amount0Delta isn't called with it in this case
        (int128 delta0, int128 delta1) = liquidityDeltaToAmountDelta(
            sqrtRatio, -SafeCastLib.toInt128(position.liquidity), tickToSqrtRatio(tickLower), tickToSqrtRatio(tickUpper)
        );

        (principal0, principal1) = (uint128(-delta0), uint128(-delta1));

        FeesPerLiquidity memory feesPerLiquidityInside = poolKey.config.isFullRange()
            ? CORE.getPoolFeesPerLiquidity(poolId)
            : CORE.getPoolFeesPerLiquidityInside(poolId, tickLower, tickUpper);
        (fees0, fees1) = position.fees(feesPerLiquidityInside);
    }

    /// @inheritdoc IPositions
    function deposit(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) public payable authorizedForNft(id) returns (uint128 liquidity, uint128 amount0, uint128 amount1) {
        SqrtRatio sqrtRatio = CORE.poolState(poolKey.toPoolId()).sqrtRatio();

        liquidity =
            maxLiquidity(sqrtRatio, tickToSqrtRatio(tickLower), tickToSqrtRatio(tickUpper), maxAmount0, maxAmount1);

        if (liquidity < minLiquidity) {
            revert DepositFailedDueToSlippage(liquidity, minLiquidity);
        }

        if (liquidity > uint128(type(int128).max)) {
            revert DepositOverflow();
        }

        (amount0, amount1) = abi.decode(
            lock(abi.encode(CALL_TYPE_DEPOSIT, msg.sender, id, poolKey, tickLower, tickUpper, liquidity)),
            (uint128, uint128)
        );
    }

    /// @inheritdoc IPositions
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        public
        payable
        authorizedForNft(id)
        returns (uint128 amount0, uint128 amount1)
    {
        (amount0, amount1) = collectFees(id, poolKey, tickLower, tickUpper, msg.sender);
    }

    /// @inheritdoc IPositions
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, address recipient)
        public
        payable
        authorizedForNft(id)
        returns (uint128 amount0, uint128 amount1)
    {
        (amount0, amount1) = withdraw(id, poolKey, tickLower, tickUpper, 0, recipient, true);
    }

    /// @inheritdoc IPositions
    function withdraw(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 liquidity,
        address recipient,
        bool withFees
    ) public payable authorizedForNft(id) returns (uint128 amount0, uint128 amount1) {
        (amount0, amount1) = abi.decode(
            lock(abi.encode(CALL_TYPE_WITHDRAW, id, poolKey, tickLower, tickUpper, liquidity, recipient, withFees)),
            (uint128, uint128)
        );
    }

    /// @inheritdoc IPositions
    function withdraw(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, uint128 liquidity)
        public
        payable
        returns (uint128 amount0, uint128 amount1)
    {
        (amount0, amount1) = withdraw(id, poolKey, tickLower, tickUpper, liquidity, address(msg.sender), true);
    }

    /// @inheritdoc IPositions
    function maybeInitializePool(PoolKey memory poolKey, int32 tick)
        external
        payable
        returns (bool initialized, SqrtRatio sqrtRatio)
    {
        // the before update position hook shouldn't be taken into account here
        sqrtRatio = CORE.poolState(poolKey.toPoolId()).sqrtRatio();
        if (sqrtRatio.isZero()) {
            initialized = true;
            sqrtRatio = CORE.initializePool(poolKey, tick);
        }
    }

    /// @inheritdoc IPositions
    function mintAndDeposit(
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1) {
        id = mint();
        (liquidity, amount0, amount1) = deposit(id, poolKey, tickLower, tickUpper, maxAmount0, maxAmount1, minLiquidity);
    }

    /// @inheritdoc IPositions
    function mintAndDepositWithSalt(
        bytes32 salt,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1) {
        id = mint(salt);
        (liquidity, amount0, amount1) = deposit(id, poolKey, tickLower, tickUpper, maxAmount0, maxAmount1, minLiquidity);
    }

    /// @inheritdoc IPositions
    function withdrawProtocolFees(address token0, address token1, uint128 amount0, uint128 amount1, address recipient)
        external
        payable
        onlyOwner
    {
        lock(abi.encode(CALL_TYPE_WITHDRAW_PROTOCOL_FEES, token0, token1, amount0, amount1, recipient));
    }

    /// @inheritdoc IPositions
    function getProtocolFees(address token0, address token1) external view returns (uint128 amount0, uint128 amount1) {
        (amount0, amount1) = CORE.savedBalances(address(this), token0, token1, bytes32(0));
    }

    /// @notice Handles protocol fee collection during fee collection
    /// @dev Abstract method that must be implemented by concrete contracts
    /// @param poolKey The pool key for the position
    /// @param amount0 The amount of token0 fees collected before protocol fee deduction
    /// @param amount1 The amount of token1 fees collected before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeSwapProtocolFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1)
        internal
        view
        virtual
        returns (uint128 protocolFee0, uint128 protocolFee1);

    /// @notice Handles protocol fee collection during liquidity withdrawal
    /// @dev Abstract method that must be implemented by concrete contracts
    /// @param poolKey The pool key for the position
    /// @param amount0 The amount of token0 being withdrawn before protocol fee deduction
    /// @param amount1 The amount of token1 being withdrawn before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeWithdrawalProtocolFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1)
        internal
        view
        virtual
        returns (uint128 protocolFee0, uint128 protocolFee1);

    /// @notice Handles lock callback data for position operations
    /// @dev Internal function that processes different types of position operations
    /// @param data Encoded operation data
    /// @return result Encoded result data
    function handleLockData(uint256, bytes memory data) internal override returns (bytes memory result) {
        uint256 callType = abi.decode(data, (uint256));

        if (callType == CALL_TYPE_DEPOSIT) {
            (
                ,
                address caller,
                uint256 id,
                PoolKey memory poolKey,
                int32 tickLower,
                int32 tickUpper,
                uint128 liquidity
            ) = abi.decode(data, (uint256, address, uint256, PoolKey, int32, int32, uint128));

            PoolBalanceUpdate balanceUpdate = CORE.updatePosition(
                poolKey,
                createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper}),
                int128(liquidity)
            );

            uint128 amount0 = uint128(balanceUpdate.delta0());
            uint128 amount1 = uint128(balanceUpdate.delta1());

            // Use multi-token payment for ERC20-only pools, fall back to individual payments for native token pools
            if (poolKey.token0 != NATIVE_TOKEN_ADDRESS) {
                ACCOUNTANT.payTwoFrom(caller, poolKey.token0, poolKey.token1, amount0, amount1);
            } else {
                if (amount0 != 0) {
                    SafeTransferLib.safeTransferETH(address(ACCOUNTANT), amount0);
                }
                if (amount1 != 0) {
                    ACCOUNTANT.payFrom(caller, poolKey.token1, amount1);
                }
            }

            result = abi.encode(amount0, amount1);
        } else if (callType == CALL_TYPE_WITHDRAW) {
            (
                ,
                uint256 id,
                PoolKey memory poolKey,
                int32 tickLower,
                int32 tickUpper,
                uint128 liquidity,
                address recipient,
                bool withFees
            ) = abi.decode(data, (uint256, uint256, PoolKey, int32, int32, uint128, address, bool));

            if (liquidity > uint128(type(int128).max)) revert WithdrawOverflow();

            uint128 amount0;
            uint128 amount1;

            // collect first in case we are withdrawing the entire amount
            if (withFees) {
                (amount0, amount1) = CORE.collectFees(
                    poolKey,
                    createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper})
                );

                // Collect swap protocol fees
                (uint128 swapProtocolFee0, uint128 swapProtocolFee1) =
                    _computeSwapProtocolFees(poolKey, amount0, amount1);

                if (swapProtocolFee0 != 0 || swapProtocolFee1 != 0) {
                    CORE.updateSavedBalances(
                        poolKey.token0, poolKey.token1, bytes32(0), int128(swapProtocolFee0), int128(swapProtocolFee1)
                    );

                    amount0 -= swapProtocolFee0;
                    amount1 -= swapProtocolFee1;
                }
            }

            if (liquidity != 0) {
                PoolBalanceUpdate balanceUpdate = CORE.updatePosition(
                    poolKey,
                    createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper}),
                    -int128(liquidity)
                );

                uint128 withdrawnAmount0 = uint128(-balanceUpdate.delta0());
                uint128 withdrawnAmount1 = uint128(-balanceUpdate.delta1());

                // Collect withdrawal protocol fees
                (uint128 withdrawalFee0, uint128 withdrawalFee1) =
                    _computeWithdrawalProtocolFees(poolKey, withdrawnAmount0, withdrawnAmount1);

                if (withdrawalFee0 != 0 || withdrawalFee1 != 0) {
                    // we know cast won't overflow because delta0 and delta1 were originally int128
                    CORE.updateSavedBalances(
                        poolKey.token0, poolKey.token1, bytes32(0), int128(withdrawalFee0), int128(withdrawalFee1)
                    );
                }

                amount0 += withdrawnAmount0 - withdrawalFee0;
                amount1 += withdrawnAmount1 - withdrawalFee1;
            }

            ACCOUNTANT.withdrawTwo(poolKey.token0, poolKey.token1, recipient, amount0, amount1);

            result = abi.encode(amount0, amount1);
        } else if (callType == CALL_TYPE_WITHDRAW_PROTOCOL_FEES) {
            (, address token0, address token1, uint128 amount0, uint128 amount1, address recipient) =
                abi.decode(data, (uint256, address, address, uint128, uint128, address));

            CORE.updateSavedBalances(token0, token1, bytes32(0), -int256(uint256(amount0)), -int256(uint256(amount1)));
            ACCOUNTANT.withdrawTwo(token0, token1, recipient, amount0, amount1);
        } else {
            // Will never actually happen
            revert();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IForwardee, IFlashAccountant} from "../interfaces/IFlashAccountant.sol";
import {Locker} from "../types/locker.sol";

/// @title Base Forwardee
/// @notice Abstract base contract for contracts that need to receive forwarded calls from the flash accountant
/// @dev Provides the forwarding mechanism and delegates actual data handling to implementing contracts
abstract contract BaseForwardee is IForwardee {
    /// @notice Thrown when a function is called by an address other than the accountant
    error BaseForwardeeAccountantOnly();

    /// @notice The flash accountant contract that can forward calls to this contract
    IFlashAccountant private immutable ACCOUNTANT;

    /// @notice Constructs the BaseForwardee with a flash accountant
    /// @param _accountant The flash accountant contract that will forward calls
    constructor(IFlashAccountant _accountant) {
        ACCOUNTANT = _accountant;
    }

    /// CALLBACK HANDLERS

    /// @inheritdoc IForwardee
    /// @dev Extracts the forwarded data from calldata and delegates to handleForwardData
    /// The first 68 bytes of calldata contain the function selector (4 bytes), id (32 bytes), and originalLocker (32 bytes)
    /// All remaining calldata is treated as the forwarded data
    /// Return data from handleForwardData is returned exactly as is, with no additional encoding or decoding
    /// Reverts are also bubbled up
    function forwarded_2374103877(Locker original) external {
        if (msg.sender != address(ACCOUNTANT)) revert BaseForwardeeAccountantOnly();

        bytes memory data = msg.data[36:];

        bytes memory result = handleForwardData(original, data);

        assembly ("memory-safe") {
            // raw return whatever the handler sent
            return(add(result, 32), mload(result))
        }
    }

    /// INTERNAL FUNCTIONS

    /// @notice Handles the execution of forwarded data
    /// @dev Must be implemented by derived contracts to define forwarding behavior
    /// @param original The original locker that called forward
    /// @param data The forwarded data to process
    /// @return result The result of processing the forwarded data
    function handleForwardData(Locker original, bytes memory data) internal virtual returns (bytes memory result);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "../types/poolKey.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {IBaseNonfungibleToken} from "./IBaseNonfungibleToken.sol";

/// @title Positions Interface
/// @notice Interface for managing liquidity positions as NFTs in Ekubo Protocol
/// @dev Defines the interface for depositing, withdrawing, and collecting fees from liquidity positions
interface IPositions is IBaseNonfungibleToken {
    /// @notice Thrown when deposit fails due to insufficient liquidity for the given slippage tolerance
    /// @param liquidity The actual liquidity that would be provided
    /// @param minLiquidity The minimum liquidity required
    error DepositFailedDueToSlippage(uint128 liquidity, uint128 minLiquidity);

    /// @notice Thrown when deposit amount would cause overflow
    error DepositOverflow();

    /// @notice Thrown when the specified withdraw liquidity amount overflows type(int128).max
    error WithdrawOverflow();

    /// @notice Gets the liquidity, principal amounts, and accumulated fees for a position
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @return liquidity Current liquidity in the position
    /// @return principal0 Principal amount of token0 in the position
    /// @return principal1 Principal amount of token1 in the position
    /// @return fees0 Accumulated fees in token0
    /// @return fees1 Accumulated fees in token1
    function getPositionFeesAndLiquidity(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        external
        view
        returns (uint128 liquidity, uint128 principal0, uint128 principal1, uint128 fees0, uint128 fees1);

    /// @notice Deposits tokens into a liquidity position
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param maxAmount0 Maximum amount of token0 to deposit
    /// @param maxAmount1 Maximum amount of token1 to deposit
    /// @param minLiquidity Minimum liquidity to receive (for slippage protection)
    /// @return liquidity Amount of liquidity added to the position
    /// @return amount0 Actual amount of token0 deposited
    /// @return amount1 Actual amount of token1 deposited
    function deposit(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint128 liquidity, uint128 amount0, uint128 amount1);

    /// @notice Collects accumulated fees from a position to msg.sender
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @return amount0 Amount of token0 fees collected
    /// @return amount1 Amount of token1 fees collected
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        external
        payable
        returns (uint128 amount0, uint128 amount1);

    /// @notice Collects accumulated fees from a position to a specified recipient
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param recipient Address to receive the collected fees
    /// @return amount0 Amount of token0 fees collected
    /// @return amount1 Amount of token1 fees collected
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, address recipient)
        external
        payable
        returns (uint128 amount0, uint128 amount1);

    /// @notice Withdraws liquidity from a position
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param liquidity Amount of liquidity to withdraw
    /// @param recipient Address to receive the withdrawn tokens
    /// @param withFees Whether to also collect accumulated fees
    /// @return amount0 Amount of token0 withdrawn
    /// @return amount1 Amount of token1 withdrawn
    function withdraw(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 liquidity,
        address recipient,
        bool withFees
    ) external payable returns (uint128 amount0, uint128 amount1);

    /// @notice Withdraws liquidity from a position to msg.sender with fees
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param liquidity Amount of liquidity to withdraw
    /// @return amount0 Amount of token0 withdrawn
    /// @return amount1 Amount of token1 withdrawn
    function withdraw(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, uint128 liquidity)
        external
        payable
        returns (uint128 amount0, uint128 amount1);

    /// @notice Initializes a pool if it hasn't been initialized yet
    /// @param poolKey Pool key identifying the pool
    /// @param tick Initial tick for the pool if initialization is needed
    /// @return initialized Whether the pool was initialized by this call
    /// @return sqrtRatio The sqrt price ratio of the pool (existing or newly set)
    function maybeInitializePool(PoolKey memory poolKey, int32 tick)
        external
        payable
        returns (bool initialized, SqrtRatio sqrtRatio);

    /// @notice Mints a new NFT and deposits liquidity into it
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param maxAmount0 Maximum amount of token0 to deposit
    /// @param maxAmount1 Maximum amount of token1 to deposit
    /// @param minLiquidity Minimum liquidity to receive (for slippage protection)
    /// @return id The newly minted NFT token ID
    /// @return liquidity Amount of liquidity added to the position
    /// @return amount0 Actual amount of token0 deposited
    /// @return amount1 Actual amount of token1 deposited
    function mintAndDeposit(
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1);

    /// @notice Mints a new NFT with a specific salt and deposits liquidity into it
    /// @param salt Salt for deterministic NFT ID generation
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param maxAmount0 Maximum amount of token0 to deposit
    /// @param maxAmount1 Maximum amount of token1 to deposit
    /// @param minLiquidity Minimum liquidity to receive (for slippage protection)
    /// @return id The newly minted NFT token ID
    /// @return liquidity Amount of liquidity added to the position
    /// @return amount0 Actual amount of token0 deposited
    /// @return amount1 Actual amount of token1 deposited
    function mintAndDepositWithSalt(
        bytes32 salt,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1);

    /// @notice Withdraws accumulated protocol fees (only callable by owner)
    /// @param token0 Address of token0
    /// @param token1 Address of token1
    /// @param amount0 Amount of token0 fees to withdraw
    /// @param amount1 Amount of token1 fees to withdraw
    /// @param recipient Address to receive the protocol fees
    function withdrawProtocolFees(address token0, address token1, uint128 amount0, uint128 amount1, address recipient)
        external
        payable;

    /// @notice Gets the accumulated protocol fees for a token pair
    /// @param token0 Address of token0
    /// @param token1 Address of token1
    /// @return amount0 Amount of token0 protocol fees
    /// @return amount1 Amount of token1 protocol fees
    function getProtocolFees(address token0, address token1) external view returns (uint128 amount0, uint128 amount1);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IExposedStorage} from "../interfaces/IExposedStorage.sol";

/// @title ExposedStorage
/// @notice Abstract contract that implements storage exposure functionality
/// @dev This contract provides the implementation for the IExposedStorage interface,
///      allowing inheriting contracts to expose their storage slots via view functions.
///      Uses inline assembly for efficient storage access.
abstract contract ExposedStorage is IExposedStorage {
    /// @inheritdoc IExposedStorage
    /// @dev Uses inline assembly to efficiently read multiple storage slots specified in calldata.
    ///      Each slot value is loaded using the SLOAD opcode and stored in memory.
    function sload() external view {
        assembly ("memory-safe") {
            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 32) } { mstore(sub(i, 4), sload(calldataload(i))) }
            return(0, sub(calldatasize(), 4))
        }
    }

    /// @inheritdoc IExposedStorage
    /// @dev Uses inline assembly to efficiently read multiple transient storage slots specified in calldata.
    ///      Each slot value is loaded using the TLOAD opcode and stored in memory.
    function tload() external view {
        assembly ("memory-safe") {
            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 32) } { mstore(sub(i, 4), tload(calldataload(i))) }
            return(0, sub(calldatasize(), 4))
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {Ownable} from "solady/auth/Ownable.sol";
import {Multicallable} from "solady/utils/Multicallable.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {nextValidTime} from "./math/time.sol";
import {IOrders} from "./interfaces/IOrders.sol";
import {IRevenueBuybacks} from "./interfaces/IRevenueBuybacks.sol";
import {BuybacksState, createBuybacksState} from "./types/buybacksState.sol";
import {OrderKey} from "./types/orderKey.sol";
import {createOrderConfig} from "./types/orderConfig.sol";
import {ExposedStorage} from "./base/ExposedStorage.sol";
import {NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";

/// @title Revenue Buybacks
/// @author Moody Salem <moody@ekubo.org>
/// @notice Creates automated revenue buyback orders using TWAMM (Time-Weighted Average Market Maker)
/// @dev Final contract that manages the creation and execution of buyback orders for protocol revenue
/// This contract automatically creates TWAMM orders to buy back a specified token using collected revenue
contract RevenueBuybacks is IRevenueBuybacks, ExposedStorage, Ownable, Multicallable {
    /// @notice The Orders contract used to create and manage TWAMM orders
    /// @dev All buyback orders are created through this contract
    IOrders public immutable ORDERS;

    /// @notice The NFT token ID that represents all buyback orders created by this contract
    /// @dev A single NFT is minted and reused for all buyback orders to simplify management
    uint256 public immutable NFT_ID;

    /// @notice The token that is purchased with collected revenue
    /// @dev This is typically the protocol's governance or utility token
    address public immutable BUY_TOKEN;

    /// @notice Constructs the RevenueBuybacks contract
    /// @param owner The address that will own this contract and have administrative privileges
    /// @param _orders The Orders contract instance for creating TWAMM orders
    /// @param _buyToken The token that will be purchased with collected revenue
    constructor(address owner, IOrders _orders, address _buyToken) {
        _initializeOwner(owner);
        ORDERS = _orders;
        BUY_TOKEN = _buyToken;
        NFT_ID = ORDERS.mint();
    }

    /// @notice Approves the Orders contract to spend unlimited amounts of a token
    /// @dev Must be called at least once for each revenue token before creating buyback orders
    /// @param token The token to approve for spending by the Orders contract
    function approveMax(address token) external {
        SafeTransferLib.safeApproveWithRetry(token, address(ORDERS), type(uint256).max);
    }

    /// @notice Withdraws leftover tokens from the contract (only callable by owner)
    /// @dev Used to recover tokens that may be stuck in the contract
    /// @param token The address of the token to withdraw
    /// @param amount The amount of tokens to withdraw
    function take(address token, uint256 amount) external onlyOwner {
        // Transfer to msg.sender since only the owner can call this function
        SafeTransferLib.safeTransfer(token, msg.sender, amount);
    }

    /// @notice Withdraws native tokens held by this contract
    /// @dev Used to recover native tokens that may be stuck in the contract
    /// @param amount The amount of native tokens to withdraw
    function takeNative(uint256 amount) external onlyOwner {
        // Transfer to msg.sender since only the owner can call this function
        SafeTransferLib.safeTransferETH(msg.sender, amount);
    }

    /// @notice Collects the proceeds from a completed buyback order
    /// @dev Can be called by anyone at any time to collect proceeds from orders that have finished
    /// @param token The revenue token that was sold in the order
    /// @param fee The fee tier of the pool where the order was executed
    /// @param endTime The end time of the order to collect proceeds from
    /// @return proceeds The amount of buyToken received from the completed order
    function collect(address token, uint64 fee, uint64 endTime) external returns (uint128 proceeds) {
        proceeds = ORDERS.collectProceeds(NFT_ID, _createOrderKey(token, fee, 0, endTime), owner());
    }

    /// @notice Allows the contract to receive ETH revenue
    /// @dev Required to accept ETH payments when ETH is used as a revenue token
    receive() external payable {}

    /// @notice Creates a new buyback order or extends an existing one with available revenue
    /// @dev Can be called by anyone to trigger the creation of buyback orders using collected revenue
    /// This function will either extend the current order (if conditions are met) or create a new order
    /// @param token The revenue token to use for creating the buyback order, or NATIVE_TOKEN_ADDRESS
    /// @return endTime The end time of the order that was created or extended
    /// @return saleRate The sale rate of the order (amount of token sold per second)
    function roll(address token) public returns (uint64 endTime, uint112 saleRate) {
        unchecked {
            BuybacksState state;
            assembly ("memory-safe") {
                state := sload(token)
            }

            if (!state.isConfigured()) {
                revert TokenNotConfigured(token);
            }

            // minOrderDuration == 0 indicates the token is not configured
            bool isEth = token == NATIVE_TOKEN_ADDRESS;
            uint256 amountToSpend = isEth ? address(this).balance : SafeTransferLib.balanceOf(token, address(this));

            uint32 timeRemaining = state.lastEndTime() - uint32(block.timestamp);
            // if the fee changed, or the amount of time exceeds the min order duration
            // note the time remaining can underflow if the last order has ended. in this case time remaining will be greater than min order duration,
            // but also greater than last order duration, so it will not be re-used.
            if (
                state.fee() == state.lastFee() && timeRemaining >= state.minOrderDuration()
                    && timeRemaining <= state.lastOrderDuration()
            ) {
                // handles overflow
                endTime = uint64(block.timestamp + timeRemaining);
            } else {
                endTime =
                    uint64(nextValidTime(block.timestamp, block.timestamp + uint256(state.targetOrderDuration()) - 1));

                state = createBuybacksState({
                    _targetOrderDuration: state.targetOrderDuration(),
                    _minOrderDuration: state.minOrderDuration(),
                    _fee: state.fee(),
                    _lastEndTime: uint32(endTime),
                    _lastOrderDuration: uint32(endTime - block.timestamp),
                    _lastFee: state.fee()
                });

                assembly ("memory-safe") {
                    sstore(token, state)
                }
            }

            if (amountToSpend != 0) {
                saleRate = ORDERS.increaseSellAmount{value: isEth ? amountToSpend : 0}(
                    NFT_ID, _createOrderKey(token, state.fee(), 0, endTime), uint128(amountToSpend), type(uint112).max
                );
            }
        }
    }

    /// @notice Configures buyback parameters for a revenue token (only callable by owner)
    /// @dev Sets the timing and fee parameters for automated buyback order creation
    /// @param token The revenue token to configure
    /// @param targetOrderDuration The target duration for new orders (in seconds)
    /// @param minOrderDuration The minimum duration threshold for creating new orders (in seconds)
    /// @param fee The fee tier for the buyback pool
    function configure(address token, uint32 targetOrderDuration, uint32 minOrderDuration, uint64 fee)
        external
        onlyOwner
    {
        if (minOrderDuration > targetOrderDuration) revert MinOrderDurationGreaterThanTargetOrderDuration();
        if (minOrderDuration == 0 && targetOrderDuration != 0) {
            revert MinOrderDurationMustBeGreaterThanZero();
        }

        BuybacksState state;
        assembly ("memory-safe") {
            state := sload(token)
        }
        state = createBuybacksState({
            _targetOrderDuration: targetOrderDuration,
            _minOrderDuration: minOrderDuration,
            _fee: fee,
            _lastEndTime: state.lastEndTime(),
            _lastOrderDuration: state.lastOrderDuration(),
            _lastFee: state.lastFee()
        });
        assembly ("memory-safe") {
            sstore(token, state)
        }

        emit Configured(token, state);
    }

    function _createOrderKey(address token, uint64 fee, uint64 startTime, uint64 endTime)
        internal
        view
        returns (OrderKey memory key)
    {
        bool isToken1 = token > BUY_TOKEN;
        address buyToken = BUY_TOKEN;
        assembly ("memory-safe") {
            mstore(add(key, mul(isToken1, 32)), token)
            mstore(add(key, mul(iszero(isToken1), 32)), buyToken)
        }

        key.config = createOrderConfig({_fee: fee, _isToken1: isToken1, _startTime: startTime, _endTime: endTime});
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints} from "../types/callPoints.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {IExposedStorage} from "../interfaces/IExposedStorage.sol";
import {IFlashAccountant} from "../interfaces/IFlashAccountant.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {PoolState} from "../types/poolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {PoolId} from "../types/poolId.sol";
import {Locker} from "../types/locker.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

/// @title Extension Interface
/// @notice Interface for pool extensions that can hook into core operations
/// @dev Extensions must register with the core contract and implement these hooks
interface IExtension {
    /// @notice Called before a pool is initialized
    /// @param caller Address that initiated the pool initialization
    /// @param key Pool key identifying the pool
    /// @param tick Initial tick for the pool
    function beforeInitializePool(address caller, PoolKey calldata key, int32 tick) external;

    /// @notice Called after a pool is initialized
    /// @param caller Address that initiated the pool initialization
    /// @param key Pool key identifying the pool
    /// @param tick Initial tick for the pool
    /// @param sqrtRatio Initial sqrt price ratio for the pool
    function afterInitializePool(address caller, PoolKey calldata key, int32 tick, SqrtRatio sqrtRatio) external;

    /// @notice Called before a position is updated
    /// @param locker The current holder of the lock performing the position update
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position that is being updated
    /// @param liquidityDelta The change in liquidity that is being requested for the position
    function beforeUpdatePosition(Locker locker, PoolKey memory poolKey, PositionId positionId, int128 liquidityDelta)
        external;

    /// @notice Called after a position is updated
    /// @param locker The current holder of the lock performing the position update
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position that was updated
    /// @param liquidityDelta Change in liquidity of the specified position key range
    /// @param balanceUpdate Change in token balances of the pool (delta0 and delta1)
    function afterUpdatePosition(
        Locker locker,
        PoolKey memory poolKey,
        PositionId positionId,
        int128 liquidityDelta,
        PoolBalanceUpdate balanceUpdate,
        PoolState stateAfter
    ) external;

    /// @notice Called before a swap is executed
    /// @param locker The current holder of the lock performing the swap
    /// @param poolKey Pool key identifying the pool
    /// @param params Swap parameters containing amount, isToken1, sqrtRatioLimit, and skipAhead
    function beforeSwap(Locker locker, PoolKey memory poolKey, SwapParameters params) external;

    /// @notice Called after a swap is executed
    /// @param locker The current holder of the lock performing the swap
    /// @param poolKey Pool key identifying the pool
    /// @param params Swap parameters containing amount, isToken1, sqrtRatioLimit, and skipAhead
    /// @param balanceUpdate Change in token balances (delta0 and delta1)
    /// @param stateAfter The pool state after the swap
    function afterSwap(
        Locker locker,
        PoolKey memory poolKey,
        SwapParameters params,
        PoolBalanceUpdate balanceUpdate,
        PoolState stateAfter
    ) external;

    /// @notice Called before fees are collected from a position
    /// @param locker The current holder of the lock collecting fees
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position for which fees will be collected
    function beforeCollectFees(Locker locker, PoolKey memory poolKey, PositionId positionId) external;

    /// @notice Called after fees are collected from a position
    /// @param locker The current holder of the lock collecting fees
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position for which fees were collected
    /// @param amount0 Amount of token0 fees collected
    /// @param amount1 Amount of token1 fees collected
    function afterCollectFees(
        Locker locker,
        PoolKey memory poolKey,
        PositionId positionId,
        uint128 amount0,
        uint128 amount1
    ) external;
}

/// @title Core Interface
/// @notice Main interface for the Ekubo Protocol core contract
/// @dev Inherits from IFlashAccountant and IExposedStorage for additional functionality
interface ICore is IFlashAccountant, IExposedStorage {
    /// @notice Emitted when an extension is registered
    /// @param extension Address of the registered extension
    event ExtensionRegistered(address extension);

    /// @notice Emitted when a pool is initialized
    /// @param poolId Unique identifier for the pool
    /// @param poolKey Pool key containing token addresses and configuration
    /// @param tick Initial tick for the pool
    /// @param sqrtRatio Initial sqrt price ratio for the pool
    event PoolInitialized(PoolId poolId, PoolKey poolKey, int32 tick, SqrtRatio sqrtRatio);

    /// @notice Emitted when a position is updated
    /// @param locker The locker that is updating the position
    /// @param poolId Unique identifier for the pool
    /// @param positionId Identifier of the position specifying a salt and the bounds
    /// @param liquidityDelta The change in liquidity for the specified pool and position keys
    /// @param balanceUpdate Change in token balances (delta0 and delta1)
    event PositionUpdated(
        address locker,
        PoolId poolId,
        PositionId positionId,
        int128 liquidityDelta,
        PoolBalanceUpdate balanceUpdate,
        PoolState stateAfter
    );

    /// @notice Emitted when fees are collected from a position
    /// @param locker The locker that is collecting fees
    /// @param poolId Unique identifier for the pool
    /// @param positionId Identifier of the position specifying a salt and the bounds
    /// @param amount0 Amount of token0 fees collected
    /// @param amount1 Amount of token1 fees collected
    event PositionFeesCollected(address locker, PoolId poolId, PositionId positionId, uint128 amount0, uint128 amount1);

    /// @notice Emitted when fees are accumulated to a pool
    /// @param poolId Unique identifier for the pool
    /// @param amount0 Amount of token0 fees accumulated
    /// @param amount1 Amount of token1 fees accumulated
    /// @dev Note locker is ommitted because it's always the extension of the pool associated with poolId
    event FeesAccumulated(PoolId poolId, uint128 amount0, uint128 amount1);

    /// @notice Thrown when extension registration fails due to invalid call points
    error FailedRegisterInvalidCallPoints();

    /// @notice Thrown when trying to register an already registered extension
    error ExtensionAlreadyRegistered();

    /// @notice Thrown when saved balance operations would cause overflow
    error SavedBalanceOverflow();

    /// @notice Thrown when trying to initialize an already initialized pool
    error PoolAlreadyInitialized();

    /// @notice Thrown when trying to use an unregistered extension
    error ExtensionNotRegistered();

    /// @notice Thrown when trying to operate on an uninitialized pool
    error PoolNotInitialized();

    /// @notice Thrown when sqrt ratio limit is out of valid range
    error SqrtRatioLimitOutOfRange();

    /// @notice Thrown when sqrt ratio limit parameter given to swap is not a valid sqrt ratio
    error InvalidSqrtRatioLimit();

    /// @notice Thrown when the sqrt ratio limit is in the wrong direction of the current price
    error SqrtRatioLimitWrongDirection();

    /// @notice Thrown when saved balance tokens are not properly sorted
    error SavedBalanceTokensNotSorted();

    /// @notice Thrown when a position update would cause liquidityNet on a tick to exceed the maximum allowed
    /// @param tick The tick that would exceed the limit
    /// @param liquidityNet The resulting liquidityNet that exceeds the limit
    /// @param maxLiquidityPerTick The maximum allowed liquidity per tick
    error MaxLiquidityPerTickExceeded(int32 tick, uint128 liquidityNet, uint128 maxLiquidityPerTick);

    /// @notice Registers an extension with the core contract
    /// @dev Extensions must call this function to become registered. The call points are validated against the caller address
    /// @param expectedCallPoints Call points configuration for the extension
    function registerExtension(CallPoints memory expectedCallPoints) external;

    /// @notice Initializes a new pool with the given tick
    /// @dev Sets the initial price for a new pool in terms of tick
    /// @param poolKey Pool key identifying the pool to initialize
    /// @param tick Initial tick for the pool
    /// @return sqrtRatio Initial sqrt price ratio for the pool
    function initializePool(PoolKey memory poolKey, int32 tick) external returns (SqrtRatio sqrtRatio);

    /// @notice Finds the previous initialized tick
    /// @param poolId Unique identifier for the pool
    /// @param fromTick Starting tick to search from
    /// @param tickSpacing Tick spacing for the pool
    /// @param skipAhead Number of ticks to skip for gas optimization
    /// @return tick The previous initialized tick
    /// @return isInitialized Whether the tick is initialized
    function prevInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized);

    /// @notice Finds the next initialized tick
    /// @param poolId Unique identifier for the pool
    /// @param fromTick Starting tick to search from
    /// @param tickSpacing Tick spacing for the pool
    /// @param skipAhead Number of ticks to skip for gas optimization
    /// @return tick The next initialized tick
    /// @return isInitialized Whether the tick is initialized
    function nextInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized);

    /// @notice Updates saved balances for later use
    /// @dev The saved balances are stored in a single slot. The resulting saved balance must fit within a uint128 container
    /// @param token0 Address of the first token (must be < token1)
    /// @param token1 Address of the second token (must be > token0)
    /// @param salt Unique identifier for the saved balance
    /// @param delta0 Change in token0 balance (positive for saving, negative for loading)
    /// @param delta1 Change in token1 balance (positive for saving, negative for loading)
    function updateSavedBalances(address token0, address token1, bytes32 salt, int256 delta0, int256 delta1)
        external
        payable;

    /// @notice Returns the accumulated fees per liquidity inside the given bounds
    /// @dev The reason this getter is exposed is that it requires conditional SLOADs for maximum efficiency
    /// @param poolId The ID of the pool to fetch the fees per liquidity inside
    /// @param tickLower Lower bound of the price range to get the snapshot
    /// @param tickLower Upper bound of the price range to get the snapshot
    /// @return feesPerLiquidity Accumulated fees per liquidity inside the bounds
    function getPoolFeesPerLiquidityInside(PoolId poolId, int32 tickLower, int32 tickUpper)
        external
        view
        returns (FeesPerLiquidity memory feesPerLiquidity);

    /// @notice Accumulates tokens as fees for a pool
    /// @dev Only callable by the extension of the specified pool key. The current locker must be the extension.
    /// The extension must call this function within a lock callback
    /// @param poolKey Pool key identifying the pool
    /// @param amount0 Amount of token0 to accumulate as fees
    /// @param amount1 Amount of token1 to accumulate as fees
    function accumulateAsFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1) external payable;

    /// @notice Updates a liquidity position and sets extra data
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position to update
    /// @param liquidityDelta The change in liquidity
    /// @return balanceUpdate Change in token balances (delta0 and delta1)
    function updatePosition(PoolKey memory poolKey, PositionId positionId, int128 liquidityDelta)
        external
        payable
        returns (PoolBalanceUpdate balanceUpdate);

    /// @notice Sets extra data for the position
    /// @param poolId ID of the pool for which the position exists
    /// @param positionId The key of the position to set extra data at
    /// @param extraData The data to set on the position
    function setExtraData(PoolId poolId, PositionId positionId, bytes16 extraData) external;

    /// @notice Collects accumulated fees from a position
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position for which to collect fees
    /// @return amount0 Amount of token0 fees collected
    /// @return amount1 Amount of token1 fees collected
    function collectFees(PoolKey memory poolKey, PositionId positionId)
        external
        returns (uint128 amount0, uint128 amount1);

    /// @notice Executes a swap against a pool
    /// @dev Function name is mined to have a zero function selector for gas efficiency
    function swap_6269342730() external payable;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";

import {BaseForwardee} from "./base/BaseForwardee.sol";
import {UsesCore} from "./base/UsesCore.sol";
import {ICore} from "./interfaces/ICore.sol";
import {toDate, toQuarter} from "./libraries/TimeDescriptor.sol";
import {CoreLib} from "./libraries/CoreLib.sol";
import {FlashAccountantLib} from "./libraries/FlashAccountantLib.sol";
import {Locker} from "./types/locker.sol";

/// @title Time-locked Token Wrapper
/// @author Ekubo Protocol
/// @notice Wraps tokens that can only be unwrapped after a specific unlock time
/// @dev Wrapping and unwrapping happens via Ekubo Core#forward. Implements full ERC20 functionality
contract TokenWrapper is UsesCore, IERC20, BaseForwardee {
    using CoreLib for *;
    using FlashAccountantLib for *;

    /// @notice Thrown when trying to unwrap the token before the token has unlocked
    error TooEarly();

    /// @notice Thrown when attempting to transfer an amount greater than the balance
    error InsufficientBalance();

    /// @notice Thrown when calling transferFrom with an insufficient allowance
    error InsufficientAllowance();

    /// @notice The underlying token that is wrapped by this contract
    IERC20 public immutable UNDERLYING_TOKEN;

    /// @notice The timestamp after which the token may be unwrapped
    uint256 public immutable UNLOCK_TIME;

    /// @notice Constructs a new TokenWrapper
    /// @param core The Ekubo Core contract
    /// @param _underlyingToken The token to be wrapped
    /// @param _unlockTime The timestamp after which tokens can be unwrapped
    constructor(ICore core, IERC20 _underlyingToken, uint256 _unlockTime) UsesCore(core) BaseForwardee(core) {
        UNDERLYING_TOKEN = _underlyingToken;
        UNLOCK_TIME = _unlockTime;
    }

    /// @inheritdoc IERC20
    mapping(address owner => mapping(address spender => uint256)) public override allowance;

    /// @notice Mapping of account balances (not public because we use coreBalance for Core)
    /// @dev Private mapping to track individual account balances
    mapping(address account => uint256) private _balanceOf;

    /// @notice Transient balance for the Core contract
    /// @dev Core never actually holds a real balance of this token, we just use this transient balance to enable low cost payments to core
    uint256 private transient coreBalance;

    /// @inheritdoc IERC20
    /// @dev Returns the transient balance for Core contract, otherwise returns stored balance
    function balanceOf(address account) external view returns (uint256) {
        if (account == address(CORE)) return coreBalance;
        return _balanceOf[account];
    }

    /// @inheritdoc IERC20
    /// @dev Total supply is tracked in Core's saved balances
    function totalSupply() external view override returns (uint256) {
        (uint128 supply,) = CORE.savedBalances({
            owner: address(this),
            token0: address(UNDERLYING_TOKEN),
            token1: address(type(uint160).max),
            salt: bytes32(0)
        });

        return supply;
    }

    /// @inheritdoc IERC20
    /// @dev Combines underlying token name with unlock date
    function name() external view returns (string memory) {
        return string.concat(UNDERLYING_TOKEN.name(), " ", toDate(UNLOCK_TIME));
    }

    /// @inheritdoc IERC20
    /// @dev Combines "g" prefix with underlying token symbol and quarter
    function symbol() external view returns (string memory) {
        return string.concat("g", UNDERLYING_TOKEN.symbol(), "-", toQuarter(UNLOCK_TIME));
    }

    /// @inheritdoc IERC20
    function decimals() external view returns (uint8) {
        return UNDERLYING_TOKEN.decimals();
    }

    /// @inheritdoc IERC20
    function transfer(address to, uint256 amount) external returns (bool) {
        // note we do not need to check that core balance is sufficient as the sender
        // even if the caller gets core to withdraw to itself, as part of a payment, it will net to 0 with the Core#withdraw call
        if (msg.sender != address(CORE)) {
            uint256 balance = _balanceOf[msg.sender];
            if (balance < amount) {
                revert InsufficientBalance();
            }
            // since we already checked balance >= amount
            unchecked {
                _balanceOf[msg.sender] = balance - amount;
            }
        }
        if (to == address(CORE)) {
            coreBalance += amount;
        } else if (to != address(0)) {
            // we save storage writes on burn by checking to != address(0)
            _balanceOf[to] += amount;
        }
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    /// @inheritdoc IERC20
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    /// @inheritdoc IERC20
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowanceCurrent = allowance[from][msg.sender];
        if (allowanceCurrent != type(uint256).max) {
            if (allowanceCurrent < amount) revert InsufficientAllowance();
            // since we already checked allowanceCurrent >= amount
            unchecked {
                allowance[from][msg.sender] = allowanceCurrent - amount;
            }
        }

        // we know `from` at this point will never be address(core) for amount > 0, since Core will never give an allowance to any address

        uint256 balance = _balanceOf[from];
        if (balance < amount) {
            revert InsufficientBalance();
        }
        // since we already checked balance >= amount
        unchecked {
            _balanceOf[from] = balance - amount;
        }

        if (to == address(CORE)) {
            coreBalance += amount;
        } else {
            _balanceOf[to] += amount;
        }
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    /// @notice Handles wrap/unwrap operations forwarded from Core
    /// @dev Encode (int256 delta) in the forwarded data, where a positive amount means wrapping and a negative amount means unwrapping
    /// For wrap: the specified amount of this wrapper token will be credited to the locker and the same amount of underlying will be debited
    /// For unwrap: the specified amount of the underlying will be credited to the locker and the same amount of this wrapper token will be debited, iff block.timestamp > unlockTime and at least that much token has been wrapped
    /// @param data Encoded int256 delta (positive for wrap, negative for unwrap)
    /// @return Empty bytes (no return data needed)
    function handleForwardData(Locker, bytes memory data) internal override returns (bytes memory) {
        (int256 amount) = abi.decode(data, (int256));

        // unwrap
        if (amount < 0) {
            if (block.timestamp < UNLOCK_TIME) revert TooEarly();
        }

        CORE.updateSavedBalances({
            token0: address(UNDERLYING_TOKEN),
            token1: address(type(uint160).max),
            salt: bytes32(0),
            delta0: amount,
            delta1: 0
        });

        CORE.updateDebt(SafeCastLib.toInt128(-amount));

        return bytes("");
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "../types/poolId.sol";
import {OrderId} from "../types/orderId.sol";
import {StorageSlot} from "../types/storageSlot.sol";

/// @title TWAMM Storage Layout
/// @notice Library providing functions to compute the storage locations for the TWAMM contract
/// @dev TWAMM uses a custom storage layout to avoid keccak's where possible.
///      For certain storage values, the pool id is used as a base offset and
///      we allocate the following relative offsets (starting from the pool id) as:
///        0: pool state
///        [REWARD_RATES_OFFSET, REWARD_RATES_OFFSET + 1]: global reward rates
///        [TIME_BITMAPS_OFFSET, TIME_BITMAPS_OFFSET + type(uint52).max]: initialized times bitmaps
///        [TIME_INFOS_OFFSET, TIME_INFOS_OFFSET + type(uint64).max]: time infos
///        [REWARD_RATES_BEFORE_OFFSET, REWARD_RATES_BEFORE_OFFSET + 2 * type(uint64).max]: reward rates before time
library TWAMMStorageLayout {
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#REWARD_RATES_OFFSET"
    uint256 internal constant REWARD_RATES_OFFSET = 0x6536a49ed1752ddb42ba94b6b00660382279a8d99d650d701d5d127e7a3bbd95;
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#TIME_BITMAPS_OFFSET"
    uint256 internal constant TIME_BITMAPS_OFFSET = 0x07f3f693b68a1a1b1b3315d4b74217931d60e9dc7f1af4989f50e7ab31c8820e;
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#TIME_INFOS_OFFSET"
    uint256 internal constant TIME_INFOS_OFFSET = 0x70db18ef1c685b7aa06d1ac5ea2d101c7261974df22a15951f768f92187043fb;
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#REWARD_RATES_BEFORE_OFFSET"
    uint256 internal constant REWARD_RATES_BEFORE_OFFSET =
        0x6a7cb7181a18ced052a38531ee9ccb088f76cd0fb0c4475d55c480aebfae7b2b;
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#ORDER_STATE_OFFSET"
    uint256 internal constant ORDER_STATE_OFFSET = 0xdc028e0b30217dc4c47f0ed37f8e3d64faf5fcf0199e7e05f83775072aa91e8d;

    /// @notice Computes the storage slot of the TWAMM pool state
    /// @param poolId The unique identifier for the pool
    /// @return slot The storage slot in the TWAMM contract
    function twammPoolStateSlot(PoolId poolId) internal pure returns (StorageSlot slot) {
        slot = StorageSlot.wrap(PoolId.unwrap(poolId));
    }

    /// @notice Computes the first storage slot of the reward rates of a pool
    /// @param poolId The unique identifier for the pool
    /// @return firstSlot The first of two consecutive storage slots in the TWAMM contract
    function poolRewardRatesSlot(PoolId poolId) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, REWARD_RATES_OFFSET)
        }
    }

    /// @notice Computes the storage slot of the first word of an initialized times bitmap for a given pool
    /// @param poolId The unique identifier for the pool
    /// @return firstSlot The first storage slot in the TWAMM contract
    function poolInitializedTimesBitmapSlot(PoolId poolId) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, TIME_BITMAPS_OFFSET)
        }
    }

    /// @notice Computes the storage slot of time info for a specific time
    /// @param poolId The unique identifier for the pool
    /// @param time The timestamp to query
    /// @return slot The storage slot in the TWAMM contract
    function poolTimeInfosSlot(PoolId poolId, uint256 time) internal pure returns (StorageSlot slot) {
        assembly ("memory-safe") {
            slot := add(poolId, add(TIME_INFOS_OFFSET, time))
        }
    }

    /// @notice Computes the storage slot of the pool reward rates before a given time
    /// @param poolId The unique identifier for the pool
    /// @param time The time to query
    /// @return firstSlot The first of two consecutive storage slots in the TWAMM contract
    function poolRewardRatesBeforeSlot(PoolId poolId, uint256 time) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, add(REWARD_RATES_BEFORE_OFFSET, mul(time, 2)))
        }
    }

    /// @notice Computes the storage slot of the order state, followed by the order reward rate snapshot for a specific order
    /// @param owner The order owner
    /// @param salt The salt used for the order
    /// @param orderId The unique identifier for the order
    /// @return slot The storage slot of the order state in the TWAMM contract, followed by the storage slot of the order reward rate snapshot
    function orderStateSlotFollowedByOrderRewardRateSnapshotSlot(address owner, bytes32 salt, OrderId orderId)
        internal
        pure
        returns (StorageSlot slot)
    {
        assembly ("memory-safe") {
            let free := mload(0x40)
            mstore(free, owner)
            mstore(add(free, 0x20), salt)
            mstore(add(free, 0x40), orderId)
            slot := add(keccak256(free, 96), ORDER_STATE_OFFSET)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Simple single owner authorization mixin.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/auth/Ownable.sol)
///
/// @dev Note:
/// This implementation does NOT auto-initialize the owner to `msg.sender`.
/// You MUST call the `_initializeOwner` in the constructor / initializer.
///
/// While the ownable portion follows
/// [EIP-173](https://eips.ethereum.org/EIPS/eip-173) for compatibility,
/// the nomenclature for the 2-step ownership handover may be unique to this codebase.
abstract contract Ownable {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                       CUSTOM ERRORS                        */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The caller is not authorized to call the function.
    error Unauthorized();

    /// @dev The `newOwner` cannot be the zero address.
    error NewOwnerIsZeroAddress();

    /// @dev The `pendingOwner` does not have a valid handover request.
    error NoHandoverRequest();

    /// @dev Cannot double-initialize.
    error AlreadyInitialized();

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                           EVENTS                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The ownership is transferred from `oldOwner` to `newOwner`.
    /// This event is intentionally kept the same as OpenZeppelin's Ownable to be
    /// compatible with indexers and [EIP-173](https://eips.ethereum.org/EIPS/eip-173),
    /// despite it not being as lightweight as a single argument event.
    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);

    /// @dev An ownership handover to `pendingOwner` has been requested.
    event OwnershipHandoverRequested(address indexed pendingOwner);

    /// @dev The ownership handover to `pendingOwner` has been canceled.
    event OwnershipHandoverCanceled(address indexed pendingOwner);

    /// @dev `keccak256(bytes("OwnershipTransferred(address,address)"))`.
    uint256 private constant _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE =
        0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0;

    /// @dev `keccak256(bytes("OwnershipHandoverRequested(address)"))`.
    uint256 private constant _OWNERSHIP_HANDOVER_REQUESTED_EVENT_SIGNATURE =
        0xdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d;

    /// @dev `keccak256(bytes("OwnershipHandoverCanceled(address)"))`.
    uint256 private constant _OWNERSHIP_HANDOVER_CANCELED_EVENT_SIGNATURE =
        0xfa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c92;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                          STORAGE                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The owner slot is given by:
    /// `bytes32(~uint256(uint32(bytes4(keccak256("_OWNER_SLOT_NOT")))))`.
    /// It is intentionally chosen to be a high value
    /// to avoid collision with lower slots.
    /// The choice of manual storage layout is to enable compatibility
    /// with both regular and upgradeable contracts.
    bytes32 internal constant _OWNER_SLOT =
        0xffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927;

    /// The ownership handover slot of `newOwner` is given by:
    /// ```
    ///     mstore(0x00, or(shl(96, user), _HANDOVER_SLOT_SEED))
    ///     let handoverSlot := keccak256(0x00, 0x20)
    /// ```
    /// It stores the expiry timestamp of the two-step ownership handover.
    uint256 private constant _HANDOVER_SLOT_SEED = 0x389a75e1;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                     INTERNAL FUNCTIONS                     */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Override to return true to make `_initializeOwner` prevent double-initialization.
    function _guardInitializeOwner() internal pure virtual returns (bool guard) {}

    /// @dev Initializes the owner directly without authorization guard.
    /// This function must be called upon initialization,
    /// regardless of whether the contract is upgradeable or not.
    /// This is to enable generalization to both regular and upgradeable contracts,
    /// and to save gas in case the initial owner is not the caller.
    /// For performance reasons, this function will not check if there
    /// is an existing owner.
    function _initializeOwner(address newOwner) internal virtual {
        if (_guardInitializeOwner()) {
            /// @solidity memory-safe-assembly
            assembly {
                let ownerSlot := _OWNER_SLOT
                if sload(ownerSlot) {
                    mstore(0x00, 0x0dc149f0) // `AlreadyInitialized()`.
                    revert(0x1c, 0x04)
                }
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Store the new value.
                sstore(ownerSlot, or(newOwner, shl(255, iszero(newOwner))))
                // Emit the {OwnershipTransferred} event.
                log3(0, 0, _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE, 0, newOwner)
            }
        } else {
            /// @solidity memory-safe-assembly
            assembly {
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Store the new value.
                sstore(_OWNER_SLOT, newOwner)
                // Emit the {OwnershipTransferred} event.
                log3(0, 0, _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE, 0, newOwner)
            }
        }
    }

    /// @dev Sets the owner directly without authorization guard.
    function _setOwner(address newOwner) internal virtual {
        if (_guardInitializeOwner()) {
            /// @solidity memory-safe-assembly
            assembly {
                let ownerSlot := _OWNER_SLOT
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Emit the {OwnershipTransferred} event.
                log3(0, 0, _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE, sload(ownerSlot), newOwner)
                // Store the new value.
                sstore(ownerSlot, or(newOwner, shl(255, iszero(newOwner))))
            }
        } else {
            /// @solidity memory-safe-assembly
            assembly {
                let ownerSlot := _OWNER_SLOT
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Emit the {OwnershipTransferred} event.
                log3(0, 0, _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE, sload(ownerSlot), newOwner)
                // Store the new value.
                sstore(ownerSlot, newOwner)
            }
        }
    }

    /// @dev Throws if the sender is not the owner.
    function _checkOwner() internal view virtual {
        /// @solidity memory-safe-assembly
        assembly {
            // If the caller is not the stored owner, revert.
            if iszero(eq(caller(), sload(_OWNER_SLOT))) {
                mstore(0x00, 0x82b42900) // `Unauthorized()`.
                revert(0x1c, 0x04)
            }
        }
    }

    /// @dev Returns how long a two-step ownership handover is valid for in seconds.
    /// Override to return a different value if needed.
    /// Made internal to conserve bytecode. Wrap it in a public function if needed.
    function _ownershipHandoverValidFor() internal view virtual returns (uint64) {
        return 48 * 3600;
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                  PUBLIC UPDATE FUNCTIONS                   */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Allows the owner to transfer the ownership to `newOwner`.
    function transferOwnership(address newOwner) public payable virtual onlyOwner {
        /// @solidity memory-safe-assembly
        assembly {
            if iszero(shl(96, newOwner)) {
                mstore(0x00, 0x7448fbae) // `NewOwnerIsZeroAddress()`.
                revert(0x1c, 0x04)
            }
        }
        _setOwner(newOwner);
    }

    /// @dev Allows the owner to renounce their ownership.
    function renounceOwnership() public payable virtual onlyOwner {
        _setOwner(address(0));
    }

    /// @dev Request a two-step ownership handover to the caller.
    /// The request will automatically expire in 48 hours (172800 seconds) by default.
    function requestOwnershipHandover() public payable virtual {
        unchecked {
            uint256 expires = block.timestamp + _ownershipHandoverValidFor();
            /// @solidity memory-safe-assembly
            assembly {
                // Compute and set the handover slot to `expires`.
                mstore(0x0c, _HANDOVER_SLOT_SEED)
                mstore(0x00, caller())
                sstore(keccak256(0x0c, 0x20), expires)
                // Emit the {OwnershipHandoverRequested} event.
                log2(0, 0, _OWNERSHIP_HANDOVER_REQUESTED_EVENT_SIGNATURE, caller())
            }
        }
    }

    /// @dev Cancels the two-step ownership handover to the caller, if any.
    function cancelOwnershipHandover() public payable virtual {
        /// @solidity memory-safe-assembly
        assembly {
            // Compute and set the handover slot to 0.
            mstore(0x0c, _HANDOVER_SLOT_SEED)
            mstore(0x00, caller())
            sstore(keccak256(0x0c, 0x20), 0)
            // Emit the {OwnershipHandoverCanceled} event.
            log2(0, 0, _OWNERSHIP_HANDOVER_CANCELED_EVENT_SIGNATURE, caller())
        }
    }

    /// @dev Allows the owner to complete the two-step ownership handover to `pendingOwner`.
    /// Reverts if there is no existing ownership handover requested by `pendingOwner`.
    function completeOwnershipHandover(address pendingOwner) public payable virtual onlyOwner {
        /// @solidity memory-safe-assembly
        assembly {
            // Compute and set the handover slot to 0.
            mstore(0x0c, _HANDOVER_SLOT_SEED)
            mstore(0x00, pendingOwner)
            let handoverSlot := keccak256(0x0c, 0x20)
            // If the handover does not exist, or has expired.
            if gt(timestamp(), sload(handoverSlot)) {
                mstore(0x00, 0x6f5e8818) // `NoHandoverRequest()`.
                revert(0x1c, 0x04)
            }
            // Set the handover slot to 0.
            sstore(handoverSlot, 0)
        }
        _setOwner(pendingOwner);
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                   PUBLIC READ FUNCTIONS                    */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Returns the owner of the contract.
    function owner() public view virtual returns (address result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := sload(_OWNER_SLOT)
        }
    }

    /// @dev Returns the expiry timestamp for the two-step ownership handover to `pendingOwner`.
    function ownershipHandoverExpiresAt(address pendingOwner)
        public
        view
        virtual
        returns (uint256 result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            // Compute the handover slot.
            mstore(0x0c, _HANDOVER_SLOT_SEED)
            mstore(0x00, pendingOwner)
            // Load the handover slot.
            result := sload(keccak256(0x0c, 0x20))
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                         MODIFIERS                          */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Marks a function as only callable by the owner.
    modifier onlyOwner() virtual {
        _checkOwner();
        _;
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @title IExposedStorage
/// @notice Interface for exposing contract storage via view methods
/// @dev This interface provides a way to access specific pieces of state in the inheriting contract.
///      It serves as a workaround in the absence of EIP-2330 (https://eips.ethereum.org/EIPS/eip-2330)
///      which would provide native support for exposing contract storage.
interface IExposedStorage {
    /// @notice Loads storage slots from the contract's persistent storage
    /// @dev Reads each 32-byte slot specified in the calldata (after the function selector) from storage
    ///      and returns all the loaded values concatenated together.
    function sload() external view;

    /// @notice Loads storage slots from the contract's transient storage
    /// @dev Reads each 32-byte slot specified in the calldata (after the function selector) from transient storage
    ///      and returns all the loaded values concatenated together. Transient storage is cleared at the end
    ///      of each transaction.
    function tload() external view;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @title Base Nonfungible Token Interface
/// @notice Interface for the base NFT functionality used by Positions and Orders contracts
/// @dev Defines the essential NFT functions needed by other contracts in the system
interface IBaseNonfungibleToken {
    function setMetadata(string memory newName, string memory newSymbol, string memory newBaseUrl) external;

    // BaseNonfungibleToken specific functions
    /// @notice Converts a minter address and salt to a token ID
    /// @param minter The minter address
    /// @param salt The salt value
    /// @return result The resulting token ID
    function saltToId(address minter, bytes32 salt) external view returns (uint256 result);

    /// @notice Mints a new token with a random salt
    /// @return id The minted token ID
    function mint() external payable returns (uint256 id);

    /// @notice Mints a new token with a specific salt
    /// @param salt The salt for deterministic ID generation
    /// @return id The minted token ID
    function mint(bytes32 salt) external payable returns (uint256 id);

    /// @notice Burns a token (only authorized addresses)
    /// @param id The token ID to burn
    function burn(uint256 id) external payable;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {Locker} from "../types/locker.sol";

interface ILocker {
    function locked_6416899205(uint256 id) external;
}

interface IForwardee {
    function forwarded_2374103877(Locker original) external;
}

/// @title IFlashAccountant
/// @notice Interface for flash loan accounting functionality using transient storage
/// @dev This interface manages debt tracking for flash loans, allowing users to borrow tokens temporarily
///      and ensuring all debts are settled before the transaction completes. Uses transient storage
///      for gas-efficient temporary state management within a single transaction.
interface IFlashAccountant {
    /// @notice Thrown when an operation is made that affects debts without an active lock
    error NotLocked();
    /// @notice Thrown when a method is called by an address other than the current locker
    error LockerOnly();
    /// @notice Thrown when the lock callback returns without clearing all the debts either via withdrawing from or paying to the accountant
    error DebtsNotZeroed(uint256 id);
    /// @notice Thrown if the contract receives a payment that exceeds type(uint128).max
    error PaymentOverflow();
    /// @notice Thrown if the contract receives a call to updateDebt that has a data length != 20
    error UpdateDebtMessageLength();

    /// @notice Creates a lock context and calls back to the caller's locked function
    /// @dev The entrypoint for all operations on the core contract. Any data passed after the
    ///      function signature is passed through back to the caller after the locked function
    ///      signature and data, with no additional encoding. Any data returned from ILocker#locked
    ///      is also returned from this function exactly as is. Reverts are bubbled up.
    ///      Ensures all debts are zeroed before completing the lock.
    function lock() external;

    /// @notice Forwards the lock context to another actor, allowing them to act on the original locker's debt
    /// @dev Temporarily changes the locker to the forwarded address for the duration of the forwarded call.
    ///      Any additional calldata is passed through to the forwardee with no additional encoding.
    ///      Any data returned from IForwardee#forwarded is returned exactly as is. Reverts are bubbled up.
    /// @param to The address to forward the lock context to
    function forward(address to) external;

    /// @notice Initiates a payment operation by recording current token balances
    /// @dev To make a payment to core, you must first call startPayments with all the tokens you'd like to send.
    ///      All the tokens that will be paid must be ABI-encoded immediately after the 4 byte function selector.
    ///      This function stores the current balance + 1 for each token to distinguish between zero balance
    ///      and uninitialized state. Returns the current balances of all specified tokens as ABI-encoded
    ///      raw bytes via assembly (no explicit Solidity return type).
    function startPayments() external;

    /// @notice Completes a payment operation by calculating and crediting token payments
    /// @dev After tokens have been transferred, call completePayments to be credited for the tokens
    ///      that have been paid to core. The credit goes to the current locker. Compares current
    ///      balances with those recorded in startPayments to determine payment amounts.
    ///      The computed payments are applied to the current locker's debt.
    ///      Returns packed uint128 payment amounts (16 bytes each) in the same order as the tokens.
    function completePayments() external;

    /// @notice Withdraws tokens from the accountant to recipients using packed calldata
    /// @dev The contract must be locked, as it tracks withdrawn amounts against the current locker's debt.
    ///      Calldata format: each withdrawal is 56 bytes: token (20) + recipient (20) + amount (16)
    ///      For native tokens, uses the NATIVE_TOKEN_ADDRESS constant and transfers ETH directly.
    function withdraw() external;

    /// @notice Updates debt for the current locker and for the token at the calling address
    /// @dev This is for deeply-integrated tokens that allow flash operations via the accountant.
    ///      The calling address is treated as the token address.
    /// @dev The debt change argument is an int128 encoded immediately after the selector.
    /// @dev The calldata length must be exactly 20 bytes in order to avoid this being called unintentionally.
    function updateDebt() external;

    /// @notice Receives ETH payments and credits them against the current locker's native token debt
    /// @dev This contract can receive ETH as a payment. The received amount is credited as a negative
    ///      debt change for the native token. Note: because we use msg.value here, this contract can
    ///      never be multicallable, i.e. it should never expose the ability to delegatecall itself
    ///      more than once in a single call.
    receive() external payable;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Contract that enables a single call to call multiple methods on itself.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/Multicallable.sol)
/// @author Modified from Solmate (https://github.com/transmissions11/solmate/blob/main/src/utils/Multicallable.sol)
///
/// WARNING:
/// This implementation is NOT to be used with ERC2771 out-of-the-box.
/// https://blog.openzeppelin.com/arbitrary-address-spoofing-vulnerability-erc2771context-multicall-public-disclosure
/// This also applies to potentially other ERCs / patterns appending to the back of calldata.
///
/// We do NOT have a check for ERC2771, as we do not inherit from OpenZeppelin's context.
/// Moreover, it is infeasible and inefficient for us to add checks and mitigations
/// for all possible ERC / patterns appending to the back of calldata.
///
/// We would highly recommend using an alternative pattern such as
/// https://github.com/Vectorized/multicaller
/// which is more flexible, futureproof, and safer by default.
abstract contract Multicallable {
    /// @dev Apply `delegatecall` with the current contract to each calldata in `data`,
    /// and store the `abi.encode` formatted results of each `delegatecall` into `results`.
    /// If any of the `delegatecall`s reverts, the entire context is reverted,
    /// and the error is bubbled up.
    ///
    /// By default, this function directly returns the results and terminates the call context.
    /// If you need to add before and after actions to the multicall, please override this function.
    function multicall(bytes[] calldata data) public payable virtual returns (bytes[] memory) {
        // Revert if `msg.value` is non-zero by default to guard against double-spending.
        // (See: https://www.paradigm.xyz/2021/08/two-rights-might-make-a-wrong)
        //
        // If you really need to pass in a `msg.value`, then you will have to
        // override this function and add in any relevant before and after checks.
        if (msg.value != 0) revert();
        // `_multicallDirectReturn` returns the results directly and terminates the call context.
        _multicallDirectReturn(_multicall(data));
    }

    /// @dev The inner logic of `multicall`.
    /// This function is included so that you can override `multicall`
    /// to add before and after actions, and use the `_multicallDirectReturn` function.
    function _multicall(bytes[] calldata data) internal virtual returns (bytes32 results) {
        /// @solidity memory-safe-assembly
        assembly {
            results := mload(0x40)
            mstore(results, 0x20)
            mstore(add(0x20, results), data.length)
            let c := add(0x40, results)
            let s := c
            let end := shl(5, data.length)
            calldatacopy(c, data.offset, end)
            end := add(c, end)
            let m := end
            if data.length {
                for {} 1 {} {
                    let o := add(data.offset, mload(c))
                    calldatacopy(m, add(o, 0x20), calldataload(o))
                    // forgefmt: disable-next-item
                    if iszero(delegatecall(gas(), address(), m, calldataload(o), codesize(), 0x00)) {
                        // Bubble up the revert if the delegatecall reverts.
                        returndatacopy(results, 0x00, returndatasize())
                        revert(results, returndatasize())
                    }
                    mstore(c, sub(m, s))
                    c := add(0x20, c)
                    // Append the `returndatasize()`, and the return data.
                    mstore(m, returndatasize())
                    let b := add(m, 0x20)
                    returndatacopy(b, 0x00, returndatasize())
                    // Advance `m` by `returndatasize() + 0x20`,
                    // rounded up to the next multiple of 32.
                    m := and(add(add(b, returndatasize()), 0x1f), 0xffffffffffffffe0)
                    mstore(add(b, returndatasize()), 0) // Zeroize the slot after the returndata.
                    if iszero(lt(c, end)) { break }
                }
            }
            mstore(0x40, m) // Allocate memory.
            results := or(shl(64, sub(m, results)), results) // Pack the bytes length into `results`.
        }
    }

    /// @dev Decodes the `results` into an array of bytes.
    /// This can be useful if you need to access the results or re-encode it.
    function _multicallResultsToBytesArray(bytes32 results)
        internal
        pure
        virtual
        returns (bytes[] memory decoded)
    {
        /// @solidity memory-safe-assembly
        assembly {
            decoded := mload(0x40)
            let c := and(0xffffffffffffffff, results) // Extract the offset.
            mstore(decoded, mload(add(c, 0x20))) // Store the length.
            let o := add(decoded, 0x20) // Start of elements in `decoded`.
            let end := add(o, shl(5, mload(decoded)))
            mstore(0x40, end) // Allocate memory.
            let s := add(c, 0x40) // Start of elements in `results`.
            let d := sub(s, o) // Difference between input and output pointers.
            for {} iszero(eq(o, end)) { o := add(o, 0x20) } { mstore(o, add(mload(add(d, o)), s)) }
        }
    }

    /// @dev Directly returns the `results` and terminates the current call context.
    /// `results` must be from `_multicall`, else behavior is undefined.
    function _multicallDirectReturn(bytes32 results) internal pure virtual {
        /// @solidity memory-safe-assembly
        assembly {
            return(and(0xffffffffffffffff, results), shr(64, results))
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Order configuration packed into a single bytes32
/// @dev Contains fee (8 bytes), isToken1 (1 byte), padding (7 bytes), start time (8 bytes), and end time (8 bytes)
type OrderConfig is bytes32;

using {fee, isToken1, startTime, endTime} for OrderConfig global;

/// @notice Extracts the fee from an order config
/// @param config The order config
/// @return r The fee
function fee(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := shr(192, config)
    }
}

/// @notice Extracts isToken1 from an order config
/// @param config The order config
/// @return r Whether the order is selling token1
function isToken1(OrderConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(iszero(byte(8, config)))
    }
}

/// @notice Extracts the start time from an order config
/// @param config The order config
/// @return r The start time
function startTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(64, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the end time from an order config
/// @param config The order config
/// @return r The end time
function endTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(config, 0xffffffffffffffff)
    }
}

/// @notice Creates an OrderConfig from individual components
/// @param _fee The fee of the TWAMM pool
/// @param _startTime The start time of the order
/// @param _endTime The end time of the order
/// @return c The packed configuration
function createOrderConfig(uint64 _fee, bool _isToken1, uint64 _startTime, uint64 _endTime)
    pure
    returns (OrderConfig c)
{
    assembly ("memory-safe") {
        // Mask inputs to ensure only relevant bits are used
        c := add(
            add(shl(192, _fee), add(shl(184, iszero(iszero(_isToken1))), shl(64, and(_startTime, 0xffffffffffffffff)))),
            and(_endTime, 0xffffffffffffffff)
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Returns the fee to charge based on the amount, which is the fee (a 0.64 number) times the
// amount, rounded up
function computeFee(uint128 amount, uint64 fee) pure returns (uint128 result) {
    assembly ("memory-safe") {
        result := shr(64, add(mul(amount, fee), 0xffffffffffffffff))
    }
}

error AmountBeforeFeeOverflow();

// Returns the amount before the fee is applied, which is the amount minus the fee, rounded up
function amountBeforeFee(uint128 afterFee, uint64 fee) pure returns (uint128 result) {
    assembly ("memory-safe") {
        let v := shl(64, afterFee)
        let d := sub(0x10000000000000000, fee)
        result := add(iszero(iszero(mod(v, d))), div(v, d))
        if shr(128, result) {
            mstore(0, 0x0d88f526)
            revert(0x1c, 0x04)
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type PoolBalanceUpdate is bytes32;

using {delta0, delta1} for PoolBalanceUpdate global;

function delta0(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, shr(128, update))
    }
}

function delta1(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, update)
    }
}

function createPoolBalanceUpdate(int128 _delta0, int128 _delta1) pure returns (PoolBalanceUpdate update) {
    assembly ("memory-safe") {
        // update = (delta0 << 128) | delta1
        update := or(shl(128, _delta0), and(_delta1, 0xffffffffffffffffffffffffffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a TWAMM order
/// @dev Wraps bytes32 to provide type safety for order identifiers
type OrderId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {LibBit} from "solady/utils/LibBit.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {amount0Delta, amount1Delta, sortAndConvertToFixedSqrtRatios} from "./delta.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";

// Liquidity Math Library
// Contains functions for calculating liquidity-related amounts and conversions
// Provides utilities for converting between liquidity changes and token amounts

/// @notice Returns the token0 and token1 delta owed for a given change in liquidity
/// @dev Calculates the token amounts required or returned when liquidity is added or removed from a position
/// @param sqrtRatio Current price (as a valid sqrt ratio)
/// @param liquidityDelta Signed liquidity change; positive = added, negative = removed
/// @param sqrtRatioLower The lower bound of the price range (as a valid sqrt ratio)
/// @param sqrtRatioUpper The upper bound of the price range (as a valid sqrt ratio)
/// @return delta0 The change in token0 amount
/// @return delta1 The change in token1 amount
function liquidityDeltaToAmountDelta(
    SqrtRatio sqrtRatio,
    int128 liquidityDelta,
    SqrtRatio sqrtRatioLower,
    SqrtRatio sqrtRatioUpper
) pure returns (int128 delta0, int128 delta1) {
    unchecked {
        if (liquidityDelta == 0) {
            return (0, 0);
        }
        bool isPositive = (liquidityDelta > 0);
        int256 sign = -1 + 2 * int256(LibBit.rawToUint(isPositive));
        // absolute value of a int128 always fits in a uint128
        uint128 magnitude = uint128(FixedPointMathLib.abs(liquidityDelta));

        if (sqrtRatio <= sqrtRatioLower) {
            delta0 = SafeCastLib.toInt128(
                sign * int256(uint256(amount0Delta(sqrtRatioLower, sqrtRatioUpper, magnitude, isPositive)))
            );
        } else if (sqrtRatio < sqrtRatioUpper) {
            delta0 = SafeCastLib.toInt128(
                sign * int256(uint256(amount0Delta(sqrtRatio, sqrtRatioUpper, magnitude, isPositive)))
            );
            delta1 = SafeCastLib.toInt128(
                sign * int256(uint256(amount1Delta(sqrtRatioLower, sqrtRatio, magnitude, isPositive)))
            );
        } else {
            delta1 = SafeCastLib.toInt128(
                sign * int256(uint256(amount1Delta(sqrtRatioLower, sqrtRatioUpper, magnitude, isPositive)))
            );
        }
    }
}

/// @notice Calculates the maximum liquidity that can be provided with a given amount of token0
/// @dev Used when the current price is below the position's range (only token0 is needed)
/// @param sqrtRatioLower The lower sqrt price ratio of the position
/// @param sqrtRatioUpper The upper sqrt price ratio of the position
/// @param amount The amount of token0 available
/// @return The maximum liquidity that can be provided
function maxLiquidityForToken0(uint256 sqrtRatioLower, uint256 sqrtRatioUpper, uint128 amount) pure returns (uint256) {
    unchecked {
        uint256 numerator1 = FixedPointMathLib.fullMulDivN(sqrtRatioLower, sqrtRatioUpper, 128);

        return FixedPointMathLib.fullMulDiv(amount, numerator1, (sqrtRatioUpper - sqrtRatioLower));
    }
}

/// @notice Calculates the maximum liquidity that can be provided with a given amount of token1
/// @dev Used when the current price is above the position's range (only token1 is needed)
/// @param sqrtRatioLower The lower sqrt price ratio of the position
/// @param sqrtRatioUpper The upper sqrt price ratio of the position
/// @param amount The amount of token1 available
/// @return The maximum liquidity that can be provided
function maxLiquidityForToken1(uint256 sqrtRatioLower, uint256 sqrtRatioUpper, uint128 amount) pure returns (uint256) {
    unchecked {
        return (uint256(amount) << 128) / (sqrtRatioUpper - sqrtRatioLower);
    }
}

/// @notice Calculates the maximum liquidity that can be provided given amounts of both tokens
/// @dev Determines the limiting factor between token0 and token1 based on current price and position bounds
/// @param _sqrtRatio Current sqrt price ratio
/// @param sqrtRatioA One bound of the position (will be sorted with sqrtRatioB)
/// @param sqrtRatioB Other bound of the position (will be sorted with sqrtRatioA)
/// @param amount0 Available amount of token0
/// @param amount1 Available amount of token1
/// @return The maximum liquidity that can be provided with the given token amounts
function maxLiquidity(
    SqrtRatio _sqrtRatio,
    SqrtRatio sqrtRatioA,
    SqrtRatio sqrtRatioB,
    uint128 amount0,
    uint128 amount1
) pure returns (uint128) {
    uint256 sqrtRatio = _sqrtRatio.toFixed();
    (uint256 sqrtRatioLower, uint256 sqrtRatioUpper) = sortAndConvertToFixedSqrtRatios(sqrtRatioA, sqrtRatioB);

    if (sqrtRatio <= sqrtRatioLower) {
        return uint128(
            FixedPointMathLib.min(type(uint128).max, maxLiquidityForToken0(sqrtRatioLower, sqrtRatioUpper, amount0))
        );
    } else if (sqrtRatio < sqrtRatioUpper) {
        return uint128(
            FixedPointMathLib.min(
                type(uint128).max,
                FixedPointMathLib.min(
                    maxLiquidityForToken0(sqrtRatio, sqrtRatioUpper, amount0),
                    maxLiquidityForToken1(sqrtRatioLower, sqrtRatio, amount1)
                )
            )
        );
    } else {
        return uint128(
            FixedPointMathLib.min(type(uint128).max, maxLiquidityForToken1(sqrtRatioLower, sqrtRatioUpper, amount1))
        );
    }
}

/// @notice Thrown when a liquidity delta operation would cause overflow
error LiquidityDeltaOverflow();

/// @notice Safely adds a liquidity delta to a liquidity amount
/// @dev Reverts if the operation would cause overflow or underflow
/// @param liquidity The current liquidity amount
/// @param liquidityDelta The change in liquidity (can be positive or negative)
/// @return result The new liquidity amount after applying the delta
function addLiquidityDelta(uint128 liquidity, int128 liquidityDelta) pure returns (uint128 result) {
    assembly ("memory-safe") {
        result := add(liquidity, liquidityDelta)
        if and(result, shl(128, 0xffffffffffffffffffffffffffffffff)) {
            mstore(0, shl(224, 0x6d862c50))
            revert(0, 4)
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK, MAX_TICK_SPACING} from "../math/constants.sol";

/// @notice Pool configuration packed into a single bytes32
/// @dev Contains extension address (20 bytes), fee (8 bytes), and pool type config (4 bytes)
/// Pool type config (32 bits):
///   - Bit 31: discriminator (1 = concentrated, 0 = stableswap)
///   - For concentrated (bit 31 = 1): bits 30-0 are tick spacing
///   - For stableswap (bit 31 = 0): bits 30-24 are amplification factor, bits 23-0 are center tick
type PoolConfig is bytes32;

using {
    fee,
    extension,
    isConcentrated,
    isStableswap,
    isFullRange,
    concentratedTickSpacing,
    stableswapAmplification,
    stableswapCenterTick,
    stableswapActiveLiquidityTickRange,
    stableswapLiquidityWidth,
    validate,
    concentratedMaxLiquidityPerTick
} for PoolConfig global;

/// @notice Extracts the fee from a pool config
/// @param config The pool config
/// @return r The fee
function fee(PoolConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(32, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the extension address from a pool config
/// @param config The pool config
/// @return r The extension address
function extension(PoolConfig config) pure returns (address r) {
    assembly ("memory-safe") {
        r := shr(96, config)
    }
}

/// @notice Checks if this is a concentrated liquidity pool
/// @param config The pool config
/// @return r True if the pool is concentrated liquidity
function isConcentrated(PoolConfig config) pure returns (bool r) {
    r = !config.isStableswap();
}

/// @notice Extracts the tick spacing from a concentrated liquidity pool config
/// @dev Only valid for concentrated liquidity pools (isConcentrated() == true)
/// @param config The pool config
/// @return r The tick spacing
function concentratedTickSpacing(PoolConfig config) pure returns (uint32 r) {
    assembly ("memory-safe") {
        // Extract lower 31 bits (bits 30-0)
        r := and(config, 0x7fffffff)
    }
}

/// @notice Checks if this is a stableswap pool
/// @param config The pool config
/// @return r True if the pool is stableswap
function isStableswap(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // = iff bit 31 is not set
        r := iszero(and(0x80000000, config))
    }
}

/// @notice Determines if this pool is full range (special case of stableswap with amplification=0 and center=0)
/// @dev Full range can be slightly optimized in that we don't need to compute the sqrt ratio at the tick boundaries
/// @param config The pool config
/// @return r True if the pool is full range
function isFullRange(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // Full range when all 32 bits are 0 (discriminator=0, amplification=0, center=0)
        r := iszero(and(config, 0xffffffff))
    }
}

/// @notice Extracts the amplification factor from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @param config The pool config
/// @return r The amplification factor (0-127)
function stableswapAmplification(PoolConfig config) pure returns (uint8 r) {
    assembly ("memory-safe") {
        // Extract bits 30-24
        r := and(shr(24, config), 0x7f)
    }
}

/// @notice Extracts the center tick from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @dev The 24-bit center tick is scaled by 16 to get the actual tick
/// @param config The pool config
/// @return r The center tick
function stableswapCenterTick(PoolConfig config) pure returns (int32 r) {
    assembly ("memory-safe") {
        // Extract bits 23-0 and sign extend to 32 bits (24-bit signed integer)
        // Then multiply by 16, since the value does not have full precision
        r := mul(signextend(2, and(config, 0xffffff)), 16)
    }
}

/// @notice Returns the width of the liquidity range
function stableswapLiquidityWidth(PoolConfig config) pure returns (uint256 width) {
    uint8 amp = config.stableswapAmplification();

    assembly ("memory-safe") {
        width := shr(amp, MAX_TICK)
    }
}

/// @notice Computes the tick range where liquidity is active for stableswap pools
/// @param config The pool config
/// @return lower The lower tick of the bounds
/// @return upper The upper tick of the bounds
function stableswapActiveLiquidityTickRange(PoolConfig config) pure returns (int32 lower, int32 upper) {
    int32 center = config.stableswapCenterTick();
    uint256 width = config.stableswapLiquidityWidth();

    assembly ("memory-safe") {
        lower := sub(center, width)
        lower := add(lower, mul(sgt(MIN_TICK, lower), sub(MIN_TICK, lower)))

        upper := add(center, width)
        upper := sub(upper, mul(sgt(upper, MAX_TICK), sub(upper, MAX_TICK)))
    }
}

/// @notice Creates a PoolConfig for a concentrated liquidity pool
/// @param _fee The fee for the pool
/// @param _tickSpacing The tick spacing for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createConcentratedPoolConfig(uint64 _fee, uint32 _tickSpacing, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Set bit 31 to 1 for concentrated liquidity, then OR with tick spacing (bits 30-0)
        let typeConfig := or(0x80000000, and(_tickSpacing, 0x7fffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a stableswap pool
/// @param _fee The fee for the pool
/// @param _amplification The amplification factor (0-127)
/// @param _centerTick The center tick (will be divided by 16 and stored as 24-bit value)
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createStableswapPoolConfig(uint64 _fee, uint8 _amplification, int32 _centerTick, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Divide center tick by 16 to get 24-bit representation
        let stableswapCenterTick24 := sdiv(_centerTick, 16)
        // Pack: bit 31 = 0 (stableswap), bits 30-24 = amplification, bits 23-0 = center tick
        let typeConfig := or(shl(24, and(_amplification, 0x7f)), and(stableswapCenterTick24, 0xffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a full range pool (stableswap with amplification=0, center=0)
/// @param _fee The fee for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createFullRangePoolConfig(uint64 _fee, address _extension) pure returns (PoolConfig c) {
    assembly ("memory-safe") {
        // All 32 bits of type config are 0 (discriminator=0, amplification=0, center=0)
        c := or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff)))
    }
}

/// @notice Computes the maximum liquidity per tick for a given concentrated liquidity pool configuration.
/// @dev Only valid for concentrated liquidity pools. Stableswap pools don't use ticks.
/// @dev Calculated as type(uint128).max / (1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2)
/// @param config The concentrated liquidity pool configuration
/// @return maxLiquidity The maximum liquidity allowed to reference each tick
function concentratedMaxLiquidityPerTick(PoolConfig config) pure returns (uint128 maxLiquidity) {
    uint32 _tickSpacing = config.concentratedTickSpacing();

    assembly ("memory-safe") {
        // Calculate total number of usable ticks: 1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2
        // This represents all ticks from -MAX_TICK_MAGNITUDE to +MAX_TICK_MAGNITUDE, and tick 0
        let numTicks := add(1, mul(div(MAX_TICK, _tickSpacing), 2))

        maxLiquidity := div(sub(shl(128, 1), 1), numTicks)
    }
}

/// @notice Thrown when tick spacing exceeds the maximum allowed value
error InvalidTickSpacing();

/// @notice Thrown when amplification factor exceeds the maximum allowed value
error InvalidStableswapAmplification();

/// @notice Thrown when center tick is not between min and max tick
error InvalidCenterTick();

/// @notice Validates that a pool config is properly formatted
/// @param config The config to validate
function validate(PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (config.concentratedTickSpacing() > MAX_TICK_SPACING || config.concentratedTickSpacing() == 0) {
            revert InvalidTickSpacing();
        }
    } else {
        // Stableswap pool: validate amplification factor <= 26
        if (config.stableswapAmplification() > 26) {
            revert InvalidStableswapAmplification();
        }
        int32 centerTick = config.stableswapCenterTick();
        if (centerTick < MIN_TICK || centerTick > MAX_TICK) {
            revert InvalidCenterTick();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK} from "../math/constants.sol";
import {PoolConfig} from "./poolConfig.sol";

type PositionId is bytes32;

using {validate, salt, tickLower, tickUpper} for PositionId global;

function salt(PositionId positionId) pure returns (bytes24 v) {
    assembly ("memory-safe") {
        v := shl(64, shr(64, positionId))
    }
}

function tickLower(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // shift down, then signextend to 32 bits
        v := signextend(3, shr(32, positionId))
    }
}

function tickUpper(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // lowest 4 bytes, then signextend to 32 bits
        v := signextend(3, positionId)
    }
}

function createPositionId(bytes24 _salt, int32 _tickLower, int32 _tickUpper) pure returns (PositionId v) {
    assembly ("memory-safe") {
        // v = salt | (tickLower << 32) | tickUpper
        v := or(shl(64, shr(64, _salt)), or(shl(32, and(_tickLower, 0xFFFFFFFF)), and(_tickUpper, 0xFFFFFFFF)))
    }
}

/// @notice Thrown when the order of the position bounds is invalid, i.e. tickLower >= tickUpper
error BoundsOrder();
/// @notice Thrown when the bounds of the position are outside the pool's min/max tick range
error MinMaxBounds();
/// @notice Thrown when the ticks of the bounds do not align with tick spacing for concentrated pools
error BoundsTickSpacing();
/// @notice Thrown when stableswap pool positions are not at the min/max tick for the config
error StableswapMustBeFullRange();

function validate(PositionId positionId, PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (positionId.tickLower() >= positionId.tickUpper()) revert BoundsOrder();
        if (positionId.tickLower() < MIN_TICK || positionId.tickUpper() > MAX_TICK) revert MinMaxBounds();
        int32 spacing = int32(config.concentratedTickSpacing());
        if (positionId.tickLower() % spacing != 0 || positionId.tickUpper() % spacing != 0) revert BoundsTickSpacing();
    } else {
        (int32 lower, int32 upper) = config.stableswapActiveLiquidityTickRange();
        // For stableswap pools, positions must be exactly min/max tick
        if (positionId.tickLower() != lower || positionId.tickUpper() != upper) revert StableswapMustBeFullRange();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK} from "../math/constants.sol";
import {PoolConfig} from "./poolConfig.sol";

type PositionId is bytes32;

using {validate, salt, tickLower, tickUpper} for PositionId global;

function salt(PositionId positionId) pure returns (bytes24 v) {
    assembly ("memory-safe") {
        v := shl(64, shr(64, positionId))
    }
}

function tickLower(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // shift down, then signextend to 32 bits
        v := signextend(3, shr(32, positionId))
    }
}

function tickUpper(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // lowest 4 bytes, then signextend to 32 bits
        v := signextend(3, positionId)
    }
}

function createPositionId(bytes24 _salt, int32 _tickLower, int32 _tickUpper) pure returns (PositionId v) {
    assembly ("memory-safe") {
        // v = salt | (tickLower << 32) | tickUpper
        v := or(shl(64, shr(64, _salt)), or(shl(32, and(_tickLower, 0xFFFFFFFF)), and(_tickUpper, 0xFFFFFFFF)))
    }
}

/// @notice Thrown when the order of the position bounds is invalid, i.e. tickLower >= tickUpper
error BoundsOrder();
/// @notice Thrown when the bounds of the position are outside the pool's min/max tick range
error MinMaxBounds();
/// @notice Thrown when the ticks of the bounds do not align with tick spacing for concentrated pools
error BoundsTickSpacing();
/// @notice Thrown when stableswap pool positions are not at the min/max tick for the config
error StableswapMustBeFullRange();

function validate(PositionId positionId, PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (positionId.tickLower() >= positionId.tickUpper()) revert BoundsOrder();
        if (positionId.tickLower() < MIN_TICK || positionId.tickUpper() > MAX_TICK) revert MinMaxBounds();
        int32 spacing = int32(config.concentratedTickSpacing());
        if (positionId.tickLower() % spacing != 0 || positionId.tickUpper() % spacing != 0) revert BoundsTickSpacing();
    } else {
        (int32 lower, int32 upper) = config.stableswapActiveLiquidityTickRange();
        // For stableswap pools, positions must be exactly min/max tick
        if (positionId.tickLower() != lower || positionId.tickUpper() != upper) revert StableswapMustBeFullRange();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type TwammPoolState is bytes32;

using {
    lastVirtualOrderExecutionTime,
    realLastVirtualOrderExecutionTime,
    saleRateToken0,
    saleRateToken1,
    parse
} for TwammPoolState global;

function lastVirtualOrderExecutionTime(TwammPoolState state) pure returns (uint32 time) {
    assembly ("memory-safe") {
        time := and(state, 0xffffffff)
    }
}

function realLastVirtualOrderExecutionTime(TwammPoolState state) view returns (uint256 time) {
    assembly ("memory-safe") {
        time := sub(timestamp(), and(sub(and(timestamp(), 0xffffffff), and(state, 0xffffffff)), 0xffffffff))
    }
}

function saleRateToken0(TwammPoolState state) pure returns (uint112 rate) {
    assembly ("memory-safe") {
        rate := shr(144, shl(112, state))
    }
}

function saleRateToken1(TwammPoolState state) pure returns (uint112 rate) {
    assembly ("memory-safe") {
        rate := shr(144, state)
    }
}

function parse(TwammPoolState state) pure returns (uint32 time, uint112 rate0, uint112 rate1) {
    assembly ("memory-safe") {
        time := and(state, 0xffffffff)
        rate0 := shr(144, shl(112, state))
        rate1 := shr(144, state)
    }
}

function createTwammPoolState(uint32 _lastVirtualOrderExecutionTime, uint112 _saleRateToken0, uint112 _saleRateToken1)
    pure
    returns (TwammPoolState s)
{
    assembly ("memory-safe") {
        // s = (lastVirtualOrderExecutionTime) | (saleRateToken0 << 32) | (saleRateToken1 << 144)
        s := or(
            or(and(_lastVirtualOrderExecutionTime, 0xffffffff), shr(112, shl(144, _saleRateToken0))),
            shl(144, _saleRateToken1)
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, MIN_SQRT_RATIO_RAW, MAX_SQRT_RATIO_RAW} from "./sqrtRatio.sol";

type SwapParameters is bytes32;

using {
    sqrtRatioLimit,
    amount,
    isToken1,
    skipAhead,
    isExactOut,
    isPriceIncreasing,
    withDefaultSqrtRatioLimit
} for SwapParameters global;

function sqrtRatioLimit(SwapParameters params) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, params)
    }
}

function amount(SwapParameters params) pure returns (int128 a) {
    assembly ("memory-safe") {
        a := signextend(15, shr(32, params))
    }
}

function isToken1(SwapParameters params) pure returns (bool t) {
    assembly ("memory-safe") {
        t := and(shr(31, params), 1)
    }
}

function skipAhead(SwapParameters params) pure returns (uint256 s) {
    assembly ("memory-safe") {
        s := and(params, 0x7fffffff)
    }
}

function createSwapParameters(SqrtRatio _sqrtRatioLimit, int128 _amount, bool _isToken1, uint256 _skipAhead)
    pure
    returns (SwapParameters p)
{
    assembly ("memory-safe") {
        // p = (sqrtRatioLimit << 160) | (amount << 32) | (isToken1 << 31) | skipAhead
        // Mask each field to ensure dirty bits don't interfere
        // For isToken1, use iszero(iszero()) to convert any non-zero value to 1
        p := or(
            shl(160, _sqrtRatioLimit),
            or(
                shl(32, and(_amount, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                or(shl(31, iszero(iszero(_isToken1))), and(_skipAhead, 0x7fffffff))
            )
        )
    }
}

function isExactOut(SwapParameters params) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(159, params), 1)
    }
}

function isPriceIncreasing(SwapParameters params) pure returns (bool yes) {
    bool _isExactOut = params.isExactOut();
    bool _isToken1 = params.isToken1();
    assembly ("memory-safe") {
        yes := xor(_isExactOut, _isToken1)
    }
}

function withDefaultSqrtRatioLimit(SwapParameters params) pure returns (SwapParameters updated) {
    bool increasing = params.isPriceIncreasing();
    assembly ("memory-safe") {
        let replace := iszero(shr(160, params))
        let orMask :=
            shl(160, mul(replace, or(mul(increasing, MAX_SQRT_RATIO_RAW), mul(iszero(increasing), MIN_SQRT_RATIO_RAW))))
        updated := or(orMask, params)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/**
 * @title Bitmap (256-bit)
 * @notice Lightweight helpers for treating a `uint256` as a 256-bit bitmap.
 * @dev
 * - Bit indices are in the range [0, 255].
 * - All operations are O(1) and implemented with memory-safe assembly.
 * - For search helpers `leSetBit` and `geSetBit`, the return value is
 *   one-based: it returns `index + 1` of the matching set bit, or `0` if none.
 *   This convention avoids the need for sentinels outside the 0..255 range.
 */
type Bitmap is uint256;

using {toggle, isSet, leSetBit, geSetBit} for Bitmap global;

/**
 * @notice Toggle (flip) the bit at `index` in `bitmap`.
 * @param bitmap The current bitmap value.
 * @param index  Bit position to toggle, in [0, 255].
 * @return result A new bitmap with the bit at `index` flipped.
 */
function toggle(Bitmap bitmap, uint8 index) pure returns (Bitmap result) {
    assembly ("memory-safe") {
        result := xor(bitmap, shl(index, 1))
    }
}

/**
 * @notice Check whether the bit at `index` is set.
 * @param bitmap The bitmap to read.
 * @param index  Bit position to test, in [0, 255].
 * @return yes True if the bit is 1, false otherwise.
 */
function isSet(Bitmap bitmap, uint8 index) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(index, bitmap), 1)
    }
}

/**
 * @notice Find the most significant set bit at or below `index`.
 * @dev Returns one-based position: `pos = bitIndex + 1`, or `0` if none.
 *      Example: if bit 9 is set and `index >= 9`, returns `10`.
 *      For `index == 255`, the mask spans all bits (wraparound yields `2^256-1`).
 * @param bitmap The bitmap to search.
 * @param index  Upper bound (inclusive) for the search, in [0, 255].
 * @return v One-based position of MSB found (bitIndex + 1), or 0 if none.
 */
function leSetBit(Bitmap bitmap, uint8 index) pure returns (uint256 v) {
    unchecked {
        assembly ("memory-safe") {
            let masked := and(bitmap, sub(shl(add(index, 1), 1), 1))
            v := sub(256, clz(masked))
        }
    }
}

/**
 * @notice Find the least significant set bit at or above `index`.
 * @dev Returns one-based position: `pos = bitIndex + 1`, or `0` if none.
 *      Example: if bit 9 is set and `index <= 9`, returns `10`.
 * @param bitmap The bitmap to search.
 * @param index  Lower bound (inclusive) for the search, in [0, 255].
 * @return v One-based position of LSB found (bitIndex + 1), or 0 if none.
 */
function geSetBit(Bitmap bitmap, uint8 index) pure returns (uint256 v) {
    assembly ("memory-safe") {
        let masked := and(bitmap, not(sub(shl(index, 1), 1)))
        v := sub(256, clz(and(masked, sub(0, masked))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type OrderState is bytes32;

using {lastUpdateTime, saleRate, amountSold, parse} for OrderState global;

function lastUpdateTime(OrderState state) pure returns (uint32 time) {
    assembly ("memory-safe") {
        time := and(state, 0xffffffff)
    }
}

function saleRate(OrderState state) pure returns (uint112 rate) {
    assembly ("memory-safe") {
        rate := shr(144, shl(112, state))
    }
}

function amountSold(OrderState state) pure returns (uint112 amount) {
    assembly ("memory-safe") {
        amount := shr(144, state)
    }
}

function parse(OrderState state) pure returns (uint32 time, uint112 rate, uint112 amount) {
    assembly ("memory-safe") {
        time := and(state, 0xffffffff)
        rate := shr(144, shl(112, state))
        amount := shr(144, state)
    }
}

function createOrderState(uint32 _lastUpdateTime, uint112 _saleRate, uint112 _amountSold) pure returns (OrderState s) {
    assembly ("memory-safe") {
        // s = (lastUpdateTime) | (saleRate << 32) | (amountSold << 144)
        s := or(
            or(and(_lastUpdateTime, 0xffffffff), shl(32, shr(144, shl(144, _saleRate)))),
            shl(144, shr(144, shl(144, _amountSold)))
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, toSqrtRatio} from "../types/sqrtRatio.sol";
import {computeFee} from "./fee.sol";
import {exp2} from "./exp2.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

error SaleRateOverflow();

/// @dev Computes sale rate = (amount << 32) / duration and reverts if the result exceeds type(uint112).max.
/// @dev Assumes duration > 0 and amount <= type(uint224).max.
function computeSaleRate(uint256 amount, uint256 duration) pure returns (uint256 saleRate) {
    assembly ("memory-safe") {
        saleRate := div(shl(32, amount), duration)
        if shr(112, saleRate) {
            // cast sig "SaleRateOverflow()"
            mstore(0, shl(224, 0x83c87460))
            revert(0, 4)
        }
    }
}

error SaleRateDeltaOverflow();

/// @dev Adds the sale rate delta to the saleRate and reverts if the result is greater than type(uint112).max
/// @dev Assumes saleRate <= type(uint112).max and saleRateDelta <= type(int112).max and saleRateDelta >= type(int112).min
function addSaleRateDelta(uint256 saleRate, int256 saleRateDelta) pure returns (uint256 result) {
    assembly ("memory-safe") {
        result := add(saleRate, saleRateDelta)
        // if any of the upper bits are non-zero, revert
        if shr(112, result) {
            // cast sig "SaleRateDeltaOverflow()"
            mstore(0, shl(224, 0xc902643d))
            revert(0, 4)
        }
    }
}

/// @dev Computes amount from sale rate: (saleRate * duration) >> 32, with optional rounding.
/// @dev Assumes the saleRate <= type(uint112).max and duration <= type(uint32).max
function computeAmountFromSaleRate(uint256 saleRate, uint256 duration, bool roundUp) pure returns (uint256 amount) {
    assembly ("memory-safe") {
        amount := shr(32, add(mul(saleRate, duration), mul(0xffffffff, roundUp)))
    }
}

/// @dev Computes reward amount = (rewardRate * saleRate) >> 128.
/// @dev saleRate is assumed to be <= type(uint112).max, thus this function is never expected to overflow
function computeRewardAmount(uint256 rewardRate, uint256 saleRate) pure returns (uint128) {
    return uint128(FixedPointMathLib.fullMulDivN(rewardRate, saleRate, 128));
}

/// @dev Computes the quantity `c = (sqrtSaleRatio - sqrtRatio) / (sqrtSaleRatio + sqrtRatio)` as a signed 64.128 number
/// @dev sqrtRatio is assumed to be between 2**192 and 2**-64, while sqrtSaleRatio values are assumed to be between 2**184 and 2**-72
function computeC(uint256 sqrtRatio, uint256 sqrtSaleRatio) pure returns (int256 c) {
    uint256 unsigned = FixedPointMathLib.fullMulDiv(
        FixedPointMathLib.dist(sqrtRatio, sqrtSaleRatio), (1 << 128), sqrtRatio + sqrtSaleRatio
    );
    assembly ("memory-safe") {
        let sign := sub(shl(1, gt(sqrtSaleRatio, sqrtRatio)), 1)
        c := mul(sign, unsigned)
    }
}

/// @dev Returns a 64.128 number representing the sqrt sale ratio
/// @dev Assumes both saleRateToken0 and saleRateToken1 are nonzero and <= type(uint112).max
function computeSqrtSaleRatio(uint256 saleRateToken0, uint256 saleRateToken1) pure returns (uint256 sqrtSaleRatio) {
    unchecked {
        uint256 saleRatio = FixedPointMathLib.rawDiv(saleRateToken1 << 128, saleRateToken0);

        if (saleRatio <= type(uint128).max) {
            // full precision for small ratios
            sqrtSaleRatio = FixedPointMathLib.sqrt(saleRatio << 128);
        } else if (saleRatio <= type(uint192).max) {
            // we know it only has 192 bits, so we can shift it 64 before rooting to get more precision
            sqrtSaleRatio = FixedPointMathLib.sqrt(saleRatio << 64) << 32;
        } else {
            // we assume it has max 240 bits, since saleRateToken1 is 112 bits and we shifted left 128
            sqrtSaleRatio = FixedPointMathLib.sqrt(saleRatio << 16) << 56;
        }
    }
}

/// @dev Computes the next sqrt ratio according to the state of the TWAMM for a fixed sale rate
/// @dev Assumes both sale rates are != 0 and <= type(uint112).max
/// @dev Assumes liquidity is <= type(uint128).max
/// @dev Assumes timeElapsed is <= type(uint32).max
function computeNextSqrtRatio(
    SqrtRatio sqrtRatio,
    uint256 liquidity,
    uint256 saleRateToken0,
    uint256 saleRateToken1,
    uint256 timeElapsed,
    uint64 fee
) pure returns (SqrtRatio sqrtRatioNext) {
    unchecked {
        // assumed:
        //   assert(saleRateToken0 != 0 && saleRateToken1 != 0);
        uint256 sqrtSaleRatio = computeSqrtSaleRatio(saleRateToken0, saleRateToken1);

        uint256 sqrtRatioFixed = sqrtRatio.toFixed();
        bool roundUp = sqrtRatioFixed > sqrtSaleRatio;

        int256 c = computeC(sqrtRatioFixed, sqrtSaleRatio);

        if (c == 0 || liquidity == 0) {
            // if liquidity is 0, we just settle the ratio of sale rates since the liquidity provides no friction to the price movement
            // if c is 0, that means the difference b/t sale ratio and sqrt ratio is too small to be detected
            // so we just assume it settles at the sale ratio
            sqrtRatioNext = toSqrtRatio(sqrtSaleRatio, roundUp);
        } else {
            uint256 sqrtSaleRateWithoutFee = FixedPointMathLib.sqrt(saleRateToken0 * saleRateToken1);
            // max 112 bits
            uint256 sqrtSaleRate = sqrtSaleRateWithoutFee - computeFee(uint128(sqrtSaleRateWithoutFee), fee);

            // (12392656037 * t * sqrtSaleRate) / liquidity == (34 + 32 + 128) - 128 bits, cannot overflow
            // uint256(12392656037) = Math.floor(Math.LOG2E * 2**33).
            // this combines the doubling, the left shifting and the converting to a base 2 exponent into a single multiplication
            uint256 exponent = FixedPointMathLib.rawDiv(sqrtSaleRate * timeElapsed * 12392656037, liquidity);
            if (exponent >= 0x400000000000000000) {
                // if the exponent is larger than this value (64), the exponent term dominates and the result is approximately the sell ratio
                sqrtRatioNext = toSqrtRatio(sqrtSaleRatio, roundUp);
            } else {
                int256 ePowExponent = int256(uint256(exp2(uint128(exponent))) << 64);

                uint256 sqrtRatioNextFixed = FixedPointMathLib.fullMulDiv(
                    sqrtSaleRatio, FixedPointMathLib.dist(ePowExponent, c), FixedPointMathLib.abs(ePowExponent + c)
                );

                // we should never exceed the sale ratio
                if (roundUp) {
                    sqrtRatioNextFixed = FixedPointMathLib.max(sqrtRatioNextFixed, sqrtSaleRatio);
                } else {
                    sqrtRatioNextFixed = FixedPointMathLib.min(sqrtRatioNextFixed, sqrtSaleRatio);
                }

                sqrtRatioNext = toSqrtRatio(sqrtRatioNextFixed, roundUp);
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a TWAMM order
/// @dev Wraps bytes32 to provide type safety for order identifiers
type OrderId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK} from "../math/constants.sol";
import {PoolConfig} from "./poolConfig.sol";

type PositionId is bytes32;

using {validate, salt, tickLower, tickUpper} for PositionId global;

function salt(PositionId positionId) pure returns (bytes24 v) {
    assembly ("memory-safe") {
        v := shl(64, shr(64, positionId))
    }
}

function tickLower(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // shift down, then signextend to 32 bits
        v := signextend(3, shr(32, positionId))
    }
}

function tickUpper(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // lowest 4 bytes, then signextend to 32 bits
        v := signextend(3, positionId)
    }
}

function createPositionId(bytes24 _salt, int32 _tickLower, int32 _tickUpper) pure returns (PositionId v) {
    assembly ("memory-safe") {
        // v = salt | (tickLower << 32) | tickUpper
        v := or(shl(64, shr(64, _salt)), or(shl(32, and(_tickLower, 0xFFFFFFFF)), and(_tickUpper, 0xFFFFFFFF)))
    }
}

/// @notice Thrown when the order of the position bounds is invalid, i.e. tickLower >= tickUpper
error BoundsOrder();
/// @notice Thrown when the bounds of the position are outside the pool's min/max tick range
error MinMaxBounds();
/// @notice Thrown when the ticks of the bounds do not align with tick spacing for concentrated pools
error BoundsTickSpacing();
/// @notice Thrown when stableswap pool positions are not at the min/max tick for the config
error StableswapMustBeFullRange();

function validate(PositionId positionId, PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (positionId.tickLower() >= positionId.tickUpper()) revert BoundsOrder();
        if (positionId.tickLower() < MIN_TICK || positionId.tickUpper() > MAX_TICK) revert MinMaxBounds();
        int32 spacing = int32(config.concentratedTickSpacing());
        if (positionId.tickLower() % spacing != 0 || positionId.tickUpper() % spacing != 0) revert BoundsTickSpacing();
    } else {
        (int32 lower, int32 upper) = config.stableswapActiveLiquidityTickRange();
        // For stableswap pools, positions must be exactly min/max tick
        if (positionId.tickLower() != lower || positionId.tickUpper() != upper) revert StableswapMustBeFullRange();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type BuybacksState is bytes32;

using {
    targetOrderDuration,
    minOrderDuration,
    fee,
    lastEndTime,
    lastOrderDuration,
    lastFee,
    isConfigured,
    parse
} for BuybacksState global;

function targetOrderDuration(BuybacksState state) pure returns (uint32 duration) {
    assembly ("memory-safe") {
        duration := and(state, 0xFFFFFFFF)
    }
}

function minOrderDuration(BuybacksState state) pure returns (uint32 duration) {
    assembly ("memory-safe") {
        duration := and(shr(32, state), 0xFFFFFFFF)
    }
}

function fee(BuybacksState state) pure returns (uint64 f) {
    assembly ("memory-safe") {
        f := and(shr(64, state), 0xFFFFFFFFFFFFFFFF)
    }
}

function lastEndTime(BuybacksState state) pure returns (uint32 endTime) {
    assembly ("memory-safe") {
        endTime := and(shr(128, state), 0xFFFFFFFF)
    }
}

function lastOrderDuration(BuybacksState state) pure returns (uint32 duration) {
    assembly ("memory-safe") {
        duration := and(shr(160, state), 0xFFFFFFFF)
    }
}

function lastFee(BuybacksState state) pure returns (uint64 f) {
    assembly ("memory-safe") {
        f := shr(192, state)
    }
}

function isConfigured(BuybacksState state) pure returns (bool) {
    return minOrderDuration(state) != 0;
}

function parse(BuybacksState state)
    pure
    returns (
        uint32 _targetOrderDuration,
        uint32 _minOrderDuration,
        uint64 _fee,
        uint32 _lastEndTime,
        uint32 _lastOrderDuration,
        uint64 _lastFee
    )
{
    assembly ("memory-safe") {
        _targetOrderDuration := and(state, 0xFFFFFFFF)
        _minOrderDuration := and(shr(32, state), 0xFFFFFFFF)
        _fee := and(shr(64, state), 0xFFFFFFFFFFFFFFFF)
        _lastEndTime := and(shr(128, state), 0xFFFFFFFF)
        _lastOrderDuration := and(shr(160, state), 0xFFFFFFFF)
        _lastFee := shr(192, state)
    }
}

function createBuybacksState(
    uint32 _targetOrderDuration,
    uint32 _minOrderDuration,
    uint64 _fee,
    uint32 _lastEndTime,
    uint32 _lastOrderDuration,
    uint64 _lastFee
) pure returns (BuybacksState state) {
    assembly ("memory-safe") {
        state := or(
            or(
                or(and(_targetOrderDuration, 0xFFFFFFFF), shl(32, and(_minOrderDuration, 0xFFFFFFFF))),
                shl(64, and(_fee, 0xFFFFFFFFFFFFFFFF))
            ),
            or(
                or(shl(128, and(_lastEndTime, 0xFFFFFFFF)), shl(160, and(_lastOrderDuration, 0xFFFFFFFF))),
                shl(192, _lastFee)
            )
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice A drop is specified by an owner, token and a root
/// @dev The owner can reclaim the drop token at any time
///      The root is the root of a merkle trie that contains all the incentives to be distributed
struct DropKey {
    /// @notice Address that owns the drop and can reclaim tokens
    address owner;
    /// @notice Token address for the drop
    address token;
    /// @notice Merkle root of the incentive distribution tree
    bytes32 root;
}

using {toDropId} for DropKey global;

/// @notice Returns the identifier of the drop
/// @param key The drop key to hash
/// @return h The unique drop identifier
function toDropId(DropKey memory key) pure returns (bytes32 h) {
    assembly ("memory-safe") {
        // assumes that owner, token have no dirty upper bits
        h := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

// For any given time `t`, there are up to 91 times that are greater than `t` and valid according to `isTimeValid`
uint256 constant MAX_NUM_VALID_TIMES = 91;

// If we constrain the sale rate delta to this value, then the current sale rate will never overflow
uint256 constant MAX_ABS_VALUE_SALE_RATE_DELTA = type(uint112).max / MAX_NUM_VALID_TIMES;

/// @dev Returns the step size, i.e. the value of which the order end or start time must be a multiple of, based on the current time and the specified time
///      The step size has a minimum of 256 seconds and increases in powers of 16 as the gap to `time` grows.
///      Assumes currentTime < type(uint256).max - 4095
/// @param currentTime The current block timestamp
/// @param time The time for which the step size is being computed, based on how far in the future it is from currentTime
function computeStepSize(uint256 currentTime, uint256 time) pure returns (uint256 stepSize) {
    assembly ("memory-safe") {
        switch gt(time, add(currentTime, 4095))
        case 1 {
            let diff := sub(time, currentTime)

            let msb := sub(255, clz(diff)) // = index of msb

            msb := sub(msb, mod(msb, 4)) // = round down to multiple of 4

            stepSize := shl(msb, 1)
        }
        default { stepSize := 256 }
    }
}

/// @dev Returns true iff the given time is a valid start or end time for a TWAMM order
function isTimeValid(uint256 currentTime, uint256 time) pure returns (bool valid) {
    uint256 stepSize = computeStepSize(currentTime, time);

    assembly ("memory-safe") {
        valid := and(iszero(mod(time, stepSize)), or(lt(time, currentTime), lt(sub(time, currentTime), 0x100000000)))
    }
}

/// @dev Returns the next valid time if there is one, or wraps around to the time 0 if there is not
///      Assumes currentTime is less than type(uint256).max - type(uint32).max
function nextValidTime(uint256 currentTime, uint256 time) pure returns (uint256 nextTime) {
    unchecked {
        uint256 stepSize = computeStepSize(currentTime, time);
        assembly ("memory-safe") {
            nextTime := add(time, stepSize)
            nextTime := sub(nextTime, mod(nextTime, stepSize))
        }

        // only if we didn't overflow
        if (nextTime != 0) {
            uint256 nextStepSize = computeStepSize(currentTime, nextTime);
            if (nextStepSize != stepSize) {
                assembly ("memory-safe") {
                    nextTime := add(time, nextStepSize)
                    nextTime := sub(nextTime, mod(nextTime, nextStepSize))
                }
            }
        }

        nextTime = FixedPointMathLib.ternary(nextTime > currentTime + type(uint32).max, 0, nextTime);
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FeesPerLiquidity} from "./feesPerLiquidity.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

// Position Type
// Represents a liquidity position in a pool
// Contains the position's liquidity amount and fee tracking information

/// @notice A liquidity position in a pool
/// @dev Tracks both the liquidity amount and the last known fees per liquidity for fee calculation
struct Position {
    /// @notice Extra data that can be set by the owner of a position
    bytes16 extraData;
    /// @notice Amount of liquidity in the position
    uint128 liquidity;
    /// @notice Snapshot of fees per liquidity when the position was last updated
    /// @dev Used to calculate fees owed to the position holder
    FeesPerLiquidity feesPerLiquidityInsideLast;
}

using {fees} for Position global;

/// @notice Calculates the fees owed to a position
/// @dev Returns the fee amounts of token0 and token1 owed to a position based on the given fees per liquidity inside snapshot
///      Note: if the computed fees overflow the uint128 type, it will return only the lower 128 bits. It is assumed that accumulated
///      fees will never exceed type(uint128).max.
/// @param position The position to calculate fees for
/// @param feesPerLiquidityInside Current fees per liquidity inside the position's bounds
/// @return Amount of token0 fees owed
/// @return Amount of token1 fees owed
function fees(Position memory position, FeesPerLiquidity memory feesPerLiquidityInside)
    pure
    returns (uint128, uint128)
{
    uint128 liquidity;
    uint256 difference0;
    uint256 difference1;
    assembly ("memory-safe") {
        liquidity := mload(add(position, 0x20))
        // feesPerLiquidityInsideLast is now at offset 0x40 due to extraData field
        let positionFpl := mload(add(position, 0x40))
        difference0 := sub(mload(feesPerLiquidityInside), mload(positionFpl))
        difference1 := sub(mload(add(feesPerLiquidityInside, 0x20)), mload(add(positionFpl, 0x20)))
    }

    return (
        uint128(FixedPointMathLib.fullMulDivN(difference0, liquidity, 128)),
        uint128(FixedPointMathLib.fullMulDivN(difference1, liquidity, 128))
    );
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {DateTimeLib} from "solady/utils/DateTimeLib.sol";
import {LibString} from "solady/utils/LibString.sol";

error UnrecognizedMonth();

// Returns the 3-letter month abbreviation for a given month number (1-12)
function getMonthAbbreviation(uint256 month) pure returns (string memory) {
    if (month == 1) return "Jan";
    if (month == 2) return "Feb";
    if (month == 3) return "Mar";
    if (month == 4) return "Apr";
    if (month == 5) return "May";
    if (month == 6) return "Jun";
    if (month == 7) return "Jul";
    if (month == 8) return "Aug";
    if (month == 9) return "Sep";
    if (month == 10) return "Oct";
    if (month == 11) return "Nov";
    if (month == 12) return "Dec";
    revert UnrecognizedMonth();
}

function toQuarter(uint256 unlockTime) pure returns (string memory quarterLabel) {
    (uint256 year, uint256 month,) = DateTimeLib.timestampToDate(unlockTime);
    year = year % 100;
    string memory shortenedYearStr = LibString.toString(year);

    unchecked {
        quarterLabel =
            string.concat(year < 10 ? "0" : "", shortenedYearStr, "Q", LibString.toString(1 + (month - 1) / 3));
    }
}

function toDate(uint256 unlockTime) pure returns (string memory dateLabel) {
    (uint256 year, uint256 month, uint256 day) = DateTimeLib.timestampToDate(unlockTime);
    string memory yearStr = LibString.toString(year);
    string memory monthStr = getMonthAbbreviation(month);
    string memory dayStr = LibString.toString(day);

    dateLabel = string.concat(monthStr, "/", dayStr, "/", yearStr);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice A drop is specified by an owner, token and a root
/// @dev The owner can reclaim the drop token at any time
///      The root is the root of a merkle trie that contains all the incentives to be distributed
struct DropKey {
    /// @notice Address that owns the drop and can reclaim tokens
    address owner;
    /// @notice Token address for the drop
    address token;
    /// @notice Merkle root of the incentive distribution tree
    bytes32 root;
}

using {toDropId} for DropKey global;

/// @notice Returns the identifier of the drop
/// @param key The drop key to hash
/// @return h The unique drop identifier
function toDropId(DropKey memory key) pure returns (bytes32 h) {
    assembly ("memory-safe") {
        // assumes that owner, token have no dirty upper bits
        h := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, toSqrtRatio} from "../types/sqrtRatio.sol";
import {computeFee} from "./fee.sol";
import {exp2} from "./exp2.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

error SaleRateOverflow();

/// @dev Computes sale rate = (amount << 32) / duration and reverts if the result exceeds type(uint112).max.
/// @dev Assumes duration > 0 and amount <= type(uint224).max.
function computeSaleRate(uint256 amount, uint256 duration) pure returns (uint256 saleRate) {
    assembly ("memory-safe") {
        saleRate := div(shl(32, amount), duration)
        if shr(112, saleRate) {
            // cast sig "SaleRateOverflow()"
            mstore(0, shl(224, 0x83c87460))
            revert(0, 4)
        }
    }
}

error SaleRateDeltaOverflow();

/// @dev Adds the sale rate delta to the saleRate and reverts if the result is greater than type(uint112).max
/// @dev Assumes saleRate <= type(uint112).max and saleRateDelta <= type(int112).max and saleRateDelta >= type(int112).min
function addSaleRateDelta(uint256 saleRate, int256 saleRateDelta) pure returns (uint256 result) {
    assembly ("memory-safe") {
        result := add(saleRate, saleRateDelta)
        // if any of the upper bits are non-zero, revert
        if shr(112, result) {
            // cast sig "SaleRateDeltaOverflow()"
            mstore(0, shl(224, 0xc902643d))
            revert(0, 4)
        }
    }
}

/// @dev Computes amount from sale rate: (saleRate * duration) >> 32, with optional rounding.
/// @dev Assumes the saleRate <= type(uint112).max and duration <= type(uint32).max
function computeAmountFromSaleRate(uint256 saleRate, uint256 duration, bool roundUp) pure returns (uint256 amount) {
    assembly ("memory-safe") {
        amount := shr(32, add(mul(saleRate, duration), mul(0xffffffff, roundUp)))
    }
}

/// @dev Computes reward amount = (rewardRate * saleRate) >> 128.
/// @dev saleRate is assumed to be <= type(uint112).max, thus this function is never expected to overflow
function computeRewardAmount(uint256 rewardRate, uint256 saleRate) pure returns (uint128) {
    return uint128(FixedPointMathLib.fullMulDivN(rewardRate, saleRate, 128));
}

/// @dev Computes the quantity `c = (sqrtSaleRatio - sqrtRatio) / (sqrtSaleRatio + sqrtRatio)` as a signed 64.128 number
/// @dev sqrtRatio is assumed to be between 2**192 and 2**-64, while sqrtSaleRatio values are assumed to be between 2**184 and 2**-72
function computeC(uint256 sqrtRatio, uint256 sqrtSaleRatio) pure returns (int256 c) {
    uint256 unsigned = FixedPointMathLib.fullMulDiv(
        FixedPointMathLib.dist(sqrtRatio, sqrtSaleRatio), (1 << 128), sqrtRatio + sqrtSaleRatio
    );
    assembly ("memory-safe") {
        let sign := sub(shl(1, gt(sqrtSaleRatio, sqrtRatio)), 1)
        c := mul(sign, unsigned)
    }
}

/// @dev Returns a 64.128 number representing the sqrt sale ratio
/// @dev Assumes both saleRateToken0 and saleRateToken1 are nonzero and <= type(uint112).max
function computeSqrtSaleRatio(uint256 saleRateToken0, uint256 saleRateToken1) pure returns (uint256 sqrtSaleRatio) {
    unchecked {
        uint256 saleRatio = FixedPointMathLib.rawDiv(saleRateToken1 << 128, saleRateToken0);

        if (saleRatio <= type(uint128).max) {
            // full precision for small ratios
            sqrtSaleRatio = FixedPointMathLib.sqrt(saleRatio << 128);
        } else if (saleRatio <= type(uint192).max) {
            // we know it only has 192 bits, so we can shift it 64 before rooting to get more precision
            sqrtSaleRatio = FixedPointMathLib.sqrt(saleRatio << 64) << 32;
        } else {
            // we assume it has max 240 bits, since saleRateToken1 is 112 bits and we shifted left 128
            sqrtSaleRatio = FixedPointMathLib.sqrt(saleRatio << 16) << 56;
        }
    }
}

/// @dev Computes the next sqrt ratio according to the state of the TWAMM for a fixed sale rate
/// @dev Assumes both sale rates are != 0 and <= type(uint112).max
/// @dev Assumes liquidity is <= type(uint128).max
/// @dev Assumes timeElapsed is <= type(uint32).max
function computeNextSqrtRatio(
    SqrtRatio sqrtRatio,
    uint256 liquidity,
    uint256 saleRateToken0,
    uint256 saleRateToken1,
    uint256 timeElapsed,
    uint64 fee
) pure returns (SqrtRatio sqrtRatioNext) {
    unchecked {
        // assumed:
        //   assert(saleRateToken0 != 0 && saleRateToken1 != 0);
        uint256 sqrtSaleRatio = computeSqrtSaleRatio(saleRateToken0, saleRateToken1);

        uint256 sqrtRatioFixed = sqrtRatio.toFixed();
        bool roundUp = sqrtRatioFixed > sqrtSaleRatio;

        int256 c = computeC(sqrtRatioFixed, sqrtSaleRatio);

        if (c == 0 || liquidity == 0) {
            // if liquidity is 0, we just settle the ratio of sale rates since the liquidity provides no friction to the price movement
            // if c is 0, that means the difference b/t sale ratio and sqrt ratio is too small to be detected
            // so we just assume it settles at the sale ratio
            sqrtRatioNext = toSqrtRatio(sqrtSaleRatio, roundUp);
        } else {
            uint256 sqrtSaleRateWithoutFee = FixedPointMathLib.sqrt(saleRateToken0 * saleRateToken1);
            // max 112 bits
            uint256 sqrtSaleRate = sqrtSaleRateWithoutFee - computeFee(uint128(sqrtSaleRateWithoutFee), fee);

            // (12392656037 * t * sqrtSaleRate) / liquidity == (34 + 32 + 128) - 128 bits, cannot overflow
            // uint256(12392656037) = Math.floor(Math.LOG2E * 2**33).
            // this combines the doubling, the left shifting and the converting to a base 2 exponent into a single multiplication
            uint256 exponent = FixedPointMathLib.rawDiv(sqrtSaleRate * timeElapsed * 12392656037, liquidity);
            if (exponent >= 0x400000000000000000) {
                // if the exponent is larger than this value (64), the exponent term dominates and the result is approximately the sell ratio
                sqrtRatioNext = toSqrtRatio(sqrtSaleRatio, roundUp);
            } else {
                int256 ePowExponent = int256(uint256(exp2(uint128(exponent))) << 64);

                uint256 sqrtRatioNextFixed = FixedPointMathLib.fullMulDiv(
                    sqrtSaleRatio, FixedPointMathLib.dist(ePowExponent, c), FixedPointMathLib.abs(ePowExponent + c)
                );

                // we should never exceed the sale ratio
                if (roundUp) {
                    sqrtRatioNextFixed = FixedPointMathLib.max(sqrtRatioNextFixed, sqrtSaleRatio);
                } else {
                    sqrtRatioNextFixed = FixedPointMathLib.min(sqrtRatioNextFixed, sqrtSaleRatio);
                }

                sqrtRatioNext = toSqrtRatio(sqrtRatioNextFixed, roundUp);
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice A claim is an individual leaf in the merkle trie
struct ClaimKey {
    /// @notice Index of the claim in the merkle tree
    uint256 index;
    /// @notice Account that can claim the incentive
    address account;
    /// @notice Amount of tokens to be claimed
    uint128 amount;
}

using {toClaimId} for ClaimKey global;

/// @notice Hashes a claim for merkle proof verification
/// @param c The claim to hash
/// @return h The hash of the claim
function toClaimId(ClaimKey memory c) pure returns (bytes32 h) {
    assembly ("memory-safe") {
        // assumes that account has no dirty upper bits
        h := keccak256(c, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

struct CallPoints {
    bool beforeInitializePool;
    bool afterInitializePool;
    bool beforeSwap;
    bool afterSwap;
    bool beforeUpdatePosition;
    bool afterUpdatePosition;
    bool beforeCollectFees;
    bool afterCollectFees;
}

using {eq, isValid, toUint8} for CallPoints global;

function eq(CallPoints memory a, CallPoints memory b) pure returns (bool) {
    return (a.beforeInitializePool == b.beforeInitializePool && a.afterInitializePool == b.afterInitializePool
            && a.beforeSwap == b.beforeSwap && a.afterSwap == b.afterSwap
            && a.beforeUpdatePosition == b.beforeUpdatePosition && a.afterUpdatePosition == b.afterUpdatePosition
            && a.beforeCollectFees == b.beforeCollectFees && a.afterCollectFees == b.afterCollectFees);
}

function isValid(CallPoints memory a) pure returns (bool) {
    return (a.beforeInitializePool || a.afterInitializePool || a.beforeSwap || a.afterSwap || a.beforeUpdatePosition
            || a.afterUpdatePosition || a.beforeCollectFees || a.afterCollectFees);
}

function toUint8(CallPoints memory callPoints) pure returns (uint8 b) {
    assembly ("memory-safe") {
        b := add(
            add(
                add(
                    add(
                        add(
                            add(
                                add(mload(callPoints), mul(128, mload(add(callPoints, 32)))),
                                mul(64, mload(add(callPoints, 64)))
                            ),
                            mul(32, mload(add(callPoints, 96)))
                        ),
                        mul(16, mload(add(callPoints, 128)))
                    ),
                    mul(8, mload(add(callPoints, 160)))
                ),
                mul(4, mload(add(callPoints, 192)))
            ),
            mul(2, mload(add(callPoints, 224)))
        )
    }
}

function addressToCallPoints(address a) pure returns (CallPoints memory result) {
    result = byteToCallPoints(uint8(uint160(a) >> 152));
}

function byteToCallPoints(uint8 b) pure returns (CallPoints memory result) {
    // note the order of bytes does not match the struct order of elements because we are matching the cairo implementation
    // which for legacy reasons has the fields in this order
    result = CallPoints({
        beforeInitializePool: (b & 1) != 0,
        afterInitializePool: (b & 128) != 0,
        beforeSwap: (b & 64) != 0,
        afterSwap: (b & 32) != 0,
        beforeUpdatePosition: (b & 16) != 0,
        afterUpdatePosition: (b & 8) != 0,
        beforeCollectFees: (b & 4) != 0,
        afterCollectFees: (b & 2) != 0
    });
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, MIN_SQRT_RATIO_RAW, MAX_SQRT_RATIO_RAW} from "./sqrtRatio.sol";

type SwapParameters is bytes32;

using {
    sqrtRatioLimit,
    amount,
    isToken1,
    skipAhead,
    isExactOut,
    isPriceIncreasing,
    withDefaultSqrtRatioLimit
} for SwapParameters global;

function sqrtRatioLimit(SwapParameters params) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, params)
    }
}

function amount(SwapParameters params) pure returns (int128 a) {
    assembly ("memory-safe") {
        a := signextend(15, shr(32, params))
    }
}

function isToken1(SwapParameters params) pure returns (bool t) {
    assembly ("memory-safe") {
        t := and(shr(31, params), 1)
    }
}

function skipAhead(SwapParameters params) pure returns (uint256 s) {
    assembly ("memory-safe") {
        s := and(params, 0x7fffffff)
    }
}

function createSwapParameters(SqrtRatio _sqrtRatioLimit, int128 _amount, bool _isToken1, uint256 _skipAhead)
    pure
    returns (SwapParameters p)
{
    assembly ("memory-safe") {
        // p = (sqrtRatioLimit << 160) | (amount << 32) | (isToken1 << 31) | skipAhead
        // Mask each field to ensure dirty bits don't interfere
        // For isToken1, use iszero(iszero()) to convert any non-zero value to 1
        p := or(
            shl(160, _sqrtRatioLimit),
            or(
                shl(32, and(_amount, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                or(shl(31, iszero(iszero(_isToken1))), and(_skipAhead, 0x7fffffff))
            )
        )
    }
}

function isExactOut(SwapParameters params) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(159, params), 1)
    }
}

function isPriceIncreasing(SwapParameters params) pure returns (bool yes) {
    bool _isExactOut = params.isExactOut();
    bool _isToken1 = params.isToken1();
    assembly ("memory-safe") {
        yes := xor(_isExactOut, _isToken1)
    }
}

function withDefaultSqrtRatioLimit(SwapParameters params) pure returns (SwapParameters updated) {
    bool increasing = params.isPriceIncreasing();
    assembly ("memory-safe") {
        let replace := iszero(shr(160, params))
        let orMask :=
            shl(160, mul(replace, or(mul(increasing, MAX_SQRT_RATIO_RAW), mul(iszero(increasing), MIN_SQRT_RATIO_RAW))))
        updated := or(orMask, params)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK} from "../math/constants.sol";
import {PoolConfig} from "./poolConfig.sol";

type PositionId is bytes32;

using {validate, salt, tickLower, tickUpper} for PositionId global;

function salt(PositionId positionId) pure returns (bytes24 v) {
    assembly ("memory-safe") {
        v := shl(64, shr(64, positionId))
    }
}

function tickLower(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // shift down, then signextend to 32 bits
        v := signextend(3, shr(32, positionId))
    }
}

function tickUpper(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // lowest 4 bytes, then signextend to 32 bits
        v := signextend(3, positionId)
    }
}

function createPositionId(bytes24 _salt, int32 _tickLower, int32 _tickUpper) pure returns (PositionId v) {
    assembly ("memory-safe") {
        // v = salt | (tickLower << 32) | tickUpper
        v := or(shl(64, shr(64, _salt)), or(shl(32, and(_tickLower, 0xFFFFFFFF)), and(_tickUpper, 0xFFFFFFFF)))
    }
}

/// @notice Thrown when the order of the position bounds is invalid, i.e. tickLower >= tickUpper
error BoundsOrder();
/// @notice Thrown when the bounds of the position are outside the pool's min/max tick range
error MinMaxBounds();
/// @notice Thrown when the ticks of the bounds do not align with tick spacing for concentrated pools
error BoundsTickSpacing();
/// @notice Thrown when stableswap pool positions are not at the min/max tick for the config
error StableswapMustBeFullRange();

function validate(PositionId positionId, PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (positionId.tickLower() >= positionId.tickUpper()) revert BoundsOrder();
        if (positionId.tickLower() < MIN_TICK || positionId.tickUpper() > MAX_TICK) revert MinMaxBounds();
        int32 spacing = int32(config.concentratedTickSpacing());
        if (positionId.tickLower() % spacing != 0 || positionId.tickUpper() % spacing != 0) revert BoundsTickSpacing();
    } else {
        (int32 lower, int32 upper) = config.stableswapActiveLiquidityTickRange();
        // For stableswap pools, positions must be exactly min/max tick
        if (positionId.tickLower() != lower || positionId.tickUpper() != upper) revert StableswapMustBeFullRange();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/**
 * @title Bitmap (256-bit)
 * @notice Lightweight helpers for treating a `uint256` as a 256-bit bitmap.
 * @dev
 * - Bit indices are in the range [0, 255].
 * - All operations are O(1) and implemented with memory-safe assembly.
 * - For search helpers `leSetBit` and `geSetBit`, the return value is
 *   one-based: it returns `index + 1` of the matching set bit, or `0` if none.
 *   This convention avoids the need for sentinels outside the 0..255 range.
 */
type Bitmap is uint256;

using {toggle, isSet, leSetBit, geSetBit} for Bitmap global;

/**
 * @notice Toggle (flip) the bit at `index` in `bitmap`.
 * @param bitmap The current bitmap value.
 * @param index  Bit position to toggle, in [0, 255].
 * @return result A new bitmap with the bit at `index` flipped.
 */
function toggle(Bitmap bitmap, uint8 index) pure returns (Bitmap result) {
    assembly ("memory-safe") {
        result := xor(bitmap, shl(index, 1))
    }
}

/**
 * @notice Check whether the bit at `index` is set.
 * @param bitmap The bitmap to read.
 * @param index  Bit position to test, in [0, 255].
 * @return yes True if the bit is 1, false otherwise.
 */
function isSet(Bitmap bitmap, uint8 index) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(index, bitmap), 1)
    }
}

/**
 * @notice Find the most significant set bit at or below `index`.
 * @dev Returns one-based position: `pos = bitIndex + 1`, or `0` if none.
 *      Example: if bit 9 is set and `index >= 9`, returns `10`.
 *      For `index == 255`, the mask spans all bits (wraparound yields `2^256-1`).
 * @param bitmap The bitmap to search.
 * @param index  Upper bound (inclusive) for the search, in [0, 255].
 * @return v One-based position of MSB found (bitIndex + 1), or 0 if none.
 */
function leSetBit(Bitmap bitmap, uint8 index) pure returns (uint256 v) {
    unchecked {
        assembly ("memory-safe") {
            let masked := and(bitmap, sub(shl(add(index, 1), 1), 1))
            v := sub(256, clz(masked))
        }
    }
}

/**
 * @notice Find the least significant set bit at or above `index`.
 * @dev Returns one-based position: `pos = bitIndex + 1`, or `0` if none.
 *      Example: if bit 9 is set and `index <= 9`, returns `10`.
 * @param bitmap The bitmap to search.
 * @param index  Lower bound (inclusive) for the search, in [0, 255].
 * @return v One-based position of LSB found (bitIndex + 1), or 0 if none.
 */
function geSetBit(Bitmap bitmap, uint8 index) pure returns (uint256 v) {
    assembly ("memory-safe") {
        let masked := and(bitmap, not(sub(shl(index, 1), 1)))
        v := sub(256, clz(and(masked, sub(0, masked))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Returns the fee to charge based on the amount, which is the fee (a 0.64 number) times the
// amount, rounded up
function computeFee(uint128 amount, uint64 fee) pure returns (uint128 result) {
    assembly ("memory-safe") {
        result := shr(64, add(mul(amount, fee), 0xffffffffffffffff))
    }
}

error AmountBeforeFeeOverflow();

// Returns the amount before the fee is applied, which is the amount minus the fee, rounded up
function amountBeforeFee(uint128 afterFee, uint64 fee) pure returns (uint128 result) {
    assembly ("memory-safe") {
        let v := shl(64, afterFee)
        let d := sub(0x10000000000000000, fee)
        result := add(iszero(iszero(mod(v, d))), div(v, d))
        if shr(128, result) {
            mstore(0, 0x0d88f526)
            revert(0x1c, 0x04)
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FeesPerLiquidity} from "./feesPerLiquidity.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

// Position Type
// Represents a liquidity position in a pool
// Contains the position's liquidity amount and fee tracking information

/// @notice A liquidity position in a pool
/// @dev Tracks both the liquidity amount and the last known fees per liquidity for fee calculation
struct Position {
    /// @notice Extra data that can be set by the owner of a position
    bytes16 extraData;
    /// @notice Amount of liquidity in the position
    uint128 liquidity;
    /// @notice Snapshot of fees per liquidity when the position was last updated
    /// @dev Used to calculate fees owed to the position holder
    FeesPerLiquidity feesPerLiquidityInsideLast;
}

using {fees} for Position global;

/// @notice Calculates the fees owed to a position
/// @dev Returns the fee amounts of token0 and token1 owed to a position based on the given fees per liquidity inside snapshot
///      Note: if the computed fees overflow the uint128 type, it will return only the lower 128 bits. It is assumed that accumulated
///      fees will never exceed type(uint128).max.
/// @param position The position to calculate fees for
/// @param feesPerLiquidityInside Current fees per liquidity inside the position's bounds
/// @return Amount of token0 fees owed
/// @return Amount of token1 fees owed
function fees(Position memory position, FeesPerLiquidity memory feesPerLiquidityInside)
    pure
    returns (uint128, uint128)
{
    uint128 liquidity;
    uint256 difference0;
    uint256 difference1;
    assembly ("memory-safe") {
        liquidity := mload(add(position, 0x20))
        // feesPerLiquidityInsideLast is now at offset 0x40 due to extraData field
        let positionFpl := mload(add(position, 0x40))
        difference0 := sub(mload(feesPerLiquidityInside), mload(positionFpl))
        difference1 := sub(mload(add(feesPerLiquidityInside, 0x20)), mload(add(positionFpl, 0x20)))
    }

    return (
        uint128(FixedPointMathLib.fullMulDivN(difference0, liquidity, 128)),
        uint128(FixedPointMathLib.fullMulDivN(difference1, liquidity, 128))
    );
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type PoolBalanceUpdate is bytes32;

using {delta0, delta1} for PoolBalanceUpdate global;

function delta0(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, shr(128, update))
    }
}

function delta1(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, update)
    }
}

function createPoolBalanceUpdate(int128 _delta0, int128 _delta1) pure returns (PoolBalanceUpdate update) {
    assembly ("memory-safe") {
        // update = (delta0 << 128) | delta1
        update := or(shl(128, _delta0), and(_delta1, 0xffffffffffffffffffffffffffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {Bitmap} from "../types/bitmap.sol";
import {nextValidTime} from "../math/time.sol";
import {StorageSlot} from "../types/storageSlot.sol";

// Returns the index of the word and the index _in_ that word which contains the bit representing whether the time is initialized
// Always rounds the time down
function timeToBitmapWordAndIndex(uint256 time) pure returns (uint256 word, uint256 index) {
    assembly ("memory-safe") {
        word := shr(16, time)
        index := and(shr(8, time), 0xff)
    }
}

// Returns the index of the word and the index _in_ that word which contains the bit representing whether the tick is initialized
/// @dev Assumes word is less than 2**252 and index is less than 2**8
function bitmapWordAndIndexToTime(uint256 word, uint256 index) pure returns (uint256 time) {
    assembly ("memory-safe") {
        time := add(shl(16, word), shl(8, index))
    }
}

// Flips the tick in the bitmap from true to false or vice versa
function flipTime(StorageSlot slot, uint256 time) {
    (uint256 word, uint256 index) = timeToBitmapWordAndIndex(time);
    StorageSlot wordSlot = slot.add(word);
    wordSlot.store(wordSlot.load() ^ bytes32(1 << index));
}

/// @dev Finds the smallest time that is equal to or greater than the given `fromTime`, initialized and stored in the next bitmap
///      If no initialized time is found, returns the greatest time in the bitmap
function findNextInitializedTime(StorageSlot slot, uint256 fromTime)
    view
    returns (uint256 nextTime, bool isInitialized)
{
    unchecked {
        // convert the given time to the bitmap position of the next nearest potential initialized time
        (uint256 word, uint256 index) = timeToBitmapWordAndIndex(fromTime);

        // find the index of the previous tick in that word
        Bitmap bitmap = Bitmap.wrap(uint256(slot.add(word).load()));
        uint256 nextIndex = bitmap.geSetBit(uint8(index));

        isInitialized = nextIndex != 0;

        assembly ("memory-safe") {
            nextIndex := mod(sub(nextIndex, 1), 256)
        }

        nextTime = bitmapWordAndIndexToTime(word, nextIndex);
    }
}

/// @dev Returns the smallest time that is greater than fromTime, less than or equal to untilTime and whether it is initialized
/// @param lastVirtualOrderExecutionTime Used to determine the next possible valid time to search
/// @param fromTime The time after which to start the search
/// @param untilTime The time where to end the search, i.e. this function will return at most the value passed to `untilTime`
function searchForNextInitializedTime(
    StorageSlot slot,
    uint256 lastVirtualOrderExecutionTime,
    uint256 fromTime,
    uint256 untilTime
) view returns (uint256 nextTime, bool isInitialized) {
    unchecked {
        nextTime = fromTime;
        while (!isInitialized && nextTime != untilTime) {
            uint256 nextValid = nextValidTime(lastVirtualOrderExecutionTime, nextTime);
            // if there is no valid time after the given nextTime, just return untilTime
            if (nextValid == 0) {
                nextTime = untilTime;
                isInitialized = false;
                break;
            }
            (nextTime, isInitialized) = findNextInitializedTime(slot, nextValid);
            if (nextTime > untilTime) {
                nextTime = untilTime;
                isInitialized = false;
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Observation is bytes32;

using {secondsPerLiquidityCumulative, tickCumulative} for Observation global;

function secondsPerLiquidityCumulative(Observation observation) pure returns (uint160 s) {
    assembly ("memory-safe") {
        s := shr(96, observation)
    }
}

function tickCumulative(Observation observation) pure returns (int64 t) {
    assembly ("memory-safe") {
        t := signextend(7, observation)
    }
}

function createObservation(uint160 _secondsPerLiquidityCumulative, int64 _tickCumulative) pure returns (Observation o) {
    assembly ("memory-safe") {
        // o = (secondsPerLiquidityCumulative << 96) | tickCumulative
        o := or(shl(96, _secondsPerLiquidityCumulative), and(_tickCumulative, 0xFFFFFFFFFFFFFFFF))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Returns the fee to charge based on the amount, which is the fee (a 0.64 number) times the
// amount, rounded up
function computeFee(uint128 amount, uint64 fee) pure returns (uint128 result) {
    assembly ("memory-safe") {
        result := shr(64, add(mul(amount, fee), 0xffffffffffffffff))
    }
}

error AmountBeforeFeeOverflow();

// Returns the amount before the fee is applied, which is the amount minus the fee, rounded up
function amountBeforeFee(uint128 afterFee, uint64 fee) pure returns (uint128 result) {
    assembly ("memory-safe") {
        let v := shl(64, afterFee)
        let d := sub(0x10000000000000000, fee)
        result := add(iszero(iszero(mod(v, d))), div(v, d))
        if shr(128, result) {
            mstore(0, 0x0d88f526)
            revert(0x1c, 0x04)
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Order configuration packed into a single bytes32
/// @dev Contains fee (8 bytes), isToken1 (1 byte), padding (7 bytes), start time (8 bytes), and end time (8 bytes)
type OrderConfig is bytes32;

using {fee, isToken1, startTime, endTime} for OrderConfig global;

/// @notice Extracts the fee from an order config
/// @param config The order config
/// @return r The fee
function fee(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := shr(192, config)
    }
}

/// @notice Extracts isToken1 from an order config
/// @param config The order config
/// @return r Whether the order is selling token1
function isToken1(OrderConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(iszero(byte(8, config)))
    }
}

/// @notice Extracts the start time from an order config
/// @param config The order config
/// @return r The start time
function startTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(64, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the end time from an order config
/// @param config The order config
/// @return r The end time
function endTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(config, 0xffffffffffffffff)
    }
}

/// @notice Creates an OrderConfig from individual components
/// @param _fee The fee of the TWAMM pool
/// @param _startTime The start time of the order
/// @param _endTime The end time of the order
/// @return c The packed configuration
function createOrderConfig(uint64 _fee, bool _isToken1, uint64 _startTime, uint64 _endTime)
    pure
    returns (OrderConfig c)
{
    assembly ("memory-safe") {
        // Mask inputs to ensure only relevant bits are used
        c := add(
            add(shl(192, _fee), add(shl(184, iszero(iszero(_isToken1))), shl(64, and(_startTime, 0xffffffffffffffff)))),
            and(_endTime, 0xffffffffffffffff)
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type PoolBalanceUpdate is bytes32;

using {delta0, delta1} for PoolBalanceUpdate global;

function delta0(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, shr(128, update))
    }
}

function delta1(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, update)
    }
}

function createPoolBalanceUpdate(int128 _delta0, int128 _delta1) pure returns (PoolBalanceUpdate update) {
    assembly ("memory-safe") {
        // update = (delta0 << 128) | delta1
        update := or(shl(128, _delta0), and(_delta1, 0xffffffffffffffffffffffffffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio} from "./sqrtRatio.sol";

type PoolState is bytes32;

using {sqrtRatio, tick, liquidity, isInitialized, parse} for PoolState global;

function sqrtRatio(PoolState state) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, state)
    }
}

function tick(PoolState state) pure returns (int32 t) {
    assembly ("memory-safe") {
        t := signextend(3, shr(128, state))
    }
}

function liquidity(PoolState state) pure returns (uint128 l) {
    assembly ("memory-safe") {
        l := shr(128, shl(128, state))
    }
}

function isInitialized(PoolState state) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := iszero(iszero(state))
    }
}

function parse(PoolState state) pure returns (SqrtRatio r, int32 t, uint128 l) {
    assembly ("memory-safe") {
        r := shr(160, state)
        t := signextend(3, shr(128, state))
        l := shr(128, shl(128, state))
    }
}

function createPoolState(SqrtRatio _sqrtRatio, int32 _tick, uint128 _liquidity) pure returns (PoolState s) {
    assembly ("memory-safe") {
        // s = (sqrtRatio << 160) | (_tick << 128) | liquidity
        s := or(shl(160, _sqrtRatio), or(shl(128, and(_tick, 0xFFFFFFFF)), shr(128, shl(128, _liquidity))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio} from "./sqrtRatio.sol";

type PoolState is bytes32;

using {sqrtRatio, tick, liquidity, isInitialized, parse} for PoolState global;

function sqrtRatio(PoolState state) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, state)
    }
}

function tick(PoolState state) pure returns (int32 t) {
    assembly ("memory-safe") {
        t := signextend(3, shr(128, state))
    }
}

function liquidity(PoolState state) pure returns (uint128 l) {
    assembly ("memory-safe") {
        l := shr(128, shl(128, state))
    }
}

function isInitialized(PoolState state) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := iszero(iszero(state))
    }
}

function parse(PoolState state) pure returns (SqrtRatio r, int32 t, uint128 l) {
    assembly ("memory-safe") {
        r := shr(160, state)
        t := signextend(3, shr(128, state))
        l := shr(128, shl(128, state))
    }
}

function createPoolState(SqrtRatio _sqrtRatio, int32 _tick, uint128 _liquidity) pure returns (PoolState s) {
    assembly ("memory-safe") {
        // s = (sqrtRatio << 160) | (_tick << 128) | liquidity
        s := or(shl(160, _sqrtRatio), or(shl(128, and(_tick, 0xFFFFFFFF)), shr(128, shl(128, _liquidity))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Order configuration packed into a single bytes32
/// @dev Contains fee (8 bytes), isToken1 (1 byte), padding (7 bytes), start time (8 bytes), and end time (8 bytes)
type OrderConfig is bytes32;

using {fee, isToken1, startTime, endTime} for OrderConfig global;

/// @notice Extracts the fee from an order config
/// @param config The order config
/// @return r The fee
function fee(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := shr(192, config)
    }
}

/// @notice Extracts isToken1 from an order config
/// @param config The order config
/// @return r Whether the order is selling token1
function isToken1(OrderConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(iszero(byte(8, config)))
    }
}

/// @notice Extracts the start time from an order config
/// @param config The order config
/// @return r The start time
function startTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(64, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the end time from an order config
/// @param config The order config
/// @return r The end time
function endTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(config, 0xffffffffffffffff)
    }
}

/// @notice Creates an OrderConfig from individual components
/// @param _fee The fee of the TWAMM pool
/// @param _startTime The start time of the order
/// @param _endTime The end time of the order
/// @return c The packed configuration
function createOrderConfig(uint64 _fee, bool _isToken1, uint64 _startTime, uint64 _endTime)
    pure
    returns (OrderConfig c)
{
    assembly ("memory-safe") {
        // Mask inputs to ensure only relevant bits are used
        c := add(
            add(shl(192, _fee), add(shl(184, iszero(iszero(_isToken1))), shl(64, and(_startTime, 0xffffffffffffffff)))),
            and(_endTime, 0xffffffffffffffff)
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @dev Computes 2^x where x is a 5.64 number and result is a 64.64 number
function exp2(uint256 x) pure returns (uint256 result) {
    unchecked {
        require(x < 0x400000000000000000); // Overflow

        result = 0x80000000000000000000000000000000;

        if (x & 0x8000000000000000 != 0) {
            result = result * 0x16A09E667F3BCC908B2FB1366EA957D3E >> 128;
        }
        if (x & 0x4000000000000000 != 0) {
            result = result * 0x1306FE0A31B7152DE8D5A46305C85EDEC >> 128;
        }
        if (x & 0x2000000000000000 != 0) {
            result = result * 0x1172B83C7D517ADCDF7C8C50EB14A791F >> 128;
        }
        if (x & 0x1000000000000000 != 0) {
            result = result * 0x10B5586CF9890F6298B92B71842A98363 >> 128;
        }
        if (x & 0x800000000000000 != 0) {
            result = result * 0x1059B0D31585743AE7C548EB68CA417FD >> 128;
        }
        if (x & 0x400000000000000 != 0) {
            result = result * 0x102C9A3E778060EE6F7CACA4F7A29BDE8 >> 128;
        }
        if (x & 0x200000000000000 != 0) {
            result = result * 0x10163DA9FB33356D84A66AE336DCDFA3F >> 128;
        }
        if (x & 0x100000000000000 != 0) {
            result = result * 0x100B1AFA5ABCBED6129AB13EC11DC9543 >> 128;
        }
        if (x & 0x80000000000000 != 0) {
            result = result * 0x10058C86DA1C09EA1FF19D294CF2F679B >> 128;
        }
        if (x & 0x40000000000000 != 0) {
            result = result * 0x1002C605E2E8CEC506D21BFC89A23A00F >> 128;
        }
        if (x & 0x20000000000000 != 0) {
            result = result * 0x100162F3904051FA128BCA9C55C31E5DF >> 128;
        }
        if (x & 0x10000000000000 != 0) {
            result = result * 0x1000B175EFFDC76BA38E31671CA939725 >> 128;
        }
        if (x & 0x8000000000000 != 0) {
            result = result * 0x100058BA01FB9F96D6CACD4B180917C3D >> 128;
        }
        if (x & 0x4000000000000 != 0) {
            result = result * 0x10002C5CC37DA9491D0985C348C68E7B3 >> 128;
        }
        if (x & 0x2000000000000 != 0) {
            result = result * 0x1000162E525EE054754457D5995292026 >> 128;
        }
        if (x & 0x1000000000000 != 0) {
            result = result * 0x10000B17255775C040618BF4A4ADE83FC >> 128;
        }
        if (x & 0x800000000000 != 0) {
            result = result * 0x1000058B91B5BC9AE2EED81E9B7D4CFAB >> 128;
        }
        if (x & 0x400000000000 != 0) {
            result = result * 0x100002C5C89D5EC6CA4D7C8ACC017B7C9 >> 128;
        }
        if (x & 0x200000000000 != 0) {
            result = result * 0x10000162E43F4F831060E02D839A9D16D >> 128;
        }
        if (x & 0x100000000000 != 0) {
            result = result * 0x100000B1721BCFC99D9F890EA06911763 >> 128;
        }
        if (x & 0x80000000000 != 0) {
            result = result * 0x10000058B90CF1E6D97F9CA14DBCC1628 >> 128;
        }
        if (x & 0x40000000000 != 0) {
            result = result * 0x1000002C5C863B73F016468F6BAC5CA2B >> 128;
        }
        if (x & 0x20000000000 != 0) {
            result = result * 0x100000162E430E5A18F6119E3C02282A5 >> 128;
        }
        if (x & 0x10000000000 != 0) {
            result = result * 0x1000000B1721835514B86E6D96EFD1BFE >> 128;
        }
        if (x & 0x8000000000 != 0) {
            result = result * 0x100000058B90C0B48C6BE5DF846C5B2EF >> 128;
        }
        if (x & 0x4000000000 != 0) {
            result = result * 0x10000002C5C8601CC6B9E94213C72737A >> 128;
        }
        if (x & 0x2000000000 != 0) {
            result = result * 0x1000000162E42FFF037DF38AA2B219F06 >> 128;
        }
        if (x & 0x1000000000 != 0) {
            result = result * 0x10000000B17217FBA9C739AA5819F44F9 >> 128;
        }
        if (x & 0x800000000 != 0) {
            result = result * 0x1000000058B90BFCDEE5ACD3C1CEDC823 >> 128;
        }
        if (x & 0x400000000 != 0) {
            result = result * 0x100000002C5C85FE31F35A6A30DA1BE50 >> 128;
        }
        if (x & 0x200000000 != 0) {
            result = result * 0x10000000162E42FF0999CE3541B9FFFCF >> 128;
        }
        if (x & 0x100000000 != 0) {
            result = result * 0x100000000B17217F80F4EF5AADDA45554 >> 128;
        }
        if (x & 0x80000000 != 0) {
            result = result * 0x10000000058B90BFBF8479BD5A81B51AD >> 128;
        }
        if (x & 0x40000000 != 0) {
            result = result * 0x1000000002C5C85FDF84BD62AE30A74CC >> 128;
        }
        if (x & 0x20000000 != 0) {
            result = result * 0x100000000162E42FEFB2FED257559BDAA >> 128;
        }
        if (x & 0x10000000 != 0) {
            result = result * 0x1000000000B17217F7D5A7716BBA4A9AE >> 128;
        }
        if (x & 0x8000000 != 0) {
            result = result * 0x100000000058B90BFBE9DDBAC5E109CCE >> 128;
        }
        if (x & 0x4000000 != 0) {
            result = result * 0x10000000002C5C85FDF4B15DE6F17EB0D >> 128;
        }
        if (x & 0x2000000 != 0) {
            result = result * 0x1000000000162E42FEFA494F1478FDE05 >> 128;
        }
        if (x & 0x1000000 != 0) {
            result = result * 0x10000000000B17217F7D20CF927C8E94C >> 128;
        }
        if (x & 0x800000 != 0) {
            result = result * 0x1000000000058B90BFBE8F71CB4E4B33D >> 128;
        }
        if (x & 0x400000 != 0) {
            result = result * 0x100000000002C5C85FDF477B662B26945 >> 128;
        }
        if (x & 0x200000 != 0) {
            result = result * 0x10000000000162E42FEFA3AE53369388C >> 128;
        }
        if (x & 0x100000 != 0) {
            result = result * 0x100000000000B17217F7D1D351A389D40 >> 128;
        }
        if (x & 0x80000 != 0) {
            result = result * 0x10000000000058B90BFBE8E8B2D3D4EDE >> 128;
        }
        if (x & 0x40000 != 0) {
            result = result * 0x1000000000002C5C85FDF4741BEA6E77E >> 128;
        }
        if (x & 0x20000 != 0) {
            result = result * 0x100000000000162E42FEFA39FE95583C2 >> 128;
        }
        if (x & 0x10000 != 0) {
            result = result * 0x1000000000000B17217F7D1CFB72B45E1 >> 128;
        }
        if (x & 0x8000 != 0) {
            result = result * 0x100000000000058B90BFBE8E7CC35C3F0 >> 128;
        }
        if (x & 0x4000 != 0) {
            result = result * 0x10000000000002C5C85FDF473E242EA38 >> 128;
        }
        if (x & 0x2000 != 0) {
            result = result * 0x1000000000000162E42FEFA39F02B772C >> 128;
        }
        if (x & 0x1000 != 0) {
            result = result * 0x10000000000000B17217F7D1CF7D83C1A >> 128;
        }
        if (x & 0x800 != 0) {
            result = result * 0x1000000000000058B90BFBE8E7BDCBE2E >> 128;
        }
        if (x & 0x400 != 0) {
            result = result * 0x100000000000002C5C85FDF473DEA871F >> 128;
        }
        if (x & 0x200 != 0) {
            result = result * 0x10000000000000162E42FEFA39EF44D91 >> 128;
        }
        if (x & 0x100 != 0) {
            result = result * 0x100000000000000B17217F7D1CF79E949 >> 128;
        }
        if (x & 0x80 != 0) {
            result = result * 0x10000000000000058B90BFBE8E7BCE544 >> 128;
        }
        if (x & 0x40 != 0) {
            result = result * 0x1000000000000002C5C85FDF473DE6ECA >> 128;
        }
        if (x & 0x20 != 0) {
            result = result * 0x100000000000000162E42FEFA39EF366F >> 128;
        }
        if (x & 0x10 != 0) {
            result = result * 0x1000000000000000B17217F7D1CF79AFA >> 128;
        }
        if (x & 0x8 != 0) {
            result = result * 0x100000000000000058B90BFBE8E7BCD6D >> 128;
        }
        if (x & 0x4 != 0) {
            result = result * 0x10000000000000002C5C85FDF473DE6B2 >> 128;
        }
        if (x & 0x2 != 0) {
            result = result * 0x1000000000000000162E42FEFA39EF358 >> 128;
        }
        if (x & 0x1 != 0) {
            result = result * 0x10000000000000000B17217F7D1CF79AB >> 128;
        }

        result >>= uint256(63 - (x >> 64));
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FeesPerLiquidity} from "./feesPerLiquidity.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

// Position Type
// Represents a liquidity position in a pool
// Contains the position's liquidity amount and fee tracking information

/// @notice A liquidity position in a pool
/// @dev Tracks both the liquidity amount and the last known fees per liquidity for fee calculation
struct Position {
    /// @notice Extra data that can be set by the owner of a position
    bytes16 extraData;
    /// @notice Amount of liquidity in the position
    uint128 liquidity;
    /// @notice Snapshot of fees per liquidity when the position was last updated
    /// @dev Used to calculate fees owed to the position holder
    FeesPerLiquidity feesPerLiquidityInsideLast;
}

using {fees} for Position global;

/// @notice Calculates the fees owed to a position
/// @dev Returns the fee amounts of token0 and token1 owed to a position based on the given fees per liquidity inside snapshot
///      Note: if the computed fees overflow the uint128 type, it will return only the lower 128 bits. It is assumed that accumulated
///      fees will never exceed type(uint128).max.
/// @param position The position to calculate fees for
/// @param feesPerLiquidityInside Current fees per liquidity inside the position's bounds
/// @return Amount of token0 fees owed
/// @return Amount of token1 fees owed
function fees(Position memory position, FeesPerLiquidity memory feesPerLiquidityInside)
    pure
    returns (uint128, uint128)
{
    uint128 liquidity;
    uint256 difference0;
    uint256 difference1;
    assembly ("memory-safe") {
        liquidity := mload(add(position, 0x20))
        // feesPerLiquidityInsideLast is now at offset 0x40 due to extraData field
        let positionFpl := mload(add(position, 0x40))
        difference0 := sub(mload(feesPerLiquidityInside), mload(positionFpl))
        difference1 := sub(mload(add(feesPerLiquidityInside, 0x20)), mload(add(positionFpl, 0x20)))
    }

    return (
        uint128(FixedPointMathLib.fullMulDivN(difference0, liquidity, 128)),
        uint128(FixedPointMathLib.fullMulDivN(difference1, liquidity, 128))
    );
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK, MAX_TICK_SPACING} from "../math/constants.sol";

/// @notice Pool configuration packed into a single bytes32
/// @dev Contains extension address (20 bytes), fee (8 bytes), and pool type config (4 bytes)
/// Pool type config (32 bits):
///   - Bit 31: discriminator (1 = concentrated, 0 = stableswap)
///   - For concentrated (bit 31 = 1): bits 30-0 are tick spacing
///   - For stableswap (bit 31 = 0): bits 30-24 are amplification factor, bits 23-0 are center tick
type PoolConfig is bytes32;

using {
    fee,
    extension,
    isConcentrated,
    isStableswap,
    isFullRange,
    concentratedTickSpacing,
    stableswapAmplification,
    stableswapCenterTick,
    stableswapActiveLiquidityTickRange,
    stableswapLiquidityWidth,
    validate,
    concentratedMaxLiquidityPerTick
} for PoolConfig global;

/// @notice Extracts the fee from a pool config
/// @param config The pool config
/// @return r The fee
function fee(PoolConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(32, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the extension address from a pool config
/// @param config The pool config
/// @return r The extension address
function extension(PoolConfig config) pure returns (address r) {
    assembly ("memory-safe") {
        r := shr(96, config)
    }
}

/// @notice Checks if this is a concentrated liquidity pool
/// @param config The pool config
/// @return r True if the pool is concentrated liquidity
function isConcentrated(PoolConfig config) pure returns (bool r) {
    r = !config.isStableswap();
}

/// @notice Extracts the tick spacing from a concentrated liquidity pool config
/// @dev Only valid for concentrated liquidity pools (isConcentrated() == true)
/// @param config The pool config
/// @return r The tick spacing
function concentratedTickSpacing(PoolConfig config) pure returns (uint32 r) {
    assembly ("memory-safe") {
        // Extract lower 31 bits (bits 30-0)
        r := and(config, 0x7fffffff)
    }
}

/// @notice Checks if this is a stableswap pool
/// @param config The pool config
/// @return r True if the pool is stableswap
function isStableswap(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // = iff bit 31 is not set
        r := iszero(and(0x80000000, config))
    }
}

/// @notice Determines if this pool is full range (special case of stableswap with amplification=0 and center=0)
/// @dev Full range can be slightly optimized in that we don't need to compute the sqrt ratio at the tick boundaries
/// @param config The pool config
/// @return r True if the pool is full range
function isFullRange(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // Full range when all 32 bits are 0 (discriminator=0, amplification=0, center=0)
        r := iszero(and(config, 0xffffffff))
    }
}

/// @notice Extracts the amplification factor from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @param config The pool config
/// @return r The amplification factor (0-127)
function stableswapAmplification(PoolConfig config) pure returns (uint8 r) {
    assembly ("memory-safe") {
        // Extract bits 30-24
        r := and(shr(24, config), 0x7f)
    }
}

/// @notice Extracts the center tick from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @dev The 24-bit center tick is scaled by 16 to get the actual tick
/// @param config The pool config
/// @return r The center tick
function stableswapCenterTick(PoolConfig config) pure returns (int32 r) {
    assembly ("memory-safe") {
        // Extract bits 23-0 and sign extend to 32 bits (24-bit signed integer)
        // Then multiply by 16, since the value does not have full precision
        r := mul(signextend(2, and(config, 0xffffff)), 16)
    }
}

/// @notice Returns the width of the liquidity range
function stableswapLiquidityWidth(PoolConfig config) pure returns (uint256 width) {
    uint8 amp = config.stableswapAmplification();

    assembly ("memory-safe") {
        width := shr(amp, MAX_TICK)
    }
}

/// @notice Computes the tick range where liquidity is active for stableswap pools
/// @param config The pool config
/// @return lower The lower tick of the bounds
/// @return upper The upper tick of the bounds
function stableswapActiveLiquidityTickRange(PoolConfig config) pure returns (int32 lower, int32 upper) {
    int32 center = config.stableswapCenterTick();
    uint256 width = config.stableswapLiquidityWidth();

    assembly ("memory-safe") {
        lower := sub(center, width)
        lower := add(lower, mul(sgt(MIN_TICK, lower), sub(MIN_TICK, lower)))

        upper := add(center, width)
        upper := sub(upper, mul(sgt(upper, MAX_TICK), sub(upper, MAX_TICK)))
    }
}

/// @notice Creates a PoolConfig for a concentrated liquidity pool
/// @param _fee The fee for the pool
/// @param _tickSpacing The tick spacing for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createConcentratedPoolConfig(uint64 _fee, uint32 _tickSpacing, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Set bit 31 to 1 for concentrated liquidity, then OR with tick spacing (bits 30-0)
        let typeConfig := or(0x80000000, and(_tickSpacing, 0x7fffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a stableswap pool
/// @param _fee The fee for the pool
/// @param _amplification The amplification factor (0-127)
/// @param _centerTick The center tick (will be divided by 16 and stored as 24-bit value)
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createStableswapPoolConfig(uint64 _fee, uint8 _amplification, int32 _centerTick, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Divide center tick by 16 to get 24-bit representation
        let stableswapCenterTick24 := sdiv(_centerTick, 16)
        // Pack: bit 31 = 0 (stableswap), bits 30-24 = amplification, bits 23-0 = center tick
        let typeConfig := or(shl(24, and(_amplification, 0x7f)), and(stableswapCenterTick24, 0xffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a full range pool (stableswap with amplification=0, center=0)
/// @param _fee The fee for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createFullRangePoolConfig(uint64 _fee, address _extension) pure returns (PoolConfig c) {
    assembly ("memory-safe") {
        // All 32 bits of type config are 0 (discriminator=0, amplification=0, center=0)
        c := or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff)))
    }
}

/// @notice Computes the maximum liquidity per tick for a given concentrated liquidity pool configuration.
/// @dev Only valid for concentrated liquidity pools. Stableswap pools don't use ticks.
/// @dev Calculated as type(uint128).max / (1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2)
/// @param config The concentrated liquidity pool configuration
/// @return maxLiquidity The maximum liquidity allowed to reference each tick
function concentratedMaxLiquidityPerTick(PoolConfig config) pure returns (uint128 maxLiquidity) {
    uint32 _tickSpacing = config.concentratedTickSpacing();

    assembly ("memory-safe") {
        // Calculate total number of usable ticks: 1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2
        // This represents all ticks from -MAX_TICK_MAGNITUDE to +MAX_TICK_MAGNITUDE, and tick 0
        let numTicks := add(1, mul(div(MAX_TICK, _tickSpacing), 2))

        maxLiquidity := div(sub(shl(128, 1), 1), numTicks)
    }
}

/// @notice Thrown when tick spacing exceeds the maximum allowed value
error InvalidTickSpacing();

/// @notice Thrown when amplification factor exceeds the maximum allowed value
error InvalidStableswapAmplification();

/// @notice Thrown when center tick is not between min and max tick
error InvalidCenterTick();

/// @notice Validates that a pool config is properly formatted
/// @param config The config to validate
function validate(PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (config.concentratedTickSpacing() > MAX_TICK_SPACING || config.concentratedTickSpacing() == 0) {
            revert InvalidTickSpacing();
        }
    } else {
        // Stableswap pool: validate amplification factor <= 26
        if (config.stableswapAmplification() > 26) {
            revert InvalidStableswapAmplification();
        }
        int32 centerTick = config.stableswapCenterTick();
        if (centerTick < MIN_TICK || centerTick > MAX_TICK) {
            revert InvalidCenterTick();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type TwammPoolState is bytes32;

using {
    lastVirtualOrderExecutionTime,
    realLastVirtualOrderExecutionTime,
    saleRateToken0,
    saleRateToken1,
    parse
} for TwammPoolState global;

function lastVirtualOrderExecutionTime(TwammPoolState state) pure returns (uint32 time) {
    assembly ("memory-safe") {
        time := and(state, 0xffffffff)
    }
}

function realLastVirtualOrderExecutionTime(TwammPoolState state) view returns (uint256 time) {
    assembly ("memory-safe") {
        time := sub(timestamp(), and(sub(and(timestamp(), 0xffffffff), and(state, 0xffffffff)), 0xffffffff))
    }
}

function saleRateToken0(TwammPoolState state) pure returns (uint112 rate) {
    assembly ("memory-safe") {
        rate := shr(144, shl(112, state))
    }
}

function saleRateToken1(TwammPoolState state) pure returns (uint112 rate) {
    assembly ("memory-safe") {
        rate := shr(144, state)
    }
}

function parse(TwammPoolState state) pure returns (uint32 time, uint112 rate0, uint112 rate1) {
    assembly ("memory-safe") {
        time := and(state, 0xffffffff)
        rate0 := shr(144, shl(112, state))
        rate1 := shr(144, state)
    }
}

function createTwammPoolState(uint32 _lastVirtualOrderExecutionTime, uint112 _saleRateToken0, uint112 _saleRateToken1)
    pure
    returns (TwammPoolState s)
{
    assembly ("memory-safe") {
        // s = (lastVirtualOrderExecutionTime) | (saleRateToken0 << 32) | (saleRateToken1 << 144)
        s := or(
            or(and(_lastVirtualOrderExecutionTime, 0xffffffff), shr(112, shl(144, _saleRateToken0))),
            shl(144, _saleRateToken1)
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MAX_TICK_MAGNITUDE} from "./constants.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {LibBit} from "solady/utils/LibBit.sol";
import {SqrtRatio, toSqrtRatio} from "../types/sqrtRatio.sol";

// Tick Math Library
// Contains functions for converting between ticks and sqrt price ratios
// Ticks represent discrete price points, while sqrt ratios represent the actual prices
// The relationship is: sqrtRatio = sqrt(1.000001^tick)

/// @notice Thrown when a tick value is outside the valid range
/// @param tick The invalid tick value
error InvalidTick(int32 tick);

/// @notice Converts a tick to its corresponding sqrt price ratio
/// @dev Uses bit manipulation and precomputed constants for gas efficiency
/// @param tick The tick to convert (must be within MIN_TICK and MAX_TICK)
/// @return r The sqrt price ratio corresponding to the tick
function tickToSqrtRatio(int32 tick) pure returns (SqrtRatio r) {
    unchecked {
        uint256 t = FixedPointMathLib.abs(tick);
        if (t > MAX_TICK_MAGNITUDE) revert InvalidTick(tick);

        uint256 ratio;
        assembly ("memory-safe") {
            // bit 0 is handled with a single conditional subtract from 2^128
            ratio := sub(0x100000000000000000000000000000000, mul(and(t, 0x1), 0x8637b66cd638344daef276cd7c5))

            // -------- Gate 1: bits 1..7 (mask 0xFE) --------
            if and(t, 0xFE) {
                if and(t, 0x2) { ratio := shr(128, mul(ratio, 0xffffef390978c398134b4ff3764fe410)) }
                if and(t, 0x4) { ratio := shr(128, mul(ratio, 0xffffde72140b00a354bd3dc828e976c9)) }
                if and(t, 0x8) { ratio := shr(128, mul(ratio, 0xffffbce42c7be6c998ad6318193c0b18)) }
                if and(t, 0x10) { ratio := shr(128, mul(ratio, 0xffff79c86a8f6150a32d9778eceef97c)) }
                if and(t, 0x20) { ratio := shr(128, mul(ratio, 0xfffef3911b7cff24ba1b3dbb5f8f5974)) }
                if and(t, 0x40) { ratio := shr(128, mul(ratio, 0xfffde72350725cc4ea8feece3b5f13c8)) }
                if and(t, 0x80) { ratio := shr(128, mul(ratio, 0xfffbce4b06c196e9247ac87695d53c60)) }
            }

            // -------- Gate 2: bits 8..14 (mask 0x7F00) --------
            if and(t, 0x7F00) {
                if and(t, 0x100) { ratio := shr(128, mul(ratio, 0xfff79ca7a4d1bf1ee8556cea23cdbaa5)) }
                if and(t, 0x200) { ratio := shr(128, mul(ratio, 0xffef3995a5b6a6267530f207142a5764)) }
                if and(t, 0x400) { ratio := shr(128, mul(ratio, 0xffde7444b28145508125d10077ba83b8)) }
                if and(t, 0x800) { ratio := shr(128, mul(ratio, 0xffbceceeb791747f10df216f2e53ec57)) }
                if and(t, 0x1000) { ratio := shr(128, mul(ratio, 0xff79eb706b9a64c6431d76e63531e929)) }
                if and(t, 0x2000) { ratio := shr(128, mul(ratio, 0xfef41d1a5f2ae3a20676bec6f7f9459a)) }
                if and(t, 0x4000) { ratio := shr(128, mul(ratio, 0xfde95287d26d81bea159c37073122c73)) }
            }

            // -------- Gate 3: bits 15..20 (mask 0x1F8000) --------
            if and(t, 0x1F8000) {
                if and(t, 0x8000) { ratio := shr(128, mul(ratio, 0xfbd701c7cbc4c8a6bb81efd232d1e4e7)) }
                if and(t, 0x10000) { ratio := shr(128, mul(ratio, 0xf7bf5211c72f5185f372aeb1d48f937e)) }
                if and(t, 0x20000) { ratio := shr(128, mul(ratio, 0xefc2bf59df33ecc28125cf78ec4f167f)) }
                if and(t, 0x40000) { ratio := shr(128, mul(ratio, 0xe08d35706200796273f0b3a981d90cfd)) }
                if and(t, 0x80000) { ratio := shr(128, mul(ratio, 0xc4f76b68947482dc198a48a54348c4ed)) }
                if and(t, 0x100000) { ratio := shr(128, mul(ratio, 0x978bcb9894317807e5fa4498eee7c0fa)) }
            }

            // -------- Gate 4: bits 21..26 (mask 0x7E00000) --------
            if and(t, 0x7E00000) {
                if and(t, 0x200000) { ratio := shr(128, mul(ratio, 0x59b63684b86e9f486ec54727371ba6ca)) }
                if and(t, 0x400000) { ratio := shr(128, mul(ratio, 0x1f703399d88f6aa83a28b22d4a1f56e3)) }
                if and(t, 0x800000) { ratio := shr(128, mul(ratio, 0x3dc5dac7376e20fc8679758d1bcdcfc)) }
                if and(t, 0x1000000) { ratio := shr(128, mul(ratio, 0xee7e32d61fdb0a5e622b820f681d0)) }
                if and(t, 0x2000000) { ratio := shr(128, mul(ratio, 0xde2ee4bc381afa7089aa84bb66)) }
                if and(t, 0x4000000) { ratio := shr(128, mul(ratio, 0xc0d55d4d7152c25fb139)) }
            }

            // If original tick > 0, invert: ratio = maxUint / ratio
            if sgt(tick, 0) { ratio := div(not(0), ratio) }
        }

        r = toSqrtRatio(ratio, false);
    }
}

uint256 constant ONE_Q127 = 1 << 127;

// Convert ln(m) series to log2(m):  log2(m) = (2 / ln 2) * s.
// Precompute K = round((2 / ln 2) * 2^64) as a uint (Q64 scalar).
// K = 53226052391377289966  (≈ 0x2e2a8eca5705fc2ee)
uint256 constant K_2_OVER_LN2_X64 = 53226052391377289966;

// 2^64 / log2(sqrt(1.000001)) for converting from log base 2 in X64 to log base tick
int256 constant INV_LB_X64 = 25572630076711825471857579;

// Error bounds of the tick computation based on the number of iterations ~= +-0.002 ticks
int256 constant ERROR_BOUNDS_X128 = int256((uint256(1) << 128) / 485);

/// @notice Converts a sqrt price ratio to its corresponding tick
/// @dev Computes log2 via one normalization + atanh series (no per-bit squaring loop)
/// @param sqrtRatio The valid sqrt price ratio to convert
/// @return tick The tick corresponding to the sqrt ratio
function sqrtRatioToTick(SqrtRatio sqrtRatio) pure returns (int32 tick) {
    unchecked {
        uint256 sqrtRatioFixed = sqrtRatio.toFixed();

        // Normalize sign via reciprocal if < 1. Keep this branch-free.
        bool negative;
        uint256 x;
        uint256 hi;
        assembly ("memory-safe") {
            negative := iszero(shr(128, sqrtRatioFixed))
            // x = negative ? (type(uint256).max / R) : R
            x := add(div(sub(0, negative), sqrtRatioFixed), mul(iszero(negative), sqrtRatioFixed))
            // We know (x >> 128) != 0 because we reciprocated sqrtRatioFixed
            hi := shr(128, x)
        }

        // Integer part of log2 via CLZ: floor(log2(hi)) = 255 - clz(hi)
        uint256 msbHigh;
        assembly ("memory-safe") {
            msbHigh := sub(255, clz(hi))
        }

        // Reduce once so X ∈ [2^127, 2^128)  (Q1.127 mantissa)
        x = x >> (msbHigh + 1);

        // Fractional log2 using atanh on y = (m-1)/(m+1), m = X/2^127 ∈ [1,2)
        uint256 a = x - ONE_Q127; // (m - 1) * 2^127
        uint256 b = x + ONE_Q127; // (m + 1) * 2^127
        uint256 yQ = FixedPointMathLib.rawDiv(a << 127, b); // y in Q1.127

        // Build odd powers via y^2 ladder
        uint256 y2 = (yQ * yQ) >> 127; // y^2
        uint256 y3 = (yQ * y2) >> 127; // y^3
        uint256 y5 = (y3 * y2) >> 127; // y^5
        uint256 y7 = (y5 * y2) >> 127; // y^7
        uint256 y9 = (y7 * y2) >> 127; // y^9
        uint256 y11 = (y9 * y2) >> 127; // y^11
        uint256 y13 = (y11 * y2) >> 127; // y^13
        uint256 y15 = (y13 * y2) >> 127; // y^15

        // s = y + y^3/3 + y^5/5 + ... + y^15/15  (Q1.127)
        uint256 s = yQ + (y3 / 3) + (y5 / 5) + (y7 / 7) + (y9 / 9) + (y11 / 11) + (y13 / 13) + (y15 / 15);

        // fracX64 = ((2/ln2) * s) in Q64.64  =>  (s * K) >> 127
        uint256 fracX64 = (s * K_2_OVER_LN2_X64) >> 127;

        // Unsigned log2 in Q64.64
        uint256 log2Unsigned = (msbHigh << 64) + fracX64;

        // Map log2 to tick-space X128
        int256 base = negative ? -int256(log2Unsigned) : int256(log2Unsigned);

        int256 logBaseTickSizeX128 = base * INV_LB_X64;

        // Add error bounds to the computed logarithm
        int32 tickLow = int32((logBaseTickSizeX128 - ERROR_BOUNDS_X128) >> 128);
        tick = int32((logBaseTickSizeX128 + ERROR_BOUNDS_X128) >> 128);

        if (tick != tickLow) {
            // tickHigh overshoots
            if (tickToSqrtRatio(tick) > sqrtRatio) {
                tick = tickLow;
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Observation is bytes32;

using {secondsPerLiquidityCumulative, tickCumulative} for Observation global;

function secondsPerLiquidityCumulative(Observation observation) pure returns (uint160 s) {
    assembly ("memory-safe") {
        s := shr(96, observation)
    }
}

function tickCumulative(Observation observation) pure returns (int64 t) {
    assembly ("memory-safe") {
        t := signextend(7, observation)
    }
}

function createObservation(uint160 _secondsPerLiquidityCumulative, int64 _tickCumulative) pure returns (Observation o) {
    assembly ("memory-safe") {
        // o = (secondsPerLiquidityCumulative << 96) | tickCumulative
        o := or(shl(96, _secondsPerLiquidityCumulative), and(_tickCumulative, 0xFFFFFFFFFFFFFFFF))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// The total fees per liquidity for each token.
// Since these are always read together we put them in a struct, even though they cannot be packed.
struct FeesPerLiquidity {
    uint256 value0;
    uint256 value1;
}

using {sub} for FeesPerLiquidity global;

function sub(FeesPerLiquidity memory a, FeesPerLiquidity memory b) pure returns (FeesPerLiquidity memory result) {
    assembly ("memory-safe") {
        mstore(result, sub(mload(a), mload(b)))
        mstore(add(result, 32), sub(mload(add(a, 32)), mload(add(b, 32))))
    }
}

function feesPerLiquidityFromAmounts(uint128 amount0, uint128 amount1, uint128 liquidity)
    pure
    returns (FeesPerLiquidity memory result)
{
    assembly ("memory-safe") {
        mstore(result, div(shl(128, amount0), liquidity))
        mstore(add(result, 32), div(shl(128, amount1), liquidity))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio} from "./sqrtRatio.sol";

type PoolState is bytes32;

using {sqrtRatio, tick, liquidity, isInitialized, parse} for PoolState global;

function sqrtRatio(PoolState state) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, state)
    }
}

function tick(PoolState state) pure returns (int32 t) {
    assembly ("memory-safe") {
        t := signextend(3, shr(128, state))
    }
}

function liquidity(PoolState state) pure returns (uint128 l) {
    assembly ("memory-safe") {
        l := shr(128, shl(128, state))
    }
}

function isInitialized(PoolState state) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := iszero(iszero(state))
    }
}

function parse(PoolState state) pure returns (SqrtRatio r, int32 t, uint128 l) {
    assembly ("memory-safe") {
        r := shr(160, state)
        t := signextend(3, shr(128, state))
        l := shr(128, shl(128, state))
    }
}

function createPoolState(SqrtRatio _sqrtRatio, int32 _tick, uint128 _liquidity) pure returns (PoolState s) {
    assembly ("memory-safe") {
        // s = (sqrtRatio << 160) | (_tick << 128) | liquidity
        s := or(shl(160, _sqrtRatio), or(shl(128, and(_tick, 0xFFFFFFFF)), shr(128, shl(128, _liquidity))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type MEVCapturePoolState is bytes32;

using {lastUpdateTime, tickLast} for MEVCapturePoolState global;

function lastUpdateTime(MEVCapturePoolState state) pure returns (uint32 v) {
    assembly ("memory-safe") {
        v := shr(224, state)
    }
}

function tickLast(MEVCapturePoolState state) pure returns (int32 v) {
    assembly ("memory-safe") {
        v := signextend(3, state)
    }
}

function createMEVCapturePoolState(uint32 _lastUpdateTime, int32 _tickLast) pure returns (MEVCapturePoolState s) {
    assembly ("memory-safe") {
        // s = (lastUpdateTime << 224) | tickLast
        s := or(shl(224, _lastUpdateTime), and(_tickLast, 0xffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "./poolKey.sol";
import {OrderConfig} from "./orderConfig.sol";
import {OrderId} from "./orderId.sol";

using {toOrderId, toPoolKey, buyToken, sellToken} for OrderKey global;

/// @notice Order key structure identifying a TWAMM order
/// @dev Contains all parameters needed to uniquely identify an order
struct OrderKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing fee, isToken1, start, and end time
    OrderConfig config;
}

/// @notice Extracts the buy token from an order key
/// @param ok The order key
/// @return r The buy token
function buyToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken0 = !ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken0, 0x20)))
    }
}

/// @notice Extracts the sell token from an order key
/// @param ok The order key
/// @return r The sell token
function sellToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken1 = ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken1, 0x20)))
    }
}

/// @notice Computes the order ID from an order key
/// @param orderKey The order key
/// @return id The computed order ID
function toOrderId(OrderKey memory orderKey) pure returns (OrderId id) {
    assembly ("memory-safe") {
        id := keccak256(orderKey, 96)
    }
}

/// @notice Converts an OrderKey to its corresponding PoolKey
/// @dev Determines the correct token ordering and constructs the pool key with TWAMM as extension
/// @param orderKey The order key containing sell/buy tokens and fee
/// @param twamm The TWAMM contract address to use as the extension
/// @return poolKey The corresponding pool key for the order
function toPoolKey(OrderKey memory orderKey, address twamm) pure returns (PoolKey memory poolKey) {
    uint64 _fee = orderKey.config.fee();
    assembly ("memory-safe") {
        mcopy(poolKey, orderKey, 64)
        mstore(add(poolKey, 64), add(shl(96, twamm), shl(32, _fee)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

struct CallPoints {
    bool beforeInitializePool;
    bool afterInitializePool;
    bool beforeSwap;
    bool afterSwap;
    bool beforeUpdatePosition;
    bool afterUpdatePosition;
    bool beforeCollectFees;
    bool afterCollectFees;
}

using {eq, isValid, toUint8} for CallPoints global;

function eq(CallPoints memory a, CallPoints memory b) pure returns (bool) {
    return (a.beforeInitializePool == b.beforeInitializePool && a.afterInitializePool == b.afterInitializePool
            && a.beforeSwap == b.beforeSwap && a.afterSwap == b.afterSwap
            && a.beforeUpdatePosition == b.beforeUpdatePosition && a.afterUpdatePosition == b.afterUpdatePosition
            && a.beforeCollectFees == b.beforeCollectFees && a.afterCollectFees == b.afterCollectFees);
}

function isValid(CallPoints memory a) pure returns (bool) {
    return (a.beforeInitializePool || a.afterInitializePool || a.beforeSwap || a.afterSwap || a.beforeUpdatePosition
            || a.afterUpdatePosition || a.beforeCollectFees || a.afterCollectFees);
}

function toUint8(CallPoints memory callPoints) pure returns (uint8 b) {
    assembly ("memory-safe") {
        b := add(
            add(
                add(
                    add(
                        add(
                            add(
                                add(mload(callPoints), mul(128, mload(add(callPoints, 32)))),
                                mul(64, mload(add(callPoints, 64)))
                            ),
                            mul(32, mload(add(callPoints, 96)))
                        ),
                        mul(16, mload(add(callPoints, 128)))
                    ),
                    mul(8, mload(add(callPoints, 160)))
                ),
                mul(4, mload(add(callPoints, 192)))
            ),
            mul(2, mload(add(callPoints, 224)))
        )
    }
}

function addressToCallPoints(address a) pure returns (CallPoints memory result) {
    result = byteToCallPoints(uint8(uint160(a) >> 152));
}

function byteToCallPoints(uint8 b) pure returns (CallPoints memory result) {
    // note the order of bytes does not match the struct order of elements because we are matching the cairo implementation
    // which for legacy reasons has the fields in this order
    result = CallPoints({
        beforeInitializePool: (b & 1) != 0,
        afterInitializePool: (b & 128) != 0,
        beforeSwap: (b & 64) != 0,
        afterSwap: (b & 32) != 0,
        beforeUpdatePosition: (b & 16) != 0,
        afterUpdatePosition: (b & 8) != 0,
        beforeCollectFees: (b & 4) != 0,
        afterCollectFees: (b & 2) != 0
    });
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

struct CallPoints {
    bool beforeInitializePool;
    bool afterInitializePool;
    bool beforeSwap;
    bool afterSwap;
    bool beforeUpdatePosition;
    bool afterUpdatePosition;
    bool beforeCollectFees;
    bool afterCollectFees;
}

using {eq, isValid, toUint8} for CallPoints global;

function eq(CallPoints memory a, CallPoints memory b) pure returns (bool) {
    return (a.beforeInitializePool == b.beforeInitializePool && a.afterInitializePool == b.afterInitializePool
            && a.beforeSwap == b.beforeSwap && a.afterSwap == b.afterSwap
            && a.beforeUpdatePosition == b.beforeUpdatePosition && a.afterUpdatePosition == b.afterUpdatePosition
            && a.beforeCollectFees == b.beforeCollectFees && a.afterCollectFees == b.afterCollectFees);
}

function isValid(CallPoints memory a) pure returns (bool) {
    return (a.beforeInitializePool || a.afterInitializePool || a.beforeSwap || a.afterSwap || a.beforeUpdatePosition
            || a.afterUpdatePosition || a.beforeCollectFees || a.afterCollectFees);
}

function toUint8(CallPoints memory callPoints) pure returns (uint8 b) {
    assembly ("memory-safe") {
        b := add(
            add(
                add(
                    add(
                        add(
                            add(
                                add(mload(callPoints), mul(128, mload(add(callPoints, 32)))),
                                mul(64, mload(add(callPoints, 64)))
                            ),
                            mul(32, mload(add(callPoints, 96)))
                        ),
                        mul(16, mload(add(callPoints, 128)))
                    ),
                    mul(8, mload(add(callPoints, 160)))
                ),
                mul(4, mload(add(callPoints, 192)))
            ),
            mul(2, mload(add(callPoints, 224)))
        )
    }
}

function addressToCallPoints(address a) pure returns (CallPoints memory result) {
    result = byteToCallPoints(uint8(uint160(a) >> 152));
}

function byteToCallPoints(uint8 b) pure returns (CallPoints memory result) {
    // note the order of bytes does not match the struct order of elements because we are matching the cairo implementation
    // which for legacy reasons has the fields in this order
    result = CallPoints({
        beforeInitializePool: (b & 1) != 0,
        afterInitializePool: (b & 128) != 0,
        beforeSwap: (b & 64) != 0,
        afterSwap: (b & 32) != 0,
        beforeUpdatePosition: (b & 16) != 0,
        afterUpdatePosition: (b & 8) != 0,
        beforeCollectFees: (b & 4) != 0,
        afterCollectFees: (b & 2) != 0
    });
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/**
 * @title Bitmap (256-bit)
 * @notice Lightweight helpers for treating a `uint256` as a 256-bit bitmap.
 * @dev
 * - Bit indices are in the range [0, 255].
 * - All operations are O(1) and implemented with memory-safe assembly.
 * - For search helpers `leSetBit` and `geSetBit`, the return value is
 *   one-based: it returns `index + 1` of the matching set bit, or `0` if none.
 *   This convention avoids the need for sentinels outside the 0..255 range.
 */
type Bitmap is uint256;

using {toggle, isSet, leSetBit, geSetBit} for Bitmap global;

/**
 * @notice Toggle (flip) the bit at `index` in `bitmap`.
 * @param bitmap The current bitmap value.
 * @param index  Bit position to toggle, in [0, 255].
 * @return result A new bitmap with the bit at `index` flipped.
 */
function toggle(Bitmap bitmap, uint8 index) pure returns (Bitmap result) {
    assembly ("memory-safe") {
        result := xor(bitmap, shl(index, 1))
    }
}

/**
 * @notice Check whether the bit at `index` is set.
 * @param bitmap The bitmap to read.
 * @param index  Bit position to test, in [0, 255].
 * @return yes True if the bit is 1, false otherwise.
 */
function isSet(Bitmap bitmap, uint8 index) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(index, bitmap), 1)
    }
}

/**
 * @notice Find the most significant set bit at or below `index`.
 * @dev Returns one-based position: `pos = bitIndex + 1`, or `0` if none.
 *      Example: if bit 9 is set and `index >= 9`, returns `10`.
 *      For `index == 255`, the mask spans all bits (wraparound yields `2^256-1`).
 * @param bitmap The bitmap to search.
 * @param index  Upper bound (inclusive) for the search, in [0, 255].
 * @return v One-based position of MSB found (bitIndex + 1), or 0 if none.
 */
function leSetBit(Bitmap bitmap, uint8 index) pure returns (uint256 v) {
    unchecked {
        assembly ("memory-safe") {
            let masked := and(bitmap, sub(shl(add(index, 1), 1), 1))
            v := sub(256, clz(masked))
        }
    }
}

/**
 * @notice Find the least significant set bit at or above `index`.
 * @dev Returns one-based position: `pos = bitIndex + 1`, or `0` if none.
 *      Example: if bit 9 is set and `index <= 9`, returns `10`.
 * @param bitmap The bitmap to search.
 * @param index  Lower bound (inclusive) for the search, in [0, 255].
 * @return v One-based position of LSB found (bitIndex + 1), or 0 if none.
 */
function geSetBit(Bitmap bitmap, uint8 index) pure returns (uint256 v) {
    assembly ("memory-safe") {
        let masked := and(bitmap, not(sub(shl(index, 1), 1)))
        v := sub(256, clz(and(masked, sub(0, masked))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/**
 * @title Bitmap (256-bit)
 * @notice Lightweight helpers for treating a `uint256` as a 256-bit bitmap.
 * @dev
 * - Bit indices are in the range [0, 255].
 * - All operations are O(1) and implemented with memory-safe assembly.
 * - For search helpers `leSetBit` and `geSetBit`, the return value is
 *   one-based: it returns `index + 1` of the matching set bit, or `0` if none.
 *   This convention avoids the need for sentinels outside the 0..255 range.
 */
type Bitmap is uint256;

using {toggle, isSet, leSetBit, geSetBit} for Bitmap global;

/**
 * @notice Toggle (flip) the bit at `index` in `bitmap`.
 * @param bitmap The current bitmap value.
 * @param index  Bit position to toggle, in [0, 255].
 * @return result A new bitmap with the bit at `index` flipped.
 */
function toggle(Bitmap bitmap, uint8 index) pure returns (Bitmap result) {
    assembly ("memory-safe") {
        result := xor(bitmap, shl(index, 1))
    }
}

/**
 * @notice Check whether the bit at `index` is set.
 * @param bitmap The bitmap to read.
 * @param index  Bit position to test, in [0, 255].
 * @return yes True if the bit is 1, false otherwise.
 */
function isSet(Bitmap bitmap, uint8 index) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(index, bitmap), 1)
    }
}

/**
 * @notice Find the most significant set bit at or below `index`.
 * @dev Returns one-based position: `pos = bitIndex + 1`, or `0` if none.
 *      Example: if bit 9 is set and `index >= 9`, returns `10`.
 *      For `index == 255`, the mask spans all bits (wraparound yields `2^256-1`).
 * @param bitmap The bitmap to search.
 * @param index  Upper bound (inclusive) for the search, in [0, 255].
 * @return v One-based position of MSB found (bitIndex + 1), or 0 if none.
 */
function leSetBit(Bitmap bitmap, uint8 index) pure returns (uint256 v) {
    unchecked {
        assembly ("memory-safe") {
            let masked := and(bitmap, sub(shl(add(index, 1), 1), 1))
            v := sub(256, clz(masked))
        }
    }
}

/**
 * @notice Find the least significant set bit at or above `index`.
 * @dev Returns one-based position: `pos = bitIndex + 1`, or `0` if none.
 *      Example: if bit 9 is set and `index <= 9`, returns `10`.
 * @param bitmap The bitmap to search.
 * @param index  Lower bound (inclusive) for the search, in [0, 255].
 * @return v One-based position of LSB found (bitIndex + 1), or 0 if none.
 */
function geSetBit(Bitmap bitmap, uint8 index) pure returns (uint256 v) {
    assembly ("memory-safe") {
        let masked := and(bitmap, not(sub(shl(index, 1), 1)))
        v := sub(256, clz(and(masked, sub(0, masked))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// The total fees per liquidity for each token.
// Since these are always read together we put them in a struct, even though they cannot be packed.
struct FeesPerLiquidity {
    uint256 value0;
    uint256 value1;
}

using {sub} for FeesPerLiquidity global;

function sub(FeesPerLiquidity memory a, FeesPerLiquidity memory b) pure returns (FeesPerLiquidity memory result) {
    assembly ("memory-safe") {
        mstore(result, sub(mload(a), mload(b)))
        mstore(add(result, 32), sub(mload(add(a, 32)), mload(add(b, 32))))
    }
}

function feesPerLiquidityFromAmounts(uint128 amount0, uint128 amount1, uint128 liquidity)
    pure
    returns (FeesPerLiquidity memory result)
{
    assembly ("memory-safe") {
        mstore(result, div(shl(128, amount0), liquidity))
        mstore(add(result, 32), div(shl(128, amount1), liquidity))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type StorageSlot is bytes32;

using {load, loadTwo, store, storeTwo, next, add, sub} for StorageSlot global;

function load(StorageSlot slot) view returns (bytes32 value) {
    assembly ("memory-safe") {
        value := sload(slot)
    }
}

function loadTwo(StorageSlot slot) view returns (bytes32 value0, bytes32 value1) {
    value0 = slot.load();
    value1 = slot.next().load();
}

function store(StorageSlot slot, bytes32 value) {
    assembly ("memory-safe") {
        sstore(slot, value)
    }
}

function storeTwo(StorageSlot slot, bytes32 value0, bytes32 value1) {
    slot.store(value0);
    slot.next().store(value1);
}

function next(StorageSlot slot) pure returns (StorageSlot nextSlot) {
    assembly ("memory-safe") {
        nextSlot := add(slot, 1)
    }
}

function add(StorageSlot slot, uint256 addend) pure returns (StorageSlot summedSlot) {
    assembly ("memory-safe") {
        summedSlot := add(slot, addend)
    }
}

function sub(StorageSlot slot, uint256 subtrahend) pure returns (StorageSlot differenceSlot) {
    assembly ("memory-safe") {
        differenceSlot := sub(slot, subtrahend)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, MIN_SQRT_RATIO_RAW, MAX_SQRT_RATIO_RAW} from "./sqrtRatio.sol";

type SwapParameters is bytes32;

using {
    sqrtRatioLimit,
    amount,
    isToken1,
    skipAhead,
    isExactOut,
    isPriceIncreasing,
    withDefaultSqrtRatioLimit
} for SwapParameters global;

function sqrtRatioLimit(SwapParameters params) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, params)
    }
}

function amount(SwapParameters params) pure returns (int128 a) {
    assembly ("memory-safe") {
        a := signextend(15, shr(32, params))
    }
}

function isToken1(SwapParameters params) pure returns (bool t) {
    assembly ("memory-safe") {
        t := and(shr(31, params), 1)
    }
}

function skipAhead(SwapParameters params) pure returns (uint256 s) {
    assembly ("memory-safe") {
        s := and(params, 0x7fffffff)
    }
}

function createSwapParameters(SqrtRatio _sqrtRatioLimit, int128 _amount, bool _isToken1, uint256 _skipAhead)
    pure
    returns (SwapParameters p)
{
    assembly ("memory-safe") {
        // p = (sqrtRatioLimit << 160) | (amount << 32) | (isToken1 << 31) | skipAhead
        // Mask each field to ensure dirty bits don't interfere
        // For isToken1, use iszero(iszero()) to convert any non-zero value to 1
        p := or(
            shl(160, _sqrtRatioLimit),
            or(
                shl(32, and(_amount, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                or(shl(31, iszero(iszero(_isToken1))), and(_skipAhead, 0x7fffffff))
            )
        )
    }
}

function isExactOut(SwapParameters params) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(159, params), 1)
    }
}

function isPriceIncreasing(SwapParameters params) pure returns (bool yes) {
    bool _isExactOut = params.isExactOut();
    bool _isToken1 = params.isToken1();
    assembly ("memory-safe") {
        yes := xor(_isExactOut, _isToken1)
    }
}

function withDefaultSqrtRatioLimit(SwapParameters params) pure returns (SwapParameters updated) {
    bool increasing = params.isPriceIncreasing();
    assembly ("memory-safe") {
        let replace := iszero(shr(160, params))
        let orMask :=
            shl(160, mul(replace, or(mul(increasing, MAX_SQRT_RATIO_RAW), mul(iszero(increasing), MIN_SQRT_RATIO_RAW))))
        updated := or(orMask, params)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK, MAX_TICK_SPACING} from "../math/constants.sol";

/// @notice Pool configuration packed into a single bytes32
/// @dev Contains extension address (20 bytes), fee (8 bytes), and pool type config (4 bytes)
/// Pool type config (32 bits):
///   - Bit 31: discriminator (1 = concentrated, 0 = stableswap)
///   - For concentrated (bit 31 = 1): bits 30-0 are tick spacing
///   - For stableswap (bit 31 = 0): bits 30-24 are amplification factor, bits 23-0 are center tick
type PoolConfig is bytes32;

using {
    fee,
    extension,
    isConcentrated,
    isStableswap,
    isFullRange,
    concentratedTickSpacing,
    stableswapAmplification,
    stableswapCenterTick,
    stableswapActiveLiquidityTickRange,
    stableswapLiquidityWidth,
    validate,
    concentratedMaxLiquidityPerTick
} for PoolConfig global;

/// @notice Extracts the fee from a pool config
/// @param config The pool config
/// @return r The fee
function fee(PoolConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(32, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the extension address from a pool config
/// @param config The pool config
/// @return r The extension address
function extension(PoolConfig config) pure returns (address r) {
    assembly ("memory-safe") {
        r := shr(96, config)
    }
}

/// @notice Checks if this is a concentrated liquidity pool
/// @param config The pool config
/// @return r True if the pool is concentrated liquidity
function isConcentrated(PoolConfig config) pure returns (bool r) {
    r = !config.isStableswap();
}

/// @notice Extracts the tick spacing from a concentrated liquidity pool config
/// @dev Only valid for concentrated liquidity pools (isConcentrated() == true)
/// @param config The pool config
/// @return r The tick spacing
function concentratedTickSpacing(PoolConfig config) pure returns (uint32 r) {
    assembly ("memory-safe") {
        // Extract lower 31 bits (bits 30-0)
        r := and(config, 0x7fffffff)
    }
}

/// @notice Checks if this is a stableswap pool
/// @param config The pool config
/// @return r True if the pool is stableswap
function isStableswap(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // = iff bit 31 is not set
        r := iszero(and(0x80000000, config))
    }
}

/// @notice Determines if this pool is full range (special case of stableswap with amplification=0 and center=0)
/// @dev Full range can be slightly optimized in that we don't need to compute the sqrt ratio at the tick boundaries
/// @param config The pool config
/// @return r True if the pool is full range
function isFullRange(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // Full range when all 32 bits are 0 (discriminator=0, amplification=0, center=0)
        r := iszero(and(config, 0xffffffff))
    }
}

/// @notice Extracts the amplification factor from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @param config The pool config
/// @return r The amplification factor (0-127)
function stableswapAmplification(PoolConfig config) pure returns (uint8 r) {
    assembly ("memory-safe") {
        // Extract bits 30-24
        r := and(shr(24, config), 0x7f)
    }
}

/// @notice Extracts the center tick from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @dev The 24-bit center tick is scaled by 16 to get the actual tick
/// @param config The pool config
/// @return r The center tick
function stableswapCenterTick(PoolConfig config) pure returns (int32 r) {
    assembly ("memory-safe") {
        // Extract bits 23-0 and sign extend to 32 bits (24-bit signed integer)
        // Then multiply by 16, since the value does not have full precision
        r := mul(signextend(2, and(config, 0xffffff)), 16)
    }
}

/// @notice Returns the width of the liquidity range
function stableswapLiquidityWidth(PoolConfig config) pure returns (uint256 width) {
    uint8 amp = config.stableswapAmplification();

    assembly ("memory-safe") {
        width := shr(amp, MAX_TICK)
    }
}

/// @notice Computes the tick range where liquidity is active for stableswap pools
/// @param config The pool config
/// @return lower The lower tick of the bounds
/// @return upper The upper tick of the bounds
function stableswapActiveLiquidityTickRange(PoolConfig config) pure returns (int32 lower, int32 upper) {
    int32 center = config.stableswapCenterTick();
    uint256 width = config.stableswapLiquidityWidth();

    assembly ("memory-safe") {
        lower := sub(center, width)
        lower := add(lower, mul(sgt(MIN_TICK, lower), sub(MIN_TICK, lower)))

        upper := add(center, width)
        upper := sub(upper, mul(sgt(upper, MAX_TICK), sub(upper, MAX_TICK)))
    }
}

/// @notice Creates a PoolConfig for a concentrated liquidity pool
/// @param _fee The fee for the pool
/// @param _tickSpacing The tick spacing for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createConcentratedPoolConfig(uint64 _fee, uint32 _tickSpacing, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Set bit 31 to 1 for concentrated liquidity, then OR with tick spacing (bits 30-0)
        let typeConfig := or(0x80000000, and(_tickSpacing, 0x7fffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a stableswap pool
/// @param _fee The fee for the pool
/// @param _amplification The amplification factor (0-127)
/// @param _centerTick The center tick (will be divided by 16 and stored as 24-bit value)
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createStableswapPoolConfig(uint64 _fee, uint8 _amplification, int32 _centerTick, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Divide center tick by 16 to get 24-bit representation
        let stableswapCenterTick24 := sdiv(_centerTick, 16)
        // Pack: bit 31 = 0 (stableswap), bits 30-24 = amplification, bits 23-0 = center tick
        let typeConfig := or(shl(24, and(_amplification, 0x7f)), and(stableswapCenterTick24, 0xffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a full range pool (stableswap with amplification=0, center=0)
/// @param _fee The fee for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createFullRangePoolConfig(uint64 _fee, address _extension) pure returns (PoolConfig c) {
    assembly ("memory-safe") {
        // All 32 bits of type config are 0 (discriminator=0, amplification=0, center=0)
        c := or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff)))
    }
}

/// @notice Computes the maximum liquidity per tick for a given concentrated liquidity pool configuration.
/// @dev Only valid for concentrated liquidity pools. Stableswap pools don't use ticks.
/// @dev Calculated as type(uint128).max / (1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2)
/// @param config The concentrated liquidity pool configuration
/// @return maxLiquidity The maximum liquidity allowed to reference each tick
function concentratedMaxLiquidityPerTick(PoolConfig config) pure returns (uint128 maxLiquidity) {
    uint32 _tickSpacing = config.concentratedTickSpacing();

    assembly ("memory-safe") {
        // Calculate total number of usable ticks: 1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2
        // This represents all ticks from -MAX_TICK_MAGNITUDE to +MAX_TICK_MAGNITUDE, and tick 0
        let numTicks := add(1, mul(div(MAX_TICK, _tickSpacing), 2))

        maxLiquidity := div(sub(shl(128, 1), 1), numTicks)
    }
}

/// @notice Thrown when tick spacing exceeds the maximum allowed value
error InvalidTickSpacing();

/// @notice Thrown when amplification factor exceeds the maximum allowed value
error InvalidStableswapAmplification();

/// @notice Thrown when center tick is not between min and max tick
error InvalidCenterTick();

/// @notice Validates that a pool config is properly formatted
/// @param config The config to validate
function validate(PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (config.concentratedTickSpacing() > MAX_TICK_SPACING || config.concentratedTickSpacing() == 0) {
            revert InvalidTickSpacing();
        }
    } else {
        // Stableswap pool: validate amplification factor <= 26
        if (config.stableswapAmplification() > 26) {
            revert InvalidStableswapAmplification();
        }
        int32 centerTick = config.stableswapCenterTick();
        if (centerTick < MIN_TICK || centerTick > MAX_TICK) {
            revert InvalidCenterTick();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK, MAX_TICK_SPACING} from "../math/constants.sol";

/// @notice Pool configuration packed into a single bytes32
/// @dev Contains extension address (20 bytes), fee (8 bytes), and pool type config (4 bytes)
/// Pool type config (32 bits):
///   - Bit 31: discriminator (1 = concentrated, 0 = stableswap)
///   - For concentrated (bit 31 = 1): bits 30-0 are tick spacing
///   - For stableswap (bit 31 = 0): bits 30-24 are amplification factor, bits 23-0 are center tick
type PoolConfig is bytes32;

using {
    fee,
    extension,
    isConcentrated,
    isStableswap,
    isFullRange,
    concentratedTickSpacing,
    stableswapAmplification,
    stableswapCenterTick,
    stableswapActiveLiquidityTickRange,
    stableswapLiquidityWidth,
    validate,
    concentratedMaxLiquidityPerTick
} for PoolConfig global;

/// @notice Extracts the fee from a pool config
/// @param config The pool config
/// @return r The fee
function fee(PoolConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(32, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the extension address from a pool config
/// @param config The pool config
/// @return r The extension address
function extension(PoolConfig config) pure returns (address r) {
    assembly ("memory-safe") {
        r := shr(96, config)
    }
}

/// @notice Checks if this is a concentrated liquidity pool
/// @param config The pool config
/// @return r True if the pool is concentrated liquidity
function isConcentrated(PoolConfig config) pure returns (bool r) {
    r = !config.isStableswap();
}

/// @notice Extracts the tick spacing from a concentrated liquidity pool config
/// @dev Only valid for concentrated liquidity pools (isConcentrated() == true)
/// @param config The pool config
/// @return r The tick spacing
function concentratedTickSpacing(PoolConfig config) pure returns (uint32 r) {
    assembly ("memory-safe") {
        // Extract lower 31 bits (bits 30-0)
        r := and(config, 0x7fffffff)
    }
}

/// @notice Checks if this is a stableswap pool
/// @param config The pool config
/// @return r True if the pool is stableswap
function isStableswap(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // = iff bit 31 is not set
        r := iszero(and(0x80000000, config))
    }
}

/// @notice Determines if this pool is full range (special case of stableswap with amplification=0 and center=0)
/// @dev Full range can be slightly optimized in that we don't need to compute the sqrt ratio at the tick boundaries
/// @param config The pool config
/// @return r True if the pool is full range
function isFullRange(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // Full range when all 32 bits are 0 (discriminator=0, amplification=0, center=0)
        r := iszero(and(config, 0xffffffff))
    }
}

/// @notice Extracts the amplification factor from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @param config The pool config
/// @return r The amplification factor (0-127)
function stableswapAmplification(PoolConfig config) pure returns (uint8 r) {
    assembly ("memory-safe") {
        // Extract bits 30-24
        r := and(shr(24, config), 0x7f)
    }
}

/// @notice Extracts the center tick from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @dev The 24-bit center tick is scaled by 16 to get the actual tick
/// @param config The pool config
/// @return r The center tick
function stableswapCenterTick(PoolConfig config) pure returns (int32 r) {
    assembly ("memory-safe") {
        // Extract bits 23-0 and sign extend to 32 bits (24-bit signed integer)
        // Then multiply by 16, since the value does not have full precision
        r := mul(signextend(2, and(config, 0xffffff)), 16)
    }
}

/// @notice Returns the width of the liquidity range
function stableswapLiquidityWidth(PoolConfig config) pure returns (uint256 width) {
    uint8 amp = config.stableswapAmplification();

    assembly ("memory-safe") {
        width := shr(amp, MAX_TICK)
    }
}

/// @notice Computes the tick range where liquidity is active for stableswap pools
/// @param config The pool config
/// @return lower The lower tick of the bounds
/// @return upper The upper tick of the bounds
function stableswapActiveLiquidityTickRange(PoolConfig config) pure returns (int32 lower, int32 upper) {
    int32 center = config.stableswapCenterTick();
    uint256 width = config.stableswapLiquidityWidth();

    assembly ("memory-safe") {
        lower := sub(center, width)
        lower := add(lower, mul(sgt(MIN_TICK, lower), sub(MIN_TICK, lower)))

        upper := add(center, width)
        upper := sub(upper, mul(sgt(upper, MAX_TICK), sub(upper, MAX_TICK)))
    }
}

/// @notice Creates a PoolConfig for a concentrated liquidity pool
/// @param _fee The fee for the pool
/// @param _tickSpacing The tick spacing for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createConcentratedPoolConfig(uint64 _fee, uint32 _tickSpacing, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Set bit 31 to 1 for concentrated liquidity, then OR with tick spacing (bits 30-0)
        let typeConfig := or(0x80000000, and(_tickSpacing, 0x7fffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a stableswap pool
/// @param _fee The fee for the pool
/// @param _amplification The amplification factor (0-127)
/// @param _centerTick The center tick (will be divided by 16 and stored as 24-bit value)
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createStableswapPoolConfig(uint64 _fee, uint8 _amplification, int32 _centerTick, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Divide center tick by 16 to get 24-bit representation
        let stableswapCenterTick24 := sdiv(_centerTick, 16)
        // Pack: bit 31 = 0 (stableswap), bits 30-24 = amplification, bits 23-0 = center tick
        let typeConfig := or(shl(24, and(_amplification, 0x7f)), and(stableswapCenterTick24, 0xffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a full range pool (stableswap with amplification=0, center=0)
/// @param _fee The fee for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createFullRangePoolConfig(uint64 _fee, address _extension) pure returns (PoolConfig c) {
    assembly ("memory-safe") {
        // All 32 bits of type config are 0 (discriminator=0, amplification=0, center=0)
        c := or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff)))
    }
}

/// @notice Computes the maximum liquidity per tick for a given concentrated liquidity pool configuration.
/// @dev Only valid for concentrated liquidity pools. Stableswap pools don't use ticks.
/// @dev Calculated as type(uint128).max / (1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2)
/// @param config The concentrated liquidity pool configuration
/// @return maxLiquidity The maximum liquidity allowed to reference each tick
function concentratedMaxLiquidityPerTick(PoolConfig config) pure returns (uint128 maxLiquidity) {
    uint32 _tickSpacing = config.concentratedTickSpacing();

    assembly ("memory-safe") {
        // Calculate total number of usable ticks: 1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2
        // This represents all ticks from -MAX_TICK_MAGNITUDE to +MAX_TICK_MAGNITUDE, and tick 0
        let numTicks := add(1, mul(div(MAX_TICK, _tickSpacing), 2))

        maxLiquidity := div(sub(shl(128, 1), 1), numTicks)
    }
}

/// @notice Thrown when tick spacing exceeds the maximum allowed value
error InvalidTickSpacing();

/// @notice Thrown when amplification factor exceeds the maximum allowed value
error InvalidStableswapAmplification();

/// @notice Thrown when center tick is not between min and max tick
error InvalidCenterTick();

/// @notice Validates that a pool config is properly formatted
/// @param config The config to validate
function validate(PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (config.concentratedTickSpacing() > MAX_TICK_SPACING || config.concentratedTickSpacing() == 0) {
            revert InvalidTickSpacing();
        }
    } else {
        // Stableswap pool: validate amplification factor <= 26
        if (config.stableswapAmplification() > 26) {
            revert InvalidStableswapAmplification();
        }
        int32 centerTick = config.stableswapCenterTick();
        if (centerTick < MIN_TICK || centerTick > MAX_TICK) {
            revert InvalidCenterTick();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SqrtRatio, toSqrtRatio, MAX_FIXED_VALUE_ROUND_UP} from "../types/sqrtRatio.sol";

// Compute the next ratio from a delta amount0, always rounded towards starting price for input, and
// away from starting price for output
/// @dev Assumes sqrt ratio and liquidity are non-zero
function nextSqrtRatioFromAmount0(SqrtRatio _sqrtRatio, uint128 liquidity, int128 amount)
    pure
    returns (SqrtRatio sqrtRatioNext)
{
    if (amount == 0) {
        return _sqrtRatio;
    }

    uint256 sqrtRatio = _sqrtRatio.toFixed();

    uint256 liquidityX128;
    assembly ("memory-safe") {
        liquidityX128 := shl(128, liquidity)
    }

    if (amount < 0) {
        uint256 amountAbs;
        assembly ("memory-safe") {
            amountAbs := sub(0, amount)
        }
        unchecked {
            // multiplication will revert on overflow, so we return the maximum value for the type
            if (amountAbs > FixedPointMathLib.rawDiv(type(uint256).max, sqrtRatio)) {
                return SqrtRatio.wrap(type(uint96).max);
            }

            uint256 product = sqrtRatio * amountAbs;

            // again it will overflow if this is the case, so return the max value
            if (product >= liquidityX128) {
                return SqrtRatio.wrap(type(uint96).max);
            }

            uint256 denominator = liquidityX128 - product;

            uint256 resultFixed = FixedPointMathLib.fullMulDivUp(liquidityX128, sqrtRatio, denominator);

            if (resultFixed > MAX_FIXED_VALUE_ROUND_UP) {
                return SqrtRatio.wrap(type(uint96).max);
            }

            sqrtRatioNext = toSqrtRatio(resultFixed, true);
        }
    } else {
        uint256 sqrtRatioRaw;
        assembly ("memory-safe") {
            // this can never overflow, amountAbs is limited to 2**128-1 and liquidityX128 / sqrtRatio is limited to (2**128-1 << 128)
            // adding the 2 values can at most equal type(uint256).max
            let denominator := add(div(liquidityX128, sqrtRatio), amount)
            sqrtRatioRaw := add(div(liquidityX128, denominator), iszero(iszero(mod(liquidityX128, denominator))))
        }

        sqrtRatioNext = toSqrtRatio(sqrtRatioRaw, true);
    }
}

/// @dev Assumes liquidity is non-zero
function nextSqrtRatioFromAmount1(SqrtRatio _sqrtRatio, uint128 liquidity, int128 amount)
    pure
    returns (SqrtRatio sqrtRatioNext)
{
    uint256 sqrtRatio = _sqrtRatio.toFixed();

    unchecked {
        uint256 liquidityU256;
        assembly ("memory-safe") {
            liquidityU256 := liquidity
        }

        if (amount < 0) {
            uint256 quotient;
            assembly ("memory-safe") {
                let numerator := shl(128, sub(0, amount))
                quotient := add(div(numerator, liquidityU256), iszero(iszero(mod(numerator, liquidityU256))))
            }

            uint256 sqrtRatioNextFixed = FixedPointMathLib.zeroFloorSub(sqrtRatio, quotient);

            sqrtRatioNext = toSqrtRatio(sqrtRatioNextFixed, false);
        } else {
            uint256 quotient;
            assembly ("memory-safe") {
                quotient := div(shl(128, amount), liquidityU256)
            }
            uint256 sum = sqrtRatio + quotient;
            if (sum < sqrtRatio || sum > type(uint192).max) {
                return SqrtRatio.wrap(type(uint96).max);
            }
            sqrtRatioNext = toSqrtRatio(sum, false);
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Snapshot is bytes32;

using {timestamp, secondsPerLiquidityCumulative, tickCumulative} for Snapshot global;

function timestamp(Snapshot snapshot) pure returns (uint32 t) {
    assembly ("memory-safe") {
        t := and(snapshot, 0xFFFFFFFF)
    }
}

function secondsPerLiquidityCumulative(Snapshot snapshot) pure returns (uint160 s) {
    assembly ("memory-safe") {
        s := and(shr(32, snapshot), 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
    }
}

function tickCumulative(Snapshot snapshot) pure returns (int64 t) {
    assembly ("memory-safe") {
        t := signextend(7, shr(192, snapshot))
    }
}

function createSnapshot(uint32 _timestamp, uint160 _secondsPerLiquidityCumulative, int64 _tickCumulative)
    pure
    returns (Snapshot s)
{
    assembly ("memory-safe") {
        // s = timestamp | (secondsPerLiquidityCumulative << 32) | (tickCumulative << 192)
        s := or(
            or(
                and(_timestamp, 0xFFFFFFFF),
                shl(32, and(_secondsPerLiquidityCumulative, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
            ),
            shl(192, and(_tickCumulative, 0xFFFFFFFFFFFFFFFF))
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Locker is bytes32;

using {id, addr, parse} for Locker global;

function id(Locker locker) pure returns (uint256 v) {
    assembly ("memory-safe") {
        v := sub(shr(160, locker), 1)
    }
}

function addr(Locker locker) pure returns (address v) {
    assembly ("memory-safe") {
        v := shr(96, shl(96, locker))
    }
}

function parse(Locker locker) pure returns (uint256 lockerId, address lockerAddr) {
    assembly ("memory-safe") {
        lockerId := sub(shr(160, locker), 1)
        lockerAddr := shr(96, shl(96, locker))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";

error Amount0DeltaOverflow();
error Amount1DeltaOverflow();

function sortAndConvertToFixedSqrtRatios(SqrtRatio sqrtRatioA, SqrtRatio sqrtRatioB)
    pure
    returns (uint256 sqrtRatioLower, uint256 sqrtRatioUpper)
{
    sqrtRatioLower = sqrtRatioA.toFixed();
    sqrtRatioUpper = sqrtRatioB.toFixed();
    assembly ("memory-safe") {
        let diff := mul(sub(sqrtRatioLower, sqrtRatioUpper), gt(sqrtRatioLower, sqrtRatioUpper))

        sqrtRatioLower := sub(sqrtRatioLower, diff)
        sqrtRatioUpper := add(sqrtRatioUpper, diff)
    }
}

/// @dev Assumes that the sqrt ratios are valid
function amount0Delta(SqrtRatio sqrtRatioA, SqrtRatio sqrtRatioB, uint128 liquidity, bool roundUp)
    pure
    returns (uint128 amount0)
{
    (uint256 sqrtRatioLower, uint256 sqrtRatioUpper) = sortAndConvertToFixedSqrtRatios(sqrtRatioA, sqrtRatioB);
    amount0 = amount0DeltaSorted(sqrtRatioLower, sqrtRatioUpper, liquidity, roundUp);
}

/// @dev Assumes that the sqrt ratios are non-zero and sorted
function amount0DeltaSorted(uint256 sqrtRatioLower, uint256 sqrtRatioUpper, uint128 liquidity, bool roundUp)
    pure
    returns (uint128 amount0)
{
    unchecked {
        uint256 liquidityX128;
        assembly ("memory-safe") {
            liquidityX128 := shl(128, liquidity)
        }
        if (roundUp) {
            uint256 result0 =
                FixedPointMathLib.fullMulDivUp(liquidityX128, (sqrtRatioUpper - sqrtRatioLower), sqrtRatioUpper);
            assembly ("memory-safe") {
                let result := add(div(result0, sqrtRatioLower), iszero(iszero(mod(result0, sqrtRatioLower))))
                if shr(128, result) {
                    // cast sig "Amount0DeltaOverflow()"
                    mstore(0, 0xb4ef2546)
                    revert(0x1c, 0x04)
                }
                amount0 := result
            }
        } else {
            uint256 result0 =
                FixedPointMathLib.fullMulDivUnchecked(liquidityX128, (sqrtRatioUpper - sqrtRatioLower), sqrtRatioUpper);
            uint256 result = FixedPointMathLib.rawDiv(result0, sqrtRatioLower);
            assembly ("memory-safe") {
                if shr(128, result) {
                    // cast sig "Amount0DeltaOverflow()"
                    mstore(0, 0xb4ef2546)
                    revert(0x1c, 0x04)
                }
                amount0 := result
            }
        }
    }
}

/// @dev Assumes that the sqrt ratios are valid
function amount1Delta(SqrtRatio sqrtRatioA, SqrtRatio sqrtRatioB, uint128 liquidity, bool roundUp)
    pure
    returns (uint128 amount1)
{
    (uint256 sqrtRatioLower, uint256 sqrtRatioUpper) = sortAndConvertToFixedSqrtRatios(sqrtRatioA, sqrtRatioB);
    amount1 = amount1DeltaSorted(sqrtRatioLower, sqrtRatioUpper, liquidity, roundUp);
}

function amount1DeltaSorted(uint256 sqrtRatioLower, uint256 sqrtRatioUpper, uint128 liquidity, bool roundUp)
    pure
    returns (uint128 amount1)
{
    unchecked {
        uint256 difference = sqrtRatioUpper - sqrtRatioLower;
        uint256 liquidityU256;
        assembly ("memory-safe") {
            liquidityU256 := liquidity
        }

        if (roundUp) {
            uint256 result = FixedPointMathLib.fullMulDivN(difference, liquidityU256, 128);
            assembly ("memory-safe") {
                // addition is safe from overflow because the result of fullMulDivN will never equal type(uint256).max
                result := add(
                    result,
                    iszero(iszero(mulmod(difference, liquidityU256, 0x100000000000000000000000000000000)))
                )
                if shr(128, result) {
                    // cast sig "Amount1DeltaOverflow()"
                    mstore(0, 0x59d2b24a)
                    revert(0x1c, 0x04)
                }
                amount1 := result
            }
        } else {
            uint256 result = FixedPointMathLib.fullMulDivN(difference, liquidityU256, 128);
            assembly ("memory-safe") {
                if shr(128, result) {
                    // cast sig "Amount1DeltaOverflow()"
                    mstore(0, 0x59d2b24a)
                    revert(0x1c, 0x04)
                }
                amount1 := result
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK} from "../math/constants.sol";
import {PoolConfig} from "./poolConfig.sol";

type PositionId is bytes32;

using {validate, salt, tickLower, tickUpper} for PositionId global;

function salt(PositionId positionId) pure returns (bytes24 v) {
    assembly ("memory-safe") {
        v := shl(64, shr(64, positionId))
    }
}

function tickLower(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // shift down, then signextend to 32 bits
        v := signextend(3, shr(32, positionId))
    }
}

function tickUpper(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // lowest 4 bytes, then signextend to 32 bits
        v := signextend(3, positionId)
    }
}

function createPositionId(bytes24 _salt, int32 _tickLower, int32 _tickUpper) pure returns (PositionId v) {
    assembly ("memory-safe") {
        // v = salt | (tickLower << 32) | tickUpper
        v := or(shl(64, shr(64, _salt)), or(shl(32, and(_tickLower, 0xFFFFFFFF)), and(_tickUpper, 0xFFFFFFFF)))
    }
}

/// @notice Thrown when the order of the position bounds is invalid, i.e. tickLower >= tickUpper
error BoundsOrder();
/// @notice Thrown when the bounds of the position are outside the pool's min/max tick range
error MinMaxBounds();
/// @notice Thrown when the ticks of the bounds do not align with tick spacing for concentrated pools
error BoundsTickSpacing();
/// @notice Thrown when stableswap pool positions are not at the min/max tick for the config
error StableswapMustBeFullRange();

function validate(PositionId positionId, PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (positionId.tickLower() >= positionId.tickUpper()) revert BoundsOrder();
        if (positionId.tickLower() < MIN_TICK || positionId.tickUpper() > MAX_TICK) revert MinMaxBounds();
        int32 spacing = int32(config.concentratedTickSpacing());
        if (positionId.tickLower() % spacing != 0 || positionId.tickUpper() % spacing != 0) revert BoundsTickSpacing();
    } else {
        (int32 lower, int32 upper) = config.stableswapActiveLiquidityTickRange();
        // For stableswap pools, positions must be exactly min/max tick
        if (positionId.tickLower() != lower || positionId.tickUpper() != upper) revert StableswapMustBeFullRange();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, MIN_SQRT_RATIO_RAW, MAX_SQRT_RATIO_RAW} from "./sqrtRatio.sol";

type SwapParameters is bytes32;

using {
    sqrtRatioLimit,
    amount,
    isToken1,
    skipAhead,
    isExactOut,
    isPriceIncreasing,
    withDefaultSqrtRatioLimit
} for SwapParameters global;

function sqrtRatioLimit(SwapParameters params) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, params)
    }
}

function amount(SwapParameters params) pure returns (int128 a) {
    assembly ("memory-safe") {
        a := signextend(15, shr(32, params))
    }
}

function isToken1(SwapParameters params) pure returns (bool t) {
    assembly ("memory-safe") {
        t := and(shr(31, params), 1)
    }
}

function skipAhead(SwapParameters params) pure returns (uint256 s) {
    assembly ("memory-safe") {
        s := and(params, 0x7fffffff)
    }
}

function createSwapParameters(SqrtRatio _sqrtRatioLimit, int128 _amount, bool _isToken1, uint256 _skipAhead)
    pure
    returns (SwapParameters p)
{
    assembly ("memory-safe") {
        // p = (sqrtRatioLimit << 160) | (amount << 32) | (isToken1 << 31) | skipAhead
        // Mask each field to ensure dirty bits don't interfere
        // For isToken1, use iszero(iszero()) to convert any non-zero value to 1
        p := or(
            shl(160, _sqrtRatioLimit),
            or(
                shl(32, and(_amount, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                or(shl(31, iszero(iszero(_isToken1))), and(_skipAhead, 0x7fffffff))
            )
        )
    }
}

function isExactOut(SwapParameters params) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(159, params), 1)
    }
}

function isPriceIncreasing(SwapParameters params) pure returns (bool yes) {
    bool _isExactOut = params.isExactOut();
    bool _isToken1 = params.isToken1();
    assembly ("memory-safe") {
        yes := xor(_isExactOut, _isToken1)
    }
}

function withDefaultSqrtRatioLimit(SwapParameters params) pure returns (SwapParameters updated) {
    bool increasing = params.isPriceIncreasing();
    assembly ("memory-safe") {
        let replace := iszero(shr(160, params))
        let orMask :=
            shl(160, mul(replace, or(mul(increasing, MAX_SQRT_RATIO_RAW), mul(iszero(increasing), MIN_SQRT_RATIO_RAW))))
        updated := or(orMask, params)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

using {funded, claimed, setFunded, setClaimed, getRemaining} for DropState global;

/// @notice Represents the state of a drop with funded and claimed amounts
/// @dev Packed into a single bytes32 slot: funded (128 bits) + claimed (128 bits)
type DropState is bytes32;

/// @notice Gets the funded amount from a drop state
/// @param state The drop state
/// @return amount The funded amount
function funded(DropState state) pure returns (uint128 amount) {
    assembly ("memory-safe") {
        amount := shr(128, state)
    }
}

/// @notice Gets the claimed amount from a drop state
/// @param state The drop state
/// @return amount The claimed amount
function claimed(DropState state) pure returns (uint128 amount) {
    assembly ("memory-safe") {
        amount := and(state, 0xffffffffffffffffffffffffffffffff)
    }
}

/// @notice Sets the funded amount in a drop state
/// @param state The drop state
/// @param amount The funded amount to set
/// @return newState The updated drop state
function setFunded(DropState state, uint128 amount) pure returns (DropState newState) {
    assembly ("memory-safe") {
        newState := or(and(state, 0xffffffffffffffffffffffffffffffff), shl(128, amount))
    }
}

/// @notice Sets the claimed amount in a drop state
/// @param state The drop state
/// @param amount The claimed amount to set
/// @return newState The updated drop state
function setClaimed(DropState state, uint128 amount) pure returns (DropState newState) {
    assembly ("memory-safe") {
        newState := or(and(state, 0xffffffffffffffffffffffffffffffff00000000000000000000000000000000), amount)
    }
}

/// @notice Gets the remaining amount (funded - claimed) from a drop state
/// @param state The drop state
/// @return remaining The remaining amount available for claims
function getRemaining(DropState state) pure returns (uint128 remaining) {
    unchecked {
        remaining = state.funded() - state.claimed();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type StorageSlot is bytes32;

using {load, loadTwo, store, storeTwo, next, add, sub} for StorageSlot global;

function load(StorageSlot slot) view returns (bytes32 value) {
    assembly ("memory-safe") {
        value := sload(slot)
    }
}

function loadTwo(StorageSlot slot) view returns (bytes32 value0, bytes32 value1) {
    value0 = slot.load();
    value1 = slot.next().load();
}

function store(StorageSlot slot, bytes32 value) {
    assembly ("memory-safe") {
        sstore(slot, value)
    }
}

function storeTwo(StorageSlot slot, bytes32 value0, bytes32 value1) {
    slot.store(value0);
    slot.next().store(value1);
}

function next(StorageSlot slot) pure returns (StorageSlot nextSlot) {
    assembly ("memory-safe") {
        nextSlot := add(slot, 1)
    }
}

function add(StorageSlot slot, uint256 addend) pure returns (StorageSlot summedSlot) {
    assembly ("memory-safe") {
        summedSlot := add(slot, addend)
    }
}

function sub(StorageSlot slot, uint256 subtrahend) pure returns (StorageSlot differenceSlot) {
    assembly ("memory-safe") {
        differenceSlot := sub(slot, subtrahend)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio} from "./sqrtRatio.sol";

type PoolState is bytes32;

using {sqrtRatio, tick, liquidity, isInitialized, parse} for PoolState global;

function sqrtRatio(PoolState state) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, state)
    }
}

function tick(PoolState state) pure returns (int32 t) {
    assembly ("memory-safe") {
        t := signextend(3, shr(128, state))
    }
}

function liquidity(PoolState state) pure returns (uint128 l) {
    assembly ("memory-safe") {
        l := shr(128, shl(128, state))
    }
}

function isInitialized(PoolState state) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := iszero(iszero(state))
    }
}

function parse(PoolState state) pure returns (SqrtRatio r, int32 t, uint128 l) {
    assembly ("memory-safe") {
        r := shr(160, state)
        t := signextend(3, shr(128, state))
        l := shr(128, shl(128, state))
    }
}

function createPoolState(SqrtRatio _sqrtRatio, int32 _tick, uint128 _liquidity) pure returns (PoolState s) {
    assembly ("memory-safe") {
        // s = (sqrtRatio << 160) | (_tick << 128) | liquidity
        s := or(shl(160, _sqrtRatio), or(shl(128, and(_tick, 0xFFFFFFFF)), shr(128, shl(128, _liquidity))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Locker is bytes32;

using {id, addr, parse} for Locker global;

function id(Locker locker) pure returns (uint256 v) {
    assembly ("memory-safe") {
        v := sub(shr(160, locker), 1)
    }
}

function addr(Locker locker) pure returns (address v) {
    assembly ("memory-safe") {
        v := shr(96, shl(96, locker))
    }
}

function parse(Locker locker) pure returns (uint256 lockerId, address lockerAddr) {
    assembly ("memory-safe") {
        lockerId := sub(shr(160, locker), 1)
        lockerAddr := shr(96, shl(96, locker))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Order configuration packed into a single bytes32
/// @dev Contains fee (8 bytes), isToken1 (1 byte), padding (7 bytes), start time (8 bytes), and end time (8 bytes)
type OrderConfig is bytes32;

using {fee, isToken1, startTime, endTime} for OrderConfig global;

/// @notice Extracts the fee from an order config
/// @param config The order config
/// @return r The fee
function fee(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := shr(192, config)
    }
}

/// @notice Extracts isToken1 from an order config
/// @param config The order config
/// @return r Whether the order is selling token1
function isToken1(OrderConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(iszero(byte(8, config)))
    }
}

/// @notice Extracts the start time from an order config
/// @param config The order config
/// @return r The start time
function startTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(64, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the end time from an order config
/// @param config The order config
/// @return r The end time
function endTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(config, 0xffffffffffffffff)
    }
}

/// @notice Creates an OrderConfig from individual components
/// @param _fee The fee of the TWAMM pool
/// @param _startTime The start time of the order
/// @param _endTime The end time of the order
/// @return c The packed configuration
function createOrderConfig(uint64 _fee, bool _isToken1, uint64 _startTime, uint64 _endTime)
    pure
    returns (OrderConfig c)
{
    assembly ("memory-safe") {
        // Mask inputs to ensure only relevant bits are used
        c := add(
            add(shl(192, _fee), add(shl(184, iszero(iszero(_isToken1))), shl(64, and(_startTime, 0xffffffffffffffff)))),
            and(_endTime, 0xffffffffffffffff)
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "./poolKey.sol";
import {OrderConfig} from "./orderConfig.sol";
import {OrderId} from "./orderId.sol";

using {toOrderId, toPoolKey, buyToken, sellToken} for OrderKey global;

/// @notice Order key structure identifying a TWAMM order
/// @dev Contains all parameters needed to uniquely identify an order
struct OrderKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing fee, isToken1, start, and end time
    OrderConfig config;
}

/// @notice Extracts the buy token from an order key
/// @param ok The order key
/// @return r The buy token
function buyToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken0 = !ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken0, 0x20)))
    }
}

/// @notice Extracts the sell token from an order key
/// @param ok The order key
/// @return r The sell token
function sellToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken1 = ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken1, 0x20)))
    }
}

/// @notice Computes the order ID from an order key
/// @param orderKey The order key
/// @return id The computed order ID
function toOrderId(OrderKey memory orderKey) pure returns (OrderId id) {
    assembly ("memory-safe") {
        id := keccak256(orderKey, 96)
    }
}

/// @notice Converts an OrderKey to its corresponding PoolKey
/// @dev Determines the correct token ordering and constructs the pool key with TWAMM as extension
/// @param orderKey The order key containing sell/buy tokens and fee
/// @param twamm The TWAMM contract address to use as the extension
/// @return poolKey The corresponding pool key for the order
function toPoolKey(OrderKey memory orderKey, address twamm) pure returns (PoolKey memory poolKey) {
    uint64 _fee = orderKey.config.fee();
    assembly ("memory-safe") {
        mcopy(poolKey, orderKey, 64)
        mstore(add(poolKey, 64), add(shl(96, twamm), shl(32, _fee)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "./poolKey.sol";
import {OrderConfig} from "./orderConfig.sol";
import {OrderId} from "./orderId.sol";

using {toOrderId, toPoolKey, buyToken, sellToken} for OrderKey global;

/// @notice Order key structure identifying a TWAMM order
/// @dev Contains all parameters needed to uniquely identify an order
struct OrderKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing fee, isToken1, start, and end time
    OrderConfig config;
}

/// @notice Extracts the buy token from an order key
/// @param ok The order key
/// @return r The buy token
function buyToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken0 = !ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken0, 0x20)))
    }
}

/// @notice Extracts the sell token from an order key
/// @param ok The order key
/// @return r The sell token
function sellToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken1 = ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken1, 0x20)))
    }
}

/// @notice Computes the order ID from an order key
/// @param orderKey The order key
/// @return id The computed order ID
function toOrderId(OrderKey memory orderKey) pure returns (OrderId id) {
    assembly ("memory-safe") {
        id := keccak256(orderKey, 96)
    }
}

/// @notice Converts an OrderKey to its corresponding PoolKey
/// @dev Determines the correct token ordering and constructs the pool key with TWAMM as extension
/// @param orderKey The order key containing sell/buy tokens and fee
/// @param twamm The TWAMM contract address to use as the extension
/// @return poolKey The corresponding pool key for the order
function toPoolKey(OrderKey memory orderKey, address twamm) pure returns (PoolKey memory poolKey) {
    uint64 _fee = orderKey.config.fee();
    assembly ("memory-safe") {
        mcopy(poolKey, orderKey, 64)
        mstore(add(poolKey, 64), add(shl(96, twamm), shl(32, _fee)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a TWAMM order
/// @dev Wraps bytes32 to provide type safety for order identifiers
type OrderId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a TWAMM order
/// @dev Wraps bytes32 to provide type safety for order identifiers
type OrderId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Locker is bytes32;

using {id, addr, parse} for Locker global;

function id(Locker locker) pure returns (uint256 v) {
    assembly ("memory-safe") {
        v := sub(shr(160, locker), 1)
    }
}

function addr(Locker locker) pure returns (address v) {
    assembly ("memory-safe") {
        v := shr(96, shl(96, locker))
    }
}

function parse(Locker locker) pure returns (uint256 lockerId, address lockerAddr) {
    assembly ("memory-safe") {
        lockerId := sub(shr(160, locker), 1)
        lockerAddr := shr(96, shl(96, locker))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type BuybacksState is bytes32;

using {
    targetOrderDuration,
    minOrderDuration,
    fee,
    lastEndTime,
    lastOrderDuration,
    lastFee,
    isConfigured,
    parse
} for BuybacksState global;

function targetOrderDuration(BuybacksState state) pure returns (uint32 duration) {
    assembly ("memory-safe") {
        duration := and(state, 0xFFFFFFFF)
    }
}

function minOrderDuration(BuybacksState state) pure returns (uint32 duration) {
    assembly ("memory-safe") {
        duration := and(shr(32, state), 0xFFFFFFFF)
    }
}

function fee(BuybacksState state) pure returns (uint64 f) {
    assembly ("memory-safe") {
        f := and(shr(64, state), 0xFFFFFFFFFFFFFFFF)
    }
}

function lastEndTime(BuybacksState state) pure returns (uint32 endTime) {
    assembly ("memory-safe") {
        endTime := and(shr(128, state), 0xFFFFFFFF)
    }
}

function lastOrderDuration(BuybacksState state) pure returns (uint32 duration) {
    assembly ("memory-safe") {
        duration := and(shr(160, state), 0xFFFFFFFF)
    }
}

function lastFee(BuybacksState state) pure returns (uint64 f) {
    assembly ("memory-safe") {
        f := shr(192, state)
    }
}

function isConfigured(BuybacksState state) pure returns (bool) {
    return minOrderDuration(state) != 0;
}

function parse(BuybacksState state)
    pure
    returns (
        uint32 _targetOrderDuration,
        uint32 _minOrderDuration,
        uint64 _fee,
        uint32 _lastEndTime,
        uint32 _lastOrderDuration,
        uint64 _lastFee
    )
{
    assembly ("memory-safe") {
        _targetOrderDuration := and(state, 0xFFFFFFFF)
        _minOrderDuration := and(shr(32, state), 0xFFFFFFFF)
        _fee := and(shr(64, state), 0xFFFFFFFFFFFFFFFF)
        _lastEndTime := and(shr(128, state), 0xFFFFFFFF)
        _lastOrderDuration := and(shr(160, state), 0xFFFFFFFF)
        _lastFee := shr(192, state)
    }
}

function createBuybacksState(
    uint32 _targetOrderDuration,
    uint32 _minOrderDuration,
    uint64 _fee,
    uint32 _lastEndTime,
    uint32 _lastOrderDuration,
    uint64 _lastFee
) pure returns (BuybacksState state) {
    assembly ("memory-safe") {
        state := or(
            or(
                or(and(_targetOrderDuration, 0xFFFFFFFF), shl(32, and(_minOrderDuration, 0xFFFFFFFF))),
                shl(64, and(_fee, 0xFFFFFFFFFFFFFFFF))
            ),
            or(
                or(shl(128, and(_lastEndTime, 0xFFFFFFFF)), shl(160, and(_lastOrderDuration, 0xFFFFFFFF))),
                shl(192, _lastFee)
            )
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type PoolBalanceUpdate is bytes32;

using {delta0, delta1} for PoolBalanceUpdate global;

function delta0(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, shr(128, update))
    }
}

function delta1(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, update)
    }
}

function createPoolBalanceUpdate(int128 _delta0, int128 _delta1) pure returns (PoolBalanceUpdate update) {
    assembly ("memory-safe") {
        // update = (delta0 << 128) | delta1
        update := or(shl(128, _delta0), and(_delta1, 0xffffffffffffffffffffffffffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "./poolKey.sol";
import {OrderConfig} from "./orderConfig.sol";
import {OrderId} from "./orderId.sol";

using {toOrderId, toPoolKey, buyToken, sellToken} for OrderKey global;

/// @notice Order key structure identifying a TWAMM order
/// @dev Contains all parameters needed to uniquely identify an order
struct OrderKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing fee, isToken1, start, and end time
    OrderConfig config;
}

/// @notice Extracts the buy token from an order key
/// @param ok The order key
/// @return r The buy token
function buyToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken0 = !ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken0, 0x20)))
    }
}

/// @notice Extracts the sell token from an order key
/// @param ok The order key
/// @return r The sell token
function sellToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken1 = ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken1, 0x20)))
    }
}

/// @notice Computes the order ID from an order key
/// @param orderKey The order key
/// @return id The computed order ID
function toOrderId(OrderKey memory orderKey) pure returns (OrderId id) {
    assembly ("memory-safe") {
        id := keccak256(orderKey, 96)
    }
}

/// @notice Converts an OrderKey to its corresponding PoolKey
/// @dev Determines the correct token ordering and constructs the pool key with TWAMM as extension
/// @param orderKey The order key containing sell/buy tokens and fee
/// @param twamm The TWAMM contract address to use as the extension
/// @return poolKey The corresponding pool key for the order
function toPoolKey(OrderKey memory orderKey, address twamm) pure returns (PoolKey memory poolKey) {
    uint64 _fee = orderKey.config.fee();
    assembly ("memory-safe") {
        mcopy(poolKey, orderKey, 64)
        mstore(add(poolKey, 64), add(shl(96, twamm), shl(32, _fee)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, MIN_SQRT_RATIO_RAW, MAX_SQRT_RATIO_RAW} from "./sqrtRatio.sol";

type SwapParameters is bytes32;

using {
    sqrtRatioLimit,
    amount,
    isToken1,
    skipAhead,
    isExactOut,
    isPriceIncreasing,
    withDefaultSqrtRatioLimit
} for SwapParameters global;

function sqrtRatioLimit(SwapParameters params) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, params)
    }
}

function amount(SwapParameters params) pure returns (int128 a) {
    assembly ("memory-safe") {
        a := signextend(15, shr(32, params))
    }
}

function isToken1(SwapParameters params) pure returns (bool t) {
    assembly ("memory-safe") {
        t := and(shr(31, params), 1)
    }
}

function skipAhead(SwapParameters params) pure returns (uint256 s) {
    assembly ("memory-safe") {
        s := and(params, 0x7fffffff)
    }
}

function createSwapParameters(SqrtRatio _sqrtRatioLimit, int128 _amount, bool _isToken1, uint256 _skipAhead)
    pure
    returns (SwapParameters p)
{
    assembly ("memory-safe") {
        // p = (sqrtRatioLimit << 160) | (amount << 32) | (isToken1 << 31) | skipAhead
        // Mask each field to ensure dirty bits don't interfere
        // For isToken1, use iszero(iszero()) to convert any non-zero value to 1
        p := or(
            shl(160, _sqrtRatioLimit),
            or(
                shl(32, and(_amount, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                or(shl(31, iszero(iszero(_isToken1))), and(_skipAhead, 0x7fffffff))
            )
        )
    }
}

function isExactOut(SwapParameters params) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(159, params), 1)
    }
}

function isPriceIncreasing(SwapParameters params) pure returns (bool yes) {
    bool _isExactOut = params.isExactOut();
    bool _isToken1 = params.isToken1();
    assembly ("memory-safe") {
        yes := xor(_isExactOut, _isToken1)
    }
}

function withDefaultSqrtRatioLimit(SwapParameters params) pure returns (SwapParameters updated) {
    bool increasing = params.isPriceIncreasing();
    assembly ("memory-safe") {
        let replace := iszero(shr(160, params))
        let orMask :=
            shl(160, mul(replace, or(mul(increasing, MAX_SQRT_RATIO_RAW), mul(iszero(increasing), MIN_SQRT_RATIO_RAW))))
        updated := or(orMask, params)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

struct CallPoints {
    bool beforeInitializePool;
    bool afterInitializePool;
    bool beforeSwap;
    bool afterSwap;
    bool beforeUpdatePosition;
    bool afterUpdatePosition;
    bool beforeCollectFees;
    bool afterCollectFees;
}

using {eq, isValid, toUint8} for CallPoints global;

function eq(CallPoints memory a, CallPoints memory b) pure returns (bool) {
    return (a.beforeInitializePool == b.beforeInitializePool && a.afterInitializePool == b.afterInitializePool
            && a.beforeSwap == b.beforeSwap && a.afterSwap == b.afterSwap
            && a.beforeUpdatePosition == b.beforeUpdatePosition && a.afterUpdatePosition == b.afterUpdatePosition
            && a.beforeCollectFees == b.beforeCollectFees && a.afterCollectFees == b.afterCollectFees);
}

function isValid(CallPoints memory a) pure returns (bool) {
    return (a.beforeInitializePool || a.afterInitializePool || a.beforeSwap || a.afterSwap || a.beforeUpdatePosition
            || a.afterUpdatePosition || a.beforeCollectFees || a.afterCollectFees);
}

function toUint8(CallPoints memory callPoints) pure returns (uint8 b) {
    assembly ("memory-safe") {
        b := add(
            add(
                add(
                    add(
                        add(
                            add(
                                add(mload(callPoints), mul(128, mload(add(callPoints, 32)))),
                                mul(64, mload(add(callPoints, 64)))
                            ),
                            mul(32, mload(add(callPoints, 96)))
                        ),
                        mul(16, mload(add(callPoints, 128)))
                    ),
                    mul(8, mload(add(callPoints, 160)))
                ),
                mul(4, mload(add(callPoints, 192)))
            ),
            mul(2, mload(add(callPoints, 224)))
        )
    }
}

function addressToCallPoints(address a) pure returns (CallPoints memory result) {
    result = byteToCallPoints(uint8(uint160(a) >> 152));
}

function byteToCallPoints(uint8 b) pure returns (CallPoints memory result) {
    // note the order of bytes does not match the struct order of elements because we are matching the cairo implementation
    // which for legacy reasons has the fields in this order
    result = CallPoints({
        beforeInitializePool: (b & 1) != 0,
        afterInitializePool: (b & 128) != 0,
        beforeSwap: (b & 64) != 0,
        afterSwap: (b & 32) != 0,
        beforeUpdatePosition: (b & 16) != 0,
        afterUpdatePosition: (b & 8) != 0,
        beforeCollectFees: (b & 4) != 0,
        afterCollectFees: (b & 2) != 0
    });
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Order configuration packed into a single bytes32
/// @dev Contains fee (8 bytes), isToken1 (1 byte), padding (7 bytes), start time (8 bytes), and end time (8 bytes)
type OrderConfig is bytes32;

using {fee, isToken1, startTime, endTime} for OrderConfig global;

/// @notice Extracts the fee from an order config
/// @param config The order config
/// @return r The fee
function fee(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := shr(192, config)
    }
}

/// @notice Extracts isToken1 from an order config
/// @param config The order config
/// @return r Whether the order is selling token1
function isToken1(OrderConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(iszero(byte(8, config)))
    }
}

/// @notice Extracts the start time from an order config
/// @param config The order config
/// @return r The start time
function startTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(64, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the end time from an order config
/// @param config The order config
/// @return r The end time
function endTime(OrderConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(config, 0xffffffffffffffff)
    }
}

/// @notice Creates an OrderConfig from individual components
/// @param _fee The fee of the TWAMM pool
/// @param _startTime The start time of the order
/// @param _endTime The end time of the order
/// @return c The packed configuration
function createOrderConfig(uint64 _fee, bool _isToken1, uint64 _startTime, uint64 _endTime)
    pure
    returns (OrderConfig c)
{
    assembly ("memory-safe") {
        // Mask inputs to ensure only relevant bits are used
        c := add(
            add(shl(192, _fee), add(shl(184, iszero(iszero(_isToken1))), shl(64, and(_startTime, 0xffffffffffffffff)))),
            and(_endTime, 0xffffffffffffffff)
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Counts is bytes32;

using {index, count, capacity, lastTimestamp} for Counts global;

function index(Counts counts) pure returns (uint32 i) {
    assembly ("memory-safe") {
        i := and(counts, 0xFFFFFFFF)
    }
}

function count(Counts counts) pure returns (uint32 c) {
    assembly ("memory-safe") {
        c := shr(224, shl(192, counts))
    }
}

function capacity(Counts counts) pure returns (uint32 c) {
    assembly ("memory-safe") {
        c := shr(224, shl(160, counts))
    }
}

function lastTimestamp(Counts counts) pure returns (uint32 t) {
    assembly ("memory-safe") {
        t := shr(224, shl(128, counts))
    }
}

function createCounts(uint32 _index, uint32 _count, uint32 _capacity, uint32 _lastTimestamp) pure returns (Counts c) {
    assembly ("memory-safe") {
        // c = index | (count << 32) | (capacity << 64) | (lastTimestamp << 96)
        c := or(
            or(or(and(_index, 0xFFFFFFFF), shl(32, and(_count, 0xFFFFFFFF))), shl(64, and(_capacity, 0xFFFFFFFF))),
            shl(96, and(_lastTimestamp, 0xFFFFFFFF))
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Packed representation of time-specific order information
/// @dev Bit layout (256 bits total):
///      - bits 255-224: numOrders (uint32)
///      - bits 223-112: saleRateDeltaToken0 (int112)
///      - bits 111-0:   saleRateDeltaToken1 (int112)
type TimeInfo is bytes32;

using {numOrders, saleRateDeltaToken0, saleRateDeltaToken1, parse} for TimeInfo global;

function numOrders(TimeInfo info) pure returns (uint32 n) {
    assembly ("memory-safe") {
        n := shr(224, info)
    }
}

function saleRateDeltaToken0(TimeInfo info) pure returns (int112 delta) {
    assembly ("memory-safe") {
        delta := signextend(13, shr(112, info))
    }
}

function saleRateDeltaToken1(TimeInfo info) pure returns (int112 delta) {
    assembly ("memory-safe") {
        delta := signextend(13, info)
    }
}

function parse(TimeInfo info) pure returns (uint32 n, int112 delta0, int112 delta1) {
    assembly ("memory-safe") {
        n := shr(224, info)
        delta0 := signextend(13, shr(112, info))
        delta1 := signextend(13, info)
    }
}

function createTimeInfo(uint32 _numOrders, int112 _saleRateDeltaToken0, int112 _saleRateDeltaToken1)
    pure
    returns (TimeInfo info)
{
    assembly ("memory-safe") {
        // info = (numOrders << 224) | ((saleRateDeltaToken0 & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF) << 112) | (saleRateDeltaToken1 & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
        info := or(
            shl(224, _numOrders),
            or(
                shl(112, and(_saleRateDeltaToken0, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                and(_saleRateDeltaToken1, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
            )
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @dev Computes 2^x where x is a 5.64 number and result is a 64.64 number
function exp2(uint256 x) pure returns (uint256 result) {
    unchecked {
        require(x < 0x400000000000000000); // Overflow

        result = 0x80000000000000000000000000000000;

        if (x & 0x8000000000000000 != 0) {
            result = result * 0x16A09E667F3BCC908B2FB1366EA957D3E >> 128;
        }
        if (x & 0x4000000000000000 != 0) {
            result = result * 0x1306FE0A31B7152DE8D5A46305C85EDEC >> 128;
        }
        if (x & 0x2000000000000000 != 0) {
            result = result * 0x1172B83C7D517ADCDF7C8C50EB14A791F >> 128;
        }
        if (x & 0x1000000000000000 != 0) {
            result = result * 0x10B5586CF9890F6298B92B71842A98363 >> 128;
        }
        if (x & 0x800000000000000 != 0) {
            result = result * 0x1059B0D31585743AE7C548EB68CA417FD >> 128;
        }
        if (x & 0x400000000000000 != 0) {
            result = result * 0x102C9A3E778060EE6F7CACA4F7A29BDE8 >> 128;
        }
        if (x & 0x200000000000000 != 0) {
            result = result * 0x10163DA9FB33356D84A66AE336DCDFA3F >> 128;
        }
        if (x & 0x100000000000000 != 0) {
            result = result * 0x100B1AFA5ABCBED6129AB13EC11DC9543 >> 128;
        }
        if (x & 0x80000000000000 != 0) {
            result = result * 0x10058C86DA1C09EA1FF19D294CF2F679B >> 128;
        }
        if (x & 0x40000000000000 != 0) {
            result = result * 0x1002C605E2E8CEC506D21BFC89A23A00F >> 128;
        }
        if (x & 0x20000000000000 != 0) {
            result = result * 0x100162F3904051FA128BCA9C55C31E5DF >> 128;
        }
        if (x & 0x10000000000000 != 0) {
            result = result * 0x1000B175EFFDC76BA38E31671CA939725 >> 128;
        }
        if (x & 0x8000000000000 != 0) {
            result = result * 0x100058BA01FB9F96D6CACD4B180917C3D >> 128;
        }
        if (x & 0x4000000000000 != 0) {
            result = result * 0x10002C5CC37DA9491D0985C348C68E7B3 >> 128;
        }
        if (x & 0x2000000000000 != 0) {
            result = result * 0x1000162E525EE054754457D5995292026 >> 128;
        }
        if (x & 0x1000000000000 != 0) {
            result = result * 0x10000B17255775C040618BF4A4ADE83FC >> 128;
        }
        if (x & 0x800000000000 != 0) {
            result = result * 0x1000058B91B5BC9AE2EED81E9B7D4CFAB >> 128;
        }
        if (x & 0x400000000000 != 0) {
            result = result * 0x100002C5C89D5EC6CA4D7C8ACC017B7C9 >> 128;
        }
        if (x & 0x200000000000 != 0) {
            result = result * 0x10000162E43F4F831060E02D839A9D16D >> 128;
        }
        if (x & 0x100000000000 != 0) {
            result = result * 0x100000B1721BCFC99D9F890EA06911763 >> 128;
        }
        if (x & 0x80000000000 != 0) {
            result = result * 0x10000058B90CF1E6D97F9CA14DBCC1628 >> 128;
        }
        if (x & 0x40000000000 != 0) {
            result = result * 0x1000002C5C863B73F016468F6BAC5CA2B >> 128;
        }
        if (x & 0x20000000000 != 0) {
            result = result * 0x100000162E430E5A18F6119E3C02282A5 >> 128;
        }
        if (x & 0x10000000000 != 0) {
            result = result * 0x1000000B1721835514B86E6D96EFD1BFE >> 128;
        }
        if (x & 0x8000000000 != 0) {
            result = result * 0x100000058B90C0B48C6BE5DF846C5B2EF >> 128;
        }
        if (x & 0x4000000000 != 0) {
            result = result * 0x10000002C5C8601CC6B9E94213C72737A >> 128;
        }
        if (x & 0x2000000000 != 0) {
            result = result * 0x1000000162E42FFF037DF38AA2B219F06 >> 128;
        }
        if (x & 0x1000000000 != 0) {
            result = result * 0x10000000B17217FBA9C739AA5819F44F9 >> 128;
        }
        if (x & 0x800000000 != 0) {
            result = result * 0x1000000058B90BFCDEE5ACD3C1CEDC823 >> 128;
        }
        if (x & 0x400000000 != 0) {
            result = result * 0x100000002C5C85FE31F35A6A30DA1BE50 >> 128;
        }
        if (x & 0x200000000 != 0) {
            result = result * 0x10000000162E42FF0999CE3541B9FFFCF >> 128;
        }
        if (x & 0x100000000 != 0) {
            result = result * 0x100000000B17217F80F4EF5AADDA45554 >> 128;
        }
        if (x & 0x80000000 != 0) {
            result = result * 0x10000000058B90BFBF8479BD5A81B51AD >> 128;
        }
        if (x & 0x40000000 != 0) {
            result = result * 0x1000000002C5C85FDF84BD62AE30A74CC >> 128;
        }
        if (x & 0x20000000 != 0) {
            result = result * 0x100000000162E42FEFB2FED257559BDAA >> 128;
        }
        if (x & 0x10000000 != 0) {
            result = result * 0x1000000000B17217F7D5A7716BBA4A9AE >> 128;
        }
        if (x & 0x8000000 != 0) {
            result = result * 0x100000000058B90BFBE9DDBAC5E109CCE >> 128;
        }
        if (x & 0x4000000 != 0) {
            result = result * 0x10000000002C5C85FDF4B15DE6F17EB0D >> 128;
        }
        if (x & 0x2000000 != 0) {
            result = result * 0x1000000000162E42FEFA494F1478FDE05 >> 128;
        }
        if (x & 0x1000000 != 0) {
            result = result * 0x10000000000B17217F7D20CF927C8E94C >> 128;
        }
        if (x & 0x800000 != 0) {
            result = result * 0x1000000000058B90BFBE8F71CB4E4B33D >> 128;
        }
        if (x & 0x400000 != 0) {
            result = result * 0x100000000002C5C85FDF477B662B26945 >> 128;
        }
        if (x & 0x200000 != 0) {
            result = result * 0x10000000000162E42FEFA3AE53369388C >> 128;
        }
        if (x & 0x100000 != 0) {
            result = result * 0x100000000000B17217F7D1D351A389D40 >> 128;
        }
        if (x & 0x80000 != 0) {
            result = result * 0x10000000000058B90BFBE8E8B2D3D4EDE >> 128;
        }
        if (x & 0x40000 != 0) {
            result = result * 0x1000000000002C5C85FDF4741BEA6E77E >> 128;
        }
        if (x & 0x20000 != 0) {
            result = result * 0x100000000000162E42FEFA39FE95583C2 >> 128;
        }
        if (x & 0x10000 != 0) {
            result = result * 0x1000000000000B17217F7D1CFB72B45E1 >> 128;
        }
        if (x & 0x8000 != 0) {
            result = result * 0x100000000000058B90BFBE8E7CC35C3F0 >> 128;
        }
        if (x & 0x4000 != 0) {
            result = result * 0x10000000000002C5C85FDF473E242EA38 >> 128;
        }
        if (x & 0x2000 != 0) {
            result = result * 0x1000000000000162E42FEFA39F02B772C >> 128;
        }
        if (x & 0x1000 != 0) {
            result = result * 0x10000000000000B17217F7D1CF7D83C1A >> 128;
        }
        if (x & 0x800 != 0) {
            result = result * 0x1000000000000058B90BFBE8E7BDCBE2E >> 128;
        }
        if (x & 0x400 != 0) {
            result = result * 0x100000000000002C5C85FDF473DEA871F >> 128;
        }
        if (x & 0x200 != 0) {
            result = result * 0x10000000000000162E42FEFA39EF44D91 >> 128;
        }
        if (x & 0x100 != 0) {
            result = result * 0x100000000000000B17217F7D1CF79E949 >> 128;
        }
        if (x & 0x80 != 0) {
            result = result * 0x10000000000000058B90BFBE8E7BCE544 >> 128;
        }
        if (x & 0x40 != 0) {
            result = result * 0x1000000000000002C5C85FDF473DE6ECA >> 128;
        }
        if (x & 0x20 != 0) {
            result = result * 0x100000000000000162E42FEFA39EF366F >> 128;
        }
        if (x & 0x10 != 0) {
            result = result * 0x1000000000000000B17217F7D1CF79AFA >> 128;
        }
        if (x & 0x8 != 0) {
            result = result * 0x100000000000000058B90BFBE8E7BCD6D >> 128;
        }
        if (x & 0x4 != 0) {
            result = result * 0x10000000000000002C5C85FDF473DE6B2 >> 128;
        }
        if (x & 0x2 != 0) {
            result = result * 0x1000000000000000162E42FEFA39EF358 >> 128;
        }
        if (x & 0x1 != 0) {
            result = result * 0x10000000000000000B17217F7D1CF79AB >> 128;
        }

        result >>= uint256(63 - (x >> 64));
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// The total fees per liquidity for each token.
// Since these are always read together we put them in a struct, even though they cannot be packed.
struct FeesPerLiquidity {
    uint256 value0;
    uint256 value1;
}

using {sub} for FeesPerLiquidity global;

function sub(FeesPerLiquidity memory a, FeesPerLiquidity memory b) pure returns (FeesPerLiquidity memory result) {
    assembly ("memory-safe") {
        mstore(result, sub(mload(a), mload(b)))
        mstore(add(result, 32), sub(mload(add(a, 32)), mload(add(b, 32))))
    }
}

function feesPerLiquidityFromAmounts(uint128 amount0, uint128 amount1, uint128 liquidity)
    pure
    returns (FeesPerLiquidity memory result)
{
    assembly ("memory-safe") {
        mstore(result, div(shl(128, amount0), liquidity))
        mstore(add(result, 32), div(shl(128, amount1), liquidity))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice A drop is specified by an owner, token and a root
/// @dev The owner can reclaim the drop token at any time
///      The root is the root of a merkle trie that contains all the incentives to be distributed
struct DropKey {
    /// @notice Address that owns the drop and can reclaim tokens
    address owner;
    /// @notice Token address for the drop
    address token;
    /// @notice Merkle root of the incentive distribution tree
    bytes32 root;
}

using {toDropId} for DropKey global;

/// @notice Returns the identifier of the drop
/// @param key The drop key to hash
/// @return h The unique drop identifier
function toDropId(DropKey memory key) pure returns (bytes32 h) {
    assembly ("memory-safe") {
        // assumes that owner, token have no dirty upper bits
        h := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Locker is bytes32;

using {id, addr, parse} for Locker global;

function id(Locker locker) pure returns (uint256 v) {
    assembly ("memory-safe") {
        v := sub(shr(160, locker), 1)
    }
}

function addr(Locker locker) pure returns (address v) {
    assembly ("memory-safe") {
        v := shr(96, shl(96, locker))
    }
}

function parse(Locker locker) pure returns (uint256 lockerId, address lockerAddr) {
    assembly ("memory-safe") {
        lockerId := sub(shr(160, locker), 1)
        lockerAddr := shr(96, shl(96, locker))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Snapshot is bytes32;

using {timestamp, secondsPerLiquidityCumulative, tickCumulative} for Snapshot global;

function timestamp(Snapshot snapshot) pure returns (uint32 t) {
    assembly ("memory-safe") {
        t := and(snapshot, 0xFFFFFFFF)
    }
}

function secondsPerLiquidityCumulative(Snapshot snapshot) pure returns (uint160 s) {
    assembly ("memory-safe") {
        s := and(shr(32, snapshot), 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
    }
}

function tickCumulative(Snapshot snapshot) pure returns (int64 t) {
    assembly ("memory-safe") {
        t := signextend(7, shr(192, snapshot))
    }
}

function createSnapshot(uint32 _timestamp, uint160 _secondsPerLiquidityCumulative, int64 _tickCumulative)
    pure
    returns (Snapshot s)
{
    assembly ("memory-safe") {
        // s = timestamp | (secondsPerLiquidityCumulative << 32) | (tickCumulative << 192)
        s := or(
            or(
                and(_timestamp, 0xFFFFFFFF),
                shl(32, and(_secondsPerLiquidityCumulative, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
            ),
            shl(192, and(_tickCumulative, 0xFFFFFFFFFFFFFFFF))
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type StorageSlot is bytes32;

using {load, loadTwo, store, storeTwo, next, add, sub} for StorageSlot global;

function load(StorageSlot slot) view returns (bytes32 value) {
    assembly ("memory-safe") {
        value := sload(slot)
    }
}

function loadTwo(StorageSlot slot) view returns (bytes32 value0, bytes32 value1) {
    value0 = slot.load();
    value1 = slot.next().load();
}

function store(StorageSlot slot, bytes32 value) {
    assembly ("memory-safe") {
        sstore(slot, value)
    }
}

function storeTwo(StorageSlot slot, bytes32 value0, bytes32 value1) {
    slot.store(value0);
    slot.next().store(value1);
}

function next(StorageSlot slot) pure returns (StorageSlot nextSlot) {
    assembly ("memory-safe") {
        nextSlot := add(slot, 1)
    }
}

function add(StorageSlot slot, uint256 addend) pure returns (StorageSlot summedSlot) {
    assembly ("memory-safe") {
        summedSlot := add(slot, addend)
    }
}

function sub(StorageSlot slot, uint256 subtrahend) pure returns (StorageSlot differenceSlot) {
    assembly ("memory-safe") {
        differenceSlot := sub(slot, subtrahend)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type PoolBalanceUpdate is bytes32;

using {delta0, delta1} for PoolBalanceUpdate global;

function delta0(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, shr(128, update))
    }
}

function delta1(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, update)
    }
}

function createPoolBalanceUpdate(int128 _delta0, int128 _delta1) pure returns (PoolBalanceUpdate update) {
    assembly ("memory-safe") {
        // update = (delta0 << 128) | delta1
        update := or(shl(128, _delta0), and(_delta1, 0xffffffffffffffffffffffffffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type TickInfo is bytes32;

using {liquidityDelta, liquidityNet, parse} for TickInfo global;

function liquidityDelta(TickInfo info) pure returns (int128 delta) {
    assembly ("memory-safe") {
        delta := signextend(15, info)
    }
}

function liquidityNet(TickInfo info) pure returns (uint128 net) {
    assembly ("memory-safe") {
        net := shr(128, info)
    }
}

function parse(TickInfo info) pure returns (int128 delta, uint128 net) {
    assembly ("memory-safe") {
        delta := signextend(15, info)
        net := shr(128, info)
    }
}

function createTickInfo(int128 _liquidityDelta, uint128 _liquidityNet) pure returns (TickInfo info) {
    assembly ("memory-safe") {
        // info = (liquidityNet << 128) | liquidityDelta
        info := or(shl(128, _liquidityNet), shr(128, shl(128, _liquidityDelta)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type StorageSlot is bytes32;

using {load, loadTwo, store, storeTwo, next, add, sub} for StorageSlot global;

function load(StorageSlot slot) view returns (bytes32 value) {
    assembly ("memory-safe") {
        value := sload(slot)
    }
}

function loadTwo(StorageSlot slot) view returns (bytes32 value0, bytes32 value1) {
    value0 = slot.load();
    value1 = slot.next().load();
}

function store(StorageSlot slot, bytes32 value) {
    assembly ("memory-safe") {
        sstore(slot, value)
    }
}

function storeTwo(StorageSlot slot, bytes32 value0, bytes32 value1) {
    slot.store(value0);
    slot.next().store(value1);
}

function next(StorageSlot slot) pure returns (StorageSlot nextSlot) {
    assembly ("memory-safe") {
        nextSlot := add(slot, 1)
    }
}

function add(StorageSlot slot, uint256 addend) pure returns (StorageSlot summedSlot) {
    assembly ("memory-safe") {
        summedSlot := add(slot, addend)
    }
}

function sub(StorageSlot slot, uint256 subtrahend) pure returns (StorageSlot differenceSlot) {
    assembly ("memory-safe") {
        differenceSlot := sub(slot, subtrahend)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MAX_TICK_MAGNITUDE} from "./constants.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {LibBit} from "solady/utils/LibBit.sol";
import {SqrtRatio, toSqrtRatio} from "../types/sqrtRatio.sol";

// Tick Math Library
// Contains functions for converting between ticks and sqrt price ratios
// Ticks represent discrete price points, while sqrt ratios represent the actual prices
// The relationship is: sqrtRatio = sqrt(1.000001^tick)

/// @notice Thrown when a tick value is outside the valid range
/// @param tick The invalid tick value
error InvalidTick(int32 tick);

/// @notice Converts a tick to its corresponding sqrt price ratio
/// @dev Uses bit manipulation and precomputed constants for gas efficiency
/// @param tick The tick to convert (must be within MIN_TICK and MAX_TICK)
/// @return r The sqrt price ratio corresponding to the tick
function tickToSqrtRatio(int32 tick) pure returns (SqrtRatio r) {
    unchecked {
        uint256 t = FixedPointMathLib.abs(tick);
        if (t > MAX_TICK_MAGNITUDE) revert InvalidTick(tick);

        uint256 ratio;
        assembly ("memory-safe") {
            // bit 0 is handled with a single conditional subtract from 2^128
            ratio := sub(0x100000000000000000000000000000000, mul(and(t, 0x1), 0x8637b66cd638344daef276cd7c5))

            // -------- Gate 1: bits 1..7 (mask 0xFE) --------
            if and(t, 0xFE) {
                if and(t, 0x2) { ratio := shr(128, mul(ratio, 0xffffef390978c398134b4ff3764fe410)) }
                if and(t, 0x4) { ratio := shr(128, mul(ratio, 0xffffde72140b00a354bd3dc828e976c9)) }
                if and(t, 0x8) { ratio := shr(128, mul(ratio, 0xffffbce42c7be6c998ad6318193c0b18)) }
                if and(t, 0x10) { ratio := shr(128, mul(ratio, 0xffff79c86a8f6150a32d9778eceef97c)) }
                if and(t, 0x20) { ratio := shr(128, mul(ratio, 0xfffef3911b7cff24ba1b3dbb5f8f5974)) }
                if and(t, 0x40) { ratio := shr(128, mul(ratio, 0xfffde72350725cc4ea8feece3b5f13c8)) }
                if and(t, 0x80) { ratio := shr(128, mul(ratio, 0xfffbce4b06c196e9247ac87695d53c60)) }
            }

            // -------- Gate 2: bits 8..14 (mask 0x7F00) --------
            if and(t, 0x7F00) {
                if and(t, 0x100) { ratio := shr(128, mul(ratio, 0xfff79ca7a4d1bf1ee8556cea23cdbaa5)) }
                if and(t, 0x200) { ratio := shr(128, mul(ratio, 0xffef3995a5b6a6267530f207142a5764)) }
                if and(t, 0x400) { ratio := shr(128, mul(ratio, 0xffde7444b28145508125d10077ba83b8)) }
                if and(t, 0x800) { ratio := shr(128, mul(ratio, 0xffbceceeb791747f10df216f2e53ec57)) }
                if and(t, 0x1000) { ratio := shr(128, mul(ratio, 0xff79eb706b9a64c6431d76e63531e929)) }
                if and(t, 0x2000) { ratio := shr(128, mul(ratio, 0xfef41d1a5f2ae3a20676bec6f7f9459a)) }
                if and(t, 0x4000) { ratio := shr(128, mul(ratio, 0xfde95287d26d81bea159c37073122c73)) }
            }

            // -------- Gate 3: bits 15..20 (mask 0x1F8000) --------
            if and(t, 0x1F8000) {
                if and(t, 0x8000) { ratio := shr(128, mul(ratio, 0xfbd701c7cbc4c8a6bb81efd232d1e4e7)) }
                if and(t, 0x10000) { ratio := shr(128, mul(ratio, 0xf7bf5211c72f5185f372aeb1d48f937e)) }
                if and(t, 0x20000) { ratio := shr(128, mul(ratio, 0xefc2bf59df33ecc28125cf78ec4f167f)) }
                if and(t, 0x40000) { ratio := shr(128, mul(ratio, 0xe08d35706200796273f0b3a981d90cfd)) }
                if and(t, 0x80000) { ratio := shr(128, mul(ratio, 0xc4f76b68947482dc198a48a54348c4ed)) }
                if and(t, 0x100000) { ratio := shr(128, mul(ratio, 0x978bcb9894317807e5fa4498eee7c0fa)) }
            }

            // -------- Gate 4: bits 21..26 (mask 0x7E00000) --------
            if and(t, 0x7E00000) {
                if and(t, 0x200000) { ratio := shr(128, mul(ratio, 0x59b63684b86e9f486ec54727371ba6ca)) }
                if and(t, 0x400000) { ratio := shr(128, mul(ratio, 0x1f703399d88f6aa83a28b22d4a1f56e3)) }
                if and(t, 0x800000) { ratio := shr(128, mul(ratio, 0x3dc5dac7376e20fc8679758d1bcdcfc)) }
                if and(t, 0x1000000) { ratio := shr(128, mul(ratio, 0xee7e32d61fdb0a5e622b820f681d0)) }
                if and(t, 0x2000000) { ratio := shr(128, mul(ratio, 0xde2ee4bc381afa7089aa84bb66)) }
                if and(t, 0x4000000) { ratio := shr(128, mul(ratio, 0xc0d55d4d7152c25fb139)) }
            }

            // If original tick > 0, invert: ratio = maxUint / ratio
            if sgt(tick, 0) { ratio := div(not(0), ratio) }
        }

        r = toSqrtRatio(ratio, false);
    }
}

uint256 constant ONE_Q127 = 1 << 127;

// Convert ln(m) series to log2(m):  log2(m) = (2 / ln 2) * s.
// Precompute K = round((2 / ln 2) * 2^64) as a uint (Q64 scalar).
// K = 53226052391377289966  (≈ 0x2e2a8eca5705fc2ee)
uint256 constant K_2_OVER_LN2_X64 = 53226052391377289966;

// 2^64 / log2(sqrt(1.000001)) for converting from log base 2 in X64 to log base tick
int256 constant INV_LB_X64 = 25572630076711825471857579;

// Error bounds of the tick computation based on the number of iterations ~= +-0.002 ticks
int256 constant ERROR_BOUNDS_X128 = int256((uint256(1) << 128) / 485);

/// @notice Converts a sqrt price ratio to its corresponding tick
/// @dev Computes log2 via one normalization + atanh series (no per-bit squaring loop)
/// @param sqrtRatio The valid sqrt price ratio to convert
/// @return tick The tick corresponding to the sqrt ratio
function sqrtRatioToTick(SqrtRatio sqrtRatio) pure returns (int32 tick) {
    unchecked {
        uint256 sqrtRatioFixed = sqrtRatio.toFixed();

        // Normalize sign via reciprocal if < 1. Keep this branch-free.
        bool negative;
        uint256 x;
        uint256 hi;
        assembly ("memory-safe") {
            negative := iszero(shr(128, sqrtRatioFixed))
            // x = negative ? (type(uint256).max / R) : R
            x := add(div(sub(0, negative), sqrtRatioFixed), mul(iszero(negative), sqrtRatioFixed))
            // We know (x >> 128) != 0 because we reciprocated sqrtRatioFixed
            hi := shr(128, x)
        }

        // Integer part of log2 via CLZ: floor(log2(hi)) = 255 - clz(hi)
        uint256 msbHigh;
        assembly ("memory-safe") {
            msbHigh := sub(255, clz(hi))
        }

        // Reduce once so X ∈ [2^127, 2^128)  (Q1.127 mantissa)
        x = x >> (msbHigh + 1);

        // Fractional log2 using atanh on y = (m-1)/(m+1), m = X/2^127 ∈ [1,2)
        uint256 a = x - ONE_Q127; // (m - 1) * 2^127
        uint256 b = x + ONE_Q127; // (m + 1) * 2^127
        uint256 yQ = FixedPointMathLib.rawDiv(a << 127, b); // y in Q1.127

        // Build odd powers via y^2 ladder
        uint256 y2 = (yQ * yQ) >> 127; // y^2
        uint256 y3 = (yQ * y2) >> 127; // y^3
        uint256 y5 = (y3 * y2) >> 127; // y^5
        uint256 y7 = (y5 * y2) >> 127; // y^7
        uint256 y9 = (y7 * y2) >> 127; // y^9
        uint256 y11 = (y9 * y2) >> 127; // y^11
        uint256 y13 = (y11 * y2) >> 127; // y^13
        uint256 y15 = (y13 * y2) >> 127; // y^15

        // s = y + y^3/3 + y^5/5 + ... + y^15/15  (Q1.127)
        uint256 s = yQ + (y3 / 3) + (y5 / 5) + (y7 / 7) + (y9 / 9) + (y11 / 11) + (y13 / 13) + (y15 / 15);

        // fracX64 = ((2/ln2) * s) in Q64.64  =>  (s * K) >> 127
        uint256 fracX64 = (s * K_2_OVER_LN2_X64) >> 127;

        // Unsigned log2 in Q64.64
        uint256 log2Unsigned = (msbHigh << 64) + fracX64;

        // Map log2 to tick-space X128
        int256 base = negative ? -int256(log2Unsigned) : int256(log2Unsigned);

        int256 logBaseTickSizeX128 = base * INV_LB_X64;

        // Add error bounds to the computed logarithm
        int32 tickLow = int32((logBaseTickSizeX128 - ERROR_BOUNDS_X128) >> 128);
        tick = int32((logBaseTickSizeX128 + ERROR_BOUNDS_X128) >> 128);

        if (tick != tickLow) {
            // tickHigh overshoots
            if (tickToSqrtRatio(tick) > sqrtRatio) {
                tick = tickLow;
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice A claim is an individual leaf in the merkle trie
struct ClaimKey {
    /// @notice Index of the claim in the merkle tree
    uint256 index;
    /// @notice Account that can claim the incentive
    address account;
    /// @notice Amount of tokens to be claimed
    uint128 amount;
}

using {toClaimId} for ClaimKey global;

/// @notice Hashes a claim for merkle proof verification
/// @param c The claim to hash
/// @return h The hash of the claim
function toClaimId(ClaimKey memory c) pure returns (bytes32 h) {
    assembly ("memory-safe") {
        // assumes that account has no dirty upper bits
        h := keccak256(c, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio} from "./sqrtRatio.sol";

type PoolState is bytes32;

using {sqrtRatio, tick, liquidity, isInitialized, parse} for PoolState global;

function sqrtRatio(PoolState state) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, state)
    }
}

function tick(PoolState state) pure returns (int32 t) {
    assembly ("memory-safe") {
        t := signextend(3, shr(128, state))
    }
}

function liquidity(PoolState state) pure returns (uint128 l) {
    assembly ("memory-safe") {
        l := shr(128, shl(128, state))
    }
}

function isInitialized(PoolState state) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := iszero(iszero(state))
    }
}

function parse(PoolState state) pure returns (SqrtRatio r, int32 t, uint128 l) {
    assembly ("memory-safe") {
        r := shr(160, state)
        t := signextend(3, shr(128, state))
        l := shr(128, shl(128, state))
    }
}

function createPoolState(SqrtRatio _sqrtRatio, int32 _tick, uint128 _liquidity) pure returns (PoolState s) {
    assembly ("memory-safe") {
        // s = (sqrtRatio << 160) | (_tick << 128) | liquidity
        s := or(shl(160, _sqrtRatio), or(shl(128, and(_tick, 0xFFFFFFFF)), shr(128, shl(128, _liquidity))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "./poolKey.sol";
import {OrderConfig} from "./orderConfig.sol";
import {OrderId} from "./orderId.sol";

using {toOrderId, toPoolKey, buyToken, sellToken} for OrderKey global;

/// @notice Order key structure identifying a TWAMM order
/// @dev Contains all parameters needed to uniquely identify an order
struct OrderKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing fee, isToken1, start, and end time
    OrderConfig config;
}

/// @notice Extracts the buy token from an order key
/// @param ok The order key
/// @return r The buy token
function buyToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken0 = !ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken0, 0x20)))
    }
}

/// @notice Extracts the sell token from an order key
/// @param ok The order key
/// @return r The sell token
function sellToken(OrderKey memory ok) pure returns (address r) {
    bool _isToken1 = ok.config.isToken1();
    assembly ("memory-safe") {
        r := mload(add(ok, mul(_isToken1, 0x20)))
    }
}

/// @notice Computes the order ID from an order key
/// @param orderKey The order key
/// @return id The computed order ID
function toOrderId(OrderKey memory orderKey) pure returns (OrderId id) {
    assembly ("memory-safe") {
        id := keccak256(orderKey, 96)
    }
}

/// @notice Converts an OrderKey to its corresponding PoolKey
/// @dev Determines the correct token ordering and constructs the pool key with TWAMM as extension
/// @param orderKey The order key containing sell/buy tokens and fee
/// @param twamm The TWAMM contract address to use as the extension
/// @return poolKey The corresponding pool key for the order
function toPoolKey(OrderKey memory orderKey, address twamm) pure returns (PoolKey memory poolKey) {
    uint64 _fee = orderKey.config.fee();
    assembly ("memory-safe") {
        mcopy(poolKey, orderKey, 64)
        mstore(add(poolKey, 64), add(shl(96, twamm), shl(32, _fee)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

using {funded, claimed, setFunded, setClaimed, getRemaining} for DropState global;

/// @notice Represents the state of a drop with funded and claimed amounts
/// @dev Packed into a single bytes32 slot: funded (128 bits) + claimed (128 bits)
type DropState is bytes32;

/// @notice Gets the funded amount from a drop state
/// @param state The drop state
/// @return amount The funded amount
function funded(DropState state) pure returns (uint128 amount) {
    assembly ("memory-safe") {
        amount := shr(128, state)
    }
}

/// @notice Gets the claimed amount from a drop state
/// @param state The drop state
/// @return amount The claimed amount
function claimed(DropState state) pure returns (uint128 amount) {
    assembly ("memory-safe") {
        amount := and(state, 0xffffffffffffffffffffffffffffffff)
    }
}

/// @notice Sets the funded amount in a drop state
/// @param state The drop state
/// @param amount The funded amount to set
/// @return newState The updated drop state
function setFunded(DropState state, uint128 amount) pure returns (DropState newState) {
    assembly ("memory-safe") {
        newState := or(and(state, 0xffffffffffffffffffffffffffffffff), shl(128, amount))
    }
}

/// @notice Sets the claimed amount in a drop state
/// @param state The drop state
/// @param amount The claimed amount to set
/// @return newState The updated drop state
function setClaimed(DropState state, uint128 amount) pure returns (DropState newState) {
    assembly ("memory-safe") {
        newState := or(and(state, 0xffffffffffffffffffffffffffffffff00000000000000000000000000000000), amount)
    }
}

/// @notice Gets the remaining amount (funded - claimed) from a drop state
/// @param state The drop state
/// @return remaining The remaining amount available for claims
function getRemaining(DropState state) pure returns (uint128 remaining) {
    unchecked {
        remaining = state.funded() - state.claimed();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type StorageSlot is bytes32;

using {load, loadTwo, store, storeTwo, next, add, sub} for StorageSlot global;

function load(StorageSlot slot) view returns (bytes32 value) {
    assembly ("memory-safe") {
        value := sload(slot)
    }
}

function loadTwo(StorageSlot slot) view returns (bytes32 value0, bytes32 value1) {
    value0 = slot.load();
    value1 = slot.next().load();
}

function store(StorageSlot slot, bytes32 value) {
    assembly ("memory-safe") {
        sstore(slot, value)
    }
}

function storeTwo(StorageSlot slot, bytes32 value0, bytes32 value1) {
    slot.store(value0);
    slot.next().store(value1);
}

function next(StorageSlot slot) pure returns (StorageSlot nextSlot) {
    assembly ("memory-safe") {
        nextSlot := add(slot, 1)
    }
}

function add(StorageSlot slot, uint256 addend) pure returns (StorageSlot summedSlot) {
    assembly ("memory-safe") {
        summedSlot := add(slot, addend)
    }
}

function sub(StorageSlot slot, uint256 subtrahend) pure returns (StorageSlot differenceSlot) {
    assembly ("memory-safe") {
        differenceSlot := sub(slot, subtrahend)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// The total fees per liquidity for each token.
// Since these are always read together we put them in a struct, even though they cannot be packed.
struct FeesPerLiquidity {
    uint256 value0;
    uint256 value1;
}

using {sub} for FeesPerLiquidity global;

function sub(FeesPerLiquidity memory a, FeesPerLiquidity memory b) pure returns (FeesPerLiquidity memory result) {
    assembly ("memory-safe") {
        mstore(result, sub(mload(a), mload(b)))
        mstore(add(result, 32), sub(mload(add(a, 32)), mload(add(b, 32))))
    }
}

function feesPerLiquidityFromAmounts(uint128 amount0, uint128 amount1, uint128 liquidity)
    pure
    returns (FeesPerLiquidity memory result)
{
    assembly ("memory-safe") {
        mstore(result, div(shl(128, amount0), liquidity))
        mstore(add(result, 32), div(shl(128, amount1), liquidity))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

// For any given time `t`, there are up to 91 times that are greater than `t` and valid according to `isTimeValid`
uint256 constant MAX_NUM_VALID_TIMES = 91;

// If we constrain the sale rate delta to this value, then the current sale rate will never overflow
uint256 constant MAX_ABS_VALUE_SALE_RATE_DELTA = type(uint112).max / MAX_NUM_VALID_TIMES;

/// @dev Returns the step size, i.e. the value of which the order end or start time must be a multiple of, based on the current time and the specified time
///      The step size has a minimum of 256 seconds and increases in powers of 16 as the gap to `time` grows.
///      Assumes currentTime < type(uint256).max - 4095
/// @param currentTime The current block timestamp
/// @param time The time for which the step size is being computed, based on how far in the future it is from currentTime
function computeStepSize(uint256 currentTime, uint256 time) pure returns (uint256 stepSize) {
    assembly ("memory-safe") {
        switch gt(time, add(currentTime, 4095))
        case 1 {
            let diff := sub(time, currentTime)

            let msb := sub(255, clz(diff)) // = index of msb

            msb := sub(msb, mod(msb, 4)) // = round down to multiple of 4

            stepSize := shl(msb, 1)
        }
        default { stepSize := 256 }
    }
}

/// @dev Returns true iff the given time is a valid start or end time for a TWAMM order
function isTimeValid(uint256 currentTime, uint256 time) pure returns (bool valid) {
    uint256 stepSize = computeStepSize(currentTime, time);

    assembly ("memory-safe") {
        valid := and(iszero(mod(time, stepSize)), or(lt(time, currentTime), lt(sub(time, currentTime), 0x100000000)))
    }
}

/// @dev Returns the next valid time if there is one, or wraps around to the time 0 if there is not
///      Assumes currentTime is less than type(uint256).max - type(uint32).max
function nextValidTime(uint256 currentTime, uint256 time) pure returns (uint256 nextTime) {
    unchecked {
        uint256 stepSize = computeStepSize(currentTime, time);
        assembly ("memory-safe") {
            nextTime := add(time, stepSize)
            nextTime := sub(nextTime, mod(nextTime, stepSize))
        }

        // only if we didn't overflow
        if (nextTime != 0) {
            uint256 nextStepSize = computeStepSize(currentTime, nextTime);
            if (nextStepSize != stepSize) {
                assembly ("memory-safe") {
                    nextTime := add(time, nextStepSize)
                    nextTime := sub(nextTime, mod(nextTime, nextStepSize))
                }
            }
        }

        nextTime = FixedPointMathLib.ternary(nextTime > currentTime + type(uint32).max, 0, nextTime);
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {Bitmap} from "../types/bitmap.sol";
import {MIN_TICK, MAX_TICK} from "../math/constants.sol";
import {StorageSlot} from "../types/storageSlot.sol";

// Addition of this offset does two things--it centers the 0 tick within a single bitmap regardless of tick spacing,
// and gives us a contiguous range of unsigned integers for all ticks
uint256 constant TICK_BITMAP_STORAGE_OFFSET = 89421695;

// Returns the index of the word and the index _in_ that word which contains the bit representing whether the tick is initialized
// Always rounds the tick down to the nearest multiple of tickSpacing
function tickToBitmapWordAndIndex(int32 tick, uint32 tickSpacing) pure returns (uint256 word, uint256 index) {
    assembly ("memory-safe") {
        let rawIndex := add(sub(sdiv(tick, tickSpacing), slt(smod(tick, tickSpacing), 0)), TICK_BITMAP_STORAGE_OFFSET)
        word := shr(8, rawIndex)
        index := and(rawIndex, 0xff)
    }
}

// Returns the index of the word and the index _in_ that word which contains the bit representing whether the tick is initialized
/// @dev This function is only safe if tickSpacing is between 1 and MAX_TICK_SPACING, and word/index correspond to the results of tickToBitmapWordAndIndex for a tick between MIN_TICK and MAX_TICK
function bitmapWordAndIndexToTick(uint256 word, uint256 index, uint32 tickSpacing) pure returns (int32 tick) {
    assembly ("memory-safe") {
        let rawIndex := add(shl(8, word), index)
        tick := mul(sub(rawIndex, TICK_BITMAP_STORAGE_OFFSET), tickSpacing)
    }
}

function loadBitmap(StorageSlot slot, uint256 word) view returns (Bitmap bitmap) {
    bitmap = Bitmap.wrap(uint256(slot.add(word).load()));
}

// Flips the tick in the bitmap from true to false or vice versa
function flipTick(StorageSlot slot, int32 tick, uint32 tickSpacing) {
    (uint256 word, uint256 index) = tickToBitmapWordAndIndex(tick, tickSpacing);
    StorageSlot wordSlot = slot.add(word);
    wordSlot.store(wordSlot.load() ^ bytes32(1 << index));
}

function findNextInitializedTick(StorageSlot slot, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
    view
    returns (int32 nextTick, bool isInitialized)
{
    unchecked {
        nextTick = fromTick;

        while (true) {
            // convert the given tick to the bitmap position of the next nearest potential initialized tick
            (uint256 word, uint256 index) = tickToBitmapWordAndIndex(nextTick + int32(tickSpacing), tickSpacing);

            Bitmap bitmap = loadBitmap(slot, word);

            // find the index of the previous tick in that word
            uint256 nextIndex = bitmap.geSetBit(uint8(index));

            // if we found one, return it
            if (nextIndex != 0) {
                (nextTick, isInitialized) = (bitmapWordAndIndexToTick(word, nextIndex - 1, tickSpacing), true);
                break;
            }

            // otherwise, return the tick of the most significant bit in the word
            nextTick = bitmapWordAndIndexToTick(word, 255, tickSpacing);

            if (nextTick >= MAX_TICK) {
                nextTick = MAX_TICK;
                break;
            }

            // if we are done searching, stop here
            if (skipAhead == 0) {
                break;
            }

            skipAhead--;
        }
    }
}

function findPrevInitializedTick(StorageSlot slot, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
    view
    returns (int32 prevTick, bool isInitialized)
{
    unchecked {
        prevTick = fromTick;

        while (true) {
            // convert the given tick to its bitmap position
            (uint256 word, uint256 index) = tickToBitmapWordAndIndex(prevTick, tickSpacing);

            Bitmap bitmap = loadBitmap(slot, word);

            // find the index of the previous tick in that word
            uint256 prevIndex = bitmap.leSetBit(uint8(index));

            if (prevIndex != 0) {
                (prevTick, isInitialized) = (bitmapWordAndIndexToTick(word, prevIndex - 1, tickSpacing), true);
                break;
            }

            prevTick = bitmapWordAndIndexToTick(word, 0, tickSpacing);

            if (prevTick <= MIN_TICK) {
                prevTick = MIN_TICK;
                break;
            }

            if (skipAhead == 0) {
                break;
            }

            skipAhead--;
            prevTick--;
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// The total fees per liquidity for each token.
// Since these are always read together we put them in a struct, even though they cannot be packed.
struct FeesPerLiquidity {
    uint256 value0;
    uint256 value1;
}

using {sub} for FeesPerLiquidity global;

function sub(FeesPerLiquidity memory a, FeesPerLiquidity memory b) pure returns (FeesPerLiquidity memory result) {
    assembly ("memory-safe") {
        mstore(result, sub(mload(a), mload(b)))
        mstore(add(result, 32), sub(mload(add(a, 32)), mload(add(b, 32))))
    }
}

function feesPerLiquidityFromAmounts(uint128 amount0, uint128 amount1, uint128 liquidity)
    pure
    returns (FeesPerLiquidity memory result)
{
    assembly ("memory-safe") {
        mstore(result, div(shl(128, amount0), liquidity))
        mstore(add(result, 32), div(shl(128, amount1), liquidity))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, toSqrtRatio} from "../types/sqrtRatio.sol";
import {computeFee} from "./fee.sol";
import {exp2} from "./exp2.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

error SaleRateOverflow();

/// @dev Computes sale rate = (amount << 32) / duration and reverts if the result exceeds type(uint112).max.
/// @dev Assumes duration > 0 and amount <= type(uint224).max.
function computeSaleRate(uint256 amount, uint256 duration) pure returns (uint256 saleRate) {
    assembly ("memory-safe") {
        saleRate := div(shl(32, amount), duration)
        if shr(112, saleRate) {
            // cast sig "SaleRateOverflow()"
            mstore(0, shl(224, 0x83c87460))
            revert(0, 4)
        }
    }
}

error SaleRateDeltaOverflow();

/// @dev Adds the sale rate delta to the saleRate and reverts if the result is greater than type(uint112).max
/// @dev Assumes saleRate <= type(uint112).max and saleRateDelta <= type(int112).max and saleRateDelta >= type(int112).min
function addSaleRateDelta(uint256 saleRate, int256 saleRateDelta) pure returns (uint256 result) {
    assembly ("memory-safe") {
        result := add(saleRate, saleRateDelta)
        // if any of the upper bits are non-zero, revert
        if shr(112, result) {
            // cast sig "SaleRateDeltaOverflow()"
            mstore(0, shl(224, 0xc902643d))
            revert(0, 4)
        }
    }
}

/// @dev Computes amount from sale rate: (saleRate * duration) >> 32, with optional rounding.
/// @dev Assumes the saleRate <= type(uint112).max and duration <= type(uint32).max
function computeAmountFromSaleRate(uint256 saleRate, uint256 duration, bool roundUp) pure returns (uint256 amount) {
    assembly ("memory-safe") {
        amount := shr(32, add(mul(saleRate, duration), mul(0xffffffff, roundUp)))
    }
}

/// @dev Computes reward amount = (rewardRate * saleRate) >> 128.
/// @dev saleRate is assumed to be <= type(uint112).max, thus this function is never expected to overflow
function computeRewardAmount(uint256 rewardRate, uint256 saleRate) pure returns (uint128) {
    return uint128(FixedPointMathLib.fullMulDivN(rewardRate, saleRate, 128));
}

/// @dev Computes the quantity `c = (sqrtSaleRatio - sqrtRatio) / (sqrtSaleRatio + sqrtRatio)` as a signed 64.128 number
/// @dev sqrtRatio is assumed to be between 2**192 and 2**-64, while sqrtSaleRatio values are assumed to be between 2**184 and 2**-72
function computeC(uint256 sqrtRatio, uint256 sqrtSaleRatio) pure returns (int256 c) {
    uint256 unsigned = FixedPointMathLib.fullMulDiv(
        FixedPointMathLib.dist(sqrtRatio, sqrtSaleRatio), (1 << 128), sqrtRatio + sqrtSaleRatio
    );
    assembly ("memory-safe") {
        let sign := sub(shl(1, gt(sqrtSaleRatio, sqrtRatio)), 1)
        c := mul(sign, unsigned)
    }
}

/// @dev Returns a 64.128 number representing the sqrt sale ratio
/// @dev Assumes both saleRateToken0 and saleRateToken1 are nonzero and <= type(uint112).max
function computeSqrtSaleRatio(uint256 saleRateToken0, uint256 saleRateToken1) pure returns (uint256 sqrtSaleRatio) {
    unchecked {
        uint256 saleRatio = FixedPointMathLib.rawDiv(saleRateToken1 << 128, saleRateToken0);

        if (saleRatio <= type(uint128).max) {
            // full precision for small ratios
            sqrtSaleRatio = FixedPointMathLib.sqrt(saleRatio << 128);
        } else if (saleRatio <= type(uint192).max) {
            // we know it only has 192 bits, so we can shift it 64 before rooting to get more precision
            sqrtSaleRatio = FixedPointMathLib.sqrt(saleRatio << 64) << 32;
        } else {
            // we assume it has max 240 bits, since saleRateToken1 is 112 bits and we shifted left 128
            sqrtSaleRatio = FixedPointMathLib.sqrt(saleRatio << 16) << 56;
        }
    }
}

/// @dev Computes the next sqrt ratio according to the state of the TWAMM for a fixed sale rate
/// @dev Assumes both sale rates are != 0 and <= type(uint112).max
/// @dev Assumes liquidity is <= type(uint128).max
/// @dev Assumes timeElapsed is <= type(uint32).max
function computeNextSqrtRatio(
    SqrtRatio sqrtRatio,
    uint256 liquidity,
    uint256 saleRateToken0,
    uint256 saleRateToken1,
    uint256 timeElapsed,
    uint64 fee
) pure returns (SqrtRatio sqrtRatioNext) {
    unchecked {
        // assumed:
        //   assert(saleRateToken0 != 0 && saleRateToken1 != 0);
        uint256 sqrtSaleRatio = computeSqrtSaleRatio(saleRateToken0, saleRateToken1);

        uint256 sqrtRatioFixed = sqrtRatio.toFixed();
        bool roundUp = sqrtRatioFixed > sqrtSaleRatio;

        int256 c = computeC(sqrtRatioFixed, sqrtSaleRatio);

        if (c == 0 || liquidity == 0) {
            // if liquidity is 0, we just settle the ratio of sale rates since the liquidity provides no friction to the price movement
            // if c is 0, that means the difference b/t sale ratio and sqrt ratio is too small to be detected
            // so we just assume it settles at the sale ratio
            sqrtRatioNext = toSqrtRatio(sqrtSaleRatio, roundUp);
        } else {
            uint256 sqrtSaleRateWithoutFee = FixedPointMathLib.sqrt(saleRateToken0 * saleRateToken1);
            // max 112 bits
            uint256 sqrtSaleRate = sqrtSaleRateWithoutFee - computeFee(uint128(sqrtSaleRateWithoutFee), fee);

            // (12392656037 * t * sqrtSaleRate) / liquidity == (34 + 32 + 128) - 128 bits, cannot overflow
            // uint256(12392656037) = Math.floor(Math.LOG2E * 2**33).
            // this combines the doubling, the left shifting and the converting to a base 2 exponent into a single multiplication
            uint256 exponent = FixedPointMathLib.rawDiv(sqrtSaleRate * timeElapsed * 12392656037, liquidity);
            if (exponent >= 0x400000000000000000) {
                // if the exponent is larger than this value (64), the exponent term dominates and the result is approximately the sell ratio
                sqrtRatioNext = toSqrtRatio(sqrtSaleRatio, roundUp);
            } else {
                int256 ePowExponent = int256(uint256(exp2(uint128(exponent))) << 64);

                uint256 sqrtRatioNextFixed = FixedPointMathLib.fullMulDiv(
                    sqrtSaleRatio, FixedPointMathLib.dist(ePowExponent, c), FixedPointMathLib.abs(ePowExponent + c)
                );

                // we should never exceed the sale ratio
                if (roundUp) {
                    sqrtRatioNextFixed = FixedPointMathLib.max(sqrtRatioNextFixed, sqrtSaleRatio);
                } else {
                    sqrtRatioNextFixed = FixedPointMathLib.min(sqrtRatioNextFixed, sqrtSaleRatio);
                }

                sqrtRatioNext = toSqrtRatio(sqrtRatioNextFixed, roundUp);
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK, MAX_TICK_SPACING} from "../math/constants.sol";

/// @notice Pool configuration packed into a single bytes32
/// @dev Contains extension address (20 bytes), fee (8 bytes), and pool type config (4 bytes)
/// Pool type config (32 bits):
///   - Bit 31: discriminator (1 = concentrated, 0 = stableswap)
///   - For concentrated (bit 31 = 1): bits 30-0 are tick spacing
///   - For stableswap (bit 31 = 0): bits 30-24 are amplification factor, bits 23-0 are center tick
type PoolConfig is bytes32;

using {
    fee,
    extension,
    isConcentrated,
    isStableswap,
    isFullRange,
    concentratedTickSpacing,
    stableswapAmplification,
    stableswapCenterTick,
    stableswapActiveLiquidityTickRange,
    stableswapLiquidityWidth,
    validate,
    concentratedMaxLiquidityPerTick
} for PoolConfig global;

/// @notice Extracts the fee from a pool config
/// @param config The pool config
/// @return r The fee
function fee(PoolConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(32, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the extension address from a pool config
/// @param config The pool config
/// @return r The extension address
function extension(PoolConfig config) pure returns (address r) {
    assembly ("memory-safe") {
        r := shr(96, config)
    }
}

/// @notice Checks if this is a concentrated liquidity pool
/// @param config The pool config
/// @return r True if the pool is concentrated liquidity
function isConcentrated(PoolConfig config) pure returns (bool r) {
    r = !config.isStableswap();
}

/// @notice Extracts the tick spacing from a concentrated liquidity pool config
/// @dev Only valid for concentrated liquidity pools (isConcentrated() == true)
/// @param config The pool config
/// @return r The tick spacing
function concentratedTickSpacing(PoolConfig config) pure returns (uint32 r) {
    assembly ("memory-safe") {
        // Extract lower 31 bits (bits 30-0)
        r := and(config, 0x7fffffff)
    }
}

/// @notice Checks if this is a stableswap pool
/// @param config The pool config
/// @return r True if the pool is stableswap
function isStableswap(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // = iff bit 31 is not set
        r := iszero(and(0x80000000, config))
    }
}

/// @notice Determines if this pool is full range (special case of stableswap with amplification=0 and center=0)
/// @dev Full range can be slightly optimized in that we don't need to compute the sqrt ratio at the tick boundaries
/// @param config The pool config
/// @return r True if the pool is full range
function isFullRange(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // Full range when all 32 bits are 0 (discriminator=0, amplification=0, center=0)
        r := iszero(and(config, 0xffffffff))
    }
}

/// @notice Extracts the amplification factor from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @param config The pool config
/// @return r The amplification factor (0-127)
function stableswapAmplification(PoolConfig config) pure returns (uint8 r) {
    assembly ("memory-safe") {
        // Extract bits 30-24
        r := and(shr(24, config), 0x7f)
    }
}

/// @notice Extracts the center tick from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @dev The 24-bit center tick is scaled by 16 to get the actual tick
/// @param config The pool config
/// @return r The center tick
function stableswapCenterTick(PoolConfig config) pure returns (int32 r) {
    assembly ("memory-safe") {
        // Extract bits 23-0 and sign extend to 32 bits (24-bit signed integer)
        // Then multiply by 16, since the value does not have full precision
        r := mul(signextend(2, and(config, 0xffffff)), 16)
    }
}

/// @notice Returns the width of the liquidity range
function stableswapLiquidityWidth(PoolConfig config) pure returns (uint256 width) {
    uint8 amp = config.stableswapAmplification();

    assembly ("memory-safe") {
        width := shr(amp, MAX_TICK)
    }
}

/// @notice Computes the tick range where liquidity is active for stableswap pools
/// @param config The pool config
/// @return lower The lower tick of the bounds
/// @return upper The upper tick of the bounds
function stableswapActiveLiquidityTickRange(PoolConfig config) pure returns (int32 lower, int32 upper) {
    int32 center = config.stableswapCenterTick();
    uint256 width = config.stableswapLiquidityWidth();

    assembly ("memory-safe") {
        lower := sub(center, width)
        lower := add(lower, mul(sgt(MIN_TICK, lower), sub(MIN_TICK, lower)))

        upper := add(center, width)
        upper := sub(upper, mul(sgt(upper, MAX_TICK), sub(upper, MAX_TICK)))
    }
}

/// @notice Creates a PoolConfig for a concentrated liquidity pool
/// @param _fee The fee for the pool
/// @param _tickSpacing The tick spacing for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createConcentratedPoolConfig(uint64 _fee, uint32 _tickSpacing, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Set bit 31 to 1 for concentrated liquidity, then OR with tick spacing (bits 30-0)
        let typeConfig := or(0x80000000, and(_tickSpacing, 0x7fffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a stableswap pool
/// @param _fee The fee for the pool
/// @param _amplification The amplification factor (0-127)
/// @param _centerTick The center tick (will be divided by 16 and stored as 24-bit value)
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createStableswapPoolConfig(uint64 _fee, uint8 _amplification, int32 _centerTick, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Divide center tick by 16 to get 24-bit representation
        let stableswapCenterTick24 := sdiv(_centerTick, 16)
        // Pack: bit 31 = 0 (stableswap), bits 30-24 = amplification, bits 23-0 = center tick
        let typeConfig := or(shl(24, and(_amplification, 0x7f)), and(stableswapCenterTick24, 0xffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a full range pool (stableswap with amplification=0, center=0)
/// @param _fee The fee for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createFullRangePoolConfig(uint64 _fee, address _extension) pure returns (PoolConfig c) {
    assembly ("memory-safe") {
        // All 32 bits of type config are 0 (discriminator=0, amplification=0, center=0)
        c := or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff)))
    }
}

/// @notice Computes the maximum liquidity per tick for a given concentrated liquidity pool configuration.
/// @dev Only valid for concentrated liquidity pools. Stableswap pools don't use ticks.
/// @dev Calculated as type(uint128).max / (1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2)
/// @param config The concentrated liquidity pool configuration
/// @return maxLiquidity The maximum liquidity allowed to reference each tick
function concentratedMaxLiquidityPerTick(PoolConfig config) pure returns (uint128 maxLiquidity) {
    uint32 _tickSpacing = config.concentratedTickSpacing();

    assembly ("memory-safe") {
        // Calculate total number of usable ticks: 1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2
        // This represents all ticks from -MAX_TICK_MAGNITUDE to +MAX_TICK_MAGNITUDE, and tick 0
        let numTicks := add(1, mul(div(MAX_TICK, _tickSpacing), 2))

        maxLiquidity := div(sub(shl(128, 1), 1), numTicks)
    }
}

/// @notice Thrown when tick spacing exceeds the maximum allowed value
error InvalidTickSpacing();

/// @notice Thrown when amplification factor exceeds the maximum allowed value
error InvalidStableswapAmplification();

/// @notice Thrown when center tick is not between min and max tick
error InvalidCenterTick();

/// @notice Validates that a pool config is properly formatted
/// @param config The config to validate
function validate(PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (config.concentratedTickSpacing() > MAX_TICK_SPACING || config.concentratedTickSpacing() == 0) {
            revert InvalidTickSpacing();
        }
    } else {
        // Stableswap pool: validate amplification factor <= 26
        if (config.stableswapAmplification() > 26) {
            revert InvalidStableswapAmplification();
        }
        int32 centerTick = config.stableswapCenterTick();
        if (centerTick < MIN_TICK || centerTick > MAX_TICK) {
            revert InvalidCenterTick();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK, MAX_TICK_SPACING} from "../math/constants.sol";

/// @notice Pool configuration packed into a single bytes32
/// @dev Contains extension address (20 bytes), fee (8 bytes), and pool type config (4 bytes)
/// Pool type config (32 bits):
///   - Bit 31: discriminator (1 = concentrated, 0 = stableswap)
///   - For concentrated (bit 31 = 1): bits 30-0 are tick spacing
///   - For stableswap (bit 31 = 0): bits 30-24 are amplification factor, bits 23-0 are center tick
type PoolConfig is bytes32;

using {
    fee,
    extension,
    isConcentrated,
    isStableswap,
    isFullRange,
    concentratedTickSpacing,
    stableswapAmplification,
    stableswapCenterTick,
    stableswapActiveLiquidityTickRange,
    stableswapLiquidityWidth,
    validate,
    concentratedMaxLiquidityPerTick
} for PoolConfig global;

/// @notice Extracts the fee from a pool config
/// @param config The pool config
/// @return r The fee
function fee(PoolConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(32, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the extension address from a pool config
/// @param config The pool config
/// @return r The extension address
function extension(PoolConfig config) pure returns (address r) {
    assembly ("memory-safe") {
        r := shr(96, config)
    }
}

/// @notice Checks if this is a concentrated liquidity pool
/// @param config The pool config
/// @return r True if the pool is concentrated liquidity
function isConcentrated(PoolConfig config) pure returns (bool r) {
    r = !config.isStableswap();
}

/// @notice Extracts the tick spacing from a concentrated liquidity pool config
/// @dev Only valid for concentrated liquidity pools (isConcentrated() == true)
/// @param config The pool config
/// @return r The tick spacing
function concentratedTickSpacing(PoolConfig config) pure returns (uint32 r) {
    assembly ("memory-safe") {
        // Extract lower 31 bits (bits 30-0)
        r := and(config, 0x7fffffff)
    }
}

/// @notice Checks if this is a stableswap pool
/// @param config The pool config
/// @return r True if the pool is stableswap
function isStableswap(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // = iff bit 31 is not set
        r := iszero(and(0x80000000, config))
    }
}

/// @notice Determines if this pool is full range (special case of stableswap with amplification=0 and center=0)
/// @dev Full range can be slightly optimized in that we don't need to compute the sqrt ratio at the tick boundaries
/// @param config The pool config
/// @return r True if the pool is full range
function isFullRange(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // Full range when all 32 bits are 0 (discriminator=0, amplification=0, center=0)
        r := iszero(and(config, 0xffffffff))
    }
}

/// @notice Extracts the amplification factor from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @param config The pool config
/// @return r The amplification factor (0-127)
function stableswapAmplification(PoolConfig config) pure returns (uint8 r) {
    assembly ("memory-safe") {
        // Extract bits 30-24
        r := and(shr(24, config), 0x7f)
    }
}

/// @notice Extracts the center tick from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @dev The 24-bit center tick is scaled by 16 to get the actual tick
/// @param config The pool config
/// @return r The center tick
function stableswapCenterTick(PoolConfig config) pure returns (int32 r) {
    assembly ("memory-safe") {
        // Extract bits 23-0 and sign extend to 32 bits (24-bit signed integer)
        // Then multiply by 16, since the value does not have full precision
        r := mul(signextend(2, and(config, 0xffffff)), 16)
    }
}

/// @notice Returns the width of the liquidity range
function stableswapLiquidityWidth(PoolConfig config) pure returns (uint256 width) {
    uint8 amp = config.stableswapAmplification();

    assembly ("memory-safe") {
        width := shr(amp, MAX_TICK)
    }
}

/// @notice Computes the tick range where liquidity is active for stableswap pools
/// @param config The pool config
/// @return lower The lower tick of the bounds
/// @return upper The upper tick of the bounds
function stableswapActiveLiquidityTickRange(PoolConfig config) pure returns (int32 lower, int32 upper) {
    int32 center = config.stableswapCenterTick();
    uint256 width = config.stableswapLiquidityWidth();

    assembly ("memory-safe") {
        lower := sub(center, width)
        lower := add(lower, mul(sgt(MIN_TICK, lower), sub(MIN_TICK, lower)))

        upper := add(center, width)
        upper := sub(upper, mul(sgt(upper, MAX_TICK), sub(upper, MAX_TICK)))
    }
}

/// @notice Creates a PoolConfig for a concentrated liquidity pool
/// @param _fee The fee for the pool
/// @param _tickSpacing The tick spacing for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createConcentratedPoolConfig(uint64 _fee, uint32 _tickSpacing, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Set bit 31 to 1 for concentrated liquidity, then OR with tick spacing (bits 30-0)
        let typeConfig := or(0x80000000, and(_tickSpacing, 0x7fffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a stableswap pool
/// @param _fee The fee for the pool
/// @param _amplification The amplification factor (0-127)
/// @param _centerTick The center tick (will be divided by 16 and stored as 24-bit value)
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createStableswapPoolConfig(uint64 _fee, uint8 _amplification, int32 _centerTick, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Divide center tick by 16 to get 24-bit representation
        let stableswapCenterTick24 := sdiv(_centerTick, 16)
        // Pack: bit 31 = 0 (stableswap), bits 30-24 = amplification, bits 23-0 = center tick
        let typeConfig := or(shl(24, and(_amplification, 0x7f)), and(stableswapCenterTick24, 0xffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a full range pool (stableswap with amplification=0, center=0)
/// @param _fee The fee for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createFullRangePoolConfig(uint64 _fee, address _extension) pure returns (PoolConfig c) {
    assembly ("memory-safe") {
        // All 32 bits of type config are 0 (discriminator=0, amplification=0, center=0)
        c := or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff)))
    }
}

/// @notice Computes the maximum liquidity per tick for a given concentrated liquidity pool configuration.
/// @dev Only valid for concentrated liquidity pools. Stableswap pools don't use ticks.
/// @dev Calculated as type(uint128).max / (1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2)
/// @param config The concentrated liquidity pool configuration
/// @return maxLiquidity The maximum liquidity allowed to reference each tick
function concentratedMaxLiquidityPerTick(PoolConfig config) pure returns (uint128 maxLiquidity) {
    uint32 _tickSpacing = config.concentratedTickSpacing();

    assembly ("memory-safe") {
        // Calculate total number of usable ticks: 1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2
        // This represents all ticks from -MAX_TICK_MAGNITUDE to +MAX_TICK_MAGNITUDE, and tick 0
        let numTicks := add(1, mul(div(MAX_TICK, _tickSpacing), 2))

        maxLiquidity := div(sub(shl(128, 1), 1), numTicks)
    }
}

/// @notice Thrown when tick spacing exceeds the maximum allowed value
error InvalidTickSpacing();

/// @notice Thrown when amplification factor exceeds the maximum allowed value
error InvalidStableswapAmplification();

/// @notice Thrown when center tick is not between min and max tick
error InvalidCenterTick();

/// @notice Validates that a pool config is properly formatted
/// @param config The config to validate
function validate(PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (config.concentratedTickSpacing() > MAX_TICK_SPACING || config.concentratedTickSpacing() == 0) {
            revert InvalidTickSpacing();
        }
    } else {
        // Stableswap pool: validate amplification factor <= 26
        if (config.stableswapAmplification() > 26) {
            revert InvalidStableswapAmplification();
        }
        int32 centerTick = config.stableswapCenterTick();
        if (centerTick < MIN_TICK || centerTick > MAX_TICK) {
            revert InvalidCenterTick();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @dev Computes 2^x where x is a 5.64 number and result is a 64.64 number
function exp2(uint256 x) pure returns (uint256 result) {
    unchecked {
        require(x < 0x400000000000000000); // Overflow

        result = 0x80000000000000000000000000000000;

        if (x & 0x8000000000000000 != 0) {
            result = result * 0x16A09E667F3BCC908B2FB1366EA957D3E >> 128;
        }
        if (x & 0x4000000000000000 != 0) {
            result = result * 0x1306FE0A31B7152DE8D5A46305C85EDEC >> 128;
        }
        if (x & 0x2000000000000000 != 0) {
            result = result * 0x1172B83C7D517ADCDF7C8C50EB14A791F >> 128;
        }
        if (x & 0x1000000000000000 != 0) {
            result = result * 0x10B5586CF9890F6298B92B71842A98363 >> 128;
        }
        if (x & 0x800000000000000 != 0) {
            result = result * 0x1059B0D31585743AE7C548EB68CA417FD >> 128;
        }
        if (x & 0x400000000000000 != 0) {
            result = result * 0x102C9A3E778060EE6F7CACA4F7A29BDE8 >> 128;
        }
        if (x & 0x200000000000000 != 0) {
            result = result * 0x10163DA9FB33356D84A66AE336DCDFA3F >> 128;
        }
        if (x & 0x100000000000000 != 0) {
            result = result * 0x100B1AFA5ABCBED6129AB13EC11DC9543 >> 128;
        }
        if (x & 0x80000000000000 != 0) {
            result = result * 0x10058C86DA1C09EA1FF19D294CF2F679B >> 128;
        }
        if (x & 0x40000000000000 != 0) {
            result = result * 0x1002C605E2E8CEC506D21BFC89A23A00F >> 128;
        }
        if (x & 0x20000000000000 != 0) {
            result = result * 0x100162F3904051FA128BCA9C55C31E5DF >> 128;
        }
        if (x & 0x10000000000000 != 0) {
            result = result * 0x1000B175EFFDC76BA38E31671CA939725 >> 128;
        }
        if (x & 0x8000000000000 != 0) {
            result = result * 0x100058BA01FB9F96D6CACD4B180917C3D >> 128;
        }
        if (x & 0x4000000000000 != 0) {
            result = result * 0x10002C5CC37DA9491D0985C348C68E7B3 >> 128;
        }
        if (x & 0x2000000000000 != 0) {
            result = result * 0x1000162E525EE054754457D5995292026 >> 128;
        }
        if (x & 0x1000000000000 != 0) {
            result = result * 0x10000B17255775C040618BF4A4ADE83FC >> 128;
        }
        if (x & 0x800000000000 != 0) {
            result = result * 0x1000058B91B5BC9AE2EED81E9B7D4CFAB >> 128;
        }
        if (x & 0x400000000000 != 0) {
            result = result * 0x100002C5C89D5EC6CA4D7C8ACC017B7C9 >> 128;
        }
        if (x & 0x200000000000 != 0) {
            result = result * 0x10000162E43F4F831060E02D839A9D16D >> 128;
        }
        if (x & 0x100000000000 != 0) {
            result = result * 0x100000B1721BCFC99D9F890EA06911763 >> 128;
        }
        if (x & 0x80000000000 != 0) {
            result = result * 0x10000058B90CF1E6D97F9CA14DBCC1628 >> 128;
        }
        if (x & 0x40000000000 != 0) {
            result = result * 0x1000002C5C863B73F016468F6BAC5CA2B >> 128;
        }
        if (x & 0x20000000000 != 0) {
            result = result * 0x100000162E430E5A18F6119E3C02282A5 >> 128;
        }
        if (x & 0x10000000000 != 0) {
            result = result * 0x1000000B1721835514B86E6D96EFD1BFE >> 128;
        }
        if (x & 0x8000000000 != 0) {
            result = result * 0x100000058B90C0B48C6BE5DF846C5B2EF >> 128;
        }
        if (x & 0x4000000000 != 0) {
            result = result * 0x10000002C5C8601CC6B9E94213C72737A >> 128;
        }
        if (x & 0x2000000000 != 0) {
            result = result * 0x1000000162E42FFF037DF38AA2B219F06 >> 128;
        }
        if (x & 0x1000000000 != 0) {
            result = result * 0x10000000B17217FBA9C739AA5819F44F9 >> 128;
        }
        if (x & 0x800000000 != 0) {
            result = result * 0x1000000058B90BFCDEE5ACD3C1CEDC823 >> 128;
        }
        if (x & 0x400000000 != 0) {
            result = result * 0x100000002C5C85FE31F35A6A30DA1BE50 >> 128;
        }
        if (x & 0x200000000 != 0) {
            result = result * 0x10000000162E42FF0999CE3541B9FFFCF >> 128;
        }
        if (x & 0x100000000 != 0) {
            result = result * 0x100000000B17217F80F4EF5AADDA45554 >> 128;
        }
        if (x & 0x80000000 != 0) {
            result = result * 0x10000000058B90BFBF8479BD5A81B51AD >> 128;
        }
        if (x & 0x40000000 != 0) {
            result = result * 0x1000000002C5C85FDF84BD62AE30A74CC >> 128;
        }
        if (x & 0x20000000 != 0) {
            result = result * 0x100000000162E42FEFB2FED257559BDAA >> 128;
        }
        if (x & 0x10000000 != 0) {
            result = result * 0x1000000000B17217F7D5A7716BBA4A9AE >> 128;
        }
        if (x & 0x8000000 != 0) {
            result = result * 0x100000000058B90BFBE9DDBAC5E109CCE >> 128;
        }
        if (x & 0x4000000 != 0) {
            result = result * 0x10000000002C5C85FDF4B15DE6F17EB0D >> 128;
        }
        if (x & 0x2000000 != 0) {
            result = result * 0x1000000000162E42FEFA494F1478FDE05 >> 128;
        }
        if (x & 0x1000000 != 0) {
            result = result * 0x10000000000B17217F7D20CF927C8E94C >> 128;
        }
        if (x & 0x800000 != 0) {
            result = result * 0x1000000000058B90BFBE8F71CB4E4B33D >> 128;
        }
        if (x & 0x400000 != 0) {
            result = result * 0x100000000002C5C85FDF477B662B26945 >> 128;
        }
        if (x & 0x200000 != 0) {
            result = result * 0x10000000000162E42FEFA3AE53369388C >> 128;
        }
        if (x & 0x100000 != 0) {
            result = result * 0x100000000000B17217F7D1D351A389D40 >> 128;
        }
        if (x & 0x80000 != 0) {
            result = result * 0x10000000000058B90BFBE8E8B2D3D4EDE >> 128;
        }
        if (x & 0x40000 != 0) {
            result = result * 0x1000000000002C5C85FDF4741BEA6E77E >> 128;
        }
        if (x & 0x20000 != 0) {
            result = result * 0x100000000000162E42FEFA39FE95583C2 >> 128;
        }
        if (x & 0x10000 != 0) {
            result = result * 0x1000000000000B17217F7D1CFB72B45E1 >> 128;
        }
        if (x & 0x8000 != 0) {
            result = result * 0x100000000000058B90BFBE8E7CC35C3F0 >> 128;
        }
        if (x & 0x4000 != 0) {
            result = result * 0x10000000000002C5C85FDF473E242EA38 >> 128;
        }
        if (x & 0x2000 != 0) {
            result = result * 0x1000000000000162E42FEFA39F02B772C >> 128;
        }
        if (x & 0x1000 != 0) {
            result = result * 0x10000000000000B17217F7D1CF7D83C1A >> 128;
        }
        if (x & 0x800 != 0) {
            result = result * 0x1000000000000058B90BFBE8E7BDCBE2E >> 128;
        }
        if (x & 0x400 != 0) {
            result = result * 0x100000000000002C5C85FDF473DEA871F >> 128;
        }
        if (x & 0x200 != 0) {
            result = result * 0x10000000000000162E42FEFA39EF44D91 >> 128;
        }
        if (x & 0x100 != 0) {
            result = result * 0x100000000000000B17217F7D1CF79E949 >> 128;
        }
        if (x & 0x80 != 0) {
            result = result * 0x10000000000000058B90BFBE8E7BCE544 >> 128;
        }
        if (x & 0x40 != 0) {
            result = result * 0x1000000000000002C5C85FDF473DE6ECA >> 128;
        }
        if (x & 0x20 != 0) {
            result = result * 0x100000000000000162E42FEFA39EF366F >> 128;
        }
        if (x & 0x10 != 0) {
            result = result * 0x1000000000000000B17217F7D1CF79AFA >> 128;
        }
        if (x & 0x8 != 0) {
            result = result * 0x100000000000000058B90BFBE8E7BCD6D >> 128;
        }
        if (x & 0x4 != 0) {
            result = result * 0x10000000000000002C5C85FDF473DE6B2 >> 128;
        }
        if (x & 0x2 != 0) {
            result = result * 0x1000000000000000162E42FEFA39EF358 >> 128;
        }
        if (x & 0x1 != 0) {
            result = result * 0x10000000000000000B17217F7D1CF79AB >> 128;
        }

        result >>= uint256(63 - (x >> 64));
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {LibBit} from "solady/utils/LibBit.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {amount0Delta, amount1Delta, sortAndConvertToFixedSqrtRatios} from "./delta.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";

// Liquidity Math Library
// Contains functions for calculating liquidity-related amounts and conversions
// Provides utilities for converting between liquidity changes and token amounts

/// @notice Returns the token0 and token1 delta owed for a given change in liquidity
/// @dev Calculates the token amounts required or returned when liquidity is added or removed from a position
/// @param sqrtRatio Current price (as a valid sqrt ratio)
/// @param liquidityDelta Signed liquidity change; positive = added, negative = removed
/// @param sqrtRatioLower The lower bound of the price range (as a valid sqrt ratio)
/// @param sqrtRatioUpper The upper bound of the price range (as a valid sqrt ratio)
/// @return delta0 The change in token0 amount
/// @return delta1 The change in token1 amount
function liquidityDeltaToAmountDelta(
    SqrtRatio sqrtRatio,
    int128 liquidityDelta,
    SqrtRatio sqrtRatioLower,
    SqrtRatio sqrtRatioUpper
) pure returns (int128 delta0, int128 delta1) {
    unchecked {
        if (liquidityDelta == 0) {
            return (0, 0);
        }
        bool isPositive = (liquidityDelta > 0);
        int256 sign = -1 + 2 * int256(LibBit.rawToUint(isPositive));
        // absolute value of a int128 always fits in a uint128
        uint128 magnitude = uint128(FixedPointMathLib.abs(liquidityDelta));

        if (sqrtRatio <= sqrtRatioLower) {
            delta0 = SafeCastLib.toInt128(
                sign * int256(uint256(amount0Delta(sqrtRatioLower, sqrtRatioUpper, magnitude, isPositive)))
            );
        } else if (sqrtRatio < sqrtRatioUpper) {
            delta0 = SafeCastLib.toInt128(
                sign * int256(uint256(amount0Delta(sqrtRatio, sqrtRatioUpper, magnitude, isPositive)))
            );
            delta1 = SafeCastLib.toInt128(
                sign * int256(uint256(amount1Delta(sqrtRatioLower, sqrtRatio, magnitude, isPositive)))
            );
        } else {
            delta1 = SafeCastLib.toInt128(
                sign * int256(uint256(amount1Delta(sqrtRatioLower, sqrtRatioUpper, magnitude, isPositive)))
            );
        }
    }
}

/// @notice Calculates the maximum liquidity that can be provided with a given amount of token0
/// @dev Used when the current price is below the position's range (only token0 is needed)
/// @param sqrtRatioLower The lower sqrt price ratio of the position
/// @param sqrtRatioUpper The upper sqrt price ratio of the position
/// @param amount The amount of token0 available
/// @return The maximum liquidity that can be provided
function maxLiquidityForToken0(uint256 sqrtRatioLower, uint256 sqrtRatioUpper, uint128 amount) pure returns (uint256) {
    unchecked {
        uint256 numerator1 = FixedPointMathLib.fullMulDivN(sqrtRatioLower, sqrtRatioUpper, 128);

        return FixedPointMathLib.fullMulDiv(amount, numerator1, (sqrtRatioUpper - sqrtRatioLower));
    }
}

/// @notice Calculates the maximum liquidity that can be provided with a given amount of token1
/// @dev Used when the current price is above the position's range (only token1 is needed)
/// @param sqrtRatioLower The lower sqrt price ratio of the position
/// @param sqrtRatioUpper The upper sqrt price ratio of the position
/// @param amount The amount of token1 available
/// @return The maximum liquidity that can be provided
function maxLiquidityForToken1(uint256 sqrtRatioLower, uint256 sqrtRatioUpper, uint128 amount) pure returns (uint256) {
    unchecked {
        return (uint256(amount) << 128) / (sqrtRatioUpper - sqrtRatioLower);
    }
}

/// @notice Calculates the maximum liquidity that can be provided given amounts of both tokens
/// @dev Determines the limiting factor between token0 and token1 based on current price and position bounds
/// @param _sqrtRatio Current sqrt price ratio
/// @param sqrtRatioA One bound of the position (will be sorted with sqrtRatioB)
/// @param sqrtRatioB Other bound of the position (will be sorted with sqrtRatioA)
/// @param amount0 Available amount of token0
/// @param amount1 Available amount of token1
/// @return The maximum liquidity that can be provided with the given token amounts
function maxLiquidity(
    SqrtRatio _sqrtRatio,
    SqrtRatio sqrtRatioA,
    SqrtRatio sqrtRatioB,
    uint128 amount0,
    uint128 amount1
) pure returns (uint128) {
    uint256 sqrtRatio = _sqrtRatio.toFixed();
    (uint256 sqrtRatioLower, uint256 sqrtRatioUpper) = sortAndConvertToFixedSqrtRatios(sqrtRatioA, sqrtRatioB);

    if (sqrtRatio <= sqrtRatioLower) {
        return uint128(
            FixedPointMathLib.min(type(uint128).max, maxLiquidityForToken0(sqrtRatioLower, sqrtRatioUpper, amount0))
        );
    } else if (sqrtRatio < sqrtRatioUpper) {
        return uint128(
            FixedPointMathLib.min(
                type(uint128).max,
                FixedPointMathLib.min(
                    maxLiquidityForToken0(sqrtRatio, sqrtRatioUpper, amount0),
                    maxLiquidityForToken1(sqrtRatioLower, sqrtRatio, amount1)
                )
            )
        );
    } else {
        return uint128(
            FixedPointMathLib.min(type(uint128).max, maxLiquidityForToken1(sqrtRatioLower, sqrtRatioUpper, amount1))
        );
    }
}

/// @notice Thrown when a liquidity delta operation would cause overflow
error LiquidityDeltaOverflow();

/// @notice Safely adds a liquidity delta to a liquidity amount
/// @dev Reverts if the operation would cause overflow or underflow
/// @param liquidity The current liquidity amount
/// @param liquidityDelta The change in liquidity (can be positive or negative)
/// @return result The new liquidity amount after applying the delta
function addLiquidityDelta(uint128 liquidity, int128 liquidityDelta) pure returns (uint128 result) {
    assembly ("memory-safe") {
        result := add(liquidity, liquidityDelta)
        if and(result, shl(128, 0xffffffffffffffffffffffffffffffff)) {
            mstore(0, shl(224, 0x6d862c50))
            revert(0, 4)
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type OrderState is bytes32;

using {lastUpdateTime, saleRate, amountSold, parse} for OrderState global;

function lastUpdateTime(OrderState state) pure returns (uint32 time) {
    assembly ("memory-safe") {
        time := and(state, 0xffffffff)
    }
}

function saleRate(OrderState state) pure returns (uint112 rate) {
    assembly ("memory-safe") {
        rate := shr(144, shl(112, state))
    }
}

function amountSold(OrderState state) pure returns (uint112 amount) {
    assembly ("memory-safe") {
        amount := shr(144, state)
    }
}

function parse(OrderState state) pure returns (uint32 time, uint112 rate, uint112 amount) {
    assembly ("memory-safe") {
        time := and(state, 0xffffffff)
        rate := shr(144, shl(112, state))
        amount := shr(144, state)
    }
}

function createOrderState(uint32 _lastUpdateTime, uint112 _saleRate, uint112 _amountSold) pure returns (OrderState s) {
    assembly ("memory-safe") {
        // s = (lastUpdateTime) | (saleRate << 32) | (amountSold << 144)
        s := or(
            or(and(_lastUpdateTime, 0xffffffff), shl(32, shr(144, shl(144, _saleRate)))),
            shl(144, shr(144, shl(144, _amountSold)))
        )
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a TWAMM order
/// @dev Wraps bytes32 to provide type safety for order identifiers
type OrderId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Locker is bytes32;

using {id, addr, parse} for Locker global;

function id(Locker locker) pure returns (uint256 v) {
    assembly ("memory-safe") {
        v := sub(shr(160, locker), 1)
    }
}

function addr(Locker locker) pure returns (address v) {
    assembly ("memory-safe") {
        v := shr(96, shl(96, locker))
    }
}

function parse(Locker locker) pure returns (uint256 lockerId, address lockerAddr) {
    assembly ("memory-safe") {
        lockerId := sub(shr(160, locker), 1)
        lockerAddr := shr(96, shl(96, locker))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";

error Amount0DeltaOverflow();
error Amount1DeltaOverflow();

function sortAndConvertToFixedSqrtRatios(SqrtRatio sqrtRatioA, SqrtRatio sqrtRatioB)
    pure
    returns (uint256 sqrtRatioLower, uint256 sqrtRatioUpper)
{
    sqrtRatioLower = sqrtRatioA.toFixed();
    sqrtRatioUpper = sqrtRatioB.toFixed();
    assembly ("memory-safe") {
        let diff := mul(sub(sqrtRatioLower, sqrtRatioUpper), gt(sqrtRatioLower, sqrtRatioUpper))

        sqrtRatioLower := sub(sqrtRatioLower, diff)
        sqrtRatioUpper := add(sqrtRatioUpper, diff)
    }
}

/// @dev Assumes that the sqrt ratios are valid
function amount0Delta(SqrtRatio sqrtRatioA, SqrtRatio sqrtRatioB, uint128 liquidity, bool roundUp)
    pure
    returns (uint128 amount0)
{
    (uint256 sqrtRatioLower, uint256 sqrtRatioUpper) = sortAndConvertToFixedSqrtRatios(sqrtRatioA, sqrtRatioB);
    amount0 = amount0DeltaSorted(sqrtRatioLower, sqrtRatioUpper, liquidity, roundUp);
}

/// @dev Assumes that the sqrt ratios are non-zero and sorted
function amount0DeltaSorted(uint256 sqrtRatioLower, uint256 sqrtRatioUpper, uint128 liquidity, bool roundUp)
    pure
    returns (uint128 amount0)
{
    unchecked {
        uint256 liquidityX128;
        assembly ("memory-safe") {
            liquidityX128 := shl(128, liquidity)
        }
        if (roundUp) {
            uint256 result0 =
                FixedPointMathLib.fullMulDivUp(liquidityX128, (sqrtRatioUpper - sqrtRatioLower), sqrtRatioUpper);
            assembly ("memory-safe") {
                let result := add(div(result0, sqrtRatioLower), iszero(iszero(mod(result0, sqrtRatioLower))))
                if shr(128, result) {
                    // cast sig "Amount0DeltaOverflow()"
                    mstore(0, 0xb4ef2546)
                    revert(0x1c, 0x04)
                }
                amount0 := result
            }
        } else {
            uint256 result0 =
                FixedPointMathLib.fullMulDivUnchecked(liquidityX128, (sqrtRatioUpper - sqrtRatioLower), sqrtRatioUpper);
            uint256 result = FixedPointMathLib.rawDiv(result0, sqrtRatioLower);
            assembly ("memory-safe") {
                if shr(128, result) {
                    // cast sig "Amount0DeltaOverflow()"
                    mstore(0, 0xb4ef2546)
                    revert(0x1c, 0x04)
                }
                amount0 := result
            }
        }
    }
}

/// @dev Assumes that the sqrt ratios are valid
function amount1Delta(SqrtRatio sqrtRatioA, SqrtRatio sqrtRatioB, uint128 liquidity, bool roundUp)
    pure
    returns (uint128 amount1)
{
    (uint256 sqrtRatioLower, uint256 sqrtRatioUpper) = sortAndConvertToFixedSqrtRatios(sqrtRatioA, sqrtRatioB);
    amount1 = amount1DeltaSorted(sqrtRatioLower, sqrtRatioUpper, liquidity, roundUp);
}

function amount1DeltaSorted(uint256 sqrtRatioLower, uint256 sqrtRatioUpper, uint128 liquidity, bool roundUp)
    pure
    returns (uint128 amount1)
{
    unchecked {
        uint256 difference = sqrtRatioUpper - sqrtRatioLower;
        uint256 liquidityU256;
        assembly ("memory-safe") {
            liquidityU256 := liquidity
        }

        if (roundUp) {
            uint256 result = FixedPointMathLib.fullMulDivN(difference, liquidityU256, 128);
            assembly ("memory-safe") {
                // addition is safe from overflow because the result of fullMulDivN will never equal type(uint256).max
                result := add(
                    result,
                    iszero(iszero(mulmod(difference, liquidityU256, 0x100000000000000000000000000000000)))
                )
                if shr(128, result) {
                    // cast sig "Amount1DeltaOverflow()"
                    mstore(0, 0x59d2b24a)
                    revert(0x1c, 0x04)
                }
                amount1 := result
            }
        } else {
            uint256 result = FixedPointMathLib.fullMulDivN(difference, liquidityU256, 128);
            assembly ("memory-safe") {
                if shr(128, result) {
                    // cast sig "Amount1DeltaOverflow()"
                    mstore(0, 0x59d2b24a)
                    revert(0x1c, 0x04)
                }
                amount1 := result
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

// For any given time `t`, there are up to 91 times that are greater than `t` and valid according to `isTimeValid`
uint256 constant MAX_NUM_VALID_TIMES = 91;

// If we constrain the sale rate delta to this value, then the current sale rate will never overflow
uint256 constant MAX_ABS_VALUE_SALE_RATE_DELTA = type(uint112).max / MAX_NUM_VALID_TIMES;

/// @dev Returns the step size, i.e. the value of which the order end or start time must be a multiple of, based on the current time and the specified time
///      The step size has a minimum of 256 seconds and increases in powers of 16 as the gap to `time` grows.
///      Assumes currentTime < type(uint256).max - 4095
/// @param currentTime The current block timestamp
/// @param time The time for which the step size is being computed, based on how far in the future it is from currentTime
function computeStepSize(uint256 currentTime, uint256 time) pure returns (uint256 stepSize) {
    assembly ("memory-safe") {
        switch gt(time, add(currentTime, 4095))
        case 1 {
            let diff := sub(time, currentTime)

            let msb := sub(255, clz(diff)) // = index of msb

            msb := sub(msb, mod(msb, 4)) // = round down to multiple of 4

            stepSize := shl(msb, 1)
        }
        default { stepSize := 256 }
    }
}

/// @dev Returns true iff the given time is a valid start or end time for a TWAMM order
function isTimeValid(uint256 currentTime, uint256 time) pure returns (bool valid) {
    uint256 stepSize = computeStepSize(currentTime, time);

    assembly ("memory-safe") {
        valid := and(iszero(mod(time, stepSize)), or(lt(time, currentTime), lt(sub(time, currentTime), 0x100000000)))
    }
}

/// @dev Returns the next valid time if there is one, or wraps around to the time 0 if there is not
///      Assumes currentTime is less than type(uint256).max - type(uint32).max
function nextValidTime(uint256 currentTime, uint256 time) pure returns (uint256 nextTime) {
    unchecked {
        uint256 stepSize = computeStepSize(currentTime, time);
        assembly ("memory-safe") {
            nextTime := add(time, stepSize)
            nextTime := sub(nextTime, mod(nextTime, stepSize))
        }

        // only if we didn't overflow
        if (nextTime != 0) {
            uint256 nextStepSize = computeStepSize(currentTime, nextTime);
            if (nextStepSize != stepSize) {
                assembly ("memory-safe") {
                    nextTime := add(time, nextStepSize)
                    nextTime := sub(nextTime, mod(nextTime, nextStepSize))
                }
            }
        }

        nextTime = FixedPointMathLib.ternary(nextTime > currentTime + type(uint32).max, 0, nextTime);
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

