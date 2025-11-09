
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {Agent} from "../../assetManager/library/data/Agent.sol";


contract AgentAlwaysAllowedMintersFacet is AssetManagerBase {
    using EnumerableSet for EnumerableSet.AddressSet;

    function addAlwaysAllowedMinterForAgent(
        address _agentVault,
        address _minter
    )
        external
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        agent.alwaysAllowedMinters.add(_minter);
    }

    function removeAlwaysAllowedMinterForAgent(
        address _agentVault,
        address _minter
    )
        external
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        agent.alwaysAllowedMinters.remove(_minter);
    }

    function alwaysAllowedMintersForAgent(
        address _agentVault
    )
        external view
        returns (address[] memory)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        return agent.alwaysAllowedMinters.values();
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {IICollateralPool} from "../../../collateralPool/interfaces/IICollateralPool.sol";


library Agent {
    error InvalidAgentVaultAddress();

    enum Status {
        EMPTY,              // agent does not exist
        NORMAL,
        LIQUIDATION,        // liquidation due to CR - ends when agent is healthy
        FULL_LIQUIDATION,   // illegal payment liquidation - must liquidate all and close vault
        DESTROYING,         // agent announced destroy, cannot mint again
        DESTROYED           // agent has been destroyed, cannot do anything except return info
    }

    // For agents to withdraw NAT collateral, they must first announce it and then wait
    // withdrawalAnnouncementSeconds.
    // The announced amount cannot be used as collateral for minting during that time.
    // This makes sure that agents cannot just remove all collateral if they are challenged.
    struct WithdrawalAnnouncement {
        // Announce amount in collateral token's minimum unit (wei).
        uint128 amountWei;

        // The timestamp when withdrawal can be executed.
        uint64 allowedAt;
    }

    // Struct to store agent's pending setting updates.
    struct SettingUpdate {
        uint128 value;
        uint64 validAt;
    }

    struct State {
        IICollateralPool collateralPool;

        // Address of the agent owner. This is the management address, which is immutable.
        // The work address can be retrieved from the global state mapping between
        // management and work addresses.
        address ownerManagementAddress;

        // Current underlying address for this agent vault.
        // The address is immutable.
        string underlyingAddressString;

        // `underlyingAddressString` is only used for sending the minter a correct payment address;
        // for matching payment addresses we always use `underlyingAddressHash = keccak256(underlyingAddressString)`
        bytes32 underlyingAddressHash;

        // Current status of the agent (changes for liquidation).
        Agent.Status status;

        // Index of collateral vault token.
        // The data is obtained as state.collateralTokens[vaultCollateralIndex].
        uint16 vaultCollateralIndex;

        // Index of token in collateral pool. This is always wrapped FLR/SGB, however the wrapping
        // contract (WNat) may change. In such case we add new collateral token with class POOL but the
        // agent must call a method to upgrade to new contract, se we must track the actual token used.
        uint16 poolCollateralIndex;

        // Position of this agent in the list of agents available for minting.
        // Value is actually `list index + 1`, so that 0 means 'not in the list'.
        uint32 availableAgentsPos;

        // Minting fee in BIPS (collected in underlying currency).
        uint16 feeBIPS;

        // Share of the minting fee that goes to the pool as percentage of the minting fee.
        uint16 poolFeeShareBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingVaultCollateralRatioBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingPoolCollateralRatioBIPS;

        // Timestamp of the startLiquidation (or liquidate) call.
        uint64 liquidationStartedAt;

        // Liquidation phase at the time when liquidation started.
        uint8 __initialLiquidationPhase; // only storage placeholder

        // Bitmap signifying which collateral type(s) triggered liquidation (LF_VAULT | LF_POOL).
        uint8 collateralsUnderwater;

        // Amount of collateral locked by collateral reservation.
        uint64 reservedAMG;

        // Amount of collateral backing minted fassets.
        uint64 mintedAMG;

        // The amount of fassets being redeemed. In this case, the fassets were already burned,
        // but the collateral must still be locked to allow payment in case of redemption failure.
        // The distinction between 'minted' and 'redeemed' assets is important in case of challenge.
        uint64 redeemingAMG;

        // The amount of fassets being redeemed EXCEPT those from pool self-close exits.
        // Unlike normal redemption, pool collateral was already withdrawn, so the redeeming collateral
        // must only be accounted for / locked for vault collateral.
        // On redemption payment failure, redeemer will be paid only in vault collateral in this case
        // (and will be paid less if there isn't enough - small extra risk for pool token holders).
        // There will always be `poolRedeemingAMG <= redeemingAMG`.
        uint64 poolRedeemingAMG;

        // When lot size changes, there may be some leftover after redemption that doesn't fit
        // a whole lot size. It is added to dustAMG and can be recovered via self-close.
        // Unlike redeemingAMG, dustAMG is still counted in the mintedAMG.
        uint64 dustAMG;

        // The amount of funds that on the agent's underlying address.
        // If it is higher than the amount needed to back mintings, it can be withdrawn after announcement.
        // It is signed int, because unreported deposits combined with other operations can in principle
        // make it negative. We could truncate it at 0, but if deposit report comes later, this would make
        // the value wrong.
        int128 underlyingBalanceUBA;

        // There can be only one announced underlying withdrawal per agent active at any time.
        // This variable holds the id, or 0 if there is no announced underlying withdrawal going on.
        uint64 announcedUnderlyingWithdrawalId;

        // The time when ongoing underlying withdrawal was announced.
        uint64 underlyingWithdrawalAnnouncedAt;

        // Announcement for vault collateral withdrawal.
        WithdrawalAnnouncement vaultCollateralWithdrawalAnnouncement;

        // Announcement for pool token withdrawal (which also means pool collateral withdrawal).
        WithdrawalAnnouncement poolTokenWithdrawalAnnouncement;

        // Underlying block when the agent was created.
        // Challenger's should track underlying address activity since this block
        // and topups are only valid after this block (both inclusive).
        uint64 underlyingBlockAtCreation;

        // The time when ongoing agent vault destroy was announced.
        uint64 destroyAllowedAt;

        // The factor set by the agent to multiply the price at which agent buys f-assets from pool
        // token holders on self-close exit (when requested or the redeemed amount is less than 1 lot).
        uint16 buyFAssetByAgentFactorBIPS;

        // The announced time when the agent is exiting available agents list.
        uint64 exitAvailableAfterTs;

        // The position of the agent in the list of all agents.
        uint32 allAgentsPos;

        // Agent's pending setting updates.
        mapping(bytes32 => SettingUpdate) settingUpdates;

        // Agent's handshake type - minting or redeeming can be rejected.
        // 0 - no verification, 1 - manual verification, ...
        uint32 __handshakeType; // only storage placeholder

        // There can only be one transfer to core vault per agent active at any time.
        uint64 activeTransferToCoreVault;

        // the request id of the active return from core vault
        uint64 activeReturnFromCoreVaultId;

        // part of the agent's reservedAMG for the core vault return
        uint64 returnFromCoreVaultReservedAMG;

        // The redemption fee share paid to the pool (as FAssets).
        // In redemption dominated situations (when agent requests return from core vault to earn
        // from redemption fees), pool can get some share to make it sustainable for pool users.
        // NOTE: the pool fee share is locked at the redemption request time, but is charged at the redemption
        // confirmation time. If agent uses all the redemption fee for transaction fees, this could make the
        // agent's free underlying balance negative.
        uint16 redemptionPoolFeeShareBIPS;

        EnumerableSet.AddressSet alwaysAllowedMinters;

        // Only used for calculating Agent.State size. See deleteStorage() below.
        uint256[1] _endMarker;
    }

    // underwater collateral classes
    uint8 internal constant LF_VAULT = 1 << 0;
    uint8 internal constant LF_POOL = 1 << 1;

    // diamond state accessors

    bytes32 internal constant AGENTS_POSITION = keccak256("fasset.AssetManager.Agent");

    // only return valid agent - fail if status is EMPTY or DESTROYED
    function get(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        Agent.Status status = agent.status;
        require(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED, InvalidAgentVaultAddress());
        return agent;
    }

    // Like get, but only fail if status is EMPTY.
    // This is useful for reading agent info after the agent has been destroyed.
    function getAllowDestroyed(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        require(agent.status != Agent.Status.EMPTY, InvalidAgentVaultAddress());
        return agent;
    }

    function getWithoutCheck(address _address)
        internal pure
        returns (Agent.State storage _agent)
    {
        bytes32 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64));
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _agent.slot := position
        }
    }

    function vaultAddress(Agent.State storage _agent)
        internal pure
        returns (address)
    {
        bytes32 position;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            position := _agent.slot
        }
        return address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64));
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Agents} from "../library/Agents.sol";
import {Globals} from "../library/Globals.sol";
import {AssetManagerState} from "../library/data/AssetManagerState.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";


