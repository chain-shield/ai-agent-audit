
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";

import {Constants} from "./Constants.sol";

struct InsuranceFund {
    uint256 balance;
}

using InsuranceFundLib for InsuranceFund global;

library InsuranceFundLib {
    using SafeTransferLib for address;

    address constant USDC = Constants.USDC;

    event InsuranceFundWithdrawal(address indexed account, uint256 amount);
    event InsuranceFundDeposit(address indexed account, uint256 amount);

    error InsufficientInsuranceFundBalance();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               INSURANCE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function pay(InsuranceFund storage self, uint256 amount) internal {
        if (amount == 0) return;
        self.balance += amount;
    }

    function claim(InsuranceFund storage self, uint256 amount) internal {
        if (amount == 0) return;
        if (self.balance < amount) revert InsufficientInsuranceFundBalance();
        self.balance -= amount;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 ADMIN
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function withdraw(InsuranceFund storage self, uint256 amount) internal {
        if (self.balance < amount) revert InsufficientInsuranceFundBalance();
        self.balance -= amount;
        USDC.safeTransfer(msg.sender, amount);
        emit InsuranceFundWithdrawal(msg.sender, amount);
    }

    function deposit(InsuranceFund storage self, uint256 amount) internal {
        self.balance += amount;
        USDC.safeTransferFrom(msg.sender, address(this), amount);
        emit InsuranceFundDeposit(msg.sender, amount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getBalance(InsuranceFund storage self) internal view returns (uint256) {
        return self.balance;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

library Constants {
    // ADDRESSES
    address constant USDC = 0xE9b6e75C243B6100ffcb1c66e8f78F96FeeA727F;
    address constant GTL = 0x037eDa3aDB1198021A9b2e88C22B464fD38db3f3;
    // ROLES
    uint256 constant ADMIN_ROLE = 1 << 7;
    uint256 constant KEEPER_ROLE = 1 << 6;
    uint256 constant LIQUIDATOR_ROLE = 1 << 5;
    uint256 constant BACKSTOP_LIQUIDATOR_ROLE = 1 << 4;
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

