
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { InitializableAbstractStrategy } from "../../utils/InitializableAbstractStrategy.sol";
import { IWETH9 } from "../../interfaces/IWETH9.sol";
import { CompoundingValidatorManager } from "./CompoundingValidatorManager.sol";

/// @title Compounding Staking SSV Strategy
/// @notice Strategy to deploy funds into DVT validators powered by the SSV Network
/// @author Origin Protocol Inc
contract CompoundingStakingSSVStrategy is
    CompoundingValidatorManager,
    InitializableAbstractStrategy
{
    // For future use
    uint256[50] private __gap;

    /// @param _baseConfig Base strategy config with
    ///   `platformAddress` not used so empty address
    ///   `vaultAddress` the address of the OETH Vault contract
    /// @param _wethAddress Address of the WETH Token contract
    /// @param _ssvNetwork Address of the SSV Network contract
    /// @param _beaconChainDepositContract Address of the beacon chain deposit contract
    /// @param _beaconProofs Address of the Beacon Proofs contract that verifies beacon chain data
    /// @param _beaconGenesisTimestamp The timestamp of the Beacon chain's genesis.
    constructor(
        BaseStrategyConfig memory _baseConfig,
        address _wethAddress,
        address _ssvNetwork,
        address _beaconChainDepositContract,
        address _beaconProofs,
        uint64 _beaconGenesisTimestamp
    )
        InitializableAbstractStrategy(_baseConfig)
        CompoundingValidatorManager(
            _wethAddress,
            _baseConfig.vaultAddress,
            _beaconChainDepositContract,
            _ssvNetwork,
            _beaconProofs,
            _beaconGenesisTimestamp
        )
    {
        // Make sure nobody owns the implementation contract
        _setGovernor(address(0));
    }

    /// @notice Set up initial internal state including
    /// 1. approving the SSVNetwork to transfer SSV tokens from this strategy contract
    /// @param _rewardTokenAddresses Not used so empty array
    /// @param _assets Not used so empty array
    /// @param _pTokens Not used so empty array
    function initialize(
        address[] memory _rewardTokenAddresses,
        address[] memory _assets,
        address[] memory _pTokens
    ) external onlyGovernor initializer {
        InitializableAbstractStrategy._initialize(
            _rewardTokenAddresses,
            _assets,
            _pTokens
        );
    }

    /// @notice Unlike other strategies, this does not deposit assets into the underlying platform.
    /// It just checks the asset is WETH and emits the Deposit event.
    /// To deposit WETH into validators, `registerSsvValidator` and `stakeEth` must be used.
    /// @param _asset Address of the WETH token.
    /// @param _amount Amount of WETH that was transferred to the strategy by the vault.
    function deposit(address _asset, uint256 _amount)
        external
        override
        onlyVault
        nonReentrant
    {
        require(_asset == WETH, "Unsupported asset");
        require(_amount > 0, "Must deposit something");

        // Account for the new WETH
        depositedWethAccountedFor += _amount;

        emit Deposit(_asset, address(0), _amount);
    }

    /// @notice Unlike other strategies, this does not deposit assets into the underlying platform.
    /// It just emits the Deposit event.
    /// To deposit WETH into validators `registerSsvValidator` and `stakeEth` must be used.
    function depositAll() external override onlyVault nonReentrant {
        uint256 wethBalance = IERC20(WETH).balanceOf(address(this));
        uint256 newWeth = wethBalance - depositedWethAccountedFor;

        if (newWeth > 0) {
            // Account for the new WETH
            depositedWethAccountedFor = wethBalance;

            emit Deposit(WETH, address(0), newWeth);
        }
    }

    /// @notice Withdraw ETH and WETH from this strategy contract.
    /// @param _recipient Address to receive withdrawn assets.
    /// @param _asset Address of the WETH token.
    /// @param _amount Amount of WETH to withdraw.
    function withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) external override nonReentrant {
        require(_asset == WETH, "Unsupported asset");
        require(
            msg.sender == vaultAddress || msg.sender == validatorRegistrator,
            "Caller not Vault or Registrator"
        );

        _withdraw(_recipient, _amount, address(this).balance);
    }

    function _withdraw(
        address _recipient,
        uint256 _withdrawAmount,
        uint256 _ethBalance
    ) internal {
        require(_withdrawAmount > 0, "Must withdraw something");
        require(_recipient == vaultAddress, "Recipient not Vault");

        // Convert any ETH from validator partial withdrawals, exits
        // or execution rewards to WETH and do the necessary accounting.
        if (_ethBalance > 0) _convertEthToWeth(_ethBalance);

        // Transfer WETH to the recipient and do the necessary accounting.
        _transferWeth(_withdrawAmount, _recipient);

        emit Withdrawal(WETH, address(0), _withdrawAmount);
    }

    /// @notice Transfer all WETH deposits, ETH from validator withdrawals and ETH from
    /// execution rewards in this strategy to the vault.
    /// This does not withdraw from the validators. That has to be done separately with the
    /// `validatorWithdrawal` operation.
    function withdrawAll() external override onlyVaultOrGovernor nonReentrant {
        uint256 ethBalance = address(this).balance;
        uint256 withdrawAmount = IERC20(WETH).balanceOf(address(this)) +
            ethBalance;

        if (withdrawAmount > 0) {
            _withdraw(vaultAddress, withdrawAmount, ethBalance);
        }
    }

    /// @notice Accounts for all the assets managed by this strategy which includes:
    /// 1. The current WETH in this strategy contract
    /// 2. The last verified ETH balance, total deposits and total validator balances
    /// @param _asset      Address of WETH asset.
    /// @return balance    Total value in ETH
    function checkBalance(address _asset)
        external
        view
        override
        returns (uint256 balance)
    {
        require(_asset == WETH, "Unsupported asset");

        // Load the last verified balance from the storage
        // and add to the latest WETH balance of this strategy.
        balance =
            lastVerifiedEthBalance +
            IWETH9(WETH).balanceOf(address(this));
    }

    /// @notice Returns bool indicating whether asset is supported by the strategy.
    /// @param _asset The address of the WETH token.
    function supportsAsset(address _asset) public view override returns (bool) {
        return _asset == WETH;
    }

    /// @notice Does nothing but needed as this function is abstract on InitializableAbstractStrategy
    /// @dev Use to be used to approve SSV tokens but that is no longer used by the SSV Network.
    function safeApproveAllTokens() public override {}

    /**
     * @notice We can accept ETH directly to this contract from anyone as it does not impact our accounting
     * like it did in the legacy NativeStakingStrategy.
     * The new ETH will be accounted for in `checkBalance` after the next snapBalances and verifyBalances txs.
     */
    receive() external payable {}

    /***************************************
                Internal functions
    ****************************************/

    /// @notice is not supported for this strategy as there is no platform token.
    function setPTokenAddress(address, address) external pure override {
        revert("Unsupported function");
    }

    /// @notice is not supported for this strategy as there is no platform token.
    function removePToken(uint256) external pure override {
        revert("Unsupported function");
    }

    /// @dev This strategy does not use a platform token like the old Aave and Compound strategies.
    function _abstractSetPToken(address _asset, address) internal override {}

    /// @dev Consensus rewards are compounded to the validator's balance instead of being
    /// swept to this strategy contract.
    /// Execution rewards from MEV and tx priority accumulate as ETH in this strategy contract.
    /// Withdrawals from validators also accumulate as ETH in this strategy contract.
    /// It's too complex to separate the rewards from withdrawals so this function is not implemented.
    /// Besides, ETH rewards are not sent to the Dripper any more. The Vault can now regulate
    /// the increase in assets.
    function _collectRewardTokens() internal pure override {
        revert("Unsupported function");
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title Library to request full or partial withdrawals from validators on the beacon chain.
 * @author Origin Protocol Inc
 */
library PartialWithdrawal {
    /// @notice The address where the withdrawal request is sent to
    /// See https://eips.ethereum.org/EIPS/eip-7002
    address internal constant WITHDRAWAL_REQUEST_ADDRESS =
        0x00000961Ef480Eb55e80D19ad83579A64c007002;

    /// @notice Requests a partial withdrawal for a given validator public key and amount.
    /// @param validatorPubKey The public key of the validator to withdraw from
    /// @param amount The amount of ETH to withdraw
    function request(bytes calldata validatorPubKey, uint64 amount)
        internal
        returns (uint256 fee_)
    {
        require(validatorPubKey.length == 48, "Invalid validator byte length");
        fee_ = fee();

        // Call the Withdrawal Request contract with the validator public key
        // and amount to be withdrawn packed together

        // This is a general purpose EL to CL request:
        // https://eips.ethereum.org/EIPS/eip-7685
        (bool success, ) = WITHDRAWAL_REQUEST_ADDRESS.call{ value: fee_ }(
            abi.encodePacked(validatorPubKey, amount)
        );

        require(success, "Withdrawal request failed");
    }

    /// @notice Gets fee for withdrawal requests contract on Beacon chain
    function fee() internal view returns (uint256) {
        // Get fee from the withdrawal request contract
        (bool success, bytes memory result) = WITHDRAWAL_REQUEST_ADDRESS
            .staticcall("");

        require(success && result.length > 0, "Failed to get fee");
        return abi.decode(result, (uint256));
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
 * @title Library to retrieve beacon block roots.
 * @author Origin Protocol Inc
 */
library BeaconRoots {
    /// @notice The address of beacon block roots oracle
    /// See https://eips.ethereum.org/EIPS/eip-4788
    address internal constant BEACON_ROOTS_ADDRESS =
        0x000F3df6D732807Ef1319fB7B8bB8522d0Beac02;

    /// @notice Returns the beacon block root for the previous block.
    /// This comes from the Beacon Roots contract defined in EIP-4788.
    /// This will revert if the block is more than 8,191 blocks old as
    /// that is the size of the beacon root's ring buffer.
    /// @param timestamp The timestamp of the block for which to get the parent root.
    /// @return parentRoot The parent block root for the given timestamp.
    function parentBlockRoot(uint64 timestamp)
        internal
        view
        returns (bytes32 parentRoot)
    {
        // Call the Beacon Roots contract to get the parent block root.
        // This does not have a function signature, so we use a staticcall.
        (bool success, bytes memory result) = BEACON_ROOTS_ADDRESS.staticcall(
            abi.encode(timestamp)
        );

        require(success && result.length > 0, "Invalid beacon timestamp");
        parentRoot = abi.decode(result, (bytes32));
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title Base contract for vault strategies.
 * @author Origin Protocol Inc
 */
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import { Initializable } from "../utils/Initializable.sol";
import { Governable } from "../governance/Governable.sol";
import { IVault } from "../interfaces/IVault.sol";

abstract contract InitializableAbstractStrategy is Initializable, Governable {
    using SafeERC20 for IERC20;

    event PTokenAdded(address indexed _asset, address _pToken);
    event PTokenRemoved(address indexed _asset, address _pToken);
    event Deposit(address indexed _asset, address _pToken, uint256 _amount);
    event Withdrawal(address indexed _asset, address _pToken, uint256 _amount);
    event RewardTokenCollected(
        address recipient,
        address rewardToken,
        uint256 amount
    );
    event RewardTokenAddressesUpdated(
        address[] _oldAddresses,
        address[] _newAddresses
    );
    event HarvesterAddressesUpdated(
        address _oldHarvesterAddress,
        address _newHarvesterAddress
    );

    /// @notice Address of the underlying platform
    address public immutable platformAddress;
    /// @notice Address of the OToken vault
    address public immutable vaultAddress;

    /// @dev Replaced with an immutable variable
    // slither-disable-next-line constable-states
    address private _deprecated_platformAddress;

    /// @dev Replaced with an immutable
    // slither-disable-next-line constable-states
    address private _deprecated_vaultAddress;

    /// @notice asset => pToken (Platform Specific Token Address)
    mapping(address => address) public assetToPToken;

    /// @notice Full list of all assets supported by the strategy
    address[] internal assetsMapped;

    // Deprecated: Reward token address
    // slither-disable-next-line constable-states
    address private _deprecated_rewardTokenAddress;

    // Deprecated: now resides in Harvester's rewardTokenConfigs
    // slither-disable-next-line constable-states
    uint256 private _deprecated_rewardLiquidationThreshold;

    /// @notice Address of the Harvester contract allowed to collect reward tokens
    address public harvesterAddress;

    /// @notice Address of the reward tokens. eg CRV, BAL, CVX, AURA
    address[] public rewardTokenAddresses;

    /* Reserved for future expansion. Used to be 100 storage slots
     * and has decreased to accommodate:
     * - harvesterAddress
     * - rewardTokenAddresses
     */
    int256[98] private _reserved;

    struct BaseStrategyConfig {
        address platformAddress; // Address of the underlying platform
        address vaultAddress; // Address of the OToken's Vault
    }

    /**
     * @dev Verifies that the caller is the Governor or Strategist.
     */
    modifier onlyGovernorOrStrategist() virtual {
        require(
            isGovernor() || msg.sender == IVault(vaultAddress).strategistAddr(),
            "Caller is not the Strategist or Governor"
        );
        _;
    }

    /**
     * @param _config The platform and OToken vault addresses
     */
    constructor(BaseStrategyConfig memory _config) {
        platformAddress = _config.platformAddress;
        vaultAddress = _config.vaultAddress;
    }

    /**
     * @dev Internal initialize function, to set up initial internal state
     * @param _rewardTokenAddresses Address of reward token for platform
     * @param _assets Addresses of initial supported assets
     * @param _pTokens Platform Token corresponding addresses
     */
    function _initialize(
        address[] memory _rewardTokenAddresses,
        address[] memory _assets,
        address[] memory _pTokens
    ) internal {
        rewardTokenAddresses = _rewardTokenAddresses;

        uint256 assetCount = _assets.length;
        require(assetCount == _pTokens.length, "Invalid input arrays");
        for (uint256 i = 0; i < assetCount; ++i) {
            _setPTokenAddress(_assets[i], _pTokens[i]);
        }
    }

    /**
     * @notice Collect accumulated reward token and send to Vault.
     *         No-ops when the harvester address is not set.
     */
    function collectRewardTokens()
        external
        virtual
        onlyHarvesterOrStrategist
        nonReentrant
    {
        if (harvesterAddress == address(0)) {
            return;
        }
        _collectRewardTokens();
    }

    /**
     * @dev Default implementation that transfers reward tokens to the Harvester.
     * Implementing strategies need to add custom logic to collect the rewards.
     */
    function _collectRewardTokens() internal virtual {
        if (harvesterAddress == address(0)) {
            return;
        }
        uint256 rewardTokenCount = rewardTokenAddresses.length;
        for (uint256 i = 0; i < rewardTokenCount; ++i) {
            IERC20 rewardToken = IERC20(rewardTokenAddresses[i]);
            uint256 balance = rewardToken.balanceOf(address(this));
            if (balance > 0) {
                emit RewardTokenCollected(
                    harvesterAddress,
                    address(rewardToken),
                    balance
                );
                rewardToken.safeTransfer(harvesterAddress, balance);
            }
        }
    }

    /**
     * @dev Verifies that the caller is the Vault.
     */
    modifier onlyVault() {
        require(msg.sender == vaultAddress, "Caller is not the Vault");
        _;
    }

    /**
     * @dev Verifies that the caller is the Harvester or Strategist.
     */
    modifier onlyHarvesterOrStrategist() {
        require(
            msg.sender == harvesterAddress ||
                msg.sender == IVault(vaultAddress).strategistAddr(),
            "Caller is not the Harvester or Strategist"
        );
        _;
    }

    /**
     * @dev Verifies that the caller is the Vault or Governor.
     */
    modifier onlyVaultOrGovernor() {
        require(
            msg.sender == vaultAddress || msg.sender == governor(),
            "Caller is not the Vault or Governor"
        );
        _;
    }

    /**
     * @dev Verifies that the caller is the Vault, Governor, or Strategist.
     */
    modifier onlyVaultOrGovernorOrStrategist() {
        require(
            msg.sender == vaultAddress ||
                msg.sender == governor() ||
                msg.sender == IVault(vaultAddress).strategistAddr(),
            "Caller is not the Vault, Governor, or Strategist"
        );
        _;
    }

    /**
     * @notice Set the reward token addresses. Any old addresses will be overwritten.
     * @param _rewardTokenAddresses Array of reward token addresses
     */
    function setRewardTokenAddresses(address[] calldata _rewardTokenAddresses)
        external
        onlyGovernor
    {
        uint256 rewardTokenCount = _rewardTokenAddresses.length;
        for (uint256 i = 0; i < rewardTokenCount; ++i) {
            require(
                _rewardTokenAddresses[i] != address(0),
                "Can not set an empty address as a reward token"
            );
        }

        emit RewardTokenAddressesUpdated(
            rewardTokenAddresses,
            _rewardTokenAddresses
        );
        rewardTokenAddresses = _rewardTokenAddresses;
    }

    /**
     * @notice Get the reward token addresses.
     * @return address[] the reward token addresses.
     */
    function getRewardTokenAddresses()
        external
        view
        returns (address[] memory)
    {
        return rewardTokenAddresses;
    }

    /**
     * @notice Provide support for asset by passing its pToken address.
     *      This method can only be called by the system Governor
     * @param _asset    Address for the asset
     * @param _pToken   Address for the corresponding platform token
     */
    function setPTokenAddress(address _asset, address _pToken)
        external
        virtual
        onlyGovernor
    {
        _setPTokenAddress(_asset, _pToken);
    }

    /**
     * @notice Remove a supported asset by passing its index.
     *      This method can only be called by the system Governor
     * @param _assetIndex Index of the asset to be removed
     */
    function removePToken(uint256 _assetIndex) external virtual onlyGovernor {
        require(_assetIndex < assetsMapped.length, "Invalid index");
        address asset = assetsMapped[_assetIndex];
        address pToken = assetToPToken[asset];

        if (_assetIndex < assetsMapped.length - 1) {
            assetsMapped[_assetIndex] = assetsMapped[assetsMapped.length - 1];
        }
        assetsMapped.pop();
        assetToPToken[asset] = address(0);

        emit PTokenRemoved(asset, pToken);
    }

    /**
     * @notice Provide support for asset by passing its pToken address.
     *      Add to internal mappings and execute the platform specific,
     * abstract method `_abstractSetPToken`
     * @param _asset    Address for the asset
     * @param _pToken   Address for the corresponding platform token
     */
    function _setPTokenAddress(address _asset, address _pToken) internal {
        require(assetToPToken[_asset] == address(0), "pToken already set");
        require(
            _asset != address(0) && _pToken != address(0),
            "Invalid addresses"
        );

        assetToPToken[_asset] = _pToken;
        assetsMapped.push(_asset);

        emit PTokenAdded(_asset, _pToken);

        _abstractSetPToken(_asset, _pToken);
    }

    /**
     * @notice Transfer token to governor. Intended for recovering tokens stuck in
     *      strategy contracts, i.e. mistaken sends.
     * @param _asset Address for the asset
     * @param _amount Amount of the asset to transfer
     */
    function transferToken(address _asset, uint256 _amount)
        public
        virtual
        onlyGovernor
    {
        require(!supportsAsset(_asset), "Cannot transfer supported asset");
        IERC20(_asset).safeTransfer(governor(), _amount);
    }

    /**
     * @notice Set the Harvester contract that can collect rewards.
     * @param _harvesterAddress Address of the harvester contract.
     */
    function setHarvesterAddress(address _harvesterAddress)
        external
        onlyGovernorOrStrategist
    {
        emit HarvesterAddressesUpdated(harvesterAddress, _harvesterAddress);
        harvesterAddress = _harvesterAddress;
    }

    /***************************************
                 Abstract
    ****************************************/

    function _abstractSetPToken(address _asset, address _pToken)
        internal
        virtual;

    function safeApproveAllTokens() external virtual;

    /**
     * @notice Deposit an amount of assets into the platform
     * @param _asset               Address for the asset
     * @param _amount              Units of asset to deposit
     */
    function deposit(address _asset, uint256 _amount) external virtual;

    /**
     * @notice Deposit all supported assets in this strategy contract to the platform
     */
    function depositAll() external virtual;

    /**
     * @notice Withdraw an `amount` of assets from the platform and
     * send to the `_recipient`.
     * @param _recipient         Address to which the asset should be sent
     * @param _asset             Address of the asset
     * @param _amount            Units of asset to withdraw
     */
    function withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) external virtual;

    /**
     * @notice Withdraw all supported assets from platform and
     * sends to the OToken's Vault.
     */
    function withdrawAll() external virtual;

    /**
     * @notice Get the total asset value held in the platform.
     *      This includes any interest that was generated since depositing.
     * @param _asset      Address of the asset
     * @return balance    Total value of the asset in the platform
     */
    function checkBalance(address _asset)
        external
        view
        virtual
        returns (uint256 balance);

    /**
     * @notice Check if an asset is supported.
     * @param _asset    Address of the asset
     * @return bool     Whether asset is supported
     */
    function supportsAsset(address _asset) public view virtual returns (bool);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { SafeCast } from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import { Math } from "@openzeppelin/contracts/utils/math/Math.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { Pausable } from "@openzeppelin/contracts/security/Pausable.sol";
import { Governable } from "../../governance/Governable.sol";
import { IDepositContract } from "../../interfaces/IDepositContract.sol";
import { IWETH9 } from "../../interfaces/IWETH9.sol";
import { ISSVNetwork, Cluster } from "../../interfaces/ISSVNetwork.sol";
import { BeaconRoots } from "../../beacon/BeaconRoots.sol";
import { PartialWithdrawal } from "../../beacon/PartialWithdrawal.sol";
import { IBeaconProofs } from "../../interfaces/IBeaconProofs.sol";

/**
 * @title Validator lifecycle management contract
 * @notice This contract implements all the required functionality to
 * register, deposit, withdraw, exit and remove validators.
 * @author Origin Protocol Inc
 */
abstract contract CompoundingValidatorManager is Governable, Pausable {
    using SafeERC20 for IERC20;

    /// @dev The amount of ETH in wei that is required for a deposit to a new validator.
    uint256 internal constant DEPOSIT_AMOUNT_WEI = 1 ether;
    /// @dev Validator balances over this amount will eventually become active on the beacon chain.
    /// Due to hysteresis, if the effective balance is 31 ETH, the actual balance
    /// must rise to 32.25 ETH to trigger an effective balance update to 32 ETH.
    /// https://eth2book.info/capella/part2/incentives/balances/#hysteresis
    uint256 internal constant MIN_ACTIVATION_BALANCE_GWEI = 32.25 ether / 1e9;
    /// @dev The maximum number of deposits that are waiting to be verified as processed on the beacon chain.
    uint256 internal constant MAX_DEPOSITS = 32;
    /// @dev The maximum number of validators that can be verified.
    uint256 internal constant MAX_VERIFIED_VALIDATORS = 48;
    /// @dev The default withdrawable epoch value on the Beacon chain.
    /// A value in the far future means the validator is not exiting.
    uint64 internal constant FAR_FUTURE_EPOCH = type(uint64).max;
    /// @dev The number of seconds between each beacon chain slot.
    uint64 internal constant SLOT_DURATION = 12;
    /// @dev The number of slots in each beacon chain epoch.
    uint64 internal constant SLOTS_PER_EPOCH = 32;
    /// @dev Minimum time in seconds to allow snapped balances to be verified.
    /// Set to 35 slots which is 3 slots more than 1 epoch (32 slots). Deposits get processed
    /// once per epoch. This larger than 1 epoch delay should achieve that `snapBalances` sometimes
    /// get called in the middle (or towards the end) of the epoch. Giving the off-chain script
    /// sufficient time after the end of the epoch to prepare the proofs and call `verifyBalances`.
    /// This is considering a malicious actor would keep calling `snapBalances` as frequent as possible
    /// to disturb our operations.
    uint64 public constant SNAP_BALANCES_DELAY = 35 * SLOT_DURATION;

    /// @notice The address of the Wrapped ETH (WETH) token contract
    address internal immutable WETH;
    /// @notice The address of the beacon chain deposit contract
    address internal immutable BEACON_CHAIN_DEPOSIT_CONTRACT;
    /// @notice The address of the SSV Network contract used to interface with
    address internal immutable SSV_NETWORK;
    /// @notice Address of the OETH Vault proxy contract
    address internal immutable VAULT_ADDRESS;
    /// @notice Address of the Beacon Proofs contract that verifies beacon chain data
    address public immutable BEACON_PROOFS;
    /// @notice The timestamp of the Beacon chain genesis.
    /// @dev this is different on Testnets like Hoodi so is set at deployment time.
    uint64 internal immutable BEACON_GENESIS_TIMESTAMP;

    /// @notice Address of the registrator - allowed to register, withdraw, exit and remove validators
    address public validatorRegistrator;

    /// @notice Deposit data for new compounding validators.
    /// @dev A `VERIFIED` deposit can mean 3 separate things:
    ///      - a deposit has been processed by the beacon chain and shall be included in the
    ///        balance of the next verifyBalances call
    ///      - a deposit has been done to a slashed validator and has probably been recovered
    ///        back to this strategy. Probably because we can not know for certain. This contract
    ///        only detects when the validator has passed its withdrawal epoch. It is close to impossible
    ///        to prove with Merkle Proofs that the postponed deposit this contract is responsible for
    ///        creating is not present anymore in BeaconChain.state.pending_deposits. This in effect
    ///        means that there might be a period where this contract thinks the deposit has been already
    ///        returned as ETH balance before it happens. This will result in some days (or weeks)
    ///        -> depending on the size of deposit queue of showing a deficit when calling `checkBalance`.
    ///        As this only offsets the yield and doesn't cause a critical double-counting we are not addressing
    ///        this issue.
    ///      - A deposit has been done to the validator, but our deposit has been front run by a malicious
    ///        actor. Funds in the deposit this contract makes are not recoverable.
    enum DepositStatus {
        UNKNOWN, // default value
        PENDING, // deposit is pending and waiting to be  verified
        VERIFIED // deposit has been verified
    }

    /// @param pubKeyHash Hash of validator's public key using the Beacon Chain's format
    /// @param amountGwei Amount of ETH in gwei that has been deposited to the beacon chain deposit contract
    /// @param slot The beacon chain slot number when the deposit has been made
    /// @param depositIndex The index of the deposit in the list of active deposits
    /// @param status The status of the deposit, either UNKNOWN, PENDING or VERIFIED
    struct DepositData {
        bytes32 pubKeyHash;
        uint64 amountGwei;
        uint64 slot;
        uint32 depositIndex;
        DepositStatus status;
    }
    /// @notice Restricts to only one deposit to an unverified validator at a time.
    /// This is to limit front-running attacks of deposits to the beacon chain contract.
    ///
    /// @dev The value is set to true when a deposit to a new validator has been done that has
    /// not yet be verified.
    bool public firstDeposit;
    /// @notice Mapping of the pending deposit roots to the deposit data
    mapping(bytes32 => DepositData) public deposits;
    /// @notice List of strategy deposit IDs to a validator.
    /// The ID is the merkle root of the pending deposit data which is unique for each validator, amount and block.
    /// Duplicate pending deposit roots are prevented so can be used as an identifier to each strategy deposit.
    /// The list can be for deposits waiting to be verified as processed on the beacon chain,
    /// or deposits that have been verified to an exiting validator and is now waiting for the
    /// validator's balance to be swept.
    /// The list may not be ordered by time of deposit.
    /// Removed deposits will move the last deposit to the removed index.
    bytes32[] public depositList;

    enum ValidatorState {
        NON_REGISTERED, // validator is not registered on the SSV network
        REGISTERED, // validator is registered on the SSV network
        STAKED, // validator has funds staked
        VERIFIED, // validator has been verified to exist on the beacon chain
        ACTIVE, // The validator balance is at least 32 ETH. The validator may not yet be active on the beacon chain.
        EXITING, // The validator has been requested to exit
        EXITED, // The validator has been verified to have a zero balance
        REMOVED, // validator has funds withdrawn to this strategy contract and is removed from the SSV
        INVALID // The validator has been front-run and the withdrawal address is not this strategy
    }

    // Validator data
    struct ValidatorData {
        ValidatorState state; // The state of the validator known to this contract
        uint40 index; // The index of the validator on the beacon chain
    }
    /// @notice List of validator public key hashes that have been verified to exist on the beacon chain.
    /// These have had a deposit processed and the validator's balance increased.
    /// Validators will be removed from this list when its verified they have a zero balance.
    bytes32[] public verifiedValidators;
    /// @notice Mapping of the hash of the validator's public key to the validator state and index.
    /// Uses the Beacon chain hashing for BLSPubkey which is sha256(abi.encodePacked(validator.pubkey, bytes16(0)))
    mapping(bytes32 => ValidatorData) public validator;

    /// @param blockRoot Beacon chain block root of the snapshot
    /// @param timestamp Timestamp of the snapshot
    /// @param ethBalance The balance of ETH in the strategy contract at the snapshot
    struct Balances {
        bytes32 blockRoot;
        uint64 timestamp;
        uint128 ethBalance;
    }
    /// @notice Mapping of the block root to the balances at that slot
    Balances public snappedBalance;
    /// @notice The last verified ETH balance of the strategy
    uint256 public lastVerifiedEthBalance;

    /// @dev This contract receives WETH as the deposit asset, but unlike other strategies doesn't immediately
    /// deposit it to an underlying platform. Rather a special privilege account stakes it to the validators.
    /// For that reason calling WETH.balanceOf(this) in a deposit function can contain WETH that has just been
    /// deposited and also WETH that has previously been deposited. To keep a correct count we need to keep track
    /// of WETH that has already been accounted for.
    /// This value represents the amount of WETH balance of this contract that has already been accounted for by the
    /// deposit events.
    /// It is important to note that this variable is not concerned with WETH that is a result of full/partial
    /// withdrawal of the validators. It is strictly concerned with WETH that has been deposited and is waiting to
    /// be staked.
    uint256 public depositedWethAccountedFor;

    // For future use
    uint256[41] private __gap;

    event RegistratorChanged(address indexed newAddress);
    event FirstDepositReset();
    event SSVValidatorRegistered(
        bytes32 indexed pubKeyHash,
        uint64[] operatorIds
    );
    event SSVValidatorRemoved(bytes32 indexed pubKeyHash, uint64[] operatorIds);
    event ETHStaked(
        bytes32 indexed pubKeyHash,
        bytes32 indexed pendingDepositRoot,
        bytes pubKey,
        uint256 amountWei
    );
    event ValidatorVerified(
        bytes32 indexed pubKeyHash,
        uint40 indexed validatorIndex
    );
    event ValidatorInvalid(bytes32 indexed pubKeyHash);
    event DepositVerified(
        bytes32 indexed pendingDepositRoot,
        uint256 amountWei
    );
    event ValidatorWithdraw(bytes32 indexed pubKeyHash, uint256 amountWei);
    event BalancesSnapped(bytes32 indexed blockRoot, uint256 ethBalance);
    event BalancesVerified(
        uint64 indexed timestamp,
        uint256 totalDepositsWei,
        uint256 totalValidatorBalance,
        uint256 ethBalance
    );

    /// @dev Throws if called by any account other than the Registrator
    modifier onlyRegistrator() {
        _onlyRegistrator();
        _;
    }

    /// @dev internal function used to reduce contract size
    function _onlyRegistrator() internal view {
        require(msg.sender == validatorRegistrator, "Not Registrator");
    }

    /// @dev Throws if called by any account other than the Registrator or Governor
    modifier onlyRegistratorOrGovernor() {
        require(
            msg.sender == validatorRegistrator || isGovernor(),
            "Not Registrator or Governor"
        );
        _;
    }

    /// @param _wethAddress Address of the Erc20 WETH Token contract
    /// @param _vaultAddress Address of the Vault
    /// @param _beaconChainDepositContract Address of the beacon chain deposit contract
    /// @param _ssvNetwork Address of the SSV Network contract
    /// @param _beaconProofs Address of the Beacon Proofs contract that verifies beacon chain data
    /// @param _beaconGenesisTimestamp The timestamp of the Beacon chain's genesis.
    constructor(
        address _wethAddress,
        address _vaultAddress,
        address _beaconChainDepositContract,
        address _ssvNetwork,
        address _beaconProofs,
        uint64 _beaconGenesisTimestamp
    ) {
        WETH = _wethAddress;
        BEACON_CHAIN_DEPOSIT_CONTRACT = _beaconChainDepositContract;
        SSV_NETWORK = _ssvNetwork;
        VAULT_ADDRESS = _vaultAddress;
        BEACON_PROOFS = _beaconProofs;
        BEACON_GENESIS_TIMESTAMP = _beaconGenesisTimestamp;

        require(
            block.timestamp > _beaconGenesisTimestamp,
            "Invalid genesis timestamp"
        );
    }

    /**
     *
     *             Admin Functions
     *
     */

    /// @notice Set the address of the registrator which can register, exit and remove validators
    function setRegistrator(address _address) external onlyGovernor {
        validatorRegistrator = _address;
        emit RegistratorChanged(_address);
    }

    /// @notice Reset the `firstDeposit` flag to false so deposits to unverified validators can be made again.
    function resetFirstDeposit() external onlyGovernor {
        require(firstDeposit, "No first deposit");

        firstDeposit = false;

        emit FirstDepositReset();
    }

    function pause() external onlyRegistratorOrGovernor {
        _pause();
    }

    function unPause() external onlyGovernor {
        _unpause();
    }

    /**
     *
     *             Validator Management
     *
     */

    /// @notice Registers a single validator in a SSV Cluster.
    /// Only the Registrator can call this function.
    /// @param publicKey The public key of the validator
    /// @param operatorIds The operator IDs of the SSV Cluster
    /// @param sharesData The shares data for the validator
    /// @param cluster The SSV cluster details including the validator count and SSV balance
    // slither-disable-start reentrancy-no-eth
    function registerSsvValidator(
        bytes calldata publicKey,
        uint64[] calldata operatorIds,
        bytes calldata sharesData,
        Cluster calldata cluster
    ) external payable onlyRegistrator whenNotPaused {
        // Hash the public key using the Beacon Chain's format
        bytes32 pubKeyHash = _hashPubKey(publicKey);
        // Check each public key has not already been used
        require(
            validator[pubKeyHash].state == ValidatorState.NON_REGISTERED,
            "Validator already registered"
        );

        // Store the validator state as registered
        validator[pubKeyHash].state = ValidatorState.REGISTERED;

        ISSVNetwork(SSV_NETWORK).registerValidator{ value: msg.value }(
            publicKey,
            operatorIds,
            sharesData,
            cluster
        );

        emit SSVValidatorRegistered(pubKeyHash, operatorIds);
    }

    // slither-disable-end reentrancy-no-eth

    struct ValidatorStakeData {
        bytes pubkey;
        bytes signature;
        bytes32 depositDataRoot;
    }

    /// @notice Stakes WETH in this strategy to a compounding validator.
    /// The first deposit to a new validator, the amount must be 1 ETH.
    /// Another deposit of at least 31 ETH is required for the validator to be activated.
    /// This second deposit has to be done after the validator has been verified.
    /// Does not convert any ETH sitting in this strategy to WETH.
    /// There can not be two deposits to the same validator in the same block for the same amount.
    /// Function is pausable so in case a run-away Registrator can be prevented from continuing
    /// to deposit funds to slashed or undesired validators.
    /// @param validatorStakeData validator data needed to stake.
    /// The `ValidatorStakeData` struct contains the pubkey, signature and depositDataRoot.
    /// Only the registrator can call this function.
    /// @param depositAmountGwei The amount of WETH to stake to the validator in Gwei.
    // slither-disable-start reentrancy-eth,reentrancy-no-eth
    function stakeEth(
        ValidatorStakeData calldata validatorStakeData,
        uint64 depositAmountGwei
    ) external onlyRegistrator whenNotPaused {
        uint256 depositAmountWei = uint256(depositAmountGwei) * 1 gwei;
        // Check there is enough WETH from the deposits sitting in this strategy contract
        // There could be ETH from withdrawals but we'll ignore that. If it's really needed
        // the ETH can be withdrawn and then deposited back to the strategy.
        require(
            depositAmountWei <= IWETH9(WETH).balanceOf(address(this)),
            "Insufficient WETH"
        );
        require(depositList.length < MAX_DEPOSITS, "Max deposits");

        // Convert required ETH from WETH and do the necessary accounting
        _convertWethToEth(depositAmountWei);

        // Hash the public key using the Beacon Chain's hashing for BLSPubkey
        bytes32 pubKeyHash = _hashPubKey(validatorStakeData.pubkey);
        ValidatorState currentState = validator[pubKeyHash].state;
        // Can only stake to a validator that has been registered, verified or active.
        // Can not stake to a validator that has been staked but not yet verified.
        require(
            (currentState == ValidatorState.REGISTERED ||
                currentState == ValidatorState.VERIFIED ||
                currentState == ValidatorState.ACTIVE),
            "Not registered or verified"
        );
        require(depositAmountWei >= 1 ether, "Deposit too small");
        if (currentState == ValidatorState.REGISTERED) {
            // Can only have one pending deposit to an unverified validator at a time.
            // This is to limit front-running deposit attacks to a single deposit.
            // The exiting deposit needs to be verified before another deposit can be made.
            // If there was a front-running attack, the validator needs to be verified as invalid
            // and the Governor calls `resetFirstDeposit` to set `firstDeposit` to false.
            require(!firstDeposit, "Existing first deposit");
            // Limits the amount of ETH that can be at risk from a front-running deposit attack.
            require(
                depositAmountWei == DEPOSIT_AMOUNT_WEI,
                "Invalid first deposit amount"
            );
            // Limits the number of validator balance proofs to verifyBalances
            require(
                verifiedValidators.length + 1 <= MAX_VERIFIED_VALIDATORS,
                "Max validators"
            );

            // Flag a deposit to an unverified validator so no other deposits can be made
            // to an unverified validator.
            firstDeposit = true;
            validator[pubKeyHash].state = ValidatorState.STAKED;
        }

        /* 0x02 to indicate that withdrawal credentials are for a compounding validator
         * that was introduced with the Pectra upgrade.
         * bytes11(0) to fill up the required zeros
         * remaining bytes20 are for the address
         */
        bytes memory withdrawalCredentials = abi.encodePacked(
            bytes1(0x02),
            bytes11(0),
            address(this)
        );

        /// After the Pectra upgrade the validators have a new restriction when proposing
        /// blocks. The timestamps are at strict intervals of 12 seconds from the genesis block
        /// forward. Each slot is created at strict 12 second intervals and those slots can
        /// either have blocks attached to them or not. This way using the block.timestamp
        /// the slot number can easily be calculated.
        uint64 depositSlot = (SafeCast.toUint64(block.timestamp) -
            BEACON_GENESIS_TIMESTAMP) / SLOT_DURATION;

        // Calculate the merkle root of the beacon chain pending deposit data.
        // This is used as the unique ID of the deposit.
        bytes32 pendingDepositRoot = IBeaconProofs(BEACON_PROOFS)
            .merkleizePendingDeposit(
                pubKeyHash,
                withdrawalCredentials,
                depositAmountGwei,
                validatorStakeData.signature,
                depositSlot
            );
        require(
            deposits[pendingDepositRoot].status == DepositStatus.UNKNOWN,
            "Duplicate deposit"
        );

        // Store the deposit data for verifyDeposit and verifyBalances
        deposits[pendingDepositRoot] = DepositData({
            pubKeyHash: pubKeyHash,
            amountGwei: depositAmountGwei,
            slot: depositSlot,
            depositIndex: SafeCast.toUint32(depositList.length),
            status: DepositStatus.PENDING
        });
        depositList.push(pendingDepositRoot);

        // Deposit to the Beacon Chain deposit contract.
        // This will create a deposit in the beacon chain's pending deposit queue.
        IDepositContract(BEACON_CHAIN_DEPOSIT_CONTRACT).deposit{
            value: depositAmountWei
        }(
            validatorStakeData.pubkey,
            withdrawalCredentials,
            validatorStakeData.signature,
            validatorStakeData.depositDataRoot
        );

        emit ETHStaked(
            pubKeyHash,
            pendingDepositRoot,
            validatorStakeData.pubkey,
            depositAmountWei
        );
    }

    // slither-disable-end reentrancy-eth,reentrancy-no-eth

    /// @notice Request a full or partial withdrawal from a validator.
    /// A zero amount will trigger a full withdrawal.
    /// If the remaining balance is < 32 ETH then only the amount in excess of 32 ETH will be withdrawn.
    /// Only the Registrator can call this function.
    /// 1 wei of value should be sent with the tx to pay for the withdrawal request fee.
    /// If no value sent, 1 wei will be taken from the strategy's ETH balance if it has any.
    /// If no ETH balance, the tx will revert.
    /// @param publicKey The public key of the validator
    /// @param amountGwei The amount of ETH to be withdrawn from the validator in Gwei.
    /// A zero amount will trigger a full withdrawal.
    // slither-disable-start reentrancy-no-eth
    function validatorWithdrawal(bytes calldata publicKey, uint64 amountGwei)
        external
        payable
        onlyRegistrator
    {
        // Hash the public key using the Beacon Chain's format
        bytes32 pubKeyHash = _hashPubKey(publicKey);
        ValidatorData memory validatorDataMem = validator[pubKeyHash];
        // Validator full withdrawal could be denied due to multiple reasons:
        //  - the validator has not been activated or active long enough
        //    (current_epoch < activation_epoch + SHARD_COMMITTEE_PERIOD)
        //  - the validator has pending balance to withdraw from a previous partial withdrawal request
        //
        // Meaning that the on-chain to beacon chain full withdrawal request could fail. Instead
        // of adding complexity of verifying if a validator is eligible for a full exit, we allow
        // multiple full withdrawal requests per validator.
        require(
            validatorDataMem.state == ValidatorState.ACTIVE ||
                validatorDataMem.state == ValidatorState.EXITING,
            "Validator not active/exiting"
        );

        // If a full withdrawal (validator exit)
        if (amountGwei == 0) {
            // For each staking strategy's deposits
            uint256 depositsCount = depositList.length;
            for (uint256 i = 0; i < depositsCount; ++i) {
                bytes32 pendingDepositRoot = depositList[i];
                // Check there is no pending deposits to the exiting validator
                require(
                    pubKeyHash != deposits[pendingDepositRoot].pubKeyHash,
                    "Pending deposit"
                );
            }

            // Store the validator state as exiting so no more deposits can be made to it.
            // This may already be EXITING if the previous exit request failed. eg the validator
            // was not active long enough.
            validator[pubKeyHash].state = ValidatorState.EXITING;
        }

        // Do not remove from the list of verified validators.
        // This is done in the verifyBalances function once the validator's balance has been verified to be zero.
        // The validator state will be set to EXITED in the verifyBalances function.

        PartialWithdrawal.request(publicKey, amountGwei);

        emit ValidatorWithdraw(pubKeyHash, uint256(amountGwei) * 1 gwei);
    }

    // slither-disable-end reentrancy-no-eth

    /// @notice Remove the validator from the SSV Cluster after:
    /// - the validator has been exited from `validatorWithdrawal` or slashed
    /// - the validator has incorrectly registered and can not be staked to
    /// - the initial deposit was front-run and the withdrawal address is not this strategy's address.
    /// Make sure `validatorWithdrawal` is called with a zero amount and the validator has exited the Beacon chain.
    /// If removed before the validator has exited the beacon chain will result in the validator being slashed.
    /// Only the registrator can call this function.
    /// @param publicKey The public key of the validator
    /// @param operatorIds The operator IDs of the SSV Cluster
    /// @param cluster The SSV cluster details including the validator count and SSV balance
    // slither-disable-start reentrancy-no-eth
    function removeSsvValidator(
        bytes calldata publicKey,
        uint64[] calldata operatorIds,
        Cluster calldata cluster
    ) external onlyRegistrator {
        // Hash the public key using the Beacon Chain's format
        bytes32 pubKeyHash = _hashPubKey(publicKey);
        ValidatorState currentState = validator[pubKeyHash].state;
        // Can remove SSV validators that were incorrectly registered and can not be deposited to.
        require(
            currentState == ValidatorState.REGISTERED ||
                currentState == ValidatorState.EXITED ||
                currentState == ValidatorState.INVALID,
            "Validator not regd or exited"
        );

        validator[pubKeyHash].state = ValidatorState.REMOVED;

        ISSVNetwork(SSV_NETWORK).removeValidator(
            publicKey,
            operatorIds,
            cluster
        );

        emit SSVValidatorRemoved(pubKeyHash, operatorIds);
    }

    /**
     *
     *             SSV Management
     *
     */

    // slither-disable-end reentrancy-no-eth

    /// @notice Migrate the SSV cluster to use ETH for payment instead of SSV tokens.
    /// @param operatorIds The operator IDs of the SSV Cluster
    /// @param cluster The SSV cluster details including the validator count and SSV balance
    function migrateClusterToETH(
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external payable onlyGovernor {
        ISSVNetwork(SSV_NETWORK).migrateClusterToETH{ value: msg.value }(
            operatorIds,
            cluster
        );

        // The SSV Network emits
        // ClusterMigratedToETH(msg.sender, operatorIds, msg.value, ssvClusterBalance, effectiveBalance, cluster)
    }

    /**
     *
     *             Beacon Chain Proofs
     *
     */

    /// @notice Verifies a validator's index to its public key.
    /// Adds to the list of verified validators if the validator's withdrawal address is this strategy's address.
    /// Marks the validator as invalid and removes the deposit if the withdrawal address is not this strategy's address.
    /// @param nextBlockTimestamp The timestamp of the execution layer block after the beacon chain slot
    /// we are verifying.
    /// The next one is needed as the Beacon Oracle returns the parent beacon block root for a block timestamp,
    /// which is the beacon block root of the previous block.
    /// @param validatorIndex The index of the validator on the beacon chain.
    /// @param pubKeyHash The hash of the validator's public key using the Beacon Chain's format
    /// @param withdrawalCredentials contain the validator type and withdrawal address. These can be incorrect and/or
    ///        malformed. In case of incorrect withdrawalCredentials the validator deposit has been front run
    /// @param validatorPubKeyProof The merkle proof for the validator public key to the beacon block root.
    /// This is 53 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    /// BeaconBlock.state.validators[validatorIndex].pubkey
    function verifyValidator(
        uint64 nextBlockTimestamp,
        uint40 validatorIndex,
        bytes32 pubKeyHash,
        bytes32 withdrawalCredentials,
        bytes calldata validatorPubKeyProof
    ) external {
        require(
            validator[pubKeyHash].state == ValidatorState.STAKED,
            "Validator not staked"
        );

        // Get the beacon block root of the slot we are verifying the validator in.
        // The parent beacon block root of the next block is the beacon block root of the slot we are verifying.
        bytes32 blockRoot = BeaconRoots.parentBlockRoot(nextBlockTimestamp);

        // Verify the validator index is for the validator with the given public key.
        // Also verify the validator's withdrawal credentials
        IBeaconProofs(BEACON_PROOFS).verifyValidator(
            blockRoot,
            pubKeyHash,
            validatorPubKeyProof,
            validatorIndex,
            withdrawalCredentials
        );

        // Store the validator state as verified
        validator[pubKeyHash] = ValidatorData({
            state: ValidatorState.VERIFIED,
            index: validatorIndex
        });

        bytes32 expectedWithdrawalCredentials = bytes32(
            abi.encodePacked(bytes1(0x02), bytes11(0), address(this))
        );

        // If the initial deposit was front-run and the withdrawal address is not this strategy
        // or the validator type is not a compounding validator (0x02)
        if (expectedWithdrawalCredentials != withdrawalCredentials) {
            // override the validator state
            validator[pubKeyHash].state = ValidatorState.INVALID;

            // Find and remove the deposit as the funds can not be recovered
            uint256 depositCount = depositList.length;
            for (uint256 i = 0; i < depositCount; i++) {
                DepositData memory deposit = deposits[depositList[i]];
                if (deposit.pubKeyHash == pubKeyHash) {
                    // next verifyBalances will correctly account for the loss of a front-run
                    // deposit. Doing it here accounts for the loss as soon as possible
                    lastVerifiedEthBalance -= Math.min(
                        lastVerifiedEthBalance,
                        uint256(deposit.amountGwei) * 1 gwei
                    );
                    _removeDeposit(depositList[i], deposit);
                    break;
                }
            }

            // Leave the `firstDeposit` flag as true so no more deposits to unverified validators can be made.
            // The Governor has to reset the `firstDeposit` to false before another deposit to
            // an unverified validator can be made.
            // The Governor can set a new `validatorRegistrator` if they suspect it has been compromised.

            emit ValidatorInvalid(pubKeyHash);
            return;
        }

        // Add the new validator to the list of verified validators
        verifiedValidators.push(pubKeyHash);

        // Reset the firstDeposit flag as the first deposit to an unverified validator has been verified.
        firstDeposit = false;

        emit ValidatorVerified(pubKeyHash, validatorIndex);
    }

    struct FirstPendingDepositSlotProofData {
        uint64 slot;
        bytes proof;
    }

    struct StrategyValidatorProofData {
        uint64 withdrawableEpoch;
        bytes withdrawableEpochProof;
    }

    /// @notice Verifies a deposit on the execution layer has been processed by the beacon chain.
    /// This means the accounting of the strategy's ETH moves from a pending deposit to a validator balance.
    ///
    /// Important: this function has a limitation where `depositProcessedSlot` that is passed by the off-chain
    /// verifier requires a slot immediately after it to propose a block otherwise the `BeaconRoots.parentBlockRoot`
    /// will fail. This shouldn't be a problem, since by the current behaviour of beacon chain only 1%-3% slots
    /// don't propose a block.
    /// @param pendingDepositRoot The unique identifier of the deposit emitted in `ETHStaked` from
    /// the `stakeEth` function.
    /// @param depositProcessedSlot Any slot on or after the strategy's deposit was processed on the beacon chain.
    /// Can not be a slot with pending deposits with the same slot as the deposit being verified.
    /// Can not be a slot before a missed slot as the Beacon Root contract will have the parent block root
    /// set for the next block timestamp in 12 seconds time.
    /// @param firstPendingDeposit a `FirstPendingDepositSlotProofData` struct containing:
    /// - slot: The beacon chain slot of the first deposit in the beacon chain's deposit queue.
    ///   Can be any non-zero value if the deposit queue is empty.
    /// - proof: The merkle proof of the first pending deposit's slot to the beacon block root.
    ///   Can be either:
    ///   * 40 witness hashes for BeaconBlock.state.PendingDeposits[0].slot when the deposit queue is not empty.
    ///   * 37 witness hashes for BeaconBlock.state.PendingDeposits[0] when the deposit queue is empty.
    ///   The 32 byte witness hashes are concatenated together starting from the leaf node.
    /// @param strategyValidatorData a `StrategyValidatorProofData` struct containing:
    /// - withdrawableEpoch: The withdrawable epoch of the validator the strategy is depositing to.
    /// - withdrawableEpochProof: The merkle proof for the withdrawable epoch of the validator the strategy
    ///   is depositing to, to the beacon block root.
    ///   This is 53 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    // slither-disable-start reentrancy-no-eth
    function verifyDeposit(
        bytes32 pendingDepositRoot,
        uint64 depositProcessedSlot,
        FirstPendingDepositSlotProofData calldata firstPendingDeposit,
        StrategyValidatorProofData calldata strategyValidatorData
    ) external {
        // Load into memory the previously saved deposit data
        DepositData memory deposit = deposits[pendingDepositRoot];
        ValidatorData memory strategyValidator = validator[deposit.pubKeyHash];
        require(deposit.status == DepositStatus.PENDING, "Deposit not pending");
        require(firstPendingDeposit.slot != 0, "Zero 1st pending deposit slot");

        // We should allow the verification of deposits for validators that have been marked as exiting
        // to cover this situation:
        //  - there are 2 pending deposits
        //  - beacon chain has slashed the validator
        //  - when verifyDeposit is called for the first deposit it sets the Validator state to EXITING
        //  - verifyDeposit should allow a secondary call for the other deposit to a slashed validator
        require(
            strategyValidator.state == ValidatorState.VERIFIED ||
                strategyValidator.state == ValidatorState.ACTIVE ||
                strategyValidator.state == ValidatorState.EXITING,
            "Not verified/active/exiting"
        );
        // The verification slot must be after the deposit's slot.
        // This is needed for when the deposit queue is empty.
        require(deposit.slot < depositProcessedSlot, "Slot not after deposit");

        uint64 snapTimestamp = snappedBalance.timestamp;

        // This check prevents an accounting error that can happen if:
        //  - snapBalances are snapped at the time of T
        //  - deposit is processed on the beacon chain after time T and before verifyBalances()
        //  - verifyDeposit is called before verifyBalances which removes a deposit from depositList
        //    and deposit balance from totalDepositsWei
        //  - verifyBalances is called under-reporting the strategy's balance
        require(
            (_calcNextBlockTimestamp(depositProcessedSlot) <= snapTimestamp) ||
                snapTimestamp == 0,
            "Deposit after balance snapshot"
        );

        // Get the parent beacon block root of the next block which is the block root of the deposit verification slot.
        // This will revert if the slot after the verification slot was missed.
        bytes32 depositBlockRoot = BeaconRoots.parentBlockRoot(
            _calcNextBlockTimestamp(depositProcessedSlot)
        );

        // Verify the slot of the first pending deposit matches the beacon chain
        bool isDepositQueueEmpty = IBeaconProofs(BEACON_PROOFS)
            .verifyFirstPendingDeposit(
                depositBlockRoot,
                firstPendingDeposit.slot,
                firstPendingDeposit.proof
            );

        // Verify the withdrawableEpoch on the validator of the strategy's deposit
        IBeaconProofs(BEACON_PROOFS).verifyValidatorWithdrawable(
            depositBlockRoot,
            strategyValidator.index,
            strategyValidatorData.withdrawableEpoch,
            strategyValidatorData.withdrawableEpochProof
        );

        uint64 firstPendingDepositEpoch = firstPendingDeposit.slot /
            SLOTS_PER_EPOCH;

        // If deposit queue is empty all deposits have certainly been processed. If not
        // a validator can either be not exiting and no further checks are required.
        // Or a validator is exiting then this function needs to make sure that the
        // pending deposit to an exited validator has certainly been processed. The
        // slot/epoch of first pending deposit is the one that contains the transaction
        // where the deposit to the ETH Deposit Contract has been made.
        //
        // Once the firstPendingDepositEpoch becomes greater than the withdrawableEpoch of
        // the slashed validator then the deposit has certainly been processed. When the beacon
        // chain reaches the withdrawableEpoch of the validator the deposit will no longer be
        // postponed. And any new deposits created (and present in the deposit queue)
        // will have an equal or larger withdrawableEpoch.
        require(
            strategyValidatorData.withdrawableEpoch == FAR_FUTURE_EPOCH ||
                strategyValidatorData.withdrawableEpoch <=
                firstPendingDepositEpoch ||
                isDepositQueueEmpty,
            "Exit Deposit likely not proc."
        );

        // solhint-disable max-line-length
        // Check the deposit slot is before the first pending deposit's slot on the beacon chain.
        // If this is not true then we can't guarantee the deposit has been processed by the beacon chain.
        // The deposit's slot can not be the same slot as the first pending deposit as there could be
        // many deposits in the same block, hence have the same pending deposit slot.
        // If the deposit queue is empty then our deposit must have been processed on the beacon chain.
        // The deposit slot can be zero for validators consolidating to a compounding validator or 0x01 validator
        // being promoted to a compounding one. Reference:
        // - [switch_to_compounding_validator](https://ethereum.github.io/consensus-specs/specs/electra/beacon-chain/#new-switch_to_compounding_validator
        // - [queue_excess_active_balance](https://ethereum.github.io/consensus-specs/specs/electra/beacon-chain/#new-queue_excess_active_balance)
        // - [process_consolidation_request](https://ethereum.github.io/consensus-specs/specs/electra/beacon-chain/#new-process_consolidation_request)
        // We can not guarantee that the deposit has been processed in that case.
        // solhint-enable max-line-length
        require(
            deposit.slot < firstPendingDeposit.slot || isDepositQueueEmpty,
            "Deposit likely not processed"
        );

        // Remove the deposit now it has been verified as processed on the beacon chain.
        _removeDeposit(pendingDepositRoot, deposit);

        emit DepositVerified(
            pendingDepositRoot,
            uint256(deposit.amountGwei) * 1 gwei
        );
    }

    function _removeDeposit(
        bytes32 pendingDepositRoot,
        DepositData memory deposit
    ) internal {
        // After verifying the proof, update the contract storage
        deposits[pendingDepositRoot].status = DepositStatus.VERIFIED;
        // Move the last deposit to the index of the verified deposit
        bytes32 lastDeposit = depositList[depositList.length - 1];
        depositList[deposit.depositIndex] = lastDeposit;
        deposits[lastDeposit].depositIndex = deposit.depositIndex;
        // Delete the last deposit from the list
        depositList.pop();
    }

    /// @dev Calculates the timestamp of the next execution block from the given slot.
    /// @param slot The beacon chain slot number used for merkle proof verification.
    function _calcNextBlockTimestamp(uint64 slot)
        internal
        view
        returns (uint64)
    {
        // Calculate the next block timestamp from the slot.
        return SLOT_DURATION * slot + BEACON_GENESIS_TIMESTAMP + SLOT_DURATION;
    }

    // slither-disable-end reentrancy-no-eth

    /// @notice Stores the current ETH balance at the current block and beacon block root
    ///         of the slot that is associated with the previous block.
    ///
    /// When snapping / verifying balance it is of a high importance that there is no
    /// miss-match in respect to ETH that is held by the contract and balances that are
    /// verified on the validators.
    ///
    /// First some context on the beacon-chain block building behaviour. Relevant parts of
    /// constructing a block on the beacon chain consist of:
    ///  - process_withdrawals: ETH is deducted from the validator's balance
    ///  - process_execution_payload: immediately after the previous step executing all the
    ///    transactions
    ///  - apply the withdrawals: adding ETH to the recipient which is the withdrawal address
    ///    contained in the withdrawal credentials of the exited validators
    ///
    /// That means that balance increases which are part of the post-block execution state are
    /// done within the block, but the transaction that are contained within that block can not
    /// see / interact with the balance from the exited validators. Only transactions in the
    /// next block can do that.
    ///
    /// When snap balances is performed the state of the chain is snapped across 2 separate
    /// chain states:
    ///  - ETH balance of the contract is recorded on block X -> and corresponding slot Y
    ///  - beacon chain block root is recorded of block X - 1 -> and corresponding slot Y - 1
    ///    given there were no missed slots. It could also be Y - 2, Y - 3 depending on how
    ///    many slots have not managed to propose a block. For the sake of simplicity this slot
    ///    will be referred to as Y - 1 as it makes no difference in the argument
    ///
    /// Given these 2 separate chain states it is paramount that verify balances can not experience
    /// miss-counting ETH or much more dangerous double counting of the ETH.
    ///
    /// When verifyBalances is called it is performed on the current block Z where Z > X. Verify
    /// balances adds up all the ETH (omitting WETH) controlled by this contract:
    ///  - ETH balance in the contract on block X
    ///  - ETH balance in Deposits on block Z that haven't been yet processed in slot Y - 1
    ///  - ETH balance in validators that are active in slot Y - 1
    ///  - skips the ETH balance in validators that have withdrawn in slot Y - 1 (or sooner)
    ///    and have their balance visible to transactions in slot Y and corresponding block X
    ///    (or sooner)
    ///
    /// Lets verify the correctness of ETH accounting given the above described behaviour.
    ///
    /// *ETH balance in the contract on block X*
    ///
    /// This is an ETH balance of the contract on a non current X block. Any ETH leaving the
    /// contract as a result of a withdrawal subtracts from the ETH accounted for on block X
    /// if `verifyBalances` has already been called. It also invalidates a `snapBalances` in
    /// case `verifyBalances` has not been called yet. Not performing this would result in not
    /// accounting for the withdrawn ETH that has happened anywhere in the block interval [X + 1, Z].
    ///
    /// Similarly to withdrawals any `stakeEth` deposits to the deposit contract adds to the ETH
    /// accounted for since the last `verifyBalances` has been called. And it invalidates the
    /// `snapBalances` in case `verifyBalances` hasn't been yet called. Not performing this
    /// would result in double counting the `stakedEth` since it would be present once in the
    /// snapped contract balance and the second time in deposit storage variables.
    ///
    /// This behaviour is correct.
    ///
    /// *ETH balance in Deposits on block Z that haven't been yet processed in slot Y - 1*
    ///
    /// The contract sums up all the ETH that has been deposited to the Beacon chain deposit
    /// contract at block Z. The execution layer doesn't have direct access to the state of
    /// deposits on the beacon chain. And if it is to sum up all the ETH that is marked to be
    /// deposited it needs to be sure to not double count ETH that is in deposits (storage vars)
    /// and could also be part of the validator balances. It does that by verifying that at
    /// slot Y - 1 none of the deposits visible on block Z have been processed. Meaning since
    /// the last snap till now all are still in queue. Which ensures they can not be part of
    /// the validator balances in later steps.
    ///
    /// This behaviour is correct.
    ///
    /// *ETH balance in validators that are active in slot Y - 1*
    ///
    /// The contract is verifying none of the deposits on Y - 1 slot have been processed and
    /// for that reason it checks the validator balances in the same slot. Ensuring accounting
    /// correctness.
    ///
    /// This behaviour is correct.
    ///
    /// *The withdrawn validators*
    ///
    /// The withdrawn validators could have their balances deducted in any slot before slot
    /// Y - 1 and the execution layer sees the balance increase in the subsequent slot. Lets
    /// look at the "worst case scenario" where the validator withdrawal is processed in the
    /// slot Y - 1 (snapped slot) and see their balance increase (in execution layer) in slot
    /// Y -> block X. The ETH balance on the contract is snapped at block X meaning that
    /// even if the validator exits at the latest possible time it is paramount that the ETH
    /// balance on the execution layer is recorded in the next block. Correctly accounting
    /// for the withdrawn ETH.
    ///
    /// Worth mentioning if the validator exit is processed by the slot Y and balance increase
    /// seen on the execution layer on block X + 1 the withdrawal is ignored by both the
    /// validator balance verification as well as execution layer contract balance snap.
    ///
    /// This behaviour is correct.
    ///
    /// The validator balances on the beacon chain can then be proved with `verifyBalances`.
    function snapBalances() external onlyRegistrator {
        uint64 currentTimestamp = SafeCast.toUint64(block.timestamp);
        require(
            snappedBalance.timestamp + SNAP_BALANCES_DELAY < currentTimestamp,
            "Snap too soon"
        );

        bytes32 blockRoot = BeaconRoots.parentBlockRoot(currentTimestamp);
        // Get the current ETH balance
        uint256 ethBalance = address(this).balance;

        // Store the snapped balance
        snappedBalance = Balances({
            blockRoot: blockRoot,
            timestamp: currentTimestamp,
            ethBalance: SafeCast.toUint128(ethBalance)
        });

        emit BalancesSnapped(blockRoot, ethBalance);
    }

    // A struct is used to avoid stack too deep errors
    struct BalanceProofs {
        // BeaconBlock.state.balances
        bytes32 balancesContainerRoot;
        bytes balancesContainerProof;
        // BeaconBlock.state.balances[validatorIndex]
        bytes32[] validatorBalanceLeaves;
        bytes[] validatorBalanceProofs;
    }

    struct PendingDepositProofs {
        bytes32 pendingDepositContainerRoot;
        bytes pendingDepositContainerProof;
        uint32[] pendingDepositIndexes;
        bytes[] pendingDepositProofs;
    }

    /// @notice Verifies the balances of all active validators on the beacon chain
    /// and checks each of the strategy's deposits are still to be processed by the beacon chain.
    /// @param balanceProofs a `BalanceProofs` struct containing the following:
    /// - balancesContainerRoot: The merkle root of the balances container
    /// - balancesContainerProof: The merkle proof for the balances container to the beacon block root.
    ///    This is 9 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    /// - validatorBalanceLeaves: Array of leaf nodes containing the validator balance with three other balances.
    /// - validatorBalanceProofs: Array of merkle proofs for the validator balance to the Balances container root.
    ///    This is 39 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    /// @param pendingDepositProofs a `PendingDepositProofs` struct containing the following:
    /// - pendingDepositContainerRoot: The merkle root of the pending deposits list container
    /// - pendingDepositContainerProof: The merkle proof from the pending deposits list container
    ///     to the beacon block root.
    ///    This is 9 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    /// - pendingDepositIndexes: Array of indexes in the pending deposits list container for each
    ///    of the strategy's deposits.
    /// - pendingDepositProofs: Array of merkle proofs for each strategy deposit in the
    ///    beacon chain's pending deposit list container to the pending deposits list container root.
    ///    These are 28 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    // slither-disable-start reentrancy-no-eth
    function verifyBalances(
        BalanceProofs calldata balanceProofs,
        PendingDepositProofs calldata pendingDepositProofs
    ) external onlyRegistrator {
        // Load previously snapped balances for the given block root
        Balances memory balancesMem = snappedBalance;
        // Check the balances are the latest
        require(balancesMem.timestamp > 0, "No snapped balances");

        uint256 verifiedValidatorsCount = verifiedValidators.length;
        uint256 totalValidatorBalance = 0;
        uint256 depositsCount = depositList.length;

        // If there are no verified validators then we can skip the balance verification
        if (verifiedValidatorsCount > 0) {
            require(
                balanceProofs.validatorBalanceProofs.length ==
                    verifiedValidatorsCount,
                "Invalid balance proofs"
            );
            require(
                balanceProofs.validatorBalanceLeaves.length ==
                    verifiedValidatorsCount,
                "Invalid balance leaves"
            );
            // verify beaconBlock.state.balances root to beacon block root
            IBeaconProofs(BEACON_PROOFS).verifyBalancesContainer(
                balancesMem.blockRoot,
                balanceProofs.balancesContainerRoot,
                balanceProofs.balancesContainerProof
            );

            bytes32[]
                memory validatorHashesMem = _getPendingDepositValidatorHashes(
                    depositsCount
                );

            // for each validator in reverse order so we can pop off exited validators at the end
            for (uint256 i = verifiedValidatorsCount; i > 0; ) {
                --i;
                ValidatorData memory validatorDataMem = validator[
                    verifiedValidators[i]
                ];
                // verify validator's balance in beaconBlock.state.balances to the
                // beaconBlock.state.balances container root
                uint256 validatorBalanceGwei = IBeaconProofs(BEACON_PROOFS)
                    .verifyValidatorBalance(
                        balanceProofs.balancesContainerRoot,
                        balanceProofs.validatorBalanceLeaves[i],
                        balanceProofs.validatorBalanceProofs[i],
                        validatorDataMem.index
                    );

                // If the validator has exited and the balance is now zero
                if (validatorBalanceGwei == 0) {
                    // Check if there are any pending deposits to this validator
                    bool depositPending = false;
                    for (uint256 j = 0; j < validatorHashesMem.length; j++) {
                        if (validatorHashesMem[j] == verifiedValidators[i]) {
                            depositPending = true;
                            break;
                        }
                    }

                    // If validator has a pending deposit we can not remove due to
                    // the following situation:
                    //  - validator has a pending deposit
                    //  - validator has been slashed
                    //  - sweep cycle has withdrawn all ETH from the validator. Balance is 0
                    //  - beacon chain has processed the deposit and set the validator balance
                    //    to deposit amount
                    //  - if validator is no longer in the list of verifiedValidators its
                    //    balance will not be considered and be under-counted.
                    if (!depositPending) {
                        // Store the validator state as exited
                        // This could have been in VERIFIED, ACTIVE or EXITING state
                        validator[verifiedValidators[i]].state = ValidatorState
                            .EXITED;

                        // Remove the validator with a zero balance from the list of verified validators

                        // Reduce the count of verified validators which is the last index before the pop removes it.
                        verifiedValidatorsCount -= 1;

                        // Move the last validator that has already been verified to the current index.
                        // There's an extra SSTORE if i is the last active validator but that's fine,
                        // It's not a common case and the code is simpler this way.
                        verifiedValidators[i] = verifiedValidators[
                            verifiedValidatorsCount
                        ];
                        // Delete the last validator from the list
                        verifiedValidators.pop();
                    }

                    // The validator balance is zero so not need to add to totalValidatorBalance
                    continue;
                } else if (
                    validatorDataMem.state == ValidatorState.VERIFIED &&
                    validatorBalanceGwei > MIN_ACTIVATION_BALANCE_GWEI
                ) {
                    // Store the validator state as active. This does not necessarily mean the
                    // validator is active on the beacon chain yet. It just means the validator has
                    // enough balance that it can become active.
                    validator[verifiedValidators[i]].state = ValidatorState
                        .ACTIVE;
                }

                // convert Gwei balance to Wei and add to the total validator balance
                totalValidatorBalance += validatorBalanceGwei * 1 gwei;
            }
        }

        uint256 totalDepositsWei = 0;

        // If there are no deposits then we can skip the deposit verification.
        // This section is after the validator balance verifications so an exited validator will be marked
        // as EXITED before the deposits are verified. If there was a deposit to an exited validator
        // then the deposit can only be removed once the validator is fully exited.
        // It is possible that validator fully exits and a postponed deposit to an exited validator increases
        // its balance again. In such case the contract will erroneously consider a deposit applied before it
        // has been applied on the beacon chain showing a smaller than real `totalValidatorBalance`.
        if (depositsCount > 0) {
            require(
                pendingDepositProofs.pendingDepositProofs.length ==
                    depositsCount,
                "Invalid deposit proofs"
            );
            require(
                pendingDepositProofs.pendingDepositIndexes.length ==
                    depositsCount,
                "Invalid deposit indexes"
            );

            // Verify from the root of the pending deposit list container to the beacon block root
            IBeaconProofs(BEACON_PROOFS).verifyPendingDepositsContainer(
                balancesMem.blockRoot,
                pendingDepositProofs.pendingDepositContainerRoot,
                pendingDepositProofs.pendingDepositContainerProof
            );

            // For each staking strategy's deposit.
            for (uint256 i = 0; i < depositsCount; ++i) {
                bytes32 pendingDepositRoot = depositList[i];

                // Verify the strategy's deposit is still pending on the beacon chain.
                IBeaconProofs(BEACON_PROOFS).verifyPendingDeposit(
                    pendingDepositProofs.pendingDepositContainerRoot,
                    pendingDepositRoot,
                    pendingDepositProofs.pendingDepositProofs[i],
                    pendingDepositProofs.pendingDepositIndexes[i]
                );

                // Convert the deposit amount from Gwei to Wei and add to the total
                totalDepositsWei +=
                    uint256(deposits[pendingDepositRoot].amountGwei) *
                    1 gwei;
            }
        }

        // Store the verified balance in storage
        lastVerifiedEthBalance =
            totalDepositsWei +
            totalValidatorBalance +
            balancesMem.ethBalance;
        // Reset the last snap timestamp so a new snapBalances has to be made
        snappedBalance.timestamp = 0;

        emit BalancesVerified(
            balancesMem.timestamp,
            totalDepositsWei,
            totalValidatorBalance,
            balancesMem.ethBalance
        );
    }

    // slither-disable-end reentrancy-no-eth

    /// @notice get a list of all validator hashes present in the pending deposits
    ///         list can have duplicate entries
    function _getPendingDepositValidatorHashes(uint256 depositsCount)
        internal
        view
        returns (bytes32[] memory validatorHashes)
    {
        validatorHashes = new bytes32[](depositsCount);
        for (uint256 i = 0; i < depositsCount; i++) {
            validatorHashes[i] = deposits[depositList[i]].pubKeyHash;
        }
    }

    /// @notice Hash a validator public key using the Beacon Chain's format
    function _hashPubKey(bytes memory pubKey) internal pure returns (bytes32) {
        require(pubKey.length == 48, "Invalid public key");
        return sha256(abi.encodePacked(pubKey, bytes16(0)));
    }

    /**
     *
     *         WETH and ETH Accounting
     *
     */

    /// @dev Called when WETH is transferred out of the strategy so
    /// the strategy knows how much WETH it has on deposit.
    /// This is so it can emit the correct amount in the Deposit event in depositAll().
    function _transferWeth(uint256 _amount, address _recipient) internal {
        IERC20(WETH).safeTransfer(_recipient, _amount);

        // The min is required as more WETH can be withdrawn than deposited
        // as the strategy earns consensus and execution rewards.
        uint256 deductAmount = Math.min(_amount, depositedWethAccountedFor);
        depositedWethAccountedFor -= deductAmount;

        // No change in ETH balance so no need to snapshot the balances
    }

    /// @dev Converts ETH to WETH and updates the accounting.
    /// @param _ethAmount The amount of ETH in wei.
    function _convertEthToWeth(uint256 _ethAmount) internal {
        // slither-disable-next-line arbitrary-send-eth
        IWETH9(WETH).deposit{ value: _ethAmount }();

        depositedWethAccountedFor += _ethAmount;

        // Store the reduced ETH balance.
        // The ETH balance in this strategy contract can be more than the last verified ETH balance
        // due to partial withdrawals or full exits being processed by the beacon chain since the last snapBalances.
        // It can also happen from execution rewards (MEV) or ETH donations.
        lastVerifiedEthBalance -= Math.min(lastVerifiedEthBalance, _ethAmount);

        // The ETH balance was decreased to WETH so we need to invalidate the last balances snap.
        snappedBalance.timestamp = 0;
    }

    /// @dev Converts WETH to ETH and updates the accounting.
    /// @param _wethAmount The amount of WETH in wei.
    function _convertWethToEth(uint256 _wethAmount) internal {
        IWETH9(WETH).withdraw(_wethAmount);

        uint256 deductAmount = Math.min(_wethAmount, depositedWethAccountedFor);
        depositedWethAccountedFor -= deductAmount;

        // Store the increased ETH balance
        lastVerifiedEthBalance += _wethAmount;

        // The ETH balance was increased from WETH so we need to invalidate the last balances snap.
        snappedBalance.timestamp = 0;
    }

    /**
     *
     *             View Functions
     *
     */

    /// @notice Returns the number of deposits waiting to be verified as processed on the beacon chain,
    /// or deposits that have been verified to an exiting validator and is now waiting for the
    /// validator's balance to be swept.
    function depositListLength() external view returns (uint256) {
        return depositList.length;
    }

    /// @notice Returns the number of verified validators.
    function verifiedValidatorsLength() external view returns (uint256) {
        return verifiedValidators.length;
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

interface IBeaconProofs {
    function verifyValidator(
        bytes32 beaconBlockRoot,
        bytes32 pubKeyHash,
        bytes calldata validatorPubKeyProof,
        uint40 validatorIndex,
        bytes32 withdrawalCredentials
    ) external view;

    function verifyValidatorWithdrawable(
        bytes32 beaconBlockRoot,
        uint40 validatorIndex,
        uint64 withdrawableEpoch,
        bytes calldata withdrawableEpochProof
    ) external view;

    function verifyBalancesContainer(
        bytes32 beaconBlockRoot,
        bytes32 balancesContainerLeaf,
        bytes calldata balancesContainerProof
    ) external view;

    function verifyValidatorBalance(
        bytes32 balancesContainerRoot,
        bytes32 validatorBalanceLeaf,
        bytes calldata balanceProof,
        uint40 validatorIndex
    ) external view returns (uint256 validatorBalance);

    function verifyPendingDepositsContainer(
        bytes32 beaconBlockRoot,
        bytes32 pendingDepositsContainerRoot,
        bytes calldata proof
    ) external view;

    function verifyPendingDeposit(
        bytes32 pendingDepositsContainerRoot,
        bytes32 pendingDepositRoot,
        bytes calldata proof,
        uint32 pendingDepositIndex
    ) external view;

    function verifyFirstPendingDeposit(
        bytes32 beaconBlockRoot,
        uint64 slot,
        bytes calldata firstPendingDepositSlotProof
    ) external view returns (bool isEmptyDepositQueue);

    function merkleizePendingDeposit(
        bytes32 pubKeyHash,
        bytes calldata withdrawalCredentials,
        uint64 amountGwei,
        bytes calldata signature,
        uint64 slot
    ) external pure returns (bytes32 root);

    function merkleizeSignature(bytes calldata signature)
        external
        pure
        returns (bytes32 root);
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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IBasicToken {
    function symbol() external view returns (string memory);

    function decimals() external view returns (uint8);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IDepositContract {
    /// @notice A processed deposit event.
    event DepositEvent(
        bytes pubkey,
        bytes withdrawal_credentials,
        bytes amount,
        bytes signature,
        bytes index
    );

    /// @notice Submit a Phase 0 DepositData object.
    /// @param pubkey A BLS12-381 public key.
    /// @param withdrawal_credentials Commitment to a public key for withdrawals.
    /// @param signature A BLS12-381 signature.
    /// @param deposit_data_root The SHA-256 hash of the SSZ-encoded DepositData object.
    /// Used as a protection against malformed input.
    function deposit(
        bytes calldata pubkey,
        bytes calldata withdrawal_credentials,
        bytes calldata signature,
        bytes32 deposit_data_root
    ) external payable;

    /// @notice Query the current deposit root hash.
    /// @return The deposit root hash.
    function get_deposit_root() external view returns (bytes32);

    /// @notice Query the current deposit count.
    /// @return The deposit count encoded as a little endian 64-bit number.
    function get_deposit_count() external view returns (bytes memory);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

struct Cluster {
    uint32 validatorCount;
    uint64 networkFeeIndex;
    uint64 index;
    bool active;
    uint256 balance;
}

interface ISSVNetwork {
    /**********/
    /* Errors */
    /**********/

    error CallerNotOwner(); // 0x5cd83192
    error CallerNotWhitelisted(); // 0x8c6e5d71
    error FeeTooLow(); // 0x732f9413
    error FeeExceedsIncreaseLimit(); // 0x958065d9
    error NoFeeDeclared(); // 0x1d226c30
    error ApprovalNotWithinTimeframe(); // 0x97e4b518
    error OperatorDoesNotExist(); // 0x961e3e8c
    error InsufficientBalance(); // 0xf4d678b8
    error ValidatorDoesNotExist(); // 0xe51315d2
    error ClusterNotLiquidatable(); // 0x60300a8d
    error InvalidPublicKeyLength(); // 0x637297a4
    error InvalidOperatorIdsLength(); // 0x38186224
    error ClusterAlreadyEnabled(); // 0x3babafd2
    error ClusterIsLiquidated(); // 0x95a0cf33
    error ClusterDoesNotExists(); // 0x185e2b16
    error IncorrectClusterState(); // 0x12e04c87
    error UnsortedOperatorsList(); // 0xdd020e25
    error NewBlockPeriodIsBelowMinimum(); // 0x6e6c9cac
    error ExceedValidatorLimit(); // 0x6df5ab76
    error TokenTransferFailed(); // 0x045c4b02
    error SameFeeChangeNotAllowed(); // 0xc81272f8
    error FeeIncreaseNotAllowed(); // 0x410a2b6c
    error NotAuthorized(); // 0xea8e4eb5
    error OperatorsListNotUnique(); // 0xa5a1ff5d
    error OperatorAlreadyExists(); // 0x289c9494
    error TargetModuleDoesNotExist(); // 0x8f9195fb
    error MaxValueExceeded(); // 0x91aa3017
    error FeeTooHigh(); // 0xcd4e6167
    error PublicKeysSharesLengthMismatch(); // 0x9ad467b8
    error IncorrectValidatorStateWithData(bytes publicKey); // 0x89307938
    error ValidatorAlreadyExistsWithData(bytes publicKey); // 0x388e7999
    error EmptyPublicKeysList(); // df83e679

    // legacy errors
    error ValidatorAlreadyExists(); // 0x8d09a73e
    error IncorrectValidatorState(); // 0x2feda3c1

    event AdminChanged(address previousAdmin, address newAdmin);
    event BeaconUpgraded(address indexed beacon);
    event ClusterDeposited(
        address indexed owner,
        uint64[] operatorIds,
        uint256 value,
        Cluster cluster
    );
    event ClusterLiquidated(
        address indexed owner,
        uint64[] operatorIds,
        Cluster cluster
    );
    event ClusterReactivated(
        address indexed owner,
        uint64[] operatorIds,
        Cluster cluster
    );
    event ClusterWithdrawn(
        address indexed owner,
        uint64[] operatorIds,
        uint256 value,
        Cluster cluster
    );
    event DeclareOperatorFeePeriodUpdated(uint64 value);
    event ExecuteOperatorFeePeriodUpdated(uint64 value);
    event FeeRecipientAddressUpdated(
        address indexed owner,
        address recipientAddress
    );
    event Initialized(uint8 version);
    event LiquidationThresholdPeriodUpdated(uint64 value);
    event MinimumLiquidationCollateralUpdated(uint256 value);
    event NetworkEarningsWithdrawn(uint256 value, address recipient);
    event NetworkFeeUpdated(uint256 oldFee, uint256 newFee);
    event OperatorAdded(
        uint64 indexed operatorId,
        address indexed owner,
        bytes publicKey,
        uint256 fee
    );
    event OperatorFeeDeclarationCancelled(
        address indexed owner,
        uint64 indexed operatorId
    );
    event OperatorFeeDeclared(
        address indexed owner,
        uint64 indexed operatorId,
        uint256 blockNumber,
        uint256 fee
    );
    event OperatorFeeExecuted(
        address indexed owner,
        uint64 indexed operatorId,
        uint256 blockNumber,
        uint256 fee
    );
    event OperatorFeeIncreaseLimitUpdated(uint64 value);
    event OperatorMaximumFeeUpdated(uint64 maxFee);
    event OperatorRemoved(uint64 indexed operatorId);
    event OperatorWhitelistUpdated(
        uint64 indexed operatorId,
        address whitelisted
    );
    event OperatorWithdrawn(
        address indexed owner,
        uint64 indexed operatorId,
        uint256 value
    );
    event OwnershipTransferStarted(
        address indexed previousOwner,
        address indexed newOwner
    );
    event OwnershipTransferred(
        address indexed previousOwner,
        address indexed newOwner
    );
    event Upgraded(address indexed implementation);
    event ValidatorAdded(
        address indexed owner,
        uint64[] operatorIds,
        bytes publicKey,
        bytes shares,
        Cluster cluster
    );
    event ValidatorExited(
        address indexed owner,
        uint64[] operatorIds,
        bytes publicKey
    );
    event ValidatorRemoved(
        address indexed owner,
        uint64[] operatorIds,
        bytes publicKey,
        Cluster cluster
    );

    fallback() external;

    function acceptOwnership() external;

    function cancelDeclaredOperatorFee(uint64 operatorId) external;

    function declareOperatorFee(uint64 operatorId, uint256 fee) external;

    function deposit(
        address clusterOwner,
        uint64[] memory operatorIds,
        uint256 amount,
        Cluster memory cluster
    ) external;

    function executeOperatorFee(uint64 operatorId) external;

    function exitValidator(bytes memory publicKey, uint64[] memory operatorIds)
        external;

    function bulkExitValidator(
        bytes[] calldata publicKeys,
        uint64[] calldata operatorIds
    ) external;

    function getVersion() external pure returns (string memory version);

    function initialize(
        address token_,
        address ssvOperators_,
        address ssvClusters_,
        address ssvDAO_,
        address ssvViews_,
        uint64 minimumBlocksBeforeLiquidation_,
        uint256 minimumLiquidationCollateral_,
        uint32 validatorsPerOperatorLimit_,
        uint64 declareOperatorFeePeriod_,
        uint64 executeOperatorFeePeriod_,
        uint64 operatorMaxFeeIncrease_
    ) external;

    function liquidate(
        address clusterOwner,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external;

    function owner() external view returns (address);

    function pendingOwner() external view returns (address);

    function proxiableUUID() external view returns (bytes32);

    function reactivate(
        uint64[] memory operatorIds,
        uint256 amount,
        Cluster memory cluster
    ) external;

    function reduceOperatorFee(uint64 operatorId, uint256 fee) external;

    function registerOperator(bytes memory publicKey, uint256 fee)
        external
        returns (uint64 id);

    function registerValidator(
        bytes memory publicKey,
        uint64[] memory operatorIds,
        bytes memory sharesData,
        Cluster memory cluster
    ) external payable;

    function bulkRegisterValidator(
        bytes[] calldata publicKeys,
        uint64[] calldata operatorIds,
        bytes[] calldata sharesData,
        Cluster memory cluster
    ) external payable;

    function migrateClusterToETH(
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external payable;

    function removeOperator(uint64 operatorId) external;

    function removeValidator(
        bytes memory publicKey,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external;

    function bulkRemoveValidator(
        bytes[] calldata publicKeys,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external;

    function renounceOwnership() external;

    function setFeeRecipientAddress(address recipientAddress) external;

    function setOperatorWhitelist(uint64 operatorId, address whitelisted)
        external;

    function transferOwnership(address newOwner) external;

    function updateDeclareOperatorFeePeriod(uint64 timeInSeconds) external;

    function updateExecuteOperatorFeePeriod(uint64 timeInSeconds) external;

    function updateLiquidationThresholdPeriod(uint64 blocks) external;

    function updateMaximumOperatorFee(uint64 maxFee) external;

    function updateMinimumLiquidationCollateral(uint256 amount) external;

    function updateModule(uint8 moduleId, address moduleAddress) external;

    function updateNetworkFee(uint256 fee) external;

    function updateOperatorFeeIncreaseLimit(uint64 percentage) external;

    function upgradeTo(address newImplementation) external;

    function upgradeToAndCall(address newImplementation, bytes memory data)
        external
        payable;

    function withdraw(
        uint64[] memory operatorIds,
        uint256 amount,
        Cluster memory cluster
    ) external;

    function withdrawAllOperatorEarnings(uint64 operatorId) external;

    function withdrawNetworkEarnings(uint256 amount) external;

    function withdrawOperatorEarnings(uint64 operatorId, uint256 amount)
        external;
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity =0.7.6;

import '@openzeppelin/contracts/token/ERC20/IERC20.sol';

/// @title Interface for WETH9
interface IWETH9 is IERC20 {
    /// @notice Deposit ether to get wrapped ether
    function deposit() external payable;

    /// @notice Withdraw wrapped ether to get ether
    function withdraw(uint256) external;
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


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IVault } from "./IVault.sol";

interface IMockVault is IVault {
    function outstandingWithdrawalsAmount() external view returns (uint256);

    function wethAvailable() external view returns (uint256);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { BeaconProofsLib } from "./BeaconProofsLib.sol";
import { IBeaconProofs } from "../interfaces/IBeaconProofs.sol";

/**
 * @title Verifies merkle proofs of beacon chain data.
 * @author Origin Protocol Inc
 */
contract BeaconProofs is IBeaconProofs {
    /// @notice Verifies the validator index is for the given validator public key.
    /// Also verify the validator's withdrawal credential points to the withdrawal address.
    /// BeaconBlock.state.validators[validatorIndex].pubkey
    /// @param beaconBlockRoot The root of the beacon block
    /// @param pubKeyHash Hash of validator's public key using the Beacon Chain's format
    /// @param proof The merkle proof for the validator public key to the beacon block root.
    /// This is 53 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    /// @param validatorIndex The validator index
    /// @param withdrawalCredentials a value containing the validator type and withdrawal address.
    function verifyValidator(
        bytes32 beaconBlockRoot,
        bytes32 pubKeyHash,
        bytes calldata proof,
        uint40 validatorIndex,
        bytes32 withdrawalCredentials
    ) external view {
        BeaconProofsLib.verifyValidator(
            beaconBlockRoot,
            pubKeyHash,
            proof,
            validatorIndex,
            withdrawalCredentials
        );
    }

    function verifyValidatorWithdrawable(
        bytes32 beaconBlockRoot,
        uint40 validatorIndex,
        uint64 withdrawableEpoch,
        bytes calldata withdrawableEpochProof
    ) external view {
        BeaconProofsLib.verifyValidatorWithdrawableEpoch(
            beaconBlockRoot,
            validatorIndex,
            withdrawableEpoch,
            withdrawableEpochProof
        );
    }

    /// @notice Verifies the balances container to the beacon block root
    /// BeaconBlock.state.balances
    /// @param beaconBlockRoot The root of the beacon block
    /// @param balancesContainerRoot The merkle root of the the balances container
    /// @param balancesContainerProof The merkle proof for the balances container to the beacon block root.
    /// This is 9 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    function verifyBalancesContainer(
        bytes32 beaconBlockRoot,
        bytes32 balancesContainerRoot,
        bytes calldata balancesContainerProof
    ) external view {
        BeaconProofsLib.verifyBalancesContainer(
            beaconBlockRoot,
            balancesContainerRoot,
            balancesContainerProof
        );
    }

    /// @notice Verifies the validator balance to the root of the Balances container.
    /// @param balancesContainerRoot The merkle root of the Balances container.
    /// @param validatorBalanceLeaf The leaf node containing the validator balance with three other balances.
    /// @param balanceProof The merkle proof for the validator balance to the Balances container root.
    /// This is 39 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    /// @param validatorIndex The validator index to verify the balance for
    /// @return validatorBalanceGwei The balance in Gwei of the validator at the given index
    function verifyValidatorBalance(
        bytes32 balancesContainerRoot,
        bytes32 validatorBalanceLeaf,
        bytes calldata balanceProof,
        uint40 validatorIndex
    ) external view returns (uint256 validatorBalanceGwei) {
        validatorBalanceGwei = BeaconProofsLib.verifyValidatorBalance(
            balancesContainerRoot,
            validatorBalanceLeaf,
            balanceProof,
            validatorIndex
        );
    }

    /// @notice Verifies the pending deposits container to the beacon block root.
    /// BeaconBlock.state.pendingDeposits
    /// @param beaconBlockRoot The root of the beacon block.
    /// @param pendingDepositsContainerRoot The merkle root of the the pending deposits container.
    /// @param proof The merkle proof for the pending deposits container to the beacon block root.
    /// This is 9 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    function verifyPendingDepositsContainer(
        bytes32 beaconBlockRoot,
        bytes32 pendingDepositsContainerRoot,
        bytes calldata proof
    ) external view {
        BeaconProofsLib.verifyPendingDepositsContainer(
            beaconBlockRoot,
            pendingDepositsContainerRoot,
            proof
        );
    }

    /// @notice Verified a pending deposit to the root of the Pending Deposits container.
    /// @param pendingDepositsContainerRoot The merkle root of the Pending Deposits container.
    /// @param pendingDepositRoot The leaf node containing the validator balance with three other balances.
    /// @param proof The merkle proof for the pending deposit root to the Pending Deposits container root.
    /// This is 28 witness hashes of 32 bytes each concatenated together starting from the leaf node.
    /// @param pendingDepositIndex The pending deposit index in the Pending Deposits container
    function verifyPendingDeposit(
        bytes32 pendingDepositsContainerRoot,
        bytes32 pendingDepositRoot,
        bytes calldata proof,
        uint32 pendingDepositIndex
    ) external view {
        BeaconProofsLib.verifyPendingDeposit(
            pendingDepositsContainerRoot,
            pendingDepositRoot,
            proof,
            pendingDepositIndex
        );
    }

    /// @notice If the deposit queue is not empty,
    /// verify the slot of the first pending deposit to the beacon block root.
    /// BeaconBlock.state.pendingDeposits[0].slot
    /// If the deposit queue is empty, verify the root of the first pending deposit is empty
    /// BeaconBlock.state.PendingDeposits[0]
    /// @param beaconBlockRoot The root of the beacon block.
    /// @param slot The beacon chain slot of the first deposit in the beacon chain's deposit queue.
    /// Can be anything if the deposit queue is empty.
    /// @param firstPendingDepositSlotProof The merkle proof to the beacon block root. Can be either:
    /// - 40 witness hashes for BeaconBlock.state.PendingDeposits[0].slot when the deposit queue is not empty.
    /// - 37 witness hashes for BeaconBlock.state.PendingDeposits[0] when the deposit queue is empty.
    /// The 32 byte witness hashes are concatenated together starting from the leaf node.
    /// @return isEmptyDepositQueue True if the deposit queue is empty, false otherwise.
    function verifyFirstPendingDeposit(
        bytes32 beaconBlockRoot,
        uint64 slot,
        bytes calldata firstPendingDepositSlotProof
    ) external view returns (bool isEmptyDepositQueue) {
        isEmptyDepositQueue = BeaconProofsLib.verifyFirstPendingDeposit(
            beaconBlockRoot,
            slot,
            firstPendingDepositSlotProof
        );
    }

    /// @notice Merkleizes a beacon chain pending deposit.
    /// @param pubKeyHash Hash of validator's public key using the Beacon Chain's format
    /// @param withdrawalCredentials The 32 byte withdrawal credentials.
    /// @param amountGwei The amount of the deposit in Gwei.
    /// @param signature The 96 byte BLS signature.
    /// @param slot The beacon chain slot the deposit was made in.
    /// @return root The merkle root of the pending deposit.
    function merkleizePendingDeposit(
        bytes32 pubKeyHash,
        bytes calldata withdrawalCredentials,
        uint64 amountGwei,
        bytes calldata signature,
        uint64 slot
    ) external pure returns (bytes32) {
        return
            BeaconProofsLib.merkleizePendingDeposit(
                pubKeyHash,
                withdrawalCredentials,
                amountGwei,
                signature,
                slot
            );
    }

    /// @notice Merkleizes a BLS signature used for validator deposits.
    /// @param signature The 96 byte BLS signature.
    /// @return root The merkle root of the signature.
    function merkleizeSignature(bytes calldata signature)
        external
        pure
        returns (bytes32 root)
    {
        return BeaconProofsLib.merkleizeSignature(signature);
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

