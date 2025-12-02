
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