abstract contract AssetManagerBase {
    error OnlyAssetManagerController();
    error NotAttached();
    error NotWhitelisted();
    error EmergencyPauseActive();

    modifier onlyAssetManagerController {
        _checkOnlyAssetManagerController();
        _;
    }

    modifier onlyAttached {
        _checkOnlyAttached();
        _;
    }

    modifier notEmergencyPaused {
        _checkEmergencyPauseNotActive();
        _;
    }

    modifier onlyAgentVaultOwner(address _agentVault) {
        Agents.requireAgentVaultOwner(_agentVault);
        _;
    }

    function _checkOnlyAssetManagerController() private view {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        require(msg.sender == settings.assetManagerController, OnlyAssetManagerController());
    }

    function _checkOnlyAttached() private view {
        require(AssetManagerState.get().attached, NotAttached());
    }

    function _checkEmergencyPauseNotActive() private view {
        AssetManagerState.State storage state = AssetManagerState.get();
        require(state.emergencyPausedUntil <= block.timestamp, EmergencyPauseActive());
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

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {IICollateralPool} from "../../../collateralPool/interfaces/IICollateralPool.sol";


library Agent {
    error InvalidAgentVaultAddress();

    enum Status {
        EMPTY,              // agent does not exist
        NORMAL,
        LIQUIDATION,        // liquidation due to CR - ends when agent is healthy
        FULL_LIQUIDATION,   // illegal payment liquidation - must liquidate all and close vault
        DESTROYING,         // agent announced destroy, cannot mint again
        DESTROYED           // agent has been destroyed, cannot do anything except return info
    }

    // For agents to withdraw NAT collateral, they must first announce it and then wait
    // withdrawalAnnouncementSeconds.
    // The announced amount cannot be used as collateral for minting during that time.
    // This makes sure that agents cannot just remove all collateral if they are challenged.
    struct WithdrawalAnnouncement {
        // Announce amount in collateral token's minimum unit (wei).
        uint128 amountWei;

        // The timestamp when withdrawal can be executed.
        uint64 allowedAt;
    }

    // Struct to store agent's pending setting updates.
    struct SettingUpdate {
        uint128 value;
        uint64 validAt;
    }

    struct State {
        IICollateralPool collateralPool;

        // Address of the agent owner. This is the management address, which is immutable.
        // The work address can be retrieved from the global state mapping between
        // management and work addresses.
        address ownerManagementAddress;

        // Current underlying address for this agent vault.
        // The address is immutable.
        string underlyingAddressString;

        // `underlyingAddressString` is only used for sending the minter a correct payment address;
        // for matching payment addresses we always use `underlyingAddressHash = keccak256(underlyingAddressString)`
        bytes32 underlyingAddressHash;

        // Current status of the agent (changes for liquidation).
        Agent.Status status;

        // Index of collateral vault token.
        // The data is obtained as state.collateralTokens[vaultCollateralIndex].
        uint16 vaultCollateralIndex;

        // Index of token in collateral pool. This is always wrapped FLR/SGB, however the wrapping
        // contract (WNat) may change. In such case we add new collateral token with class POOL but the
        // agent must call a method to upgrade to new contract, se we must track the actual token used.
        uint16 poolCollateralIndex;

        // Position of this agent in the list of agents available for minting.
        // Value is actually `list index + 1`, so that 0 means 'not in the list'.
        uint32 availableAgentsPos;

        // Minting fee in BIPS (collected in underlying currency).
        uint16 feeBIPS;

        // Share of the minting fee that goes to the pool as percentage of the minting fee.
        uint16 poolFeeShareBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingVaultCollateralRatioBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingPoolCollateralRatioBIPS;

        // Timestamp of the startLiquidation (or liquidate) call.
        uint64 liquidationStartedAt;

        // Liquidation phase at the time when liquidation started.
        uint8 __initialLiquidationPhase; // only storage placeholder

        // Bitmap signifying which collateral type(s) triggered liquidation (LF_VAULT | LF_POOL).
        uint8 collateralsUnderwater;

        // Amount of collateral locked by collateral reservation.
        uint64 reservedAMG;

        // Amount of collateral backing minted fassets.
        uint64 mintedAMG;

        // The amount of fassets being redeemed. In this case, the fassets were already burned,
        // but the collateral must still be locked to allow payment in case of redemption failure.
        // The distinction between 'minted' and 'redeemed' assets is important in case of challenge.
        uint64 redeemingAMG;

        // The amount of fassets being redeemed EXCEPT those from pool self-close exits.
        // Unlike normal redemption, pool collateral was already withdrawn, so the redeeming collateral
        // must only be accounted for / locked for vault collateral.
        // On redemption payment failure, redeemer will be paid only in vault collateral in this case
        // (and will be paid less if there isn't enough - small extra risk for pool token holders).
        // There will always be `poolRedeemingAMG <= redeemingAMG`.
        uint64 poolRedeemingAMG;

        // When lot size changes, there may be some leftover after redemption that doesn't fit
        // a whole lot size. It is added to dustAMG and can be recovered via self-close.
        // Unlike redeemingAMG, dustAMG is still counted in the mintedAMG.
        uint64 dustAMG;

        // The amount of funds that on the agent's underlying address.
        // If it is higher than the amount needed to back mintings, it can be withdrawn after announcement.
        // It is signed int, because unreported deposits combined with other operations can in principle
        // make it negative. We could truncate it at 0, but if deposit report comes later, this would make
        // the value wrong.
        int128 underlyingBalanceUBA;

        // There can be only one announced underlying withdrawal per agent active at any time.
        // This variable holds the id, or 0 if there is no announced underlying withdrawal going on.
        uint64 announcedUnderlyingWithdrawalId;

        // The time when ongoing underlying withdrawal was announced.
        uint64 underlyingWithdrawalAnnouncedAt;

        // Announcement for vault collateral withdrawal.
        WithdrawalAnnouncement vaultCollateralWithdrawalAnnouncement;

        // Announcement for pool token withdrawal (which also means pool collateral withdrawal).
        WithdrawalAnnouncement poolTokenWithdrawalAnnouncement;

        // Underlying block when the agent was created.
        // Challenger's should track underlying address activity since this block
        // and topups are only valid after this block (both inclusive).
        uint64 underlyingBlockAtCreation;

        // The time when ongoing agent vault destroy was announced.
        uint64 destroyAllowedAt;

        // The factor set by the agent to multiply the price at which agent buys f-assets from pool
        // token holders on self-close exit (when requested or the redeemed amount is less than 1 lot).
        uint16 buyFAssetByAgentFactorBIPS;

        // The announced time when the agent is exiting available agents list.
        uint64 exitAvailableAfterTs;

        // The position of the agent in the list of all agents.
        uint32 allAgentsPos;

        // Agent's pending setting updates.
        mapping(bytes32 => SettingUpdate) settingUpdates;

        // Agent's handshake type - minting or redeeming can be rejected.
        // 0 - no verification, 1 - manual verification, ...
        uint32 __handshakeType; // only storage placeholder

        // There can only be one transfer to core vault per agent active at any time.
        uint64 activeTransferToCoreVault;

        // the request id of the active return from core vault
        uint64 activeReturnFromCoreVaultId;

        // part of the agent's reservedAMG for the core vault return
        uint64 returnFromCoreVaultReservedAMG;

        // The redemption fee share paid to the pool (as FAssets).
        // In redemption dominated situations (when agent requests return from core vault to earn
        // from redemption fees), pool can get some share to make it sustainable for pool users.
        // NOTE: the pool fee share is locked at the redemption request time, but is charged at the redemption
        // confirmation time. If agent uses all the redemption fee for transaction fees, this could make the
        // agent's free underlying balance negative.
        uint16 redemptionPoolFeeShareBIPS;

        EnumerableSet.AddressSet alwaysAllowedMinters;

        // Only used for calculating Agent.State size. See deleteStorage() below.
        uint256[1] _endMarker;
    }

    // underwater collateral classes
    uint8 internal constant LF_VAULT = 1 << 0;
    uint8 internal constant LF_POOL = 1 << 1;

    // diamond state accessors

    bytes32 internal constant AGENTS_POSITION = keccak256("fasset.AssetManager.Agent");

    // only return valid agent - fail if status is EMPTY or DESTROYED
    function get(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        Agent.Status status = agent.status;
        require(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED, InvalidAgentVaultAddress());
        return agent;
    }

    // Like get, but only fail if status is EMPTY.
    // This is useful for reading agent info after the agent has been destroyed.
    function getAllowDestroyed(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        require(agent.status != Agent.Status.EMPTY, InvalidAgentVaultAddress());
        return agent;
    }

    function getWithoutCheck(address _address)
        internal pure
        returns (Agent.State storage _agent)
    {
        bytes32 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64));
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _agent.slot := position
        }
    }

    function vaultAddress(Agent.State storage _agent)
        internal pure
        returns (address)
    {
        bytes32 position;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            position := _agent.slot
        }
        return address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64));
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {ICollateralPool} from "../../userInterfaces/ICollateralPool.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";

/**
 * Collateral pool methods that are only callable by the asset manager or pool token.
 */
interface IICollateralPool is ICollateralPool {
    function setPoolToken(address _poolToken) external;

    function depositNat() external payable;

    function payout(
        address _receiver,
        uint256 _amountWei,
        uint256 _agentResponsibilityWei
    ) external;

    function destroy(address payable _recipient) external;

    function upgradeWNatContract(IWNat newWNat) external;

    function setExitCollateralRatioBIPS(uint256 _value) external;

    function fAssetFeeDeposited(uint256 _amount) external;

    function wNat() external view returns (IWNat);

    function debtFreeTokensOf(address _account) external view returns (uint256);

    function debtLockedTokensOf(
        address _account
    ) external view returns (uint256);

    function assetManager() external view returns (IIAssetManager);
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
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {Collateral} from "./data/Collateral.sol";
import {Globals} from "./Globals.sol";
import {Conversion} from "./Conversion.sol";
import {Agent} from "./data/Agent.sol";
import {RedemptionQueue} from "./data/RedemptionQueue.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {AgentInfo} from "../../userInterfaces/data/AgentInfo.sol";

library Agents {
    using SafeCast for uint256;
    using SafePct for uint256;
    using Agent for Agent.State;
    using RedemptionQueue for RedemptionQueue.State;

    error AgentNotWhitelisted();
    error OnlyAgentVaultOwner();
    error OnlyCollateralPool();


    function getAllAgents(
        uint256 _start,
        uint256 _end
    )
        internal view
        returns (address[] memory _agents, uint256 _totalLength)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        _totalLength = state.allAgents.length;
        _end = Math.min(_end, _totalLength);
        _start = Math.min(_start, _end);
        _agents = new address[](_end - _start);
        for (uint256 i = _start; i < _end; i++) {
            _agents[i - _start] = state.allAgents[i];
        }
    }

