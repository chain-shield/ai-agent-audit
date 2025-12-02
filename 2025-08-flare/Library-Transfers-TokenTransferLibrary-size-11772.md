
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {Reentrancy} from "../../openzeppelin/library/Reentrancy.sol";


library Transfers {
    uint256 internal constant TRANSFER_GAS_ALLOWANCE = 100_000;

    error TransferFailed();

    // make sure the transfer is only called in non-reentrant method
    modifier requireReentrancyGuard {
        Reentrancy.requireReentrancyGuard();
        _;
    }

    /**
     * Transfer the given amount of NAT to recipient without gas limit of `address.transfer()`.
     *
     * **Warning**: Must guard with nonReentrant, otherwise the method will fail.
     *
     * **Warning 2**: may fail, so only use when the top-level transaction sender controls recipient address
     * (and therefore expects to fail if there is something strange at that address).
     *
     * @param _recipient the recipient address
     * @param _amount the amount in NAT Wei
     */
    function transferNAT(address payable _recipient, uint256 _amount)
        internal
        requireReentrancyGuard
    {
        if (_amount > 0) {
            /* solhint-disable avoid-low-level-calls */
            //slither-disable-next-line arbitrary-send-eth
            (bool success, ) = _recipient.call{value: _amount, gas: TRANSFER_GAS_ALLOWANCE}("");
            /* solhint-enable avoid-low-level-calls */
            require(success, TransferFailed());
        }
    }

    /**
     * Deposits the given amount of NAT to recipient on WNat contract.
     *
     * @param _wNat the WNat contract address
     * @param _recipient the recipient address
     * @param _amount the amount in NAT Wei
     */
    function depositWNat(IWNat _wNat, address _recipient, uint256 _amount)
        internal
    {
        if (_amount > 0) {
            _wNat.depositTo{value: _amount}(_recipient);
        }
    }
}
END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
// solhint-disable gas-custom-errors
// solhint-disable reason-string

pragma solidity ^0.8.27;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {IGovernanceVotePower} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceVotePower.sol";
import {IVPContractEvents} from "@flarenetwork/flare-periphery-contracts/flare/IVPContractEvents.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {IVPToken, IWNat} from "../interfaces/IWNat.sol";


