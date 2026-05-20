
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title Generalized 4626 Strategy
 * @notice Investment strategy for ERC-4626 Tokenized Vaults
 * @dev This strategy should not be used for the Morpho V2 Vaults as those are not
 *      completley ERC-4626 compliant - they don't implement the maxWithdraw() and
 *      maxRedeem() functions and rather return 0 when any of them is called.
 * @author Origin Protocol Inc
 */
import { IERC4626 } from "../../lib/openzeppelin/interfaces/IERC4626.sol";
import { IERC20, InitializableAbstractStrategy } from "../utils/InitializableAbstractStrategy.sol";
import { IDistributor } from "../interfaces/IMerkl.sol";

contract Generalized4626Strategy is InitializableAbstractStrategy {
    /// @notice The address of the Merkle Distributor contract.
    IDistributor public constant merkleDistributor =
        IDistributor(0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae);

    /// @dev Replaced with an immutable variable
    // slither-disable-next-line constable-states
    address private _deprecate_shareToken;
    /// @dev Replaced with an immutable variable
    // slither-disable-next-line constable-states
    address private _deprecate_assetToken;

    IERC20 public immutable shareToken;
    IERC20 public immutable assetToken;

    // For future use
    uint256[50] private __gap;

    event ClaimedRewards(address indexed token, uint256 amount);

    /**
     * @param _baseConfig Base strategy config with platformAddress (ERC-4626 Vault contract), eg sfrxETH or sDAI,
     * and vaultAddress (OToken Vault contract), eg VaultProxy or OETHVaultProxy
     * @param _assetToken Address of the ERC-4626 asset token. eg frxETH or DAI
     */
    constructor(BaseStrategyConfig memory _baseConfig, address _assetToken)
        InitializableAbstractStrategy(_baseConfig)
    {
        shareToken = IERC20(_baseConfig.platformAddress);
        assetToken = IERC20(_assetToken);
    }

    function initialize() external virtual onlyGovernor initializer {
        address[] memory rewardTokens = new address[](0);
        address[] memory assets = new address[](1);
        address[] memory pTokens = new address[](1);

        assets[0] = address(assetToken);
        pTokens[0] = address(platformAddress);

        InitializableAbstractStrategy._initialize(
            rewardTokens,
            assets,
            pTokens
        );
    }

    /**
     * @dev Deposit assets by converting them to shares
     * @param _asset Address of asset to deposit
     * @param _amount Amount of asset to deposit
     */
    function deposit(address _asset, uint256 _amount)
        external
        virtual
        override
        onlyVault
        nonReentrant
    {
        _deposit(_asset, _amount);
    }

    /**
     * @dev Deposit assets by converting them to shares
     * @param _asset Address of asset to deposit
     * @param _amount Amount of asset to deposit
     */
    function _deposit(address _asset, uint256 _amount) internal virtual {
        require(_amount > 0, "Must deposit something");
        require(_asset == address(assetToken), "Unexpected asset address");

        // slither-disable-next-line unused-return
        IERC4626(platformAddress).deposit(_amount, address(this));
        emit Deposit(_asset, address(shareToken), _amount);
    }

    /**
     * @dev Deposit the entire balance of assetToken to gain shareToken
     */
    function depositAll() external virtual override onlyVault nonReentrant {
        uint256 balance = assetToken.balanceOf(address(this));
        if (balance > 0) {
            _deposit(address(assetToken), balance);
        }
    }

    /**
     * @dev Withdraw asset by burning shares
     * @param _recipient Address to receive withdrawn asset
     * @param _asset Address of asset to withdraw
     * @param _amount Amount of asset to withdraw
     */
    function withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) external virtual override onlyVault nonReentrant {
        _withdraw(_recipient, _asset, _amount);
    }

    function _withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) internal virtual {
        require(_amount > 0, "Must withdraw something");
        require(_recipient != address(0), "Must specify recipient");
        require(_asset == address(assetToken), "Unexpected asset address");

        // slither-disable-next-line unused-return
        IERC4626(platformAddress).withdraw(_amount, _recipient, address(this));
        emit Withdrawal(_asset, address(shareToken), _amount);
    }

    /**
     * @dev Internal method to respond to the addition of new asset / share tokens
     */
    function _abstractSetPToken(address, address) internal virtual override {
        _approveBase();
    }

    /**
     * @dev Remove all assets from platform and send them to Vault contract.
     */
    function withdrawAll()
        external
        virtual
        override
        onlyVaultOrGovernor
        nonReentrant
    {
        // @dev Don't use for Morpho V2 Vaults as below line will return 0
        uint256 sharesToRedeem = IERC4626(platformAddress).maxRedeem(
            address(this)
        );

        uint256 assetAmount = 0;
        if (sharesToRedeem > 0) {
            assetAmount = IERC4626(platformAddress).redeem(
                sharesToRedeem,
                vaultAddress,
                address(this)
            );
            emit Withdrawal(
                address(assetToken),
                address(shareToken),
                assetAmount
            );
        }
    }

    /**
     * @notice Get the total asset value held in the platform
     * @param _asset      Address of the asset
     * @return balance    Total value of the asset in the platform
     */
    function checkBalance(address _asset)
        public
        view
        virtual
        override
        returns (uint256 balance)
    {
        require(_asset == address(assetToken), "Unexpected asset address");
        /* We are intentionally not counting the amount of assetToken parked on the
         * contract toward the checkBalance. The deposit and withdraw functions
         * should not result in assetToken being unused and owned by this strategy
         * contract.
         */
        IERC4626 platform = IERC4626(platformAddress);
        return platform.previewRedeem(platform.balanceOf(address(this)));
    }

    /**
     * @notice Governor approves the ERC-4626 Tokenized Vault to spend the asset.
     */
    function safeApproveAllTokens() external override onlyGovernor {
        _approveBase();
    }

    function _approveBase() internal virtual {
        // Approval the asset to be transferred to the ERC-4626 Tokenized Vault.
        // Used by the ERC-4626 deposit() and mint() functions
        // slither-disable-next-line unused-return
        assetToken.approve(platformAddress, type(uint256).max);
    }

    /**
     * @dev Returns bool indicating whether asset is supported by strategy
     * @param _asset Address of the asset
     */
    function supportsAsset(address _asset)
        public
        view
        virtual
        override
        returns (bool)
    {
        return _asset == address(assetToken);
    }

    /**
     * @notice is not supported for this strategy as the asset and
     * ERC-4626 Tokenized Vault are set at deploy time.
     * @dev If the ERC-4626 Tokenized Vault needed to be changed, a new
     * contract would need to be deployed and the proxy updated.
     */
    function setPTokenAddress(address, address) external override onlyGovernor {
        revert("unsupported function");
    }

    /**
     * @notice is not supported for this strategy as the asset and
     * ERC-4626 Tokenized Vault are set at deploy time.
     * @dev If the ERC-4626 Tokenized Vault needed to be changed, a new
     * contract would need to be deployed and the proxy updated.
     */
    function removePToken(uint256) external override onlyGovernor {
        revert("unsupported function");
    }

    /// @notice Claim tokens from the Merkle Distributor
    /// @param token The address of the token to claim.
    /// @param amount The amount of tokens to claim.
    /// @param proof The Merkle proof to validate the claim.
    function merkleClaim(
        address token,
        uint256 amount,
        bytes32[] calldata proof
    ) external {
        address[] memory users = new address[](1);
        users[0] = address(this);

        address[] memory tokens = new address[](1);
        tokens[0] = token;

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = amount;

        bytes32[][] memory proofs = new bytes32[][](1);
        proofs[0] = proof;

        merkleDistributor.claim(users, tokens, amounts, proofs);

        emit ClaimedRewards(token, amount);
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
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

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.0;

/**
 * @dev Interface of the ERC20 standard as defined in the EIP.
 */
interface IERC20 {
    /**
     * @dev Returns the amount of tokens in existence.
     */
    function totalSupply() external view returns (uint256);

    /**
     * @dev Returns the amount of tokens owned by `account`.
     */
    function balanceOf(address account) external view returns (uint256);

    /**
     * @dev Moves `amount` tokens from the caller's account to `recipient`.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transfer(address recipient, uint256 amount) external returns (bool);

    /**
     * @dev Returns the remaining number of tokens that `spender` will be
     * allowed to spend on behalf of `owner` through {transferFrom}. This is
     * zero by default.
     *
     * This value changes when {approve} or {transferFrom} are called.
     */
    function allowance(address owner, address spender) external view returns (uint256);

    /**
     * @dev Sets `amount` as the allowance of `spender` over the caller's tokens.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * IMPORTANT: Beware that changing an allowance with this method brings the risk
     * that someone may use both the old and the new allowance by unfortunate
     * transaction ordering. One possible solution to mitigate this race
     * condition is to first reduce the spender's allowance to 0 and set the
     * desired value afterwards:
     * https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
     *
     * Emits an {Approval} event.
     */
    function approve(address spender, uint256 amount) external returns (bool);

    /**
     * @dev Moves `amount` tokens from `sender` to `recipient` using the
     * allowance mechanism. `amount` is then deducted from the caller's
     * allowance.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);

    /**
     * @dev Emitted when `value` tokens are moved from one account (`from`) to
     * another (`to`).
     *
     * Note that `value` may be zero.
     */
    event Transfer(address indexed from, address indexed to, uint256 value);

    /**
     * @dev Emitted when the allowance of a `spender` for an `owner` is set by
     * a call to {approve}. `value` is the new allowance.
     */
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

interface IDistributor {
    event Claimed(address indexed user, address indexed token, uint256 amount);

    function claim(
        address[] calldata users,
        address[] calldata tokens,
        uint256[] calldata amounts,
        bytes32[][] calldata proofs
    ) external;
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IERC4626 is IERC20, IERC20Metadata {
    event Deposit(address indexed caller, address indexed owner, uint256 assets, uint256 shares);

    event Withdraw(
        address indexed caller,
        address indexed receiver,
        address indexed owner,
        uint256 assets,
        uint256 shares
    );

    /**
     * @dev Returns the address of the underlying token used for the Vault for accounting, depositing, and withdrawing.
     *
     * - MUST be an ERC-20 token contract.
     * - MUST NOT revert.
     */
    function asset() external view returns (address assetTokenAddress);

    /**
     * @dev Returns the total amount of the underlying asset that is “managed” by Vault.
     *
     * - SHOULD include any compounding that occurs from yield.
     * - MUST be inclusive of any fees that are charged against assets in the Vault.
     * - MUST NOT revert.
     */
    function totalAssets() external view returns (uint256 totalManagedAssets);

    /**
     * @dev Returns the amount of shares that the Vault would exchange for the amount of assets provided, in an ideal
     * scenario where all the conditions are met.
     *
     * - MUST NOT be inclusive of any fees that are charged against assets in the Vault.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT reflect slippage or other on-chain conditions, when performing the actual exchange.
     * - MUST NOT revert.
     *
     * NOTE: This calculation MAY NOT reflect the “per-user” price-per-share, and instead should reflect the
     * “average-user’s” price-per-share, meaning what the average user should expect to see when exchanging to and
     * from.
     */
    function convertToShares(uint256 assets) external view returns (uint256 shares);

    /**
     * @dev Returns the amount of assets that the Vault would exchange for the amount of shares provided, in an ideal
     * scenario where all the conditions are met.
     *
     * - MUST NOT be inclusive of any fees that are charged against assets in the Vault.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT reflect slippage or other on-chain conditions, when performing the actual exchange.
     * - MUST NOT revert.
     *
     * NOTE: This calculation MAY NOT reflect the “per-user” price-per-share, and instead should reflect the
     * “average-user’s” price-per-share, meaning what the average user should expect to see when exchanging to and
     * from.
     */
    function convertToAssets(uint256 shares) external view returns (uint256 assets);

    /**
     * @dev Returns the maximum amount of the underlying asset that can be deposited into the Vault for the receiver,
     * through a deposit call.
     *
     * - MUST return a limited value if receiver is subject to some deposit limit.
     * - MUST return 2 ** 256 - 1 if there is no limit on the maximum amount of assets that may be deposited.
     * - MUST NOT revert.
     */
    function maxDeposit(address receiver) external view returns (uint256 maxAssets);

    /**
     * @dev Allows an on-chain or off-chain user to simulate the effects of their deposit at the current block, given
     * current on-chain conditions.
     *
     * - MUST return as close to and no more than the exact amount of Vault shares that would be minted in a deposit
     *   call in the same transaction. I.e. deposit should return the same or more shares as previewDeposit if called
     *   in the same transaction.
     * - MUST NOT account for deposit limits like those returned from maxDeposit and should always act as though the
     *   deposit would be accepted, regardless if the user has enough tokens approved, etc.
     * - MUST be inclusive of deposit fees. Integrators should be aware of the existence of deposit fees.
     * - MUST NOT revert.
     *
     * NOTE: any unfavorable discrepancy between convertToShares and previewDeposit SHOULD be considered slippage in
     * share price or some other type of condition, meaning the depositor will lose assets by depositing.
     */
    function previewDeposit(uint256 assets) external view returns (uint256 shares);

    /**
     * @dev Mints shares Vault shares to receiver by depositing exactly amount of underlying tokens.
     *
     * - MUST emit the Deposit event.
     * - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the
     *   deposit execution, and are accounted for during deposit.
     * - MUST revert if all of assets cannot be deposited (due to deposit limit being reached, slippage, the user not
     *   approving enough underlying tokens to the Vault contract, etc).
     *
     * NOTE: most implementations will require pre-approval of the Vault with the Vault’s underlying asset token.
     */
    function deposit(uint256 assets, address receiver) external returns (uint256 shares);

    /**
     * @dev Returns the maximum amount of the Vault shares that can be minted for the receiver, through a mint call.
     * - MUST return a limited value if receiver is subject to some mint limit.
     * - MUST return 2 ** 256 - 1 if there is no limit on the maximum amount of shares that may be minted.
     * - MUST NOT revert.
     */
    function maxMint(address receiver) external view returns (uint256 maxShares);

    /**
     * @dev Allows an on-chain or off-chain user to simulate the effects of their mint at the current block, given
     * current on-chain conditions.
     *
     * - MUST return as close to and no fewer than the exact amount of assets that would be deposited in a mint call
     *   in the same transaction. I.e. mint should return the same or fewer assets as previewMint if called in the
     *   same transaction.
     * - MUST NOT account for mint limits like those returned from maxMint and should always act as though the mint
     *   would be accepted, regardless if the user has enough tokens approved, etc.
     * - MUST be inclusive of deposit fees. Integrators should be aware of the existence of deposit fees.
     * - MUST NOT revert.
     *
     * NOTE: any unfavorable discrepancy between convertToAssets and previewMint SHOULD be considered slippage in
     * share price or some other type of condition, meaning the depositor will lose assets by minting.
     */
    function previewMint(uint256 shares) external view returns (uint256 assets);

    /**
     * @dev Mints exactly shares Vault shares to receiver by depositing amount of underlying tokens.
     *
     * - MUST emit the Deposit event.
     * - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the mint
     *   execution, and are accounted for during mint.
     * - MUST revert if all of shares cannot be minted (due to deposit limit being reached, slippage, the user not
     *   approving enough underlying tokens to the Vault contract, etc).
     *
     * NOTE: most implementations will require pre-approval of the Vault with the Vault’s underlying asset token.
     */
    function mint(uint256 shares, address receiver) external returns (uint256 assets);

    /**
     * @dev Returns the maximum amount of the underlying asset that can be withdrawn from the owner balance in the
     * Vault, through a withdraw call.
     *
     * - MUST return a limited value if owner is subject to some withdrawal limit or timelock.
     * - MUST NOT revert.
     */
    function maxWithdraw(address owner) external view returns (uint256 maxAssets);

    /**
     * @dev Allows an on-chain or off-chain user to simulate the effects of their withdrawal at the current block,
     * given current on-chain conditions.
     *
     * - MUST return as close to and no fewer than the exact amount of Vault shares that would be burned in a withdraw
     *   call in the same transaction. I.e. withdraw should return the same or fewer shares as previewWithdraw if
     *   called
     *   in the same transaction.
     * - MUST NOT account for withdrawal limits like those returned from maxWithdraw and should always act as though
     *   the withdrawal would be accepted, regardless if the user has enough shares, etc.
     * - MUST be inclusive of withdrawal fees. Integrators should be aware of the existence of withdrawal fees.
     * - MUST NOT revert.
     *
     * NOTE: any unfavorable discrepancy between convertToShares and previewWithdraw SHOULD be considered slippage in
     * share price or some other type of condition, meaning the depositor will lose assets by depositing.
     */
    function previewWithdraw(uint256 assets) external view returns (uint256 shares);

    /**
     * @dev Burns shares from owner and sends exactly assets of underlying tokens to receiver.
     *
     * - MUST emit the Withdraw event.
     * - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the
     *   withdraw execution, and are accounted for during withdraw.
     * - MUST revert if all of assets cannot be withdrawn (due to withdrawal limit being reached, slippage, the owner
     *   not having enough shares, etc).
     *
     * Note that some implementations will require pre-requesting to the Vault before a withdrawal may be performed.
     * Those methods should be performed separately.
     */
    function withdraw(
        uint256 assets,
        address receiver,
        address owner
    ) external returns (uint256 shares);

    /**
     * @dev Returns the maximum amount of Vault shares that can be redeemed from the owner balance in the Vault,
     * through a redeem call.
     *
     * - MUST return a limited value if owner is subject to some withdrawal limit or timelock.
     * - MUST return balanceOf(owner) if owner is not subject to any withdrawal limit or timelock.
     * - MUST NOT revert.
     */
    function maxRedeem(address owner) external view returns (uint256 maxShares);

    /**
     * @dev Allows an on-chain or off-chain user to simulate the effects of their redeemption at the current block,
     * given current on-chain conditions.
     *
     * - MUST return as close to and no more than the exact amount of assets that would be withdrawn in a redeem call
     *   in the same transaction. I.e. redeem should return the same or more assets as previewRedeem if called in the
     *   same transaction.
     * - MUST NOT account for redemption limits like those returned from maxRedeem and should always act as though the
     *   redemption would be accepted, regardless if the user has enough shares, etc.
     * - MUST be inclusive of withdrawal fees. Integrators should be aware of the existence of withdrawal fees.
     * - MUST NOT revert.
     *
     * NOTE: any unfavorable discrepancy between convertToAssets and previewRedeem SHOULD be considered slippage in
     * share price or some other type of condition, meaning the depositor will lose assets by redeeming.
     */
    function previewRedeem(uint256 shares) external view returns (uint256 assets);

    /**
     * @dev Burns exactly shares from owner and sends assets of underlying tokens to receiver.
     *
     * - MUST emit the Withdraw event.
     * - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the
     *   redeem execution, and are accounted for during redeem.
     * - MUST revert if all of shares cannot be redeemed (due to withdrawal limit being reached, slippage, the owner
     *   not having enough shares, etc).
     *
     * NOTE: some implementations will require pre-requesting to the Vault before a withdrawal may be performed.
     * Those methods should be performed separately.
     */
    function redeem(
        uint256 shares,
        address receiver,
        address owner
    ) external returns (uint256 assets);
}

## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { WOETH } from "./WOETH.sol";

/**
 * @title Wrapped Origin Sonic (wOS) token on Sonic
 * @author Origin Protocol Inc
 */
contract WOSonic is WOETH {
    constructor(ERC20 underlying_) WOETH(underlying_) {}

    function name()
        public
        view
        virtual
        override(WOETH)
        returns (string memory)
    {
        return "Wrapped OS";
    }

    function symbol()
        public
        view
        virtual
        override(WOETH)
        returns (string memory)
    {
        return "wOS";
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { WOETH } from "./WOETH.sol";

/**
 * @title Wrapped OUSD Token Contract
 * @author Origin Protocol Inc
 */
contract WrappedOusd is WOETH {
    constructor(ERC20 underlying_) WOETH(underlying_) {}

    function name()
        public
        view
        virtual
        override(WOETH)
        returns (string memory)
    {
        return "Wrapped OUSD";
    }

    function symbol()
        public
        view
        virtual
        override(WOETH)
        returns (string memory)
    {
        return "WOUSD";
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { WOETH } from "./WOETH.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title wOETH (Plume) Token Contract
 * @author Origin Protocol Inc
 */

contract WOETHPlume is WOETH {
    constructor(ERC20 underlying_) WOETH(underlying_) {}

    function name() public view virtual override returns (string memory) {
        return "Wrapped Super OETH";
    }

    function symbol() public view virtual override returns (string memory) {
        return "wsuperOETHp";
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { WOETH } from "./WOETH.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title OETH Token Contract
 * @author Origin Protocol Inc
 */

contract WOETHBase is WOETH {
    constructor(ERC20 underlying_) WOETH(underlying_) {}

    function name() public view virtual override returns (string memory) {
        return "Wrapped Super OETH";
    }

    function symbol() public view virtual override returns (string memory) {
        return "wsuperOETHb";
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IERC4626 } from "../../../lib/openzeppelin/interfaces/IERC4626.sol";

interface IVaultV2 is IERC4626 {
    function liquidityAdapter() external view returns (address);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { ERC4626 } from "../../lib/openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import { Governable } from "../governance/Governable.sol";
import { Initializable } from "../utils/Initializable.sol";
import { OETH } from "./OETH.sol";

/**
 * @title Wrapped OETH Token Contract
 * @author Origin Protocol Inc
 *
 * @dev An important capability of this contract is that it isn't susceptible to changes of the
 * exchange rate of WOETH/OETH if/when someone sends the underlying asset (OETH) to the contract.
 * If OETH weren't rebasing this could be achieved by solely tracking the ERC20 transfers of the OETH
 * token on mint, deposit, redeem, withdraw. The issue is that OETH is rebasing and OETH balances
 * will change when the token rebases.
 * For that reason the contract logic checks the actual underlying OETH token balance only once
 * (either on a fresh contract creation or upgrade) and considering the WOETH supply and
 * rebasingCreditsPerToken calculates the _adjuster. Once the adjuster is calculated any donations
 * to the contract are ignored. The totalSupply (instead of querying OETH balance) works off of
 * adjuster the current WOETH supply and rebasingCreditsPerToken. This makes WOETH value accrual
 * completely follow OETH's value accrual.
 * WOETH is safe to use in lending markets as the VualtCore's _rebase contains safeguards preventing
 * any sudden large rebases.
 */

contract WOETH is ERC4626, Governable, Initializable {
    using SafeERC20 for IERC20;
    /* This is a 1e27 adjustment constant that expresses the difference in exchange rate between
     * OETH's rebase since inception (expressed with rebasingCreditsPerToken) and WOETH to OETH
     * conversion.
     *
     * If WOETH and OETH are deployed at the same time, the value of adjuster is a neutral 1e27
     */
    uint256 public adjuster;
    uint256[49] private __gap;

    // no need to set ERC20 name and symbol since they are overridden in WOETH & WOETHBase
    constructor(ERC20 underlying_) ERC20("", "") ERC4626(underlying_) {}

    /**
     * @notice Enable OETH rebasing for this contract
     */
    function initialize() external onlyGovernor initializer {
        OETH(address(asset())).rebaseOptIn();

        initialize2();
    }

    /**
     * @notice secondary initializer that newly deployed contracts will execute as part
     *         of primary initialize function and the existing contracts will have it called
     *         as a governance operation.
     */
    function initialize2() public onlyGovernor {
        require(adjuster == 0, "Initialize2 already called");

        if (totalSupply() == 0) {
            adjuster = 1e27;
        } else {
            adjuster =
                (rebasingCreditsPerTokenHighres() *
                    ERC20(asset()).balanceOf(address(this))) /
                totalSupply();
        }
    }

    function name()
        public
        view
        virtual
        override(ERC20, IERC20Metadata)
        returns (string memory)
    {
        return "Wrapped OETH";
    }

    function symbol()
        public
        view
        virtual
        override(ERC20, IERC20Metadata)
        returns (string memory)
    {
        return "wOETH";
    }

    /**
     * @notice Transfer token to governor. Intended for recovering tokens stuck in
     *      contract, i.e. mistaken sends. Cannot transfer OETH
     * @param asset_ Address for the asset
     * @param amount_ Amount of the asset to transfer
     */
    function transferToken(address asset_, uint256 amount_)
        external
        onlyGovernor
    {
        require(asset_ != address(asset()), "Cannot collect core asset");
        IERC20(asset_).safeTransfer(governor(), amount_);
    }

    /// @inheritdoc ERC4626
    function convertToShares(uint256 assets)
        public
        view
        virtual
        override
        returns (uint256 shares)
    {
        return (assets * rebasingCreditsPerTokenHighres()) / adjuster;
    }

    /// @inheritdoc ERC4626
    function convertToAssets(uint256 shares)
        public
        view
        virtual
        override
        returns (uint256 assets)
    {
        return (shares * adjuster) / rebasingCreditsPerTokenHighres();
    }

    /// @inheritdoc ERC4626
    function totalAssets() public view override returns (uint256) {
        return (totalSupply() * adjuster) / rebasingCreditsPerTokenHighres();
    }

    function rebasingCreditsPerTokenHighres() internal view returns (uint256) {
        return OETH(asset()).rebasingCreditsPerTokenHighres();
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