    function getAgentStatus(
        Agent.State storage _agent
    )
        internal view
        returns (AgentInfo.Status)
    {
        Agent.Status status = _agent.status;
        if (status == Agent.Status.NORMAL) {
            return AgentInfo.Status.NORMAL;
        } else if (status == Agent.Status.LIQUIDATION) {
            return AgentInfo.Status.LIQUIDATION;
        } else if (status == Agent.Status.FULL_LIQUIDATION) {
            return AgentInfo.Status.FULL_LIQUIDATION;
        } else if (status == Agent.Status.DESTROYING) {
            return AgentInfo.Status.DESTROYING;
        } else {
            assert (status == Agent.Status.DESTROYED);
            return AgentInfo.Status.DESTROYED;
        }
    }

    function isOwner(
        Agent.State storage _agent,
        address _address
    )
        internal view
        returns (bool)
    {
        return _address == _agent.ownerManagementAddress || _address == getWorkAddress(_agent);
    }

    function getWorkAddress(Agent.State storage _agent)
        internal view
        returns (address)
    {
        return Globals.getAgentOwnerRegistry().getWorkAddress(_agent.ownerManagementAddress);
    }

    function getOwnerPayAddress(Agent.State storage _agent)
        internal view
        returns (address payable)
    {
        address workAddress = getWorkAddress(_agent);
        return workAddress != address(0) ? payable(workAddress) : payable(_agent.ownerManagementAddress);
    }

    function requireWhitelisted(
        address _ownerManagementAddress
    )
        internal view
    {
        require(Globals.getAgentOwnerRegistry().isWhitelisted(_ownerManagementAddress),
            AgentNotWhitelisted());
    }

    function requireWhitelistedAgentVaultOwner(
        Agent.State storage _agent
    )
        internal view
    {
        requireWhitelisted(_agent.ownerManagementAddress);
    }

    function requireAgentVaultOwner(
        address _agentVault
    )
        internal view
    {
        require(isOwner(Agent.get(_agentVault), msg.sender), OnlyAgentVaultOwner());
    }

    function requireAgentVaultOwner(
        Agent.State storage _agent
    )
        internal view
    {
        require(isOwner(_agent, msg.sender), OnlyAgentVaultOwner());
    }

    function requireCollateralPool(
        Agent.State storage _agent
    )
        internal view
    {
        require(msg.sender == address(_agent.collateralPool), OnlyCollateralPool());
    }

    function isCollateralToken(
        Agent.State storage _agent,
        IERC20 _token
    )
        internal view
        returns (bool)
    {
        return _token == getPoolWNat(_agent) || _token == getVaultCollateralToken(_agent);
    }

    function getVaultCollateralToken(Agent.State storage _agent)
        internal view
        returns (IERC20)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[_agent.vaultCollateralIndex].token;
    }

    function getVaultCollateral(Agent.State storage _agent)
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[_agent.vaultCollateralIndex];
    }

    function convertUSD5ToVaultCollateralWei(Agent.State storage _agent, uint256 _amountUSD5)
        internal view
        returns (uint256)
    {
        return Conversion.convertFromUSD5(_amountUSD5, getVaultCollateral(_agent));
    }

    function getPoolWNat(Agent.State storage _agent)
        internal view
        returns (IWNat)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return IWNat(address(state.collateralTokens[_agent.poolCollateralIndex].token));
    }

    function getPoolCollateral(Agent.State storage _agent)
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[_agent.poolCollateralIndex];
    }

    function getCollateral(Agent.State storage _agent, Collateral.Kind _kind)
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        assert (_kind != Collateral.Kind.AGENT_POOL);   // there is no agent pool collateral token
        AssetManagerState.State storage state = AssetManagerState.get();
        if (_kind == Collateral.Kind.VAULT) {
            return state.collateralTokens[_agent.vaultCollateralIndex];
        } else {
            return state.collateralTokens[_agent.poolCollateralIndex];
        }
    }

    function collateralUnderwater(Agent.State storage _agent, Collateral.Kind _kind)
        internal view
        returns (bool)
    {
        if (_kind == Collateral.Kind.VAULT) {
            return (_agent.collateralsUnderwater & Agent.LF_VAULT) != 0;
        } else {
            // AGENT_POOL collateral cannot be underwater (it only affects minting),
            // so this function will only be used for VAULT and POOL
            assert(_kind == Collateral.Kind.POOL);
            return (_agent.collateralsUnderwater & Agent.LF_POOL) != 0;
        }
    }

    function withdrawalAnnouncement(Agent.State storage _agent, Collateral.Kind _kind)
        internal view
        returns (Agent.WithdrawalAnnouncement storage)
    {
        assert (_kind != Collateral.Kind.POOL);     // agent cannot withdraw from pool
        return _kind == Collateral.Kind.VAULT
            ? _agent.vaultCollateralWithdrawalAnnouncement
            : _agent.poolTokenWithdrawalAnnouncement;
    }

    function totalBackedAMG(Agent.State storage _agent)
        internal view
        returns (uint64)
    {
        // this must always hold, so assert it is true, otherwise the following line
        // would need `max(redeemingAMG, poolRedeemingAMG)`
        assert(_agent.poolRedeemingAMG <= _agent.redeemingAMG);
        return _agent.mintedAMG + _agent.reservedAMG + _agent.redeemingAMG;
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
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IDistributionToDelegators} from "@flarenetwork/flare-periphery-contracts/flare/IDistributionToDelegators.sol";
import {IRewardManager} from "@flarenetwork/flare-periphery-contracts/flare/IRewardManager.sol";
import {ICollateralPoolToken} from "./ICollateralPoolToken.sol";


interface ICollateralPool {
    event CPEntered(
        address indexed tokenHolder,
        uint256 amountNatWei,
        uint256 receivedTokensWei,
        uint256 timelockExpiresAt);

    event CPExited(
        address indexed tokenHolder,
        uint256 burnedTokensWei,
        uint256 receivedNatWei);

    event CPSelfCloseExited(
        address indexed tokenHolder,
        uint256 burnedTokensWei,
        uint256 receivedNatWei,
        uint256 closedFAssetsUBA);

    event CPFeeDebtPaid(
        address indexed tokenHolder,
        uint256 paidFeesUBA);

    event CPFeesWithdrawn(
        address indexed tokenHolder,
        uint256 withdrawnFeesUBA);

    event CPFeeDebtChanged(
        address indexed tokenHolder,
        int256 newFeeDebtUBA);

    // Emitted when asset manager forces payout from the pool
    event CPPaidOut(
        address indexed recipient,
        uint256 paidNatWei,
        uint256 burnedTokensWei);

    event CPClaimedReward(
        uint256 amountNatWei,
        uint8 rewardType);

    error OnlyAssetManager();
    error OnlyAgent();
    error AlreadyInitialized();
    error OnlyInternalUse();
    error PoolTokenAlreadySet();
    error AmountOfNatTooLow();
    error AmountOfCollateralTooLow();
    error DepositResultsInZeroTokens();
    error TokenShareIsZero();
    error TokenBalanceTooLow();
    error SentAmountTooLow();
    error CollateralRatioFallsBelowExitCR();
    error InvalidRecipientAddress();
    error RedemptionRequiresClosingTooManyTickets();
    error FreeFAssetBalanceTooSmall();
    error WithdrawZeroFAsset();
    error ZeroFAssetDebtPayment();
    error PaymentLargerThanFeeDebt();
    error FAssetAllowanceTooSmall();
    error TokenSupplyAfterExitTooLow();
    error CollateralAfterExitTooLow();
    error CannotDestroyPoolWithIssuedTokens();

    /**
     * Enters the collateral pool by depositing NAT.
     * The tokens have a timelock before which exiting or transferring tokens is not possible.
     * If there are some FAsset fees already in the pool, tokens also have some fee debt, which
     * has to be paid off to make tokens transferable (but they can be redeemed).
     */
    function enter()
        external payable
        returns (uint256 _receivedTokens, uint256 _timelockExpiresAt);

    /**
     * Exits the pool by redeeming the given amount of pool tokens for a share of NAT and f-asset fees.
     * Exiting with non-transferable tokens awards the user with NAT only, while transferable tokens also entitle
     * one to a share of f-asset fees. As there are multiple ways to split spending transferable and
     * non-transferable tokens, the method also takes a parameter called `_exitType`.
     * Exiting with collateral that sinks pool's collateral ratio below exit CR is not allowed and
     *  will revert. In that case, see selfCloseExit.
     * @param _tokenShare   The amount of pool tokens to be redeemed
     */
    function exit(uint256 _tokenShare)
        external
        returns (uint256 _natShare);

