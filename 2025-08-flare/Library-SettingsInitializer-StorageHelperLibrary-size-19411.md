
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafePct} from "../../utils/library/SafePct.sol";
import {Globals} from "./Globals.sol";
import {SettingsValidators} from "./SettingsValidators.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";


library SettingsInitializer {

    struct SettingsWrapper {
        AssetManagerSettings.Data settings;
    }

    error CannotBeZero();
    error MustBeZero();
    error WindowTooSmall();
    error ValueTooSmall();
    error BipsValueTooHigh();
    error BipsValueTooLow();
    error MustBeTwoHours();
    error ZeroAddress();
    error MintingCapTooSmall();

    function validateAndSet(
        AssetManagerSettings.Data memory _settings
    )
        internal
    {
        _validateSettings(_settings);
        _setAllSettings(_settings);
    }

    function _setAllSettings(
        AssetManagerSettings.Data memory _settings
    )
        private
    {
        // cannot set value at pointer structure received by Globals.getSettings() due to Solidity limitation,
        // so we need to create wrapper structure at the same address and then set member
        SettingsWrapper storage wrapper;
        bytes32 position = Globals.ASSET_MANAGER_SETTINGS_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            wrapper.slot := position
        }
        wrapper.settings = _settings;
    }

    function _validateSettings(
        AssetManagerSettings.Data memory _settings
    )
        private pure
    {
        require(_settings.fAsset != address(0), ZeroAddress());
        require(_settings.agentVaultFactory != address(0), ZeroAddress());
        require(_settings.collateralPoolFactory != address(0), ZeroAddress());
        require(_settings.collateralPoolTokenFactory != address(0), ZeroAddress());
        require(_settings.fdcVerification != address(0), ZeroAddress());
        require(_settings.priceReader != address(0), ZeroAddress());
        require(_settings.agentOwnerRegistry != address(0), ZeroAddress());

        require(_settings.assetUnitUBA > 0, CannotBeZero());
        require(_settings.assetMintingGranularityUBA > 0, CannotBeZero());
        require(_settings.underlyingBlocksForPayment > 0, CannotBeZero());
        require(_settings.underlyingSecondsForPayment > 0, CannotBeZero());
        require(_settings.redemptionFeeBIPS > 0, CannotBeZero());
        require(_settings.collateralReservationFeeBIPS > 0, CannotBeZero());
        require(_settings.confirmationByOthersRewardUSD5 > 0, CannotBeZero());
        require(_settings.maxRedeemedTickets > 0, CannotBeZero());
        require(_settings.maxTrustedPriceAgeSeconds > 0, CannotBeZero());
        require(_settings.minUpdateRepeatTimeSeconds > 0, CannotBeZero());
        require(_settings.withdrawalWaitMinSeconds > 0, CannotBeZero());
        require(_settings.averageBlockTimeMS > 0, CannotBeZero());
        SettingsValidators.validateTimeForPayment(_settings.underlyingBlocksForPayment,
            _settings.underlyingSecondsForPayment, _settings.averageBlockTimeMS);
        require(_settings.lotSizeAMG > 0, CannotBeZero());
        require(_settings.mintingCapAMG == 0 || _settings.mintingCapAMG >= _settings.lotSizeAMG,
            MintingCapTooSmall());
        require(_settings.collateralReservationFeeBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        require(_settings.redemptionFeeBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        require(_settings.redemptionDefaultFactorVaultCollateralBIPS > SafePct.MAX_BIPS, BipsValueTooLow());
        require(_settings.attestationWindowSeconds >= 1 days, WindowTooSmall());
        require(_settings.confirmationByOthersAfterSeconds >= 2 hours, MustBeTwoHours());
        require(_settings.vaultCollateralBuyForFlareFactorBIPS >= SafePct.MAX_BIPS, ValueTooSmall());
        require(_settings.agentTimelockedOperationWindowSeconds >= 1 hours, ValueTooSmall());
        require(_settings.collateralPoolTokenTimelockSeconds >= 1 minutes, ValueTooSmall());
        require(_settings.liquidationStepSeconds > 0, CannotBeZero());
        SettingsValidators.validateLiquidationFactors(_settings.liquidationCollateralFactorBIPS,
            _settings.liquidationFactorVaultCollateralBIPS);
        // removed settings
        require(_settings.__whitelist == address(0), MustBeZero());
        require(_settings.__ccbTimeSeconds == 0, MustBeZero());
        require(_settings.__announcedUnderlyingConfirmationMinSeconds == 0, MustBeZero());
        require(_settings.__buybackCollateralFactorBIPS == 0, MustBeZero());
        require(_settings.__minUnderlyingBackingBIPS == 0, MustBeZero());
        require(_settings.__redemptionDefaultFactorPoolBIPS == 0, MustBeZero());
        require(_settings.__cancelCollateralReservationAfterSeconds == 0, MustBeZero());
        require(_settings.__rejectOrCancelCollateralReservationReturnFactorBIPS == 0, MustBeZero());
        require(_settings.__rejectRedemptionRequestWindowSeconds == 0, MustBeZero());
        require(_settings.__takeOverRedemptionRequestWindowSeconds == 0, MustBeZero());
        require(_settings.__rejectedRedemptionDefaultFactorVaultCollateralBIPS == 0, MustBeZero());
        require(_settings.__rejectedRedemptionDefaultFactorPoolBIPS == 0, MustBeZero());
    }
}
END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafePct} from "../../utils/library/SafePct.sol";