contract WNatMock is IWNat, ERC20 {
    using EnumerableSet for EnumerableSet.AddressSet;

    IGovernanceVotePower private governanceVP;

    struct Delegation {
        address delegateAddress; // address to which the vote power is delegated
        uint16 bips; // 10000 bips = 100%
    }

    mapping(address => Delegation[]) private delegations;
    mapping(address delegatee => EnumerableSet.AddressSet) private delegators;

    constructor(
        address /*_governance*/,
        string memory _name,
        string memory _symbol
    )
        ERC20(_name, _symbol)
    {
    }

    receive() external payable {
        deposit();
    }

    function name() public view override(ERC20, IVPToken) returns (string memory) {
        return ERC20.name();
    }

    function symbol() public view override(ERC20, IVPToken) returns (string memory) {
        return ERC20.symbol();
    }

    function decimals() public view override(ERC20, IVPToken) returns (uint8) {
        return ERC20.decimals();
    }

    function deposit() public payable {
        _mint(msg.sender, msg.value);
    }

    function depositTo(address _recipient) public payable {
        _mint(_recipient, msg.value);
    }

    function withdraw(uint256 _amount) external {
        _burn(msg.sender, _amount);
        payable(msg.sender).transfer(_amount);
    }

    function withdrawFrom(address _owner, uint256 _amount) external {
        _spendAllowance(_owner, msg.sender, _amount);
        _burn(_owner, _amount);
        payable(msg.sender).transfer(_amount);
    }

    function delegate(address _to, uint256 _bips) external {
        require(_to != address(0), "cannot delegate to zero address");
        require(_to != msg.sender, "cannot delegate to self");
        require(_bips <= SafePct.MAX_BIPS, "bips out of range");

        uint256 totalBips = 0;
        bool update = false;
        Delegation[] storage ownerDelegations = delegations[msg.sender];
        for (uint256 i = 0; i < ownerDelegations.length; i++) {
            if (ownerDelegations[i].delegateAddress == _to) {
                ownerDelegations[i].bips = uint16(_bips);
                update = true;
            }
            totalBips += ownerDelegations[i].bips;
        }
        if (!update) {
            // add new delegation
            ownerDelegations.push(Delegation(_to, uint16(_bips)));
            totalBips += _bips;
        }
        require(totalBips <= SafePct.MAX_BIPS, "total bips cannot exceed 10000");
        require(ownerDelegations.length <= 2, "cannot have more than 2 delegations");

        // update delegators set
        delegators[_to].add(msg.sender);
    }

    function undelegateAll() external {
        // remove from delegators set
        Delegation[] storage ownerDelegations = delegations[msg.sender];
        for (uint256 i = 0; i < ownerDelegations.length; i++) {
            address delegateAddress = ownerDelegations[i].delegateAddress;
            delegators[delegateAddress].remove(msg.sender);
        }
        delete delegations[msg.sender];
    }

    function delegatesOf(
        address _owner
    )
        external view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        )
    {
        Delegation[] storage ownerDelegations = delegations[_owner];
        _count = ownerDelegations.length;
        _delegateAddresses = new address[](_count);
        _bips = new uint256[](_count);
        for (uint256 i = 0; i < _count; i++) {
            _delegateAddresses[i] = ownerDelegations[i].delegateAddress;
            _bips[i] = ownerDelegations[i].bips;
        }
        _delegationMode = 1; // 1 means delegation by percentage (bips)
    }

    function totalVotePower() external view returns(uint256) {
        return totalSupply();
    }

    function votePowerOf(address _owner) external view returns(uint256 _votePower) {
        uint256 balance = balanceOf(_owner);
        _votePower = balance;

        // Subtract delegated vote power
        Delegation[] storage ownerDelegations = delegations[_owner];
        for (uint256 i = 0; i < ownerDelegations.length; i++) {
            uint256 bips = ownerDelegations[i].bips;
            _votePower -= (balance * bips) / SafePct.MAX_BIPS;
        }

        // Add vote power from delegators
        EnumerableSet.AddressSet storage ownerDelegators = delegators[_owner];
        for (uint256 i = 0; i < ownerDelegators.length(); i++) {
            address delegator = ownerDelegators.at(i);
            Delegation[] storage delegatorDelegations = delegations[delegator];
            for (uint256 j = 0; j < delegatorDelegations.length; j++) {
                if (delegatorDelegations[j].delegateAddress == _owner) {
                    uint256 bips = delegatorDelegations[j].bips;
                    _votePower += (balanceOf(delegator) * bips) / SafePct.MAX_BIPS;
                }
            }
        }
    }

    function undelegatedVotePowerOf(address _owner) external view returns(uint256 _votePower) {
        uint256 balance = balanceOf(_owner);
        _votePower = balance;

        // Subtract delegated vote power
        Delegation[] storage ownerDelegations = delegations[_owner];
        for (uint256 i = 0; i < ownerDelegations.length; i++) {
            uint256 bips = ownerDelegations[i].bips;
            _votePower -= (balance * bips) / SafePct.MAX_BIPS;
        }
    }

    function votePowerFromTo(address _from, address _to) external view returns(uint256) {
        Delegation[] storage ownerDelegations = delegations[_from];
        for (uint256 i = 0; i < ownerDelegations.length; i++) {
            if (ownerDelegations[i].delegateAddress == _to) {
                uint256 balance = balanceOf(_from);
                uint256 bips = ownerDelegations[i].bips;
                return (balance * bips) / SafePct.MAX_BIPS;
            }
        }
        return 0; // No delegations found
    }

    function delegationModeOf(address /*_who*/) external pure returns(uint256) {
        // In this mock implementation, we only support delegation by percentage (bips)
        return 1; // 1 means delegations by percentage (bips)
    }

    function setGovernanceVotePower(IGovernanceVotePower _governanceVotePower) external {
        governanceVP = _governanceVotePower;
    }

    function governanceVotePower() external view returns (IGovernanceVotePower) {
        return governanceVP;
    }

    //////// UNIMPLEMENTED METHODS ////////

    function balanceOfAt(address /*_owner*/, uint256 /*_blockNumber*/) external pure returns (uint256) {
        revert("not implemented");
    }

    function totalSupplyAt(uint256 /*_blockNumber*/) external pure returns(uint256) {
        revert("not implemented");
    }

    function totalVotePowerAt(uint256 /*_blockNumber*/) external pure returns(uint256) {
        revert("not implemented");
    }

    function votePowerFromToAt(address /*_from*/, address /*_to*/, uint256 /*_blockNumber*/)
        external pure returns(uint256)
    {
        revert("not implemented");
    }

    function votePowerOfAt(address /*_owner*/, uint256 /*_blockNumber*/) external pure returns(uint256) {
        revert("not implemented");
    }

    function votePowerOfAtIgnoringRevocation(address /*_owner*/, uint256 /*_blockNumber*/)
        external pure returns(uint256)
    {
        revert("not implemented");
    }

    function delegatesOfAt(address /*_who*/, uint256 /*_blockNumber*/)
        external pure
        returns (
            address[] memory /*_delegateAddresses*/,
            uint256[] memory /*_bips*/,
            uint256 /*_count*/,
            uint256 /*_delegationMode*/
        )
    {
        revert("not implemented");
    }

    function undelegatedVotePowerOfAt(address /*_owner*/, uint256 /*_blockNumber*/) external pure returns(uint256) {
        revert("not implemented");
    }

    function batchDelegate(address[] memory /*_delegatees*/, uint256[] memory /*_bips*/) external pure {
        revert("not implemented");
    }

    function delegateExplicit(address /*_to*/, uint256 /*_amount*/) external pure {
        revert("not implemented");
    }

    function revokeDelegationAt(address /*_who*/, uint256 /*_blockNumber*/) external pure {
        revert("not implemented");
    }

    function undelegateAllExplicit(address[] memory /*_delegateAddresses*/) external pure returns (uint256) {
        revert("not implemented");
    }

    function readVotePowerContract() external pure returns (IVPContractEvents) {
        revert("not implemented");
    }

    function writeVotePowerContract() external pure returns (IVPContractEvents) {
        revert("not implemented");
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IVPToken} from "@flarenetwork/flare-periphery-contracts/flare/IVPToken.sol";

