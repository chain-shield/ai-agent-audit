
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title OUSD Token Contract
 * @dev ERC20 compatible contract for OUSD
 * @dev Implements an elastic supply
 * @author Origin Protocol Inc
 */
import { IVault } from "../interfaces/IVault.sol";
import { Governable } from "../governance/Governable.sol";
import { SafeCast } from "@openzeppelin/contracts/utils/math/SafeCast.sol";

contract OUSD is Governable {
    using SafeCast for int256;
    using SafeCast for uint256;

    /// @dev Event triggered when the supply changes
    /// @param totalSupply Updated token total supply
    /// @param rebasingCredits Updated token rebasing credits
    /// @param rebasingCreditsPerToken Updated token rebasing credits per token
    event TotalSupplyUpdatedHighres(
        uint256 totalSupply,
        uint256 rebasingCredits,
        uint256 rebasingCreditsPerToken
    );
    /// @dev Event triggered when an account opts in for rebasing
    /// @param account Address of the account
    event AccountRebasingEnabled(address account);
    /// @dev Event triggered when an account opts out of rebasing
    /// @param account Address of the account
    event AccountRebasingDisabled(address account);
    /// @dev Emitted when `value` tokens are moved from one account `from` to
    ///      another `to`.
    /// @param from Address of the account tokens are moved from
    /// @param to Address of the account tokens are moved to
    /// @param value Amount of tokens transferred
    event Transfer(address indexed from, address indexed to, uint256 value);
    /// @dev Emitted when the allowance of a `spender` for an `owner` is set by
    ///      a call to {approve}. `value` is the new allowance.
    /// @param owner Address of the owner approving allowance
    /// @param spender Address of the spender allowance is granted to
    /// @param value Amount of tokens spender can transfer
    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 value
    );
    /// @dev Yield resulting from {changeSupply} that a `source` account would
    ///      receive is directed to `target` account.
    /// @param source Address of the source forwarding the yield
    /// @param target Address of the target receiving the yield
    event YieldDelegated(address source, address target);
    /// @dev Yield delegation from `source` account to the `target` account is
    ///      suspended.
    /// @param source Address of the source suspending yield forwarding
    /// @param target Address of the target no longer receiving yield from `source`
    ///        account
    event YieldUndelegated(address source, address target);

    enum RebaseOptions {
        NotSet,
        StdNonRebasing,
        StdRebasing,
        YieldDelegationSource,
        YieldDelegationTarget
    }

    uint256[154] private _gap; // Slots to align with deployed contract
    uint256 private constant MAX_SUPPLY = type(uint128).max;
    /// @dev The amount of tokens in existence
    uint256 public totalSupply;
    mapping(address => mapping(address => uint256)) private allowances;
    /// @dev The vault with privileges to execute {mint}, {burn}
    ///     and {changeSupply}
    address public vaultAddress;
    mapping(address => uint256) internal creditBalances;
    // the 2 storage variables below need trailing underscores to not name collide with public functions
    uint256 private rebasingCredits_; // Sum of all rebasing credits (creditBalances for rebasing accounts)
    uint256 private rebasingCreditsPerToken_;
    /// @dev The amount of tokens that are not rebasing - receiving yield
    uint256 public nonRebasingSupply;
    mapping(address => uint256) internal alternativeCreditsPerToken;
    /// @dev A map of all addresses and their respective RebaseOptions
    mapping(address => RebaseOptions) public rebaseState;
    mapping(address => uint256) private __deprecated_isUpgraded;
    /// @dev A map of addresses that have yields forwarded to. This is an
    ///      inverse mapping of {yieldFrom}
    /// Key Account forwarding yield
    /// Value Account receiving yield
    mapping(address => address) public yieldTo;
    /// @dev A map of addresses that are receiving the yield. This is an
    ///      inverse mapping of {yieldTo}
    /// Key Account receiving yield
    /// Value Account forwarding yield
    mapping(address => address) public yieldFrom;

    uint256 private constant RESOLUTION_INCREASE = 1e9;
    uint256[34] private __gap; // including below gap totals up to 200

    /// @dev Verifies that the caller is the Governor or Strategist.
    modifier onlyGovernorOrStrategist() {
        require(
            isGovernor() || msg.sender == IVault(vaultAddress).strategistAddr(),
            "Caller is not the Strategist or Governor"
        );
        _;
    }

    /// @dev Initializes the contract and sets necessary variables.
    /// @param _vaultAddress Address of the vault contract
    /// @param _initialCreditsPerToken The starting rebasing credits per token.
    function initialize(address _vaultAddress, uint256 _initialCreditsPerToken)
        external
        onlyGovernor
    {
        require(_vaultAddress != address(0), "Zero vault address");
        require(vaultAddress == address(0), "Already initialized");

        rebasingCreditsPerToken_ = _initialCreditsPerToken;
        vaultAddress = _vaultAddress;
    }

    /// @dev Returns the symbol of the token, a shorter version
    ///      of the name.
    function symbol() external pure virtual returns (string memory) {
        return "OUSD";
    }

    /// @dev Returns the name of the token.
    function name() external pure virtual returns (string memory) {
        return "Origin Dollar";
    }

    /// @dev Returns the number of decimals used to get its user representation.
    function decimals() external pure virtual returns (uint8) {
        return 18;
    }

    /**
     * @dev Verifies that the caller is the Vault contract
     */
    modifier onlyVault() {
        require(vaultAddress == msg.sender, "Caller is not the Vault");
        _;
    }

    /**
     * @return High resolution rebasingCreditsPerToken
     */
    function rebasingCreditsPerTokenHighres() external view returns (uint256) {
        return rebasingCreditsPerToken_;
    }

    /**
     * @return Low resolution rebasingCreditsPerToken
     */
    function rebasingCreditsPerToken() external view returns (uint256) {
        return rebasingCreditsPerToken_ / RESOLUTION_INCREASE;
    }

    /**
     * @return High resolution total number of rebasing credits
     */
    function rebasingCreditsHighres() external view returns (uint256) {
        return rebasingCredits_;
    }

    /**
     * @return Low resolution total number of rebasing credits
     */
    function rebasingCredits() external view returns (uint256) {
        return rebasingCredits_ / RESOLUTION_INCREASE;
    }

    /**
     * @notice Gets the balance of the specified address.
     * @param _account Address to query the balance of.
     * @return A uint256 representing the amount of base units owned by the
     *         specified address.
     */
    function balanceOf(address _account) public view returns (uint256) {
        RebaseOptions state = rebaseState[_account];
        if (state == RebaseOptions.YieldDelegationSource) {
            // Saves a slot read when transferring to or from a yield delegating source
            // since we know creditBalances equals the balance.
            return creditBalances[_account];
        }
        uint256 baseBalance = (creditBalances[_account] * 1e18) /
            _creditsPerToken(_account);
        if (state == RebaseOptions.YieldDelegationTarget) {
            // creditBalances of yieldFrom accounts equals token balances
            return baseBalance - creditBalances[yieldFrom[_account]];
        }
        return baseBalance;
    }

    /**
     * @notice Gets the credits balance of the specified address.
     * @dev Backwards compatible with old low res credits per token.
     * @param _account The address to query the balance of.
     * @return (uint256, uint256) Credit balance and credits per token of the
     *         address
     */
    function creditsBalanceOf(address _account)
        external
        view
        returns (uint256, uint256)
    {
        uint256 cpt = _creditsPerToken(_account);
        if (cpt == 1e27) {
            // For a period before the resolution upgrade, we created all new
            // contract accounts at high resolution. Since they are not changing
            // as a result of this upgrade, we will return their true values
            return (creditBalances[_account], cpt);
        } else {
            return (
                creditBalances[_account] / RESOLUTION_INCREASE,
                cpt / RESOLUTION_INCREASE
            );
        }
    }

    /**
     * @notice Gets the credits balance of the specified address.
     * @param _account The address to query the balance of.
     * @return (uint256, uint256, bool) Credit balance, credits per token of the
     *         address, and isUpgraded
     */
    function creditsBalanceOfHighres(address _account)
        external
        view
        returns (
            uint256,
            uint256,
            bool
        )
    {
        return (
            creditBalances[_account],
            _creditsPerToken(_account),
            true // all accounts have their resolution "upgraded"
        );
    }

    // Backwards compatible view
    function nonRebasingCreditsPerToken(address _account)
        external
        view
        returns (uint256)
    {
        return alternativeCreditsPerToken[_account];
    }

    /**
     * @notice Transfer tokens to a specified address.
     * @param _to the address to transfer to.
     * @param _value the amount to be transferred.
     * @return true on success.
     */
    function transfer(address _to, uint256 _value) external returns (bool) {
        require(_to != address(0), "Transfer to zero address");

        _executeTransfer(msg.sender, _to, _value);

        emit Transfer(msg.sender, _to, _value);
        return true;
    }

    /**
     * @notice Transfer tokens from one address to another.
     * @param _from The address you want to send tokens from.
     * @param _to The address you want to transfer to.
     * @param _value The amount of tokens to be transferred.
     * @return true on success.
     */
    function transferFrom(
        address _from,
        address _to,
        uint256 _value
    ) external returns (bool) {
        require(_to != address(0), "Transfer to zero address");
        uint256 userAllowance = allowances[_from][msg.sender];
        require(_value <= userAllowance, "Allowance exceeded");

        unchecked {
            allowances[_from][msg.sender] = userAllowance - _value;
        }

        _executeTransfer(_from, _to, _value);

        emit Transfer(_from, _to, _value);
        return true;
    }

    function _executeTransfer(
        address _from,
        address _to,
        uint256 _value
    ) internal {
        (
            int256 fromRebasingCreditsDiff,
            int256 fromNonRebasingSupplyDiff
        ) = _adjustAccount(_from, -_value.toInt256());
        (
            int256 toRebasingCreditsDiff,
            int256 toNonRebasingSupplyDiff
        ) = _adjustAccount(_to, _value.toInt256());

        _adjustGlobals(
            fromRebasingCreditsDiff + toRebasingCreditsDiff,
            fromNonRebasingSupplyDiff + toNonRebasingSupplyDiff
        );
    }

    function _adjustAccount(address _account, int256 _balanceChange)
        internal
        returns (int256 rebasingCreditsDiff, int256 nonRebasingSupplyDiff)
    {
        RebaseOptions state = rebaseState[_account];
        int256 currentBalance = balanceOf(_account).toInt256();
        if (currentBalance + _balanceChange < 0) {
            revert("Transfer amount exceeds balance");
        }
        uint256 newBalance = (currentBalance + _balanceChange).toUint256();

        if (state == RebaseOptions.YieldDelegationSource) {
            address target = yieldTo[_account];
            uint256 targetOldBalance = balanceOf(target);
            uint256 targetNewCredits = _balanceToRebasingCredits(
                targetOldBalance + newBalance
            );
            rebasingCreditsDiff =
                targetNewCredits.toInt256() -
                creditBalances[target].toInt256();

            creditBalances[_account] = newBalance;
            creditBalances[target] = targetNewCredits;
        } else if (state == RebaseOptions.YieldDelegationTarget) {
            uint256 newCredits = _balanceToRebasingCredits(
                newBalance + creditBalances[yieldFrom[_account]]
            );
            rebasingCreditsDiff =
                newCredits.toInt256() -
                creditBalances[_account].toInt256();
            creditBalances[_account] = newCredits;
        } else {
            _autoMigrate(_account);
            uint256 alternativeCreditsPerTokenMem = alternativeCreditsPerToken[
                _account
            ];
            if (alternativeCreditsPerTokenMem > 0) {
                nonRebasingSupplyDiff = _balanceChange;
                if (alternativeCreditsPerTokenMem != 1e18) {
                    alternativeCreditsPerToken[_account] = 1e18;
                }
                creditBalances[_account] = newBalance;
            } else {
                uint256 newCredits = _balanceToRebasingCredits(newBalance);
                rebasingCreditsDiff =
                    newCredits.toInt256() -
                    creditBalances[_account].toInt256();
                creditBalances[_account] = newCredits;
            }
        }
    }

    function _adjustGlobals(
        int256 _rebasingCreditsDiff,
        int256 _nonRebasingSupplyDiff
    ) internal {
        if (_rebasingCreditsDiff != 0) {
            rebasingCredits_ = (rebasingCredits_.toInt256() +
                _rebasingCreditsDiff).toUint256();
        }
        if (_nonRebasingSupplyDiff != 0) {
            nonRebasingSupply = (nonRebasingSupply.toInt256() +
                _nonRebasingSupplyDiff).toUint256();
        }
    }

    /**
     * @notice Function to check the amount of tokens that _owner has allowed
     *      to `_spender`.
     * @param _owner The address which owns the funds.
     * @param _spender The address which will spend the funds.
     * @return The number of tokens still available for the _spender.
     */
    function allowance(address _owner, address _spender)
        external
        view
        returns (uint256)
    {
        return allowances[_owner][_spender];
    }

    /**
     * @notice Approve the passed address to spend the specified amount of
     *      tokens on behalf of msg.sender.
     * @param _spender The address which will spend the funds.
     * @param _value The amount of tokens to be spent.
     * @return true on success.
     */
    function approve(address _spender, uint256 _value) external returns (bool) {
        allowances[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }

    /**
     * @notice Creates `_amount` tokens and assigns them to `_account`,
     *     increasing the total supply.
     */
    function mint(address _account, uint256 _amount) external onlyVault {
        require(_account != address(0), "Mint to the zero address");

        // Account
        (
            int256 toRebasingCreditsDiff,
            int256 toNonRebasingSupplyDiff
        ) = _adjustAccount(_account, _amount.toInt256());
        // Globals
        _adjustGlobals(toRebasingCreditsDiff, toNonRebasingSupplyDiff);
        totalSupply = totalSupply + _amount;

        require(totalSupply < MAX_SUPPLY, "Max supply");
        emit Transfer(address(0), _account, _amount);
    }

    /**
     * @notice Destroys `_amount` tokens from `_account`,
     *     reducing the total supply.
     */
    function burn(address _account, uint256 _amount) external onlyVault {
        require(_account != address(0), "Burn from the zero address");
        if (_amount == 0) {
            return;
        }

        // Account
        (
            int256 toRebasingCreditsDiff,
            int256 toNonRebasingSupplyDiff
        ) = _adjustAccount(_account, -_amount.toInt256());
        // Globals
        _adjustGlobals(toRebasingCreditsDiff, toNonRebasingSupplyDiff);
        totalSupply = totalSupply - _amount;

        emit Transfer(_account, address(0), _amount);
    }

    /**
     * @dev Get the credits per token for an account. Returns a fixed amount
     *      if the account is non-rebasing.
     * @param _account Address of the account.
     */
    function _creditsPerToken(address _account)
        internal
        view
        returns (uint256)
    {
        uint256 alternativeCreditsPerTokenMem = alternativeCreditsPerToken[
            _account
        ];
        if (alternativeCreditsPerTokenMem != 0) {
            return alternativeCreditsPerTokenMem;
        } else {
            return rebasingCreditsPerToken_;
        }
    }

    /**
     * @dev Auto migrate contracts to be non rebasing,
     *     unless they have opted into yield.
     * @param _account Address of the account.
     */
    function _autoMigrate(address _account) internal {
        uint256 codeLen = _account.code.length;
        bool isEOA = (codeLen == 0) ||
            (codeLen == 23 && bytes3(_account.code) == 0xef0100);
        // In previous code versions, contracts would not have had their
        // rebaseState[_account] set to RebaseOptions.NonRebasing when migrated
        // therefore we check the actual accounting used on the account as well.
        if (
            (!isEOA) &&
            rebaseState[_account] == RebaseOptions.NotSet &&
            alternativeCreditsPerToken[_account] == 0
        ) {
            _rebaseOptOut(_account);
        }
    }

    /**
     * @dev Calculates credits from contract's global rebasingCreditsPerToken_, and
     *      also balance that corresponds to those credits. The latter is important
     *      when adjusting the contract's global nonRebasingSupply to circumvent any
     *      possible rounding errors.
     *
     * @param _balance Balance of the account.
     */
    function _balanceToRebasingCredits(uint256 _balance)
        internal
        view
        returns (uint256 rebasingCredits)
    {
        // Rounds up, because we need to ensure that accounts always have
        // at least the balance that they should have.
        // Note this should always be used on an absolute account value,
        // not on a possibly negative diff, because then the rounding would be wrong.
        return ((_balance) * rebasingCreditsPerToken_ + 1e18 - 1) / 1e18;
    }

    /**
     * @notice The calling account will start receiving yield after a successful call.
     * @param _account Address of the account.
     */
    function governanceRebaseOptIn(address _account) external onlyGovernor {
        require(_account != address(0), "Zero address not allowed");
        _rebaseOptIn(_account);
    }

    /**
     * @notice The calling account will start receiving yield after a successful call.
     */
    function rebaseOptIn() external {
        _rebaseOptIn(msg.sender);
    }

    function _rebaseOptIn(address _account) internal {
        uint256 balance = balanceOf(_account);

        // prettier-ignore
        require(
            alternativeCreditsPerToken[_account] > 0 ||
                // Accounts may explicitly `rebaseOptIn` regardless of
                // accounting if they have a 0 balance.
                creditBalances[_account] == 0
            ,
            "Account must be non-rebasing"
        );
        RebaseOptions state = rebaseState[_account];
        // prettier-ignore
        require(
            state == RebaseOptions.StdNonRebasing ||
                state == RebaseOptions.NotSet,
            "Only standard non-rebasing accounts can opt in"
        );

        uint256 newCredits = _balanceToRebasingCredits(balance);

        // Account
        rebaseState[_account] = RebaseOptions.StdRebasing;
        alternativeCreditsPerToken[_account] = 0;
        creditBalances[_account] = newCredits;
        // Globals
        _adjustGlobals(newCredits.toInt256(), -balance.toInt256());

        emit AccountRebasingEnabled(_account);
    }

    /**
     * @notice The calling account will no longer receive yield
     */
    function rebaseOptOut() external {
        _rebaseOptOut(msg.sender);
    }

    function _rebaseOptOut(address _account) internal {
        require(
            alternativeCreditsPerToken[_account] == 0,
            "Account must be rebasing"
        );
        RebaseOptions state = rebaseState[_account];
        require(
            state == RebaseOptions.StdRebasing || state == RebaseOptions.NotSet,
            "Only standard rebasing accounts can opt out"
        );

        uint256 oldCredits = creditBalances[_account];
        uint256 balance = balanceOf(_account);

        // Account
        rebaseState[_account] = RebaseOptions.StdNonRebasing;
        alternativeCreditsPerToken[_account] = 1e18;
        creditBalances[_account] = balance;
        // Globals
        _adjustGlobals(-oldCredits.toInt256(), balance.toInt256());

        emit AccountRebasingDisabled(_account);
    }

    /**
     * @notice Distribute yield to users. This changes the exchange rate
     *  between "credits" and OUSD tokens to change rebasing user's balances.
     * @param _newTotalSupply New total supply of OUSD.
     */
    function changeSupply(uint256 _newTotalSupply) external onlyVault {
        require(totalSupply > 0, "Cannot increase 0 supply");

        if (totalSupply == _newTotalSupply) {
            emit TotalSupplyUpdatedHighres(
                totalSupply,
                rebasingCredits_,
                rebasingCreditsPerToken_
            );
            return;
        }

        totalSupply = _newTotalSupply > MAX_SUPPLY
            ? MAX_SUPPLY
            : _newTotalSupply;

        uint256 rebasingSupply = totalSupply - nonRebasingSupply;
        // round up in the favour of the protocol
        rebasingCreditsPerToken_ =
            (rebasingCredits_ * 1e18 + rebasingSupply - 1) /
            rebasingSupply;

        require(rebasingCreditsPerToken_ > 0, "Invalid change in supply");

        emit TotalSupplyUpdatedHighres(
            totalSupply,
            rebasingCredits_,
            rebasingCreditsPerToken_
        );
    }

    /*
     * @notice Send the yield from one account to another account.
     *         Each account keeps its own balances.
     */
    function delegateYield(address _from, address _to)
        external
        onlyGovernorOrStrategist
    {
        require(_from != address(0), "Zero from address not allowed");
        require(_to != address(0), "Zero to address not allowed");

        require(_from != _to, "Cannot delegate to self");
        require(
            yieldFrom[_to] == address(0) &&
                yieldTo[_to] == address(0) &&
                yieldFrom[_from] == address(0) &&
                yieldTo[_from] == address(0),
            "Blocked by existing yield delegation"
        );
        RebaseOptions stateFrom = rebaseState[_from];
        RebaseOptions stateTo = rebaseState[_to];

        require(
            stateFrom == RebaseOptions.NotSet ||
                stateFrom == RebaseOptions.StdNonRebasing ||
                stateFrom == RebaseOptions.StdRebasing,
            "Invalid rebaseState from"
        );

        require(
            stateTo == RebaseOptions.NotSet ||
                stateTo == RebaseOptions.StdNonRebasing ||
                stateTo == RebaseOptions.StdRebasing,
            "Invalid rebaseState to"
        );

        if (alternativeCreditsPerToken[_from] == 0) {
            _rebaseOptOut(_from);
        }
        if (alternativeCreditsPerToken[_to] > 0) {
            _rebaseOptIn(_to);
        }

        uint256 fromBalance = balanceOf(_from);
        uint256 toBalance = balanceOf(_to);
        uint256 oldToCredits = creditBalances[_to];
        uint256 newToCredits = _balanceToRebasingCredits(
            fromBalance + toBalance
        );

        // Set up the bidirectional links
        yieldTo[_from] = _to;
        yieldFrom[_to] = _from;

        // Local
        rebaseState[_from] = RebaseOptions.YieldDelegationSource;
        alternativeCreditsPerToken[_from] = 1e18;
        creditBalances[_from] = fromBalance;
        rebaseState[_to] = RebaseOptions.YieldDelegationTarget;
        creditBalances[_to] = newToCredits;

        // Global
        int256 creditsChange = newToCredits.toInt256() -
            oldToCredits.toInt256();
        _adjustGlobals(creditsChange, -(fromBalance).toInt256());
        emit YieldDelegated(_from, _to);
    }

    /*
     * @notice Stop sending the yield from one account to another account.
     */
    function undelegateYield(address _from) external onlyGovernorOrStrategist {
        // Require a delegation, which will also ensure a valid delegation
        require(yieldTo[_from] != address(0), "Zero address not allowed");

        address to = yieldTo[_from];
        uint256 fromBalance = balanceOf(_from);
        uint256 toBalance = balanceOf(to);
        uint256 oldToCredits = creditBalances[to];
        uint256 newToCredits = _balanceToRebasingCredits(toBalance);

        // Remove the bidirectional links
        yieldFrom[to] = address(0);
        yieldTo[_from] = address(0);

        // Local
        rebaseState[_from] = RebaseOptions.StdNonRebasing;
        // alternativeCreditsPerToken[from] already 1e18 from `delegateYield()`
        creditBalances[_from] = fromBalance;
        rebaseState[to] = RebaseOptions.StdRebasing;
        // alternativeCreditsPerToken[to] already 0 from `delegateYield()`
        creditBalances[to] = newToCredits;

        // Global
        int256 creditsChange = newToCredits.toInt256() -
            oldToCredits.toInt256();
        _adjustGlobals(creditsChange, fromBalance.toInt256());
        emit YieldUndelegated(_from, to);
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title Base for contracts that are managed by the Origin Protocol's Governor.
 * @dev Copy of the openzeppelin Ownable.sol contract with nomenclature change
 *      from owner to governor and renounce methods removed. Does not use
 *      Context.sol like Ownable.sol does for simplification.
 * @author Origin Protocol Inc
 */
abstract contract Governable {
    // Storage position of the owner and pendingOwner of the contract
    // keccak256("OUSD.governor");
    bytes32 private constant governorPosition =
        0x7bea13895fa79d2831e0a9e28edede30099005a50d652d8957cf8a607ee6ca4a;

    // keccak256("OUSD.pending.governor");
    bytes32 private constant pendingGovernorPosition =
        0x44c4d30b2eaad5130ad70c3ba6972730566f3e6359ab83e800d905c61b1c51db;

    // keccak256("OUSD.reentry.status");
    bytes32 private constant reentryStatusPosition =
        0x53bf423e48ed90e97d02ab0ebab13b2a235a6bfbe9c321847d5c175333ac4535;

    // See OpenZeppelin ReentrancyGuard implementation
    uint256 constant _NOT_ENTERED = 1;
    uint256 constant _ENTERED = 2;

    event PendingGovernorshipTransfer(
        address indexed previousGovernor,
        address indexed newGovernor
    );

    event GovernorshipTransferred(
        address indexed previousGovernor,
        address indexed newGovernor
    );

    /**
     * @notice Returns the address of the current Governor.
     */
    function governor() public view returns (address) {
        return _governor();
    }

    /**
     * @dev Returns the address of the current Governor.
     */
    function _governor() internal view returns (address governorOut) {
        bytes32 position = governorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            governorOut := sload(position)
        }
    }

    /**
     * @dev Returns the address of the pending Governor.
     */
    function _pendingGovernor()
        internal
        view
        returns (address pendingGovernor)
    {
        bytes32 position = pendingGovernorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            pendingGovernor := sload(position)
        }
    }

    /**
     * @dev Throws if called by any account other than the Governor.
     */
    modifier onlyGovernor() {
        require(isGovernor(), "Caller is not the Governor");
        _;
    }

    /**
     * @notice Returns true if the caller is the current Governor.
     */
    function isGovernor() public view returns (bool) {
        return msg.sender == _governor();
    }

    function _setGovernor(address newGovernor) internal {
        emit GovernorshipTransferred(_governor(), newGovernor);

        bytes32 position = governorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, newGovernor)
        }
    }

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and make it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        bytes32 position = reentryStatusPosition;
        uint256 _reentry_status;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _reentry_status := sload(position)
        }

        // On the first call to nonReentrant, _notEntered will be true
        require(_reentry_status != _ENTERED, "Reentrant call");

        // Any calls to nonReentrant after this point will fail
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, _ENTERED)
        }

        _;

        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, _NOT_ENTERED)
        }
    }

    function _setPendingGovernor(address newGovernor) internal {
        bytes32 position = pendingGovernorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, newGovernor)
        }
    }

    /**
     * @notice Transfers Governance of the contract to a new account (`newGovernor`).
     * Can only be called by the current Governor. Must be claimed for this to complete
     * @param _newGovernor Address of the new Governor
     */
    function transferGovernance(address _newGovernor) external onlyGovernor {
        _setPendingGovernor(_newGovernor);
        emit PendingGovernorshipTransfer(_governor(), _newGovernor);
    }

    /**
     * @notice Claim Governance of the contract to a new account (`newGovernor`).
     * Can only be called by the new Governor.
     */
    function claimGovernance() external {
        require(
            msg.sender == _pendingGovernor(),
            "Only the pending Governor can complete the claim"
        );
        _changeGovernor(msg.sender);
    }

    /**
     * @dev Change Governance of the contract to a new account (`newGovernor`).
     * @param _newGovernor Address of the new Governor
     */
    function _changeGovernor(address _newGovernor) internal {
        require(_newGovernor != address(0), "New Governor is address(0)");
        _setGovernor(_newGovernor);
    }
}