library SettingsValidators {
    using SafePct for uint256;

    error FactorsNotIncreasing();
    error VaultCollateralFactorHigherThanTotal();
    error FactorNotAboveOne();
    error AtLeastOneFactorRequired();
    error LengthsNotEqual();
    error ValueTooHigh();

    uint256 internal constant MAXIMUM_PROOF_WINDOW = 1 days;

    function validateTimeForPayment(
        uint256 _underlyingBlocks,
        uint256 _underlyingSeconds,
        uint256 _averageBlockTimeMS
    )
        internal pure
    {
        require(_underlyingSeconds <= MAXIMUM_PROOF_WINDOW, ValueTooHigh());
        require(_underlyingBlocks * _averageBlockTimeMS / 1000 <= MAXIMUM_PROOF_WINDOW, ValueTooHigh());
    }

    function validateLiquidationFactors(
        uint256[] memory liquidationFactors,
        uint256[] memory vaultCollateralFactors
    )
        internal pure
    {
        require(liquidationFactors.length == vaultCollateralFactors.length, LengthsNotEqual());
        require(liquidationFactors.length >= 1, AtLeastOneFactorRequired());
        for (uint256 i = 0; i < liquidationFactors.length; i++) {
            // per item validations
            require(liquidationFactors[i] > SafePct.MAX_BIPS, FactorNotAboveOne());
            require(vaultCollateralFactors[i] <= liquidationFactors[i], VaultCollateralFactorHigherThanTotal());
            require(i == 0 || liquidationFactors[i] > liquidationFactors[i - 1], FactorsNotIncreasing());
        }
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


library AssetManagerSettings {
    struct Data {
        // Required contracts.
        // Only used to verify that calls come from assetManagerController.
        // Type: AssetManagerController
        // changed via address updater
        address assetManagerController;

        // The f-asset contract managed by this asset manager.
        // Type: IIFAsset
        // immutable
        address fAsset;

        // Factory for creating new agent vaults.
        // Type: IIAgentVaultFactory
        // timelocked
        address agentVaultFactory;

        // Factory for creating new agent collateral pools.
        // Type: IICollateralPoolFactory
        // timelocked
        address collateralPoolFactory;

        // Factory for creating new agent collateral pool tokens.
        // Type: IICollateralPoolTokenFactory
        // timelocked
        address collateralPoolTokenFactory;

        // The suffix to pool token name and symbol that identifies new vault's collateral pool token.
        // When vault is created, the owner passes own suffix which will be appended to this.
        string poolTokenSuffix;

        // If set, the whitelist contains a list of accounts that can call public methods
        // (minting, redeeming, challenging, etc.)
        // This can be `address(0)`, in which case no whitelist checks are done.
        // Type: IWhitelist
        // timelocked
        address __whitelist; // only storage placeholder

        // If set, the owner address registry contains a list of allowed agent owner's
        // management addresses and mappings from management to work address.
        // Type: IAgentOwnerRegistry
        // timelocked
        address agentOwnerRegistry;

        // Attestation client verifies and decodes attestation proofs.
        // Type: IFdcVerification
        // changed via address updater
        address fdcVerification;

        // The address where burned NAT is sent.
        // immutable
        address payable burnAddress;

        // The contract that reads prices from FTSO system in an FTSO version independent way.
        // Type: IPriceReader
        // timelocked
        address priceReader;

        // Same as assetToken.decimals()
        // immutable
        uint8 assetDecimals;

        // Number of decimals of precision of minted amounts.
        // assetMintingGranularityUBA = 10 ** (assetDecimals - assetMintingDecimals)
        // immutable
        uint8 assetMintingDecimals;

        // Must match attestation data chainId.
        // immutable
        bytes32 chainId;

        // Average time between two successive blocks on the underlying chain, in milliseconds.
        // rate-limited
        uint32 averageBlockTimeMS;

        // The minimum amount of pool tokens the agent must hold to be able to mint.
        // To be able to mint, the NAT value of all backed fassets together with new ones times this percentage
        // must be smaller than the agent's pool tokens' amount converted to NAT.
        // rate-limited
        uint32 mintingPoolHoldingsRequiredBIPS;

        // Collateral reservation fee that must be paid by the minter.
        // Payment is in NAT, but is proportional to the value of assets to be minted.
        // rate-limited
        uint16 collateralReservationFeeBIPS;

        // Asset unit value (e.g. 1 BTC or 1 ETH) in UBA = 10 ** assetToken.decimals()
        // immutable
        uint64 assetUnitUBA;

        // The granularity in which lots are measured = the value of AMG (asset minting granularity) in UBA.
        // Can only be changed via redeploy of AssetManager.
        // AMG is used internally instead of UBA so that minted quantities fit into 64bits to reduce storage.
        // So assetMintingGranularityUBA should be set so that the max supply in AMG of this currency
        // in foreseeable time (say 100yr) cannot overflow 64 bits.
        // immutable
        uint64 assetMintingGranularityUBA;

        // Lot size in asset minting granularity. May change, which affects subsequent mintings and redemptions.
        // timelocked
        uint64 lotSizeAMG;

        // The percentage of minted f-assets that the agent must hold in his underlying address.
        uint16 __minUnderlyingBackingBIPS; // only storage placeholder

        // for some chains (e.g. Ethereum) we require that agent proves that underlying address is an EOA address
        // this must be done by presenting a payment proof from that address
        // immutable
        bool __requireEOAAddressProof; // only storage placeholder

        // Maximum minted amount of the f-asset.
        // rate-limited
        uint64 mintingCapAMG;

        // Number of underlying blocks that the minter or agent is allowed to pay underlying value.
        // If payment not reported in that time, minting/redemption can be challenged and default action triggered.
        // CAREFUL: Count starts from the current proved block height, so the minters and agents should
        // make sure that current block height is fresh, otherwise they might not have enough time for payment.
        // timelocked
        uint64 underlyingBlocksForPayment;

        // Minimum time to allow agent to pay for redemption or minter to pay for minting.
        // This is useful for fast chains, when there can be more than one block per second.
        // Redemption/minting payment failure can be called only after underlyingSecondsForPayment have elapsed
        // on underlying chain.
        // CAREFUL: Count starts from the current proved block timestamp, so the minters and agents should
        // make sure that current block timestamp is fresh, otherwise they might not have enough time for payment.
        // This is partially mitigated by adding local duration since the last block height update to
        // the current underlying block timestamp.
        // timelocked
        uint64 underlyingSecondsForPayment;

        // Redemption fee in underlying currency base amount (UBA).
        // rate-limited
        uint16 redemptionFeeBIPS;

        // On redemption underlying payment failure, redeemer is compensated with
        // redemption value recalculated in flare/sgb times redemption failure factor.
        // Expressed in BIPS, e.g. 12000 for factor of 1.2.
        // This is the part of factor paid from agent's vault collateral.
        // rate-limited
        uint32 redemptionDefaultFactorVaultCollateralBIPS;

        // This is the part of redemption factor paid from agent's pool collateral.
        // rate-limited
        uint32 __redemptionDefaultFactorPoolBIPS; // only storage placeholder

        // If the agent or redeemer becomes unresponsive, we still need payment or non-payment confirmations
        // to be presented eventually to properly track agent's underlying balance.
        // Therefore we allow anybody to confirm payments/non-payments this many seconds after request was made.
        // rate-limited
        uint64 confirmationByOthersAfterSeconds;

        // The user who makes abandoned redemption confirmations gets rewarded by the following amount.
        // rate-limited
        uint128 confirmationByOthersRewardUSD5;

        // To prevent unbounded work, the number of tickets redeemed in a single request is limited.
        // rate-limited
        // >= 1
        uint16 maxRedeemedTickets;

        // Challenge reward can be composed of two part - fixed and proportional (any of them can be zero).
        // This is the proportional part (in BIPS).
        // rate-limited
        uint16 paymentChallengeRewardBIPS;

        // Challenge reward can be composed of two part - fixed and proportional (any of them can be zero).
        // This is the fixed part (in vault collateral token wei).
        // rate-limited
        uint128 paymentChallengeRewardUSD5;

        // Agent has to announce any collateral withdrawal ar vault destroy and then wait for at least
        // withdrawalWaitMinSeconds. This prevents challenged agent to remove all collateral before
        // challenge can be proved.
        // rate-limited
        uint64 withdrawalWaitMinSeconds;

        // Maximum age that trusted price feed is valid.
        // Otherwise (if there were no trusted votes for that long) just use generic ftso price feed.
        // rate-limited
        uint64 maxTrustedPriceAgeSeconds;

        // Agent can remain in CCB for this much time, after that liquidation starts automatically.
        // rate-limited
        uint64 __ccbTimeSeconds; // only storage placeholder

        // Amount of seconds (typically 1 day) that the payment/non-payment proofs must be available.
        // This setting is used in `unstickMinting` and `finishRedemptionWithoutPayment` to prove that the time when
        // payment/non-payment could be proved has already passed.
        // rate-limited
        uint64 attestationWindowSeconds;

        // Minimum time after an update of a setting before the same setting can be updated again.
        // timelocked
        uint64 minUpdateRepeatTimeSeconds;

        // Ratio at which the agents can buy back their collateral when f-asset is terminated.
        // Typically a bit more than 1 to incentivize agents to buy f-assets and self-close instead.
        // immutable
        uint64 __buybackCollateralFactorBIPS; // only storage placeholder

        // Minimum time that has to pass between underlying withdrawal announcement and the confirmation.
        // Any value is ok, but higher values give more security against multiple announcement attack by a miner.
        // Shouldn't be much bigger than Flare data connector response time, so that payments can be confirmed without
        // extra wait. Should be smaller than confirmationByOthersAfterSeconds (e.g. less than 1 hour).
        // rate-limited
        uint64 __announcedUnderlyingConfirmationMinSeconds;

        // Minimum time from the moment token is deprecated to when it becomes invalid and agents still using
        // it as vault collateral get liquidated.
        // timelocked
        uint64 tokenInvalidationTimeMinSeconds;

        // On some rare occasions (stuck minting), the agent has to unlock collateral.
        // For this, part of collateral corresponding to FTSO asset value is burned and the rest is released.
        // However, we cannot burn typical vault collateral (stablecoins), so the agent must buy them for NAT
        // at FTSO price multiplied with this factor (should be a bit above 1) and then we burn the NATs.
        // timelocked
        uint32 vaultCollateralBuyForFlareFactorBIPS;

        // Amount of seconds that have to pass between available list exit announcement and execution.
        // rate-limited
        uint64 agentExitAvailableTimelockSeconds;

        // Amount of seconds that have to pass between agent fee and pool fee share change announcement and execution.
        // rate-limited
        uint64 agentFeeChangeTimelockSeconds;

        // Amount of seconds that have to pass between agent-set minting collateral ratio (vault or pool)
        // change announcement and execution.
        // rate-limited
        uint64 agentMintingCRChangeTimelockSeconds;

        // Amount of seconds that have to pass between agent-set settings for pool exit collateral ratio
        // change announcement and execution.
        // rate-limited
        uint64 poolExitCRChangeTimelockSeconds;

        // Amount of seconds that an agent is allowed to execute an update once it is allowed.
        // rate-limited
        uint64 agentTimelockedOperationWindowSeconds;

        // duration of the timelock for collateral pool tokens after minting
        uint32 collateralPoolTokenTimelockSeconds;

        // If there was no liquidator for the current liquidation offer,
        // go to the next step of liquidation after a certain period of time.
        // rate-limited
        uint64 liquidationStepSeconds;

        // Factor with which to multiply the asset price in native currency to obtain the payment
        // to the liquidator.
        // Expressed in BIPS, e.g. [12000, 16000, 20000] means that the liquidator will be paid 1.2, 1.6 and 2.0
        // times the market price of the liquidated assets after each `liquidationStepSeconds`.
        // Values in the array must increase and be greater than 100%.
        // rate-limited
        uint256[] liquidationCollateralFactorBIPS;

        // How much of the liquidation is paid in vault collateral.
        // The remainder will be paid in pool NAT collateral.
        uint256[] liquidationFactorVaultCollateralBIPS;

        // Minimum time that the system must wait before performing diamond cut.
        // The actual timelock is the maximum of this setting and GovernanceSettings.timelock.
        uint64 diamondCutMinTimelockSeconds;

        // The maximum total pause that can be triggered by non-governance (but governance allowed) caller.
        // The duration count can be reset by the governance.
        uint64 maxEmergencyPauseDurationSeconds;

        // The amount of time since last emergency pause after which the total pause duration counter
        // will reset automatically.
        uint64 emergencyPauseDurationResetAfterSeconds;

        // The amount of time after which the collateral reservation can be cancelled if the
        // handshake is not completed.
        // rate-limited
        uint64 __cancelCollateralReservationAfterSeconds; // only storage placeholder

        // The amount of collateral reservation fee returned to the minter in case of rejection or cancellation.
        // Expressed in BIPS, e.g. 9500 for factor of 0.95, max 10000 for factor of 1.0.
        // rate-limited
        uint16 __rejectOrCancelCollateralReservationReturnFactorBIPS; // only storage placeholder

        // Time window inside which the agent can reject the redemption request.
        // rate-limited
        uint64 __rejectRedemptionRequestWindowSeconds; // only storage placeholder

        // Time window inside which the agent can take over the redemption request from another agent
        // that has rejected it.
        // rate-limited
        uint64 __takeOverRedemptionRequestWindowSeconds; // only storage placeholder

        // On redemption rejection, without take over, redeemer is compensated with
        // redemption value recalculated in flare/sgb times redemption failure factor.
        // Expressed in BIPS, e.g. 12000 for factor of 1.2.
        // This is the part of factor paid from agent's vault collateral.
        // rate-limited
        uint32 __rejectedRedemptionDefaultFactorVaultCollateralBIPS; // only storage placeholder

        // This is the part of rejected redemption factor paid from agent's pool collateral.
        // rate-limited
        uint32 __rejectedRedemptionDefaultFactorPoolBIPS; // only storage placeholder
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {CollateralType} from "../../../userInterfaces/data/CollateralType.sol";

library CollateralTypeInt {
    struct Data {
        // The ERC20 token contract for this collateral type.
        // immutable
        IERC20 token;
        // The kind of collateral for this token.
        // immutable
        CollateralType.Class collateralClass;
        // Same as token.decimals(), when that exists.
        // immutable
        uint8 decimals;
        // If some token should not be used anymore as collateral, it has to be announced in advance and it
        // is still valid until this timestamp. After that time, the corresponding collateral is considered as
        // zero and the agents that haven't replaced it are liquidated.
        // When the invalidation has not been announced, this value is 0.
        uint64 validUntil;
        // When `true`, the FTSO with symbol `assetFtsoSymbol` returns asset price relative to this token
        // (such FTSO's will probably exist for major stablecoins).
        // When `false`, the FTSOs with symbols `assetFtsoSymbol` and `tokenFtsoSymbol` give asset and token
        // price relative to the same reference currency and the asset/token price is calculated as their ratio.
        // immutable
        bool directPricePair;
        // FTSO symbol for the asset, relative to this token or a reference currency
        // (it depends on the value of `directPricePair`).
        // immutable
        string assetFtsoSymbol;
        // FTSO symbol for this token in reference currency.
        // Used for asset/token price calculation when `directPricePair` is `false`.
        // Otherwise it is irrelevant to asset/token price calculation, but if it is nonempty,
        // it is still used in calculation of challenger and confirmation rewards
        // (otherwise we assume it approximates the value of USD and pay directly the USD amount in vault collateral).
        // immutable
        string tokenFtsoSymbol;
        // Minimum collateral ratio for healthy agents.
        // timelocked
        uint32 minCollateralRatioBIPS;
        // Minimum collateral ratio for agent in CCB (Collateral call band).
        // If the agent's collateral ratio is less than this, skip the CCB and go straight to liquidation.
        // A bit smaller than minCollateralRatioBIPS.
        // timelocked
        uint32 __ccbMinCollateralRatioBIPS; // only storage placeholder
        // Minimum collateral ratio required to get agent out of liquidation.
        // Will always be greater than minCollateralRatioBIPS.
        // timelocked
        uint32 safetyMinCollateralRatioBIPS;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

/**
 * Agent owner management and work address management
 */
interface IAgentOwnerRegistry {

    event Whitelisted(address value);
    event WhitelistingRevoked(address value);

    /**
     * Agent owner's work address has been set.
     */
    event WorkAddressChanged(
        address indexed managementAddress,
        address prevWorkAddress,
        address workAddress);

    event AgentDataChanged(
        address indexed managementAddress,
        string name,
        string description,
        string iconUrl,
        string termsOfUseUrl);

    error AgentNotWhitelisted();
    error WorkAddressInUse();


    /**
     * Returns true if the address is whitelisted, false otherwise.
     * @param _address address to check
     */
    function isWhitelisted(address _address) external view returns (bool);

    /**
     * Return agent owner's name.
     * @param _managementAddress agent owner's management address
     */
    function getAgentName(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Return agent owner's description.
     * @param _managementAddress agent owner's management address
     */
    function getAgentDescription(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Return url of the agent owner's icon.
     * @param _managementAddress agent owner's management address
     */
    function getAgentIconUrl(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Return url of the agent's page with terms of use.
     * @param _managementAddress agent owner's management address
     */
    function getAgentTermsOfUseUrl(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Get the (unique) work address for the given management address.
     */
    function getWorkAddress(address _managementAddress)
        external view
        returns (address);

    /**
     * Get the (unique) management address for the given work address.
     */
    function getManagementAddress(address _workAddress)
        external view
        returns (address);
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
pragma solidity >=0.7.6 <0.9;

import {IICleanable} from "@flarenetwork/flare-periphery-contracts/flare/token/interfaces/IICleanable.sol";
import {IFAsset} from "../../userInterfaces/IFAsset.sol";
import {IICheckPointable} from "./IICheckPointable.sol";


interface IIFAsset is IFAsset, IICheckPointable, IICleanable {
    /**
     * Mints `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `mint()`.
     */
    function mint(address _owner, uint256 _amount) external;

    /**
     * Burns `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `burn()`.
     */
    function burn(address _owner, uint256 _amount) external;

    /**
     * Set the contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function setCleanupBlockNumberManager(address _cleanupBlockNumberManager) external;

    /**
     * The contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function cleanupBlockNumberManager() external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IIFAsset} from "../../fassetToken/interfaces/IIFAsset.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAgentOwnerRegistry} from "../../userInterfaces/IAgentOwnerRegistry.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";


// global state helpers
library Globals {
    bytes32 internal constant ASSET_MANAGER_SETTINGS_POSITION = keccak256("fasset.AssetManager.Settings");

    function getSettings()
        internal pure
        returns (AssetManagerSettings.Data storage _settings)
    {
        bytes32 position = ASSET_MANAGER_SETTINGS_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _settings.slot := position
        }
    }

    function getWNat()
        internal view
        returns (IWNat)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return IWNat(address(state.collateralTokens[state.poolCollateralIndex].token));
    }

    function getPoolCollateral()
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[state.poolCollateralIndex];
    }

    function getFAsset()
        internal view
        returns (IIFAsset)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return IIFAsset(settings.fAsset);
    }

    function getAgentOwnerRegistry()
        internal view
        returns (IAgentOwnerRegistry)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return IAgentOwnerRegistry(settings.agentOwnerRegistry);
    }

    function getBurnAddress()
        internal view
        returns (address payable)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return settings.burnAddress;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {RedemptionQueue} from "./RedemptionQueue.sol";
import {PaymentConfirmations} from "./PaymentConfirmations.sol";
import {UnderlyingAddressOwnership} from "./UnderlyingAddressOwnership.sol";
import {CollateralReservation} from "./CollateralReservation.sol";
import {Redemption} from "./Redemption.sol";
import {CollateralTypeInt} from "./CollateralTypeInt.sol";


library AssetManagerState {
    struct State {
        // All collateral types, used for vault or pool.
        // Pool collateral (always WNat) has index 0.
        CollateralTypeInt.Data[] collateralTokens;

        // mapping((collateralClass, tokenAddress) => collateralTokens index + 1)
        mapping(bytes32 => uint256) collateralTokenIndex;

        // makes sure pool tokens have unique names and symbols
        mapping(string => bool) reservedPoolTokenSuffixes;

        // A list of all agents (for use by monitoring or challengers).
        // Type: array of agent vault addresses; when one is deleted, its position is filled with last
        address[] allAgents;

        // A list of all agents that are available for minting.
        // Type: array of agent vault addresses; when one is deleted, its position is filled with last
        address[] availableAgents;

        // Ownership of underlying source addresses is needed to prevent someone
        // overtaking the payer and presenting an underlying payment as his own.
        UnderlyingAddressOwnership.State underlyingAddressOwnership;

        // Type: mapping collateralReservationId => collateralReservation
        mapping(uint256 => CollateralReservation.Data) crts;

        // redemption queue
        RedemptionQueue.State redemptionQueue;

        // mapping redemptionRequest_id => request
        mapping(uint256 => Redemption.Request) redemptionRequests;

        // verified payment hashes; expire in 5 days
        PaymentConfirmations.State paymentConfirmations;

        // New ids (listed together to save storage); all must be incremented before assigning, so 0 means empty
        uint64 newCrtId;
        uint64 newRedemptionRequestId;
        uint64 newPaymentAnnouncementId;

        // Total collateral reservations (in underlying AMG units). Used by minting cap.
        uint64 totalReservedCollateralAMG;

        // Pool collateral is always wrapped NAT, but the wrapping contract may change.
        // In this case, new pool collateral token must be added and set as current.
        uint16 poolCollateralIndex;

        // Current block number and timestamp on the underlying chain
        uint64 currentUnderlyingBlock;
        uint64 currentUnderlyingBlockTimestamp;

        // The timestamp (on this network) when the underlying block was last updated
        uint64 currentUnderlyingBlockUpdatedAt;

        // If non-zero, minting is paused and has been paused at the time indicated by timestamp mintingPausedAt.
        // When asset manager is paused, no new mintings can be done, but redemptions still work.
        uint64 mintingPausedAt;

        // If non-zero, asset manager is paused and will be paused until the time indicated.
        // When asset manager is paused, all dangerous operations ar blocked (mintings, redemptions, etc.).
        // It is an extreme measure, which can be used in case there is a dangerous hole in the system.
        uint64 emergencyPausedUntil;

        // When emergency pause is not done by governance, the total allowed pause is limited.
        // So the caller must state the duration after which the pause will automatically end.
        // When total pauses exceed the max allowed length, pausing is only allowed by the governance.
        // An emergencyPause call by the governance optionally resets the total duration counter to 0.
        uint64 emergencyPausedTotalDuration;

        // When emergency pause was triggered by governance, only governance can unpause.
        bool emergencyPausedByGovernance;

        // If non-zero, asset manager is paused and will be paused until the time indicated.
        // When asset manager is paused, all dangerous operations ar blocked (mintings, redemptions, etc.).
        // It is an extreme measure, which can be used in case there is a dangerous hole in the system.
        uint64 transfersEmergencyPausedUntil;

        // When emergency pause is not done by governance, the total allowed pause is limited.
        // So the caller must state the duration after which the pause will automatically end.
        // When total pauses exceed the max allowed length, pausing is only allowed by the governance.
        // An emergencyPause call by the governance optionally resets the total duration counter to 0.
        uint64 transfersEmergencyPausedTotalDuration;

        // When emergency pause was triggered by governance, only governance can unpause.
        bool transfersEmergencyPausedByGovernance;

        // When true, asset manager has been added to the asset manager controller.
        // Even though the asset manager controller address is set at the construction time, the manager may not
        // be able to be added to the controller immediately because the method addAssetManager must be called
        // by the governance multisig (with timelock).
        // During this time it is impossible to verify through the controller that the asset manager is legit.
        // Therefore creating agents and minting is disabled until the asset manager controller notifies
        // the asset manager that it has been added.
        bool attached;
    }

    // diamond state access to state and settings

    bytes32 internal constant STATE_POSITION = keccak256("fasset.AssetManager.State");

    function get() internal pure returns (AssetManagerState.State storage _state) {
        // Only direct constants are allowed in inline assembly, so we assign it here
        bytes32 position = STATE_POSITION;
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

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IAgentOwnerRegistry} from "../../userInterfaces/IAgentOwnerRegistry.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {GovernedUUPSProxyImplementation} from "../../governance/implementation/GovernedUUPSProxyImplementation.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";


contract AgentOwnerRegistry is GovernedUUPSProxyImplementation, IERC165, IAgentOwnerRegistry {

    event ManagerChanged(address manager);

    error AddressZero();
    error OnlyGovernanceOrManager();

    /**
     * When nonzero, this is the address that can perform whitelisting operations
     * instead of the governance.
     */
    address public manager;

    mapping(address => bool) private whitelist;

    mapping(address => address) private workToMgmtAddress;
    mapping(address => address) private mgmtToWorkAddress;

    mapping(address => string) private agentName;
    mapping(address => string) private agentDescription;
    mapping(address => string) private agentIconUrl;
    mapping(address => string) private agentTouUrl;

    modifier onlyGovernanceOrManager {
        require(msg.sender == manager || msg.sender == governance(), OnlyGovernanceOrManager());
        _;
    }

    function initialize(IGovernanceSettings _governanceSettings, address _initialGovernance) external {
        initialise(_governanceSettings, _initialGovernance);    // also marks as initialized
    }

    function revokeAddress(address _address) external onlyGovernanceOrManager {
        _removeAddressFromWhitelist(_address);
    }

    function setManager(address _manager) external onlyGovernance {
        manager = _manager;
        emit ManagerChanged(_manager);
    }

    /**
     * Add agent to the whitelist and set data for agent presentation.
     * If the agent is already whitelisted, only updates agent presentation data.
     * @param _managementAddress the agent owner's address
     * @param _name agent owner's name
     * @param _description agent owner's description
     * @param _iconUrl url of the agent owner's icon image; governance or manager should check it is in correct format
     *      and size and it is on a server where it cannot change or be deleted
     * @param _touUrl url of the agent's page with terms of use; similar considerations apply as for icon url
     */
    function whitelistAndDescribeAgent(
        address _managementAddress,
        string memory _name,
        string memory _description,
        string memory _iconUrl,
        string memory _touUrl
    )
        external
        onlyGovernanceOrManager
    {
        _addAddressToWhitelist(_managementAddress);
        _setAgentData(_managementAddress, _name, _description, _iconUrl, _touUrl);
    }

    /**
     * Associate a work address with the agent owner's management address.
     * Every owner (management address) can have only one work address, so as soon as the new one is set, the old
     * one stops working.
     * NOTE: May only be called by an agent on the allowed agent list and only from the management address.
     */
    function setWorkAddress(address _ownerWorkAddress)
        external
    {
        require(isWhitelisted(msg.sender), AgentNotWhitelisted());
        require(_ownerWorkAddress == address(0) || workToMgmtAddress[_ownerWorkAddress] == address(0),
               WorkAddressInUse());
        // delete old work to management mapping
        address oldWorkAddress = mgmtToWorkAddress[msg.sender];
        if (oldWorkAddress != address(0)) {
            workToMgmtAddress[oldWorkAddress] = address(0);
        }
        // create a new bidirectional mapping
        mgmtToWorkAddress[msg.sender] = _ownerWorkAddress;
        if (_ownerWorkAddress != address(0)) {
            workToMgmtAddress[_ownerWorkAddress] = msg.sender;
        }
        emit WorkAddressChanged(msg.sender, oldWorkAddress, _ownerWorkAddress);
    }

    /**
     * Set agent owner's name.
     * @param _managementAddress agent owner's management address
     * @param _name new agent owner's name
     */
    function setAgentName(address _managementAddress, string memory _name)
        external
        onlyGovernanceOrManager
    {
        agentName[_managementAddress] = _name;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Set agent owner's description.
     * @param _managementAddress agent owner's management address
     * @param _description new agent owner's description
     */
    function setAgentDescription(address _managementAddress, string memory _description)
        external
        onlyGovernanceOrManager
    {
        agentDescription[_managementAddress] = _description;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Set url of the agent owner's icon.
     * @param _managementAddress agent owner's management address
     * @param _iconUrl new url of the agent owner's icon
     */
    function setAgentIconUrl(address _managementAddress, string memory _iconUrl)
        external
        onlyGovernanceOrManager
    {
        agentIconUrl[_managementAddress] = _iconUrl;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Set url of the agent's page with terms of use.
     * @param _managementAddress agent owner's management address
     * @param _touUrl new url of the agent's page with terms of use
     */
    function setAgentTermsOfUseUrl(address _managementAddress, string memory _touUrl)
        external
        onlyGovernanceOrManager
    {
        agentTouUrl[_managementAddress] = _touUrl;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Return agent owner's name.
     * @param _managementAddress agent owner's management address
     */
    function getAgentName(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentName[_managementAddress];
    }

    /**
     * Return agent owner's description.
     * @param _managementAddress agent owner's management address
     */
    function getAgentDescription(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentDescription[_managementAddress];
    }

    /**
     * Return url of the agent owner's icon.
     * @param _managementAddress agent owner's management address
     */
    function getAgentIconUrl(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentIconUrl[_managementAddress];
    }

    /**
     * Return url of the agent's page with terms of use.
     * @param _managementAddress agent owner's management address
     */
    function getAgentTermsOfUseUrl(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentTouUrl[_managementAddress];
    }

    /**
     * Get the (unique) work address for the given management address.
     */
    function getWorkAddress(address _managementAddress)
        external view override
        returns (address)
    {
        return mgmtToWorkAddress[_managementAddress];
    }

    /**
     * Get the (unique) management address for the given work address.
     */
    function getManagementAddress(address _workAddress)
        external view override
        returns (address)
    {
        return workToMgmtAddress[_workAddress];
    }

    function isWhitelisted(address _address) public view override returns (bool) {
        return whitelist[_address];
    }

    function _addAddressToWhitelist(address _address) internal {
        require(_address != address(0), AddressZero());
        if (whitelist[_address]) return;
        whitelist[_address] = true;
        emit Whitelisted(_address);
    }

    function _removeAddressFromWhitelist(address _address) internal {
        if (!whitelist[_address]) return;
        delete whitelist[_address];
        emit WhitelistingRevoked(_address);
    }

    function _setAgentData(
        address _managementAddress,
        string memory _name,
        string memory _description,
        string memory _iconUrl,
        string memory _touUrl
    ) private {
        agentName[_managementAddress] = _name;
        agentDescription[_managementAddress] = _description;
        agentIconUrl[_managementAddress] = _iconUrl;
        agentTouUrl[_managementAddress] = _touUrl;
        emit AgentDataChanged(_managementAddress, _name, _description, _iconUrl, _touUrl);
    }

    function _emitDataChanged(address _managementAddress) private {
        emit AgentDataChanged(_managementAddress,
            agentName[_managementAddress],
            agentDescription[_managementAddress],
            agentIconUrl[_managementAddress],
            agentTouUrl[_managementAddress]);
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        public pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IAgentOwnerRegistry).interfaceId;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IDiamondCut} from "../../diamond/interfaces/IDiamondCut.sol";
import {IGoverned} from "../../governance/interfaces/IGoverned.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {IISettingsManagement} from "./IISettingsManagement.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";


/**
 * Asset Manager methods used internally in AgentVault, CollateralPool and AssetManagerController.
 */
interface IIAssetManager is IAssetManager, IGoverned, IDiamondCut, IISettingsManagement {
    ////////////////////////////////////////////////////////////////////////////////////
    // Settings update

    /**
     * When `attached` is true, asset manager has been added to the asset manager controller.
     * Even though the asset manager controller address is set at the construction time, the manager may not
     * be able to be added to the controller immediately because the method addAssetManager must be called
     * by the governance multisig (with timelock). During this time it is impossible to verify through the
     * controller that the asset manager is legit.
     * Therefore creating agents and minting is disabled until the asset manager controller notifies
     * the asset manager that it has been added.
     * The `attached` can be set to false when the retired asset manager is removed from the controller.
     * NOTE: this method will be called automatically when the asset manager is added to a controller
     *      and cannot be called directly.
     */
    function attachController(bool attached) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause

    /**
     * Trigger pause of most operations.
     */
    function emergencyPause(bool _byGovernance, uint256 _duration)
        external;

    /**
     * Reset total duration of 3rd party pauses, so that they can trigger pause again.
     * Otherwise, the total duration is automatically reset emergencyPauseDurationResetAfterSeconds after last pause.
     */
    function resetEmergencyPauseTotalDuration()
        external;

    /**
     * Emergency pause details, useful for monitors.
     */
    function emergencyPauseDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance);

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency transfer pause

    /**
     * Trigger pause of most operations.
     */
    function emergencyPauseTransfers(bool _byGovernance, uint256 _duration)
        external;

    /**
     * Reset total duration of 3rd party pauses, so that they can trigger pause again.
     * Otherwise, the total duration is automatically reset emergencyPauseDurationResetAfterSeconds after last pause.
     */
    function resetEmergencyPauseTransfersTotalDuration()
        external;

    /**
     * Emergency pause details, useful for monitors.
     */
    function emergencyPauseTransfersDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance);

    ////////////////////////////////////////////////////////////////////////////////////
    // Upgrade

    /**
     * When asset manager is paused, no new minting can be made.
     * All other operations continue normally.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function pauseMinting() external;

    /**
     * Minting can continue.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function unpauseMinting() external;

    /**
     * When agent vault, collateral pool or collateral pool token factory is upgraded, new agent vaults
     * automatically get the new implementation from the factory. The existing vaults can be batch updated
     * by this method.
     * Parameters `_start` and `_end` allow limiting the upgrades to a selection of all agents, to avoid
     * breaking the block gas limit.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     * @param _start the start index of the list of agent vaults (in getAllAgents()) to upgrade
     * @param _end the end index (exclusive) of the list of agent vaults to upgrade;
     *  can be larger then the number of agents, if gas is not an issue
     */
    function upgradeAgentVaultsAndPools(
        uint256 _start,
        uint256 _end
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Collateral type management

    /**
     * Add new vault collateral type (new token type and initial collateral ratios).
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function addCollateralType(
        CollateralType.Data calldata _data
    ) external;

    /**
     * Update collateral ratios for collateral type identified by `_collateralClass` and `_token`.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function setCollateralRatiosForToken(
        CollateralType.Class _collateralClass,
        IERC20 _token,
        uint256 _minCollateralRatioBIPS,
        uint256 _safetyMinCollateralRatioBIPS
    ) external;

    /**
     * Deprecate collateral type identified by `_collateralClass` and `_token`.
     * After `_invalidationTimeSec` the collateral will become invalid and all the agents
     * that still use it as collateral will be liquidated.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function deprecateCollateralType(
        CollateralType.Class _collateralClass,
        IERC20 _token,
        uint256 _invalidationTimeSec
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Collateral pool redemptions

    /**
     * Create a redemption from a single agent. Used in self-close exit from the collateral pool.
     * NOTE: only collateral pool can call this method.
     */
    function redeemFromAgent(
        address _agentVault,
        address _receiver,
        uint256 _amountUBA,
        string memory _receiverUnderlyingAddress,
        address payable _executor
    ) external payable;

    /**
     * Burn fassets from  a single agent and get paid in vault collateral by the agent.
     * Price is FTSO price, multiplied by factor buyFAssetByAgentFactorBIPS (set by agent).
     * Used in self-close exit from the collateral pool when requested or when self-close amount is less than 1 lot.
     * NOTE: only collateral pool can call this method.
     */
    function redeemFromAgentInCollateral(
        address _agentVault,
        address _receiver,
        uint256 _amountUBA
    ) external;

    /**
     * To avoid unlimited work, the maximum number of redemption tickets closed in redemption, self close
     * or liquidation is limited. This means that a single redemption/self close/liquidation is limited.
     * This function calculates the maximum single redemption amount.
     */
    function maxRedemptionFromAgent(address _agentVault)
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // Functions, used by agent vault during collateral deposit/withdraw

    /**
     * Called by AgentVault when agent calls `withdraw()`.
     * NOTE: may only be called from an agent vault, not from an EOA address.
     * @param _valueNATWei the withdrawn amount
     */
    function beforeCollateralWithdrawal(
        IERC20 _token,
        uint256 _valueNATWei
    ) external;

    /**
     * Called by AgentVault when there was a deposit.
     * May pull agent out of liquidation.
     * NOTE: may only be called from an agent vault or collateral pool, not from an EOA address.
     */
    function updateCollateral(
        address _agentVault,
        IERC20 _token
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // View functions used internally by agent vault and collateral pool.

    /**
     * Get current WNat contract set in the asset manager.
     * Used internally by agent vault and collateral pool.
     * @return WNat contract
     */
    function getWNat()
        external view
        returns (IWNat);

    /**
     * Returns price of asset (UBA) in NAT Wei as a fraction.
     * Used internally by collateral pool.
     */
    function assetPriceNatWei()
        external view
        returns (uint256 _multiplier, uint256 _divisor);

    /**
     * Returns the number of f-assets that the agent's pool identified by `_agentVault` is backing.
     * This is the same as the number of f-assets the agent is backing, but excluding
     * f-assets being redeemed by pool self-close redemptions.
     * Used internally by collateral pool.
     */
    function getFAssetsBackedByPool(address _agentVault)
        external view
        returns (uint256);

    /**
     * Returns the duration for which the collateral pool tokens are timelocked after minting.
     * Timelocking is done to battle sandwich attacks aimed at stealing newly deposited f-asset
     * fees from the pool.
     */
    function getCollateralPoolTokenTimelockSeconds()
        external view
        returns (uint256);

    /**
     * Check if `_token` is either vault collateral token for `_agentVault` or the pool token.
     * These types of tokens cannot be simply transferred from the agent vault, but can only be
     * withdrawn after announcement if they are not backing any f-assets.
     * Used internally by agent vault.
     */
    function isLockedVaultToken(address _agentVault, IERC20 _token)
        external view
        returns (bool);

    /**
     * Check if `_token` is any of the vault collateral tokens (including already invalidated).
     */
    function isVaultCollateralToken(IERC20 _token)
        external view
        returns (bool);

    /**
     * True if `_address` is either work or management address of the owner of the agent identified by `_agentVault`.
     * Used internally by agent vault.
     */
    function isAgentVaultOwner(address _agentVault, address _address)
        external view
        returns (bool);

    /**
     * Return the work address for the given management address.
     */
    function getWorkAddress(address _managementAddress)
        external view
        returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IICheckPointable} from "../interfaces/IICheckPointable.sol";
import {CheckPointHistory} from "../library/CheckPointHistory.sol";
import {CheckPointsByAddress} from "../library/CheckPointsByAddress.sol";

/**
 * @title Check Pointable ERC20 Behavior
 * @notice ERC20 behavior which adds balance check point features.
 **/
abstract contract CheckPointable is IICheckPointable {
    error CheckPointableReadingFromCleanedupBlock();
    error OnlyCleanerContract();
    error CleanupBlockNumberMustNeverDecrease();
    error CleanupBlockMustBeInThePast();

    using CheckPointHistory for CheckPointHistory.CheckPointHistoryState;
    using CheckPointsByAddress for CheckPointsByAddress.CheckPointsByAddressState;

    // The number of history cleanup steps executed for every write operation.
    // It is more than 1 to make as certain as possible that all history gets cleaned eventually.
    uint256 private constant CLEANUP_COUNT = 2;

    // Private member variables
    CheckPointsByAddress.CheckPointsByAddressState private balanceHistory;
    CheckPointHistory.CheckPointHistoryState private totalSupply;

    // Historic data for the blocks before `cleanupBlockNumber` can be erased,
    // history before that block should never be used since it can be inconsistent.
    uint256 private cleanupBlockNumber;

    // Address of the contract that is allowed to call methods for history cleaning.
    address public cleanerContract;

    /**
     * Emitted when a total supply cache entry is created.
     * Allows history cleaners to track total supply cache cleanup opportunities off-chain.
     */
    event CreatedTotalSupplyCache(uint256 _blockNumber);

    // Most cleanup opportunities can be deduced from standard event
    // Transfer(from, to, amount):
    //   - balance history for `from` (if nonzero) and `to` (if nonzero)
    //   - total supply history when either `from` or `to` is zero

    modifier notBeforeCleanupBlock(uint256 _blockNumber) {
        require(_blockNumber >= cleanupBlockNumber, CheckPointableReadingFromCleanedupBlock());
        _;
    }

    modifier onlyCleaner {
        require(msg.sender == cleanerContract, OnlyCleanerContract());
        _;
    }

    /**
     * @dev Queries the token balance of `_owner` at a specific `_blockNumber`.
     * @param _owner The address from which the balance will be retrieved.
     * @param _blockNumber The block number when the balance is queried.
     * @return _balance The balance at `_blockNumber`.
     **/
    function balanceOfAt(address _owner, uint256 _blockNumber)
        public virtual view
        notBeforeCleanupBlock(_blockNumber)
        returns (uint256 _balance)
    {
        return balanceHistory.valueOfAt(_owner, _blockNumber);
    }

    /**
     * @notice Burn current token `amount` for `owner` of checkpoints at current block.
     * @param _owner The address of the owner to burn tokens.
     * @param _amount The amount to burn.
     */
    function _burnForAtNow(address _owner, uint256 _amount) internal virtual {
        uint256 newBalance = balanceOfAt(_owner, block.number) - _amount;
        balanceHistory.writeValue(_owner, newBalance);
        balanceHistory.cleanupOldCheckpoints(_owner, CLEANUP_COUNT, cleanupBlockNumber);
        totalSupply.writeValue(totalSupplyAt(block.number) - _amount);
        totalSupply.cleanupOldCheckpoints(CLEANUP_COUNT, cleanupBlockNumber);
    }

    /**
     * @notice Mint current token `amount` for `owner` of checkpoints at current block.
     * @param _owner The address of the owner to burn tokens.
     * @param _amount The amount to burn.
     */
    function _mintForAtNow(address _owner, uint256 _amount) internal virtual {
        uint256 newBalance = balanceOfAt(_owner, block.number) + _amount;
        balanceHistory.writeValue(_owner, newBalance);
        balanceHistory.cleanupOldCheckpoints(_owner, CLEANUP_COUNT, cleanupBlockNumber);
        totalSupply.writeValue(totalSupplyAt(block.number) + _amount);
        totalSupply.cleanupOldCheckpoints(CLEANUP_COUNT, cleanupBlockNumber);
    }

    /**
     * @notice Total amount of tokens at a specific `_blockNumber`.
     * @param _blockNumber The block number when the _totalSupply is queried
     * @return _totalSupply The total amount of tokens at `_blockNumber`
     **/
    function totalSupplyAt(uint256 _blockNumber)
        public virtual view
        notBeforeCleanupBlock(_blockNumber)
        returns(uint256 _totalSupply)
    {
        return totalSupply.valueAt(_blockNumber);
    }

    /**
     * @notice Transmit token `_amount` `_from` address `_to` address of checkpoints at current block.
     * @param _from The address of the sender.
     * @param _to The address of the receiver.
     * @param _amount The amount to transmit.
     */
    function _transmitAtNow(address _from, address _to, uint256 _amount) internal virtual {
        balanceHistory.transmit(_from, _to, _amount);
        balanceHistory.cleanupOldCheckpoints(_from, CLEANUP_COUNT, cleanupBlockNumber);
        balanceHistory.cleanupOldCheckpoints(_to, CLEANUP_COUNT, cleanupBlockNumber);
    }

    /**
     * Set the cleanup block number.
     */
    function _setCleanupBlockNumber(uint256 _blockNumber) internal {
        require(_blockNumber >= cleanupBlockNumber, CleanupBlockNumberMustNeverDecrease());
        require(_blockNumber < block.number, CleanupBlockMustBeInThePast());
        cleanupBlockNumber = _blockNumber;
    }

    /**
     * Get the cleanup block number.
     */
    function _cleanupBlockNumber() internal view returns (uint256) {
        return cleanupBlockNumber;
    }

    /**
     * @notice Update history at token transfer, the CheckPointable part of `_beforeTokenTransfer` hook.
     * @param _from The address of the sender.
     * @param _to The address of the receiver.
     * @param _amount The amount to transmit.
     */
    function _updateBalanceHistoryAtTransfer(address _from, address _to, uint256 _amount) internal virtual {
        if (_from == address(0)) {
            // mint checkpoint balance data for transferee
            _mintForAtNow(_to, _amount);
        } else if (_to == address(0)) {
            // burn checkpoint data for transferer
            _burnForAtNow(_from, _amount);
        } else {
            // transfer checkpoint balance data
            _transmitAtNow(_from, _to, _amount);
        }
    }

    // history cleanup methods

    /**
     * Set the contract that is allowed to call history cleaning methods.
     */
    function _setCleanerContract(address _cleanerContract) internal {
        cleanerContract = _cleanerContract;
    }

    /**
     * Delete balance checkpoints that expired (i.e. are before `cleanupBlockNumber`).
     * Method can only be called from the `cleanerContract` (which may be a proxy to external cleaners).
     * @param _owner balance owner account address
     * @param _count maximum number of checkpoints to delete
     * @return the number of checkpoints deleted
     */
    function balanceHistoryCleanup(address _owner, uint256 _count) external onlyCleaner returns (uint256) {
        return balanceHistory.cleanupOldCheckpoints(_owner, _count, cleanupBlockNumber);
    }

    /**
     * Delete total supply checkpoints that expired (i.e. are before `cleanupBlockNumber`).
     * Method can only be called from the `cleanerContract` (which may be a proxy to external cleaners).
     * @param _count maximum number of checkpoints to delete
     * @return the number of checkpoints deleted
     */
    function totalSupplyHistoryCleanup(uint256 _count) external onlyCleaner returns (uint256) {
        return totalSupply.cleanupOldCheckpoints(_count, cleanupBlockNumber);
    }
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
pragma solidity ^0.8.27;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IERC5267} from "@openzeppelin/contracts/interfaces/IERC5267.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IIFAsset} from "../interfaces/IIFAsset.sol";
import {ERC20Permit} from "../../openzeppelin/token/ERC20Permit.sol";
import {CheckPointable} from "./CheckPointable.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IICleanable} from "@flarenetwork/flare-periphery-contracts/flare/token/interfaces/IICleanable.sol";
import {IFAsset} from "../../userInterfaces/IFAsset.sol";
import {IICheckPointable} from "../interfaces/IICheckPointable.sol";


contract FAsset is IIFAsset, IERC165, ERC20, CheckPointable, UUPSUpgradeable, ERC20Permit {
    error OnlyAssetManager();
    error AlreadyInitialized();
    error AlreadyUpgraded();
    error OnlyDeployer();
    error ZeroAssetManager();
    error CannotReplaceAssetManager();
    error OnlyCleanupBlockManager();
    error FAssetTerminated();
    error FAssetBalanceTooLow();
    error CannotTransferToSelf();
    error EmergencyPauseOfTransfersActive();

    /**
     * The name of the underlying asset.
     */
    string public override assetName;

    /**
     * The symbol of the underlying asset.
     */
    string public override assetSymbol;

    /**
     * The contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    address public cleanupBlockNumberManager;

    /**
     * Get the asset manager, corresponding to this fAsset.
     * fAssets and asset managers are in 1:1 correspondence.
     */
    address public override assetManager;

    uint64 private __terminatedAt; // only storage placeholder

    string private _name;
    string private _symbol;
    uint8 private _decimals;

    // the address that created this contract and is allowed to set initial settings
    address private _deployer;
    bool private _initialized;
    uint16 private _version;

    modifier onlyAssetManager() {
        require(msg.sender == assetManager, OnlyAssetManager());
        _;
    }

    constructor()
        ERC20("", "")
    {
        _initialized = true;
        _version = 1000;
    }

    function initialize(
        string memory name_,
        string memory symbol_,
        string memory assetName_,
        string memory assetSymbol_,
        uint8 decimals_
    )
        external
    {
        require(!_initialized, AlreadyInitialized());
        _initialized = true;
        _deployer = msg.sender;
        _name = name_;
        _symbol = symbol_;
        _decimals = decimals_;
        assetName = assetName_;
        assetSymbol = assetSymbol_;
        initializeV1r1();
    }

    function initializeV1r1() public {
        require(_version == 0, AlreadyUpgraded());
        _version = 1;
        initializeEIP712(_name, "1");
    }

    /**
     * Set asset manager contract this can be done only once and must be just after deploy
     * (otherwise nothing can be minted).
     */
    function setAssetManager(address _assetManager)
        external
    {
        require (msg.sender == _deployer, OnlyDeployer());
        require(_assetManager != address(0), ZeroAssetManager());
        require(assetManager == address(0), CannotReplaceAssetManager());
        assetManager = _assetManager;
    }

    /**
     * Mints `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `mint()`.
     */
    function mint(address _owner, uint256 _amount)
        external override
        onlyAssetManager
    {
        _mint(_owner, _amount);
    }

    /**
     * Burns `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `burn()`.
     */
    function burn(address _owner, uint256 _amount)
        external override
        onlyAssetManager
    {
        _burn(_owner, _amount);
    }

    /**
     * Returns the name of the token.
     */
    function name() public view virtual override(ERC20, IERC20Metadata) returns (string memory) {
        return _name;
    }

    /**
     * Returns the symbol of the token, usually a shorter version of the name.
     */
    function symbol() public view virtual override(ERC20, IERC20Metadata) returns (string memory) {
        return _symbol;
    }
    /**
     * Implements IERC20Metadata method and returns configurable number of decimals.
     */
    function decimals() public view virtual override(ERC20, IERC20Metadata) returns (uint8) {
        return _decimals;
    }

    /**
     * Set the cleanup block number.
     * Historic data for the blocks before `cleanupBlockNumber` can be erased,
     * history before that block should never be used since it can be inconsistent.
     * In particular, cleanup block number must be before current vote power block.
     * @param _blockNumber The new cleanup block number.
     */
    function setCleanupBlockNumber(uint256 _blockNumber)
        external override
    {
        require(msg.sender == cleanupBlockNumberManager, OnlyCleanupBlockManager());
        _setCleanupBlockNumber(_blockNumber);
    }

    /**
     * Get the current cleanup block number.
     */
    function cleanupBlockNumber()
        external view override
        returns (uint256)
    {
        return _cleanupBlockNumber();
    }

    /**
     * Set the contract that is allowed to call history cleaning methods.
     */
    function setCleanerContract(address _cleanerContract)
        external override
        onlyAssetManager
    {
        _setCleanerContract(_cleanerContract);
    }

    /**
     * Set the contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function setCleanupBlockNumberManager(address _cleanupBlockNumberManager)
        external
        onlyAssetManager
    {
        cleanupBlockNumberManager = _cleanupBlockNumberManager;
    }

    function _beforeTokenTransfer(address _from, address _to, uint256 _amount)
        internal override
    {
        require(_from == address(0) || balanceOf(_from) >= _amount, FAssetBalanceTooLow());
        require(_from != _to, CannotTransferToSelf());
        // mint and redeem are allowed on transfer pause, but not transfer
        require(_from == address(0) || _to == address(0) || !IAssetManager(assetManager).transfersEmergencyPaused(),
            EmergencyPauseOfTransfersActive());
        // update balance history
        _updateBalanceHistoryAtTransfer(_from, _to, _amount);
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IERC20).interfaceId
            || _interfaceId == type(IERC20Metadata).interfaceId
            || _interfaceId == type(IERC5267).interfaceId
            || _interfaceId == type(IERC20Permit).interfaceId
            || _interfaceId == type(IICheckPointable).interfaceId
            || _interfaceId == type(IFAsset).interfaceId
            || _interfaceId == type(IIFAsset).interfaceId
            || _interfaceId == type(IICleanable).interfaceId;
    }

    // support for ERC20Permit
    function _approve(address _owner, address _spender, uint256 _amount)
        internal virtual override (ERC20, ERC20Permit)
    {
        ERC20._approve(_owner, _spender, _amount);
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // UUPS proxy upgrade

    function implementation() external view returns (address) {
        return _getImplementation();
    }

    /**
     * Upgrade calls can only arrive through asset manager.
     * See UUPSUpgradeable._authorizeUpgrade.
     */
    function _authorizeUpgrade(address /* _newImplementation */)
        internal virtual override
        onlyAssetManager
    { // solhint-disable-line no-empty-blocks
    }
}

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