/**
 * @title Wrapped Native token
 * @notice Accept native token deposits and mint ERC20 WNAT (wrapped native) tokens 1-1.
 */
interface IWNat is IVPToken {
    /**
     * @notice Deposit Native and mint wNat ERC20.
     */
    function deposit() external payable;

    /**
     * @notice Deposit Native from msg.sender and mints WNAT ERC20 to recipient address.
     * @param recipient An address to receive minted WNAT.
     */
    function depositTo(address recipient) external payable;

    /**
     * @notice Withdraw Native and burn WNAT ERC20.
     * @param amount The amount to withdraw.
     */
    function withdraw(uint256 amount) external;

    /**
     * @notice Withdraw WNAT from an owner and send native tokens to msg.sender given an allowance.
     * @param owner An address spending the Native tokens.
     * @param amount The amount to spend.
     *
     * Requirements:
     *
     * - `owner` must have a balance of at least `amount`.
     * - the caller must have allowance for `owners`'s tokens of at least
     * `amount`.
     */
    function withdrawFrom(address owner, uint256 amount) external;
}

// SPDX-License-Identifier: MIT

// OpenZeppelin Contracts (last updated v4.9.0) (security/ReentrancyGuard.sol)
// Modified by FlareLabs to use diamond storage

pragma solidity ^0.8.27;


/**
 * Code for the `ReentrancyGuard` contract.
 */
