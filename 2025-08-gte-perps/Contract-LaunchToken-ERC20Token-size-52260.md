
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {ERC20} from "@solady/tokens/ERC20.sol";

import {ILaunchpad} from "./interfaces/ILaunchpad.sol";

contract LaunchToken is ERC20 {
    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                         ERRORS AND EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0xd386ef3e
    error BadAuth();
    /// @dev sig: 0x9ca33913
    error TransfersDisabledWhileBonding();
    /// @dev sig: 0xe97e187c
    error TotalSupplyExceedsMaxShares();

    /// @dev event-sig:
    event TransfersUnlocked(uint256 timestamp, uint256 eventNonce);
    event FeeShareIncreased(address indexed account, uint256 amount, uint256 eventNonce);
    event FeeShareDecreased(address indexed account, uint256 amount, uint256 eventNonce);
    event FeeShareConcluded(uint256 timestamp, uint256 eventNonce);

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            CUSTOM STATE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev The abi version of this impl so the indexer can handle event-changing upgrades
    uint256 public constant ABI_VERSION = 1;

    address public immutable launchpad;
    address public immutable gteRouter;
    string private _name;
    string private _symbol;
    string private _mediaURI;

    bool public unlocked;
    uint256 public eventNonce;
    uint256 public totalFeeShare;
    mapping(address => uint256) public bondingShare;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                             CONSTRUCTOR
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    // slither-disable-next-line missing-zero-check
    constructor(string memory name_, string memory symbol_, string memory mediaUri_, address gteRouter_) {
        _name = name_;
        _symbol = symbol_;
        _mediaURI = mediaUri_;
        gteRouter = gteRouter_;
        launchpad = msg.sender;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                MODIFIERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    modifier onlyLaunchpad() {
        if (msg.sender != launchpad) revert BadAuth();
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            PUBLIC VIEWS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Implements solady token name
    function name() public view override returns (string memory) {
        return _name;
    }

    /// @notice Implements solady token symbol
    function symbol() public view override returns (string memory) {
        return _symbol;
    }

    /// @notice Additional data field for token image
    function mediaURI() public view returns (string memory) {
        return _mediaURI;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              OWNER-ONLY
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Entrypoint for launchpad (deployer) to unlock transfers
    function unlock() external onlyLaunchpad {
        unlocked = true;
        emit TransfersUnlocked(block.timestamp, _incEventNonce());
    }

    /// @notice Entrypoint for launchpad to mint token (launchpad only calls once)
    function mint(uint256 amount) external onlyLaunchpad {
        _mint(launchpad, amount);

        if (totalSupply() > type(uint96).max) revert TotalSupplyExceedsMaxShares();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            INTERNAL ASSERTIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _beforeTokenTransfer(address from, address to, uint256 amount) internal override {
        if (!unlocked && from != launchpad && to != launchpad && to != gteRouter) {
            revert TransfersDisabledWhileBonding();
        }

        if (!unlocked) {
            if (from != launchpad && to != launchpad && to != gteRouter) revert TransfersDisabledWhileBonding();

            if (from == launchpad && to != launchpad) _increaseFeeShares(to, amount);
            else if (to != launchpad && to != gteRouter) revert TransfersDisabledWhileBonding();
        }

        if (from != launchpad) _decreaseFeeShares(from, amount);
    }

    function _increaseFeeShares(address account, uint256 amount) internal {
        if (amount == 0 || account == address(0)) return;

        emit FeeShareIncreased(account, amount, _incEventNonce());

        unchecked {
            totalFeeShare += amount;
            bondingShare[account] += amount;
        }

        ILaunchpad(launchpad).increaseStake(account, uint96(amount));
    }

    function _decreaseFeeShares(address account, uint256 amount) internal {
        uint256 share = bondingShare[account];
        if (share == 0 || account == address(0)) return;

        amount = amount > share ? share : amount;

        emit FeeShareDecreased(account, amount, _incEventNonce());

        unchecked {
            totalFeeShare -= amount;
            bondingShare[account] -= amount;
        }

        if (totalFeeShare == 0 && !unlocked) _endRewards();

        ILaunchpad(launchpad).decreaseStake(account, uint96(amount));
    }

    /// @dev Hook to end rewards program for this base token if no more pre-bonding shares exist
    function _endRewards() internal {
        ILaunchpad(launchpad).endRewards();

        emit FeeShareConcluded(block.timestamp, _incEventNonce());
    }

    function _incEventNonce() internal returns (uint256 nonce) {
        nonce = eventNonce;
        eventNonce++;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.27;

import {Ownable2StepUpgradeable} from "@openzeppelin-contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

contract LaunchpadLPVault is Ownable2StepUpgradeable {
    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0xaf62991d
    error FallbackRevert();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                STATES
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    address public launchpad;

    /// @dev The abi version of this impl so the indexer can handle event-changing upgrades
    uint256 public constant ABI_VERSION = 1;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                    CONSTRUCTOR AND INITIALIZATION
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    constructor() {
        _disableInitializers();
    }

    function initialize(address launchpad_, address initialOwner) external initializer {
        launchpad = launchpad_;
        __Ownable_init(initialOwner);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                FALLBACKS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    fallback() external {
        revert FallbackRevert();
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.1.0) (access/Ownable2Step.sol)

pragma solidity ^0.8.20;

import {OwnableUpgradeable} from "./OwnableUpgradeable.sol";
import {Initializable} from "../proxy/utils/Initializable.sol";

/**
 * @dev Contract module which provides access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * This extension of the {Ownable} contract includes a two-step mechanism to transfer
 * ownership, where the new owner must call {acceptOwnership} in order to replace the
 * old one. This can help prevent common mistakes, such as transfers of ownership to
 * incorrect accounts, or to contracts that are unable to interact with the
 * permission system.
 *
 * The initial owner is specified at deployment time in the constructor for `Ownable`. This
 * can later be changed with {transferOwnership} and {acceptOwnership}.
 *
 * This module is used through inheritance. It will make available all functions
 * from parent (Ownable).
 */
abstract contract Ownable2StepUpgradeable is Initializable, OwnableUpgradeable {
    /// @custom:storage-location erc7201:openzeppelin.storage.Ownable2Step
    struct Ownable2StepStorage {
        address _pendingOwner;
    }

    // keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.Ownable2Step")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant Ownable2StepStorageLocation = 0x237e158222e3e6968b72b9db0d8043aacf074ad9f650f0d1606b4d82ee432c00;

    function _getOwnable2StepStorage() private pure returns (Ownable2StepStorage storage $) {
        assembly {
            $.slot := Ownable2StepStorageLocation
        }
    }

    event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);

    function __Ownable2Step_init() internal onlyInitializing {
    }

    function __Ownable2Step_init_unchained() internal onlyInitializing {
    }
    /**
     * @dev Returns the address of the pending owner.
     */
    function pendingOwner() public view virtual returns (address) {
        Ownable2StepStorage storage $ = _getOwnable2StepStorage();
        return $._pendingOwner;
    }

    /**
     * @dev Starts the ownership transfer of the contract to a new account. Replaces the pending transfer if there is one.
     * Can only be called by the current owner.
     *
     * Setting `newOwner` to the zero address is allowed; this can be used to cancel an initiated ownership transfer.
     */
    function transferOwnership(address newOwner) public virtual override onlyOwner {
        Ownable2StepStorage storage $ = _getOwnable2StepStorage();
        $._pendingOwner = newOwner;
        emit OwnershipTransferStarted(owner(), newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`) and deletes any pending owner.
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual override {
        Ownable2StepStorage storage $ = _getOwnable2StepStorage();
        delete $._pendingOwner;
        super._transferOwnership(newOwner);
    }

    /**
     * @dev The new owner accepts the ownership transfer.
     */
    function acceptOwnership() public virtual {
        address sender = _msgSender();
        if (pendingOwner() != sender) {
            revert OwnableUnauthorizedAccount(sender);
        }
        _transferOwnership(sender);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {LaunchToken} from "../LaunchToken.sol";
import {IBondingCurveMinimal} from "../BondingCurves/IBondingCurveMinimal.sol";
import {ICLOBManager} from "../../clob/ICLOBManager.sol";
import {IOperatorPanel} from "../../utils/interfaces/IOperatorPanel.sol";
import {IDistributor} from "./IDistributor.sol";
import {IUniswapV2RouterMinimal} from "./IUniswapV2RouterMinimal.sol";
import {IUniswapV2FactoryMinimal} from "./IUniswapV2FactoryMinimal.sol";
import {LaunchpadLPVault} from "../LaunchpadLPVault.sol";

interface ILaunchpad {
    struct BuyData {
        address account;
        address token;
        address recipient;
        uint256 amountOutBase;
        uint256 maxAmountInQuote;
    }

    function quoteBaseForQuote(address token, uint256 quoteAmount, bool isBuy)
        external
        view
        returns (uint256 baseAmount);
    function quoteQuoteForBase(address token, uint256 baseAmount, bool isBuy)
        external
        view
        returns (uint256 quoteAmount);

    struct LaunchData {
        bool active;
        address quote;
        IBondingCurveMinimal curve;
    }

    function launch(string memory name, string memory symbol, string memory mediaURI)
        external
        payable
        returns (address token);

    function buy(BuyData calldata buyData)
        external
        returns (uint256 amountOutBaseActual, uint256 amountInQuoteActual);

    function sell(address account, address token, address recipient, uint256 amountInBase, uint256 minAmountOutQuote)
        external
        returns (uint256 amountInBaseActual, uint256 amountOutQuoteActual);

    function increaseStake(address account, uint96 shares) external;

    function decreaseStake(address account, uint96 shares) external;
    
    function endRewards() external;

    function updateBondingCurve(address newBondingCurve) external;

    function pullFees() external;

    // slither-disable-next-line naming-convention
    function TOTAL_SUPPLY() external view returns (uint256);

    // slither-disable-next-line naming-convention
    function BONDING_SUPPLY() external view returns (uint256);

    // slither-disable-next-line naming-convention
    function ABI_VERSION() external view returns (uint256);

    function gteRouter() external view returns (address);

    function operator() external view returns (IOperatorPanel);

    function distributor() external view returns (IDistributor);

    function uniV2Router() external view returns (IUniswapV2RouterMinimal);

    function launchpadLPVault() external view returns (LaunchpadLPVault);

    function currentQuoteAsset() external view returns (LaunchToken);

    function currentBondingCurve() external view returns (IBondingCurveMinimal);

    function launchFee() external view returns (uint256);

    function eventNonce() external view returns (uint256);

    function launches(address launchToken) external view returns (LaunchData memory);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IAccountManager} from "../account-manager/IAccountManager.sol";
import {FeeTiers} from "./types/FeeData.sol";
import {ICLOB} from "./ICLOB.sol";
import {Side, OrderId} from "./types/Order.sol";
import {MakerCredit} from "./types/TransientMakerData.sol";

struct ConfigParams {
    address quoteToken;
    address baseToken;
    uint256 quoteSize;
    uint256 baseSize;
}

struct SettingsParams {
    address owner;
    uint8 maxLimitsPerTx;
    uint256 minLimitOrderAmountInBase;
    uint256 tickSize;
    uint256 lotSizeInBase;
}

interface ICLOBManager {
    // Basic getters from ICLOBAdminPanel
    function beacon() external view returns (address);
    function getMarketAddress(address quoteToken, address baseToken) external view returns (address);
    function isMarket(address market) external view returns (bool);

    // Market creation and management from ICLOBAdminPanel
    function createMarket(address baseToken, address quoteToken, SettingsParams calldata settings)
        external
        returns (address marketAddress);

    // Limit management getters
    function getMaxLimitExempt(address account) external view returns (bool);

    // Admin settings
    function setMaxLimitsPerTx(ICLOB market, uint8 newMaxLimits) external;
    function setTickSize(ICLOB market, uint256 newTickSize) external;
    function setLotSizeInBase(ICLOB market, uint256 newLotSize) external;
    function setMinLimitOrderAmountInBase(ICLOB market, uint256 newMinLimitOrderAmountInBase) external;
    function adminCancelExpiredOrders(ICLOB market, OrderId[] calldata ids, Side side) external;
    function setAccountFeeTiers(address[] calldata accounts, FeeTiers[] calldata feeTiers) external;
    function setMaxLimitsExempt(address[] calldata accounts, bool[] calldata toggles) external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.27;

import {IERC165} from "@openzeppelin/interfaces/IERC165.sol";

interface IBondingCurveMinimal is IERC165 {
    function init(bytes memory data) external;

    function initializeCurve(address token, uint256 totalSupply, uint256 bondingSupply) external;
    function buy(address token, uint256 baseAmount) external returns (uint256 quoteAmount);
    function sell(address token, uint256 baseAmount) external returns (uint256 quoteAmount);

    function quoteBaseForQuote(address token, uint256 quoteAmount, bool isBuy)
        external
        view
        returns (uint256 baseAmount);
    function quoteQuoteForBase(address token, uint256 baseAmount, bool isBuy)
        external
        view
        returns (uint256 quoteAmount);
    function baseSoldFromCurve(address token) external view returns (uint256);
    function quoteBoughtByCurve(address token) external view returns (uint256);
    function totalSupply(address token) external view returns (uint256);
    function bondingSupply(address token) external view returns (uint256);

    function supportsInterface(bytes4 interfaceId) external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

interface IOperatorPanel {
    function approveOperator(address account, address operator, uint256 roles) external;
    function disapproveOperator(address account, address operator, uint256 roles) external;
    function getOperatorRoleApprovals(address account, address operator) external view returns (uint256);
    function getOperatorEventNonce() external view returns (uint256);
}

pragma solidity 0.8.27;

import {IGTELaunchpadV2Pair} from "../uniswap/interfaces/IGTELaunchpadV2Pair.sol";

import {UserRewardData, RewardPoolDataMemory} from "../libraries/RewardsTracker.sol";

interface IDistributor {
    function getUserData(address launchAsset, address account) external view returns (UserRewardData memory);
    function getUserDataForTokens(address[] calldata launchAssets, address account)
        external
        view
        returns (UserRewardData[] memory);
    function increaseStake(address launchAsset, address account, uint96 shares)
        external
        returns (uint256 baseAmount, uint256 quoteAmount);
    function decreaseStake(address launchAsset, address account, uint96 shares)
        external
        returns (uint256 baseAmount, uint256 quoteAmount);
    function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount);
    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external;
    function createRewardsPair(address launchAsset, address quoteToken) external;

    function endRewards(IGTELaunchpadV2Pair pair) external;
}

pragma solidity 0.8.27;

interface IUniswapV2FactoryMinimal {
    function createPair(address tokenA, address tokenB) external returns (address pair);
    function getPair(address tokenA, address tokenB) external view returns (address pair);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

interface IUniswapV2RouterMinimal {
    function factory() external view returns (address);

    function addLiquidity(
        address tokenA,
        address tokenB,
        uint256 amountADesired,
        uint256 amountBDesired,
        uint256 amountAMin,
        uint256 amountBMin,
        address to,
        uint256 deadline
    ) external returns (uint256 amountA, uint256 amountB, uint256 liquidity);

    function swapTokensForExactTokens(
        uint256 amountOut,
        uint256 amountInMax,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);

    function getAmountIn(uint256 amountOut, uint256 reserveIn, uint256 reserveOut)
        external
        pure
        returns (uint256 amountIn);
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

// import {Initializable} from "@solady/utils/Initializable.sol";
import {OwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";

import {IDistributor, IGTELaunchpadV2Pair} from "./interfaces/IDistributor.sol";
import "./libraries/RewardsTracker.sol";

/// @notice This contract receives tokens from
contract Distributor is OwnableRoles, IDistributor {
    using SafeTransferLib for address;

    event TotalPendingRewardsIncreased(address indexed asset, uint256 amount);
    event TotalPendingRewardsDecreased(address indexed asset, uint256 amount);

    error Initialized();
    error RewardsExist();
    error RewardsDoNotExist();
    error ClaimAmountExceedsTotalPendingRewards();
    error NoSharesToIncentivize();
    error SkimOverflow();

    uint256 public constant ADMIN_ROLE = _ROLE_0;

    bool private initialized;
    address public launchpad;

    /// @dev metadata to recover donations while preserving pending rewards
    mapping(address => uint256) public totalPendingRewards;

    constructor() {
        _initializeOwner(msg.sender);
    }

    /// @dev There is no init check anywhere else because this contract can't be used until the launchpad address is set
    function initialize(address _launchpad) public onlyOwner {
        if (initialized) revert Initialized();
        launchpad = _launchpad;
        initialized = true;
    }

    modifier onlyLaunchpad() {
        if (msg.sender != launchpad) revert Unauthorized();
        _;
    }

    function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
        if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();

        asset.safeTransfer(msg.sender, amount);
    }

    function getRewardsPoolData(address launchAsset) external view returns (RewardPoolDataMemory memory) {
        return RewardsTrackerStorage.getRewardPool(launchAsset).getRewardsPoolData();
    }

    function getUserData(address launchAsset, address account) external view returns (UserRewardData memory) {
        return RewardsTrackerStorage.getRewardPool(launchAsset).getUserData(account);
    }

    function getUserDataForTokens(address[] calldata launchAssets, address account)
        external
        view
        returns (UserRewardData[] memory)
    {
        UserRewardData[] memory data = new UserRewardData[](launchAssets.length);

        for (uint256 i = 0; i < launchAssets.length; i++) {
            data[i] = RewardsTrackerStorage.getRewardPool(launchAssets[i]).getUserData(account);
        }
        return data;
    }

    function getPendingRewards(address launchAsset, address account)
        external
        view
        returns (uint256 pendingBase, uint256 pendingQuote)
    {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

        return rs.getPendingRewards(account);
    }

    function endRewards(IGTELaunchpadV2Pair pair) external onlyLaunchpad {
        pair.endRewardsAccrual();
    }

    /// @notice Initializes the rewards pair from the launchpad
    /// @dev Neither the launchAsset, nor the quoteAsset can be the baseAsset of an existing reward pool
    function createRewardsPair(address launchAsset, address quoteAsset) external onlyLaunchpad {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
        RewardPoolData storage rsq = RewardsTrackerStorage.getRewardPool(quoteAsset);

        // Sanity check in case the admin makes the quote asset of launchpad an existing asset
        if (rs.quoteAsset != address(0) || rsq.quoteAsset != address(0)) revert RewardsExist();

        rs.initializePair(launchAsset, quoteAsset);
    }

    /// @notice Allows rewards to be added to a pool regardless of token order
    /// @dev Pools can only be created once per asset combo, regardless of the order of the assets
    /// Additionally, anyone can add rewards as incentive, even while a pair is still bonding
    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
        (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) =
            (token0, token1, amount0, amount1);
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);

        if (rs.quoteAsset == address(0)) {
            rs = RewardsTrackerStorage.getRewardPool(token1);

            if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();

            (launchAsset, quoteAsset, launchAssetAmount, quoteAssetAmount) = (token1, token0, amount1, amount0);
        }

        if (rs.totalShares == 0) revert NoSharesToIncentivize();

        if (launchAssetAmount > 0) {
            rs.addBaseRewards(launchAsset, launchAssetAmount);
            _increaseTotalPending(launchAsset, launchAssetAmount);
            launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
        }

        if (quoteAssetAmount > 0) {
            rs.addQuoteRewards(launchAsset, quoteAsset, quoteAssetAmount);
            _increaseTotalPending(quoteAsset, quoteAssetAmount);
            quoteAsset.safeTransferFrom(msg.sender, address(this), uint256(quoteAssetAmount));
        }
    }

    /// @dev This can only be called while `launchAsset` is bonding, so we dont need to check if the pool exists or is still active
    function increaseStake(address launchAsset, address account, uint96 shares)
        external
        onlyLaunchpad
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

        (baseAmount, quoteAmount) = rs.stake(account, uint96(shares));
        _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
    }

    /// @dev This can only be called while `launchAsset` still shares to remove from bonders, so it cannot be called after the pool has been deactivated
    function decreaseStake(address launchAsset, address account, uint96 shares)
        external
        onlyLaunchpad
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

        (baseAmount, quoteAmount) = rs.unstake(account, uint96(shares));
        _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
    }

    /// @dev This can be called even after a pool has been deactivated, as accounts may still have pending rewards
    function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount) {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

        (baseAmount, quoteAmount) = rs.claim(msg.sender);

        _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
    }

    function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
        if (baseAmount > 0) {
            _decreaseTotalPending(base, baseAmount);
            base.safeTransfer(msg.sender, baseAmount);
        }

        if (quoteAmount > 0) {
            _decreaseTotalPending(quote, quoteAmount);
            quote.safeTransfer(msg.sender, quoteAmount);
        }
    }

    function _increaseTotalPending(address asset, uint256 amount) internal {
        unchecked {
            totalPendingRewards[asset] += amount;
        }

        emit TotalPendingRewardsIncreased(asset, amount);
    }

    function _decreaseTotalPending(address asset, uint256 amount) internal {
        uint256 currTotal = totalPendingRewards[asset];

        if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();

        unchecked {
            totalPendingRewards[asset] -= amount;
        }

        emit TotalPendingRewardsDecreased(asset, amount);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.27;

import {IERC165} from "@openzeppelin/interfaces/IERC165.sol";
import {IBondingCurveMinimal} from "./IBondingCurveMinimal.sol";
import {Ownable} from "@solady/auth/Ownable.sol";

contract SimpleBondingCurve is IBondingCurveMinimal {
    struct Reserves {
        uint256 quoteReserve;
        uint256 baseReserve;
    }

    struct Supply {
        uint256 totalSupply;
        uint256 bondingSupply;
    }

    /// @dev sig: 0x811abebed4bd76417e15038991a2a59847b86a0ece32d4dcc5c37f7641f0580d
    event VirtualReservesSet(uint256 virtualBase, uint256 virtualQuote);
    /// @dev sig: 0xf1dc3d06c4e72b9153d6aba8efebafe8d45e438daab475fa1410c907c526d98b
    event ReservesSet(address indexed token, uint256 quoteReserve, uint256 baseReserve);
    /// @dev sig: 0x1b77ab811805ecfa41dda62047dae05b29f779f97d4735116c794a5c1a050cf0
    event NewTokenLaunched(address indexed token, uint256 virtualBase, uint256 virtualQuote);

    /// @dev can only be set once, at initialization; see {Launchpad.sol}::line_162
    uint256 public VIRTUAL_BASE; // can be customized to change curve
    uint256 public VIRTUAL_QUOTE; // can be customized to change curve

    mapping(address token => Reserves) internal reserves;
    mapping(address token => Supply) internal supply;

    address public immutable launchpad;

    /// @dev sig:0xed6fcad9
    error NotLaunchpad();
    /// @dev sig: 0x8447642d
    error NotLaunchpadOwner();
    /// @dev sig: 0x56965ca0
    error InvalidVirtualBase();
    /// @dev sig: 0xad5eefd0
    error InvalidVirtualQuote();

    constructor(address launchpad_) {
        launchpad = launchpad_;
    }

    modifier onlyLaunchpad() {
        if (msg.sender != launchpad) revert NotLaunchpad();
        _;
    }

    modifier onlyLaunchpadOwner() {
        if (msg.sender != Ownable(launchpad).owner()) revert NotLaunchpadOwner();
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                NEW CURVE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev {SimpleBondingCurve} awaits an ABI-encoded (virtualBase, virtualQuote) tuple
    function init(bytes memory data) external onlyLaunchpad {
        (uint256 virtualBase, uint256 virtualQuote) = abi.decode(data, (uint256, uint256));

        _setVirtualReserves(virtualBase, virtualQuote);
    }

    /// @dev other kinds of curves might require more than just setting the reserves for that token curve's launch
    function initializeCurve(address token, uint256 totalSupply_, uint256 bondingSupply_) external onlyLaunchpad {
        _setReserves(token, VIRTUAL_QUOTE, bondingSupply_ + VIRTUAL_BASE);
        _setSupply(token, totalSupply_, bondingSupply_);

        emit NewTokenLaunched(token, VIRTUAL_BASE, VIRTUAL_QUOTE);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                SETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev deprecated
    function setReserves(address token, uint256 quoteReserve, uint256 baseReserve) external onlyLaunchpadOwner {
        _setReserves(token, quoteReserve, baseReserve);
    }

    /// @dev deprecated
    function setVirtualReserves(uint256 virtualBase, uint256 virtualQuote) external onlyLaunchpadOwner {
        if (virtualBase == 0) revert InvalidVirtualBase();
        if (virtualQuote == 0) revert InvalidVirtualQuote();

        _setVirtualReserves(virtualBase, virtualQuote);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            TRADING LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function buy(address token, uint256 baseAmount) external onlyLaunchpad returns (uint256 quoteAmount) {
        Reserves storage r = reserves[token];

        quoteAmount = _getQuoteAmount(baseAmount, r.quoteReserve, r.baseReserve, true);

        r.quoteReserve += quoteAmount;
        r.baseReserve -= baseAmount;
    }

    function sell(address token, uint256 baseAmount) external onlyLaunchpad returns (uint256 quoteAmount) {
        Reserves storage r = reserves[token];

        quoteAmount = _getQuoteAmount(baseAmount, r.quoteReserve, r.baseReserve, false);

        r.quoteReserve -= quoteAmount;
        r.baseReserve += baseAmount;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function bondingSupply(address token) external view returns (uint256) {
        return supply[token].bondingSupply;
    }

    function totalSupply(address token) external view returns (uint256) {
        return supply[token].totalSupply;
    }

    function baseSoldFromCurve(address token) external view returns (uint256) {
        return (supply[token].bondingSupply + VIRTUAL_BASE) - reserves[token].baseReserve;
    }

    function quoteBoughtByCurve(address token) external view returns (uint256) {
        return reserves[token].quoteReserve - VIRTUAL_QUOTE;
    }

    function getReserves(address token) external view returns (uint256 quoteReserve, uint256 baseReserve) {
        Reserves storage r = reserves[token];
        quoteReserve = r.quoteReserve;
        baseReserve = r.baseReserve;
    }

    function quoteBaseForQuote(address token, uint256 quoteAmount, bool isBuy)
        external
        view
        returns (uint256 baseAmount)
    {
        Reserves storage r = reserves[token];
        baseAmount = _getBaseAmount(quoteAmount, r.quoteReserve, r.baseReserve, isBuy);
    }

    function quoteQuoteForBase(address token, uint256 baseAmount, bool isBuy)
        external
        view
        returns (uint256 quoteAmount)
    {
        Reserves storage r = reserves[token];
        quoteAmount = _getQuoteAmount(baseAmount, r.quoteReserve, r.baseReserve, isBuy);
    }

    function supportsInterface(bytes4 interfaceId) external pure returns (bool) {
        return interfaceId == type(IERC165).interfaceId || interfaceId == type(IBondingCurveMinimal).interfaceId;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _setReserves(address token, uint256 quoteReserve, uint256 baseReserve) internal {
        Reserves storage r = reserves[token];
        r.quoteReserve = quoteReserve;
        r.baseReserve = baseReserve;

        emit ReservesSet(token, quoteReserve, baseReserve);
    }

    function _setSupply(address token, uint256 totalSupply_, uint256 bondingSupply_) internal {
        Supply storage s = supply[token];
        s.totalSupply = totalSupply_;
        s.bondingSupply = bondingSupply_;
    }

    function _setVirtualReserves(uint256 virtualBase, uint256 virtualQuote) internal {
        if (virtualBase == 0) revert InvalidVirtualBase();
        if (virtualQuote == 0) revert InvalidVirtualQuote();

        VIRTUAL_BASE = virtualBase;
        VIRTUAL_QUOTE = virtualQuote;

        emit VirtualReservesSet(virtualBase, virtualQuote);
    }

    function _getBaseAmount(uint256 quoteAmount, uint256 quoteReserve, uint256 baseReserve, bool isBuy)
        internal
        pure
        returns (uint256 baseAmount)
    {
        uint256 quoteReserveAfter = isBuy ? quoteReserve + quoteAmount : quoteReserve - quoteAmount;

        return (quoteAmount * baseReserve) / quoteReserveAfter;
    }

    function _getQuoteAmount(uint256 baseAmount, uint256 quoteReserve, uint256 baseReserve, bool isBuy)
        internal
        pure
        returns (uint256 quoteAmount)
    {
        uint256 baseReserveAfter = isBuy ? baseReserve - baseAmount : baseReserve + baseAmount;

        return (quoteReserve * baseAmount) / baseReserveAfter;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "@gte-univ2-core/interfaces/IUniswapV2Pair.sol";
import "@gte-univ2-core/UniswapV2ERC20.sol";
import "@gte-univ2-core/libraries/Math.sol";
import "@gte-univ2-core/libraries/UQ112x112.sol";
import "@gte-univ2-core/interfaces/IERC20.sol";
import "@gte-univ2-core/interfaces/IUniswapV2Factory.sol";
import "@gte-univ2-core/interfaces/IUniswapV2Callee.sol";

import "../interfaces/IDistributor.sol";
import "./interfaces/IGTELaunchpadV2Pair.sol";

// import "forge-std/Console.sol";

contract GTELaunchpadV2Pair is IUniswapV2Pair, IGTELaunchpadV2Pair, UniswapV2ERC20 {
    using SafeMath for uint256;
    using UQ112x112 for uint224;

    event RewardsPoolDeactivated();
    event LaunchpadFeesAccrued(uint112 fee0, uint112 fee1);
    event LaunchpadFeesCollected(uint256 collected0, uint256 collected1);
    event LaunchpadFeesLastAccrued(uint112 fee0, uint112 fee1);

    uint256 public constant REWARDS_FEE_SHARE = 1;
    uint256 public constant MINIMUM_LIQUIDITY = 10 ** 3;
    bytes4 private constant TRANSFER_SELECTOR = bytes4(keccak256(bytes("transfer(address,uint256)")));
    bytes4 private constant APPROVE_SELECTOR = bytes4(keccak256(bytes("approve(address,uint256)")));

    // If the launchpad is non 0, then this pool will accumulate a share of swap fees that can be claimed
    address public launchpadLp;
    address public launchpadFeeDistributor;

    address public factory;
    address public token0;
    address public token1;

    uint112 private reserve0; // uses single storage slot, accessible via getReserves
    uint112 private reserve1; // uses single storage slot, accessible via getReserves
    uint32 private blockTimestampLast; // uses single storage slot, accessible via getReserves

    uint256 public price0CumulativeLast;
    uint256 public price1CumulativeLast;
    uint256 public kLast; // reserve0 * reserve1, as of immediately after the most recent liquidity event

    uint112 public accruedLaunchpadFee0;
    uint112 public accruedLaunchpadFee1;

    uint256 public rewardsPoolActive = 1;

    uint256 private unlocked = 1;

    modifier lock() {
        if (unlocked != 1) revert("UniswapV2: LOCKED");
        unlocked = 0;
        _;
        unlocked = 1;
    }

    function getReserves() public view returns (uint112 _reserve0, uint112 _reserve1, uint32 _blockTimestampLast) {
        _reserve0 = reserve0;
        _reserve1 = reserve1;
        _blockTimestampLast = blockTimestampLast;
    }

    function getAccruedLaunchpadFees() public view returns (uint112, uint112, uint32) {
        return (accruedLaunchpadFee0, accruedLaunchpadFee1, blockTimestampLast);
    }

    function _safeTransfer(address token, address to, uint256 value) private {
        (bool success, bytes memory data) = token.call(abi.encodeWithSelector(TRANSFER_SELECTOR, to, value));
        if (!success || !(data.length == 0 || abi.decode(data, (bool)))) revert("UniswapV2: TRANSFER_FAILED");
    }

    function _safeApprove(address token, address to, uint256 value) private {
        (bool success, bytes memory data) = token.call(abi.encodeWithSelector(APPROVE_SELECTOR, to, value));
        if (!success || !(data.length == 0 || abi.decode(data, (bool)))) revert("UniswapV2: APPROVAL_FAILED");
    }

    constructor() {
        factory = msg.sender;
    }

    // called once by the factory at time of deployment
    function initialize(address _token0, address _token1, address _launchpadLp, address _launchpadFeeDistributor)
        external
    {
        if (msg.sender != factory) revert("UniswapV2: FORBIDDEN"); // sufficient check
        token0 = _token0;
        token1 = _token1;
        launchpadLp = _launchpadLp;
        launchpadFeeDistributor = _launchpadFeeDistributor;
        rewardsPoolActive = 1;
    }

    function endRewardsAccrual() external {
        if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");

        // There are no more shares, so prevent distribution and accrual of any remaining rewards
        delete accruedLaunchpadFee0;
        delete accruedLaunchpadFee1;
        delete rewardsPoolActive;

        _update(
            IERC20(token0).balanceOf(address(this)),
            IERC20(token1).balanceOf(address(this)),
            reserve0,
            reserve1,
            uint112(0),
            uint112(0)
        );

        emit RewardsPoolDeactivated();
    }

    // update reserves and, on the first call per block, price accumulators
    function _update(
        uint256 balance0,
        uint256 balance1,
        uint112 _reserve0,
        uint112 _reserve1,
        uint112 newLaunchpadFee0,
        uint112 newLaunchpadFee1
    ) private {
        if (balance0 > type(uint112).max || balance1 > type(uint112).max) revert("UniswapV2: OVERFLOW");

        // New accrued fees must AT LEAST equal existing undistributed fees so that the Sync can be accurate
        uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
        uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;

        uint32 blockTimestamp = uint32(block.timestamp % 2 ** 32);
        uint32 timeElapsed = blockTimestamp - blockTimestampLast; // overflow is desired
        if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
            // * never overflows, and + overflow is desired
            price0CumulativeLast += uint256(UQ112x112.encode(_reserve1).uqdiv(_reserve0)) * timeElapsed;
            price1CumulativeLast += uint256(UQ112x112.encode(_reserve0).uqdiv(_reserve1)) * timeElapsed;

            if (launchpadFeeDistributor > address(0)) {
                if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
                    delete accruedLaunchpadFee0;
                    delete accruedLaunchpadFee1;
                    _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
                }
            }
        } else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) {
            accruedLaunchpadFee0 = totalLaunchpadFee0;
            accruedLaunchpadFee1 = totalLaunchpadFee1;
            emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
        }

        // Balances contain both accrued and new launchpad fees earned this tx
        // as balance is called before any fee distributions, so subtract the total
        reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
        reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;

        blockTimestampLast = blockTimestamp;
        emit Sync(_reserve0, _reserve1);
    }

    // if fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k)
    function _mintFee(uint112 _reserve0, uint112 _reserve1) private returns (bool feeOn) {
        address feeTo = IUniswapV2Factory(factory).feeTo();
        feeOn = feeTo != address(0);
        uint256 _kLast = kLast; // gas savings
        if (feeOn) {
            if (_kLast != 0) {
                uint256 rootK = Math.sqrt(uint256(_reserve0).mul(_reserve1));
                uint256 rootKLast = Math.sqrt(_kLast);
                if (rootK > rootKLast) {
                    uint256 numerator = totalSupply.mul(rootK.sub(rootKLast));
                    uint256 denominator = rootK.mul(5).add(rootKLast);
                    uint256 liquidity = numerator / denominator;
                    if (liquidity > 0) _mint(feeTo, liquidity);
                }
            }
        } else if (_kLast != 0) {
            kLast = 0;
        }
    }

    // this low-level function should be called from a contract which performs important safety checks
    function mint(address to) external lock returns (uint256 liquidity) {
        (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        uint256 balance0 = IERC20(token0).balanceOf(address(this));
        uint256 balance1 = IERC20(token1).balanceOf(address(this));
        uint256 amount0 = balance0.sub(_reserve0);
        uint256 amount1 = balance1.sub(_reserve1);

        bool feeOn = _mintFee(_reserve0, _reserve1);
        uint256 _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
        if (_totalSupply == 0) {
            liquidity = Math.sqrt(amount0.mul(amount1)).sub(MINIMUM_LIQUIDITY);
            _mint(address(0), MINIMUM_LIQUIDITY); // permanently lock the first MINIMUM_LIQUIDITY tokens
        } else {
            liquidity = Math.min(amount0.mul(_totalSupply) / _reserve0, amount1.mul(_totalSupply) / _reserve1);
        }
        if (liquidity == 0) revert("UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED");
        _mint(to, liquidity);

        _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
        if (feeOn) kLast = uint256(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
        emit Mint(msg.sender, amount0, amount1);
    }

    // this low-level function should be called from a contract which performs important safety checks
    function burn(address to) external lock returns (uint256 amount0, uint256 amount1) {
        (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        address _token0 = token0; // gas savings
        address _token1 = token1; // gas savings
        uint256 balance0 = IERC20(_token0).balanceOf(address(this));
        uint256 balance1 = IERC20(_token1).balanceOf(address(this));
        uint256 liquidity = balanceOf[address(this)];

        bool feeOn = _mintFee(_reserve0, _reserve1);
        uint256 _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
        amount0 = liquidity.mul(balance0) / _totalSupply; // using balances ensures pro-rata distribution
        amount1 = liquidity.mul(balance1) / _totalSupply; // using balances ensures pro-rata distribution
        if (amount0 == 0 || amount1 == 0) revert("UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED");
        _burn(address(this), liquidity);
        _safeTransfer(_token0, to, amount0);
        _safeTransfer(_token1, to, amount1);
        balance0 = IERC20(_token0).balanceOf(address(this));
        balance1 = IERC20(_token1).balanceOf(address(this));

        _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
        if (feeOn) kLast = uint256(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
        emit Burn(msg.sender, amount0, amount1, to);
    }

    // this low-level function should be called from a contract which performs important safety checks
    function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
        if (amount0Out == 0 && amount1Out == 0) revert("UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT");
        (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        if (amount0Out >= _reserve0 || amount1Out >= _reserve1) revert("UniswapV2: INSUFFICIENT_LIQUIDITY");

        uint256 balance0;
        uint256 balance1;
        {
            // scope for _token{0,1}, avoids stack too deep errors
            address _token0 = token0;
            address _token1 = token1;
            if (to == _token0 || to == _token1) revert("UniswapV2: INVALID_TO");
            if (amount0Out > 0) _safeTransfer(_token0, to, amount0Out); // optimistically transfer tokens
            if (amount1Out > 0) _safeTransfer(_token1, to, amount1Out); // optimistically transfer tokens
            if (data.length > 0) IUniswapV2Callee(to).uniswapV2Call(msg.sender, amount0Out, amount1Out, data);
            balance0 = IERC20(_token0).balanceOf(address(this));
            balance1 = IERC20(_token1).balanceOf(address(this));
        }
        uint256 amount0In = balance0 > _reserve0 - amount0Out ? balance0 - (_reserve0 - amount0Out) : 0;
        uint256 amount1In = balance1 > _reserve1 - amount1Out ? balance1 - (_reserve1 - amount1Out) : 0;
        if (amount0In == 0 && amount1In == 0) revert("UniswapV2: INSUFFICIENT_INPUT_AMOUNT");

        {
            // scope for reserve{0,1}Adjusted and launchpadFee{0,1}, avoids stack too deep errors
            uint256 balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3));
            uint256 balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3));

            if (balance0Adjusted.mul(balance1Adjusted) < uint256(_reserve0).mul(_reserve1).mul(1000 ** 2)) {
                revert("UniswapV2: K");
            }

            (uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
                && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

            _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
        }

        emit Swap(msg.sender, amount0In, amount1In, amount0Out, amount1Out, to);
    }

    function _getLaunchpadFees(uint256 amount0In, uint256 amount1In)
        internal
        view
        returns (uint112 fee0, uint112 fee1)
    {
        // Only swap fees that represent the launchpad's share of the LP supply should accrue towards unclaimed launchpad fees.
        // MINIMUM_LIQUIDITY is permanently locked to address(0) when liquidity is first added by the launchpad.
        // Therefore, we add it back here so that the fee share math represents the whole of *initial* liquidity added,
        // not just the launchpad's slightly lower actual lp balance
        uint256 totalLpBal = this.totalSupply();
        uint256 launchpadLpBal = this.balanceOf(launchpadLp) + MINIMUM_LIQUIDITY;

        if (amount0In > 0) fee0 = uint112(amount0In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));
        if (amount1In > 0) fee1 = uint112(amount1In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));

        return (fee0, fee1);
    }

    function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
        if ((fee0 | fee1) > 0) {
            address _token0 = token0;
            address _token1 = token1;
            address distributor = launchpadFeeDistributor;

            // Since only pairs created by the launchpad can accrue fee tracking, the tokens are trusted
            if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
            if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

            IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

            emit LaunchpadFeesCollected(fee0, fee1);
        }
    }

    // force balances to match reserves
    function skim(address to) external lock {
        address _token0 = token0; // gas savings
        address _token1 = token1; // gas savings
        _safeTransfer(_token0, to, IERC20(_token0).balanceOf(address(this)).sub(reserve0 + accruedLaunchpadFee0));
        _safeTransfer(_token1, to, IERC20(_token1).balanceOf(address(this)).sub(reserve1 + accruedLaunchpadFee1));
    }

    // force reserves to match balances
    function sync() external lock {
        _update(
            IERC20(token0).balanceOf(address(this)),
            IERC20(token1).balanceOf(address(this)),
            reserve0,
            reserve1,
            uint112(0),
            uint112(0)
        );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {IAccountManager} from "./IAccountManager.sol";
import {ICLOB} from "../clob/ICLOB.sol";
import {MakerCredit} from "../clob/types/TransientMakerData.sol";
import {IPerpManager} from "../perps/interfaces/IPerpManager.sol";
import {Side} from "../clob/types/Order.sol";
import {OperatorHelperLib} from "../utils/types/OperatorHelperLib.sol";
import {EventNonceLib as AccountEventNonce} from "../utils/types/EventNonce.sol";
import {Initializable} from "@solady/utils/Initializable.sol";
import {OwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {OperatorPanel, SpotOperatorRoles} from "../utils/OperatorPanel.sol";
import {
    FeeData,
    FeeDataLib,
    FeeDataStorageLib,
    PackedFeeRates,
    PackedFeeRatesLib,
    FeeTiers
} from "../clob/types/FeeData.sol";

struct AccountManagerStorage {
    mapping(address market => bool) isMarket;
    mapping(address account => mapping(address asset => uint256)) accountTokenBalances;
}

/**
 * @title AccountManager
 * @notice Handles account balances, deposits, withdrawals, for GTE spot as well as inheriting Operator
 */
contract AccountManager is IAccountManager, OperatorPanel, Initializable, OwnableRoles {
    using SafeTransferLib for address;
    using FixedPointMathLib for uint256;
    using PackedFeeRatesLib for PackedFeeRates;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x07796b317344e6f18fa32ed89b6074ad66549cee7fb7b8c3e9f1c42c496f1c5c
    event MarketRegistered(uint256 indexed eventNonce, address indexed market);
    /// @dev sig: 0x1ae35cf838a52070167575d4dedf6631cc160136bee10eeca1575d2e3cc8a075
    event AccountDebited(uint256 indexed eventNonce, address indexed account, address indexed token, uint256 amount);
    /// @dev sig: 0x074f9f8975d437bea257b7e6abcfb4b45312683f7f8f120dde3faae76f783b58
    event AccountCredited(uint256 indexed eventNonce, address indexed account, address indexed token, uint256 amount);

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x00b8f216
    error BalanceInsufficient();
    /// @dev sig: 0x467cb8b4
    error GTERouterUnauthorized();
    /// @dev sig: 0x30eee8ba
    error CLOBManagerUnauthorized();
    /// @dev sig: 0x9d1c9c18
    error MarketUnauthorized();
    /// @dev sig: 0x38422dcd
    error UnmatchingArrayLengths();
    error NotPerpManager();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            IMMUTABLE STATE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    uint256 public constant FEE_COLLECTOR = 1;

    /// @dev The global router address that can bypass the operator check
    address public immutable gteRouter;
    /// @dev The CLOBManager address that can call settlement functions
    address public immutable clobManager;
    /// @dev Packed spot maker fee rates for all tiers
    PackedFeeRates public immutable spotMakerFeeRates;
    /// @dev Packed spot taker fee rates for all tiers
    PackedFeeRates public immutable spotTakerFeeRates;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                MODIFIERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Ensures msg.sender is a registered market
    modifier onlyMarket() {
        if (!_getAccountStorage().isMarket[msg.sender]) revert MarketUnauthorized();
        _;
    }

    /// @dev Ensures msg.sender is the router
    modifier onlyGTERouter() {
        if (msg.sender != gteRouter) revert GTERouterUnauthorized();
        _;
    }

    /// @dev Ensures msg.sender is the CLOBManager
    modifier onlyCLOBManager() {
        if (msg.sender != clobManager) revert CLOBManagerUnauthorized();
        _;
    }

    /// @dev Ensures that if an account is not the msg.sender, both that account and the owner have approved msg.sender
    modifier onlySenderOrOperator(address account, SpotOperatorRoles requiredRole) {
        OperatorHelperLib.onlySenderOrOperator(_getOperatorStorage(), gteRouter, account, requiredRole);
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                CONSTRUCTOR
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    constructor(
        address _gteRouter,
        address _clobManager,
        address _operatorHub,
        uint16[] memory _spotMakerFees,
        uint16[] memory _spotTakerFees,
        address _perpManager
    ) OperatorPanel(_operatorHub) {
        gteRouter = _gteRouter;
        clobManager = _clobManager;
        spotMakerFeeRates = PackedFeeRatesLib.packFeeRates(_spotMakerFees);
        spotTakerFeeRates = PackedFeeRatesLib.packFeeRates(_spotTakerFees);
        perpManager = IPerpManager(_perpManager);
        _disableInitializers();
    }

    /// @dev Initializes the contract
    function initialize(address _owner) external initializer {
        _initializeOwner(_owner);
    }

    IPerpManager immutable perpManager;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            EXTERNAL GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Gets an `account`'s balance of `token`
    function getAccountBalance(address account, address token) external view returns (uint256) {
        return _getAccountStorage().accountTokenBalances[account][token];
    }

    /// @notice Gets the current event nonce
    function getEventNonce() external view returns (uint256) {
        return AccountEventNonce.getCurrentNonce();
    }

    /// @notice Gets the total fees collected for a token
    function getTotalFees(address token) external view returns (uint256) {
        return FeeDataStorageLib.getFeeDataStorage().totalFees[token];
    }

    /// @notice Gets the unclaimed fees for a token
    function getUnclaimedFees(address token) external view returns (uint256) {
        return FeeDataStorageLib.getFeeDataStorage().unclaimedFees[token];
    }

    /// @notice Gets the fee tier for an account
    function getFeeTier(address account) external view returns (FeeTiers) {
        return FeeDataStorageLib.getFeeDataStorage().getAccountFeeTier(account);
    }

    /// @notice Gets the spot taker fee rate for a given fee tier
    function getSpotTakerFeeRateForTier(FeeTiers tier) external view returns (uint256) {
        return spotTakerFeeRates.getFeeAt(uint256(tier));
    }

    /// @notice Gets the spot maker fee rate for a given fee tier
    function getSpotMakerFeeRateForTier(FeeTiers tier) external view returns (uint256) {
        return spotMakerFeeRates.getFeeAt(uint256(tier));
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ACCOUNTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Deposits via transfer from the account
    function deposit(address account, address token, uint256 amount)
        external
        virtual
        onlySenderOrOperator(account, SpotOperatorRoles.SPOT_DEPOSIT)
    {
        _creditAccount(_getAccountStorage(), account, token, amount);
        token.safeTransferFrom(account, address(this), amount);
    }

    function depositTo(address account, address token, uint256 amount)
        external
    {
        _creditAccount(_getAccountStorage(), account, token, amount);
        token.safeTransferFrom(msg.sender, address(this), amount);
    }

    function depositFromPerps(address account, uint256 amount)
        external
        onlySenderOrOperator(account, SpotOperatorRoles.PERP_TO_SPOT_DEPOSIT)
    {
        perpManager.withdrawToSpot(account, amount);
        _creditAccount(_getAccountStorage(), account, perpManager.getCollateralAsset(), amount);
    }

    /// @notice Deposits via transfer from the router
    function depositFromRouter(address account, address token, uint256 amount) external onlyGTERouter {
        _creditAccount(_getAccountStorage(), account, token, amount);
        token.safeTransferFrom(gteRouter, address(this), amount);
    }

    /// @notice Withdraws to account
    function withdraw(address account, address token, uint256 amount)
        external
        virtual
        onlySenderOrOperator(account, SpotOperatorRoles.SPOT_WITHDRAW)
    {
        _debitAccount(_getAccountStorage(), account, token, amount);
        token.safeTransfer(account, amount);
    }

    function withdrawToPerps(address account, uint256 amount) external {
        if (msg.sender != address(perpManager)) revert NotPerpManager();

        address token = perpManager.getCollateralAsset();

        _debitAccount(_getAccountStorage(), account, token, amount);
        token.safeTransfer(address(perpManager), amount);
    }

    /// @notice Withdraws from account to router
    function withdrawToRouter(address account, address token, uint256 amount) external onlyGTERouter {
        _debitAccount(_getAccountStorage(), account, token, amount);
        token.safeTransfer(gteRouter, amount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ADMIN
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Registers a market address, can only be called by CLOBManager
    function registerMarket(address market) external onlyCLOBManager {
        _getAccountStorage().isMarket[market] = true;
        emit MarketRegistered(AccountEventNonce.inc(), market);
    }

    /// @notice Collects accrued fees for a token and transfers to recipient
    function collectFees(address token, address feeRecipient)
        external
        virtual
        onlyOwnerOrRoles(FEE_COLLECTOR)
        returns (uint256 fee)
    {
        FeeData storage feeData = FeeDataStorageLib.getFeeDataStorage();
        fee = feeData.claimFees(token);

        if (fee > 0) {
            // Transfer fees directly from contract balance to recipient
            token.safeTransfer(feeRecipient, fee);
        }
    }

    /// @notice Sets the spot fee tier for a single account, can only be called by CLOBManager
    function setSpotAccountFeeTier(address account, FeeTiers feeTier) external virtual onlyCLOBManager {
        FeeData storage feeData = FeeDataStorageLib.getFeeDataStorage();
        feeData.setAccountFeeTier(account, feeTier);
    }

    /// @notice Sets the spot fee tiers for multiple accounts, can only be called by CLOBManager
    function setSpotAccountFeeTiers(address[] calldata accounts, FeeTiers[] calldata feeTiers)
        external
        virtual
        onlyCLOBManager
    {
        if (accounts.length != feeTiers.length) revert UnmatchingArrayLengths();

        FeeData storage feeData = FeeDataStorageLib.getFeeDataStorage();
        for (uint256 i = 0; i < accounts.length; i++) {
            feeData.setAccountFeeTier(accounts[i], feeTiers[i]);
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                SETTLEMENT
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice The hook for markets to perform account settlement after a fill, including fee calculations
    function settleIncomingOrder(ICLOB.SettleParams calldata params)
        external
        virtual
        onlyMarket
        returns (uint256 takerFee)
    {
        AccountManagerStorage storage self = _getAccountStorage();
        FeeData storage feeData = FeeDataStorageLib.getFeeDataStorage();

        // Credit taker less fee
        address takerFeeToken;
        if (params.side == Side.BUY) {
            takerFee = feeData.getTakerFee(spotTakerFeeRates, params.taker, params.takerBaseAmount);
            takerFeeToken = params.baseToken;

            // Taker settlement
            _debitAccount(self, params.taker, params.quoteToken, params.takerQuoteAmount);
            _creditAccount(self, params.taker, params.baseToken, params.takerBaseAmount - takerFee);
        } else {
            takerFee = feeData.getTakerFee(spotTakerFeeRates, params.taker, params.takerQuoteAmount);
            takerFeeToken = params.quoteToken;

            // Taker settlement
            _debitAccount(self, params.taker, params.baseToken, params.takerBaseAmount);
            _creditAccount(self, params.taker, params.quoteToken, params.takerQuoteAmount - takerFee);
        }

        // Accrue taker fee
        if (takerFee > 0) feeData.accrueFee(takerFeeToken, takerFee);

        // Process maker settlement and fees
        uint256 currMakerFee = 0;
        uint256 totalQuoteMakerFee = 0;
        uint256 totalBaseMakerFee = 0;

        for (uint256 i; i < params.makerCredits.length; ++i) {
            MakerCredit memory credit = params.makerCredits[i];

            // Calculate fees only for the matching side
            if (params.side == Side.BUY && credit.quoteAmount > 0) {
                currMakerFee = feeData.getMakerFee(spotMakerFeeRates, credit.maker, credit.quoteAmount);
                credit.quoteAmount -= currMakerFee;
                totalQuoteMakerFee += currMakerFee;
            } else if (params.side == Side.SELL && credit.baseAmount > 0) {
                currMakerFee = feeData.getMakerFee(spotMakerFeeRates, credit.maker, credit.baseAmount);
                credit.baseAmount -= currMakerFee;
                totalBaseMakerFee += currMakerFee;
            }

            // Credit both base and quote amounts if any (not just fills less fee, but also expiry and non-competitive refunds)
            if (credit.baseAmount > 0) _creditAccountNoEvent(self, credit.maker, params.baseToken, credit.baseAmount);

            if (credit.quoteAmount > 0) {
                _creditAccountNoEvent(self, credit.maker, params.quoteToken, credit.quoteAmount);
            }
        }

        // Accrue total collected maker fees
        if (totalBaseMakerFee > 0) feeData.accrueFee(params.baseToken, totalBaseMakerFee);
        if (totalQuoteMakerFee > 0) feeData.accrueFee(params.quoteToken, totalQuoteMakerFee);
    }

    /// @notice Credits account, called by markets for amends/cancels
    function creditAccount(address account, address token, uint256 amount) external virtual onlyMarket {
        _creditAccount(_getAccountStorage(), account, token, amount);
    }

    /// @notice Credits account without event, called by markets for non-competitive order removal
    function creditAccountNoEvent(address account, address token, uint256 amount) external virtual onlyMarket {
        _creditAccountNoEvent(_getAccountStorage(), account, token, amount);
    }

    /// @notice Debits account, called by markets for amends
    function debitAccount(address account, address token, uint256 amount) external virtual onlyMarket {
        _debitAccount(_getAccountStorage(), account, token, amount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            INTERNAL HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _creditAccount(AccountManagerStorage storage self, address account, address token, uint256 amount)
        internal
    {
        unchecked {
            self.accountTokenBalances[account][token] += amount;
        }
        emit AccountCredited(AccountEventNonce.inc(), account, token, amount);
    }

    function _creditAccountNoEvent(AccountManagerStorage storage self, address account, address token, uint256 amount)
        internal
    {
        unchecked {
            self.accountTokenBalances[account][token] += amount;
        }
    }

    function _debitAccount(AccountManagerStorage storage self, address account, address token, uint256 amount)
        internal
    {
        if (self.accountTokenBalances[account][token] < amount) revert BalanceInsufficient();

        unchecked {
            self.accountTokenBalances[account][token] -= amount;
        }
        emit AccountDebited(AccountEventNonce.inc(), account, token, amount);
    }

    /// @dev Helper to set the storage slot of the storage struct for this contract
    function _getAccountStorage() internal pure returns (AccountManagerStorage storage ds) {
        return AccountManagerStorageLib.getAccountManagerStorage();
    }
}

using AccountManagerStorageLib for AccountManagerStorage global;

/// @custom:storage-location erc7201:AccountManagerStorage
library AccountManagerStorageLib {
    bytes32 constant ACCOUNT_MANAGER_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("AccountManagerStorage")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev Gets the storage slot of the storage struct for the contract calling this library function
    // slither-disable-next-line uninitialized-storage
    function getAccountManagerStorage() internal pure returns (AccountManagerStorage storage self) {
        bytes32 position = ACCOUNT_MANAGER_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := position
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

// import {BondingCurve} from "./BondingCurve.sol";
import {LaunchToken} from "./LaunchToken.sol";
import {IUniswapV2RouterMinimal} from "./interfaces/IUniswapV2RouterMinimal.sol";
import {IUniswapV2FactoryMinimal} from "./interfaces/IUniswapV2FactoryMinimal.sol";
import {ILaunchpad, IBondingCurveMinimal} from "./interfaces/ILaunchpad.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";
import {ERC165Checker} from "@openzeppelin/utils/introspection/ERC165Checker.sol";

import {LaunchpadLPVault} from "./LaunchpadLPVault.sol";
import {Initializable} from "@solady/utils/Initializable.sol";
import {Ownable} from "@solady/auth/Ownable.sol";
import {ReentrancyGuard} from "@solady/utils/ReentrancyGuard.sol";

import {ICLOBManager} from "contracts/clob/ICLOBManager.sol";
import {IUniV2Factory} from "./interfaces/IUniV2Factory.sol";
import {IUniswapV2Pair} from "./interfaces/IUniswapV2Pair.sol";
import {IDistributor} from "./interfaces/IDistributor.sol";
import {IGTELaunchpadV2Pair} from "./uniswap/interfaces/IGTELaunchpadV2Pair.sol";

import {SpotOperatorRoles} from "contracts/utils/OperatorPanel.sol";
import {OperatorHelperLib} from "contracts/utils/types/OperatorHelperLib.sol";
import {IOperatorPanel} from "contracts/utils/interfaces/IOperatorPanel.sol";
import {EventNonceLib as LaunchpadEventNonce} from "contracts/utils/types/EventNonce.sol";

contract Launchpad is ILaunchpad, Initializable, Ownable, ReentrancyGuard {
    using SafeTransferLib for address;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0xedb9fe87bd33aab1b87917d7ac18478fc6e849256def39db1a60ca615a1a35f9
    event LaunchpadDeployed(address indexed quoteAsset, address bondingCurve, address router, uint256 eventNonce);
    /// @dev sig: 0xb32a6b288aadfa675cb030ee2f51c6dc12675a9e5531231a996b8edb611f1956
    event BondingLocked(address indexed token, IUniswapV2Pair indexed pairAddress, uint256 eventNonce);
    /// @dev sig: 0xca2a6f300abd801d3ade4ca6344f9caba868f5165eb754544d4fe6195fe07212
    event BondingCurveUpdated(address indexed oldCurve, address indexed newCurve, uint256 eventNonce);
    /// @dev sig: 0x8d4aad4953d0ca700d468f3753aa14432d1b35b43ec6409f051fb6aa43a89607
    event TokenLaunched(
        address indexed dev,
        address indexed token,
        address indexed quoteAsset,
        IBondingCurveMinimal bondingCurve,
        uint256 timestamp,
        uint256 eventNonce
    );
    /// @dev sig: 0x221ca85ebf95f18d1618caabee27ca0867de44313b2989c305e6e6f96f582e40
    event QuoteAssetUpdated(
        address indexed oldQuoteToken, address indexed newQuoteToken, uint256 newQuoteTokenDecimals, uint256 eventNonce
    );
    /// @dev sig: 0xe8f92b6d8befe44289e67ee6740a1b61cfea7bd8ebe8c2050c4ec7ef555d5fc5
    event Swap(
        address indexed buyer,
        address indexed token,
        int256 baseDelta,
        int256 quoteDelta,
        uint256 nextAmountSold,
        uint256 newPrice,
        uint256 eventNonce
    );

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0xa2c1d73f
    error BadLaunchFee();
    /// @dev sig: 0x2fe7552a
    error InvalidCurve();
    /// @dev sig: 0xfe717103
    error BondingCurveSetupFailed(bytes returnData);
    /// @dev sig: 0xc7022a01
    error MissingCredits();
    /// @dev sig: 0x5e2acf84
    error OnlyLaunchAsset();
    /// @dev sig: 0x9efab874
    error BondingInactive();
    /// @dev sig: 0x9c8d2cd2
    error InvalidRecipient();
    /// @dev sig: 0x4233ebcb
    error DustAttackInvalid();
    /// @dev sig: 0x1d33d88c
    error InvalidQuoteAsset();
    /// @dev sig: 0x6f156a5e
    error UninitializedCurve();
    /// @dev sig: 0xa3265e40
    error UninitializedQuote();
    /// @dev sig: 0x9b480a76
    error InvalidQuoteScaling();
    /// @dev sig: 0xad73b7b2
    error UnsupportedRewardToken();
    /// @dev sig: 0x6728a9f6
    error SlippageToleranceExceeded();
    /// @dev sig: 0xb12d13eb
    error ETHTransferFailed();
    /// @dev sig: 0xa1d718af
    error InsufficientBaseSold();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                STATE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev The abi version of this impl so the indexer can handle event-changing upgrades
    uint256 public constant ABI_VERSION = 1;
    uint256 public constant TOTAL_SUPPLY = 1 ether * 1e9;
    uint256 public constant BONDING_SUPPLY = 800_000_000 ether;

    address public immutable gteRouter;
    IOperatorPanel public immutable operator;
    IDistributor public immutable distributor;
    IUniswapV2RouterMinimal public immutable uniV2Router;
    IUniswapV2FactoryMinimal internal immutable uniV2Factory;

    // @todo make proxy addr immutable with deterministic addr
    LaunchpadLPVault public launchpadLPVault;

    /// @dev This is just the quote ERC20 cast as LaunchToken so we dont have to import ERC20
    LaunchToken public currentQuoteAsset;
    IBondingCurveMinimal public currentBondingCurve;

    mapping(address token => LaunchData) internal _launches;

    uint256 public launchFee;
    bytes public uniV2InitCodeHash;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                MODIFIERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    modifier onlyLaunchAsset() {
        if (_launches[msg.sender].quote == address(0)) revert OnlyLaunchAsset();
        _;
    }

    modifier onlyBondingActive(address token) {
        if (!_launches[token].active) revert BondingInactive();
        _;
    }

    modifier onlySenderOrOperator(address account, SpotOperatorRoles requiredRole) {
        if (msg.sender != gteRouter) OperatorHelperLib.onlySenderOrOperator(operator, account, requiredRole);
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                CONSTRUCTOR
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    // slither-disable-next-line missing-zero-check
    constructor(
        address uniV2Router_,
        address gteRouter_,
        address clobFactory_,
        address operator_,
        address distributor_
    ) {
        uniV2Router = IUniswapV2RouterMinimal(uniV2Router_);
        gteRouter = gteRouter_;
        operator = IOperatorPanel(operator_);
        uniV2Factory = IUniswapV2FactoryMinimal(uniV2Router.factory());
        distributor = IDistributor(distributor_);

        _disableInitializers();
    }

    /// @dev bondingCurveSetupData should contain an ABI-encoded call destined for the bonding curve contract
    function initialize(
        address owner_,
        address quoteAsset_,
        address bondingCurve_,
        address launchpadLPVault_,
        bytes memory bondingCurveInitData
    ) external initializer {
        _initializeOwner(owner_);

        if (quoteAsset_ == address(0)) revert InvalidQuoteAsset();
        if (!ERC165Checker.supportsInterface(bondingCurve_, type(IBondingCurveMinimal).interfaceId)) {
            revert InvalidCurve();
        }

        // Sanity check that the new quote asset at the very least implements an ERC20 approval
        LaunchToken(quoteAsset_).approve(address(this), 0);

        currentBondingCurve = IBondingCurveMinimal(bondingCurve_);
        currentQuoteAsset = LaunchToken(quoteAsset_);
        launchpadLPVault = LaunchpadLPVault(launchpadLPVault_);

        // e.g. bondingCurve.setVirtualReserves({virtualBase: virtualBase_, virtualQuote: virtualQuote_});
        currentBondingCurve.init(bondingCurveInitData);

        emit LaunchpadDeployed(quoteAsset_, bondingCurve_, address(uniV2Router), LaunchpadEventNonce.inc());
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                PUBLIC VIEWS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function launches(address launchToken) public view returns (LaunchData memory) {
        return _launches[launchToken];
    }

    function baseSoldFromCurve(address token) public view returns (uint256) {
        return _launches[token].curve.baseSoldFromCurve(token);
    }

    function quoteBoughtByCurve(address token) public view returns (uint256) {
        return _launches[token].curve.quoteBoughtByCurve(token);
    }

    function quoteBaseForQuote(address token, uint256 quoteAmount, bool isBuy)
        public
        view
        returns (uint256 baseAmount)
    {
        return _launches[token].curve.quoteBaseForQuote(token, quoteAmount, isBuy);
    }

    function quoteQuoteForBase(address token, uint256 baseAmount, bool isBuy)
        public
        view
        returns (uint256 quoteAmount)
    {
        return _launches[token].curve.quoteQuoteForBase(token, baseAmount, isBuy);
    }

    function eventNonce() external view returns (uint256) {
        return LaunchpadEventNonce.getCurrentNonce();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            PUBLIC WRITES
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Launches a new token
    function launch(string memory name, string memory symbol, string memory mediaURI)
        external
        payable
        nonReentrant
        returns (address token)
    {
        if (msg.value != launchFee) revert BadLaunchFee();

        address quote = address(currentQuoteAsset);
        IBondingCurveMinimal curve = currentBondingCurve;

        if (quote == address(0)) revert UninitializedQuote();
        if (address(curve) == address(0)) revert UninitializedCurve();

        token = address(new LaunchToken(name, symbol, mediaURI, gteRouter));

        curve.initializeCurve(token, TOTAL_SUPPLY, BONDING_SUPPLY);
        distributor.createRewardsPair(token, quote);

        _launches[token] = LaunchData({active: true, curve: curve, quote: quote});

        emit TokenLaunched({
            dev: msg.sender,
            token: token,
            quoteAsset: quote,
            bondingCurve: curve,
            timestamp: block.timestamp,
            eventNonce: LaunchpadEventNonce.inc()
        });

        LaunchToken(token).mint(TOTAL_SUPPLY);
    }

    /// @notice Buys an `amountOutBase` of a bonding `token` so long as it costs less than `maxAmountInQuote`
    function buy(BuyData calldata buyData)
        external
        nonReentrant
        onlyBondingActive(buyData.token)
        onlySenderOrOperator(buyData.account, SpotOperatorRoles.LAUNCHPAD_FILL)
        returns (uint256 amountOutBaseActual, uint256 amountInQuote)
    {
        IUniswapV2Pair pair = _assertValidRecipient(buyData.recipient, buyData.token);
        LaunchData memory data = _launches[buyData.token];

        (amountOutBaseActual, data.active) = _checkGraduation(buyData.token, data, buyData.amountOutBase);

        amountInQuote = data.curve.buy(buyData.token, amountOutBaseActual);

        if (data.active && amountInQuote == 0) revert DustAttackInvalid();
        if (amountInQuote > buyData.maxAmountInQuote) revert SlippageToleranceExceeded();

        buyData.token.safeTransfer(buyData.recipient, amountOutBaseActual);
        address(data.quote).safeTransferFrom(buyData.account, address(this), amountInQuote);

        _emitSwapEvent({
            account: buyData.account,
            token: buyData.token,
            baseAmount: amountOutBaseActual,
            quoteAmount: amountInQuote,
            isBuy: true,
            curve: data.curve
        });

        // If graduated, handle AMM setup and remaining swap
        if (!data.active) {
            (amountOutBaseActual, amountInQuote) = _graduate(buyData, pair, data, amountOutBaseActual, amountInQuote);
        }
    }

    function _graduate(
        BuyData calldata buyData,
        IUniswapV2Pair pair,
        LaunchData memory data,
        uint256 amountOutBaseActual,
        uint256 amountInQuote
    ) internal returns (uint256 finalAmountOutBaseActual, uint256 finalAmountInQuote) {
        LaunchToken(buyData.token).unlock();
        _launches[buyData.token].active = false;
        emit BondingLocked(buyData.token, pair, LaunchpadEventNonce.inc());

        uint256 additionalQuote = _createPairAndSwapRemaining({
            token: buyData.token,
            pair: pair,
            data: data,
            remainingBase: buyData.amountOutBase - amountOutBaseActual,
            remainingQuote: buyData.maxAmountInQuote - amountInQuote,
            recipient: buyData.recipient
        });

        finalAmountInQuote = amountInQuote + additionalQuote;
        finalAmountOutBaseActual = additionalQuote > 0 ? buyData.amountOutBase : amountOutBaseActual;
    }

    /// @notice Sells an `amountInBase` of a bonding `token` as long as the proceeds are at least `minAmountOutQuote`
    function sell(address account, address token, address recipient, uint256 amountInBase, uint256 minAmountOutQuote)
        external
        nonReentrant
        onlyBondingActive(token)
        onlySenderOrOperator(account, SpotOperatorRoles.LAUNCHPAD_FILL)
        returns (uint256 amountInBaseActual, uint256 amountOutQuoteActual)
    {
        LaunchData memory data = _launches[token];

        uint256 currentBaseSold = data.curve.baseSoldFromCurve(token);
        if (currentBaseSold < amountInBase) revert InsufficientBaseSold();

        // slither-disable-next-line reentrancy-no-eth
        uint256 amountOutQuote = data.curve.sell(token, amountInBase);

        if (amountOutQuote == 0) revert DustAttackInvalid();
        if (amountOutQuote < minAmountOutQuote) revert SlippageToleranceExceeded();

        _emitSwapEvent({
            account: account,
            token: token,
            baseAmount: amountInBase,
            quoteAmount: amountOutQuote,
            isBuy: false,
            curve: data.curve
        });

        token.safeTransferFrom(account, address(this), amountInBase);
        data.quote.safeTransfer(recipient, amountOutQuote);

        return (amountInBase, amountOutQuote);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               OWNER-ONLY
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Updates the bonding curve, live launches are not affected
    function updateBondingCurve(address newBondingCurve) external onlyOwner {
        if (!ERC165Checker.supportsInterface(newBondingCurve, type(IBondingCurveMinimal).interfaceId)) {
            revert InvalidCurve();
        }

        emit BondingCurveUpdated(address(currentBondingCurve), newBondingCurve, LaunchpadEventNonce.inc());

        currentBondingCurve = IBondingCurveMinimal(newBondingCurve);
    }

    /// @notice Updates the quote asset.
    /// @dev The quote asset cannot have been an existing LaunchToken
    function updateQuoteAsset(address newQuoteAsset) external onlyOwner {
        if (newQuoteAsset == address(0) || _launches[newQuoteAsset].quote != address(0)) revert InvalidQuoteAsset();

        // Check new quote at least implements approve in lieu of ERC165
        LaunchToken(newQuoteAsset).approve(address(this), 0);

        emit QuoteAssetUpdated(
            address(currentQuoteAsset), newQuoteAsset, LaunchToken(newQuoteAsset).decimals(), LaunchpadEventNonce.inc()
        );

        currentQuoteAsset = LaunchToken(newQuoteAsset);
    }

    // @todo event
    function updateInitCodeHash(bytes memory newHash) external onlyOwner {
        uniV2InitCodeHash = newHash;
    }

    /// @notice Pulls the fees earned from launching tokens
    function pullFees() external onlyOwner {
        // slither-disable-next-line low-level-calls
        (bool success,) = payable(msg.sender).call{value: address(this).balance}("");

        if (!success) revert ETHTransferFailed();
    }

    // @todo event
    function updateLaunchFee(uint256 newLaunchFee) external onlyOwner {
        launchFee = newLaunchFee;
    }

    // @todo event
    function updateLaunchpadLPVault(address newLaunchpadLPVault) external onlyOwner {
        launchpadLPVault = LaunchpadLPVault(newLaunchpadLPVault);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                FEE-SHARING
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function increaseStake(address account, uint96 shares) external onlyLaunchAsset {
        distributor.increaseStake(msg.sender, account, shares);
    }

    function decreaseStake(address account, uint96 shares) external onlyLaunchAsset {
        distributor.decreaseStake(msg.sender, account, shares);
    }

    // @todo this call needs to be simplified. the token can know the pair address and call it directly
    // launchpadLp in the pair is going to be a different address than this, so we cant call it directly here
    // pair just knows the distributor address which is why we pass the call to distributor, or we have to add the launchpad address as well
    function endRewards() external onlyLaunchAsset {
        address quote = _launches[msg.sender].quote;
        // @todo stick with one pair interface
        IGTELaunchpadV2Pair pair = IGTELaunchpadV2Pair(address(pairFor(address(uniV2Factory), msg.sender, quote)));

        distributor.endRewards(pair);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            INTERNAL LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _checkGraduation(address token, LaunchData memory data, uint256 amountOutBase)
        internal
        view
        returns (uint256 amountOutBaseActual, bool stillActive)
    {
        uint256 maxBaseForSale = data.curve.bondingSupply(token);

        uint256 baseSold = data.curve.baseSoldFromCurve(token);
        uint256 nextAmountSold = baseSold + amountOutBase;

        // No graduation, can buy full amount of base requested from curve
        if (nextAmountSold < maxBaseForSale) return (amountOutBase, true);

        amountOutBaseActual = maxBaseForSale - baseSold;

        return (amountOutBaseActual, false);
    }

    /// @dev Internal struct to help with stack depth
    struct SwapRemainingData {
        address token;
        address quote;
        address recipient;
        uint256 baseAmount;
        uint256 quoteAmount;
    }

    function _createPairAndSwapRemaining(
        address token,
        IUniswapV2Pair pair,
        LaunchData memory data,
        uint256 remainingBase,
        uint256 remainingQuote,
        address recipient
    ) internal returns (uint256 additionalQuoteUsed) {
        /// @todo sr wardens, please flag in your QA report your thoughts on the comments below

        // Create or get the pair
        try uniV2Factory.createPair(token, data.quote) returns (address p) {
            pair = IUniswapV2Pair(p);
        } catch {
            // Do nothing, pair exists
            // @todo its more gas but lets check pair exists and create if it doest.
            // try catch in solidity is horrible and should be avoided
        }

        pair.skim(owner());

        // Add initial liquidity
        uint256 tokensToLock = data.curve.totalSupply(token) - data.curve.bondingSupply(token);
        uint256 quoteToLock = data.curve.quoteBoughtByCurve(token);

        token.safeApprove(address(uniV2Router), tokensToLock);
        data.quote.safeApprove(address(uniV2Router), quoteToLock);

        uniV2Router.addLiquidity({
            tokenA: token,
            tokenB: address(data.quote),
            amountADesired: tokensToLock,
            amountBDesired: quoteToLock,
            amountAMin: 0,
            amountBMin: 0,
            to: address(launchpadLPVault),
            deadline: block.timestamp
        });

        // Handle remaining swap if needed

        // @todo clean up control flow here and confirm this is the right trigger
        if (remainingBase > 0 && remainingQuote > 0) {
            uint256 quoteNeeded =
                uniV2Router.getAmountIn({amountOut: remainingBase, reserveIn: quoteToLock, reserveOut: tokensToLock});

            if (remainingQuote >= quoteNeeded) {
                SwapRemainingData memory d = SwapRemainingData({
                    token: token,
                    quote: data.quote,
                    recipient: recipient,
                    baseAmount: remainingBase,
                    quoteAmount: quoteNeeded
                });

                (, uint256 quoteUsed) = _swapRemaining(d);
                return quoteUsed;
            }
        }

        return 0;
    }

    /// @dev Tries to perform an exact out swap of the remaining quote tokens from a partially filled buy
    function _swapRemaining(SwapRemainingData memory data) internal returns (uint256, uint256) {
        // Transfer the remaining quote from the user
        data.quote.safeTransferFrom(msg.sender, address(this), data.quoteAmount);

        // Prepare swap path
        address[] memory path = new address[](2);
        path[0] = data.quote;
        path[1] = data.token;

        // Approve router to spend remaining quote
        data.quote.safeApprove(address(uniV2Router), data.quoteAmount);

        try uniV2Router.swapTokensForExactTokens(
            data.baseAmount, data.quoteAmount, path, data.recipient, block.timestamp + 1
        ) {
            // Return the tokens received and quote used
            return (data.baseAmount, data.quoteAmount);
        } catch {
            // If swap fails, return the additional quote tokens to the user and remove approval
            data.quote.safeApprove(address(uniV2Router), 0);
            data.quote.safeTransfer(msg.sender, data.quoteAmount);
            return (0, 0);
        }
    }

    /// @dev Since the launchpad is the only address able to send or receive launch tokens during bonding,
    /// and an arbitrary `recipient` can be specified for
    function _assertValidRecipient(address recipient, address baseToken) internal view returns (IUniswapV2Pair pair) {
        pair = pairFor(address(uniV2Factory), baseToken, _launches[baseToken].quote);
        if (address(pair) == recipient) revert InvalidRecipient();
    }

    // calculates the CREATE2 address for a pair without making any external calls
    function pairFor(address factory, address tokenA, address tokenB) internal view returns (IUniswapV2Pair pair) {
        (address token0, address token1) = sortTokens(tokenA, tokenB);
        pair = IUniswapV2Pair(
            address(
                uint160(
                    uint256(
                        keccak256(
                            abi.encodePacked(
                                hex"ff",
                                factory,
                                keccak256(abi.encodePacked(token0, token1)),
                                uniV2InitCodeHash // init code hash
                            )
                        )
                    )
                )
            )
        );
    }

    function _emitSwapEvent(
        address account,
        address token,
        uint256 baseAmount,
        uint256 quoteAmount,
        bool isBuy,
        IBondingCurveMinimal curve
    ) internal {
        int256 baseDelta = isBuy ? int256(baseAmount) : -int256(baseAmount);
        int256 quoteDelta = isBuy ? -int256(quoteAmount) : int256(quoteAmount);

        emit Swap({
            buyer: account,
            token: token,
            baseDelta: baseDelta,
            quoteDelta: quoteDelta,
            nextAmountSold: curve.baseSoldFromCurve(token), // Current state after trade
            newPrice: curve.quoteBoughtByCurve(token),
            eventNonce: LaunchpadEventNonce.inc()
        });
    }

    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    function sortTokens(address tokenA, address tokenB) internal pure returns (address token0, address token1) {
        if (tokenA == tokenB) revert("UniswapV2Library: IDENTICAL_ADDRESSES");
        (token0, token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        if (token0 == address(0)) revert("UniswapV2Library: ZERO_ADDRESS");
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

// Local types, libs, contracts, and interfaces
import {CLOB, ICLOB} from "./CLOB.sol";
import {Side, OrderId} from "./types/Order.sol";
import {MakerCredit} from "./types/TransientMakerData.sol";
import {ICLOBManager, ConfigParams, SettingsParams} from "./ICLOBManager.sol";
import {FeeTiers} from "./types/FeeData.sol";
import {CLOBStorageLib, MarketConfig, MarketSettings, MIN_MIN_LIMIT_ORDER_AMOUNT_BASE} from "./types/Book.sol";

// Internal package libs and interfaces
import {IAccountManager} from "../account-manager/IAccountManager.sol";
import {EventNonceLib as CLOBEventNonce} from "contracts/utils/types/EventNonce.sol";

// Solady and OZ imports
import {Initializable} from "@solady/utils/Initializable.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {OwnableRoles as CLOBAdminOwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {IERC20Metadata} from "@openzeppelin/token/ERC20/extensions/IERC20Metadata.sol";
import {BeaconProxy, IBeacon} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";

struct CLOBManagerStorage {
    mapping(address clob => bool) isCLOB;
    mapping(bytes32 tokenPairHash => address) clob;
    mapping(address account => bool) maxLimitWhitelist;
}

using CLOBManagerStorageLib for CLOBManagerStorage global;

/// @custom:storage-location erc7201:CLOBManagerStorage
library CLOBManagerStorageLib {
    bytes32 constant CLOB_MANAGER_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("CLOBManagerStorage")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev Gets the storage slot of the storage struct for the contract calling this library function
    // slither-disable-next-line uninitialized-storage
    function getCLOBManagerStorage() internal pure returns (CLOBManagerStorage storage self) {
        bytes32 position = CLOB_MANAGER_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := position
        }
    }
}

/**
 * @title CLOBManager
 * @notice Main contract that handles CLOB admin functionality and fee calculations
 */
contract CLOBManager is ICLOBManager, CLOBAdminOwnableRoles, Initializable {
    using FixedPointMathLib for uint256;
    using SafeTransferLib for address;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x1e4f7d8c
    error InvalidPair();
    /// @dev sig: 0x8fc6f59b
    error MarketExists();
    /// @dev sig: 0xe591f33d
    error InvalidSettings();
    /// @dev sig: 0x1eb00b06
    error InvalidTokenAddress();
    /// @dev sig: 0x353f2237
    error AdminPanelArrayLengthsInvalid();
    /// @dev sig: 0xf9f68635
    error MarketUnauthorized();
    /// @dev sig: 0x6fbe54bd
    error InvalidBeaconAddress();
    /// @dev sig: 0x19ae8c78
    error CLOBBeaconMustHaveRouter();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    event MarketCreated(
        uint256 indexed eventNonce,
        address indexed creator,
        address indexed baseToken,
        address quoteToken,
        address market,
        uint8 quoteDecimals,
        uint8 baseDecimals,
        ConfigParams config,
        SettingsParams settings
    );

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                CONSTANTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev The abi version of this impl so the indexer can handle event-changing upgrades
    uint256 public constant ABI_VERSION = 1;

    /// @dev Create and call markets to edit their settings
    uint256 public constant MARKET_MANAGER = 1;
    /// @dev Sets users' fee tiers in this contract
    uint256 public constant FEE_TIER_SETTER = 1 << 1;
    /// @dev Whitelists addresses to bypass the markets' max limits per txn
    uint256 public constant MAX_LIMIT_WHITELISTER = 1 << 2;
    /// @dev Clears expired orders from markets
    uint256 public constant EXPIRED_ORDER_CLEARER = 1 << 3;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            IMMUTABLE STATE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev The beacon proxy containing the logic implementation all clobs' storage use
    address public immutable beacon;
    /// @dev The external AccountManager contract
    IAccountManager public immutable accountManager;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            CONSTRUCTOR
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    constructor(address _beacon, address _accountManager) {
        if (_beacon == address(0)) revert InvalidBeaconAddress();
        beacon = _beacon;
        accountManager = IAccountManager(_accountManager);
        _disableInitializers();
    }

    /// @dev Initializes the contract following ERC1967Factory pattern
    function initialize(address _owner) external initializer {
        _initializeOwner(_owner);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            EXTERNAL GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Gets the market address for a given `tokenA` and `tokenB`
    function getMarketAddress(address tokenA, address tokenB) external view returns (address marketAddress) {
        return _getStorage().clob[_getTokenHash(tokenA, tokenB)];
    }

    /// @notice Gets if `market` is a clob created by this factory
    function isMarket(address market) external view returns (bool) {
        return _getStorage().isCLOB[market];
    }

    /// @notice Gets whether an account is exempt from max limits
    function getMaxLimitExempt(address account) external view returns (bool) {
        return _getStorage().maxLimitWhitelist[account];
    }

    /// @notice Gets the current event nonce
    function getEventNonce() external view returns (uint256) {
        return CLOBEventNonce.getCurrentNonce();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            ADMIN FUNCTIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Creates a new market for `quoteToken` and `baseToken` using beacon proxy
    function createMarket(address baseToken, address quoteToken, SettingsParams calldata settings)
        external
        virtual
        onlyOwnerOrRoles(MARKET_MANAGER)
        returns (address marketAddress)
    {
        _assertValidTokenPair(quoteToken, baseToken);

        uint8 quoteDecimals = IERC20Metadata(quoteToken).decimals();
        uint8 baseDecimals = IERC20Metadata(baseToken).decimals();

        ConfigParams memory config;

        config.quoteToken = quoteToken;
        config.baseToken = baseToken;
        config.quoteSize = 10 ** quoteDecimals;
        config.baseSize = 10 ** baseDecimals;

        _assertValidSettings(settings, config.baseSize);

        CLOBManagerStorage storage self = _getStorage();

        bytes32 tokenPairHash = _getTokenHash(quoteToken, baseToken);

        if (self.clob[tokenPairHash] > address(0)) revert MarketExists();

        bytes memory initData = abi.encodeWithSelector(
            CLOB.initialize.selector,
            MarketConfig({
                quoteToken: config.quoteToken,
                baseToken: config.baseToken,
                quoteSize: config.quoteSize,
                baseSize: config.baseSize
            }),
            MarketSettings({
                status: true,
                maxLimitsPerTx: settings.maxLimitsPerTx,
                minLimitOrderAmountInBase: settings.minLimitOrderAmountInBase,
                tickSize: settings.tickSize,
                lotSizeInBase: settings.lotSizeInBase
            }),
            settings.owner
        );

        // Beacon is immutable and itself non upgradeable
        marketAddress = address(new BeaconProxy(beacon, initData));

        self.isCLOB[marketAddress] = true;
        self.clob[tokenPairHash] = marketAddress;

        // Register the market in AccountManager
        accountManager.registerMarket(marketAddress);

        _emitMarketCreated(msg.sender, marketAddress, quoteDecimals, baseDecimals, config, settings);
    }

    /// @notice Sets the tick size for a market
    function setTickSize(ICLOB market, uint256 newTickSize) external onlyOwnerOrRoles(MARKET_MANAGER) {
        market.setTickSize(newTickSize);
    }

    /// @notice Sets the lot size for a market
    function setLotSizeInBase(ICLOB market, uint256 newLotSize) external onlyOwnerOrRoles(MARKET_MANAGER) {
        market.setLotSizeInBase(newLotSize);
    }

    /// @notice Sets the min limit order amount in base for a market
    function setMinLimitOrderAmountInBase(ICLOB market, uint256 newMinLimitOrderAmountInBase)
        external
        onlyOwnerOrRoles(MARKET_MANAGER)
    {
        market.setMinLimitOrderAmountInBase(newMinLimitOrderAmountInBase);
    }

    /// @notice Clears out expired orders from one side of a market
    function adminCancelExpiredOrders(ICLOB market, OrderId[] calldata ids, Side side)
        external
        onlyOwnerOrRoles(EXPIRED_ORDER_CLEARER)
    {
        market.adminCancelExpiredOrders(ids, side);
    }

    /// @notice Sets fee tiers for accounts
    function setAccountFeeTiers(address[] calldata accounts, FeeTiers[] calldata feeTiers)
        external
        onlyOwnerOrRoles(FEE_TIER_SETTER)
    {
        accountManager.setSpotAccountFeeTiers(accounts, feeTiers);
    }

    /// @notice Sets max limit exemptions for accounts
    function setMaxLimitsExempt(address[] calldata accounts, bool[] calldata toggles)
        external
        onlyOwnerOrRoles(MAX_LIMIT_WHITELISTER)
    {
        if (accounts.length != toggles.length) revert AdminPanelArrayLengthsInvalid();

        CLOBManagerStorage storage self = _getStorage();
        for (uint256 i = 0; i < accounts.length; i++) {
            self.maxLimitWhitelist[accounts[i]] = toggles[i];
        }
    }

    /// @notice Sets the max limits per tx for a market
    function setMaxLimitsPerTx(ICLOB market, uint8 newMaxLimits) external onlyOwnerOrRoles(MARKET_MANAGER) {
        market.setMaxLimitsPerTx(newMaxLimits);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            INTERNAL ASSERTIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Checks config and settings params are within correct bounds
    function _assertValidSettings(SettingsParams calldata settings, uint256 baseSize) internal pure {
        if (settings.maxLimitsPerTx == 0) revert InvalidSettings();
        if (settings.minLimitOrderAmountInBase < MIN_MIN_LIMIT_ORDER_AMOUNT_BASE) revert InvalidSettings();
        if (settings.minLimitOrderAmountInBase < settings.lotSizeInBase) revert InvalidSettings();
        if (settings.tickSize.fullMulDiv(settings.lotSizeInBase, baseSize) == 0) revert InvalidSettings();
    }

    /// @dev Performs sanity checks on the addresses passed to make it slightly more difficult to deploy a broken market
    function _assertValidTokenPair(address quoteToken, address baseToken) internal pure {
        if (quoteToken == baseToken) revert InvalidPair();
        if (quoteToken == address(0)) revert InvalidTokenAddress();
        if (baseToken == address(0)) revert InvalidTokenAddress();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            PRIVATE HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Event helper that prevents stack from blowing without IR
    function _emitMarketCreated(
        address creator,
        address marketAddress,
        uint8 quoteDecimals,
        uint8 baseDecimals,
        ConfigParams memory config,
        SettingsParams calldata settings
    ) internal {
        emit MarketCreated(
            CLOBEventNonce.inc(),
            creator,
            config.baseToken,
            config.quoteToken,
            marketAddress,
            quoteDecimals,
            baseDecimals,
            config,
            settings
        );
    }

    /// @dev Gets the token hash which can be used as a UID for a market
    function _getTokenHash(address tokenA, address tokenB) internal pure returns (bytes32) {
        (tokenA, tokenB) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);

        return keccak256(abi.encodePacked(tokenA, tokenB));
    }

    /// @dev Helper to set the storage slot of the storage struct for this contract
    function _getStorage() internal pure returns (CLOBManagerStorage storage ds) {
        return CLOBManagerStorageLib.getCLOBManagerStorage();
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";

import {AdminPanel} from "./modules/AdminPanel.sol";
import {LiquidatorPanel} from "./modules/LiquidatorPanel.sol";
import {ViewPort} from "./modules/ViewPort.sol";

import {ClearingHouse, ClearingHouseLib} from "./types/ClearingHouse.sol";
import {Market, MarketLib} from "./types/Market.sol";
import {Position} from "./types/Position.sol";
import {StorageLib} from "./types/StorageLib.sol";
import {CLOBLib} from "./types/CLOBLib.sol";

import {Side, TiF, BookType, TradeType} from "./types/Enums.sol";
import {PlaceOrderArgs, PlaceOrderResult, AmendLimitOrderArgs} from "./types/Structs.sol";

import {IAccountManager} from "..//account-manager/IAccountManager.sol";

import {OperatorHelperLib} from "../utils/types/OperatorHelperLib.sol";
import {OperatorPanel, OperatorStorage, OperatorStorageLib, PerpsOperatorRoles} from "../utils/OperatorPanel.sol";

/// CONCURRENCY TODO ///
// @todo make nonces market-specific
// @todo isolate insurance payments, claims, and balance per market (will have to also make liquidations per market)
contract PerpManager is AdminPanel, LiquidatorPanel, ViewPort, OperatorPanel {
    using OperatorHelperLib for OperatorStorage;
    using FixedPointMathLib for uint256;
    using SafeCastLib for uint256;

    event PositionLeverageSet(
        bytes32 indexed asset,
        address indexed account,
        uint256 indexed subaccount,
        uint256 newLeverage,
        int256 collateralDelta,
        int256 newMargin,
        uint256 nonce
    );

    event MarginAdded(
        address indexed account, uint256 indexed subaccount, uint256 amount, int256 newMargin, uint256 nonce
    );
    event MarginRemoved(
        address indexed account, uint256 indexed subaccount, uint256 amount, int256 newMargin, uint256 nonce
    );

    error RemainingMarginInsufficient();
    error InvalidDeposit();
    error InvalidWithdraw();
    error NotAccountManager();
    error InvalidBackstopLimitOrder();

    constructor(address _accountManager, address _operatorHub) OperatorPanel(_operatorHub) {
        accountManager = IAccountManager(_accountManager);
        _disableInitializers();
    }

    IAccountManager immutable accountManager;

    struct __UpdateLeverageCache__ {
        DynamicArrayLib.DynamicArray assets;
        Position[] positions;
        int256 fundingPayment;
        uint256 currentLeverage;
        uint256 orderbookNotional;
        uint256 newOrderbookMargin;
        uint256 currentOrderbookMargin;
        int256 collateralDeltaFromBook;
        uint256 newMargin;
    }

    struct __MarginUpdateCache__ {
        DynamicArrayLib.DynamicArray assets;
        Position[] positions;
        int256 fundingPayment;
        uint256 intendedMargin;
    }

    modifier onlySenderOrOperator(address account, PerpsOperatorRoles requiredRole) {
        OperatorStorageLib.getOperatorStorage().onlySenderOrOperator(account, requiredRole);
        _;
    }

    modifier onlyActiveProtocol() override (AdminPanel, LiquidatorPanel) {
        if (!StorageLib.loadClearingHouse().active) revert ProtocolNotActive();
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            FREE COLLATERAL
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function deposit(address account, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.DEPOSIT_ACCOUNT)
    {
        StorageLib.loadCollateralManager().depositFreeCollateral(account, account, amount);
    }

    function withdraw(address account, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.WITHDRAW_ACCOUNT)
    {
        StorageLib.loadCollateralManager().withdrawFreeCollateral(account, amount);
    }

    function depositTo(address account, uint256 amount) external {
        StorageLib.loadCollateralManager().depositFreeCollateral({
            from: msg.sender,
            to: account,
            amount: amount
        });
    }

    function depositFromSpot(address account, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.SPOT_TO_PERP_DEPOSIT)
    {
        accountManager.withdrawToPerps(account, amount);
        StorageLib.loadCollateralManager().depositFromSpot(account, amount);
    }

    function withdrawToSpot(address account, uint256 amount) external {
        if (msg.sender != address(accountManager)) revert NotAccountManager();
        StorageLib.loadCollateralManager().withdrawToSpot(account, amount, address(accountManager));
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 MARGIN
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function addMargin(address account, uint256 subaccount, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.DEPOSIT_MARGIN)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        __MarginUpdateCache__ memory cache;

        // load account
        (cache.assets, cache.positions) = clearingHouse.getAccount(account, subaccount);

        if (amount == 0) revert InvalidDeposit();
        if (cache.positions.length == 0) revert InvalidDeposit();

        // realize funding payment
        cache.fundingPayment = ClearingHouseLib.realizeFundingPayment(cache.assets, cache.positions);

        // settle margin update
        int256 remainingMargin = StorageLib.loadCollateralManager().settleMarginUpdate({
            account: account,
            subaccount: subaccount,
            marginDelta: amount.toInt256(),
            fundingPayment: cache.fundingPayment
        });

        // assert not liquidatable
        clearingHouse.assertNotLiquidatable({assets: cache.assets, positions: cache.positions, margin: remainingMargin});

        // set position update (note: this will just be the new position.lastCumulativeFunding)
        clearingHouse.setPositions({
            tradedAsset: "",
            account: account,
            subaccount: subaccount,
            assets: cache.assets,
            positions: cache.positions
        });

        emit MarginAdded(account, subaccount, amount, remainingMargin, StorageLib.incNonce());
    }

    function removeMargin(address account, uint256 subaccount, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.WITHDRAW_MARGIN)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        __MarginUpdateCache__ memory cache;

        // load account
        (cache.assets, cache.positions) = clearingHouse.getAccount(account, subaccount);

        if (amount == 0) revert InvalidWithdraw();
        if (cache.positions.length == 0) revert InvalidWithdraw();

        // realize funding payment
        cache.fundingPayment = ClearingHouseLib.realizeFundingPayment(cache.assets, cache.positions);

        // settle margin update
        int256 remainingMargin = StorageLib.loadCollateralManager().settleMarginUpdate({
            account: account,
            subaccount: subaccount,
            marginDelta: -amount.toInt256(),
            fundingPayment: cache.fundingPayment
        });

        // assert post withdraw margin requirement (margin + upnl) >= max(intendedMargin, totalNotional / 10)
        // where intendedMargin is the sum of notional / leverage for open positions
        clearingHouse.assertPostWithdrawalMarginRequired({
            assets: cache.assets,
            positions: cache.positions,
            margin: remainingMargin
        });

        // set position update (note: this will just be the new position.lastCumulativeFunding)
        clearingHouse.setPositions({
            tradedAsset: "",
            account: account,
            subaccount: subaccount,
            assets: cache.assets,
            positions: cache.positions
        });

        emit MarginRemoved(account, subaccount, amount, remainingMargin, StorageLib.incNonce());
    }

    function setPositionLeverage(bytes32 asset, address account, uint256 subaccount, uint256 newLeverage)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.SET_LEVERAGE)
        returns (int256 collateralDelta)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();
        Market storage market = clearingHouse.market[asset];

        MarketLib.assertActive(asset);
        MarketLib.assertMaxLeverage(asset, newLeverage);

        __UpdateLeverageCache__ memory cache;

        // handle collateral delta for book oi
        cache.currentLeverage = market.getPositionLeverage(account, subaccount);
        cache.orderbookNotional = market.orderbookNotional[account][subaccount];

        cache.newOrderbookMargin = cache.orderbookNotional.fullMulDiv(1e18, newLeverage);
        cache.currentOrderbookMargin = cache.orderbookNotional.fullMulDiv(1e18, cache.currentLeverage);

        cache.collateralDeltaFromBook = cache.newOrderbookMargin.toInt256() - cache.currentOrderbookMargin.toInt256();

        // set new leverage before loading account
        market.position[account][subaccount].leverage = newLeverage;

        // empty position
        if (market.position[account][subaccount].amount == 0) {
            StorageLib.loadCollateralManager().handleCollateralDelta({
                account: account,
                collateralDelta: cache.collateralDeltaFromBook
            });

            // margin doesn't change on leverage update for empty positions
            int256 margin = StorageLib.loadCollateralManager().getMarginBalance(account, subaccount);

            emit PositionLeverageSet(
                asset, account, subaccount, newLeverage, cache.collateralDeltaFromBook, margin, StorageLib.incNonce()
            );

            return cache.collateralDeltaFromBook;
        }

        // load account
        (cache.assets, cache.positions) = clearingHouse.getAccount(account, subaccount);

        // realize funding payment
        cache.fundingPayment = ClearingHouseLib.realizeFundingPayment(cache.assets, cache.positions);

        cache.newMargin = clearingHouse.getIntendedMargin(cache.assets, cache.positions);

        // assert open margin requirement met
        clearingHouse.assertOpenMarginRequired({
            assets: cache.assets,
            positions: cache.positions,
            margin: cache.newMargin.toInt256()
        });

        clearingHouse.setPositions({
            tradedAsset: "",
            account: account,
            subaccount: subaccount,
            assets: cache.assets,
            positions: cache.positions
        });

        // settle delta between new and prev margin & new and prev orderbook collateral
        collateralDelta = StorageLib.loadCollateralManager().settleNewLeverage({
            account: account,
            subaccount: subaccount,
            collateralDeltaFromBook: cache.collateralDeltaFromBook,
            newMargin: cache.newMargin.toInt256(),
            fundingPayment: cache.fundingPayment
        });

        emit PositionLeverageSet(
            asset, account, subaccount, newLeverage, collateralDelta, cache.newMargin.toInt256(), StorageLib.incNonce()
        );
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              ORDER PLACE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function placeOrder(address account, PlaceOrderArgs calldata args)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (PlaceOrderResult memory result)
    {
        return StorageLib.loadClearingHouse().placeOrder(account, args, BookType.STANDARD);
    }

    function postLimitOrderBackstop(address account, PlaceOrderArgs calldata args)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (PlaceOrderResult memory result)
    {
        if (args.tif != TiF.MOC) revert InvalidBackstopLimitOrder();

        return StorageLib.loadClearingHouse().placeOrder(account, args, BookType.BACKSTOP);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                          ORDER AMEND / CANCEL
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function amendLimitOrder(address account, AmendLimitOrderArgs calldata args)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (int256 collateralDelta)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        collateralDelta = clearingHouse.market[args.asset].amendLimitOrder(account, args, BookType.STANDARD);

        StorageLib.loadCollateralManager().handleCollateralDelta({account: account, collateralDelta: collateralDelta});
    }

    function cancelLimitOrders(bytes32 asset, address account, uint256 subaccount, uint256[] calldata orderIds)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (uint256 refund)
    {
        refund = CLOBLib.cancel(asset, account, subaccount, orderIds, BookType.STANDARD);

        StorageLib.loadCollateralManager().handleCollateralDelta({account: account, collateralDelta: -refund.toInt256()});
    }

    function amendLimitOrderBackstop(address account, AmendLimitOrderArgs calldata args)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (int256 collateralDelta)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        collateralDelta = clearingHouse.market[args.asset].amendLimitOrder(account, args, BookType.BACKSTOP);

        StorageLib.loadCollateralManager().handleCollateralDelta({account: account, collateralDelta: collateralDelta});
    }

    function cancelLimitOrdersBackstop(bytes32 asset, address account, uint256 subaccount, uint256[] calldata orderIds)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (uint256 refund)
    {
        refund = CLOBLib.cancel(asset, account, subaccount, orderIds, BookType.BACKSTOP);

        StorageLib.loadCollateralManager().handleCollateralDelta({account: account, collateralDelta: -refund.toInt256()});
    }

    function cancelConditionalOrders(address account, uint256[] calldata nonces)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        for (uint256 i; i < nonces.length; i++) {
            clearingHouse.nonceUsed[account][nonces[i]] = true;
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               HELPER
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _getCollateral(uint256 baseAmount, uint256 price, uint256 leverage)
        private
        pure
        returns (uint256 collateral)
    {
        collateral = baseAmount.fullMulDiv(price, 1e18).fullMulDiv(1e18, leverage);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {IOperatorPanel} from "./interfaces/IOperatorPanel.sol";
import {EventNonceLib as OperatorEventNonce} from "./types/EventNonce.sol";

// @todo rename "spot" to "account"
enum SpotOperatorRoles {
    ADMIN,
    PLACE_ORDER,
    SPOT_DEPOSIT,
    SPOT_WITHDRAW,
    PERP_TO_SPOT_DEPOSIT,
    LAUNCHPAD_FILL
}

enum PerpsOperatorRoles {
    ADMIN,
    PLACE_ORDER,
    SET_LEVERAGE,
    DEPOSIT_MARGIN,
    WITHDRAW_MARGIN,
    DEPOSIT_ACCOUNT,
    WITHDRAW_ACCOUNT,
    SPOT_TO_PERP_DEPOSIT
}

struct OperatorStorage {
    mapping(address account => mapping(address operator => uint256)) operatorRoleApprovals;
}

using OperatorStorageLib for OperatorStorage global;

/// @custom:storage-location erc7201:OperatorStorage
library OperatorStorageLib {
    bytes32 constant OPERATOR_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("OperatorStorage")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev Gets the storage slot of the storage struct for the contract calling this library function
    // slither-disable-next-line uninitialized-storage
    function getOperatorStorage() internal pure returns (OperatorStorage storage self) {
        bytes32 position = OPERATOR_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := position
        }
    }
}

abstract contract OperatorPanel is IOperatorPanel {
    /// @dev sig: 0xb816c81e0d2e75687754a9cb3111541c16ab454792482bf1dd02093f2203f353
    event OperatorApproved(
        uint256 indexed eventNonce, address indexed account, address indexed operator, uint256 newRoles
    );
    /// @dev sig: 0x1145ef8300109b8668d5581d376603c552d28f5aaefa3ca8fb7524286a41a7ae
    event OperatorDisapproved(
        uint256 indexed eventNonce, address indexed account, address indexed operator, uint256 removedRoles
    );

    /// @dev sig: 0x732ea322
    error OperatorDoesNotHaveRole();
    /// @dev sig: 0xe9a05878
    error OperatorChangeUnauthorized();

    address public immutable operatorHub;

    constructor(address operatorHub_) {
        operatorHub = operatorHub_;
    }

    modifier onlySenderOrOperatorHub(address account) {
        if (msg.sender != account && msg.sender != operatorHub) revert OperatorChangeUnauthorized();
        _;
    }

    function _getOperatorStorage() internal pure returns (OperatorStorage storage self) {
        return OperatorStorageLib.getOperatorStorage();
    }

    function getOperatorRoleApprovals(address account, address operator) external view returns (uint256) {
        return _getOperatorStorage().operatorRoleApprovals[account][operator];
    }

    function approveOperator(address account, address operator, uint256 roles)
        external
        onlySenderOrOperatorHub(account)
    {
        OperatorStorage storage self = _getOperatorStorage();

        uint256 approvedRoles = self.operatorRoleApprovals[account][operator];
        self.operatorRoleApprovals[account][operator] = approvedRoles | roles;

        emit OperatorApproved(OperatorEventNonce.inc(), account, operator, roles);
    }

    function disapproveOperator(address account, address operator, uint256 roles)
        external
        onlySenderOrOperatorHub(account)
    {
        OperatorStorage storage self = _getOperatorStorage();

        uint256 approvedRoles = self.operatorRoleApprovals[account][operator];
        self.operatorRoleApprovals[account][operator] = approvedRoles & (~roles);

        emit OperatorDisapproved(OperatorEventNonce.inc(), account, operator, roles);
    }

    function getOperatorEventNonce() external view returns (uint256) {
        return OperatorEventNonce.getCurrentNonce();
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

// Local types, libs, and interfaces
import {ICLOB} from "./ICLOB.sol";
import {ICLOBManager} from "./ICLOBManager.sol";
import {IAccountManager} from "../account-manager/IAccountManager.sol";
import {CLOBStorageLib} from "./types/Book.sol";
import {TransientMakerData, MakerCredit} from "./types/TransientMakerData.sol";
import {Order, OrderLib, OrderId, OrderIdLib, Side} from "./types/Order.sol";
import {Book, BookLib, Limit, MarketConfig, MarketSettings} from "./types/Book.sol";

// Internal package types, libs, and interfaces
import {IOperatorPanel} from "contracts/utils/interfaces/IOperatorPanel.sol";
import {SpotOperatorRoles} from "contracts/utils/OperatorPanel.sol";
import {OperatorHelperLib} from "contracts/utils/types/OperatorHelperLib.sol";
import {EventNonceLib as CLOBEventNonce} from "contracts/utils/types/EventNonce.sol";

// Solady and OZ imports
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin-contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

/**
 * @title CLOB
 * Main spot market contract for trading asset pairs on an orderbook
 */
contract CLOB is ICLOB, Ownable2StepUpgradeable {
    using OrderLib for *;
    using OrderIdLib for uint256;
    using OrderIdLib for address;
    using FixedPointMathLib for uint256;
    using SafeCastLib for uint256;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0xc0208cc462e0f7d7b2329363da41c40e123ba2c9db4b8b03a183140d67ad1c60
    event CancelFailed(uint256 indexed eventNonce, uint256 orderId, address owner);

    /// @dev sig: 0xacb8106c549e32473004de43588b1bd716fc82873c60790caab04149f2cb9466
    event OrderCanceled(
        uint256 indexed eventNonce,
        uint256 indexed orderId,
        address indexed owner,
        uint256 quoteTokenRefunded,
        uint256 baseTokenRefunded,
        CancelType context
    );

    /// @dev sig: 0x06956ad87855e4ad9efb290bad3c7ef7a8c7cff5e28b5926b570b492c45b9c37
    event OrderAmended(
        uint256 indexed eventNonce, Order preAmend, AmendArgs args, int256 quoteTokenDelta, int256 baseTokenDelta
    );

    /// @dev sig: 0x76a9cd4a6124a3883e613ae4146376b48db63cfd526306751587a148642fce56
    event OrderProcessed(
        uint256 indexed eventNonce,
        address indexed account,
        uint256 indexed orderId,
        ICLOB.TiF tif,
        uint256 limitPrice,
        uint256 basePosted,
        int256 quoteDelta,
        int256 baseDelta,
        uint256 takerFee
    );

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x175e7f45
    error ZeroAmend();
    /// @dev sig: 0xb82df155
    error ZeroOrder();
    /// @dev sig: 0x91b373e1
    error AmendInvalid();
    /// @dev sig: 0xc56873ba
    error OrderExpired();
    /// @dev sig: 0xd8a00083
    error ZeroCostTrade();
    /// @dev sig: 0xf1a5cd31
    error FOKOrderNotFilled();
    /// @dev sig: 0xba2ea531
    error AmendUnauthorized();
    /// @dev sig: 0xf99412b1
    error CancelUnauthorized();
    /// @dev sig: 0xd268c85f
    error ManagerUnauthorized();
    /// @dev sig: 0xd093feb7
    error FactoryUnauthorized();
    /// @dev sig: 0x3e27eb6d
    error PostOnlyOrderWouldFill();
    /// @dev sig: 0xb134397c
    error AmendNonPostOnlyInvalid();
    /// @dev sig: 0x315ff5e5
    error MaxOrdersInBookPostNotCompetitive();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                CONSTANTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/


    /// @dev The abi version of this impl so the indexer can handle event-changing upgrades
    uint256 public constant ABI_VERSION = 1;

    /// @dev The global router address available to all CLOBs that can bypass the operator check
    address public immutable gteRouter;
    /// @dev The operator contract for role-based access control (same as accountManager)
    IOperatorPanel public immutable operator;
    /// @dev The factory that created this contract and controls its settings as well as processing maker settlement
    ICLOBManager public immutable factory;
    /// @dev The account manager contract for direct balance operations (and operator checks)
    IAccountManager public immutable accountManager;
    /// @dev Maximum number of maker orders allowed per side of the order book
    /// before the least competitive orders get bumped
    uint256 public immutable maxNumOrdersPerSide;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                MODIFIERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    modifier onlySenderOrOperator(address account, SpotOperatorRoles requiredRole) {
        OperatorHelperLib.onlySenderOrOperator(operator, gteRouter, account, requiredRole);
        _;
    }

    modifier onlyManager() {
        if (msg.sender != address(factory)) revert ManagerUnauthorized();
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                    CONSTRUCTOR AND INITIALIZATION
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor(address _factory, address _gteRouter, address _accountManager, uint256 _maxNumOrdersPerSide) {
        factory = ICLOBManager(_factory);
        gteRouter = _gteRouter;
        operator = IOperatorPanel(_accountManager);
        accountManager = IAccountManager(_accountManager);
        maxNumOrdersPerSide = _maxNumOrdersPerSide;
        _disableInitializers();
    }

    /// @notice Initializes the `marketConfig`, `marketSettings`, and `initialOwner` of the market
    function initialize(MarketConfig memory marketConfig, MarketSettings memory marketSettings, address initialOwner)
        external
        initializer
    {
        __CLOB_init(marketConfig, marketSettings, initialOwner);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            EXTERNAL GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Gets base token
    function getBaseToken() external view returns (address) {
        return _getStorage().config().baseToken;
    }

    /// @notice Gets quote token
    function getQuoteToken() external view returns (address) {
        return _getStorage().config().quoteToken;
    }

    /// @notice Gets the base token amount equivalent to `quoteAmount` at a given `price`
    /// @dev This price does not have to be within tick size
    function getBaseTokenAmount(uint256 price, uint256 quoteAmount) external view returns (uint256) {
        return _getStorage().getBaseTokenAmount(price, quoteAmount);
    }

    /// @notice Gets the quote token amount equivalent to `baseAmount` at a given `price`
    /// @dev This price dos not have to be within tick size
    function getQuoteTokenAmount(uint256 price, uint256 baseAmount) external view returns (uint256) {
        return _getStorage().getQuoteTokenAmount(price, baseAmount);
    }

    /// @notice Gets the market config
    function getMarketConfig() external view returns (MarketConfig memory) {
        return _getStorage().config();
    }

    /// @notice Gets the market settings
    function getMarketSettings() external view returns (MarketSettings memory) {
        return _getStorage().settings();
    }

    /// @notice Gets tick size
    function getTickSize() external view returns (uint256) {
        return _getStorage().settings().tickSize;
    }

    /// @notice Gets lot size in base
    function getLotSizeInBase() external view returns (uint256) {
        return _getStorage().settings().lotSizeInBase;
    }

    /// @notice Gets quote and base open interest
    function getOpenInterest() external view returns (uint256 quoteOi, uint256 baseOi) {
        return (_getStorage().metadata().quoteTokenOpenInterest, _getStorage().metadata().baseTokenOpenInterest);
    }

    /// @notice Gets an order in the book from its id
    function getOrder(uint256 orderId) external view returns (Order memory) {
        return _getStorage().orders[orderId.toOrderId()];
    }

    /// @notice Gets top of book as price (max bid and min ask)
    function getTOB() external view returns (uint256 maxBid, uint256 minAsk) {
        return (_getStorage().getBestBidPrice(), _getStorage().getBestAskPrice());
    }

    /// @notice Gets the bid or ask Limit at a price depending on `side`
    function getLimit(uint256 price, Side side) external view returns (Limit memory) {
        return _getStorage().getLimit(price, side);
    }

    /// @notice Gets total bid limit orders in the book
    function getNumBids() external view returns (uint256) {
        return _getStorage().metadata().numBids;
    }

    /// @notice Gets total ask limit orders in the book
    function getNumAsks() external view returns (uint256) {
        return _getStorage().metadata().numAsks;
    }

    /// @notice Gets a list of orders, starting at an orderId
    function getNextOrders(uint256 startOrderId, uint256 numOrders) external view returns (Order[] memory) {
        return _getStorage().getNextOrders(startOrderId.toOrderId(), numOrders);
    }

    /// @notice Gets the next populated higher price limit to `price` on a side of the book
    function getNextBiggestPrice(uint256 price, Side side) external view returns (uint256) {
        return _getStorage().getNextBiggestPrice(price, side);
    }

    /// @notice Gets the next populated lower price limit to `price` on a side of the book
    function getNextSmallestPrice(uint256 price, Side side) external view returns (uint256) {
        return _getStorage().getNextSmallestPrice(price, side);
    }

    /// @notice Gets the next order id (nonce) that will be used upon placing an order
    /// @dev Placing both limit and fill orders increment the next orderId
    function getNextOrderId() external view returns (uint256) {
        return (_getStorage().metadata().orderIdCounter + 1);
    }

    /// @notice Gets the current event nonce
    function getEventNonce() external view returns (uint256) {
        return CLOBEventNonce.getCurrentNonce();
    }

    /// @notice Gets `pageSize` of orders from TOB down from a `startPrice` and on a given `side` of the book
    function getOrdersPaginated(uint256 startPrice, Side side, uint256 pageSize)
        external
        view
        returns (Order[] memory result, Order memory nextOrder)
    {
        Book storage ds = _getStorage();

        nextOrder = side == Side.BUY
            ? ds.orders[ds.bidLimits[startPrice].headOrder]
            : ds.orders[ds.askLimits[startPrice].headOrder];

        return ds.getOrdersPaginated(nextOrder, pageSize);
    }

    /// @notice Gets `pageSize` of orders from TOB down, starting at `startOrderId`
    function getOrdersPaginated(OrderId startOrderId, uint256 pageSize)
        external
        view
        returns (Order[] memory result, Order memory nextOrder)
    {
        Book storage ds = _getStorage();
        nextOrder = ds.orders[startOrderId];

        return ds.getOrdersPaginated(nextOrder, pageSize);
    }

    function getBaseQuanta() external view returns (uint256) {
        return _getStorage().getBaseQuanta();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            AUTH-ONLY SETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Sets the new max limits per txn
    function setMaxLimitsPerTx(uint8 newMaxLimits) external onlyManager {
        _getStorage().setMaxLimitsPerTx(newMaxLimits);
    }

    /// @notice Sets the tick size of the book
    /// @dev New orders' limit prices % tickSize must be 0
    function setTickSize(uint256 tickSize) external onlyManager {
        _getStorage().setTickSize(tickSize);
    }

    /// @notice Sets the minimum amount an order (in base) must be to be placed on the book
    /// @dev Reducing an order below this amount will cause the order to get cancelled
    function setMinLimitOrderAmountInBase(uint256 newMinLimitOrderAmountInBase) external onlyManager {
        _getStorage().setMinLimitOrderAmountInBase(newMinLimitOrderAmountInBase);
    }

    /// @notice Sets the lot size in base for standardized trade sizes
    /// @dev Orders must be multiples of lot size. Setting to 0 disables lot size restrictions
    function setLotSizeInBase(uint256 newLotSizeInBase) external onlyManager {
        _getStorage().setLotSizeInBase(newLotSizeInBase);
    }

    /// @notice Clears out expired orders from one side of the book
    /// @dev Cancels must be on a single side bc settlement only treats one side (either base or quote)
    /// as a refund and the other side as a fill that incurs trading fees
    function adminCancelExpiredOrders(OrderId[] calldata ids, Side side) external onlyManager returns (bool[] memory) {
        bool[] memory removed = new bool[](ids.length);

        Book storage ds = _getStorage();

        for (uint256 i = 0; i < ids.length; i++) {
            Order storage o = ds.orders[ids[i]];

            if (!o.isExpired() || o.side != side) continue;

            removed[i] = true;
            side == Side.BUY ? _removeExpiredBid(ds, o) : _removeExpiredAsk(ds, o);
        }

        // Virtual taker side is opposite of cancelled orders' side to ensure refunds aren't charged fees
        Side virtualTakerSide = side == Side.BUY ? Side.SELL : Side.BUY;
        _settleIncomingOrder({
            ds: ds,
            account: address(0),
            side: virtualTakerSide,
            quoteTokenAmount: 0,
            baseTokenAmount: 0
        });

        return removed;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        EXTERNAL ORDER PLACEMENT
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function placeOrder(address account, ICLOB.PlaceOrderArgs calldata args)
        external
        onlySenderOrOperator(account, SpotOperatorRoles.PLACE_ORDER)
        returns (ICLOB.PlaceOrderResult memory)
    {
        Book storage ds = _getStorage();

        // inc order nonce regardless if the order is a pure take, or a custom id is used
        uint256 orderId = ds.incrementOrderId();

        if (args.clientOrderId > 0) {
            orderId = account.getClientOrderId(args.clientOrderId);
            ds.assertUnusedOrderId(orderId);
        }

        Order memory newOrder = args.toOrderChecked(orderId, account);

        // Fires an {OrderProcessed} event at the end of either sub-routines
        if (args.side == Side.BUY) return _processBid(ds, account, newOrder, args);
        else return _processAsk(ds, account, newOrder, args);
    }

    /// @notice Amends an existing order for `account`
    function amend(address account, AmendArgs calldata args)
        external
        onlySenderOrOperator(account, SpotOperatorRoles.PLACE_ORDER)
        returns (int256 quoteDelta, int256 baseDelta)
    {
        Book storage ds = _getStorage();
        Order storage order = ds.orders[args.orderId.toOrderId()];

        if (order.id.unwrap() == 0) revert OrderLib.OrderNotFound();
        if (order.owner != account) revert AmendUnauthorized();

        ds.assertLimitPriceInBounds(args.price);
        ds.assertMakeAmountInBounds(args.amountInBase);

        if (args.cancelTimestamp.isExpired()) revert AmendInvalid();

        // Update order
        (quoteDelta, baseDelta) = _processAmend(ds, order, args);
    }

    /// @notice Cancels a list of orders for `account`
    function cancel(address account, CancelArgs memory args)
        external
        onlySenderOrOperator(account, SpotOperatorRoles.PLACE_ORDER)
        returns (uint256, uint256)
    {
        Book storage ds = _getStorage();
        (address quoteToken, address baseToken) = (ds.config().quoteToken, ds.config().baseToken);

        (uint256 totalQuoteTokenRefunded, uint256 totalBaseTokenRefunded) = _executeCancel(ds, account, args);

        if (totalBaseTokenRefunded > 0) accountManager.creditAccount(account, baseToken, totalBaseTokenRefunded);
        if (totalQuoteTokenRefunded > 0) accountManager.creditAccount(account, quoteToken, totalQuoteTokenRefunded);

        return (totalQuoteTokenRefunded, totalBaseTokenRefunded);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            INTERNAL FILL LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Performs matching and settlement for a bid order
    function _processBid(Book storage ds, address account, Order memory newOrder, ICLOB.PlaceOrderArgs calldata args)
        internal
        returns (ICLOB.PlaceOrderResult memory res)
    {
        (uint256 postAmount, uint256 totalQuoteSent, uint256 totalBaseReceived) =
            _executeBid(ds, newOrder, args.tif, args.baseDenominated);

        if (postAmount + totalQuoteSent + totalBaseReceived == 0) revert ZeroOrder();

        if (totalBaseReceived != totalQuoteSent && (totalBaseReceived == 0 || totalQuoteSent == 0)) {
            revert ZeroCostTrade();
        }

        uint256 takerFee = _settleIncomingOrder(ds, account, Side.BUY, totalQuoteSent + postAmount, totalBaseReceived);

        // Populate result struct
        res.account = account;
        res.orderId = newOrder.id.unwrap();
        res.quoteTokenAmountTraded = -int256(totalQuoteSent);
        res.baseTokenAmountTraded = int256(totalBaseReceived);
        res.takerFee = takerFee;

        // Sets the base posted of the remainder of the order that was a make
        // Order amount is always converted to base when posting
        if (uint8(args.tif) <= 1) res.basePosted = newOrder.amount;

        // Set whether this was a market order (limitPrice = max) or limit order
        res.wasMarketOrder = (args.limitPrice == type(uint256).max);

        emit OrderProcessed({
            eventNonce: CLOBEventNonce.inc(),
            account: account,
            orderId: res.orderId,
            tif: args.tif,
            limitPrice: args.limitPrice,
            basePosted: res.basePosted,
            quoteDelta: res.quoteTokenAmountTraded,
            baseDelta: res.baseTokenAmountTraded,
            takerFee: takerFee
        });
    }

    function _processAsk(Book storage ds, address account, Order memory newOrder, ICLOB.PlaceOrderArgs calldata args)
        internal
        returns (ICLOB.PlaceOrderResult memory res)
    {
        (uint256 postAmount, uint256 totalQuoteReceived, uint256 totalBaseSent) =
            _executeAsk(ds, newOrder, args.tif, args.baseDenominated);

        if (postAmount + totalQuoteReceived + totalBaseSent == 0) revert ZeroOrder();

        if (totalBaseSent != totalQuoteReceived && (totalBaseSent == 0 || totalQuoteReceived == 0)) {
            revert ZeroCostTrade();
        }

        uint256 takerFee = _settleIncomingOrder(ds, account, Side.SELL, totalQuoteReceived, totalBaseSent + postAmount);

        // Populate result struct
        res.account = account;
        res.orderId = newOrder.id.unwrap();
        res.quoteTokenAmountTraded = int256(totalQuoteReceived);
        res.baseTokenAmountTraded = -int256(totalBaseSent);
        res.takerFee = takerFee;

        // Sets the base posted of the remainder of the order that was a make
        // Order amount is always converted to base when posting
        if (uint8(args.tif) <= 1) res.basePosted = newOrder.amount;

        // Set whether this was a market order (limitPrice = 0) or limit order
        res.wasMarketOrder = (args.limitPrice == 0);

        emit OrderProcessed({
            eventNonce: CLOBEventNonce.inc(),
            account: account,
            orderId: res.orderId,
            tif: args.tif,
            limitPrice: args.limitPrice,
            basePosted: res.basePosted,
            quoteDelta: res.quoteTokenAmountTraded,
            baseDelta: res.baseTokenAmountTraded,
            takerFee: takerFee
        });
    }

    /// @dev Performs the core matching and placement of a bid order into the book
    function _executeBid(Book storage ds, Order memory newOrder, ICLOB.TiF tif, bool baseDenominated)
        internal
        returns (uint256 postAmount, uint256 totalQuoteSent, uint256 totalBaseReceived)
    {
        // Attempt to fill any of the incoming order that's overlapping into asks
        if (ds.getBestAskPrice() <= newOrder.price) {
            if (tif == ICLOB.TiF.MOC) revert PostOnlyOrderWouldFill();
            (totalQuoteSent, totalBaseReceived) = _matchIncomingBid(ds, newOrder, baseDenominated);
        }

        if (tif == ICLOB.TiF.FOK && newOrder.amount > 0) revert FOKOrderNotFilled();

        bool isTake = false;
        (isTake, newOrder.amount) = _getTakeOrPostAmount(
            ds, tif, newOrder.amount, totalQuoteSent | totalBaseReceived > 0, baseDenominated, newOrder.price
        );

        // The order was a TAKE only either due to TIF settings,
        // or because a partially filled GTC had insufficient remaining amount in base
        if (isTake) return (0, totalQuoteSent, totalBaseReceived);

        // Validate price and amount bounds
        ds.assertLimitPriceInBounds(newOrder.price);

        // // Enforce per-tx max limit placements (unless exempt) and increment counter
        ds.incrementLimitsPlaced(address(factory), msg.sender);

        // The book is full, pop the least competitive order (or revert if incoming is the least competitive)
        if (ds.metadata().numBids == maxNumOrdersPerSide) {
            uint256 minBidPrice = ds.getWorstBidPrice();
            if (newOrder.price <= minBidPrice) revert MaxOrdersInBookPostNotCompetitive();

            _removeNonCompetitiveOrder(ds, ds.orders[ds.bidLimits[minBidPrice].tailOrder]);
        }

        ds.addOrderToBook(newOrder);
        postAmount = ds.getQuoteTokenAmount(newOrder.price, newOrder.amount);

        return (postAmount, totalQuoteSent, totalBaseReceived);
    }

    function _executeAsk(Book storage ds, Order memory newOrder, ICLOB.TiF tif, bool baseDenominated)
        internal
        returns (uint256 postAmount, uint256 totalQuoteReceived, uint256 totalBaseSent)
    {
        // Attempt to fill any of the incoming order that's overlapping into bids
        if (ds.getBestBidPrice() >= newOrder.price) {
            if (tif == ICLOB.TiF.MOC) revert PostOnlyOrderWouldFill();
            (totalQuoteReceived, totalBaseSent) = _matchIncomingAsk(ds, newOrder, baseDenominated);
        }

        if (tif == ICLOB.TiF.FOK && newOrder.amount > 0) revert FOKOrderNotFilled();

        bool isTake = false;
        (isTake, newOrder.amount) = _getTakeOrPostAmount(
            ds, tif, newOrder.amount, totalQuoteReceived | totalBaseSent > 0, baseDenominated, newOrder.price
        );

        // The order was a TAKE only either due to TIF settings,
        // or because a partially filled GTC had insufficient remaining amount in base
        if (isTake) return (0, totalQuoteReceived, totalBaseSent);

        // Validate price and amount bounds
        ds.assertLimitPriceInBounds(newOrder.price);

        // Enforce per-tx max limit placements (unless exempt) and increment counter
        ds.incrementLimitsPlaced(address(factory), msg.sender);

        // The book is full, pop the least competitive order (or revert if incoming is the least competitive)
        if (ds.metadata().numAsks == maxNumOrdersPerSide) {
            uint256 maxAskPrice = ds.getWorstAskPrice();
            if (newOrder.price >= maxAskPrice) revert MaxOrdersInBookPostNotCompetitive();

            _removeNonCompetitiveOrder(ds, ds.orders[ds.askLimits[maxAskPrice].tailOrder]);
        }

        ds.addOrderToBook(newOrder);
        postAmount = newOrder.amount;

        return (postAmount, totalQuoteReceived, totalBaseSent);
    }

    function _getTakeOrPostAmount(
        Book storage ds,
        ICLOB.TiF tif,
        uint256 remainingAmount,
        bool matchOccurred,
        bool baseDenominated,
        uint256 limitPrice
    ) internal view returns (bool isTake, uint256 postAmount) {
        // Order is explicitly a TAKE
        if (tif == ICLOB.TiF.FOK || tif == ICLOB.TiF.IOC) return (true, 0);

        // Order amount must be in base and conform to current lot size before posting
        remainingAmount = baseDenominated
            ? ds.boundToLots(remainingAmount)
            : ds.boundToLots(ds.getBaseTokenAmount(remainingAmount, limitPrice));

        // There is not enough order amount to post
        if (remainingAmount < ds.settings().minLimitOrderAmountInBase) {
            // If a GTC had any take, and the remaining amount is invalid,
            // this is permissible as a full take instead of reverting
            if (tif == ICLOB.TiF.GTC && matchOccurred) return (true, 0);

            // The make-only order violates minimum amount
            revert BookLib.LimitOrderAmountInvalid();
        }

        return (false, remainingAmount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL AMEND LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Performs the amending of an order
    function _processAmend(Book storage ds, Order storage order, AmendArgs calldata args)
        internal
        returns (int256 quoteTokenDelta, int256 baseTokenDelta)
    {
        Order memory preAmend = order;
        address maker = preAmend.owner;

        if (args.cancelTimestamp.isExpired() || args.amountInBase < ds.settings().minLimitOrderAmountInBase) {
            revert AmendInvalid();
        }

        // Check lot size compliance after other validations
        ds.assertLotSizeCompliant(args.amountInBase);

        if (order.side != args.side || order.price != args.price) {
            // change place in book
            (quoteTokenDelta, baseTokenDelta) = _executeAmendNewOrder(ds, order, args);
        } else if (order.amount != args.amountInBase) {
            // change amount
            (quoteTokenDelta, baseTokenDelta) =
                _executeAmendAmount(ds, order, args.amountInBase, uint32(args.cancelTimestamp));
        } else if (args.cancelTimestamp != order.cancelTimestamp) {
            order.cancelTimestamp = uint32(args.cancelTimestamp);
        } else {
            revert ZeroAmend();
        }

        emit OrderAmended(CLOBEventNonce.inc(), preAmend, args, quoteTokenDelta, baseTokenDelta);

        _settleAmend(ds, maker, quoteTokenDelta, baseTokenDelta);
    }

    /// @dev Performs the removal and replacement of an amended order with a new price or side
    function _executeAmendNewOrder(Book storage ds, Order storage order, AmendArgs calldata args)
        internal
        returns (int256 quoteTokenDelta, int256 baseTokenDelta)
    {
        Order memory newOrder;

        newOrder.owner = order.owner;
        newOrder.id = order.id;
        newOrder.side = args.side;
        newOrder.price = args.price;
        newOrder.amount = args.amountInBase;
        newOrder.cancelTimestamp = uint32(args.cancelTimestamp);

        if (order.side == Side.BUY) quoteTokenDelta = ds.getQuoteTokenAmount(order.price, order.amount).toInt256();
        else baseTokenDelta = order.amount.toInt256();

        ds.removeOrderFromBook(order);

        uint256 postAmount;
        if (args.side == Side.BUY) {
            (postAmount,,) = _executeBid(ds, newOrder, ICLOB.TiF.MOC, true);

            quoteTokenDelta -= postAmount.toInt256();
        } else {
            (postAmount,,) = _executeAsk(ds, newOrder, ICLOB.TiF.MOC, true);

            baseTokenDelta -= postAmount.toInt256();
        }
    }

    /// @dev Performs the updating of an amended order with a new amount
    function _executeAmendAmount(Book storage ds, Order storage order, uint256 amount, uint32 cancelTimestamp)
        internal
        returns (int256 quoteTokenDelta, int256 baseTokenDelta)
    {
        if (order.side == Side.BUY) {
            int256 oldAmountInQuote = ds.getQuoteTokenAmount(order.price, order.amount).toInt256();
            int256 newAmountInQuote = ds.getQuoteTokenAmount(order.price, amount).toInt256();

            quoteTokenDelta = oldAmountInQuote - newAmountInQuote;

            ds.metadata().quoteTokenOpenInterest =
                uint256(ds.metadata().quoteTokenOpenInterest.toInt256() - quoteTokenDelta);
        } else {
            baseTokenDelta = order.amount.toInt256() - amount.toInt256();

            ds.metadata().baseTokenOpenInterest =
                uint256(ds.metadata().baseTokenOpenInterest.toInt256() - baseTokenDelta);
        }

        order.amount = amount;
        order.cancelTimestamp = cancelTimestamp;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL MATCHING LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Internal struct to prevent blowing stack
    struct __MatchData__ {
        uint256 matchedAmount;
        uint256 baseDelta;
        uint256 quoteDelta;
    }

    /// @dev Match incoming bid order to best asks
    function _matchIncomingBid(Book storage ds, Order memory incomingOrder, bool amountIsBase)
        internal
        returns (uint256 totalQuoteSent, uint256 totalBaseReceived)
    {
        uint256 bestAskPrice = ds.getBestAskPrice();

        while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) {
            Limit storage limit = ds.askLimits[bestAskPrice];
            Order storage bestAskOrder = ds.orders[limit.headOrder];

            if (bestAskOrder.isExpired()) {
                _removeExpiredAsk(ds, bestAskOrder);
                bestAskPrice = ds.getBestAskPrice();
                continue;
            }

            // slither-disable-next-line uninitialized-local
            __MatchData__ memory currMatch =
                _matchIncomingOrder(ds, bestAskOrder, incomingOrder, bestAskPrice, amountIsBase);

            incomingOrder.amount -= currMatch.matchedAmount;

            totalQuoteSent += currMatch.quoteDelta;
            totalBaseReceived += currMatch.baseDelta;

            bestAskPrice = ds.getBestAskPrice();
        }
    }

    /// @dev Match incoming ask order to best bids
    function _matchIncomingAsk(Book storage ds, Order memory incomingOrder, bool amountIsBase)
        internal
        returns (uint256 totalQuoteReceived, uint256 totalBaseSent)
    {
        uint256 bestBidPrice = ds.getBestBidPrice();

        while (bestBidPrice >= incomingOrder.price && incomingOrder.amount > 0) {
            Limit storage limit = ds.bidLimits[bestBidPrice];
            Order storage bestBidOrder = ds.orders[limit.headOrder];

            if (bestBidOrder.isExpired()) {
                _removeExpiredBid(ds, bestBidOrder);
                bestBidPrice = ds.getBestBidPrice();
                continue;
            }

            // slither-disable-next-line uninitialized-local
            __MatchData__ memory currMatch =
                _matchIncomingOrder(ds, bestBidOrder, incomingOrder, bestBidPrice, amountIsBase);

            incomingOrder.amount -= currMatch.matchedAmount;

            totalQuoteReceived += currMatch.quoteDelta;
            totalBaseSent += currMatch.baseDelta;

            bestBidPrice = ds.getBestBidPrice();
        }
    }

    function _boundMakerToLotSize(Book storage ds, Order storage order, uint256 lotSize) internal {
        uint256 remainder = order.amount % lotSize;
        if (remainder == 0) return;

        if (remainder == order.amount) {
            if (order.side == Side.BUY) _removeExpiredBid(ds, order);
            else _removeExpiredAsk(ds, order);
            return;
        }

        if (order.side == Side.BUY) {
            uint256 quoteTokenAmount = ds.getQuoteTokenAmount(order.price, remainder);
            TransientMakerData.addQuoteToken(order.owner, quoteTokenAmount);

            ds.metadata().quoteTokenOpenInterest -= quoteTokenAmount;
        } else {
            TransientMakerData.addBaseToken(order.owner, remainder);
            ds.metadata().baseTokenOpenInterest -= remainder;
        }
        order.amount -= remainder;
    }

    /// @dev Matches an incoming order to its next counterparty order, crediting the maker and removing the counterparty order if fully filled
    function _matchIncomingOrder(
        Book storage ds,
        Order storage makerOrder,
        Order memory takerOrder,
        uint256 matchedPrice,
        bool amountIsBase
    ) internal returns (__MatchData__ memory matchData) {
        uint256 lotSize = ds.settings().lotSizeInBase;

        _boundMakerToLotSize(ds, makerOrder, lotSize);
        uint256 matchedBase = makerOrder.amount;

        if (amountIsBase) {
            // denominated in base
            matchData.baseDelta = (matchedBase.min(takerOrder.amount) / lotSize) * lotSize;
            matchData.quoteDelta = ds.getQuoteTokenAmount(matchedPrice, matchData.baseDelta);
            matchData.matchedAmount = matchData.baseDelta != matchedBase ? takerOrder.amount : matchData.baseDelta;
        } else {
            // denominated in quote
            matchData.baseDelta =
                (matchedBase.min(ds.getBaseTokenAmount(matchedPrice, takerOrder.amount)) / lotSize) * lotSize;
            matchData.quoteDelta = ds.getQuoteTokenAmount(matchedPrice, matchData.baseDelta);
            matchData.matchedAmount = matchData.baseDelta != matchedBase ? takerOrder.amount : matchData.quoteDelta;
        }

        // Early return if no tradeable amount due to lot size constraints (dust)
        if (matchData.baseDelta == 0) return matchData;

        bool orderRemoved = matchData.baseDelta == matchedBase;

        // Handle token accounting for maker.
        if (takerOrder.side == Side.BUY) {
            TransientMakerData.addQuoteToken(makerOrder.owner, matchData.quoteDelta);

            if (!orderRemoved) ds.metadata().baseTokenOpenInterest -= matchData.baseDelta;
        } else {
            TransientMakerData.addBaseToken(makerOrder.owner, matchData.baseDelta);

            if (!orderRemoved) ds.metadata().quoteTokenOpenInterest -= matchData.quoteDelta;
        }

        if (orderRemoved) ds.removeOrderFromBook(makerOrder);
        else makerOrder.amount -= matchData.baseDelta;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL EXPIRY LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Removes an expired ask, adding the order's amount to settlement data as a base refund
    function _removeExpiredAsk(Book storage ds, Order storage order) internal {
        uint256 baseTokenAmount = order.amount;

        // We can add the refund to maker fills because both cancelled asks and filled bids are credited in baseTokens
        TransientMakerData.addBaseToken(order.owner, baseTokenAmount);

        ds.removeOrderFromBook(order);
    }

    /// @dev Removes an expired bid, adding the order's amount to settlement as a quote refund
    function _removeExpiredBid(Book storage ds, Order storage order) internal {
        uint256 quoteTokenAmount = ds.getQuoteTokenAmount(order.price, order.amount);

        // We can add the refund to maker fills because both cancelled bids and filled asks are credited in quoteTokens
        TransientMakerData.addQuoteToken(order.owner, quoteTokenAmount);

        ds.removeOrderFromBook(order);
    }

    /// @notice Removes the least competitive order from the book
    function _removeNonCompetitiveOrder(Book storage ds, Order storage order) internal {
        uint256 quoteRefunded;
        uint256 baseRefunded;
        if (order.side == Side.BUY) {
            quoteRefunded = ds.getQuoteTokenAmount(order.price, order.amount);
            accountManager.creditAccountNoEvent(order.owner, address(ds.config().quoteToken), quoteRefunded);
        } else {
            baseRefunded = order.amount;
            accountManager.creditAccountNoEvent(order.owner, address(ds.config().baseToken), baseRefunded);
        }

        emit OrderCanceled(
            CLOBEventNonce.inc(),
            order.id.unwrap(),
            order.owner,
            quoteRefunded,
            baseRefunded,
            CancelType.NON_COMPETITIVE
        );

        ds.removeOrderFromBook(order);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL CANCEL LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Performs the cancellation of an account's orders
    function _executeCancel(Book storage ds, address account, CancelArgs memory args)
        internal
        returns (uint256 totalQuoteTokenRefunded, uint256 totalBaseTokenRefunded)
    {
        uint256 numOrders = args.orderIds.length;
        for (uint256 i = 0; i < numOrders; i++) {
            uint256 orderId = args.orderIds[i];
            Order storage order = ds.orders[orderId.toOrderId()];

            if (order.isNull()) {
                emit CancelFailed(CLOBEventNonce.inc(), orderId, account);
                continue; // Order may have been matched
            } else if (order.owner != account) {
                revert CancelUnauthorized();
            }

            uint256 quoteTokenRefunded = 0;
            uint256 baseTokenRefunded = 0;

            if (order.side == Side.BUY) {
                quoteTokenRefunded = ds.getQuoteTokenAmount(order.price, order.amount);
                totalQuoteTokenRefunded += quoteTokenRefunded;
            } else {
                baseTokenRefunded = order.amount;
                totalBaseTokenRefunded += baseTokenRefunded;
            }

            ds.removeOrderFromBook(order);

            uint256 eventNonce = CLOBEventNonce.inc();
            emit OrderCanceled(eventNonce, orderId, account, quoteTokenRefunded, baseTokenRefunded, CancelType.USER);
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL SETTLEMENT LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Settles token accounting in the factory for the incoming trade
    function _settleIncomingOrder(
        Book storage ds,
        address account,
        Side side,
        uint256 quoteTokenAmount,
        uint256 baseTokenAmount
    ) internal returns (uint256 takerFee) {
        SettleParams memory settleParams;

        (settleParams.quoteToken, settleParams.baseToken) = (ds.config().quoteToken, ds.config().baseToken);

        settleParams.taker = account;
        settleParams.side = side;

        settleParams.takerQuoteAmount = quoteTokenAmount;
        settleParams.takerBaseAmount = baseTokenAmount;

        settleParams.makerCredits = TransientMakerData.getMakerCreditsAndClearStorage();

        return accountManager.settleIncomingOrder(settleParams);
    }

    /// @dev Settles the token deltas in the factory from an amend
    function _settleAmend(Book storage ds, address maker, int256 quoteTokenDelta, int256 baseTokenDelta) internal {
        if (quoteTokenDelta > 0) {
            accountManager.creditAccount(maker, address(ds.config().quoteToken), uint256(quoteTokenDelta));
        } else if (quoteTokenDelta < 0) {
            accountManager.debitAccount(maker, address(ds.config().quoteToken), uint256(-quoteTokenDelta));
        }

        if (baseTokenDelta > 0) {
            accountManager.creditAccount(maker, address(ds.config().baseToken), uint256(baseTokenDelta));
        } else if (baseTokenDelta < 0) {
            accountManager.debitAccount(maker, address(ds.config().baseToken), uint256(-baseTokenDelta));
        }
    }

    // This naming reflects OZ initializer naming
    // slither-disable-next-line naming-convention
    function __CLOB_init(MarketConfig memory marketConfig, MarketSettings memory marketSettings, address initialOwner)
        internal
    {
        __Ownable_init(initialOwner);
        CLOBStorageLib.init(_getStorage(), marketConfig, marketSettings);
    }

    /// @dev Helper to assign the storage slot to the Book struct
    function _getStorage() internal pure returns (Book storage) {
        return CLOBStorageLib._getCLOBStorage();
    }
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

