
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {DynamicArrayLib} from "solady/utils/DynamicArrayLib.sol";
import {NATIVE_TOKEN_ADDRESS} from "../math/constants.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

contract TokenDataFetcher {
    using DynamicArrayLib for *;

    struct Balance {
        address token;
        uint256 amount;
    }

    struct Allowance {
        address token;
        address spender;
        uint256 amount;
    }

    function getNonzeroBalancesAndAllowances(address owner, address[] memory tokens, address[] memory spenders)
        external
        view
        returns (Balance[] memory balances, Allowance[] memory allowances)
    {
        unchecked {
            DynamicArrayLib.DynamicArray memory balanceTuples;
            DynamicArrayLib.DynamicArray memory allowanceTuples;

            for (uint256 i = 0; i < tokens.length; i++) {
                address token = tokens[i];

                uint256 balance;
                if (token == NATIVE_TOKEN_ADDRESS) {
                    balance = address(owner).balance;
                } else {
                    balance = SafeTransferLib.balanceOf(token, owner);
                }

                if (balance != 0) {
                    balanceTuples.p(uint256(uint160(token)));
                    balanceTuples.p(balance);
                }

                if (token != NATIVE_TOKEN_ADDRESS) {
                    for (uint256 j = 0; j < spenders.length; j++) {
                        address spender = spenders[j];

                        (bool success, bytes memory result) =
                            token.staticcall(abi.encodeWithSelector(IERC20.allowance.selector, owner, spender));
                        if (success && result.length == 32) {
                            uint256 allowance = abi.decode(result, (uint256));
                            if (allowance > 0) {
                                allowanceTuples.p(uint256(uint160(token)));
                                allowanceTuples.p(uint256(uint160(spender)));
                                allowanceTuples.p(allowance);
                            }
                        }
                    }
                }
            }

            balances = new Balance[](balanceTuples.length() / 2);
            for (uint256 i = 0; i < balances.length; i++) {
                address token = address(uint160(balanceTuples.get((i * 2))));
                uint256 balance = balanceTuples.get((i * 2) + 1);
                balances[i] = Balance(token, balance);
            }

            allowances = new Allowance[](allowanceTuples.length() / 3);
            for (uint256 i = 0; i < allowances.length; i++) {
                address token = address(uint160(allowanceTuples.get(i * 3)));
                address spender = address(uint160(allowanceTuples.get((i * 3) + 1)));
                uint256 balance = allowanceTuples.get((i * 3) + 2);
                allowances[i] = Allowance(token, spender, balance);
            }
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