    /**
     * Exits the pool by redeeming the given amount of pool tokens and burning f-assets in a way that doesn't
     * endanger the pool collateral ratio. Specifically, if pool's collateral ratio is above exit CR, then
     * the method burns an amount of user's f-assets that do not lower collateral ratio below exit CR. If, on
     * the other hand, collateral pool is below exit CR, then the method burns an amount of user's f-assets
     * that preserve the pool's collateral ratio.
     * F-assets will be redeemed in collateral if their value does not exceed one lot, regardless of
     *  `_redeemToCollateral` value.
     * Method first tries to satisfy the condition by taking f-assets out of sender's f-asset fee share,
     *  specified by `_tokenShare`. If it is not enough it moves on to spending total sender's f-asset fees. If they
     *  are not enough, it takes from the sender's f-asset balance. Spending sender's f-asset fees means that
     *  transferable tokens are converted to non-transferable.
     * In case of self-close via redemption, the user can set executor to trigger possible default.
     * In this case, some NAT can be sent with transaction, to pay the executor's fee.
     * @param _tokenShare                   The amount of pool tokens to be liquidated
     * @param _redeemToCollateral           Specifies if redeemed f-assets should be exchanged to vault collateral
     *                                      by the agent
     * @param _redeemerUnderlyingAddress    Redeemer's address on the underlying chain
     * @param _executor                     The account that is allowed to execute redemption default
     */
    function selfCloseExit(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    ) external payable;

    /**
     * Collect f-asset fees by locking an appropriate ratio of transferable tokens
     * @param _amount  The amount of f-asset fees to withdraw.
     *                 Must be positive and smaller or equal to the sender's fAsset fees.
     */
    function withdrawFees(uint256 _amount) external;

    /**
     * Exits the pool by redeeming the given amount of pool tokens for a share of NAT and f-asset fees.
     * Exiting with non-transferable tokens awards the user with NAT only, while transferable tokens also entitle
     * one to a share of f-asset fees. As there are multiple ways to split spending transferable and
     * non-transferable tokens, the method also takes a parameter called `_exitType`.
     * Exiting with collateral that sinks pool's collateral ratio below exit CR is not allowed and
     *  will revert. In that case, see selfCloseExit.
     * @param _tokenShare   The amount of pool tokens to be redeemed
     * @param _recipient    The address to which NATs and FAsset fees will be transferred
     */
    function exitTo(uint256 _tokenShare, address payable _recipient)
        external
        returns (uint256 _natShare);

    /**
     * Exits the pool by redeeming the given amount of pool tokens and burning f-assets in a way that doesn't
     * endanger the pool collateral ratio. Specifically, if pool's collateral ratio is above exit CR, then
     * the method burns an amount of user's f-assets that do not lower collateral ratio below exit CR. If, on
     * the other hand, collateral pool is below exit CR, then the method burns an amount of user's f-assets
     * that preserve the pool's collateral ratio.
     * F-assets will be redeemed in collateral if their value does not exceed one lot, regardless of
     *  `_redeemToCollateral` value.
     * Method first tries to satisfy the condition by taking f-assets out of sender's f-asset fee share,
     *  specified by `_tokenShare`. If it is not enough it moves on to spending total sender's f-asset fees. If they
     *  are not enough, it takes from the sender's f-asset balance. Spending sender's f-asset fees means that
     *  transferable tokens are converted to non-transferable.
     * In case of self-close via redemption, the user can set executor to trigger possible default.
     * In this case, some NAT can be sent with transaction, to pay the executor's fee.
     * @param _tokenShare                   The amount of pool tokens to be liquidated
     * @param _redeemToCollateral           Specifies if redeemed f-assets should be exchanged to vault collateral
     *                                      by the agent
     * @param _recipient                    The address to which NATs and FAsset fees will be transferred
     * @param _redeemerUnderlyingAddress    Redeemer's address on the underlying chain
     * @param _executor                     The account that is allowed to execute redemption default
     */
    function selfCloseExitTo(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        address payable _recipient,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    ) external payable;

    /**
     * Collect f-asset fees by locking an appropriate ratio of transferable tokens
     * @param _amount       The amount of f-asset fees to withdraw.
     *                      Must be positive and smaller or equal to the sender's fAsset fees.
     * @param _recipient    The address to which FAsset fees will be transferred
     */
    function withdrawFeesTo(uint256 _amount, address _recipient) external;

    /**
     * Unlock pool tokens by paying f-asset fee debt
     * @param _fassets  The amount of debt f-asset fees to pay for
     */
    function payFAssetFeeDebt(uint256 _fassets) external;

    /**
     * Claim airdrops earned by holding wrapped native tokens in the pool.
     * NOTE: only the owner of the pool's corresponding agent vault may call this method.
     */
    function claimAirdropDistribution(
        IDistributionToDelegators _distribution,
        uint256 _month
    ) external
        returns(uint256 _claimedAmount);

    /**
     * Opt out of airdrops for wrapped native tokens in the pool.
     * NOTE: only the owner of the pool's corresponding agent vault may call this method.
     */
    function optOutOfAirdrop(
        IDistributionToDelegators _distribution
    ) external;

    /**
     * Delegate WNat vote power for the wrapped native tokens held in this vault.
     * NOTE: only the owner of the pool's corresponding agent vault may call this method.
     */
    function delegate(address _to, uint256 _bips) external;

    /**
     * Clear WNat delegation.
     */
    function undelegateAll() external;

    /**
     * Claim the rewards earned by delegating the vote power for the pool.
     * NOTE: only the owner of the pool's corresponding agent vault may call this method.
     */
    function claimDelegationRewards(
        IRewardManager _rewardManager,
        uint24 _lastRewardEpoch,
        IRewardManager.RewardClaimWithProof[] calldata _proofs
    ) external
        returns(uint256 _claimedAmount);

    /**
     * Get the ERC20 pool token used by this collateral pool
     */
    function poolToken()
        external view
        returns (ICollateralPoolToken);

    /**
     * Get the vault of the agent that owns this collateral pool
     */
    function agentVault()
        external view
        returns (address);

    /**
     * Get the exit collateral ratio in BIPS
     * This is the collateral ratio below which exiting the pool is not allowed
     */
    function exitCollateralRatioBIPS()
        external view
        returns (uint32);


    /**
     * Return total amount of collateral in the pool.
     * This can be different to WNat.balanceOf(poolAddress), because the collateral has to be tracked
     * to prevent unexpected deposit type of attacks on the pool.
     */
    function totalCollateral()
        external view
        returns (uint256);

    /**
     * Returns the f-asset fees belonging to this user.
     * This is the amount of f-assets the user can withdraw by burning transferable pool tokens.
     * @param _account User address
     */
    function fAssetFeesOf(address _account)
        external view
        returns (uint256);

    /**
     * Returns the total f-asset fees in the pool.
     * This can be different to FAsset.balanceOf(poolAddress), because the collateral has to be tracked
     * to prevent unexpected deposit type of attacks on the pool.
     */
    function totalFAssetFees()
        external view
        returns (uint256);

    /**
     * Returns the user's f-asset fee debt.
     * This is the amount of f-assets the user has to pay to make all pool tokens transferable.
     * The debt is created on entering the pool if the user doesn't provide the f-assets corresponding
     * to the share of the f-asset fees already in the pool.
     * @param _account User address
     */
    function fAssetFeeDebtOf(address _account)
        external view
        returns (int256);

    /**
     * Returns the total f-asset fee debt for all users.
     */
    function totalFAssetFeeDebt()
        external view
        returns (int256);

    /**
     * Get the amount of fassets that need to be burned to perform self close exit.
     */
    function fAssetRequiredForSelfCloseExit(uint256 _tokenAmountWei)
        external view
        returns (uint256);
}

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {ReentrancyGuard} from "../../openzeppelin/security/ReentrancyGuard.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {Transfers} from "../../utils/library/Transfers.sol";
import {MathUtils} from "../../utils/library/MathUtils.sol";
import {IFAsset} from "../../userInterfaces/IFAsset.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IICollateralPool} from "../../collateralPool/interfaces/IICollateralPool.sol";
import {IICollateralPoolToken} from "../interfaces/IICollateralPoolToken.sol";
import {ICollateralPoolToken} from "../../userInterfaces/ICollateralPoolToken.sol";
import {IRewardManager} from "@flarenetwork/flare-periphery-contracts/flare/IRewardManager.sol";
import {IDistributionToDelegators} from "@flarenetwork/flare-periphery-contracts/flare/IDistributionToDelegators.sol";
import {ICollateralPool} from "../../userInterfaces/ICollateralPool.sol";