library Reentrancy {

    error ReentrancyGuardReentrantCall();
    error ReentrancyGuardRequired();

    // Booleans are more expensive than uint256 or any type that takes up a full
    // word because each write operation emits an extra SLOAD to first read the
    // slot's contents, replace the bits taken up by the boolean, and then write
    // back. This is the compiler's defense against contract upgrades and
    // pointer aliasing, and it cannot be disabled.

    // The values being non-zero value makes deployment a bit more expensive,
    // but in exchange the refund on every call to nonReentrant will be lower in
    // amount. Since refunds are capped to a percentage of the total
    // transaction's gas, it is best to keep them low in cases like this one, to
    // increase the likelihood of the full refund coming into effect.
    uint256 private constant _NOT_ENTERED = 1;
    uint256 private constant _ENTERED = 2;

    struct ReentrancyGuardState {
        uint256 status;
    }

    /**
     * Should be called once at construction time of the main diamond contract.
     * Not a big issue if it is never called - just the first nonReentrant method call will use more gas.
     */
    function initializeReentrancyGuard() internal {
        ReentrancyGuardState storage state = _reentrancyGuardState();
        state.status = _NOT_ENTERED;
    }

    function nonReentrantBefore() internal {
        ReentrancyGuardState storage state = _reentrancyGuardState();
        // On the first call to nonReentrant, state.status will be _NOT_ENTERED
        require(state.status != _ENTERED, ReentrancyGuardReentrantCall());

        // Any calls to nonReentrant after this point will fail
        state.status = _ENTERED;
    }

    function nonReentrantAfter() internal {
        ReentrancyGuardState storage state = _reentrancyGuardState();
        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        state.status = _NOT_ENTERED;
    }

    /**
     * @dev Returns true if the reentrancy guard is currently set to "entered", which indicates there is a
     * `nonReentrant` function in the call stack.
     */
    function reentrancyGuardEntered() internal view returns (bool) {
        ReentrancyGuardState storage state = _reentrancyGuardState();
        return state.status == _ENTERED;
    }

    /**
     * Marks a piece of code that can only be executed within a `nonReentrant` method.
     * Useful to prevent e.g. NAT transfers that don't properly guard against reentrancy
     * and to make them fail at test time.
     */
    function requireReentrancyGuard() internal view {
        require(reentrancyGuardEntered(), ReentrancyGuardRequired());
    }

    function _reentrancyGuardState() private pure returns (ReentrancyGuardState storage _state) {
        bytes32 position = keccak256("utils.ReentrancyGuard.ReentrancyGuardState");
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _state.slot := position
        }
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

/**
 * @dev Compute percentages safely without phantom overflows.
 *
 * Intermediate operations can overflow even when the result will always
 * fit into computed type. Developers usually
 * assume that overflows raise errors. `SafePct` restores this intuition by
 * reverting the transaction when such an operation overflows.
 *
 * Using this library instead of the unchecked operations eliminates an entire
 * class of bugs, so it's recommended to use it always.
 */
library SafePct {
    uint256 internal constant MAX_BIPS = 10_000;

    error DivisionByZero();

    /**
     * Calculates `floor(x * y / z)`, reverting on overflow, but only if the result overflows.
     * Requirement: intermediate operations must revert on overflow.
     */
    function mulDiv(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        require(z > 0, DivisionByZero());

        if (x == 0) return 0;
        unchecked {
            uint256 xy = x * y;
            if (xy / x == y) { // no overflow happened (works in unchecked)
                return xy / z;
            }
        }

        //slither-disable-next-line divide-before-multiply
        uint256 a = x / z;
        uint256 b = x % z; // x = a * z + b

        //slither-disable-next-line divide-before-multiply
        uint256 c = y / z;
        uint256 d = y % z; // y = c * z + d

        return (a * c * z) + (a * d) + (b * c) + (b * d / z);
    }

    /**
     * Calculates `ceiling(x * y / z)`.
     */
    function mulDivRoundUp(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        uint256 resultRoundDown = mulDiv(x, y, z);
        unchecked {
            // safe - if z == 0, above mulDiv call would revert
            uint256 remainder = mulmod(x, y, z);
            // safe - overflow only possible if z == 1, but then remainder == 0
            return remainder == 0 ? resultRoundDown : resultRoundDown + 1;
        }
    }

    /**
     * Return `x * y BIPS` = `x * y / 10_000`, rounded down.
     */
    function mulBips(uint256 x, uint256 y) internal pure returns (uint256) {
        return mulDiv(x, y, MAX_BIPS);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IVPToken} from "@flarenetwork/flare-periphery-contracts/flare/IVPToken.sol";

/**
 * @title Wrapped Native token
 * @notice Accept native token deposits and mint ERC20 WNAT (wrapped native) tokens 1-1.
 */
interface IWNat is IVPToken {
    /**
     * @notice Deposit Native and mint wNat ERC20.
     */
    function deposit() external payable;

    /**
     * @notice Deposit Native from msg.sender and mints WNAT ERC20 to recipient address.
     * @param recipient An address to receive minted WNAT.
     */
    function depositTo(address recipient) external payable;

    /**
     * @notice Withdraw Native and burn WNAT ERC20.
     * @param amount The amount to withdraw.
     */
    function withdraw(uint256 amount) external;

    /**
     * @notice Withdraw WNAT from an owner and send native tokens to msg.sender given an allowance.
     * @param owner An address spending the Native tokens.
     * @param amount The amount to spend.
     *
     * Requirements:
     *
     * - `owner` must have a balance of at least `amount`.
     * - the caller must have allowance for `owners`'s tokens of at least
     * `amount`.
     */
    function withdrawFrom(address owner, uint256 amount) external;
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

interface IVPContractEvents {
    /**
     * Event triggered when an account delegates or undelegates another account.
     * Definition: `votePowerFromTo(from, to)` is `changed` from `priorVotePower` to `newVotePower`.
     * For undelegation, `newVotePower` is 0.
     *
     * Note: the event is always emitted from VPToken's `writeVotePowerContract`.
     */
    event Delegate(
        address indexed from,
        address indexed to,
        uint256 priorVotePower,
        uint256 newVotePower
    );

    /**
     * Event triggered only when account `delegator` revokes delegation to `delegatee`
     * for a single block in the past (typically the current vote block).
     *
     * Note: the event is always emitted from VPToken's `writeVotePowerContract` and/or `readVotePowerContract`.
     */
    event Revoke(
        address indexed delegator,
        address indexed delegatee,
        uint256 votePower,
        uint256 blockNumber
    );
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IGovernanceVotePower} from "./IGovernanceVotePower.sol";
import {IVPContractEvents} from "./IVPContractEvents.sol";

interface IVPToken is IERC20 {
    /**
     * @notice Delegate by percentage `_bips` of voting power to `_to` from `msg.sender`.
     * @param _to The address of the recipient
     * @param _bips The percentage of voting power to be delegated expressed in basis points (1/100 of one percent).
     *   Not cumulative - every call resets the delegation value (and value of 0 undelegates `to`).
     **/
    function delegate(address _to, uint256 _bips) external;

    /**
     * @notice Undelegate all percentage delegations from the sender and then delegate corresponding
     *   `_bips` percentage of voting power from the sender to each member of `_delegatees`.
     * @param _delegatees The addresses of the new recipients.
     * @param _bips The percentages of voting power to be delegated expressed in basis points (1/100 of one percent).
     *   Total of all `_bips` values must be at most 10000.
     **/
    function batchDelegate(
        address[] memory _delegatees,
        uint256[] memory _bips
    ) external;

    /**
     * @notice Explicitly delegate `_amount` of voting power to `_to` from `msg.sender`.
     * @param _to The address of the recipient
     * @param _amount An explicit vote power amount to be delegated.
     *   Not cumulative - every call resets the delegation value (and value of 0 undelegates `to`).
     **/
    function delegateExplicit(address _to, uint _amount) external;

    /**
     * @notice Revoke all delegation from sender to `_who` at given block.
     *    Only affects the reads via `votePowerOfAtCached()` in the block `_blockNumber`.
     *    Block `_blockNumber` must be in the past.
     *    This method should be used only to prevent rogue delegate voting in the current voting block.
     *    To stop delegating use delegate/delegateExplicit with value of 0 or undelegateAll/undelegateAllExplicit.
     * @param _who Address of the delegatee
     * @param _blockNumber The block number at which to revoke delegation.
     */
    function revokeDelegationAt(address _who, uint _blockNumber) external;

    /**
     * @notice Undelegate all voting power for delegates of `msg.sender`
     *    Can only be used with percentage delegation.
     *    Does not reset delegation mode back to NOTSET.
     **/
    function undelegateAll() external;

    /**
     * @notice Undelegate all explicit vote power by amount delegates for `msg.sender`.
     *    Can only be used with explicit delegation.
     *    Does not reset delegation mode back to NOTSET.
     * @param _delegateAddresses Explicit delegation does not store delegatees' addresses,
     *   so the caller must supply them.
     * @return The amount still delegated (in case the list of delegates was incomplete).
     */
    function undelegateAllExplicit(
        address[] memory _delegateAddresses
    ) external returns (uint256);

    /**
     * @dev Should be compatible with ERC20 method
     */
    function name() external view returns (string memory);

    /**
     * @dev Should be compatible with ERC20 method
     */
    function symbol() external view returns (string memory);

    /**
     * @dev Should be compatible with ERC20 method
     */
    function decimals() external view returns (uint8);

    /**
     * @notice Total amount of tokens at a specific `_blockNumber`.
     * @param _blockNumber The block number when the totalSupply is queried
     * @return The total amount of tokens at `_blockNumber`
     **/
    function totalSupplyAt(uint _blockNumber) external view returns (uint256);

    /**
     * @dev Queries the token balance of `_owner` at a specific `_blockNumber`.
     * @param _owner The address from which the balance will be retrieved.
     * @param _blockNumber The block number when the balance is queried.
     * @return The balance at `_blockNumber`.
     **/
    function balanceOfAt(
        address _owner,
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the current total vote power.
     * @return The current total vote power (sum of all accounts' vote powers).
     */
    function totalVotePower() external view returns (uint256);

    /**
     * @notice Get the total vote power at block `_blockNumber`
     * @param _blockNumber The block number at which to fetch.
     * @return The total vote power at the block  (sum of all accounts' vote powers).
     */
    function totalVotePowerAt(
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the current vote power of `_owner`.
     * @param _owner The address to get voting power.
     * @return Current vote power of `_owner`.
     */
    function votePowerOf(address _owner) external view returns (uint256);

    /**
     * @notice Get the vote power of `_owner` at block `_blockNumber`
     * @param _owner The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_owner` at `_blockNumber`.
     */
    function votePowerOfAt(
        address _owner,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the vote power of `_owner` at block `_blockNumber`, ignoring revocation information (and cache).
     * @param _owner The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_owner` at `_blockNumber`. Result doesn't change if vote power is revoked.
     */
    function votePowerOfAtIgnoringRevocation(
        address _owner,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the delegation mode for '_who'. This mode determines whether vote power is
     *  allocated by percentage or by explicit value. Once the delegation mode is set,
     *  it never changes, even if all delegations are removed.
     * @param _who The address to get delegation mode.
     * @return delegation mode: 0 = NOTSET, 1 = PERCENTAGE, 2 = AMOUNT (i.e. explicit)
     */
    function delegationModeOf(address _who) external view returns (uint256);

    /**
     * @notice Get current delegated vote power `_from` delegator delegated `_to` delegatee.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @return The delegated vote power.
     */
    function votePowerFromTo(
        address _from,
        address _to
    ) external view returns (uint256);

    /**
     * @notice Get delegated the vote power `_from` delegator delegated `_to` delegatee at `_blockNumber`.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @param _blockNumber The block number at which to fetch.
     * @return The delegated vote power.
     */
    function votePowerFromToAt(
        address _from,
        address _to,
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Compute the current undelegated vote power of `_owner`
     * @param _owner The address to get undelegated voting power.
     * @return The unallocated vote power of `_owner`
     */
    function undelegatedVotePowerOf(
        address _owner
    ) external view returns (uint256);

    /**
     * @notice Get the undelegated vote power of `_owner` at given block.
     * @param _owner The address to get undelegated voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return The undelegated vote power of `_owner` (= owner's own balance minus all delegations from owner)
     */
    function undelegatedVotePowerOfAt(
        address _owner,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the vote power delegation `delegationAddresses`
     *  and `_bips` of `_who`. Returned in two separate positional arrays.
     * @param _who The address to get delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOf(
        address _who
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * @notice Get the vote power delegation `delegationAddresses`
     *  and `pcts` of `_who`. Returned in two separate positional arrays.
     * @param _who The address to get delegations.
     * @param _blockNumber The block for which we want to know the delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOfAt(
        address _who,
        uint256 _blockNumber
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * Returns VPContract used for readonly operations (view methods).
     * The only non-view method that might be called on it is `revokeDelegationAt`.
     *
     * @notice `readVotePowerContract` is almost always equal to `writeVotePowerContract`
     * except during upgrade from one VPContract to a new version (which should happen
     * rarely or never and will be anounced before).
     *
     * @notice You shouldn't call any methods on VPContract directly, all are exposed
     * via VPToken (and state changing methods are forbidden from direct calls).
     * This is the reason why this method returns `IVPContractEvents` - it should only be used
     * for listening to events (`Revoke` only).
     */
    function readVotePowerContract() external view returns (IVPContractEvents);

    /**
     * Returns VPContract used for state changing operations (non-view methods).
     * The only non-view method that might be called on it is `revokeDelegationAt`.
     *
     * @notice `writeVotePowerContract` is almost always equal to `readVotePowerContract`
     * except during upgrade from one VPContract to a new version (which should happen
     * rarely or never and will be anounced before). In the case of upgrade,
     * `writeVotePowerContract` will be replaced first to establish delegations, and
     * after some perio (e.g. after a reward epoch ends) `readVotePowerContract` will be set equal to it.
     *
     * @notice You shouldn't call any methods on VPContract directly, all are exposed
     * via VPToken (and state changing methods are forbidden from direct calls).
     * This is the reason why this method returns `IVPContractEvents` - it should only be used
     * for listening to events (`Delegate` and `Revoke` only).
     */
    function writeVotePowerContract() external view returns (IVPContractEvents);

    /**
     * When set, allows token owners to participate in governance voting
     * and delegate governance vote power.
     */
    function governanceVotePower() external view returns (IGovernanceVotePower);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

/**
 * Interface for contracts delegating their governance vote power.
 */
interface IGovernanceVotePower {
    /**
     * Delegates all governance vote power of `msg.sender` to address `_to`.
     * @param _to The address of the recipient.
     */
    function delegate(address _to) external;

    /**
     * Undelegates all governance vote power of `msg.sender`.
     */
    function undelegate() external;

    /**
     * Gets the governance vote power of an address at a given block number, including
     * all delegations made to it.
     * @param _who The address being queried.
     * @param _blockNumber The block number at which to fetch the vote power.
     * @return Governance vote power of `_who` at `_blockNumber`.
     */
    function votePowerOfAt(
        address _who,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * Gets the governance vote power of an address at the latest block, including
     * all delegations made to it.
     * @param _who The address being queried.
     * @return Governance vote power of `account` at the lastest block.
     */
    function getVotes(address _who) external view returns (uint256);

    /**
     * Gets the address an account is delegating its governance vote power to, at a given block number.
     * @param _who The address being queried.
     * @param _blockNumber The block number at which to fetch the address.
     * @return Address where `_who` was delegating its governance vote power at block `_blockNumber`.
     */
    function getDelegateOfAt(
        address _who,
        uint256 _blockNumber
    ) external view returns (address);

    /**
     * Gets the address an account is delegating its governance vote power to, at the latest block number.
     * @param _who The address being queried.
     * @return Address where `_who` is currently delegating its governance vote power.
     */
    function getDelegateOfAtNow(address _who) external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "../../IVPToken.sol";
import "../../IVPContractEvents.sol";
import "./IICleanable.sol";

interface IIVPContract is IICleanable, IVPContractEvents {
    /**
     * Update vote powers when tokens are transfered.
     * Also update delegated vote powers for percentage delegation
     * and check for enough funds for explicit delegations.
     **/
    function updateAtTokenTransfer(
        address _from,
        address _to,
        uint256 _fromBalance,
        uint256 _toBalance,
        uint256 _amount
    ) external;

    /**
     * @notice Delegate `_bips` percentage of voting power to `_to` from `_from`
     * @param _from The address of the delegator
     * @param _to The address of the recipient
     * @param _balance The delegator's current balance
     * @param _bips The percentage of voting power to be delegated expressed in basis points (1/100 of one percent).
     *   Not cumulative - every call resets the delegation value (and value of 0 revokes delegation).
     **/
    function delegate(
        address _from,
        address _to,
        uint256 _balance,
        uint256 _bips
    ) external;

    /**
     * @notice Explicitly delegate `_amount` of voting power to `_to` from `msg.sender`.
     * @param _from The address of the delegator
     * @param _to The address of the recipient
     * @param _balance The delegator's current balance
     * @param _amount An explicit vote power amount to be delegated.
     *   Not cumulative - every call resets the delegation value (and value of 0 undelegates `to`).
     **/
    function delegateExplicit(
        address _from,
        address _to,
        uint256 _balance,
        uint _amount
    ) external;

    /**
     * @notice Revoke all delegation from sender to `_who` at given block.
     *    Only affects the reads via `votePowerOfAtCached()` in the block `_blockNumber`.
     *    Block `_blockNumber` must be in the past.
     *    This method should be used only to prevent rogue delegate voting in the current voting block.
     *    To stop delegating use delegate/delegateExplicit with value of 0 or undelegateAll/undelegateAllExplicit.
     * @param _from The address of the delegator
     * @param _who Address of the delegatee
     * @param _balance The delegator's current balance
     * @param _blockNumber The block number at which to revoke delegation.
     **/
    function revokeDelegationAt(
        address _from,
        address _who,
        uint256 _balance,
        uint _blockNumber
    ) external;

    /**
     * @notice Undelegate all voting power for delegates of `msg.sender`
     *    Can only be used with percentage delegation.
     *    Does not reset delegation mode back to NOTSET.
     * @param _from The address of the delegator
     **/
    function undelegateAll(address _from, uint256 _balance) external;

    /**
     * @notice Undelegate all explicit vote power by amount delegates for `msg.sender`.
     *    Can only be used with explicit delegation.
     *    Does not reset delegation mode back to NOTSET.
     * @param _from The address of the delegator
     * @param _delegateAddresses Explicit delegation does not store delegatees' addresses,
     *   so the caller must supply them.
     * @return The amount still delegated (in case the list of delegates was incomplete).
     */
    function undelegateAllExplicit(
        address _from,
        address[] memory _delegateAddresses
    ) external returns (uint256);

    /**
     * @notice Get the vote power of `_who` at block `_blockNumber`
     *   Reads/updates cache and upholds revocations.
     * @param _who The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_who` at `_blockNumber`.
     */
    function votePowerOfAtCached(
        address _who,
        uint256 _blockNumber
    ) external returns (uint256);

    /**
     * @notice Get the current vote power of `_who`.
     * @param _who The address to get voting power.
     * @return Current vote power of `_who`.
     */
    function votePowerOf(address _who) external view returns (uint256);

    /**
     * @notice Get the vote power of `_who` at block `_blockNumber`
     * @param _who The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_who` at `_blockNumber`.
     */
    function votePowerOfAt(
        address _who,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the vote power of `_who` at block `_blockNumber`, ignoring revocation information (and cache).
     * @param _who The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_who` at `_blockNumber`. Result doesn't change if vote power is revoked.
     */
    function votePowerOfAtIgnoringRevocation(
        address _who,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * Return vote powers for several addresses in a batch.
     * @param _owners The list of addresses to fetch vote power of.
     * @param _blockNumber The block number at which to fetch.
     * @return A list of vote powers.
     */
    function batchVotePowerOfAt(
        address[] memory _owners,
        uint256 _blockNumber
    ) external view returns (uint256[] memory);

    /**
     * @notice Get current delegated vote power `_from` delegator delegated `_to` delegatee.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @param _balance The delegator's current balance
     * @return The delegated vote power.
     */
    function votePowerFromTo(
        address _from,
        address _to,
        uint256 _balance
    ) external view returns (uint256);

    /**
     * @notice Get delegated the vote power `_from` delegator delegated `_to` delegatee at `_blockNumber`.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @param _balance The delegator's current balance
     * @param _blockNumber The block number at which to fetch.
     * @return The delegated vote power.
     */
    function votePowerFromToAt(
        address _from,
        address _to,
        uint256 _balance,
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Compute the current undelegated vote power of `_owner`
     * @param _owner The address to get undelegated voting power.
     * @param _balance Owner's current balance
     * @return The unallocated vote power of `_owner`
     */
    function undelegatedVotePowerOf(
        address _owner,
        uint256 _balance
    ) external view returns (uint256);

    /**
     * @notice Get the undelegated vote power of `_owner` at given block.
     * @param _owner The address to get undelegated voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return The undelegated vote power of `_owner` (= owner's own balance minus all delegations from owner)
     */
    function undelegatedVotePowerOfAt(
        address _owner,
        uint256 _balance,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the delegation mode for '_who'. This mode determines whether vote power is
     *  allocated by percentage or by explicit value.
     * @param _who The address to get delegation mode.
     * @return Delegation mode (NOTSET=0, PERCENTAGE=1, AMOUNT=2))
     */
    function delegationModeOf(address _who) external view returns (uint256);

    /**
     * @notice Get the vote power delegation `_delegateAddresses`
     *  and `pcts` of an `_owner`. Returned in two separate positional arrays.
     * @param _owner The address to get delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOf(
        address _owner
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * @notice Get the vote power delegation `delegationAddresses`
     *  and `pcts` of an `_owner`. Returned in two separate positional arrays.
     * @param _owner The address to get delegations.
     * @param _blockNumber The block for which we want to know the delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOfAt(
        address _owner,
        uint256 _blockNumber
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * The VPToken (or some other contract) that owns this VPContract.
     * All state changing methods may be called only from this address.
     * This is because original msg.sender is sent in `_from` parameter
     * and we must be sure that it cannot be faked by directly calling VPContract.
     * Owner token is also used in case of replacement to recover vote powers from balances.
     */
    function ownerToken() external view returns (IVPToken);

    /**
     * Return true if this IIVPContract is configured to be used as a replacement for other contract.
     * It means that vote powers are not necessarily correct at the initialization, therefore
     * every method that reads vote power must check whether it is initialized for that address and block.
     */
    function isReplacement() external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "../../IVPToken.sol";
import "../../IGovernanceVotePower.sol";
import "./IIVPContract.sol";
import "./IIGovernanceVotePower.sol";
import "./IICleanable.sol";

interface IIVPToken is IVPToken, IICleanable {
    /**
     * Set the contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function setCleanupBlockNumberManager(
        address _cleanupBlockNumberManager
    ) external;

    /**
     * Sets new governance vote power contract that allows token owners to participate in governance voting
     * and delegate governance vote power.
     */
    function setGovernanceVotePower(
        IIGovernanceVotePower _governanceVotePower
    ) external;

    /**
     * @notice Get the total vote power at block `_blockNumber` using cache.
     *   It tries to read the cached value and if not found, reads the actual value and stores it in cache.
     *   Can only be used if `_blockNumber` is in the past, otherwise reverts.
     * @param _blockNumber The block number at which to fetch.
     * @return The total vote power at the block (sum of all accounts' vote powers).
     */
    function totalVotePowerAtCached(
        uint256 _blockNumber
    ) external returns (uint256);

    /**
     * @notice Get the vote power of `_owner` at block `_blockNumber` using cache.
     *   It tries to read the cached value and if not found, reads the actual value and stores it in cache.
     *   Can only be used if _blockNumber is in the past, otherwise reverts.
     * @param _owner The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_owner` at `_blockNumber`.
     */
    function votePowerOfAtCached(
        address _owner,
        uint256 _blockNumber
    ) external returns (uint256);

    /**
     * Return vote powers for several addresses in a batch.
     * @param _owners The list of addresses to fetch vote power of.
     * @param _blockNumber The block number at which to fetch.
     * @return A list of vote powers.
     */
    function batchVotePowerOfAt(
        address[] memory _owners,
        uint256 _blockNumber
    ) external view returns (uint256[] memory);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "../../IVPToken.sol";
import "../../IPChainStakeMirror.sol";
import "../../IGovernanceVotePower.sol";

/**
 * Internal interface for contracts delegating their governance vote power.
 */
interface IIGovernanceVotePower is IGovernanceVotePower {
    /**
     * Emitted when a delegate's vote power changes, as a result of a new delegation
     * or a token transfer, for example.
     *
     * The event is always emitted from a `GovernanceVotePower` contract.
     * @param delegate The account receiving the changing delegated vote power.
     * @param previousBalance Delegated vote power before the change.
     * @param newBalance Delegated vote power after the change.
     */
    event DelegateVotesChanged(
        address indexed delegate,
        uint256 previousBalance,
        uint256 newBalance
    );

    /**
     * Emitted when an account starts delegating vote power or switches its delegation
     * to another address.
     *
     * The event is always emitted from a `GovernanceVotePower` contract.
     * @param delegator Account delegating its vote power.
     * @param fromDelegate Account receiving the delegation before the change.
     * Can be address(0) if there was no previous delegation.
     * @param toDelegate Account receiving the delegation after the change.
     * Can be address(0) if `delegator` just undelegated all its vote power.
     */
    event DelegateChanged(
        address indexed delegator,
        address indexed fromDelegate,
        address indexed toDelegate
    );

    /**
     * Update governance vote power of all involved delegates after tokens are transferred.
     *
     * This function **MUST** be called after each governance token transfer for the
     * delegates to reflect the correct balance.
     * @param _from Source address of the transfer.
     * @param _to Destination address of the transfer.
     * @param _fromBalance _Ignored._
     * @param _toBalance _Ignored._
     * @param _amount Amount being transferred.
     */
    function updateAtTokenTransfer(
        address _from,
        address _to,
        uint256 _fromBalance,
        uint256 _toBalance,
        uint256 _amount
    ) external;

    /**
     * Set the cleanup block number.
     * Historic data for the blocks before `cleanupBlockNumber` can be erased.
     * History before that block should never be used since it can be inconsistent.
     * In particular, cleanup block number must be lower than the current vote power block.
     * @param _blockNumber The new cleanup block number.
     */
    function setCleanupBlockNumber(uint256 _blockNumber) external;

    /**
     * Set the contract that is allowed to call history cleaning methods.
     * @param _cleanerContract Address of the cleanup contract.
     * Usually this will be an instance of `CleanupBlockNumberManager`.
     */
    function setCleanerContract(address _cleanerContract) external;

    /**
     * Get the token that this governance vote power contract belongs to.
     * @return The IVPToken interface owning this contract.
     */
    function ownerToken() external view returns (IVPToken);

    /**
     * Get the stake mirror contract that this governance vote power contract belongs to.
     * @return The IPChainStakeMirror interface owning this contract.
     */
    function pChainStakeMirror() external view returns (IPChainStakeMirror);

    /**
     * Get the current cleanup block number set with `setCleanupBlockNumber()`.
     * @return The currently set cleanup block number.
     */
    function getCleanupBlockNumber() external view returns (uint256);
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

