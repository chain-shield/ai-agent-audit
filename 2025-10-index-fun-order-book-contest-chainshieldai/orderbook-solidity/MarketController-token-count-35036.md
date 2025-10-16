
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "./IMarket.sol";
import "./IMarketController.sol";
import "./IMarketResolver.sol";
import "../Token/IPositionTokens.sol";
import "../Vault/IVault.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {EIP712Upgradeable} from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";

/**
 * @title MarketController
 * @notice Upgradeable orderbook-based prediction market controller with EIP-712 order matching
 * @dev Coordinates between position tokens, vault, and market resolver contracts with UUPS upgradeability
 */
contract MarketController is
    IMarketController,
    Initializable,
    UUPSUpgradeable,
    OwnableUpgradeable,
    ReentrancyGuardUpgradeable,
    EIP712Upgradeable
{
    using ECDSA for bytes32;

    // EIP-712 Type Hash
    bytes32 private constant ORDER_TYPEHASH = keccak256(
        "Order(address user,bytes32 questionId,uint256 outcome,uint256 amount,uint256 price,uint256 nonce,uint256 expiration,bool isBuyOrder)"
    );

    /// @notice Maximum fee rate in basis points (10% = 1000 bp)
    uint256 public constant MAX_FEE_RATE = 1000;

    /// @notice Position tokens contract for minting/burning
    IPositionTokens public positionTokens;

    /// @notice Market resolver for proof verification
    IMarketResolver public marketResolver;

    /// @notice Vault contract for collateral management
    IVault public vault;

    /// @notice Market contract for metadata
    IMarket public market;

    /// @notice Oracle address for condition ID generation
    address public oracle;

    /// @notice Global trading pause state (emergency stop)
    bool public globalTradingPaused;

    /// @notice Per-market trading pause state (for trading hours control)
    mapping(bytes32 => bool) public marketTradingPaused;

    /// @notice Authorized matchers for order matching
    mapping(address => bool) public authorizedMatchers;

    /// @notice Order management for EIP-712
    mapping(bytes32 => uint256) public filledAmounts; // orderHash => filled amount

    /// @notice Fee rate in basis points (e.g., 400 = 4%)
    uint256 public feeRate;

    /// @notice Trade fee rate in basis points (e.g., 100 = 1%)
    uint256 public tradeFeeRate;

    /// @notice Treasury address where fees are sent
    address public treasury;

    /// @notice Custom fee rates for specific users (0 = use default rate)
    mapping(address => uint256) public userFeeRate;

    /// @notice Custom trade fee rates for specific users (0 = use default rate)
    mapping(address => uint256) public userTradeFeeRate;

    /// @dev Gap for future storage variables
    uint256[31] private __gap;

    modifier onlyAuthorizedMatcher() {
        require(authorizedMatchers[msg.sender], "Not authorized matcher");
        _;
    }

    /// @dev Restricts betting to markets that are still open
    modifier onlyOpenMarket(bytes32 questionId) {
        require(market.isMarketOpen(questionId), "Market is closed for betting");
        _;
    }

    /// @dev Restricts trading when paused (global or market-specific)
    modifier whenTradingActive(bytes32 questionId) {
        require(!globalTradingPaused, "Global trading paused");
        require(!marketTradingPaused[questionId], "Market trading paused");
        _;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the upgradeable contract
     * @param _initialOwner Initial owner of the contract
     * @param _positionTokens Address of position tokens contract
     * @param _marketResolver Address of market resolver contract
     * @param _vault Address of vault contract
     * @param _marketContract Address of market contract
     * @param _oracle Address of oracle for condition generation
     */
    function initialize(
        address _initialOwner,
        address _positionTokens,
        address _marketResolver,
        address _vault,
        address _marketContract,
        address _oracle
    ) public initializer {
        require(_positionTokens != address(0), "Invalid position tokens");
        require(_marketResolver != address(0), "Invalid market resolver");
        require(_vault != address(0), "Invalid vault");
        require(_marketContract != address(0), "Invalid market contract");
        require(_oracle != address(0), "Invalid oracle");

        __Ownable_init(_initialOwner);
        __UUPSUpgradeable_init();
        __ReentrancyGuard_init();
        __EIP712_init("PredictionMarketOrders", "1");

        positionTokens = IPositionTokens(_positionTokens);
        marketResolver = IMarketResolver(_marketResolver);
        vault = IVault(_vault);
        market = IMarket(_marketContract);
        oracle = _oracle;

        // Initialize with no fee and no treasury
        feeRate = 0;
        treasury = address(0);
    }

    /**
     * @notice Authorizes contract upgrades
     * @param newImplementation Address of the new implementation
     * @dev Only callable by contract owner
     */
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    // ============ Contract Management Functions ============

    /**
     * @notice Updates the position tokens contract address
     * @param _positionTokens New position tokens contract address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updatePositionTokens(address _positionTokens) external onlyOwner {
        require(_positionTokens != address(0), "Invalid position tokens");
        positionTokens = IPositionTokens(_positionTokens);
    }

    /**
     * @notice Updates the market resolver contract address
     * @param _marketResolver New market resolver contract address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateMarketResolver(address _marketResolver) external onlyOwner {
        require(_marketResolver != address(0), "Invalid market resolver");
        marketResolver = IMarketResolver(_marketResolver);
    }

    /**
     * @notice Updates the vault contract address
     * @param _vault New vault contract address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateVault(address _vault) external onlyOwner {
        require(_vault != address(0), "Invalid vault");
        vault = IVault(_vault);
    }

    /**
     * @notice Updates the market contract address
     * @param _market New market contract address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateMarket(address _market) external onlyOwner {
        require(_market != address(0), "Invalid market");
        market = IMarket(_market);
    }

    /**
     * @notice Updates the oracle address
     * @param _oracle New oracle address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateOracle(address _oracle) external onlyOwner {
        require(_oracle != address(0), "Invalid oracle");
        oracle = _oracle;
    }

    // ============ Fee Management Functions ============

    /**
     * @notice Sets the default fee rate for winnings claims
     * @param _feeRate Fee rate in basis points (e.g., 400 = 4%)
     * @dev Only callable by contract owner, maximum 10% to prevent abuse
     */
    function setFeeRate(uint256 _feeRate) external onlyAuthorizedMatcher {
        require(_feeRate <= MAX_FEE_RATE, "Fee rate exceeds maximum");

        uint256 oldRate = feeRate;
        feeRate = _feeRate;

        emit FeeRateUpdated(oldRate, _feeRate);
    }

    /**
     * @notice Sets the treasury address where fees are sent
     * @param _treasury Address of the treasury
     * @dev Only callable by contract owner
     */
    function setTreasury(address _treasury) external onlyAuthorizedMatcher {
        require(_treasury != address(0), "Invalid treasury address");

        address oldTreasury = treasury;
        treasury = _treasury;

        emit TreasuryUpdated(oldTreasury, _treasury);
    }

    /**
     * @notice Sets a custom fee rate for specific user (fee tier system)
     * @param user Address of the user
     * @param _feeRate Custom fee rate in basis points (0 = use default rate)
     * @dev Only callable by contract owner, allows preferential rates for MMs/whales
     */
    function setUserFeeRate(address user, uint256 _feeRate) external onlyAuthorizedMatcher {
        require(_feeRate <= MAX_FEE_RATE, "Fee rate exceeds maximum");

        userFeeRate[user] = _feeRate;

        emit UserFeeRateSet(user, _feeRate);
    }

    /**
     * @notice Gets the effective fee rate for a user
     * @param user Address of the user
     * @return Fee rate in basis points that will be applied to this user
     */
    function getEffectiveFeeRate(address user) external view returns (uint256) {
        return _getEffectiveFeeRate(user);
    }

    /**
    * @notice Sets the default trade fee rate
    * @param _tradeFeeRate Fee rate in basis points (e.g., 100 = 1%)
    * @dev Only callable by authorized matcher, maximum 10%
    */
    function setTradeFeeRate(uint256 _tradeFeeRate) external onlyAuthorizedMatcher {
        require(_tradeFeeRate <= MAX_FEE_RATE, "Fee rate exceeds maximum");

        uint256 oldRate = tradeFeeRate;
        tradeFeeRate = _tradeFeeRate;

        emit TradeFeeRateUpdated(oldRate, _tradeFeeRate);
    }

    /**
    * @notice Sets a custom trade fee rate for specific user
    * @param user Address of the user
    * @param _tradeFeeRate Custom fee rate in basis points (0 = use default rate)
    * @dev Only callable by authorized matcher
    */
    function setUserTradeFeeRate(address user, uint256 _tradeFeeRate) external onlyAuthorizedMatcher {
        require(_tradeFeeRate <= MAX_FEE_RATE, "Fee rate exceeds maximum");

        userTradeFeeRate[user] = _tradeFeeRate;

        emit UserTradeFeeRateSet(user, _tradeFeeRate);
    }

    /**
    * @notice Gets the effective trade fee rate for a user
    * @param user Address of the user
    * @return Fee rate in basis points that will be applied to this user's trades
    */
    function getEffectiveTradeFeeRate(address user) external view returns (uint256) {
        return _getEffectiveTradeFeeRate(user);
    }

    // ============ Trading Hours Control Functions ============

    /**
     * @notice Sets global trading pause state (emergency stop)
     * @param paused True to pause all trading, false to resume
     * @dev Only callable by contract owner, affects all markets
     */
    function setGlobalTradingPaused(bool paused) external onlyAuthorizedMatcher {
        globalTradingPaused = paused;
        emit GlobalTradingPauseChanged(paused);
    }

    /**
     * @notice Sets trading pause state for specific market
     * @param questionId Market question identifier
     * @param paused True to pause trading, false to resume
     * @dev Only callable by contract owner, for trading hours control
     */
    function setMarketTradingPaused(bytes32 questionId, bool paused) external onlyAuthorizedMatcher {
        marketTradingPaused[questionId] = paused;
        emit MarketTradingPauseChanged(questionId, paused);
    }

    /**
     * @notice Sets trading pause state for multiple markets (batch operation)
     * @param questionIds Array of market question identifiers
     * @param paused True to pause trading, false to resume
     * @dev Only callable by contract owner, gas-efficient for bulk updates
     */
    function batchSetMarketTradingPaused(bytes32[] calldata questionIds, bool paused) external onlyAuthorizedMatcher {
        for (uint256 i = 0; i < questionIds.length; i++) {
            marketTradingPaused[questionIds[i]] = paused;
        }
        emit BatchMarketTradingPauseChanged(questionIds, paused);
    }

    /**
     * @notice Checks if trading is active for a specific market
     * @param questionId Market question identifier
     * @return True if trading is active (not paused globally or for this market)
     */
    function isTradingActive(bytes32 questionId) external view returns (bool) {
        return !globalTradingPaused && !marketTradingPaused[questionId];
    }

    // ============ Market Management Functions ============

    /**
    * @notice Creates a new market with optional time-based resolution and epoch rolling
    * @param questionId Unique market question identifier
    * @param outcomeCount Number of possible outcomes
    * @param resolutionTime Timestamp when market should resolve (0 for manual resolution)
    * @param epochDuration Duration of each epoch in seconds (0 for manual epoch advancement)
    * @dev Only callable by authorized matchers
    * @dev For continuous markets: set resolutionTime=0, epochDuration=86400 (daily rolling)
    * @dev For legacy markets: set epochDuration=0 (requires manual advanceEpoch calls)
    *
    * Examples:
    * - Daily continuous market: resolutionTime=0, epochDuration=86400
    * - Weekly continuous market: resolutionTime=0, epochDuration=604800
    * - One-time event: resolutionTime=futureTimestamp, epochDuration=0
    */
    function createMarket(
        bytes32 questionId,
        uint256 outcomeCount,
        uint256 resolutionTime,
        uint256 epochDuration
    ) external onlyAuthorizedMatcher {
        market.createMarket(questionId, outcomeCount, resolutionTime, epochDuration);
    }

    /**
    * @notice Updates resolution time for existing market
    * @param questionId Market question identifier
    * @param resolutionTime New resolution timestamp (0 for manual resolution)
    * @dev Only callable by authorized matchers, only before current resolution time
    */
    function updateMarketResolutionTime(bytes32 questionId, uint256 resolutionTime) external onlyAuthorizedMatcher {
        market.updateResolutionTime(questionId, resolutionTime);
    }

    /**
    * @notice Advances market to next epoch (manual mode only)
    * @param questionId Market question identifier
    * @dev Only callable by authorized matchers
    * @dev Only works for manual epoch markets (epochDuration = 0)
    * @dev Time-based markets advance automatically, calling this will revert
    */
    function advanceMarketEpoch(bytes32 questionId) external onlyAuthorizedMatcher {
        market.advanceEpoch(questionId);
    }

    // ============ Order Matching Functions ============

    /**
     * @notice Executes a trade using EIP-712 signed orders
     * @param buyOrder The buy order details
     * @param sellOrder The sell order details
     * @param buySignature The buyer's signature
     * @param sellSignature The seller's signature
     * @param fillAmount Amount to fill (must not exceed either order)
     */
    function executeOrderMatch(
        IMarketController.Order calldata buyOrder,
        IMarketController.Order calldata sellOrder,
        bytes calldata buySignature,
        bytes calldata sellSignature,
        uint256 fillAmount
    )
        external
        onlyAuthorizedMatcher
        nonReentrant
        onlyOpenMarket(buyOrder.questionId)
        whenTradingActive(buyOrder.questionId)
    {
        require(buyOrder.questionId == sellOrder.questionId, "Question ID mismatch");
        require(buyOrder.outcome == sellOrder.outcome, "Outcome mismatch");
        require(buyOrder.isBuyOrder && !sellOrder.isBuyOrder, "Order type mismatch");
        require(buyOrder.price >= sellOrder.price, "Price mismatch");
        require(fillAmount > 0, "Invalid fill amount");

        // Verify both signatures
        bytes32 buyOrderHash = _verifyOrder(buyOrder, buySignature);
        bytes32 sellOrderHash = _verifyOrder(sellOrder, sellSignature);

        // Check fill amounts don't exceed remaining
        require(filledAmounts[buyOrderHash] + fillAmount <= buyOrder.amount, "Buy order overfilled");
        require(filledAmounts[sellOrderHash] + fillAmount <= sellOrder.amount, "Sell order overfilled");

        // Update filled amounts
        filledAmounts[buyOrderHash] += fillAmount;
        filledAmounts[sellOrderHash] += fillAmount;

        // Execute the trade with settlement mode detection
        _executeTrade(buyOrder, sellOrder, fillAmount);

        uint256 currentEpoch = market.getCurrentEpoch(buyOrder.questionId);

        emit OrderFilled(
            buyOrderHash,
            buyOrder.user,
            buyOrder.questionId,
            buyOrder.outcome,
            fillAmount,
            sellOrder.price, // Execution price
            true, // isBuyOrder
            currentEpoch,
            sellOrder.user // taker
        );

        emit OrderFilled(
            sellOrderHash,
            sellOrder.user,
            sellOrder.questionId,
            sellOrder.outcome,
            fillAmount,
            sellOrder.price, // Execution price
            false, // isBuyOrder
            currentEpoch,
            buyOrder.user // taker
        );
    }

    /**
     * @notice Executes a single signed order against matcher's liquidity
     * @param order The order to execute
     * @param signature The user's signature
     * @param fillAmount Amount to fill
     * @param counterparty Address providing liquidity (authorized matcher)
     */
    function executeSingleOrder(
        IMarketController.Order calldata order,
        bytes calldata signature,
        uint256 fillAmount,
        address counterparty
    )
        external
        onlyAuthorizedMatcher
        nonReentrant
        onlyOpenMarket(order.questionId)
        whenTradingActive(order.questionId)
    {
        require(fillAmount > 0, "Invalid fill amount");

        bytes32 orderHash = _verifyOrder(order, signature);

        // Check fill amount doesn't exceed remaining
        require(filledAmounts[orderHash] + fillAmount <= order.amount, "Order overfilled");

        // Update filled amount
        filledAmounts[orderHash] += fillAmount;

        // Execute against matcher's liquidity
        _executeAgainstMatcher(order, fillAmount, counterparty);

        uint256 currentEpoch = market.getCurrentEpoch(order.questionId);

        emit OrderFilled(
            orderHash,
            order.user,
            order.questionId,
            order.outcome,
            fillAmount,
            order.price, // Execution price
            order.isBuyOrder,
            currentEpoch,
            counterparty // taker
        );
    }

    /**
     * @notice Allows users to cancel their own orders
     * @param order The order to cancel
     * @param signature The user's signature (for verification)
     */
    function cancelOrder(IMarketController.Order calldata order, bytes calldata signature) external {
        require(order.user == _msgSender(), "Not order owner");

        bytes32 orderHash = _verifyOrder(order, signature);

        // Mark as fully filled to prevent execution
        filledAmounts[orderHash] = order.amount;

        emit OrderCancelled(orderHash, order.user);
    }

    /**
     * @notice Get the EIP-712 hash for an order
     */
    function getOrderHash(IMarketController.Order calldata order) external view returns (bytes32) {
        return _hashTypedDataV4(keccak256(abi.encode(
            ORDER_TYPEHASH,
            order.user,
            order.questionId,
            order.outcome,
            order.amount,
            order.price,
            order.nonce,
            order.expiration,
            order.isBuyOrder
        )));
    }

    /**
     * @notice Check how much of an order has been filled
     */
    function getOrderFillAmount(bytes32 orderHash) external view returns (uint256) {
        return filledAmounts[orderHash];
    }

    /**
     * @notice Check remaining amount for an order
     */
    function getOrderRemainingAmount(IMarketController.Order calldata order) external view returns (uint256) {
        bytes32 orderHash = _hashTypedDataV4(keccak256(abi.encode(
            ORDER_TYPEHASH,
            order.user,
            order.questionId,
            order.outcome,
            order.amount,
            order.price,
            order.nonce,
            order.expiration,
            order.isBuyOrder
        )));

        uint256 filled = filledAmounts[orderHash];
        return filled >= order.amount ? 0 : order.amount - filled;
    }

    // ============ Claims and Resolution Functions ============

    /**
     * @notice Claims winnings from resolved market with fee collection
     * @param questionId Market question identifier
     * @param epoch Specific epoch for the claim
     * @param outcome Winning outcome being claimed
     * @param merkleProof Proof of winning outcome
     * @return Payout amount transferred to user (after fees)
     * @dev Claims are always allowed regardless of trading hours
     */
    function claimWinnings(bytes32 questionId, uint256 epoch, uint256 outcome, bytes32[] calldata merkleProof)
        external
        nonReentrant
        returns (uint256)
    {
        require(epoch > 0, "Invalid epoch");
        require(market.getCurrentEpoch(questionId) >= epoch, "Epoch not reached");

        uint256 numberOfOutcomes = market.getOutcomeCount(questionId);
        bytes32 conditionId = market.getConditionId(oracle, questionId, numberOfOutcomes, epoch);

        require(marketResolver.getResolutionStatus(conditionId), "Market not resolved");

        uint256 tokenId = positionTokens.getTokenId(conditionId, outcome);
        uint256 userBalance = positionTokens.balanceOf(_msgSender(), tokenId);
        require(userBalance > 0, "No tokens to claim");

        // Verify the outcome won using proof verification
        require(marketResolver.verifyProof(conditionId, outcome, merkleProof), "Invalid proof");

        // Calculate fees and net payout
        (uint256 netPayout, uint256 feeAmount) = _calculatePayoutAndFee(_msgSender(), userBalance);

        // Burn tokens
        positionTokens.burn(_msgSender(), tokenId, userBalance);

        // Handle fee collection and payout
        _processPayout(conditionId, _msgSender(), questionId, netPayout, feeAmount);

        emit WinningsClaimed(_msgSender(), questionId, epoch, outcome, netPayout);
        return netPayout;
    }

    /**
     * @notice Claims winnings from multiple resolved markets in a single transaction with fee collection
     * @param claims Array of claim requests for different markets/epochs
     * @return totalPayout Total payout amount transferred to user across all claims (after fees)
     * @dev Processes all valid claims and skips invalid ones, enabling users to claim all winnings efficiently
     */
    function batchClaimWinnings(ClaimRequest[] calldata claims)
        external
        nonReentrant
        returns (uint256 totalPayout)
    {
        require(claims.length > 0, "No claims provided");
        require(claims.length <= 50, "Too many claims"); // Gas limit protection

        address user = _msgSender();
        uint256 validClaims = 0;

        // Arrays for batch operations
        bytes32[] memory conditionIds = new bytes32[](claims.length);
        address[] memory users = new address[](claims.length);
        uint256[] memory netAmounts = new uint256[](claims.length);
        uint256[] memory feeAmounts = new uint256[](claims.length);
        uint256[] memory tokenIds = new uint256[](claims.length);
        uint256[] memory grossAmounts = new uint256[](claims.length);

        // Process each claim
        for (uint256 i = 0; i < claims.length; i++) {
            ClaimRequest memory claim = claims[i];

            // Validate basic parameters
            if (claim.epoch == 0) continue;
            if (market.getCurrentEpoch(claim.questionId) < claim.epoch) continue;

            uint256 numberOfOutcomes = market.getOutcomeCount(claim.questionId);
            if (numberOfOutcomes == 0) continue; // Market doesn't exist

            bytes32 conditionId = market.getConditionId(oracle, claim.questionId, numberOfOutcomes, claim.epoch);

            // Check if market is resolved
            if (!marketResolver.getResolutionStatus(conditionId)) continue;

            uint256 tokenId = positionTokens.getTokenId(conditionId, claim.outcome);
            uint256 userBalance = positionTokens.balanceOf(user, tokenId);

            // Skip if user has no tokens for this claim
            if (userBalance == 0) continue;

            // Verify the outcome won using proof verification
            if (!marketResolver.verifyProof(conditionId, claim.outcome, claim.merkleProof)) continue;

            // Calculate fees for this claim
            (uint256 netPayout, uint256 feeAmount) = _calculatePayoutAndFee(user, userBalance);

            // Add to batch arrays
            conditionIds[validClaims] = conditionId;
            users[validClaims] = user;
            netAmounts[validClaims] = netPayout;
            feeAmounts[validClaims] = feeAmount;
            tokenIds[validClaims] = tokenId;
            grossAmounts[validClaims] = userBalance; // Store original amount for burning

            totalPayout += netPayout;
            validClaims++;

            // Emit individual claim event for tracking
            emit WinningsClaimed(user, claim.questionId, claim.epoch, claim.outcome, netPayout);
        }

        require(validClaims > 0, "No valid claims found");

        // Perform batch token burns (burn the original gross amounts)
        for (uint256 i = 0; i < validClaims; i++) {
            positionTokens.burn(user, tokenIds[i], grossAmounts[i]);
        }

        // Prepare arrays for batch vault operations (resize to valid claims only)
        bytes32[] memory finalConditionIds = new bytes32[](validClaims);
        address[] memory finalUsers = new address[](validClaims);
        uint256[] memory finalNetAmounts = new uint256[](validClaims);

        for (uint256 i = 0; i < validClaims; i++) {
            finalConditionIds[i] = conditionIds[i];
            finalUsers[i] = users[i];
            finalNetAmounts[i] = netAmounts[i];
        }

        // Unlock collateral for user (net amounts)
        vault.batchUnlockCollateral(finalConditionIds, finalUsers, finalNetAmounts);

        // Handle fee collection in batch if there are fees and treasury is set
        if (treasury != address(0)) {
            // Prepare arrays for fee collection
            address[] memory treasuryUsers = new address[](validClaims);
            uint256[] memory finalFeeAmounts = new uint256[](validClaims);

            uint256 feeClaims = 0;
            for (uint256 i = 0; i < validClaims; i++) {
                if (feeAmounts[i] > 0) {
                    treasuryUsers[feeClaims] = treasury;
                    finalFeeAmounts[feeClaims] = feeAmounts[i];
                    feeClaims++;
                }
            }

            // Only process fee collection if there are actual fees
            if (feeClaims > 0) {
                // Resize arrays for fee collection
                bytes32[] memory feeConditionIds = new bytes32[](feeClaims);
                address[] memory feeUsers = new address[](feeClaims);
                uint256[] memory feeAmountsArray = new uint256[](feeClaims);

                uint256 feeIndex = 0;
                for (uint256 i = 0; i < validClaims; i++) {
                    if (feeAmounts[i] > 0) {
                        feeConditionIds[feeIndex] = finalConditionIds[i];
                        feeUsers[feeIndex] = treasury;
                        feeAmountsArray[feeIndex] = feeAmounts[i];
                        feeIndex++;
                    }
                }

                vault.batchUnlockCollateral(feeConditionIds, feeUsers, feeAmountsArray);
            }
        }

        emit BatchWinningsClaimed(user, totalPayout, validClaims);
        return totalPayout;
    }

    /**
     * @notice Emergency function to resolve markets that have passed their resolution time
     * @param questionId Market question identifier
     * @param numberOfOutcomes Number of possible outcomes for validation
     * @param merkleRoot Root hash of merkle tree containing valid outcomes
     * @dev Can only be called after resolution time has passed, provides fallback resolution
     */
    function emergencyResolveMarket(bytes32 questionId, uint256 numberOfOutcomes, bytes32 merkleRoot)
        external
        onlyAuthorizedMatcher
    {
        require(market.isMarketReadyForResolution(questionId), "Market not ready for resolution");

        // Validate numberOfOutcomes parameter matches the actual market
        uint256 actualOutcomes = market.getOutcomeCount(questionId);
        require(actualOutcomes > 0, "Market not found");
        require(numberOfOutcomes == actualOutcomes, "Outcome count mismatch");
        require(numberOfOutcomes <= 256, "Maximum 256 outcomes supported");

        uint256 currentEpoch = market.getCurrentEpoch(questionId);
        marketResolver.resolveMarketEpoch(questionId, currentEpoch, numberOfOutcomes, merkleRoot);

        emit EmergencyResolution(questionId, currentEpoch, merkleRoot);
    }

    /**
     * @notice Sets authorization status for off-chain matching systems
     * @param matcher Address of the matching system
     * @param authorized Whether the matcher is authorized
     * @dev Only callable by contract owner
     */
    function setAuthorizedMatcher(address matcher, bool authorized) external onlyOwner {
        authorizedMatchers[matcher] = authorized;
    }

    // ============ Internal Functions ============

    /**
     * @notice Calculates net payout and fee amount for a user
     * @param user Address of the user claiming
     * @param grossPayout Gross payout amount before fees
     * @return netPayout Amount to be paid to user after fees
     * @return feeAmount Amount to be paid to treasury as fee
     */
    function _calculatePayoutAndFee(address user, uint256 grossPayout) internal view returns (uint256 netPayout, uint256 feeAmount) {
        if (grossPayout == 0) {
            return (0, 0);
        }

        uint256 applicableFeeRate = _getEffectiveFeeRate(user);

        if (applicableFeeRate == 0 || treasury == address(0)) {
            return (grossPayout, 0);
        }

        feeAmount = (grossPayout * applicableFeeRate) / 10000;
        netPayout = grossPayout - feeAmount;

        return (netPayout, feeAmount);
    }

    /**
     * @notice Processes payout by unlocking collateral for user and collecting fees
     * @param conditionId Condition identifier for the market
     * @param user User receiving the payout
     * @param questionId Market question identifier for event emission
     * @param netPayout Amount to unlock for user
     * @param feeAmount Amount to unlock for treasury
     */
    function _processPayout(
        bytes32 conditionId,
        address user,
        bytes32 questionId,
        uint256 netPayout,
        uint256 feeAmount
    ) internal {
        // Unlock collateral for user
        vault.unlockCollateral(conditionId, user, netPayout);

        // Collect fee to treasury if applicable
        if (feeAmount > 0 && treasury != address(0)) {
            vault.unlockCollateral(conditionId, treasury, feeAmount);
            emit FeeCollected(user, questionId, feeAmount, netPayout);
        }
    }

    /**
     * @notice Gets the effective fee rate for a user (internal version)
     * @param user Address of the user
     * @return Fee rate in basis points
     */
    function _getEffectiveFeeRate(address user) internal view returns (uint256) {
        return userFeeRate[user] > 0 ? userFeeRate[user] : feeRate;
    }

    /**
    * @notice Gets the effective trade fee rate for a user (internal version)
    * @param user Address of the user
    * @return Fee rate in basis points
    */
    function _getEffectiveTradeFeeRate(address user) internal view returns (uint256) {
        return userTradeFeeRate[user] > 0 ? userTradeFeeRate[user] : tradeFeeRate;
    }

    /**
     * @notice Verifies an order signature and basic validity
     */
    function _verifyOrder(IMarketController.Order calldata order, bytes calldata signature) internal view returns (bytes32) {
        // Check expiration
        require(block.timestamp <= order.expiration, "Order expired");

        // Generate order hash
        bytes32 orderHash = _hashTypedDataV4(keccak256(abi.encode(
            ORDER_TYPEHASH,
            order.user,
            order.questionId,
            order.outcome,
            order.amount,
            order.price,
            order.nonce,
            order.expiration,
            order.isBuyOrder
        )));

        // Verify signature
        address signer = orderHash.recover(signature);
        require(signer == order.user, "Invalid signature");

        // Check if order is already fully filled
        require(filledAmounts[orderHash] < order.amount, "Order fully filled");

        return orderHash;
    }

    /**
     * @notice Executes a trade between two orders with intelligent settlement mode detection
     * @dev Determines whether to use token swap (if seller has tokens) or JIT minting (if not)
     */
    function _executeTrade(IMarketController.Order calldata buyOrder, IMarketController.Order calldata sellOrder, uint256 fillAmount) internal {
        uint256 numberOfOutcomes = market.getOutcomeCount(buyOrder.questionId);
        bytes32 conditionId = market.getConditionId(oracle, buyOrder.questionId, numberOfOutcomes, 0);
        uint256 tokenId = positionTokens.getTokenId(conditionId, buyOrder.outcome);

        // Check if seller has existing tokens (SWAP mode vs JIT MINTING mode)
        uint256 sellerBalance = positionTokens.balanceOf(sellOrder.user, tokenId);

        if (sellerBalance >= fillAmount) {
            // SWAP MODE: Seller has tokens, execute direct token-for-USDC swap
            _executeTokenSwap(conditionId, buyOrder, sellOrder, fillAmount, tokenId);
        } else {
            // JIT MINTING MODE: Neither has tokens, mint complete set with proportional contributions
            _executeJITMinting(conditionId, buyOrder, sellOrder, fillAmount, numberOfOutcomes);
        }
    }

    /**
    * @notice Executes token swap when seller has existing tokens
    * @param conditionId Condition identifier
    * @param buyOrder Buyer's order
    * @param sellOrder Seller's order
    * @param fillAmount Amount being traded
    * @param tokenId Token ID being traded
    */
    function _executeTokenSwap(
        bytes32 conditionId,
        IMarketController.Order calldata buyOrder,
        IMarketController.Order calldata sellOrder,
        uint256 fillAmount,
        uint256 tokenId
    ) internal {
        // Calculate payment based on execution price (seller's price in a match)
        uint256 paymentAmount = (fillAmount * sellOrder.price) / 10000;

        // Calculate trade fee from buyer
        uint256 buyerFeeRate = _getEffectiveTradeFeeRate(buyOrder.user);
        uint256 tradeFee = (paymentAmount * buyerFeeRate) / 10000;
        uint256 netPayment = paymentAmount - tradeFee;

        // Burn tokens from seller and mint to buyer
        positionTokens.burn(sellOrder.user, tokenId, fillAmount);
        uint256[] memory tokenIds = new uint256[](1);
        uint256[] memory amounts = new uint256[](1);
        tokenIds[0] = tokenId;
        amounts[0] = fillAmount;
        positionTokens.mintBatch(buyOrder.user, tokenIds, amounts);

        // Transfer fee to treasury if applicable
        if (tradeFee > 0 && treasury != address(0)) {
            vault.transferBetweenUsers(conditionId, buyOrder.user, treasury, tradeFee);
            emit TradeFeeCollected(buyOrder.user, buyOrder.questionId, tradeFee, netPayment);
        }
        else {
            netPayment = paymentAmount;
        }

        // Transfer net payment to seller
        vault.transferBetweenUsers(conditionId, buyOrder.user, sellOrder.user, netPayment);
    }

    /**
    * @notice Executes JIT minting when neither party has tokens
    * @param conditionId Condition identifier
    * @param buyOrder Buyer's order
    * @param sellOrder Seller's order
    * @param fillAmount Amount being traded
    * @param numberOfOutcomes Total outcomes in market
    */
    function _executeJITMinting(
        bytes32 conditionId,
        IMarketController.Order calldata buyOrder,
        IMarketController.Order calldata sellOrder,
        uint256 fillAmount,
        uint256 numberOfOutcomes
    ) internal {
        // Calculate proportional contributions based on execution price
        // Buyer pays: fillAmount * price (e.g., $0.60 per token)
        // Seller pays: fillAmount * (1 - price) (e.g., $0.40 per token)
        uint256 buyerPayment = (fillAmount * sellOrder.price) / 10000;
        uint256 sellerPayment = fillAmount - buyerPayment;

        // Calculate trade fees for both parties
        uint256 buyerFeeRate = _getEffectiveTradeFeeRate(buyOrder.user);
        uint256 sellerFeeRate = _getEffectiveTradeFeeRate(sellOrder.user);
        uint256 buyerFee = (buyerPayment * buyerFeeRate) / 10000;
        uint256 sellerFee = (sellerPayment * sellerFeeRate) / 10000;

        // Both parties lock their proportional collateral
        vault.lockCollateral(conditionId, buyOrder.user, buyerPayment);
        vault.lockCollateral(conditionId, sellOrder.user, sellerPayment);

        // Transfer fees to treasury (from available balance, after locking position collateral)
        if (treasury != address(0)) {
            if (buyerFee > 0) {
                vault.transferBetweenUsers(conditionId, buyOrder.user, treasury, buyerFee);
                emit TradeFeeCollected(buyOrder.user, buyOrder.questionId, buyerFee, buyerPayment);
            }
            if (sellerFee > 0) {
                vault.transferBetweenUsers(conditionId, sellOrder.user, treasury, sellerFee);
                emit TradeFeeCollected(sellOrder.user, sellOrder.questionId, sellerFee, sellerPayment);
            }
        }

        // Mint complete sets - buyer gets their outcome, seller gets the rest
        uint256[] memory buyerTokenIds = new uint256[](1);
        uint256[] memory buyerAmounts = new uint256[](1);
        uint256[] memory sellerTokenIds = new uint256[](numberOfOutcomes - 1);
        uint256[] memory sellerAmounts = new uint256[](numberOfOutcomes - 1);

        uint256 sellerIndex = 0;
        for (uint256 i = 0; i < numberOfOutcomes; i++) {
            uint256 outcomeIndex = 1 << i;
            uint256 tokenId = positionTokens.getTokenId(conditionId, outcomeIndex);

            if (outcomeIndex == buyOrder.outcome) {
                // This is the outcome the buyer wants
                buyerTokenIds[0] = tokenId;
                buyerAmounts[0] = fillAmount;
            } else {
                // These go to the seller
                sellerTokenIds[sellerIndex] = tokenId;
                sellerAmounts[sellerIndex] = fillAmount;
                sellerIndex++;
            }
        }

        // Mint tokens directly to recipients
        positionTokens.mintBatch(buyOrder.user, buyerTokenIds, buyerAmounts);
        if (sellerTokenIds.length > 0) {
            positionTokens.mintBatch(sellOrder.user, sellerTokenIds, sellerAmounts);
        }
    }

    /**
    * @notice Executes order against matcher's liquidity (inventory-based only)
    * @dev Matchers are expected to pre-mint and hold inventory. Does not support JIT minting.
    */
    function _executeAgainstMatcher(IMarketController.Order calldata order, uint256 fillAmount, address matcher) internal {
        uint256 numberOfOutcomes = market.getOutcomeCount(order.questionId);
        bytes32 conditionId = market.getConditionId(oracle, order.questionId, numberOfOutcomes, 0);
        uint256 tokenId = positionTokens.getTokenId(conditionId, order.outcome);

        uint256 paymentAmount = (fillAmount * order.price) / 10000;

        if (order.isBuyOrder) {
            // User wants to buy - matcher must have tokens in inventory
            require(positionTokens.balanceOf(matcher, tokenId) >= fillAmount, "Matcher insufficient inventory");

            // Calculate trade fee from user (buyer)
            uint256 buyerFeeRate = _getEffectiveTradeFeeRate(order.user);
            uint256 tradeFee = (paymentAmount * buyerFeeRate) / 10000;
            uint256 netPayment = paymentAmount - tradeFee;

            // Burn tokens from matcher and mint to user
            positionTokens.burn(matcher, tokenId, fillAmount);
            uint256[] memory tokenIds = new uint256[](1);
            uint256[] memory amounts = new uint256[](1);
            tokenIds[0] = tokenId;
            amounts[0] = fillAmount;
            positionTokens.mintBatch(order.user, tokenIds, amounts);

            // Transfer fee to treasury if applicable
            if (tradeFee > 0 && treasury != address(0)) {
                vault.transferBetweenUsers(conditionId, order.user, treasury, tradeFee);
                emit TradeFeeCollected(order.user, order.questionId, tradeFee, netPayment);
            }
            else {
                netPayment = paymentAmount;
            }

            // Transfer net payment from user to matcher
            vault.transferBetweenUsers(conditionId, order.user, matcher, netPayment);
        } else {
            // User wants to sell - they must have the tokens
            require(positionTokens.balanceOf(order.user, tokenId) >= fillAmount, "Insufficient tokens");

            // Calculate trade fee from matcher (buyer in this case)
            uint256 matcherFeeRate = _getEffectiveTradeFeeRate(matcher);
            uint256 tradeFee = (paymentAmount * matcherFeeRate) / 10000;
            uint256 netPayment = paymentAmount - tradeFee;

            // Burn tokens from user and mint to matcher
            positionTokens.burn(order.user, tokenId, fillAmount);
            uint256[] memory tokenIds = new uint256[](1);
            uint256[] memory amounts = new uint256[](1);
            tokenIds[0] = tokenId;
            amounts[0] = fillAmount;
            positionTokens.mintBatch(matcher, tokenIds, amounts);

            // Transfer fee to treasury if applicable
            if (tradeFee > 0 && treasury != address(0)) {
                vault.transferBetweenUsers(conditionId, matcher, treasury, tradeFee);
                emit TradeFeeCollected(matcher, order.questionId, tradeFee, netPayment);
            }
            else {
                netPayment = paymentAmount;
            }

            // Transfer net payment from matcher to user
            vault.transferBetweenUsers(conditionId, matcher, order.user, netPayment);
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {IPositionTokens} from "./IPositionTokens.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ERC1155Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC1155/ERC1155Upgradeable.sol";

/**
 * @title PositionTokens
 * @notice Upgradeable ERC1155 conditional tokens for prediction market positions
 * @dev Pure token operations without business logic with UUPS upgradeability
 */
contract PositionTokens is Initializable, UUPSUpgradeable, ERC1155Upgradeable, OwnableUpgradeable, IPositionTokens {
    /// @notice Address authorized to mint/burn tokens
    address public marketController;

    /// @dev Gap for future storage variables
    uint256[49] private __gap;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the upgradeable contract
     * @param _initialOwner Initial owner of the contract
     */
    function initialize(address _initialOwner) public initializer {
        __ERC1155_init("");
        __Ownable_init(_initialOwner);
        __UUPSUpgradeable_init();
    }

    /// @dev Restricts function access to authorized market controller
    modifier onlyMarketController() {
        require(msg.sender == marketController, "Only MarketController can call this function");
        _;
    }

    /**
     * @notice Authorizes contract upgrades
     * @param newImplementation Address of the new implementation
     * @dev Only callable by contract owner
     */
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    // ============ Contract Management Functions ============

    /**
     * @notice Updates the market controller address
     * @param _marketController New market controller address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateMarketController(address _marketController) external onlyOwner {
        require(_marketController != address(0), "Invalid MarketController address");
        marketController = _marketController;
    }

    /**
     * @notice Sets the authorized MarketController address
     * @param _marketController Address of MarketController contract
     * @dev Only callable by contract owner
     */
    function setMarketController(address _marketController) external onlyOwner {
        require(_marketController != address(0), "Invalid MarketController address");
        marketController = _marketController;
    }

    /**
     * @notice Mints position tokens for specific condition and outcomes
     * @param to Address to mint tokens to
     * @param ids Array of token IDs to mint
     * @param amounts Array of amounts to mint for each ID
     */
    function mintBatch(address to, uint256[] calldata ids, uint256[] calldata amounts) external onlyMarketController {
        _mintBatch(to, ids, amounts, "");
    }

    /**
     * @notice Burns position tokens from holder
     * @param from Address to burn tokens from
     * @param id Token ID to burn
     * @param amount Amount to burn
     */
    function burn(address from, uint256 id, uint256 amount) external onlyMarketController {
        _burn(from, id, amount);
    }

    /**
     * @notice Burns multiple position token types from holder in single transaction
     * @param from Address to burn tokens from
     * @param ids Array of token IDs to burn
     * @param amounts Array of amounts to burn for each ID
     * @dev Only callable by authorized MarketController contract, more gas efficient for multiple burns
     */
    function burnBatch(address from, uint256[] calldata ids, uint256[] calldata amounts)
        external
        onlyMarketController
    {
        _burnBatch(from, ids, amounts);
    }

    /**
     * @notice Generates unique token ID for condition and outcome
     * @param conditionId Condition identifier
     * @param selectedOutcome Outcome index
     * @return Unique token ID
     */
    function getTokenId(bytes32 conditionId, uint256 selectedOutcome) external pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(conditionId, selectedOutcome)));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "./IMarketResolver.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import {MerkleProof} from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

/**
 * @title MarketResolver
 * @notice Upgradeable version handling market resolution and merkle proof verification for prediction markets
 * @dev Manages resolution data and validates outcome proofs independently of token mechanics with UUPS upgradeability
 */
contract MarketResolver is
    Initializable,
    UUPSUpgradeable,
    OwnableUpgradeable,
    ReentrancyGuardUpgradeable,
    IMarketResolver
{
    /// @notice Maps condition ID to merkle root for outcome verification
    mapping(bytes32 => bytes32) public resolutionMerkleRoots;

    /// @notice Tracks which conditions have been resolved to prevent re-resolution
    mapping(bytes32 => bool) public isResolved;

    /// @notice Address authorized to resolve markets
    address public oracle;

    /// @notice Address authorized for emergency resolution
    address public emergencyResolver;

    /// @dev Gap for future storage variables
    uint256[46] private __gap;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the upgradeable contract
     * @param _initialOwner Initial owner of the contract
     * @param _oracle Initial oracle address
     */
    function initialize(address _initialOwner, address _oracle) public initializer {
        __Ownable_init(_initialOwner);
        __UUPSUpgradeable_init();
        __ReentrancyGuard_init();

        oracle = _oracle;
    }

    /// @dev Restricts function access to contract owner (oracle)
    modifier onlyOracle() {
        require(msg.sender == oracle, "Caller is not the oracle");
        _;
    }

    /// @dev Restricts emergency resolution to authorized emergency resolver
    modifier onlyEmergencyResolver() {
        require(msg.sender == emergencyResolver, "Caller is not emergency resolver");
        _;
    }

    /// @dev Allows both oracle and emergency resolver to resolve markets
    modifier onlyAuthorizedResolver() {
        require(msg.sender == oracle || msg.sender == emergencyResolver, "Not authorized to resolve");
        _;
    }

    /**
     * @notice Authorizes contract upgrades
     * @param newImplementation Address of the new implementation
     * @dev Only callable by contract owner
     */
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    // ============ Contract Management Functions ============

    /**
     * @notice Updates the oracle address
     * @param _oracle New oracle address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateOracle(address _oracle) external onlyOwner {
        require(_oracle != address(0), "Invalid oracle address");
        oracle = _oracle;
    }

    /**
     * @notice Updates the emergency resolver address
     * @param _emergencyResolver New emergency resolver address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateEmergencyResolver(address _emergencyResolver) external onlyOwner {
        require(_emergencyResolver != address(0), "Invalid emergency resolver address");
        emergencyResolver = _emergencyResolver;
    }

    /**
     * @notice Sets the authorized oracle address
     * @param _oracle Address of oracle for market resolution
     * @dev Only callable by contract owner
     */
    function setOracle(address _oracle) external onlyOwner {
        require(_oracle != address(0), "Invalid oracle address");
        oracle = _oracle;
    }

    /**
     * @notice Sets the authorized emergency resolver address
     * @param _emergencyResolver Address of emergency resolver (usually MarketController)
     * @dev Only callable by contract owner
     */
    function setEmergencyResolver(address _emergencyResolver) external onlyOwner {
        require(_emergencyResolver != address(0), "Invalid emergency resolver address");
        emergencyResolver = _emergencyResolver;
    }

    /**
     * @notice Resolves specific market epoch by setting merkle root
     * @param questionId Market question identifier
     * @param epoch Specific epoch to resolve
     * @param numberOfOutcomes Number of possible outcomes for validation
     * @param merkleRoot Root hash of merkle tree containing valid outcomes
     * @dev Oracle can resolve any epoch, enables flexible resolution timing
     */
    function resolveMarketEpoch(bytes32 questionId, uint256 epoch, uint256 numberOfOutcomes, bytes32 merkleRoot)
        public
        onlyAuthorizedResolver
        nonReentrant
    {
        _resolveMarketEpochInternal(questionId, epoch, numberOfOutcomes, merkleRoot);
    }

    /**
     * @notice Verifies if outcome won
     * @param conditionId Condition identifier
     * @param selectedOutcome Outcome being verified (1 or 2 for binary)
     * @param merkleProof Proof data for verification
     * @return True if outcome won
     */
    function verifyProof(bytes32 conditionId, uint256 selectedOutcome, bytes32[] calldata merkleProof)
        external
        view
        returns (bool)
    {
        bytes32 merkleRoot = resolutionMerkleRoots[conditionId];
        require(merkleRoot != bytes32(0), "Condition not resolved");

        bytes32 leaf = keccak256(abi.encodePacked(selectedOutcome));
        return MerkleProof.verify(merkleProof, merkleRoot, leaf);
    }

    /**
     * @notice Gets merkle root for resolved condition
     * @param conditionId Condition identifier
     * @return Merkle root hash, zero if not resolved
     */
    function getResolutionRoot(bytes32 conditionId) external view returns (bytes32) {
        return resolutionMerkleRoots[conditionId];
    }

    /**
     * @notice Checks if condition has been resolved
     * @param conditionId Condition identifier
     * @return True if condition is resolved
     */
    function getResolutionStatus(bytes32 conditionId) external view returns (bool) {
        return isResolved[conditionId];
    }

    /**
     * @notice Generates condition ID for market resolution
     * @param oracleAddr Oracle address resolving the condition
     * @param questionId Market question identifier
     * @param numberOfOutcomes Number of possible outcomes
     * @param epoch Specific epoch for the condition
     * @return Unique condition identifier
     * @dev Matches condition ID generation used in other contracts
     */
    function getConditionId(address oracleAddr, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch)
        public
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(oracleAddr, questionId, numberOfOutcomes, epoch));
    }

    /**
     * @notice Batch resolves multiple market epochs for gas efficiency
     * @param questionIds Array of market question identifiers
     * @param epochs Array of epochs to resolve
     * @param numberOfOutcomes Array of outcome counts
     * @param merkleRoots Array of merkle roots
     * @dev Arrays must have equal length, enables efficient bulk resolution
     */
    function batchResolveMarkets(
        bytes32[] calldata questionIds,
        uint256[] calldata epochs,
        uint256[] calldata numberOfOutcomes,
        bytes32[] calldata merkleRoots
    ) external onlyAuthorizedResolver {
        require(
            questionIds.length == epochs.length && epochs.length == numberOfOutcomes.length
                && numberOfOutcomes.length == merkleRoots.length,
            "Array length mismatch"
        );

        for (uint256 i = 0; i < questionIds.length; i++) {
            _resolveMarketEpochInternal(questionIds[i], epochs[i], numberOfOutcomes[i], merkleRoots[i]);
        }
    }

    /**
     * @notice Internal function for resolving market epoch without reentrancy guard
     * @param questionId Market question identifier
     * @param epoch Specific epoch to resolve
     * @param numberOfOutcomes Number of possible outcomes for validation
     * @param merkleRoot Root hash of merkle tree containing valid outcomes
     */
    function _resolveMarketEpochInternal(
        bytes32 questionId,
        uint256 epoch,
        uint256 numberOfOutcomes,
        bytes32 merkleRoot
    ) internal {
        require(epoch > 0, "Invalid epoch");
        require(numberOfOutcomes > 0, "Invalid outcome count");
        require(merkleRoot != bytes32(0), "Invalid merkle root");

        bytes32 conditionId = getConditionId(oracle, questionId, numberOfOutcomes, epoch);

        require(!isResolved[conditionId], "Already resolved");

        resolutionMerkleRoots[conditionId] = merkleRoot;
        isResolved[conditionId] = true;

        emit ConditionResolved(conditionId, questionId, epoch, merkleRoot);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

/**
 * @title IVault
 * @notice Interface for collateral management in prediction markets
 */
interface IVault {
    /// @notice Emitted when user deposits collateral to their account
    event CollateralDeposited(address indexed user, uint256 amount);

    /// @notice Emitted when user withdraws available collateral
    event CollateralWithdrawn(address indexed user, uint256 amount);

    /// @notice Emitted when collateral is locked for a specific condition
    event CollateralLocked(bytes32 indexed conditionId, address indexed user, uint256 amount);

    /// @notice Emitted when collateral is unlocked from a resolved condition
    event CollateralUnlocked(bytes32 indexed conditionId, address indexed user, uint256 amount);

    /// @notice Emitted when multiple collateral locks are processed in batch
    event BatchCollateralLocked(bytes32[] conditionIds, address[] users, uint256[] amounts);

    /// @notice Emitted when multiple collateral unlocks are processed in batch
    event BatchCollateralUnlocked(bytes32[] conditionIds, address[] users, uint256[] amounts);
    
    /// @notice Emitted when collateral is transferred between users during token swaps
    event CollateralTransferred(bytes32 indexed conditionId, address indexed from, address indexed to, uint256 amount);

    /**
     * @notice Deposits ERC20 collateral tokens into user's available balance
     * @param amount Number of collateral tokens to deposit
     * @dev Requires prior ERC20 approval for vault contract
     */
    function depositCollateral(uint256 amount) external;

    /**
     * @notice Withdraws available collateral tokens to user's wallet
     * @param amount Number of tokens to withdraw
     * @dev Only withdraws from unlocked balance, reverts on insufficient funds
     */
    function withdrawCollateral(uint256 amount) external;

    /**
     * @notice Locks user's available collateral for position token minting
     * @param conditionId Market condition identifier
     * @param user Address whose collateral to lock
     * @param amount Collateral amount to lock
     * @dev Called by MarketController contract during position creation
     */
    function lockCollateral(bytes32 conditionId, address user, uint256 amount) external;

    /**
     * @notice Unlocks collateral from resolved condition to user's available balance
     * @param conditionId Resolved condition identifier
     * @param user User redeeming position tokens
     * @param amount Payout amount determined by market resolution
     * @dev Called by MarketController contract after successful claim verification
     */
    function unlockCollateral(bytes32 conditionId, address user, uint256 amount) external;

    /**
     * @notice Transfers collateral between users' vault balances (for token swaps)
     * @param conditionId Market condition identifier for tracking
     * @param from User sending collateral
     * @param to User receiving collateral
     * @param amount Amount to transfer
     * @dev Called by MarketController during token swap settlements
     */
    function transferBetweenUsers(bytes32 conditionId, address from, address to, uint256 amount) external;

    /**
     * @notice Locks collateral for multiple conditions in a single transaction
     * @param conditionIds Array of market condition identifiers
     * @param users Array of addresses whose collateral to lock
     * @param amounts Array of collateral amounts to lock
     * @dev Arrays must have equal length, called by MarketController for batch operations
     */
    function batchLockCollateral(bytes32[] calldata conditionIds, address[] calldata users, uint256[] calldata amounts)
        external;

    /**
     * @notice Unlocks collateral from multiple resolved conditions in a single transaction
     * @param conditionIds Array of resolved condition identifiers
     * @param users Array of users redeeming position tokens
     * @param amounts Array of payout amounts determined by market resolution
     * @dev Arrays must have equal length, called by MarketController for batch operations
     */
    function batchUnlockCollateral(
        bytes32[] calldata conditionIds,
        address[] calldata users,
        uint256[] calldata amounts
    ) external;

    /**
     * @notice Returns user's available collateral balance
     * @param user Address to query
     * @return Available balance for withdrawal or position creation
     */
    function getAvailableBalance(address user) external view returns (uint256);

    /**
     * @notice Returns total locked collateral for specific condition
     * @param conditionId Condition identifier
     * @return Total locked amount across all participants
     */
    function getTotalLocked(bytes32 conditionId) external view returns (uint256);

    /**
     * @notice Updates authorized MarketController contract address
     * @param _contract New authorized contract address
     * @dev Only callable by contract owner
     */
    function setMarketController(address _contract) external;

    /**
     * @notice Sets contract pause state for emergency situations
     * @param _paused Pause state boolean
     * @dev Only callable by contract owner, affects user-facing operations
     */
    function setPaused(bool _paused) external;

    /**
     * @notice Returns the authorized MarketController address
     * @return Address of the MarketController contract
     */
    function marketController() external view returns (address);

    /**
     * @notice Returns whether the contract is currently paused
     * @return True if contract operations are paused
     */
    function paused() external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {IERC1155} from "@openzeppelin/contracts/token/ERC1155/IERC1155.sol";

/**
 * @title IPositionTokens
 * @notice ERC1155 conditional tokens for prediction market positions
 * @dev Extends ERC1155 with prediction market specific functionality
 */
interface IPositionTokens is IERC1155 {
    /// @notice Emitted when position tokens are minted for market positions
    event PositionTokensMinted(address indexed to, uint256[] ids, uint256[] amounts, bytes32 indexed conditionId);

    /// @notice Emitted when position tokens are burned during settlement
    event PositionTokensBurned(address indexed from, uint256[] ids, uint256[] amounts, bytes32 indexed conditionId);

    /**
     * @notice Mints position tokens for specific condition and outcomes
     * @param to Address to mint tokens to
     * @param ids Array of token IDs to mint
     * @param amounts Array of amounts to mint for each ID
     * @dev Only callable by authorized MarketController contract
     */
    function mintBatch(address to, uint256[] calldata ids, uint256[] calldata amounts) external;

    /**
     * @notice Burns position tokens from holder
     * @param from Address to burn tokens from
     * @param id Token ID to burn
     * @param amount Amount to burn
     * @dev Only callable by authorized MarketController contract
     */
    function burn(address from, uint256 id, uint256 amount) external;

    /**
     * @notice Burns multiple position token types from holder in single transaction
     * @param from Address to burn tokens from
     * @param ids Array of token IDs to burn
     * @param amounts Array of amounts to burn for each ID
     * @dev Only callable by authorized MarketController contract, more gas efficient for multiple burns
     */
    function burnBatch(address from, uint256[] calldata ids, uint256[] calldata amounts) external;

    /**
     * @notice Generates unique token ID for condition and outcome combination
     * @param conditionId Condition identifier from market resolution system
     * @param selectedOutcome Outcome index representing specific market result
     * @return Unique token ID for the condition-outcome pair
     * @dev Token ID is deterministic hash of condition and outcome
     */
    function getTokenId(bytes32 conditionId, uint256 selectedOutcome) external pure returns (uint256);

    /**
     * @notice Sets the authorized MarketController contract address
     * @param marketController Address of MarketController contract
     * @dev Only callable by contract owner, establishes minting/burning permissions
     */
    function setMarketController(address marketController) external;

    /**
     * @notice Returns the authorized MarketController address
     * @return Address of the MarketController contract
     */
    function marketController() external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

/**
 * @title IMarketResolver
 * @notice Interface for market resolution and merkle proof verification
 */
interface IMarketResolver {
    /// @notice Emitted when market condition is resolved with merkle root
    event ConditionResolved(
        bytes32 indexed conditionId,
        bytes32 indexed questionId,
        uint256 indexed epoch,
        bytes32 merkleRoot
    );

    /**
     * @notice Resolves specific market epoch by setting merkle root
     * @param questionId Market question identifier
     * @param epoch Specific epoch to resolve
     * @param numberOfOutcomes Number of possible outcomes for validation
     * @param merkleRoot Root hash of merkle tree containing valid outcomes
     */
    function resolveMarketEpoch(bytes32 questionId, uint256 epoch, uint256 numberOfOutcomes, bytes32 merkleRoot)
        external;

    /**
     * @notice Verifies if outcome won
     * @param conditionId Condition identifier
     * @param selectedOutcome Outcome being verified (1 or 2 for binary)
     * @param merkleProof Proof data for verification
     * @return True if outcome won
     */
    function verifyProof(bytes32 conditionId, uint256 selectedOutcome, bytes32[] calldata merkleProof)
        external
        view
        returns (bool);

    /**
     * @notice Gets merkle root for resolved condition
     * @param conditionId Condition identifier
     * @return Merkle root hash, zero if not resolved
     */
    function getResolutionRoot(bytes32 conditionId) external view returns (bytes32);

    /**
     * @notice Checks if condition has been resolved
     * @param conditionId Condition identifier
     * @return True if condition is resolved
     */
    function getResolutionStatus(bytes32 conditionId) external view returns (bool);

    /**
     * @notice Batch resolves multiple market epochs for gas efficiency
     * @param questionIds Array of market question identifiers
     * @param epochs Array of epochs to resolve
     * @param numberOfOutcomes Array of outcome counts
     * @param merkleRoots Array of merkle roots
     */
    function batchResolveMarkets(
        bytes32[] calldata questionIds,
        uint256[] calldata epochs,
        uint256[] calldata numberOfOutcomes,
        bytes32[] calldata merkleRoots
    ) external;

    /**
     * @notice Sets the authorized oracle address
     * @param oracle Address of oracle for market resolution
     */
    function setOracle(address oracle) external;

    /**
     * @notice Sets the authorized emergency resolver address
     * @param emergencyResolver Address of emergency resolver (usually MarketController)
     */
    function setEmergencyResolver(address emergencyResolver) external;

    /**
     * @notice Returns the current oracle address
     * @return Address of the oracle
     */
    function oracle() external view returns (address);

    /**
     * @notice Returns the current emergency resolver address
     * @return Address of the emergency resolver
     */
    function emergencyResolver() external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

/**
 * @title IMarketContract
 * @notice Interface for market metadata and configuration management
 */
interface IMarket {
    /// @notice Emitted when new market is created
    event MarketCreated(bytes32 indexed questionId, uint256 outcomeCount, uint256 initialEpoch, uint256 resolutionTime);

    /// @notice Emitted when market epoch is advanced
    event EpochAdvanced(bytes32 indexed questionId, uint256 previousEpoch, uint256 newEpoch);

    /// @notice Emitted when market resolution time is updated
    event ResolutionTimeUpdated(bytes32 indexed questionId, uint256 oldResolutionTime, uint256 newResolutionTime);

    /**
     * @notice Creates new market with specified outcomes and resolution time
     * @param questionId Unique market question identifier
     * @param outcomeCount Number of possible outcomes for this market
     * @param resolutionTime Timestamp when market should resolve (0 for manual resolution)
     * @param epochDuration Duration of each epoch in seconds (0 for manual epoch advancement)
     */
    function createMarket(bytes32 questionId, uint256 outcomeCount, uint256 resolutionTime, uint256 epochDuration)
        external;

    /**
     * @notice Updates resolution time for existing market
     * @param questionId Market question identifier
     * @param resolutionTime New resolution timestamp (0 for manual resolution)
     */
    function updateResolutionTime(bytes32 questionId, uint256 resolutionTime) external;

    /**
     * @notice Advances market to next epoch (manual mode only)
     * @param questionId Market question identifier
     */
    function advanceEpoch(bytes32 questionId) external;

    /**
     * @notice Checks if market is currently open for betting
     * @param questionId Market question identifier
     * @return True if market is open for betting
     */
    function isMarketOpen(bytes32 questionId) external view returns (bool);

    /**
     * @notice Checks if market is ready for resolution
     * @param questionId Market question identifier
     * @return True if market can be resolved
     */
    function isMarketReadyForResolution(bytes32 questionId) external view returns (bool);

    /**
     * @notice Gets resolution timestamp for market
     * @param questionId Market question identifier
     * @return Resolution timestamp (0 if manual resolution)
     */
    function getResolutionTime(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Gets creation timestamp for market
     * @param questionId Market question identifier
     * @return Creation timestamp
     */
    function getCreationTime(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Generates condition ID for market and epoch
     * @param oracle Oracle address
     * @param questionId Market question identifier
     * @param numberOfOutcomes Number of outcomes
     * @param epoch Specific epoch (0 for current)
     * @return Condition identifier
     */
    function getConditionId(address oracle, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch)
        external
        view
        returns (bytes32);

    /**
     * @notice Gets number of possible outcomes for market
     * @param questionId Market question identifier
     * @return Number of outcomes
     */
    function getOutcomeCount(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Gets current epoch for market
     * @param questionId Market question identifier
     * @return Current epoch number
     */
    function getCurrentEpoch(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Gets epoch duration for market
     * @param questionId Market question identifier
     * @return Epoch duration in seconds (0 if manual epochs)
     */
    function getEpochDuration(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Gets the timestamp when a specific epoch starts
     * @param questionId Market question identifier
     * @param epoch The epoch number to query
     * @return Timestamp when the epoch starts (0 if manual epochs)
     */
    function getEpochStartTime(bytes32 questionId, uint256 epoch) external view returns (uint256);

    /**
     * @notice Gets the timestamp when a specific epoch ends
     * @param questionId Market question identifier
     * @param epoch The epoch number to query
     * @return Timestamp when the epoch ends (0 if manual epochs)
     */
    function getEpochEndTime(bytes32 questionId, uint256 epoch) external view returns (uint256);

    /**
     * @notice Checks if market exists
     * @param questionId Market question identifier
     * @return True if market exists
     */
    function getMarketExists(bytes32 questionId) external view returns (bool);

    /**
     * @notice Sets the authorized MarketController address
     * @param marketController Address of MarketController contract
     */
    function setMarketController(address marketController) external;
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "./IVault.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @title Vault
 * @notice Upgradeable version managing collateral deposits, withdrawals, and position funding for conditional token markets
 * @dev Handles ERC20 collateral for prediction market positions across multiple conditions with UUPS upgradeability
 */
contract Vault is Initializable, UUPSUpgradeable, IVault, OwnableUpgradeable, ReentrancyGuardUpgradeable {
    using SafeERC20 for IERC20;

    /// @notice The ERC20 token used as collateral for all positions
    IERC20 public collateralToken;

    /// @notice Address of the MarketController contract authorized to lock/unlock funds
    address public marketController;

    /// @notice Available collateral balance per user address
    mapping(address => uint256) private userBalances;

    /// @notice Total collateral locked per condition ID (market + epoch combination)
    mapping(bytes32 => uint256) private totalLockedPerCondition;

    /// @notice Emergency pause state for contract operations
    bool public paused;

    /// @dev Gap for future storage variables - increased by 1 to account for removed mapping
    uint256[44] private __gap;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the upgradeable contract
     * @param _initialOwner Initial owner of the contract
     * @param _collateralToken ERC20 token address for collateral
     * @param _marketController Address authorized to lock/unlock collateral
     */
    function initialize(address _initialOwner, address _collateralToken, address _marketController)
        public
        initializer
    {
        require(_collateralToken != address(0), "Invalid collateral token");
        require(_marketController != address(0), "Invalid market controller");

        __Ownable_init(_initialOwner);
        __UUPSUpgradeable_init();
        __ReentrancyGuard_init();

        collateralToken = IERC20(_collateralToken);
        marketController = _marketController;
    }

    /// @dev Restricts function access to the MarketController contract only
    modifier onlyMarketController() {
        require(msg.sender == marketController, "Unauthorized caller");
        _;
    }

    /// @dev Prevents function execution when contract is paused
    modifier whenNotPaused() {
        require(!paused, "Contract paused");
        _;
    }

    /**
     * @notice Authorizes contract upgrades
     * @param newImplementation Address of the new implementation
     * @dev Only callable by contract owner
     */
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    // ============ Contract Management Functions ============

    /**
     * @notice Updates the collateral token address
     * @param _collateralToken New collateral token address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateCollateralToken(address _collateralToken) external onlyOwner {
        require(_collateralToken != address(0), "Invalid collateral token");
        collateralToken = IERC20(_collateralToken);
    }

    /**
     * @notice Updates the market controller address
     * @param _marketController New market controller address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateMarketController(address _marketController) external onlyOwner {
        require(_marketController != address(0), "Invalid market controller");
        marketController = _marketController;
    }

    /**
     * @notice Deposits ERC20 collateral tokens into user's available balance
     * @param amount Number of collateral tokens to deposit
     * @dev Requires prior ERC20 approval for vault contract
     */
    function depositCollateral(uint256 amount) external nonReentrant whenNotPaused {
        require(amount > 0, "Invalid amount");

        collateralToken.safeTransferFrom(_msgSender(), address(this), amount);
        userBalances[_msgSender()] += amount;

        emit CollateralDeposited(_msgSender(), amount);
    }

    /**
     * @notice Withdraws available collateral tokens to user's wallet
     * @param amount Number of tokens to withdraw
     * @dev Only withdraws from unlocked balance, reverts on insufficient funds
     */
    function withdrawCollateral(uint256 amount) external nonReentrant whenNotPaused {
        require(amount > 0, "Invalid amount");
        require(userBalances[_msgSender()] >= amount, "Insufficient balance");

        userBalances[_msgSender()] -= amount;
        collateralToken.safeTransfer(_msgSender(), amount);

        emit CollateralWithdrawn(_msgSender(), amount);
    }

    /**
     * @notice Locks user's available collateral for position token minting
     * @param conditionId Market condition identifier
     * @param user Address whose collateral to lock
     * @param amount Collateral amount to lock
     * @dev Called by MarketController contract during position creation
     */
    function lockCollateral(bytes32 conditionId, address user, uint256 amount)
        external
        onlyMarketController
        nonReentrant
    {
        require(userBalances[user] >= amount, "Insufficient balance");

        userBalances[user] -= amount;
        totalLockedPerCondition[conditionId] += amount;

        emit CollateralLocked(conditionId, user, amount);
    }

    /**
     * @notice Unlocks collateral from resolved condition to user's available balance
     * @param conditionId Resolved condition identifier
     * @param user User redeeming position tokens
     * @param amount Payout amount determined by market resolution
     * @dev Called by MarketController contract after successful claim verification
     * @dev Allows unlocking to any user for proper market payout distribution
     */
    function unlockCollateral(bytes32 conditionId, address user, uint256 amount)
        external
        onlyMarketController
        nonReentrant
    {
        require(totalLockedPerCondition[conditionId] >= amount, "Invalid unlock amount");

        totalLockedPerCondition[conditionId] -= amount;
        userBalances[user] += amount;

        emit CollateralUnlocked(conditionId, user, amount);
    }

    /**
     * @notice Transfers collateral between users' vault balances (for token swaps)
     * @param conditionId Market condition identifier for tracking
     * @param from User sending collateral
     * @param to User receiving collateral
     * @param amount Amount to transfer
     * @dev Called by MarketController during token swap settlements
     */
    function transferBetweenUsers(bytes32 conditionId, address from, address to, uint256 amount)
        external
        onlyMarketController
        nonReentrant
    {
        require(userBalances[from] >= amount, "Insufficient balance");
        require(to != address(0), "Invalid recipient");

        userBalances[from] -= amount;
        userBalances[to] += amount;

        emit CollateralTransferred(conditionId, from, to, amount);
    }

    /**
     * @notice Locks collateral for multiple conditions in a single transaction
     * @param conditionIds Array of market condition identifiers
     * @param users Array of addresses whose collateral to lock
     * @param amounts Array of collateral amounts to lock
     * @dev Arrays must have equal length, called by MarketController for batch operations
     */
    function batchLockCollateral(bytes32[] calldata conditionIds, address[] calldata users, uint256[] calldata amounts)
        external
        onlyMarketController
        nonReentrant
    {
        require(conditionIds.length == users.length && users.length == amounts.length, "Array length mismatch");

        for (uint256 i = 0; i < conditionIds.length; i++) {
            require(userBalances[users[i]] >= amounts[i], "Insufficient balance");

            userBalances[users[i]] -= amounts[i];
            totalLockedPerCondition[conditionIds[i]] += amounts[i];

            emit CollateralLocked(conditionIds[i], users[i], amounts[i]);
        }
    }

    /**
     * @notice Unlocks collateral from multiple resolved conditions in a single transaction
     * @param conditionIds Array of resolved condition identifiers
     * @param users Array of users redeeming position tokens
     * @param amounts Array of payout amounts determined by market resolution
     * @dev Arrays must have equal length, called by MarketController for batch operations
     */
    function batchUnlockCollateral(
        bytes32[] calldata conditionIds,
        address[] calldata users,
        uint256[] calldata amounts
    ) external onlyMarketController nonReentrant {
        require(conditionIds.length == users.length && users.length == amounts.length, "Array length mismatch");

        for (uint256 i = 0; i < conditionIds.length; i++) {
            require(totalLockedPerCondition[conditionIds[i]] >= amounts[i], "Invalid unlock amount");

            totalLockedPerCondition[conditionIds[i]] -= amounts[i];
            userBalances[users[i]] += amounts[i];

            emit CollateralUnlocked(conditionIds[i], users[i], amounts[i]);
        }
    }

    /**
     * @notice Returns user's available collateral balance
     * @param user Address to query
     * @return Available balance for withdrawal or position creation
     */
    function getAvailableBalance(address user) external view returns (uint256) {
        return userBalances[user];
    }

    /**
     * @notice Returns total locked collateral for specific condition
     * @param conditionId Condition identifier
     * @return Total locked amount across all participants
     */
    function getTotalLocked(bytes32 conditionId) external view returns (uint256) {
        return totalLockedPerCondition[conditionId];
    }

    /**
     * @notice Updates authorized MarketController contract address
     * @param _contract New authorized contract address
     * @dev Only callable by contract owner
     */
    function setMarketController(address _contract) external onlyOwner {
        require(_contract != address(0), "Invalid address");
        marketController = _contract;
    }

    /**
     * @notice Sets contract pause state for emergency situations
     * @param _paused Pause state boolean
     * @dev Only callable by contract owner, affects user-facing operations
     */
    function setPaused(bool _paused) external onlyOwner {
        paused = _paused;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

/**
 * @title IMarketController
 * @notice Interface for orderbook-based market operations with EIP-712 order matching
 */
interface IMarketController {
    // Order structure for EIP-712
    struct Order {
        address user;           // User placing the order
        bytes32 questionId;     // Market question ID
        uint256 outcome;        // Specific outcome (1, 2, 4, 8, etc.)
        uint256 amount;         // Amount of tokens
        uint256 price;          // Price per token (in basis points, 10000 = 100%)
        uint256 nonce;          // Unique nonce for replay protection
        uint256 expiration;     // Order expiration timestamp
        bool isBuyOrder;        // true = buy, false = sell
    }

    // Batch claim structure
    struct ClaimRequest {
        bytes32 questionId;     // Market question ID
        uint256 epoch;          // Specific epoch for the claim
        uint256 outcome;        // Winning outcome being claimed
        bytes32[] merkleProof;  // Proof of winning outcome
    }

    /// @notice Emitted when user claims winnings from resolved market
    event WinningsClaimed(
        address indexed user, bytes32 indexed questionId, uint256 epoch, uint256 outcome, uint256 payout
    );

    /// @notice Emitted when user claims winnings from multiple markets in batch
    event BatchWinningsClaimed(
        address indexed user, uint256 totalPayout, uint256 claimsProcessed
    );

    /// @notice Emitted when emergency resolution is triggered
    event EmergencyResolution(bytes32 indexed questionId, uint256 indexed epoch, bytes32 merkleRoot);

    /// @notice Emitted when global trading pause state changes
    event GlobalTradingPauseChanged(bool paused);

    /// @notice Emitted when market-specific trading pause state changes
    event MarketTradingPauseChanged(bytes32 indexed questionId, bool paused);

    /// @notice Emitted when multiple markets' trading pause state changes
    event BatchMarketTradingPauseChanged(bytes32[] questionIds, bool paused);

    /// @notice Emitted when an order is filled
    event OrderFilled(
        bytes32 indexed orderHash,
        address indexed user,           // User who placed the order
        bytes32 indexed questionId,     // Market question ID
        uint256 outcome,                // Outcome being traded
        uint256 fillAmount,             // Amount filled
        uint256 price,                  // Execution price
        bool isBuyOrder,                // Order direction
        uint256 epoch,                  // Market epoch
        address taker                   // Address that matched/took the order
    );

    /// @notice Emitted when an order is cancelled
    event OrderCancelled(bytes32 indexed orderHash, address indexed user);

    /// @notice Emitted when fee is collected on winnings
    event FeeCollected(address indexed user, bytes32 indexed questionId, uint256 feeAmount, uint256 netPayout);

    /// @notice Emitted when default fee rate is updated
    event FeeRateUpdated(uint256 oldRate, uint256 newRate);

    /// @notice Emitted when treasury address is updated
    event TreasuryUpdated(address indexed oldTreasury, address indexed newTreasury);

    /// @notice Emitted when custom user fee rate is set
    event UserFeeRateSet(address indexed user, uint256 feeRate);

    /// @notice Emitted when trade fee is collected
    event TradeFeeCollected(
        address indexed user,
        bytes32 indexed questionId,
        uint256 feeAmount,
        uint256 netAmount
    );

    /// @notice Emitted when trade fee rate is updated
    event TradeFeeRateUpdated(uint256 oldRate, uint256 newRate);

    /// @notice Emitted when custom user trade fee rate is set
    event UserTradeFeeRateSet(address indexed user, uint256 feeRate);

    // ============ Trading Hours Control Functions ============

    /**
     * @notice Sets global trading pause state (emergency stop)
     * @param paused True to pause all trading, false to resume
     * @dev Only callable by contract owner, affects all markets
     */
    function setGlobalTradingPaused(bool paused) external;

    /**
     * @notice Sets trading pause state for specific market
     * @param questionId Market question identifier
     * @param paused True to pause trading, false to resume
     * @dev Only callable by contract owner, for trading hours control
     */
    function setMarketTradingPaused(bytes32 questionId, bool paused) external;

    /**
     * @notice Sets trading pause state for multiple markets (batch operation)
     * @param questionIds Array of market question identifiers
     * @param paused True to pause trading, false to resume
     * @dev Only callable by contract owner, gas-efficient for bulk updates
     */
    function batchSetMarketTradingPaused(bytes32[] calldata questionIds, bool paused) external;

    /**
     * @notice Checks if trading is active for a specific market
     * @param questionId Market question identifier
     * @return True if trading is active (not paused globally or for this market)
     */
    function isTradingActive(bytes32 questionId) external view returns (bool);

    /**
     * @notice Returns global trading pause state
     * @return True if all trading is paused
     */
    function globalTradingPaused() external view returns (bool);

    /**
     * @notice Returns market-specific trading pause state
     * @param questionId Market question identifier
     * @return True if trading is paused for this market
     */
    function marketTradingPaused(bytes32 questionId) external view returns (bool);

    // ============ Market Management Functions ============

    /**
     * @notice Creates a new prediction market with optional time-based resolution
     * @param questionId Unique market question identifier
     * @param outcomeCount Number of possible outcomes for this market
     * @param resolutionTime Timestamp when market should resolve (0 for manual resolution)
     * @param epochDuration Duration of each epoch in seconds (0 for manual epoch advancement)
     * @dev Only callable by contract owner
     * @dev For continuous markets, set resolutionTime=0 and epochDuration>0 (e.g., 86400 for daily)
     */
    function createMarket(bytes32 questionId, uint256 outcomeCount, uint256 resolutionTime, uint256 epochDuration) external;

    /**
     * @notice Updates resolution time for existing market
     * @param questionId Market question identifier
     * @param resolutionTime New resolution timestamp (0 for manual resolution)
     * @dev Only callable by contract owner, only before current resolution time
     */
    function updateMarketResolutionTime(bytes32 questionId, uint256 resolutionTime) external;

    /**
     * @notice Advances market to next epoch for continuous markets
     * @param questionId Market question identifier
     * @dev Only callable by contract owner
     * @dev Only works for manual epoch markets (epochDuration = 0)
     */
    function advanceMarketEpoch(bytes32 questionId) external;

    // ============ Order Matching Functions ============

    /**
     * @notice Executes a trade using EIP-712 signed orders
     * @param buyOrder The buy order details
     * @param sellOrder The sell order details
     * @param buySignature The buyer's signature
     * @param sellSignature The seller's signature
     * @param fillAmount Amount to fill (must not exceed either order)
     */
    function executeOrderMatch(
        Order calldata buyOrder,
        Order calldata sellOrder,
        bytes calldata buySignature,
        bytes calldata sellSignature,
        uint256 fillAmount
    ) external;

    /**
     * @notice Executes a single signed order against matcher's liquidity
     * @param order The order to execute
     * @param signature The user's signature
     * @param fillAmount Amount to fill
     * @param counterparty Address providing liquidity (authorized matcher)
     */
    function executeSingleOrder(
        Order calldata order,
        bytes calldata signature,
        uint256 fillAmount,
        address counterparty
    ) external;

    /**
     * @notice Allows users to cancel their own orders
     * @param order The order to cancel
     * @param signature The user's signature (for verification)
     */
    function cancelOrder(Order calldata order, bytes calldata signature) external;

    /**
     * @notice Emergency function to resolve markets that have passed their resolution time
     * @param questionId Market question identifier
     * @param numberOfOutcomes Number of possible outcomes for validation
     * @param merkleRoot Root hash of merkle tree containing valid outcomes
     * @dev Can only be called after resolution time has passed, provides fallback resolution
     */
    function emergencyResolveMarket(bytes32 questionId, uint256 numberOfOutcomes, bytes32 merkleRoot) external;

    /**
     * @notice Sets authorization status for off-chain matching systems
     * @param matcher Address of the matching system
     * @param authorized Whether the matcher is authorized
     * @dev Only callable by contract owner
     */
    function setAuthorizedMatcher(address matcher, bool authorized) external;

    /**
     * @notice Claims winnings from resolved market
     * @param questionId Market question identifier
     * @param epoch Specific epoch for the claim
     * @param outcome Winning outcome being claimed
     * @param merkleProof Proof of winning outcome
     * @return Payout amount transferred to user (after fees)
     */
    function claimWinnings(bytes32 questionId, uint256 epoch, uint256 outcome, bytes32[] calldata merkleProof)
        external
        returns (uint256);

    /**
     * @notice Claims winnings from multiple resolved markets in a single transaction
     * @param claims Array of claim requests for different markets/epochs
     * @return totalPayout Total payout amount transferred to user across all claims (after fees)
     * @dev Processes all valid claims and skips invalid ones, enabling users to claim all winnings efficiently
     */
    function batchClaimWinnings(ClaimRequest[] calldata claims)
        external
        returns (uint256 totalPayout);

    // ============ Fee Management Functions ============

    /**
     * @notice Sets the default fee rate for winnings claims
     * @param _feeRate Fee rate in basis points (e.g., 400 = 4%)
     * @dev Only callable by contract owner, maximum 10000 basis points (100%)
     */
    function setFeeRate(uint256 _feeRate) external;

    /**
     * @notice Sets the treasury address where fees are sent
     * @param _treasury Address of the treasury
     * @dev Only callable by contract owner
     */
    function setTreasury(address _treasury) external;

    /**
     * @notice Sets a custom fee rate for specific user (fee tier system)
     * @param user Address of the user
     * @param _feeRate Custom fee rate in basis points (0 = use default rate)
     * @dev Only callable by contract owner, allows preferential rates for MMs/whales
     */
    function setUserFeeRate(address user, uint256 _feeRate) external;

    /**
     * @notice Gets the effective fee rate for a user
     * @param user Address of the user
     * @return Fee rate in basis points that will be applied to this user
     */
    function getEffectiveFeeRate(address user) external view returns (uint256);

    /**
     * @notice Returns the default fee rate
     * @return Fee rate in basis points
     */
    function feeRate() external view returns (uint256);

    /**
     * @notice Returns the treasury address
     * @return Address of the treasury
     */
    function treasury() external view returns (address);

    /**
     * @notice Returns custom fee rate for a user
     * @param user Address of the user
     * @return Custom fee rate (0 if using default)
     */
    function userFeeRate(address user) external view returns (uint256);

    // ============ Order Query Functions ============

    /**
     * @notice Get the EIP-712 hash for an order
     */
    function getOrderHash(Order calldata order) external view returns (bytes32);

    /**
     * @notice Check how much of an order has been filled
     */
    function getOrderFillAmount(bytes32 orderHash) external view returns (uint256);

    /**
     * @notice Check remaining amount for an order
     */
    function getOrderRemainingAmount(Order calldata order) external view returns (uint256);

    /**
     * @notice Checks if address is authorized to execute matching operations
     * @param matcher Address to check authorization for
     * @return True if matcher is authorized
     */
    function authorizedMatchers(address matcher) external view returns (bool);
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

import "../src/Market/Market.sol";
import "../src/Market/MarketController.sol";
import "../src/Market/MarketResolver.sol";
import "../src/Token/PositionTokens.sol";
import "../src/Vault/Vault.sol";

/**
 * @title Deploy
 * @notice Deterministic deployment script using Solidity's built-in Create2
 * @dev Deploys all contracts with deterministic salts for consistent addresses across chains
 */
contract Deploy is Script {
    // Deployment configuration
    struct DeployConfig {
        address owner;
        address oracle;
        address collateralToken;
        bool deployMockToken;
        uint256 mockTokenSupply;
        bytes32 salt; // Global salt for deterministic deployment
    }

    // Deployed contract addresses
    struct DeployedContracts {
        address collateralToken;
        address marketImpl;
        address marketResolverImpl;
        address positionTokensImpl;
        address vaultImpl;
        address marketControllerImpl;
        address market;
        address marketResolver;
        address positionTokens;
        address vault;
        address marketController;
    }

    // Custom salts for each contract type
    bytes32 constant MARKET_IMPL_SALT = keccak256("PredictionMarket.MarketImpl.v1");
    bytes32 constant MARKET_RESOLVER_IMPL_SALT = keccak256("PredictionMarket.MarketResolverImpl.v1");
    bytes32 constant POSITION_TOKENS_IMPL_SALT = keccak256("PredictionMarket.PositionTokensImpl.v1");
    bytes32 constant VAULT_IMPL_SALT = keccak256("PredictionMarket.VaultImpl.v1");
    bytes32 constant MARKET_CONTROLLER_IMPL_SALT = keccak256("PredictionMarket.MarketControllerImpl.v1");
    bytes32 constant MARKET_PROXY_SALT = keccak256("PredictionMarket.Market.v1");
    bytes32 constant MARKET_RESOLVER_PROXY_SALT = keccak256("PredictionMarket.MarketResolver.v1");
    bytes32 constant POSITION_TOKENS_PROXY_SALT = keccak256("PredictionMarket.PositionTokens.v1");
    bytes32 constant VAULT_PROXY_SALT = keccak256("PredictionMarket.Vault.v1");
    bytes32 constant MARKET_CONTROLLER_PROXY_SALT = keccak256("PredictionMarket.MarketController.v1");

    function run() external {
        // Load configuration
        DeployConfig memory config = getDeployConfig();

        console.log("=== Deterministic Prediction Market Deployment (Create2) ===");
        console.log("Deployer:", msg.sender);
        console.log("Owner:", config.owner);
        console.log("Oracle:", config.oracle);
        console.log("Deploy Mock Token:", config.deployMockToken);
        console.log("Global Salt:", vm.toString(config.salt));

        vm.startBroadcast();

        // Deploy contracts using Create2
        DeployedContracts memory contracts = deployContractsCreate2(config);

        // Link contracts
        linkContracts(contracts);

        vm.stopBroadcast();

        // Save deployment addresses
        saveDeploymentAddresses(contracts, config);

        // Verify deployment
        verifyDeployment(contracts, config);

        console.log("=== Deployment Complete ===");
        logFinalAddresses(contracts);
    }

    function deployContractsCreate2(DeployConfig memory config)
        internal
        returns (DeployedContracts memory contracts)
    {
        console.log("\n--- Deploying Contracts with Create2 ---");

        // 1. Deploy or use existing collateral token
        if (config.deployMockToken) {
            console.log("Deploying mock ERC20 token with Create2...");
            
            address expectedToken = Create2.computeAddress(
                config.salt,
                keccak256(type(ERC20Mock).creationCode),
                msg.sender
            );
            
            if (expectedToken.code.length == 0) {
                ERC20Mock token = new ERC20Mock{salt: config.salt}();
                contracts.collateralToken = address(token);
                
                // Mint initial supply to deployer for testing
                token.mint(msg.sender, config.mockTokenSupply);
                console.log("Mock token deployed:", contracts.collateralToken);
            } else {
                contracts.collateralToken = expectedToken;
                console.log("Mock token already exists:", contracts.collateralToken);
            }
        } else {
            contracts.collateralToken = config.collateralToken;
            console.log("Using existing collateral token:", contracts.collateralToken);
        }

        // 2. Deploy implementation contracts with Create2
        console.log("\nDeploying implementation contracts...");

        // MarketContract implementation
        address expectedMarketImpl = Create2.computeAddress(
            MARKET_IMPL_SALT,
            keccak256(type(MarketContract).creationCode),
            msg.sender
        );
        
        if (expectedMarketImpl.code.length == 0) {
            MarketContract marketImpl = new MarketContract{salt: MARKET_IMPL_SALT}();
            contracts.marketImpl = address(marketImpl);
            console.log("MarketContract implementation deployed:", contracts.marketImpl);
        } else {
            contracts.marketImpl = expectedMarketImpl;
            console.log("MarketContract implementation already exists:", contracts.marketImpl);
        }

        // MarketResolver implementation
        address expectedMarketResolverImpl = Create2.computeAddress(
            MARKET_RESOLVER_IMPL_SALT,
            keccak256(type(MarketResolver).creationCode),
            msg.sender
        );
        
        if (expectedMarketResolverImpl.code.length == 0) {
            MarketResolver marketResolverImpl = new MarketResolver{salt: MARKET_RESOLVER_IMPL_SALT}();
            contracts.marketResolverImpl = address(marketResolverImpl);
            console.log("MarketResolver implementation deployed:", contracts.marketResolverImpl);
        } else {
            contracts.marketResolverImpl = expectedMarketResolverImpl;
            console.log("MarketResolver implementation already exists:", contracts.marketResolverImpl);
        }

        // PositionTokens implementation
        address expectedPositionTokensImpl = Create2.computeAddress(
            POSITION_TOKENS_IMPL_SALT,
            keccak256(type(PositionTokens).creationCode),
            msg.sender
        );
        
        if (expectedPositionTokensImpl.code.length == 0) {
            PositionTokens positionTokensImpl = new PositionTokens{salt: POSITION_TOKENS_IMPL_SALT}();
            contracts.positionTokensImpl = address(positionTokensImpl);
            console.log("PositionTokens implementation deployed:", contracts.positionTokensImpl);
        } else {
            contracts.positionTokensImpl = expectedPositionTokensImpl;
            console.log("PositionTokens implementation already exists:", contracts.positionTokensImpl);
        }

        // Vault implementation
        address expectedVaultImpl = Create2.computeAddress(
            VAULT_IMPL_SALT,
            keccak256(type(Vault).creationCode),
            msg.sender
        );
        
        if (expectedVaultImpl.code.length == 0) {
            Vault vaultImpl = new Vault{salt: VAULT_IMPL_SALT}();
            contracts.vaultImpl = address(vaultImpl);
            console.log("Vault implementation deployed:", contracts.vaultImpl);
        } else {
            contracts.vaultImpl = expectedVaultImpl;
            console.log("Vault implementation already exists:", contracts.vaultImpl);
        }

        // MarketController implementation
        address expectedMarketControllerImpl = Create2.computeAddress(
            MARKET_CONTROLLER_IMPL_SALT,
            keccak256(type(MarketController).creationCode),
            msg.sender
        );
        
        if (expectedMarketControllerImpl.code.length == 0) {
            MarketController marketControllerImpl = new MarketController{salt: MARKET_CONTROLLER_IMPL_SALT}();
            contracts.marketControllerImpl = address(marketControllerImpl);
            console.log("MarketController implementation deployed:", contracts.marketControllerImpl);
        } else {
            contracts.marketControllerImpl = expectedMarketControllerImpl;
            console.log("MarketController implementation already exists:", contracts.marketControllerImpl);
        }

        // 3. Deploy proxies with Create2
        console.log("\nDeploying proxies...");

        // Market proxy
        bytes memory marketInitData = abi.encodeWithSelector(MarketContract.initialize.selector, config.owner);
        bytes memory marketProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.marketImpl, marketInitData)
        );
        
        address expectedMarket = Create2.computeAddress(
            MARKET_PROXY_SALT,
            keccak256(marketProxyBytecode),
            msg.sender
        );
        
        if (expectedMarket.code.length == 0) {
            ERC1967Proxy marketProxy = new ERC1967Proxy{salt: MARKET_PROXY_SALT}(
                contracts.marketImpl,
                marketInitData
            );
            contracts.market = address(marketProxy);
            console.log("Market proxy deployed:", contracts.market);
        } else {
            contracts.market = expectedMarket;
            console.log("Market proxy already exists:", contracts.market);
        }

        // MarketResolver proxy
        bytes memory marketResolverInitData = abi.encodeWithSelector(
            MarketResolver.initialize.selector,
            config.owner,
            config.oracle
        );
        bytes memory marketResolverProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.marketResolverImpl, marketResolverInitData)
        );
        
        address expectedMarketResolver = Create2.computeAddress(
            MARKET_RESOLVER_PROXY_SALT,
            keccak256(marketResolverProxyBytecode),
            msg.sender
        );
        
        if (expectedMarketResolver.code.length == 0) {
            ERC1967Proxy marketResolverProxy = new ERC1967Proxy{salt: MARKET_RESOLVER_PROXY_SALT}(
                contracts.marketResolverImpl,
                marketResolverInitData
            );
            contracts.marketResolver = address(marketResolverProxy);
            console.log("MarketResolver proxy deployed:", contracts.marketResolver);
        } else {
            contracts.marketResolver = expectedMarketResolver;
            console.log("MarketResolver proxy already exists:", contracts.marketResolver);
        }

        // PositionTokens proxy
        bytes memory positionTokensInitData = abi.encodeWithSelector(PositionTokens.initialize.selector, config.owner);
        bytes memory positionTokensProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.positionTokensImpl, positionTokensInitData)
        );
        
        address expectedPositionTokens = Create2.computeAddress(
            POSITION_TOKENS_PROXY_SALT,
            keccak256(positionTokensProxyBytecode),
            msg.sender
        );
        
        if (expectedPositionTokens.code.length == 0) {
            ERC1967Proxy positionTokensProxy = new ERC1967Proxy{salt: POSITION_TOKENS_PROXY_SALT}(
                contracts.positionTokensImpl,
                positionTokensInitData
            );
            contracts.positionTokens = address(positionTokensProxy);
            console.log("PositionTokens proxy deployed:", contracts.positionTokens);
        } else {
            contracts.positionTokens = expectedPositionTokens;
            console.log("PositionTokens proxy already exists:", contracts.positionTokens);
        }

        // Vault proxy
        bytes memory vaultInitData = abi.encodeWithSelector(
            Vault.initialize.selector,
            config.owner,
            contracts.collateralToken,
            msg.sender // Temporary, will be updated to MarketController
        );
        bytes memory vaultProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.vaultImpl, vaultInitData)
        );
        
        address expectedVault = Create2.computeAddress(
            VAULT_PROXY_SALT,
            keccak256(vaultProxyBytecode),
            msg.sender
        );
        
        if (expectedVault.code.length == 0) {
            ERC1967Proxy vaultProxy = new ERC1967Proxy{salt: VAULT_PROXY_SALT}(
                contracts.vaultImpl,
                vaultInitData
            );
            contracts.vault = address(vaultProxy);
            console.log("Vault proxy deployed:", contracts.vault);
        } else {
            contracts.vault = expectedVault;
            console.log("Vault proxy already exists:", contracts.vault);
        }

        // MarketController proxy (deployed last as it needs other contract addresses)
        bytes memory marketControllerInitData = abi.encodeWithSelector(
            MarketController.initialize.selector,
            config.owner,
            contracts.positionTokens,
            contracts.marketResolver,
            contracts.vault,
            contracts.market,
            config.oracle
        );
        bytes memory marketControllerProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.marketControllerImpl, marketControllerInitData)
        );
        
        address expectedMarketController = Create2.computeAddress(
            MARKET_CONTROLLER_PROXY_SALT,
            keccak256(marketControllerProxyBytecode),
            msg.sender
        );
        
        if (expectedMarketController.code.length == 0) {
            ERC1967Proxy marketControllerProxy = new ERC1967Proxy{salt: MARKET_CONTROLLER_PROXY_SALT}(
                contracts.marketControllerImpl,
                marketControllerInitData
            );
            contracts.marketController = address(marketControllerProxy);
            console.log("MarketController proxy deployed:", contracts.marketController);
        } else {
            contracts.marketController = expectedMarketController;
            console.log("MarketController proxy already exists:", contracts.marketController);
        }
    }

    function linkContracts(DeployedContracts memory contracts) internal {
        console.log("\n--- Linking Contracts ---");

        // Set MarketController in all contracts
        MarketContract(contracts.market).setMarketController(contracts.marketController);
        console.log("Set MarketController in Market");

        PositionTokens(contracts.positionTokens).setMarketController(contracts.marketController);
        console.log("Set MarketController in PositionTokens");

        Vault(contracts.vault).setMarketController(contracts.marketController);
        console.log("Set MarketController in Vault");

        // Set EmergencyResolver in MarketResolver
        MarketResolver(contracts.marketResolver).setEmergencyResolver(contracts.marketController);
        console.log("Set EmergencyResolver in MarketResolver");

        console.log("Contract linking complete!");
    }

    function getDeployConfig() internal view returns (DeployConfig memory config) {
        // Try to read from environment variables, otherwise use defaults
        try vm.envAddress("OWNER_ADDRESS") returns (address ownerAddr) {
            config.owner = ownerAddr;
        } catch {
            config.owner = msg.sender;
        }
        
        try vm.envAddress("ORACLE_ADDRESS") returns (address oracleAddr) {
            config.oracle = oracleAddr;
        } catch {
            config.oracle = msg.sender;
        }
        
        try vm.envAddress("COLLATERAL_TOKEN_ADDRESS") returns (address tokenAddr) {
            config.collateralToken = tokenAddr;
        } catch {
            config.collateralToken = address(0);
        }
        
        config.deployMockToken = (config.collateralToken == address(0));
        
        try vm.envUint("MOCK_TOKEN_SUPPLY") returns (uint256 supply) {
            config.mockTokenSupply = supply;
        } catch {
            config.mockTokenSupply = 1_000_000e18;
        }
        
        // Generate deterministic salt from deployer address and protocol name
        // This ensures same addresses across chains when using same deployer
        string memory saltString;
        try vm.envString("DEPLOYMENT_SALT") returns (string memory envSalt) {
            saltString = envSalt;
        } catch {
            saltString = "PredictionMarket.v1.0";
        }
        config.salt = keccak256(abi.encodePacked(saltString, msg.sender));

        // Validate configuration
        require(config.owner != address(0), "Owner address cannot be zero");
        require(config.oracle != address(0), "Oracle address cannot be zero");
        if (!config.deployMockToken) {
            require(config.collateralToken != address(0), "Collateral token address cannot be zero");
        }
    }

    function logFinalAddresses(DeployedContracts memory contracts) internal pure {
        console.log("\n--- Final Deployed Addresses ---");
        console.log("Main Entry Point:");
        console.log("  MarketController:", contracts.marketController);
        console.log("Supporting Contracts:");
        console.log("  Market:", contracts.market);
        console.log("  MarketResolver:", contracts.marketResolver);
        console.log("  PositionTokens:", contracts.positionTokens);
        console.log("  Vault:", contracts.vault);
        console.log("  CollateralToken:", contracts.collateralToken);
        console.log("\nThese addresses will be identical on all chains when using same deployer!");
    }

    function saveDeploymentAddresses(DeployedContracts memory contracts, DeployConfig memory config) internal {
        console.log("\n--- Saving Deployment Addresses ---");

        string memory json = "deployment";

        // Network info
        vm.serializeString(json, "network", getNetworkName());
        vm.serializeUint(json, "chainId", block.chainid);
        vm.serializeUint(json, "blockNumber", block.number);
        vm.serializeUint(json, "timestamp", block.timestamp);
        vm.serializeBytes32(json, "deploymentSalt", config.salt);
        vm.serializeBool(json, "deterministicDeployment", true);

        // Contract addresses
        vm.serializeAddress(json, "collateralToken", contracts.collateralToken);
        vm.serializeAddress(json, "marketImpl", contracts.marketImpl);
        vm.serializeAddress(json, "marketResolverImpl", contracts.marketResolverImpl);
        vm.serializeAddress(json, "positionTokensImpl", contracts.positionTokensImpl);
        vm.serializeAddress(json, "vaultImpl", contracts.vaultImpl);
        vm.serializeAddress(json, "marketControllerImpl", contracts.marketControllerImpl);
        vm.serializeAddress(json, "market", contracts.market);
        vm.serializeAddress(json, "marketResolver", contracts.marketResolver);
        vm.serializeAddress(json, "positionTokens", contracts.positionTokens);
        vm.serializeAddress(json, "vault", contracts.vault);
        string memory finalJson = vm.serializeAddress(json, "marketController", contracts.marketController);

        string memory fileName = string.concat("deployments/", getNetworkName(), ".json");
        vm.writeJson(finalJson, fileName);
        console.log("Deployment addresses saved to:", fileName);
    }

    function verifyDeployment(DeployedContracts memory contracts, DeployConfig memory config) internal view {
        console.log("\n--- Verifying Deployment ---");

        // Verify proxy ownership
        require(MarketContract(contracts.market).owner() == config.owner, "Market owner mismatch");
        require(MarketResolver(contracts.marketResolver).owner() == config.owner, "MarketResolver owner mismatch");
        require(PositionTokens(contracts.positionTokens).owner() == config.owner, "PositionTokens owner mismatch");
        require(Vault(contracts.vault).owner() == config.owner, "Vault owner mismatch");
        require(MarketController(contracts.marketController).owner() == config.owner, "MarketController owner mismatch");
        console.log("Owner verification passed");

        // Verify contract linking
        require(
            MarketContract(contracts.market).marketController() == contracts.marketController,
            "Market controller link failed"
        );
        require(
            PositionTokens(contracts.positionTokens).marketController() == contracts.marketController,
            "PositionTokens controller link failed"
        );
        require(Vault(contracts.vault).marketController() == contracts.marketController, "Vault controller link failed");
        require(
            MarketResolver(contracts.marketResolver).emergencyResolver() == contracts.marketController,
            "Emergency resolver link failed"
        );
        console.log("Contract linking verification passed");

        // Verify oracle setup
        require(MarketResolver(contracts.marketResolver).oracle() == config.oracle, "Oracle setup failed");
        console.log("Oracle verification passed");

        // Verify collateral token
        require(
            address(Vault(contracts.vault).collateralToken()) == contracts.collateralToken,
            "Collateral token setup failed"
        );
        console.log("Collateral token verification passed");

        console.log("All verifications passed!");
    }

    function getNetworkName() internal view returns (string memory) {
        uint256 chainId = block.chainid;

        if (chainId == 1) return "mainnet";
        if (chainId == 11155111) return "sepolia";
        if (chainId == 17000) return "holesky";
        if (chainId == 137) return "polygon";
        if (chainId == 80001) return "mumbai";
        if (chainId == 42161) return "arbitrum";
        if (chainId == 421614) return "arbitrum-sepolia";
        if (chainId == 10) return "optimism";
        if (chainId == 11155420) return "optimism-sepolia";
        if (chainId == 8453) return "base";
        if (chainId == 84532) return "base-sepolia";
        if (chainId == 56) return "bsc";
        if (chainId == 97) return "bsc-testnet";
        if (chainId == 146) return "sonic";
        if (chainId == 57054) return "sonic-testnet";
        if (chainId == 31337) return "anvil";

        return string.concat("chain-", vm.toString(chainId));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "../src/Market/Market.sol";
import "../src/Market/MarketController.sol";
import "../src/Market/MarketResolver.sol";
import "../src/Token/PositionTokens.sol";
import "../src/Vault/Vault.sol";

/**
 * @title Upgrade
 * @notice UUPS upgrade script for prediction market contracts
 * @dev Upgrades implementation contracts while preserving proxy addresses and state
 */
contract Upgrade is Script {
    using stdJson for string;

    struct CurrentDeployment {
        address collateralToken;
        address marketImpl;
        address marketResolverImpl;
        address positionTokensImpl;
        address vaultImpl;
        address marketControllerImpl;
        address market;           // proxy
        address marketResolver;   // proxy
        address positionTokens;   // proxy
        address vault;           // proxy
        address marketController; // proxy
        bytes32 deploymentSalt;
        bool deterministicDeployment;
    }

    struct NewImplementations {
        address marketImpl;
        address marketResolverImpl;
        address positionTokensImpl;
        address vaultImpl;
        address marketControllerImpl;
    }

    struct UpgradeConfig {
        bool upgradeMarket;
        bool upgradeMarketResolver;
        bool upgradePositionTokens;
        bool upgradeVault;
        bool upgradeMarketController;
        bool deployDeterministic;  // Whether to use Create2 for new implementations
        string upgradeReason;      // Optional reason for upgrade
    }

    // New salts for upgraded implementations (increment version)
    bytes32 constant MARKET_IMPL_SALT_V6 = keccak256("PredictionMarket.MarketImpl.v6");
    bytes32 constant MARKET_RESOLVER_IMPL_SALT_V6 = keccak256("PredictionMarket.MarketResolverImpl.v6");
    bytes32 constant POSITION_TOKENS_IMPL_SALT_V6 = keccak256("PredictionMarket.PositionTokensImpl.v6");
    bytes32 constant VAULT_IMPL_SALT_V6 = keccak256("PredictionMarket.VaultImpl.v6");
    bytes32 constant MARKET_CONTROLLER_IMPL_SALT_V6 = keccak256("PredictionMarket.MarketControllerImpl.v6");

    function run() external {
        console.log("=== UUPS Contract Upgrade ===");

        // Load current deployment
        CurrentDeployment memory current = loadCurrentDeployment();

        // Get upgrade configuration
        UpgradeConfig memory config = getUpgradeConfig();

        console.log("Upgrade Configuration:");
        console.log("  Network:", getNetworkName());
        console.log("  Upgrade Market:", config.upgradeMarket);
        console.log("  Upgrade MarketResolver:", config.upgradeMarketResolver);
        console.log("  Upgrade PositionTokens:", config.upgradePositionTokens);
        console.log("  Upgrade Vault:", config.upgradeVault);
        console.log("  Upgrade MarketController:", config.upgradeMarketController);
        console.log("  Deploy Deterministic:", config.deployDeterministic);
        if (bytes(config.upgradeReason).length > 0) {
            console.log("  Reason:", config.upgradeReason);
        }

        vm.startBroadcast();

        // Deploy new implementations
        NewImplementations memory newImpls = deployNewImplementations(current, config);

        // Perform upgrades
        performUpgrades(current, newImpls, config);

        vm.stopBroadcast();

        // Verify upgrades
        verifyUpgrades(current, newImpls, config);

        // Save upgrade information
        saveUpgradeInfo(current, newImpls, config);

        console.log("=== Upgrade Complete ===");
        logUpgradeSummary(current, newImpls, config);
    }

    function loadCurrentDeployment() internal view returns (CurrentDeployment memory deployment) {
        string memory networkName = getNetworkName();
        string memory fileName = string.concat("deployments/", networkName, ".json");

        console.log("Loading current deployment from:", fileName);

        string memory json = vm.readFile(fileName);

        deployment.collateralToken = json.readAddress(".collateralToken");
        deployment.marketImpl = json.readAddress(".marketImpl");
        deployment.marketResolverImpl = json.readAddress(".marketResolverImpl");
        deployment.positionTokensImpl = json.readAddress(".positionTokensImpl");
        deployment.vaultImpl = json.readAddress(".vaultImpl");
        deployment.marketControllerImpl = json.readAddress(".marketControllerImpl");
        deployment.market = json.readAddress(".market");
        deployment.marketResolver = json.readAddress(".marketResolver");
        deployment.positionTokens = json.readAddress(".positionTokens");
        deployment.vault = json.readAddress(".vault");
        deployment.marketController = json.readAddress(".marketController");
        deployment.deploymentSalt = json.readBytes32(".deploymentSalt");
        deployment.deterministicDeployment = json.readBool(".deterministicDeployment");

        console.log("Current deployment loaded:");
        console.log("  MarketController proxy:", deployment.marketController);
        console.log("  Current implementation:", deployment.marketControllerImpl);
    }

    function deployNewImplementations(CurrentDeployment memory current, UpgradeConfig memory config)
        internal
        returns (NewImplementations memory newImpls)
    {
        console.log("\n--- Deploying New Implementations ---");

        if (config.upgradeMarket) {
            if (config.deployDeterministic) {
                console.log("Deploying MarketContract implementation (Create2)...");
                MarketContract marketImpl = new MarketContract{salt: MARKET_IMPL_SALT_V6}();
                newImpls.marketImpl = address(marketImpl);
            } else {
                console.log("Deploying MarketContract implementation...");
                MarketContract marketImpl = new MarketContract();
                newImpls.marketImpl = address(marketImpl);
            }
            console.log("  New MarketContract implementation:", newImpls.marketImpl);
        } else {
            newImpls.marketImpl = current.marketImpl;
        }

        if (config.upgradeMarketResolver) {
            if (config.deployDeterministic) {
                console.log("Deploying MarketResolver implementation (Create2)...");
                MarketResolver marketResolverImpl = new MarketResolver{salt: MARKET_RESOLVER_IMPL_SALT_V6}();
                newImpls.marketResolverImpl = address(marketResolverImpl);
            } else {
                console.log("Deploying MarketResolver implementation...");
                MarketResolver marketResolverImpl = new MarketResolver();
                newImpls.marketResolverImpl = address(marketResolverImpl);
            }
            console.log("  New MarketResolver implementation:", newImpls.marketResolverImpl);
        } else {
            newImpls.marketResolverImpl = current.marketResolverImpl;
        }

        if (config.upgradePositionTokens) {
            if (config.deployDeterministic) {
                console.log("Deploying PositionTokens implementation (Create2)...");
                PositionTokens positionTokensImpl = new PositionTokens{salt: POSITION_TOKENS_IMPL_SALT_V6}();
                newImpls.positionTokensImpl = address(positionTokensImpl);
            } else {
                console.log("Deploying PositionTokens implementation...");
                PositionTokens positionTokensImpl = new PositionTokens();
                newImpls.positionTokensImpl = address(positionTokensImpl);
            }
            console.log("  New PositionTokens implementation:", newImpls.positionTokensImpl);
        } else {
            newImpls.positionTokensImpl = current.positionTokensImpl;
        }

        if (config.upgradeVault) {
            if (config.deployDeterministic) {
                console.log("Deploying Vault implementation (Create2)...");
                Vault vaultImpl = new Vault{salt: VAULT_IMPL_SALT_V6}();
                newImpls.vaultImpl = address(vaultImpl);
            } else {
                console.log("Deploying Vault implementation...");
                Vault vaultImpl = new Vault();
                newImpls.vaultImpl = address(vaultImpl);
            }
            console.log("  New Vault implementation:", newImpls.vaultImpl);
        } else {
            newImpls.vaultImpl = current.vaultImpl;
        }

        if (config.upgradeMarketController) {
            if (config.deployDeterministic) {
                console.log("Deploying MarketController implementation (Create2)...");
                MarketController marketControllerImpl = new MarketController{salt: MARKET_CONTROLLER_IMPL_SALT_V6}();
                newImpls.marketControllerImpl = address(marketControllerImpl);
            } else {
                console.log("Deploying MarketController implementation...");
                MarketController marketControllerImpl = new MarketController();
                newImpls.marketControllerImpl = address(marketControllerImpl);
            }
            console.log("  New MarketController implementation:", newImpls.marketControllerImpl);
        } else {
            newImpls.marketControllerImpl = current.marketControllerImpl;
        }
    }

    function performUpgrades(
        CurrentDeployment memory current,
        NewImplementations memory newImpls,
        UpgradeConfig memory config
    ) internal {
        console.log("\n--- Performing UUPS Upgrades ---");

        if (config.upgradeMarket) {
            console.log("Upgrading Market proxy...");
            UUPSUpgradeable(current.market).upgradeToAndCall(
                newImpls.marketImpl,
                ""  // No initialization data needed
            );
            console.log("  Market upgraded successfully");
        }

        if (config.upgradeMarketResolver) {
            console.log("Upgrading MarketResolver proxy...");
            UUPSUpgradeable(current.marketResolver).upgradeToAndCall(
                newImpls.marketResolverImpl,
                ""
            );
            console.log("  MarketResolver upgraded successfully");
        }

        if (config.upgradePositionTokens) {
            console.log("Upgrading PositionTokens proxy...");
            UUPSUpgradeable(current.positionTokens).upgradeToAndCall(
                newImpls.positionTokensImpl,
                ""
            );
            console.log("  PositionTokens upgraded successfully");
        }

        if (config.upgradeVault) {
            console.log("Upgrading Vault proxy...");
            UUPSUpgradeable(current.vault).upgradeToAndCall(
                newImpls.vaultImpl,
                ""
            );
            console.log("  Vault upgraded successfully");
        }

        if (config.upgradeMarketController) {
            console.log("Upgrading MarketController proxy...");
            UUPSUpgradeable(current.marketController).upgradeToAndCall(
                newImpls.marketControllerImpl,
                ""
            );
            console.log("  MarketController upgraded successfully");
        }
    }

    function verifyUpgrades(
        CurrentDeployment memory current,
        NewImplementations memory newImpls,
        UpgradeConfig memory config
    ) internal view {
        console.log("\n--- Verifying Upgrades ---");

        if (config.upgradeMarket) {
            // Verify the proxy is still owned by the correct owner and functioning
            address owner = MarketContract(current.market).owner();
            console.log("Market proxy owner verified:", owner);

            // Verify proxy points to new implementation
            // Note: This verification is simplified - in practice you'd check implementation address
            require(owner != address(0), "Market upgrade verification failed");
        }

        if (config.upgradeMarketController) {
            address owner = MarketController(current.marketController).owner();
            console.log("MarketController proxy owner verified:", owner);
            require(owner != address(0), "MarketController upgrade verification failed");
        }

        if (config.upgradeVault) {
            address owner = Vault(current.vault).owner();
            console.log("Vault proxy owner verified:", owner);
            require(owner != address(0), "Vault upgrade verification failed");
        }

        console.log("All upgrade verifications passed!");
    }

    function saveUpgradeInfo(
        CurrentDeployment memory current,
        NewImplementations memory newImpls,
        UpgradeConfig memory config
    ) internal {
        console.log("\n--- Saving Upgrade Information ---");

        string memory networkName = getNetworkName();

        // Update the main deployment file with new implementation addresses
        string memory json = "upgrade";

        // Network info
        vm.serializeString(json, "network", networkName);
        vm.serializeUint(json, "chainId", block.chainid);
        vm.serializeUint(json, "upgradeBlockNumber", block.number);
        vm.serializeUint(json, "upgradeTimestamp", block.timestamp);
        vm.serializeBytes32(json, "deploymentSalt", current.deploymentSalt);
        vm.serializeBool(json, "deterministicDeployment", current.deterministicDeployment);

        // Contract addresses (proxies remain the same)
        vm.serializeAddress(json, "collateralToken", current.collateralToken);
        vm.serializeAddress(json, "marketImpl", newImpls.marketImpl);
        vm.serializeAddress(json, "marketResolverImpl", newImpls.marketResolverImpl);
        vm.serializeAddress(json, "positionTokensImpl", newImpls.positionTokensImpl);
        vm.serializeAddress(json, "vaultImpl", newImpls.vaultImpl);
        vm.serializeAddress(json, "marketControllerImpl", newImpls.marketControllerImpl);
        vm.serializeAddress(json, "market", current.market);
        vm.serializeAddress(json, "marketResolver", current.marketResolver);
        vm.serializeAddress(json, "positionTokens", current.positionTokens);
        vm.serializeAddress(json, "vault", current.vault);
        string memory finalJson = vm.serializeAddress(json, "marketController", current.marketController);

        // Save updated deployment
        string memory fileName = string.concat("deployments/", networkName, ".json");
        vm.writeJson(finalJson, fileName);
        console.log("Updated deployment saved to:", fileName);

        // Also save upgrade history
        string memory upgradeJson = "upgradeHistory";
        vm.serializeUint(upgradeJson, "blockNumber", block.number);
        vm.serializeUint(upgradeJson, "timestamp", block.timestamp);
        vm.serializeString(upgradeJson, "reason", config.upgradeReason);

        // Previous implementations
        vm.serializeAddress(upgradeJson, "previous_marketImpl", current.marketImpl);
        vm.serializeAddress(upgradeJson, "previous_marketResolverImpl", current.marketResolverImpl);
        vm.serializeAddress(upgradeJson, "previous_positionTokensImpl", current.positionTokensImpl);
        vm.serializeAddress(upgradeJson, "previous_vaultImpl", current.vaultImpl);
        vm.serializeAddress(upgradeJson, "previous_marketControllerImpl", current.marketControllerImpl);

        // New implementations
        vm.serializeAddress(upgradeJson, "new_marketImpl", newImpls.marketImpl);
        vm.serializeAddress(upgradeJson, "new_marketResolverImpl", newImpls.marketResolverImpl);
        vm.serializeAddress(upgradeJson, "new_positionTokensImpl", newImpls.positionTokensImpl);
        vm.serializeAddress(upgradeJson, "new_vaultImpl", newImpls.vaultImpl);
        string memory finalUpgradeJson = vm.serializeAddress(upgradeJson, "new_marketControllerImpl", newImpls.marketControllerImpl);

        string memory upgradeHistoryFile = string.concat("deployments/", networkName, "-upgrade-", vm.toString(block.timestamp), ".json");
        vm.writeJson(finalUpgradeJson, upgradeHistoryFile);
        console.log("Upgrade history saved to:", upgradeHistoryFile);
    }

    function getUpgradeConfig() internal view returns (UpgradeConfig memory config) {
        // Read configuration from environment variables with defaults
        try vm.envBool("UPGRADE_MARKET") returns (bool upgrade) {
            config.upgradeMarket = upgrade;
        } catch {
            config.upgradeMarket = true;  // Default: upgrade all
        }

        try vm.envBool("UPGRADE_MARKET_RESOLVER") returns (bool upgrade) {
            config.upgradeMarketResolver = upgrade;
        } catch {
            config.upgradeMarketResolver = true;
        }

        try vm.envBool("UPGRADE_POSITION_TOKENS") returns (bool upgrade) {
            config.upgradePositionTokens = upgrade;
        } catch {
            config.upgradePositionTokens = true;
        }

        try vm.envBool("UPGRADE_VAULT") returns (bool upgrade) {
            config.upgradeVault = upgrade;
        } catch {
            config.upgradeVault = true;
        }

        try vm.envBool("UPGRADE_MARKET_CONTROLLER") returns (bool upgrade) {
            config.upgradeMarketController = upgrade;
        } catch {
            config.upgradeMarketController = true;
        }

        try vm.envBool("UPGRADE_DETERMINISTIC") returns (bool deterministic) {
            config.deployDeterministic = deterministic;
        } catch {
            config.deployDeterministic = true;  // Default to deterministic for consistency
        }

        try vm.envString("UPGRADE_REASON") returns (string memory reason) {
            config.upgradeReason = reason;
        } catch {
            config.upgradeReason = "Contract upgrade";
        }
    }

    function logUpgradeSummary(
        CurrentDeployment memory current,
        NewImplementations memory newImpls,
        UpgradeConfig memory config
    ) internal pure {
        console.log("\n--- Upgrade Summary ---");
        console.log("Proxy addresses (unchanged):");
        console.log("  MarketController:", current.marketController);
        console.log("  Market:", current.market);
        console.log("  MarketResolver:", current.marketResolver);
        console.log("  PositionTokens:", current.positionTokens);
        console.log("  Vault:", current.vault);
        console.log("");
        console.log("Implementation changes:");

        if (config.upgradeMarketController) {
            console.log("  MarketController:", current.marketControllerImpl, "->", newImpls.marketControllerImpl);
        }
        if (config.upgradeMarket) {
            console.log("  Market:", current.marketImpl, "->", newImpls.marketImpl);
        }
        if (config.upgradeMarketResolver) {
            console.log("  MarketResolver:", current.marketResolverImpl, "->", newImpls.marketResolverImpl);
        }
        if (config.upgradePositionTokens) {
            console.log("  PositionTokens:", current.positionTokensImpl, "->", newImpls.positionTokensImpl);
        }
        if (config.upgradeVault) {
            console.log("  Vault:", current.vaultImpl, "->", newImpls.vaultImpl);
        }
        console.log("");
        console.log("Users can continue using the same proxy addresses!");
        console.log("All state and balances are preserved.");
    }

    function getNetworkName() internal view returns (string memory) {
        uint256 chainId = block.chainid;

        if (chainId == 1) return "mainnet";
        if (chainId == 11155111) return "sepolia";
        if (chainId == 17000) return "holesky";
        if (chainId == 137) return "polygon";
        if (chainId == 80001) return "mumbai";
        if (chainId == 42161) return "arbitrum";
        if (chainId == 421614) return "arbitrum-sepolia";
        if (chainId == 10) return "optimism";
        if (chainId == 11155420) return "optimism-sepolia";
        if (chainId == 8453) return "base";
        if (chainId == 84532) return "base-sepolia";
        if (chainId == 56) return "bsc";
        if (chainId == 97) return "bsc-testnet";
        if (chainId == 146) return "sonic";
        if (chainId == 57054) return "sonic-testnet";
        if (chainId == 31337) return "anvil";

        return string.concat("chain-", vm.toString(chainId));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

import "../src/Market/Market.sol";
import "../src/Market/MarketController.sol";
import "../src/Market/MarketResolver.sol";
import "../src/Token/PositionTokens.sol";
import "../src/Vault/Vault.sol";

/**
 * @title Setup
 * @notice Post-deployment setup script for prediction market contracts
 * @dev Handles initial configuration, test markets, and user setup
 */
contract Setup is Script {
    using stdJson for string;

    struct DeployedContracts {
        address collateralToken;
        address market;
        address marketResolver;
        address positionTokens;
        address vault;
        address marketController;
    }

    struct SetupConfig {
        bool createTestMarkets;
        bool setupTestUsers;
        bool setAuthorizedMatchers;
        address[] testUsers;
        address[] authorizedMatchers;
        uint256 testTokenAmount;
    }

    function run() external {
        console.log("=== Post-Deployment Setup ===");

        // Load deployed contract addresses
        DeployedContracts memory contracts = loadDeployedContracts();

        // Load setup configuration
        SetupConfig memory config = getSetupConfig();

        vm.startBroadcast();

        // Execute setup steps
        if (config.createTestMarkets) {
            createTestMarkets(contracts);
        }

        if (config.setupTestUsers) {
            setupTestUsers(contracts, config);
        }

        if (config.setAuthorizedMatchers) {
            setAuthorizedMatchers(contracts, config);
        }

        vm.stopBroadcast();

        // Verify setup
        verifySetup(contracts, config);

        console.log("=== Setup Complete ===");
    }

    function loadDeployedContracts() internal view returns (DeployedContracts memory contracts) {
        string memory networkName = getNetworkName();
        string memory fileName = string.concat("deployments/", networkName, ".json");

        console.log("Loading deployment from:", fileName);

        string memory json = vm.readFile(fileName);

        contracts.collateralToken = json.readAddress(".collateralToken");
        contracts.market = json.readAddress(".market");
        contracts.marketResolver = json.readAddress(".marketResolver");
        contracts.positionTokens = json.readAddress(".positionTokens");
        contracts.vault = json.readAddress(".vault");
        contracts.marketController = json.readAddress(".marketController");

        console.log("Loaded deployment addresses:");
        console.log("  Market:", contracts.market);
        console.log("  MarketController:", contracts.marketController);
        console.log("  Vault:", contracts.vault);
    }

    function createTestMarkets(DeployedContracts memory contracts) internal {
        console.log("\n--- Creating Test Markets ---");

        MarketController controller = MarketController(contracts.marketController);

        // Create various test markets
        bytes32[] memory questionIds = new bytes32[](5);
        string[] memory descriptions = new string[](5);
        uint256[] memory outcomeCounts = new uint256[](5);
        uint256[] memory resolutionTimes = new uint256[](5);

        // Market 1: Bitcoin price binary (manual resolution)
        questionIds[0] = keccak256("BTC_USD_50000");
        descriptions[0] = "Will Bitcoin price exceed $50,000 by end of month?";
        outcomeCounts[0] = 2;
        resolutionTimes[0] = 0; // Manual resolution

        // Market 2: Ethereum price binary (time-based resolution)
        questionIds[1] = keccak256("ETH_USD_3000");
        descriptions[1] = "Will Ethereum price exceed $3,000 by end of week?";
        outcomeCounts[1] = 2;
        resolutionTimes[1] = block.timestamp + 7 days;

        // Market 3: Stock market direction (multi-outcome)
        questionIds[2] = keccak256("SPY_DIRECTION");
        descriptions[2] = "S&P 500 direction next week: Up, Down, Sideways, Volatile";
        outcomeCounts[2] = 4;
        resolutionTimes[2] = block.timestamp + 7 days;

        // Market 4: Weather prediction
        questionIds[3] = keccak256("WEATHER_NYC");
        descriptions[3] = "NYC weather tomorrow: Sunny, Cloudy, Rainy";
        outcomeCounts[3] = 3;
        resolutionTimes[3] = block.timestamp + 1 days;

        // Market 5: Sports outcome
        questionIds[4] = keccak256("SPORTS_MATCH");
        descriptions[4] = "Next major game outcome: Team A, Team B";
        outcomeCounts[4] = 2;
        resolutionTimes[4] = block.timestamp + 3 days;

        for (uint256 i = 0; i < questionIds.length; i++) {
            try controller.createMarket(questionIds[i], outcomeCounts[i], resolutionTimes[i], 0) {
                console.log("Created market:", descriptions[i]);
                console.log("  Question ID:", vm.toString(questionIds[i]));
                console.log("  Outcomes:", outcomeCounts[i]);
                if (resolutionTimes[i] > 0) {
                    console.log("  Resolution time:", resolutionTimes[i]);
                } else {
                    console.log("  Manual resolution");
                }
            } catch {
                console.log("Failed to create market:", descriptions[i]);
            }
        }

        // Save test market info
        saveTestMarketInfo(questionIds, descriptions, outcomeCounts, resolutionTimes);
    }

    function setupTestUsers(DeployedContracts memory contracts, SetupConfig memory config) internal {
        console.log("\n--- Setting Up Test Users ---");

        // Only setup test users if we have a mock token (development environment)
        if (config.testUsers.length == 0) {
            console.log("No test users specified, skipping...");
            return;
        }

        ERC20Mock collateralToken = ERC20Mock(contracts.collateralToken);
        // Vault vault = Vault(contracts.vault);

        // Check if this is a mock token by trying to mint (will revert if not mock)
        try collateralToken.mint(address(this), 1) {
            // This is a mock token, we can mint for test users
            for (uint256 i = 0; i < config.testUsers.length; i++) {
                address user = config.testUsers[i];

                // Mint test tokens
                collateralToken.mint(user, config.testTokenAmount);
                console.log("Minted", config.testTokenAmount, "tokens for user:", user);

                // Note: Users will need to approve and deposit manually or via frontend
            }
        } catch {
            console.log("Not a mock token, skipping test user setup");
        }
    }

    function setAuthorizedMatchers(DeployedContracts memory contracts, SetupConfig memory config) internal {
        console.log("\n--- Setting Authorized Matchers ---");

        if (config.authorizedMatchers.length == 0) {
            console.log("No authorized matchers specified, skipping...");
            return;
        }

        MarketController controller = MarketController(contracts.marketController);

        for (uint256 i = 0; i < config.authorizedMatchers.length; i++) {
            address matcher = config.authorizedMatchers[i];
            controller.setAuthorizedMatcher(matcher, true);
            console.log("Authorized matcher:", matcher);
        }
    }

    function getSetupConfig() internal view returns (SetupConfig memory config) {
        // Read configuration from environment variables with try/catch
        try vm.envBool("CREATE_TEST_MARKETS") returns (bool createMarkets) {
            config.createTestMarkets = createMarkets;
        } catch {
            config.createTestMarkets = true;
        }
        
        try vm.envBool("SETUP_TEST_USERS") returns (bool setupUsers) {
            config.setupTestUsers = setupUsers;
        } catch {
            config.setupTestUsers = false;
        }
        
        try vm.envBool("SET_AUTHORIZED_MATCHERS") returns (bool setMatchers) {
            config.setAuthorizedMatchers = setMatchers;
        } catch {
            config.setAuthorizedMatchers = false;
        }
        
        try vm.envUint("TEST_TOKEN_AMOUNT") returns (uint256 tokenAmount) {
            config.testTokenAmount = tokenAmount;
        } catch {
            config.testTokenAmount = 10000e18;
        }

        // Parse test user addresses (comma-separated)
        try vm.envString("TEST_USER_ADDRESSES") returns (string memory userAddresses) {
            if (bytes(userAddresses).length > 0) {
                config.testUsers = parseAddresses(userAddresses);
            }
        } catch {
            // No test users specified
        }

        // Parse authorized matcher addresses (comma-separated)
        try vm.envString("AUTHORIZED_MATCHER_ADDRESSES") returns (string memory matcherAddresses) {
            if (bytes(matcherAddresses).length > 0) {
                config.authorizedMatchers = parseAddresses(matcherAddresses);
            }
        } catch {
            // No matchers specified
        }

        console.log("Setup configuration:");
        console.log("  Create test markets:", config.createTestMarkets);
        console.log("  Setup test users:", config.setupTestUsers);
        console.log("  Set authorized matchers:", config.setAuthorizedMatchers);
        console.log("  Test users count:", config.testUsers.length);
        console.log("  Authorized matchers count:", config.authorizedMatchers.length);
    }

    function parseAddresses(string memory addressString) internal pure returns (address[] memory addresses) {
        // Simple comma-separated address parser
        // In practice, you might want a more robust parser
        bytes memory data = bytes(addressString);
        uint256 count = 1;

        // Count commas to determine array size
        for (uint256 i = 0; i < data.length; i++) {
            if (data[i] == bytes1(",")) {
                count++;
            }
        }

        addresses = new address[](count);
        // Note: This is a simplified implementation
        // For production, use a proper CSV parser or JSON format
    }

    function saveTestMarketInfo(
        bytes32[] memory questionIds,
        string[] memory descriptions,
        uint256[] memory outcomeCounts,
        uint256[] memory resolutionTimes
    ) internal {
        console.log("\n--- Saving Test Market Info ---");

        string memory json = "testMarkets";

        for (uint256 i = 0; i < questionIds.length; i++) {
            string memory marketKey = string.concat("market", vm.toString(i));

            vm.serializeBytes32(json, string.concat(marketKey, ".questionId"), questionIds[i]);
            vm.serializeString(json, string.concat(marketKey, ".description"), descriptions[i]);
            vm.serializeUint(json, string.concat(marketKey, ".outcomeCount"), outcomeCounts[i]);
            vm.serializeUint(json, string.concat(marketKey, ".resolutionTime"), resolutionTimes[i]);
        }

        string memory finalJson = vm.serializeUint(json, "count", questionIds.length);

        string memory fileName = string.concat("deployments/", getNetworkName(), "-test-markets.json");
        vm.writeJson(finalJson, fileName);
        console.log("Test market info saved to:", fileName);
    }

    function verifySetup(DeployedContracts memory contracts, SetupConfig memory config) internal view {
        console.log("\n--- Verifying Setup ---");

        MarketContract market = MarketContract(contracts.market);

        if (config.createTestMarkets) {
            // Verify at least one test market was created
            bytes32 testQuestionId = keccak256("BTC_USD_50000");
            require(market.getMarketExists(testQuestionId), "Test market creation failed");
            console.log("Test markets verified");
        }

        console.log("Setup verification complete!");
    }

    function getNetworkName() internal view returns (string memory) {
        uint256 chainId = block.chainid;

        if (chainId == 1) return "mainnet";
        if (chainId == 11155111) return "sepolia";
        if (chainId == 17000) return "holesky";
        if (chainId == 137) return "polygon";
        if (chainId == 80001) return "mumbai";
        if (chainId == 42161) return "arbitrum";
        if (chainId == 421614) return "arbitrum-sepolia";
        if (chainId == 10) return "optimism";
        if (chainId == 11155420) return "optimism-sepolia";
        if (chainId == 8453) return "base";
        if (chainId == 84532) return "base-sepolia";
        if (chainId == 56) return "bsc";
        if (chainId == 97) return "bsc-testnet";
        if (chainId == 146) return "sonic";
        if (chainId == 57054) return "sonic-testnet";
        if (chainId == 31337) return "anvil";

        return string.concat("chain-", vm.toString(chainId));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

import "../src/Market/Market.sol";
import "../src/Market/MarketController.sol";
import "../src/Market/MarketResolver.sol";
import "../src/Token/PositionTokens.sol";
import "../src/Vault/Vault.sol";

/**
 * @title PredictAddresses
 * @notice Predict contract addresses that will be deployed via built-in Create2
 * @dev Run this before deployment to verify addresses will be consistent across chains
 */
contract PredictAddresses is Script {
    
    // Custom salts for each contract type (must match Deploy.s.sol)
    bytes32 constant MARKET_IMPL_SALT = keccak256("PredictionMarket.MarketImpl.v1");
    bytes32 constant MARKET_RESOLVER_IMPL_SALT = keccak256("PredictionMarket.MarketResolverImpl.v1");
    bytes32 constant POSITION_TOKENS_IMPL_SALT = keccak256("PredictionMarket.PositionTokensImpl.v1");
    bytes32 constant VAULT_IMPL_SALT = keccak256("PredictionMarket.VaultImpl.v1");
    bytes32 constant MARKET_CONTROLLER_IMPL_SALT = keccak256("PredictionMarket.MarketControllerImpl.v1");
    bytes32 constant MARKET_PROXY_SALT = keccak256("PredictionMarket.Market.v1");
    bytes32 constant MARKET_RESOLVER_PROXY_SALT = keccak256("PredictionMarket.MarketResolver.v1");
    bytes32 constant POSITION_TOKENS_PROXY_SALT = keccak256("PredictionMarket.PositionTokens.v1");
    bytes32 constant VAULT_PROXY_SALT = keccak256("PredictionMarket.Vault.v1");
    bytes32 constant MARKET_CONTROLLER_PROXY_SALT = keccak256("PredictionMarket.MarketController.v1");

    struct PredictionConfig {
        address deployer;
        address owner;
        address oracle;
        address collateralToken;
        bool deployMockToken;
        bytes32 salt;
    }

    struct PredictedAddresses {
        address collateralToken;
        address marketImpl;
        address marketResolverImpl;
        address positionTokensImpl;
        address vaultImpl;
        address marketControllerImpl;
        address market;
        address marketResolver;
        address positionTokens;
        address vault;
        address marketController;
    }

    function run() external view {
        console.log("=== Address Prediction for Create2 Deployment ===");
        
        PredictionConfig memory config = getPredictionConfig();
        
        console.log("Deployer:", config.deployer);
        console.log("Owner:", config.owner);
        console.log("Oracle:", config.oracle);
        console.log("Deploy Mock Token:", config.deployMockToken);
        console.log("Global Salt:", vm.toString(config.salt));
        console.log("Current Chain ID:", block.chainid);
        
        PredictedAddresses memory addresses = predictAllAddresses(config);
        
        logPredictedAddresses(addresses);
        generateDeploymentSummary(addresses, config);
        logNetworkSpecificInfo();
    }

    function predictAllAddresses(PredictionConfig memory config) internal pure returns (PredictedAddresses memory addresses) {
        // Predict collateral token
        if (config.deployMockToken) {
            addresses.collateralToken = Create2.computeAddress(
                config.salt,
                keccak256(type(ERC20Mock).creationCode),
                config.deployer
            );
        } else {
            addresses.collateralToken = config.collateralToken;
        }

        // Predict implementation addresses using Create2
        addresses.marketImpl = Create2.computeAddress(
            MARKET_IMPL_SALT,
            keccak256(type(MarketContract).creationCode),
            config.deployer
        );

        addresses.marketResolverImpl = Create2.computeAddress(
            MARKET_RESOLVER_IMPL_SALT,
            keccak256(type(MarketResolver).creationCode),
            config.deployer
        );

        addresses.positionTokensImpl = Create2.computeAddress(
            POSITION_TOKENS_IMPL_SALT,
            keccak256(type(PositionTokens).creationCode),
            config.deployer
        );

        addresses.vaultImpl = Create2.computeAddress(
            VAULT_IMPL_SALT,
            keccak256(type(Vault).creationCode),
            config.deployer
        );

        addresses.marketControllerImpl = Create2.computeAddress(
            MARKET_CONTROLLER_IMPL_SALT,
            keccak256(type(MarketController).creationCode),
            config.deployer
        );

        // Predict proxy addresses - need to include constructor parameters
        // Market proxy
        bytes memory marketInitData = abi.encodeWithSelector(MarketContract.initialize.selector, config.owner);
        bytes memory marketProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.marketImpl, marketInitData)
        );
        addresses.market = Create2.computeAddress(MARKET_PROXY_SALT, keccak256(marketProxyBytecode), config.deployer);

        // MarketResolver proxy
        bytes memory marketResolverInitData = abi.encodeWithSelector(
            MarketResolver.initialize.selector,
            config.owner,
            config.oracle
        );
        bytes memory marketResolverProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.marketResolverImpl, marketResolverInitData)
        );
        addresses.marketResolver = Create2.computeAddress(
            MARKET_RESOLVER_PROXY_SALT,
            keccak256(marketResolverProxyBytecode),
            config.deployer
        );

        // PositionTokens proxy
        bytes memory positionTokensInitData = abi.encodeWithSelector(PositionTokens.initialize.selector, config.owner);
        bytes memory positionTokensProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.positionTokensImpl, positionTokensInitData)
        );
        addresses.positionTokens = Create2.computeAddress(
            POSITION_TOKENS_PROXY_SALT,
            keccak256(positionTokensProxyBytecode),
            config.deployer
        );

        // Vault proxy
        bytes memory vaultInitData = abi.encodeWithSelector(
            Vault.initialize.selector,
            config.owner,
            addresses.collateralToken,
            config.deployer // Temporary, will be updated to MarketController
        );
        bytes memory vaultProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.vaultImpl, vaultInitData)
        );
        addresses.vault = Create2.computeAddress(VAULT_PROXY_SALT, keccak256(vaultProxyBytecode), config.deployer);

        // MarketController proxy
        bytes memory marketControllerInitData = abi.encodeWithSelector(
            MarketController.initialize.selector,
            config.owner,
            addresses.positionTokens,
            addresses.marketResolver,
            addresses.vault,
            addresses.market,
            config.oracle
        );
        bytes memory marketControllerProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.marketControllerImpl, marketControllerInitData)
        );
        addresses.marketController = Create2.computeAddress(
            MARKET_CONTROLLER_PROXY_SALT,
            keccak256(marketControllerProxyBytecode),
            config.deployer
        );
    }

    function getPredictionConfig() internal view returns (PredictionConfig memory config) {
        // Use msg.sender as the deployer (this matches the Deploy script)
        config.deployer = msg.sender;
        
        try vm.envAddress("OWNER_ADDRESS") returns (address ownerAddr) {
            config.owner = ownerAddr;
        } catch {
            config.owner = msg.sender;
        }
        
        try vm.envAddress("ORACLE_ADDRESS") returns (address oracleAddr) {
            config.oracle = oracleAddr;
        } catch {
            config.oracle = msg.sender;
        }
        
        try vm.envAddress("COLLATERAL_TOKEN_ADDRESS") returns (address tokenAddr) {
            config.collateralToken = tokenAddr;
        } catch {
            config.collateralToken = address(0);
        }
        
        config.deployMockToken = (config.collateralToken == address(0));
        
        // Generate deterministic salt from deployer address and protocol name
        string memory saltString;
        try vm.envString("DEPLOYMENT_SALT") returns (string memory envSalt) {
            saltString = envSalt;
        } catch {
            saltString = "PredictionMarket.v1.0";
        }
        config.salt = keccak256(abi.encodePacked(saltString, msg.sender));
    }

    function logPredictedAddresses(PredictedAddresses memory addresses) internal pure {
        console.log("\n=== PREDICTED ADDRESSES ===");
        console.log("(These will be IDENTICAL on all supported chains)");
        
        console.log("\nIMPLEMENTATION CONTRACTS:");
        console.log("MarketContract:      ", addresses.marketImpl);
        console.log("MarketResolver:      ", addresses.marketResolverImpl);
        console.log("PositionTokens:      ", addresses.positionTokensImpl);
        console.log("Vault:               ", addresses.vaultImpl);
        console.log("MarketController:    ", addresses.marketControllerImpl);
        
        console.log("\nPROXY CONTRACTS (Main Interfaces):");
        console.log("Market:              ", addresses.market);
        console.log("MarketResolver:      ", addresses.marketResolver);
        console.log("PositionTokens:      ", addresses.positionTokens);
        console.log("Vault:               ", addresses.vault);
        console.log("MarketController:    ", addresses.marketController);
        
        console.log("\nCOLLATERAL TOKEN:");
        console.log("CollateralToken:     ", addresses.collateralToken);
        
        console.log("\nMAIN ENTRY POINT:");
        console.log("MarketController:    ", addresses.marketController);
        console.log("(This is the contract users will interact with)");
    }

    function generateDeploymentSummary(PredictedAddresses memory addresses, PredictionConfig memory config) internal pure {
        console.log("\n=== DEPLOYMENT SUMMARY ===");
        
        console.log("Deployment Method: Solidity Create2 (Deterministic)");
        console.log("Deployer Address:  ", config.deployer);
        console.log("Same addresses on: ALL supported chains");
        console.log("Owner Will Be:     ", config.owner);
        console.log("Oracle Will Be:    ", config.oracle);
        console.log("");
        
        console.log("PRIMARY CONTRACT FOR USERS:");
        console.log("MarketController: ", addresses.marketController);
        console.log("");
        
        console.log("COPY-PASTE READY ADDRESSES:");
        console.log("MARKET_CONTROLLER_ADDRESS=", addresses.marketController);
        console.log("MARKET_ADDRESS=", addresses.market);
        console.log("VAULT_ADDRESS=", addresses.vault);
        console.log("POSITION_TOKENS_ADDRESS=", addresses.positionTokens);
        console.log("MARKET_RESOLVER_ADDRESS=", addresses.marketResolver);
        console.log("COLLATERAL_TOKEN_ADDRESS=", addresses.collateralToken);
    }

    function logNetworkSpecificInfo() internal view {
        console.log("\n=== NETWORK DEPLOYMENT STATUS ===");
        
        uint256 chainId = block.chainid;
        string memory networkName = getNetworkName();
        
        console.log("Current Network:", networkName);
        console.log("Chain ID:", chainId);
        
        // Check if we have deployed contracts at predicted addresses
        string memory deploymentFile = string.concat("deployments/", networkName, ".json");
        
        try vm.readFile(deploymentFile) returns (string memory) {
            console.log("Deployment exists for this network");
            console.log("File:", deploymentFile);
        } catch {
            console.log("No deployment found for this network");
            console.log("Run deployment with: make deploy-", networkName);
        }
        
        console.log("\nSUPPORTED NETWORKS FOR IDENTICAL ADDRESSES:");
        console.log("- Ethereum Mainnet (chainId: 1)");
        console.log("- Ethereum Sepolia (chainId: 11155111)");
        console.log("- Arbitrum One (chainId: 42161)");
        console.log("- Arbitrum Sepolia (chainId: 421614)");
        console.log("- BSC Mainnet (chainId: 56)");
        console.log("- BSC Testnet (chainId: 97)");
        console.log("- Sonic Mainnet (chainId: 146)");
        console.log("- Sonic Testnet (chainId: 57054)");
        console.log("- Polygon (chainId: 137)");
        console.log("- Base (chainId: 8453)");
        console.log("- Optimism (chainId: 10)");
        console.log("");
        console.log("Addresses will be identical when using the same deployer account.");
    }

    function getNetworkName() internal view returns (string memory) {
        uint256 chainId = block.chainid;

        if (chainId == 1) return "mainnet";
        if (chainId == 11155111) return "sepolia";
        if (chainId == 17000) return "holesky";
        if (chainId == 137) return "polygon";
        if (chainId == 80001) return "mumbai";
        if (chainId == 42161) return "arbitrum";
        if (chainId == 421614) return "arbitrum-sepolia";
        if (chainId == 10) return "optimism";
        if (chainId == 11155420) return "optimism-sepolia";
        if (chainId == 8453) return "base";
        if (chainId == 84532) return "base-sepolia";
        if (chainId == 56) return "bsc";
        if (chainId == 97) return "bsc-testnet";
        if (chainId == 146) return "sonic";
        if (chainId == 57054) return "sonic-testnet";
        if (chainId == 31337) return "anvil";

        return string.concat("chain-", vm.toString(chainId));
    }
}