//slither-disable reentrancy    // all possible reentrancies guarded by nonReentrant
contract CollateralPool is IICollateralPool, ReentrancyGuard, UUPSUpgradeable, IERC165 {
    using SafeCast for uint256;
    using SafeCast for int256;
    using SafePct for uint256;
    using SafeERC20 for IFAsset;
    using SafeERC20 for IWNat;

    struct AssetPrice {
        uint256 mul;
        uint256 div;
    }

    uint256 public constant MIN_NAT_TO_ENTER = 1 ether;
    uint256 public constant MIN_TOKEN_SUPPLY_AFTER_EXIT = 1 ether;
    uint256 public constant MIN_NAT_BALANCE_AFTER_EXIT = 1 ether;

    address public agentVault;          // practically immutable because there is no setter
    IIAssetManager public assetManager; // practically immutable because there is no setter
    IFAsset public fAsset;              // practically immutable because there is no setter
    IICollateralPoolToken public token; // only changed once at deploy time

    IWNat public wNat;
    uint32 public exitCollateralRatioBIPS;

    uint32 private __topupCollateralRatioBIPS; // only storage placeholder
    uint16 private __topupTokenPriceFactorBIPS; // only storage placeholder

    bool private internalWithdrawal;
    bool private initialized;

    mapping(address => int256) private _fAssetFeeDebtOf;
    int256 public totalFAssetFeeDebt;
    uint256 public totalFAssetFees;
    uint256 public totalCollateral;

    modifier onlyAssetManager {
        require(msg.sender == address(assetManager), OnlyAssetManager());
        _;
    }

    modifier onlyAgent {
        require(isAgentVaultOwner(msg.sender), OnlyAgent());
        _;
    }

    // Only used in some tests.
    // The implementation in production will always be deployed with all zero addresses and parameters.
    constructor (
        address _agentVault,
        address _assetManager,
        address _fAsset,
        uint32 _exitCollateralRatioBIPS
    ) {
        initialize(_agentVault, _assetManager, _fAsset, _exitCollateralRatioBIPS);
    }

    function initialize(
        address _agentVault,
        address _assetManager,
        address _fAsset,
        uint32 _exitCollateralRatioBIPS
    )
        public
    {
        require(!initialized, AlreadyInitialized());
        initialized = true;
        // init vars
        agentVault = _agentVault;
        assetManager = IIAssetManager(_assetManager);
        fAsset = IFAsset(_fAsset);
        // for proxy implementation, assetManager will be 0
        wNat = address(assetManager) != address(0) ? assetManager.getWNat() : IWNat(address(0));
        exitCollateralRatioBIPS = _exitCollateralRatioBIPS;
        initializeReentrancyGuard();
    }

    receive() external payable {
        require(internalWithdrawal, OnlyInternalUse());
    }

    function setPoolToken(address _poolToken)
        external
        onlyAssetManager
    {
        require(address(token) == address(0), PoolTokenAlreadySet());
        token = IICollateralPoolToken(_poolToken);
    }

    /**
     * @notice Returns the collateral pool token contract used by this contract
     */
    function poolToken() external view returns (ICollateralPoolToken) {
        return token;
    }

    function setExitCollateralRatioBIPS(uint256 _exitCollateralRatioBIPS)
        external
        onlyAssetManager
    {
        exitCollateralRatioBIPS = _exitCollateralRatioBIPS.toUint32();
    }

    /**
     * @notice Enters the collateral pool by depositing some NAT
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function enter()
        external payable
        nonReentrant
        returns (uint256, uint256)
    {
        require(msg.value >= MIN_NAT_TO_ENTER, AmountOfNatTooLow());
        uint256 totalPoolTokens = token.totalSupply();
        if (totalPoolTokens == 0) {
            // this conditions are set for keeping a stable token value
            require(msg.value >= totalCollateral, AmountOfCollateralTooLow());
            AssetPrice memory assetPrice = _getAssetPrice();
            require(msg.value >= totalFAssetFees.mulDiv(assetPrice.mul, assetPrice.div), AmountOfCollateralTooLow());
        }
        // calculate obtained pool tokens and free f-assets
        uint256 tokenShare = _collateralToTokenShare(msg.value);
        require(tokenShare > 0, DepositResultsInZeroTokens());
        // calculate and create fee debt
        uint256 feeDebt = totalPoolTokens > 0 ? _totalVirtualFees().mulDiv(tokenShare, totalPoolTokens) : 0;
        _createFAssetFeeDebt(msg.sender, feeDebt);
        // deposit collateral
        _depositWNat();
        // mint pool tokens to the sender
        uint256 timelockExp = token.mint(msg.sender, tokenShare);
        // emit event
        emit CPEntered(msg.sender, msg.value, tokenShare, timelockExp);
        return (tokenShare, timelockExp);
    }

    /**
     * @notice Exits the pool by liquidating the given amount of pool tokens
     * @param _tokenShare   The amount of pool tokens to be liquidated
     *                      Must be positive and smaller or equal to the sender's token balance
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function exit(uint256 _tokenShare)
        external
        nonReentrant
        returns (uint256)
    {
        return _exitTo(_tokenShare, payable(msg.sender));
    }

    /**
     * @notice Exits the pool by liquidating the given amount of pool tokens
     * @param _tokenShare   The amount of pool tokens to be liquidated
     *                      Must be positive and smaller or equal to the sender's token balance
     * @param _recipient    The address to which NATs and FAsset fees will be transferred
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function exitTo(uint256 _tokenShare, address payable _recipient)
        external
        nonReentrant
        returns (uint256)
    {
        return _exitTo(_tokenShare, _recipient);
    }

    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function _exitTo(uint256 _tokenShare, address payable _recipient)
        private
        returns (uint256)
    {
        require(_tokenShare > 0, TokenShareIsZero());
        require(_tokenShare <= token.balanceOf(msg.sender), TokenBalanceTooLow());
        _requireMinTokenSupplyAfterExit(_tokenShare);
        // token.totalSupply() >= token.balanceOf(msg.sender) >= _tokenShare > 0
        uint256 natShare = totalCollateral.mulDiv(_tokenShare, token.totalSupply());
        require(natShare > 0, SentAmountTooLow());
        _requireMinNatSupplyAfterExit(natShare);
        require(_staysAboveExitCR(natShare), CollateralRatioFallsBelowExitCR());
        // update the fasset fee debt
        uint256 debtFAssetFeeShare = _tokensToVirtualFeeShare(_tokenShare);
        _deleteFAssetFeeDebt(msg.sender, debtFAssetFeeShare);
        token.burn(msg.sender, _tokenShare, false);
        _withdrawWNatTo(_recipient, natShare);
        // emit event
        emit CPExited(msg.sender, _tokenShare, natShare);
        return natShare;
    }

    /**
     * @notice Exits the pool by liquidating the given amount of pool tokens and redeeming
     *  f-assets in a way that either preserves the pool collateral ratio or keeps it above exit CR
     * @param _tokenShare                   The amount of pool tokens to be liquidated
     *                                      Must be positive and smaller or equal to the sender's token balance
     * @param _redeemToCollateral           Specifies if redeemed f-assets should be exchanged to vault collateral
     *                                      by the agent
     * @param _redeemerUnderlyingAddress    Redeemer's address on the underlying chain
     * @param _executor                     The account that is allowed to execute redemption default
     * @notice F-assets will be redeemed in collateral if their value does not exceed one lot
     * @notice All f-asset fees will be redeemed along with potential additionally required f-assets taken
     *  from the sender's f-asset account
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function selfCloseExit(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    )
        external payable
        nonReentrant
    {
        _selfCloseExitTo(_tokenShare, _redeemToCollateral, payable(msg.sender), _redeemerUnderlyingAddress, _executor);
    }

    /**
     * @notice Exits the pool by liquidating the given amount of pool tokens and redeeming
     *  f-assets in a way that either preserves the pool collateral ratio or keeps it above exit CR
     * @param _tokenShare                   The amount of pool tokens to be liquidated
     *                                      Must be positive and smaller or equal to the sender's token balance
     * @param _redeemToCollateral           Specifies if redeemed f-assets should be exchanged to vault collateral
     *                                      by the agent
     * @param _recipient                    The address to which NATs and FAsset fees will be transferred
     * @param _redeemerUnderlyingAddress    Redeemer's address on the underlying chain
     * @param _executor                     The account that is allowed to execute redemption default
     * @notice F-assets will be redeemed in collateral if their value does not exceed one lot
     * @notice All f-asset fees will be redeemed along with potential additionally required f-assets taken
     *  from the sender's f-asset account
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function selfCloseExitTo(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        address payable _recipient,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    )
        external payable
        nonReentrant
    {
        require(_recipient != address(0) && _recipient != address(this) && _recipient != agentVault,
            InvalidRecipientAddress());
        _selfCloseExitTo(_tokenShare, _redeemToCollateral, _recipient, _redeemerUnderlyingAddress, _executor);
    }

    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function _selfCloseExitTo(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        address payable _recipient,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    )
        private
    {
        require(_tokenShare > 0, TokenShareIsZero());
        require(_tokenShare <= token.balanceOf(msg.sender), TokenBalanceTooLow());
        _requireMinTokenSupplyAfterExit(_tokenShare);
        // token.totalSupply() >= token.balanceOf(msg.sender) >= _tokenShare > 0
        uint256 natShare = totalCollateral.mulDiv(_tokenShare, token.totalSupply());
        require(natShare > 0, SentAmountTooLow());
        _requireMinNatSupplyAfterExit(natShare);
        uint256 maxAgentRedemption = assetManager.maxRedemptionFromAgent(agentVault);
        uint256 requiredFAssets = _getFAssetRequiredToNotSpoilCR(natShare);
        // Rare case: if agent has too many low-valued open tickets they can't redeem the requiredFAssets
        // in one transaction. In that case, we revert and the user should retry with lower amount.
        require(maxAgentRedemption > requiredFAssets, RedemptionRequiresClosingTooManyTickets());
        // get owner f-asset fees to be spent (maximize fee withdrawal to cover the potentially necessary f-assets)
        uint256 debtFAssetFeeShare = _tokensToVirtualFeeShare(_tokenShare);
        // transfer the owner's fassets that will be redeemed
        require(fAsset.allowance(msg.sender, address(this)) >= requiredFAssets, FAssetAllowanceTooSmall());
        fAsset.safeTransferFrom(msg.sender, address(this), requiredFAssets);
        // redeem f-assets if necessary
        bool returnFunds = true;
        if (requiredFAssets > 0) {
            if (requiredFAssets < assetManager.lotSize() || _redeemToCollateral) {
                assetManager.redeemFromAgentInCollateral(agentVault, _recipient, requiredFAssets);
            } else {
                returnFunds = _executor == address(0);
                // pass `msg.value` to `redeemFromAgent` for the executor fee if `_executor` is set
                assetManager.redeemFromAgent{ value: returnFunds ? 0 : msg.value }(
                    agentVault, _recipient, requiredFAssets, _redeemerUnderlyingAddress, _executor);
            }
        }
        _deleteFAssetFeeDebt(msg.sender, debtFAssetFeeShare);
        token.burn(msg.sender, _tokenShare, false);
        _withdrawWNatTo(_recipient, natShare);
        if (returnFunds) {
            // return any NAT included by mistake to the recipient
            Transfers.transferNAT(_recipient, msg.value);
        }
        // emit event
        emit CPSelfCloseExited(msg.sender, _tokenShare, natShare, requiredFAssets);
    }

    /**
     * Get the amount of fassets that need to be burned to perform self close exit.
     */
    function fAssetRequiredForSelfCloseExit(uint256 _tokenAmountWei)
        external view
        returns (uint256)
    {
        uint256 tokenNatWeiEquiv = totalCollateral.mulDiv(_tokenAmountWei, token.totalSupply());
        return _getFAssetRequiredToNotSpoilCR(tokenNatWeiEquiv);
    }

    /**
     * @notice Collect f-asset fees by locking free tokens
     * @param _fAssets  The amount of f-asset fees to withdraw
     *                  Must be positive and smaller or equal to the sender's reward f-assets
     */
    function withdrawFees(uint256 _fAssets)
        external
        nonReentrant
    {
        _withdrawFeesTo(_fAssets, msg.sender);
    }

    /**
     * @notice Collect f-asset fees by locking free tokens
     * @param _fAssets      The amount of f-asset fees to withdraw
     *                      Must be positive and smaller or equal to the sender's fAsset fees.
     * @param _recipient    The address to which FAsset fees will be transferred
     */
    function withdrawFeesTo(uint256 _fAssets, address _recipient)
        external
        nonReentrant
    {
        _withdrawFeesTo(_fAssets, _recipient);
    }

    /**
     * @notice Collect f-asset fees by locking free tokens
     * @param _fAssets      The amount of f-asset fees to withdraw
     *                      Must be positive and smaller or equal to the sender's reward f-assets
     * @param _recipient    The address to which NATs and FAsset fees will be transferred
     */
    function _withdrawFeesTo(uint256 _fAssets, address _recipient)
        private
    {
        require(_fAssets > 0, WithdrawZeroFAsset());
        uint256 freeFAssetFeeShare = _fAssetFeesOf(msg.sender);
        require(_fAssets <= freeFAssetFeeShare, FreeFAssetBalanceTooSmall());
        _createFAssetFeeDebt(msg.sender, _fAssets);
        _transferFAssetTo(_recipient, _fAssets);
        // emit event
        emit CPFeesWithdrawn(msg.sender, _fAssets);
    }

    /**
     * @notice Free debt tokens by paying f-assets
     * @param _fAssets  Amount of payed f-assets
     *                  _fAssets must be positive and smaller or equal to the sender's debt f-assets
     */
    function payFAssetFeeDebt(uint256 _fAssets)
        external
        nonReentrant
    {
        require(_fAssets != 0, ZeroFAssetDebtPayment());
        require(_fAssets.toInt256() <= _fAssetFeeDebtOf[msg.sender], PaymentLargerThanFeeDebt());
        require(fAsset.allowance(msg.sender, address(this)) >= _fAssets, FAssetAllowanceTooSmall());
        _deleteFAssetFeeDebt(msg.sender, _fAssets);
        _transferFAssetFrom(msg.sender, _fAssets);
        // emit event
        emit CPFeeDebtPaid(msg.sender, _fAssets);
    }

    // support for liquidation / redemption default payments
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function payout(
        address _recipient,
        uint256 _amount,
        uint256 _agentResponsibilityWei
    )
        external
        onlyAssetManager
        nonReentrant
    {
        // slash agent vault's pool tokens worth _agentResponsibilityWei in FLR (or less if there is not enough)
        uint256 agentTokenBalance = token.balanceOf(agentVault);
        uint256 maxSlashedTokens = totalCollateral > 0 ?
            token.totalSupply().mulDivRoundUp(_agentResponsibilityWei, totalCollateral) : agentTokenBalance;
        uint256 slashedTokens = Math.min(maxSlashedTokens, agentTokenBalance);
        if (slashedTokens > 0) {
            uint256 debtFAssetFeeShare = _tokensToVirtualFeeShare(slashedTokens);
            _deleteFAssetFeeDebt(agentVault, debtFAssetFeeShare);
            token.burn(agentVault, slashedTokens, true);
        }
        // transfer collateral to the recipient
        _transferWNatTo(_recipient, _amount);
        emit CPPaidOut(_recipient, _amount, slashedTokens);
    }

    function _collateralToTokenShare(
        uint256 _collateral
    )
        internal view
        returns (uint256)
    {
        uint256 totalPoolTokens = token.totalSupply();
        if (totalCollateral == 0 || totalPoolTokens == 0) { // pool is empty
            return _collateral;
        }
        return totalPoolTokens.mulDiv(_collateral, totalCollateral);
    }

    // _tokens is assumed to be smaller or equal to _account's token balance
    function _tokensToVirtualFeeShare(
        uint256 _tokens
    )
        internal view
        returns (uint256)
    {
        if (_tokens == 0) return 0;
        uint256 totalPoolTokens = token.totalSupply();
        assert(_tokens <= totalPoolTokens);
        // poolTokenSupply >= _tokens AND _tokens > 0 together imply poolTokenSupply != 0
        return _totalVirtualFees().mulDiv(_tokens, totalPoolTokens);
    }

    function _getFAssetRequiredToNotSpoilCR(
        uint256 _natShare
    )
        internal view
        returns (uint256)
    {
        // calculate f-assets required for CR to stay above max(exitCR, poolCR) when taking out _natShare
        // if pool is below exitCR, we shouldn't require it be increased above exitCR, only preserved
        // if pool is above exitCR, we require only for it to stay that way (like in the normal exit)
        AssetPrice memory assetPrice = _getAssetPrice();
        uint256 exitCR = _safeExitCR();
        uint256 backedFAssets = _agentBackedFAssets();
        uint256 resultWithoutRounding;
        if (_isAboveCR(assetPrice, backedFAssets, totalCollateral, exitCR)) {
            // f-asset required for CR to stay above exitCR (might not be needed)
            // solve (N - n) / (p / q (F - f)) >= cr get f = max(0, F - q (N - n) / (p cr))
            // assetPrice.mul > 0, exitCR > 1
            resultWithoutRounding = MathUtils.subOrZero(backedFAssets,
                assetPrice.div * (totalCollateral - _natShare) * SafePct.MAX_BIPS / (assetPrice.mul * exitCR));
        } else {
            // f-asset that preserves pool CR (assume poolNatBalance >= natShare > 0)
            // solve (N - n) / (F - f) = N / F get f = n F / N
            resultWithoutRounding = backedFAssets.mulDivRoundUp(_natShare, totalCollateral);
        }
        return MathUtils.roundUp(resultWithoutRounding, assetManager.assetMintingGranularityUBA());
    }

    function _staysAboveExitCR(
        uint256 _withdrawnNat
    )
        internal view
        returns (bool)
    {
        return _isAboveCR(_getAssetPrice(), _agentBackedFAssets(), totalCollateral - _withdrawnNat, _safeExitCR());
    }

    function _isAboveCR(
        AssetPrice memory _assetPrice,
        uint256 _backedFAssets,
        uint256 _poolCollateralNat,
        uint256 _crBIPS
    )
        internal pure
        returns (bool)
    {
        // check (N - n) / (F p / q) >= cr get (N - n) q >= F p cr
        return _poolCollateralNat * _assetPrice.div >= (_backedFAssets * _assetPrice.mul).mulBips(_crBIPS);
    }

    function _agentBackedFAssets()
        internal view
        returns (uint256)
    {
        return assetManager.getFAssetsBackedByPool(agentVault);
    }

    function _virtualFAssetFeesOf(
        address _account
    )
        internal view
        returns (uint256)
    {
        uint256 tokens = token.balanceOf(_account);
        return _tokensToVirtualFeeShare(tokens);
    }

    function _fAssetFeesOf(
        address _account
    )
        internal view
        returns (uint256)
    {
        int256 virtualFAssetFees = _virtualFAssetFeesOf(_account).toInt256();
        int256 accountFeeDebt = _fAssetFeeDebtOf[_account];
        int256 userFees = virtualFAssetFees - accountFeeDebt;
        // note: rounding errors can make debtFassets larger than virtualFassets by at most one
        // this can happen only when user has no free f-assets (that is why MathUtils.subOrZero)
        // note: rounding errors can make freeFassets larger than total pool f-asset fees by small amounts
        // (The reason for Math.min and Math.positivePart is to restrict to interval [0, poolFAssetFees])
        return Math.min(MathUtils.positivePart(userFees), totalFAssetFees);
    }

    function _debtFreeTokensOf(
        address _account
    )
        internal view
        returns (uint256)
    {
        int256 accountFeeDebt = _fAssetFeeDebtOf[_account];
        if (accountFeeDebt <= 0) {
            // with no debt, all tokens are free
            // this avoids the case where freeFassets == poolVirtualFAssetFees == 0
            return token.balanceOf(_account);
        }
        uint256 virtualFassets = _virtualFAssetFeesOf(_account);
        assert(virtualFassets <= _totalVirtualFees());
        uint256 freeFassets = MathUtils.positivePart(virtualFassets.toInt256() - accountFeeDebt);
        if (freeFassets == 0) return 0;
        // nonzero divisor: _totalVirtualFees() >= virtualFassets >= freeFassets > 0
        return token.totalSupply().mulDiv(freeFassets, _totalVirtualFees());
    }

    function _getAssetPrice()
        internal view
        returns (AssetPrice memory)
    {
        (uint256 assetPriceMul, uint256 assetPriceDiv) = assetManager.assetPriceNatWei();
        return AssetPrice({
            mul: assetPriceMul,
            div: assetPriceDiv
        });
    }

    function _totalVirtualFees()
        internal view
        returns (uint256)
    {
        int256 virtualFees = totalFAssetFees.toInt256() + totalFAssetFeeDebt;
        // Invariant: virtualFees >= 0 always (otherwise the following line will revert).
        // Proof: the places where `totalFAssetFees` and `totalFAssetFeeDebt` change are: `enter`,
        // `exit`/`selfCloseExit`, `withdrawFees` and `payFAssetFeeDebt`.
        // In `withdrawFees` and `payFAssetFeeDebt`, amounts of `totalFAssetFees` and `totalFAssetFeeDebt`
        // change with opposite sign, so virtualFees is unchanged.
        // In `enter`, the `totalFAssetFeeDebt` increases and the other is unchanged, so virtualFees increases.
        // Thus the only place where `totalFAssetFeeDebt` and thus virtualFees decreases is in`exit`/`selfCloseExit`.
        // The decrease there is by `_tokensToVirtualFeeShare()`, which is virtualFees times a factor
        // `tokenShare/totalTokens`, which is checked to be at most 1.
        return virtualFees.toUint256();
    }

    // if governance changes `minPoolCollateralRatioBIPS` it can be higher than `exitCollateralRatioBIPS`
    function _safeExitCR()
        internal view
        returns (uint256)
    {
        uint256 minPoolCollateralRatioBIPS = assetManager.getAgentMinPoolCollateralRatioBIPS(agentVault);
        return Math.max(minPoolCollateralRatioBIPS, exitCollateralRatioBIPS);
    }

    function _requireMinTokenSupplyAfterExit(
        uint256 _tokenShare
    )
        internal view
    {
        uint256 totalPoolTokens = token.totalSupply();
        require(totalPoolTokens == _tokenShare || totalPoolTokens - _tokenShare >= MIN_TOKEN_SUPPLY_AFTER_EXIT,
            TokenSupplyAfterExitTooLow());
    }

    function _requireMinNatSupplyAfterExit(
        uint256 _natShare
    )
        internal view
    {
        require(totalCollateral == _natShare || totalCollateral - _natShare >= MIN_NAT_BALANCE_AFTER_EXIT,
            CollateralAfterExitTooLow());
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // tracking wNat collateral and f-asset fees

    function depositNat()
        external payable
        onlyAssetManager
        nonReentrant
    {
        _depositWNat();
    }

    // this is needed to track asset manager's minting fee deposit
    function fAssetFeeDeposited(
        uint256 _amount
    )
        external
        onlyAssetManager
    {
        totalFAssetFees += _amount;
    }

    function _createFAssetFeeDebt(address _account, uint256 _fAssets)
        internal
    {
        if (_fAssets == 0) return;
        int256 fAssets = _fAssets.toInt256();
        _fAssetFeeDebtOf[_account] += fAssets;
        totalFAssetFeeDebt += fAssets;
        emit CPFeeDebtChanged(_account, _fAssetFeeDebtOf[_account]);
    }

    // _fAssets should be smaller or equal to _account's f-asset debt
    function _deleteFAssetFeeDebt(address _account, uint256 _fAssets)
        internal
    {
        if (_fAssets == 0) return;
        int256 fAssets = _fAssets.toInt256();
        _fAssetFeeDebtOf[_account] -= fAssets;
        totalFAssetFeeDebt -= fAssets;
        emit CPFeeDebtChanged(_account, _fAssetFeeDebtOf[_account]);
    }

    function _transferFAssetFrom(
        address _from,
        uint256 _amount
    )
        internal
    {
        if (_amount > 0) {
            totalFAssetFees += _amount;
            fAsset.safeTransferFrom(_from, address(this), _amount);
        }
    }

    function _transferFAssetTo(
        address _to,
        uint256 _amount
    )
        internal
    {
        if (_amount > 0) {
            totalFAssetFees -= _amount;
            fAsset.safeTransfer(_to, _amount);
        }
    }

    function _transferWNatTo(
        address _to,
        uint256 _amount
    )
        internal
    {
        if (_amount > 0) {
            totalCollateral -= _amount;
            wNat.safeTransfer(_to, _amount);
        }
    }

    function _withdrawWNatTo(
        address payable _recipient,
        uint256 _amount
    )
        internal
    {
        if (_amount > 0) {
            totalCollateral -= _amount;
            internalWithdrawal = true;
            wNat.withdraw(_amount);
            internalWithdrawal = false;
            Transfers.transferNAT(_recipient, _amount);
        }
    }

    function _depositWNat()
        internal
    {
        // msg.value is always > 0 in this contract
        if (msg.value > 0) {
            totalCollateral += msg.value;
            wNat.deposit{value: msg.value}();
            assetManager.updateCollateral(agentVault, wNat);
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // methods for viewing user balances

    /**
     * @notice Returns the sum of the user's reward f-assets and their corresponding f-asset debt
     * @param _account  User address
     */
    function virtualFAssetOf(address _account)
        external view
        returns (uint256)
    {
        return _virtualFAssetFeesOf(_account);
    }

    /**
     * @notice Returns user's reward f-assets
     * @param _account  User address
     */
    function fAssetFeesOf(address _account)
        external view
        returns (uint256)
    {
        return _fAssetFeesOf(_account);
    }

    /**
     * @notice Returns user's f-asset debt
     * @param _account  User address
     */
    function fAssetFeeDebtOf(address _account)
        external view
        returns (int256)
    {
        return _fAssetFeeDebtOf[_account];
    }

    /**
     * @notice Returns user's debt tokens
     * @param _account  User address
     */
    function debtLockedTokensOf(address _account)
        external view
        returns (uint256)
    {
        return MathUtils.subOrZero(token.balanceOf(_account), _debtFreeTokensOf(_account));
    }

    /**
     * @notice Returns user's free tokens
     * @param _account  User address
     */
    function debtFreeTokensOf(address _account)
        external view
        returns (uint256)
    {
        return _debtFreeTokensOf(_account);
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // Methods to allow for management and destruction of the pool

    function destroy(address payable _recipient)
        external
        onlyAssetManager
        nonReentrant
    {
        require(token.totalSupply() == 0, CannotDestroyPoolWithIssuedTokens());
        // transfer native balance as WNat, if any
        Transfers.depositWNat(wNat, _recipient, address(this).balance);
        // transfer untracked f-assets and wNat, if any
        uint256 untrackedWNat = wNat.balanceOf(address(this));
        uint256 untrackedFAsset = fAsset.balanceOf(address(this));
        if (untrackedWNat > 0) {
            wNat.safeTransfer(_recipient, untrackedWNat);
        }
        if (untrackedFAsset > 0) {
            fAsset.safeTransfer(_recipient, untrackedFAsset);
        }
    }

    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function upgradeWNatContract(IWNat _newWNat)
        external
        onlyAssetManager
        nonReentrant
    {
        if (_newWNat == wNat) return;
        // transfer all funds to new WNat
        uint256 balance = wNat.balanceOf(address(this));
        internalWithdrawal = true;
        wNat.withdraw(balance);
        internalWithdrawal = false;
        _newWNat.deposit{value: balance}();
        // set new WNat contract
        wNat = _newWNat;
        assetManager.updateCollateral(agentVault, wNat);
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // Delegation of the pool's collateral and airdrop claiming (same as in AgentVault)

    function delegate(address _to, uint256 _bips) external onlyAgent {
        wNat.delegate(_to, _bips);
    }

    function undelegateAll() external onlyAgent {
        wNat.undelegateAll();
    }

    function delegateGovernance(address _to) external onlyAgent {
        wNat.governanceVotePower().delegate(_to);
    }

    function undelegateGovernance() external onlyAgent {
        wNat.governanceVotePower().undelegate();
    }

    function claimDelegationRewards(
        IRewardManager _rewardManager,
        uint24 _lastRewardEpoch,
        IRewardManager.RewardClaimWithProof[] calldata _proofs
    )
        external
        onlyAgent
        nonReentrant
        returns (uint256)
    {
        uint256 balanceBefore = wNat.balanceOf(address(this));
        _rewardManager.claim(address(this), payable(address(this)), _lastRewardEpoch, true, _proofs);
        uint256 balanceAfter = wNat.balanceOf(address(this));
        uint256 claimed = balanceAfter - balanceBefore;
        totalCollateral += claimed;
        assetManager.updateCollateral(agentVault, wNat);
        emit CPClaimedReward(claimed, 1);
        return claimed;
    }

    function claimAirdropDistribution(
        IDistributionToDelegators _distribution,
        uint256 _month
    )
        external
        onlyAgent
        nonReentrant
        returns(uint256)
    {
        uint256 balanceBefore = wNat.balanceOf(address(this));
        _distribution.claim(address(this), payable(address(this)), _month, true);
        uint256 balanceAfter = wNat.balanceOf(address(this));
        uint256 claimed = balanceAfter - balanceBefore;
        totalCollateral += claimed;
        assetManager.updateCollateral(agentVault, wNat);
        emit CPClaimedReward(claimed, 0);
        return claimed;
    }

    function optOutOfAirdrop(
        IDistributionToDelegators _distribution
    )
        external
        onlyAgent
        nonReentrant
    {
        _distribution.optOutOfAirdrop();
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

    ////////////////////////////////////////////////////////////////////////////////////
    // The rest

    function isAgentVaultOwner(address _address)
        internal view
        returns (bool)
    {
        return assetManager.isAgentVaultOwner(agentVault, _address);
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(ICollateralPool).interfaceId
            || _interfaceId == type(IICollateralPool).interfaceId;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {ICollateralPoolToken} from "../../userInterfaces/ICollateralPoolToken.sol";


interface IICollateralPoolToken is ICollateralPoolToken, IERC165 {

    function mint(address _account, uint256 _amount) external returns (uint256 _timelockExpiresAt);
    function burn(address _account, uint256 _amount, bool _ignoreTimelocked) external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {IICollateralPoolToken} from "../interfaces/IICollateralPoolToken.sol";
import {IICollateralPool} from "../../collateralPool/interfaces/IICollateralPool.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {ICollateralPoolToken} from "../../userInterfaces/ICollateralPoolToken.sol";


contract CollateralPoolToken is IICollateralPoolToken, ERC20, UUPSUpgradeable {
    using SafeCast for uint256;

    error OnlyAssetManager();
    error InsufficientNonTimelockedBalance();
    error InsufficientTransferableBalance();
    error AlreadyInitialized();
    error OnlyCollateralPool();

    struct Timelock {
        uint128 amount;
        uint64 endTime;
    }

    struct TimelockQueue {
        mapping(uint256 => Timelock) data;
        uint128 start;
        uint128 end;
    }

    address public collateralPool;  // practically immutable because there is no setter

    string private tokenName;       // practically immutable because there is no setter
    string private tokenSymbol;     // practically immutable because there is no setter

    mapping(address => TimelockQueue) private timelocksByAccount;
    bool private ignoreTimelocked;
    bool private initialized;

    modifier onlyCollateralPool {
        require(msg.sender == collateralPool, OnlyCollateralPool());
        _;
    }

    // Only used in some tests.
    // The implementation in production will always be deployed with all zero address for collateral pool.
    constructor(
        address _collateralPool,
        string memory _tokenName,
        string memory _tokenSymbol
    )
        ERC20(_tokenName, _tokenSymbol)
    {
        initialize(_collateralPool, _tokenName, _tokenSymbol);
    }

    function initialize(
        address _collateralPool,
        string memory _tokenName,
        string memory _tokenSymbol
    )
        public
    {
        require(!initialized, AlreadyInitialized());
        initialized = true;
        // init vars
        collateralPool = _collateralPool;
        tokenName = _tokenName;
        tokenSymbol = _tokenSymbol;
    }

    /**
     * @dev Returns the name of the token.
     */
    function name() public view virtual override returns (string memory) {
        return tokenName;
    }

    /**
     * @dev Returns the symbol of the token, usually a shorter version of the
     * name.
     */
    function symbol() public view virtual override returns (string memory) {
        return tokenSymbol;
    }

    function mint(
        address _account,
        uint256 _amount
    )
        external
        onlyCollateralPool
        returns (uint256 _timelockExpiresAt)
    {
        _mint(_account, _amount);
        uint256 timelockDuration = _getTimelockDuration();
        _timelockExpiresAt = block.timestamp + timelockDuration;
        if (timelockDuration > 0 && _amount > 0) {
            TimelockQueue storage timelocks = timelocksByAccount[_account];
            timelocks.data[timelocks.end++] = Timelock({
                amount: _amount.toUint128(),
                endTime: _timelockExpiresAt.toUint64()
            });
        }
    }

    function burn(
        address _account,
        uint256 _amount,
        bool _ignoreTimelocked
    )
        external
        onlyCollateralPool
    {
        if (_ignoreTimelocked) {
            ignoreTimelocked = true;
        }
        _burn(_account, _amount);
        if (_ignoreTimelocked) {
            ignoreTimelocked = false;
        }
    }

    function lockedBalanceOf(
        address _account
    )
        external view
        returns (uint256)
    {
        uint256 debtLockedBalance = debtLockedBalanceOf(_account);
        uint256 timelockedBalance = timelockedBalanceOf(_account);
        return (debtLockedBalance > timelockedBalance) ? debtLockedBalance : timelockedBalance;
    }

    function transferableBalanceOf(
        address _account
    )
        external view
        returns (uint256)
    {
        uint256 debtFreeBalance = debtFreeBalanceOf(_account);
        uint256 nonTimelockedBalance = nonTimelockedBalanceOf(_account);
        return (debtFreeBalance < nonTimelockedBalance) ? debtFreeBalance : nonTimelockedBalance;
    }

    function debtFreeBalanceOf(
        address _account
    )
        public view
        returns (uint256)
    {
        return IICollateralPool(collateralPool).debtFreeTokensOf(_account);
    }

    function debtLockedBalanceOf(
        address _account
    )
        public view
        returns (uint256)
    {
        return IICollateralPool(collateralPool).debtLockedTokensOf(_account);
    }

    function timelockedBalanceOf(
        address _account
    )
        public view
        returns (uint256 _timelocked)
    {
        TimelockQueue storage timelocks = timelocksByAccount[_account];
        uint256 end = timelocks.end;
        for (uint256 i = timelocks.start; i < end; i++) {
            Timelock storage timelock = timelocks.data[i];
            if (timelock.endTime > block.timestamp) {
                _timelocked += timelock.amount;
            }
        }
        // in agent payout, locked tokens can be burnt without a timelock update,
        // which makes timelockedBalance > totalBalance
        uint256 totalBalance = balanceOf(_account);
        _timelocked = (_timelocked < totalBalance) ? _timelocked : totalBalance;
    }

    function nonTimelockedBalanceOf(
        address _account
    )
        public view
        returns (uint256)
    {
        return balanceOf(_account) - timelockedBalanceOf(_account);
    }

    function _beforeTokenTransfer(
        address _from, address /* _to */, uint256 _amount
    )
        internal override
    {
        if (msg.sender != collateralPool) {
            uint256 transferable = debtFreeBalanceOf(_from);
            require(_amount <= transferable, InsufficientTransferableBalance());
        }
        // either user transfer or non-minting collateral pool with ignoreTimelocked=false flag
        if (!ignoreTimelocked && _from != address(0)) {
            // 10 is some arbitrary number that is usually enough; however, there isn't much damage
            // if it is too little - just the non-timelocked balance may be too small and you have to call again
            cleanupExpiredTimelocks(_from, 10);
            uint256 nonTimelocked = nonTimelockedBalanceOf(_from);
            require(_amount <= nonTimelocked, InsufficientNonTimelockedBalance());
        }
        // if ignoreTimelock, then we are spending from timelocked balance,
        // the reason why it is not updated is because it might not fit in one transaction
        // (if timelock data is too large), which could block the asset manager from making
        // agent payout from the pool
    }

    // this can be called externally by anyone with different _maxTimelockedEntries,
    // if there are too many timelocked entries to clear in one transaction
    // (should be rare, especially if timelock duration is short - e.g. <= day)
    function cleanupExpiredTimelocks(
        address _account,
        uint256 _maxTimelockedEntries
    )
        public
        returns (bool _cleanedAllExpired)
    {
        TimelockQueue storage timelocks = timelocksByAccount[_account];
        uint256 start = timelocks.start;
        for (uint256 count = 0; count < _maxTimelockedEntries; count++) {
            if (start >= timelocks.end || timelocks.data[start].endTime > block.timestamp) {
                break;
            }
            delete timelocks.data[start++];
        }
        timelocks.start = start.toUint128();
        return start >= timelocks.end || timelocks.data[start].endTime > block.timestamp;
    }

    function _getTimelockDuration()
        internal view
        returns (uint256)
    {
        IIAssetManager assetManager = IICollateralPool(collateralPool).assetManager();
        return assetManager.getCollateralPoolTokenTimelockSeconds();
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
            || _interfaceId == type(ICollateralPoolToken).interfaceId;
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
    {
        IIAssetManager assetManager = IICollateralPool(collateralPool).assetManager();
        require(msg.sender == address(assetManager), OnlyAssetManager());
    }
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
pragma solidity >=0.7.6 <0.9;

import "../../IRewardManager.sol";

/**
 * RewardManager internal interface.
 */
interface IIRewardManager is IRewardManager {
    /**
     * Claim rewards for `_rewardOwner` and transfer them to `_recipient`.
     * It can be called only by FtsoRewardManagerProxy contract.
     * @param _msgSender Address of the message sender.
     * @param _rewardOwner Address of the reward owner.
     * @param _recipient Address of the reward recipient.
     * @param _rewardEpochId Id of the reward epoch up to which the rewards are claimed.
     * @param _wrap Indicates if the reward should be wrapped (deposited) to the WNAT contract.
     * @param _proofs Array of reward claims with merkle proofs.
     * @return _rewardAmountWei Amount of rewarded native tokens (wei).
     */
    function claimProxy(
        address _msgSender,
        address _rewardOwner,
        address payable _recipient,
        uint24 _rewardEpochId,
        bool _wrap,
        RewardClaimWithProof[] calldata _proofs
    ) external returns (uint256 _rewardAmountWei);

    /**
     * Receives funds from reward offers manager.
     * @param _rewardEpochId ID of the reward epoch for which the funds are received.
     * @param _inflation Indicates if the funds come from the inflation (true) or from the community (false).
     * @dev Only reward offers manager can call this method.
     */
    function receiveRewards(
        uint24 _rewardEpochId,
        bool _inflation
    ) external payable;

    /**
     * Collects funds from expired reward epoch and calculates totals.
     *
     * Triggered by FlareSystemsManager on finalization of a reward epoch.
     * Operation is irreversible: when some reward epoch is closed according to current
     * settings, it cannot be reopened even if new parameters would
     * allow it, because `nextRewardEpochIdToExpire` in FlareSystemsManager never decreases.
     * @param _rewardEpochId Id of the reward epoch to close.
     */
    function closeExpiredRewardEpoch(uint256 _rewardEpochId) external;
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