// SPDX-License-Identifier: MIT

// solhint-disable-next-line compiler-version
pragma solidity >=0.4.24 <0.8.0;

import "../utils/Address.sol";

/**
 * @dev This is a base contract to aid in writing upgradeable contracts, or any kind of contract that will be deployed
 * behind a proxy. Since a proxied contract can't have a constructor, it's common to move constructor logic to an
 * external initializer function, usually called `initialize`. It then becomes necessary to protect this initializer
 * function so it can only be called once. The {initializer} modifier provided by this contract will have this effect.
 *
 * TIP: To avoid leaving the proxy in an uninitialized state, the initializer function should be called as early as
 * possible by providing the encoded function call as the `_data` argument to {UpgradeableProxy-constructor}.
 *
 * CAUTION: When used with inheritance, manual care must be taken to not invoke a parent initializer twice, or to ensure
 * that all initializers are idempotent. This is not verified automatically as constructors are by Solidity.
 */
abstract contract Initializable {

    /**
     * @dev Indicates that the contract has been initialized.
     */
    bool private _initialized;

    /**
     * @dev Indicates that the contract is in the process of being initialized.
     */
    bool private _initializing;

    /**
     * @dev Modifier to protect an initializer function from being invoked twice.
     */
    modifier initializer() {
        require(_initializing || _isConstructor() || !_initialized, "Initializable: contract is already initialized");

        bool isTopLevelCall = !_initializing;
        if (isTopLevelCall) {
            _initializing = true;
            _initialized = true;
        }

        _;

        if (isTopLevelCall) {
            _initializing = false;
        }
    }

    /// @dev Returns true if and only if the function is running in the constructor
    function _isConstructor() private view returns (bool) {
        return !Address.isContract(address(this));
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IBasicToken } from "../interfaces/IBasicToken.sol";

library Helpers {
    /**
     * @notice Fetch the `symbol()` from an ERC20 token
     * @dev Grabs the `symbol()` from a contract
     * @param _token Address of the ERC20 token
     * @return string Symbol of the ERC20 token
     */
    function getSymbol(address _token) internal view returns (string memory) {
        string memory symbol = IBasicToken(_token).symbol();
        return symbol;
    }

    /**
     * @notice Fetch the `decimals()` from an ERC20 token
     * @dev Grabs the `decimals()` from a contract and fails if
     *      the decimal value does not live within a certain range
     * @param _token Address of the ERC20 token
     * @return uint256 Decimals of the ERC20 token
     */
    function getDecimals(address _token) internal view returns (uint256) {
        uint256 decimals = IBasicToken(_token).decimals();
        require(
            decimals >= 4 && decimals <= 18,
            "Token must have sufficient decimal places"
        );

        return decimals;
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title OToken VaultStorage contract
 * @notice The VaultStorage contract defines the storage for the Vault contracts
 * @author Origin Protocol Inc
 */

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { Address } from "@openzeppelin/contracts/utils/Address.sol";

import { IStrategy } from "../interfaces/IStrategy.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { Governable } from "../governance/Governable.sol";
import { OUSD } from "../token/OUSD.sol";
import { Initializable } from "../utils/Initializable.sol";
import "../utils/Helpers.sol";

abstract contract VaultStorage is Initializable, Governable {
    using SafeERC20 for IERC20;

    event AssetAllocated(address _asset, address _strategy, uint256 _amount);
    event StrategyApproved(address _addr);
    event StrategyRemoved(address _addr);
    event Mint(address _addr, uint256 _value);
    event Redeem(address _addr, uint256 _value);
    event CapitalPaused();
    event CapitalUnpaused();
    event DefaultStrategyUpdated(address _strategy);
    event RebasePaused();
    event RebaseUnpaused();
    event VaultBufferUpdated(uint256 _vaultBuffer);
    event AllocateThresholdUpdated(uint256 _threshold);
    event StrategistUpdated(address _address);
    event MaxSupplyDiffChanged(uint256 maxSupplyDiff);
    event YieldDistribution(address _to, uint256 _yield, uint256 _fee);
    event TrusteeFeeBpsChanged(uint256 _basis);
    event TrusteeAddressChanged(address _address);
    event StrategyAddedToMintWhitelist(address indexed strategy);
    event StrategyRemovedFromMintWhitelist(address indexed strategy);
    event RebasePerSecondMaxChanged(uint256 rebaseRatePerSecond);
    event DripDurationChanged(uint256 dripDuration);
    event OperatorUpdated(address newOperator);
    event WithdrawalRequested(
        address indexed _withdrawer,
        uint256 indexed _requestId,
        uint256 _amount,
        uint256 _queued
    );
    event WithdrawalClaimed(
        address indexed _withdrawer,
        uint256 indexed _requestId,
        uint256 _amount
    );
    event WithdrawalClaimable(uint256 _claimable, uint256 _newClaimable);
    event WithdrawalClaimDelayUpdated(uint256 _newDelay);

    // Since we are proxy, all state should be uninitalized.
    // Since this storage contract does not have logic directly on it
    // we should not be checking for to see if these variables can be constant.
    // slither-disable-start uninitialized-state
    // slither-disable-start constable-states

    /// @dev mapping of supported vault assets to their configuration
    uint256 private _deprecated_assets;
    /// @dev list of all assets supported by the vault.
    address[] private _deprecated_allAssets;

    // Strategies approved for use by the Vault
    struct Strategy {
        bool isSupported;
        uint256 _deprecated; // Deprecated storage slot
    }
    /// @dev mapping of strategy contracts to their configuration
    mapping(address => Strategy) public strategies;
    /// @dev list of all vault strategies
    address[] internal allStrategies;

    /// @notice Address of the Oracle price provider contract
    address private _deprecated_priceProvider;
    /// @notice pause rebasing if true
    bool public rebasePaused;
    /// @notice pause operations that change the OToken supply.
    /// eg mint, redeem, allocate, mint/burn for strategy
    bool public capitalPaused;
    /// @notice Redemption fee in basis points. eg 50 = 0.5%
    uint256 private _deprecated_redeemFeeBps;
    /// @notice Percentage of assets to keep in Vault to handle (most) withdrawals. 100% = 1e18.
    uint256 public vaultBuffer;
    /// @notice OToken mints over this amount automatically allocate funds. 18 decimals.
    uint256 public autoAllocateThreshold;
    /// @dev Deprecated. Was the auto-rebase trigger threshold for mint/redeem.
    ///      Storage slot retained for proxy compatibility; no longer read or written.
    uint256 internal __deprecatedRebaseThreshold;

    /// @dev Address of the OToken token. eg OUSD or OETH.
    OUSD public oToken;

    /// @dev Address of the contract responsible for post rebase syncs with AMMs
    address private _deprecated_rebaseHooksAddr = address(0);

    /// @dev Deprecated: Address of Uniswap
    address private _deprecated_uniswapAddr = address(0);

    /// @notice Address of the Strategist
    address public strategistAddr = address(0);

    /// @notice Mapping of asset address to the Strategy that they should automatically
    // be allocated to
    uint256 private _deprecated_assetDefaultStrategies;

    /// @notice Max difference between total supply and total value of assets. 18 decimals.
    uint256 public maxSupplyDiff;

    /// @notice Trustee contract that can collect a percentage of yield
    address public trusteeAddress;

    /// @notice Amount of yield collected in basis points. eg 2000 = 20%
    uint256 public trusteeFeeBps;

    /// @dev Deprecated: Tokens that should be swapped for stablecoins
    address[] private _deprecated_swapTokens;

    /// @notice Metapool strategy that is allowed to mint/burn OTokens without changing collateral

    address private _deprecated_ousdMetaStrategy;

    /// @notice How much OTokens are currently minted by the strategy
    int256 private _deprecated_netOusdMintedForStrategy;

    /// @notice How much net total OTokens are allowed to be minted by all strategies
    uint256 private _deprecated_netOusdMintForStrategyThreshold;

    uint256 private _deprecated_swapConfig;

    // List of strategies that can mint oTokens directly
    // Used in OETHBaseVaultCore
    mapping(address => bool) public isMintWhitelistedStrategy;

    /// @notice Address of the Dripper contract that streams harvested rewards to the Vault
    /// @dev The vault is proxied so needs to be set with setDripper against the proxy contract.
    address private _deprecated_dripper;

    /// Withdrawal Queue Storage /////

    struct WithdrawalQueueMetadata {
        // cumulative total of all withdrawal requests included the ones that have already been claimed
        uint128 queued;
        // cumulative total of all the requests that can be claimed including the ones that have already been claimed
        uint128 claimable;
        // total of all the requests that have been claimed
        uint128 claimed;
        // index of the next withdrawal request starting at 0
        uint128 nextWithdrawalIndex;
    }

    /// @notice Global metadata for the withdrawal queue including:
    /// queued - cumulative total of all withdrawal requests included the ones that have already been claimed
    /// claimable - cumulative total of all the requests that can be claimed including the ones already claimed
    /// claimed - total of all the requests that have been claimed
    /// nextWithdrawalIndex - index of the next withdrawal request starting at 0
    WithdrawalQueueMetadata public withdrawalQueueMetadata;

    struct WithdrawalRequest {
        address withdrawer;
        bool claimed;
        uint40 timestamp; // timestamp of the withdrawal request
        // Amount of oTokens to redeem. eg OETH
        uint128 amount;
        // cumulative total of all withdrawal requests including this one.
        // this request can be claimed when this queued amount is less than or equal to the queue's claimable amount.
        uint128 queued;
    }

    /// @notice Mapping of withdrawal request indices to the user withdrawal request data
    mapping(uint256 => WithdrawalRequest) public withdrawalRequests;

    /// @notice Sets a minimum delay that is required to elapse between
    ///     requesting async withdrawals and claiming the request.
    ///     When set to 0 async withdrawals are disabled.
    uint256 public withdrawalClaimDelay;

    /// @notice Time in seconds that the vault last rebased yield.
    uint64 public lastRebase;

    /// @notice Automatic rebase yield calculations. In seconds. Set to 0 or 1 to disable.
    uint64 public dripDuration;

    /// @notice max rebase percentage per second
    ///   Can be used to set maximum yield of the protocol,
    ///   spreading out yield over time
    uint64 public rebasePerSecondMax;

    /// @notice target rebase rate limit, based on past rates and funds available.
    uint64 public rebasePerSecondTarget;

    uint256 internal constant MAX_REBASE = 0.02 ether;
    uint256 internal constant MAX_REBASE_PER_SECOND =
        uint256(0.05 ether) / 1 days;

    /// @notice Default strategy for asset
    address public defaultStrategy;

    /// @notice Address authorized to call `rebase()` directly. The Governor
    ///         and Strategist are always allowed in addition to this address.
    address public operatorAddr;

    // For future use
    uint256[41] private __gap;

    /// @notice Index of WETH asset in allAssets array
    /// Legacy OETHVaultCore code, relocated here for vault consistency.
    uint256 private _deprecated_wethAssetIndex;

    /// @dev Address of the asset (eg. WETH or USDC)
    address public immutable asset;
    uint8 internal immutable assetDecimals;

    // slither-disable-end constable-states
    // slither-disable-end uninitialized-state

    constructor(address _asset) {
        uint8 _decimals = IERC20Metadata(_asset).decimals();
        require(_decimals <= 18, "invalid asset decimals");
        asset = _asset;
        assetDecimals = _decimals;
    }

    /// @notice Deprecated: use `oToken()` instead.
    function oUSD() external view returns (OUSD) {
        return oToken;
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IBasicToken } from "../interfaces/IBasicToken.sol";

library Helpers {
    /**
     * @notice Fetch the `symbol()` from an ERC20 token
     * @dev Grabs the `symbol()` from a contract
     * @param _token Address of the ERC20 token
     * @return string Symbol of the ERC20 token
     */
    function getSymbol(address _token) internal view returns (string memory) {
        string memory symbol = IBasicToken(_token).symbol();
        return symbol;
    }

    /**
     * @notice Fetch the `decimals()` from an ERC20 token
     * @dev Grabs the `decimals()` from a contract and fails if
     *      the decimal value does not live within a certain range
     * @param _token Address of the ERC20 token
     * @return uint256 Decimals of the ERC20 token
     */
    function getDecimals(address _token) internal view returns (uint256) {
        uint256 decimals = IBasicToken(_token).decimals();
        require(
            decimals >= 4 && decimals <= 18,
            "Token must have sufficient decimal places"
        );

        return decimals;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IBasicToken {
    function symbol() external view returns (string memory);

    function decimals() external view returns (uint8);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { VaultStorage } from "../vault/VaultStorage.sol";

interface IVault {
    // slither-disable-start constable-states

    event AssetAllocated(address _asset, address _strategy, uint256 _amount);
    event StrategyApproved(address _addr);
    event StrategyRemoved(address _addr);
    event Mint(address _addr, uint256 _value);
    event Redeem(address _addr, uint256 _value);
    event CapitalPaused();
    event CapitalUnpaused();
    event DefaultStrategyUpdated(address _strategy);
    event RebasePaused();
    event RebaseUnpaused();
    event VaultBufferUpdated(uint256 _vaultBuffer);
    event AllocateThresholdUpdated(uint256 _threshold);
    event StrategistUpdated(address _address);
    event MaxSupplyDiffChanged(uint256 maxSupplyDiff);
    event YieldDistribution(address _to, uint256 _yield, uint256 _fee);
    event TrusteeFeeBpsChanged(uint256 _basis);
    event TrusteeAddressChanged(address _address);
    event StrategyAddedToMintWhitelist(address indexed strategy);
    event StrategyRemovedFromMintWhitelist(address indexed strategy);
    event RebasePerSecondMaxChanged(uint256 rebaseRatePerSecond);
    event DripDurationChanged(uint256 dripDuration);
    event WithdrawalRequested(
        address indexed _withdrawer,
        uint256 indexed _requestId,
        uint256 _amount,
        uint256 _queued
    );
    event WithdrawalClaimed(
        address indexed _withdrawer,
        uint256 indexed _requestId,
        uint256 _amount
    );
    event WithdrawalClaimable(uint256 _claimable, uint256 _newClaimable);
    event WithdrawalClaimDelayUpdated(uint256 _newDelay);

    // Governable.sol
    function transferGovernance(address _newGovernor) external;

    function claimGovernance() external;

    function governor() external view returns (address);

    // VaultAdmin.sol
    function setVaultBuffer(uint256 _vaultBuffer) external;

    function vaultBuffer() external view returns (uint256);

    function setAutoAllocateThreshold(uint256 _threshold) external;

    function autoAllocateThreshold() external view returns (uint256);

    function setStrategistAddr(address _address) external;

    function strategistAddr() external view returns (address);

    function setOperatorAddr(address _operator) external;

    function operatorAddr() external view returns (address);

    function setMaxSupplyDiff(uint256 _maxSupplyDiff) external;

    function maxSupplyDiff() external view returns (uint256);

    function setTrusteeAddress(address _address) external;

    function trusteeAddress() external view returns (address);

    function setTrusteeFeeBps(uint256 _basis) external;

    function trusteeFeeBps() external view returns (uint256);

    function approveStrategy(address _addr) external;

    function removeStrategy(address _addr) external;

    function setDefaultStrategy(address _strategy) external;

    function defaultStrategy() external view returns (address);

    function pauseRebase() external;

    function unpauseRebase() external;

    function rebasePaused() external view returns (bool);

    function pauseCapital() external;

    function unpauseCapital() external;

    function capitalPaused() external view returns (bool);

    function transferToken(address _asset, uint256 _amount) external;

    function withdrawAllFromStrategy(address _strategyAddr) external;

    function withdrawAllFromStrategies() external;

    function withdrawFromStrategy(
        address _strategyFromAddress,
        address[] calldata _assets,
        uint256[] calldata _amounts
    ) external;

    function depositToStrategy(
        address _strategyToAddress,
        address[] calldata _assets,
        uint256[] calldata _amounts
    ) external;

    // VaultCore.sol
    function mint(uint256 _amount) external;

    function mintForStrategy(uint256 _amount) external;

    function burnForStrategy(uint256 _amount) external;

    function allocate() external;

    function rebase() external;

    function totalValue() external view returns (uint256 value);

    function checkBalance(address _asset) external view returns (uint256);

    function getAssetCount() external view returns (uint256);

    function getAllAssets() external view returns (address[] memory);

    function getStrategyCount() external view returns (uint256);

    function getAllStrategies() external view returns (address[] memory);

    function strategies(address _addr)
        external
        view
        returns (VaultStorage.Strategy memory);

    /// @notice Deprecated: use `asset()` instead.
    function isSupportedAsset(address _asset) external view returns (bool);

    function asset() external view returns (address);

    function oToken() external view returns (address);

    function initialize(address) external;

    function addWithdrawalQueueLiquidity() external;

    function requestWithdrawal(uint256 _amount)
        external
        returns (uint256 requestId, uint256 queued);

    function claimWithdrawal(uint256 requestId)
        external
        returns (uint256 amount);

    function claimWithdrawals(uint256[] memory requestIds)
        external
        returns (uint256[] memory amounts, uint256 totalAmount);

    function withdrawalQueueMetadata()
        external
        view
        returns (VaultStorage.WithdrawalQueueMetadata memory);

    function withdrawalRequests(uint256 requestId)
        external
        view
        returns (VaultStorage.WithdrawalRequest memory);

    function addStrategyToMintWhitelist(address strategyAddr) external;

    function removeStrategyFromMintWhitelist(address strategyAddr) external;

    function isMintWhitelistedStrategy(address strategyAddr)
        external
        view
        returns (bool);

    function withdrawalClaimDelay() external view returns (uint256);

    function setWithdrawalClaimDelay(uint256 newDelay) external;

    function lastRebase() external view returns (uint64);

    function dripDuration() external view returns (uint64);

    function setDripDuration(uint256 _dripDuration) external;

    function rebasePerSecondMax() external view returns (uint64);

    function setRebaseRateMax(uint256 yearlyApr) external;

    function rebasePerSecondTarget() external view returns (uint64);

    function previewYield() external view returns (uint256 yield);

    // slither-disable-end constable-states
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title Platform interface to integrate with lending platform like Compound, AAVE etc.
 */
interface IStrategy {
    /**
     * @dev Deposit the given asset to platform
     * @param _asset asset address
     * @param _amount Amount to deposit
     */
    function deposit(address _asset, uint256 _amount) external;

    /**
     * @dev Deposit the entire balance of all supported assets in the Strategy
     *      to the platform
     */
    function depositAll() external;

    /**
     * @dev Withdraw given asset from Lending platform
     */
    function withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) external;

    /**
     * @dev Liquidate all assets in strategy and return them to Vault.
     */
    function withdrawAll() external;

    /**
     * @dev Returns the current balance of the given asset.
     */
    function checkBalance(address _asset)
        external
        view
        returns (uint256 balance);

    /**
     * @dev Returns bool indicating whether strategy supports asset.
     */
    function supportsAsset(address _asset) external view returns (bool);

    /**
     * @dev Collect reward tokens from the Strategy.
     */
    function collectRewardTokens() external;

    /**
     * @dev The address array of the reward tokens for the Strategy.
     */
    function getRewardTokenAddresses() external view returns (address[] memory);

    function harvesterAddress() external view returns (address);

    function transferToken(address token, uint256 amount) external;

    function setRewardTokenAddresses(address[] calldata _rewardTokenAddresses)
        external;
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IVault } from "./IVault.sol";

interface IMockVault is IVault {
    function outstandingWithdrawalsAmount() external view returns (uint256);

    function wethAvailable() external view returns (uint256);
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

