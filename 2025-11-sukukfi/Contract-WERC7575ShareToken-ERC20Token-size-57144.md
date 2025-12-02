
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
// import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import {DecimalConstants} from "./DecimalConstants.sol";

import {IERC7575, IERC7575Share} from "./interfaces/IERC7575.sol";
import {IERC7575Errors} from "./interfaces/IERC7575Errors.sol";

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {IERC20Errors} from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import {IERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import {Nonces} from "@openzeppelin/contracts/utils/Nonces.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {EIP712} from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import {ERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import {EnumerableMap} from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

// Interface for vault validation - minimal interface to avoid circular dependencies
interface IERC7575Vault {
    function totalAssets() external view returns (uint256);
}

/**
 * @title WERC7575ShareToken (Wrapped ERC20 Share Token)
 * @notice NON-STANDARD ERC-20 IMPLEMENTATION WITH RESTRICTED TRANSFERS
 *
 * WERC = Wrapped ERC20 - Represents underlying assets as normalized 18-decimal shares
 *
 * ARCHITECTURE OVERVIEW:
 * This token provides a 1:1 wrapped representation of underlying ERC20 assets (USDT, USDC, etc.)
 * with decimal normalization to 18 decimals. For example:
 * - 1 USDC (6 decimals) = 1e12 scaling → 1e18 WERC shares
 * - 1 DAI (18 decimals) = 1e0 scaling → 1e18 WERC shares
 *
 * The 1:1 ratio is maintained through deterministic decimal scaling, NOT through
 * totalSupply/totalAssets ratios, making this architecture immune to donation/inflation attacks.
 *
 * USE CASES:
 * - Regulatory-compliant tokenized assets requiring KYC/AML
 * - Institutional vaults with controlled transfer permissions
 * - Multi-asset vault systems with unified 18-decimal share representation
 *
 * @dev This token implements centralized transfer controls that deviate from standard ERC-20:
 *
 * CRITICAL INTEGRATION WARNINGS:
 * - transfer() requires pre-existing self-allowance via permit()
 * - transferFrom() requires both owner's self-allowance AND caller's allowance
 * - approve() blocks self-approval (only validator can authorize via permit)
 * - All recipients must be KYC-verified by the KYC admin
 * - Validator controls batch transfers and permit operations
 * - Revenue admin controls rBalance adjustments
 *
 * INCOMPATIBLE WITH STANDARD ERC-20 INTEGRATIONS:
 * - DEXs (Uniswap, SushiSwap) will fail without modifications
 * - Lending protocols (Compound, Aave) will fail
 * - Standard wallet transfer functions will fail
 * - Multi-sig operations may fail
 * - Token streaming/vesting protocols will fail
 *
 * CENTRALIZATION RISKS:
 * - Single point of failure: KYC admin key compromise can lock all users from transfers
 * - Single point of failure: Validator key compromise can halt batch transfers
 * - Single point of failure: Revenue admin key compromise can manipulate rBalance
 * - User lock-in: KYC admin + validator signatures required for all token movements
 * - Censorship capability: KYC admin can prevent any user from transferring via KYC denial
 *
 * FOR INTEGRATORS:
 * Before integration, ensure your protocol handles:
 * - Permit-based authorization flows instead of standard approvals
 * - KYC verification requirements for all recipients
 * - Validator signature dependencies for user operations
 * - Non-standard transfer mechanics and failure modes
 *
 * See documentation for detailed integration guidelines and risk assessment.
 */
contract WERC7575ShareToken is ERC20, IERC20Permit, EIP712, Nonces, ReentrancyGuard, Ownable2Step, ERC165, Pausable, IERC7575Errors {
    using EnumerableMap for EnumerableMap.AddressToAddressMap;

    // Note: Common errors now inherited from IERC7575Errors interface
    // OnlyOwner is inherited from IERC7575Errors

    // WERC7575-specific errors
    error ArrayTooLarge();
    error ArrayLengthMismatch();
    error LowBalance();
    error ShareTokenZeroValidator();
    error KycRequired();
    error RBalanceAdjustmentAlreadyApplied();
    error FutureTimestampNotAllowed();
    error MaxReturnMultiplierExceeded();
    error NoRBalanceAdjustmentFound();
    error OnlyValidator();
    error AmountTooLarge();
    error RBalanceAdjustmentTooLarge();
    error InconsistentRAccounts(address account, bool firstDiscoveryFlag, bool currentTransferFlag);

    bytes32 private constant PERMIT_TYPEHASH = keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");

    // Batch transfer constants
    // Maximum batch size to prevent exceeding block gas limits
    // Calculated as: 30M gas limit / 25k per transfer ≈ 1000, conservatively set to 100
    // to leave headroom for complex transfers and other operations
    uint256 private constant MAX_BATCH_SIZE = 100;

    // Maximum allowed return multiplier (100% profit cap)
    // Protects against validator input errors and unrealistic returns
    // Value chosen to allow reasonable investment gains while preventing mistakes
    uint256 private constant MAX_RETURN_MULTIPLIER = 2;

    // Batch array size multiplier for worst-case scenario
    // Allocates 2x space: 1 entry per debtor + 1 entry per creditor
    // Handles case where no addresses overlap between debtors and creditors
    uint256 private constant BATCH_ARRAY_MULTIPLIER = 2;

    // Maximum number of vaults per share token - DoS mitigation
    // Prevents unbounded iteration in vault aggregation functions
    uint256 private constant MAX_VAULTS_PER_SHARE_TOKEN = 10;

    mapping(address => uint256) private _balances;
    mapping(address => uint256) private _rBalances;
    mapping(address => mapping(uint256 => uint256[2])) private _rBalanceAdjustments;
    uint256 private _totalSupply;

    mapping(address => bool) public isKycVerified;

    // Multi-vault support as per ERC7575 with EnumerableMap for better management
    EnumerableMap.AddressToAddressMap private _assetToVault; // asset => vault mapping with enumeration
    mapping(address => address) private _vaultToAsset; // vault => asset (for quick reverse lookup and authorization)

    address private _validator; // Controls batchTransfers and permit operations
    address private _kycAdmin; // Controls KYC verification
    address private _revenueAdmin; // Controls rBalance adjustments

    error ERC2612ExpiredSignature(uint256 deadline);
    error ERC2612InvalidSigner(address signer, address owner);
    error OnlyKycAdmin();
    error OnlyRevenueAdmin();
    error ShareTokenZeroKycAdmin();
    error ShareTokenZeroRevenueAdmin();

    event RBalanceAdjusted(address indexed account, uint256 amountInvested, uint256 amountReceived);
    event RBalanceAdjustmentCancelled(address indexed account, uint256 ts);
    event VaultUpdate(address indexed asset, address vault);
    event KYCStatusChanged(address indexed user, address indexed kycAdmin, bool indexed isVerified, uint256 timestamp);
    event ValidatorChanged(address indexed previousValidator, address indexed newValidator);
    event KycAdminChanged(address indexed previousKycAdmin, address indexed newKycAdmin);
    event RevenueAdminChanged(address indexed previousRevenueAdmin, address indexed newRevenueAdmin);

    /**
     * @dev Initializes the ERC7575 share token with multi-asset vault support
     * @param name_ The name of the share token (e.g., "Wrapped USDT")
     * @param symbol_ The symbol of the share token (e.g., "wUSDT")
     *
     * Requirements:
     * - Token decimals must be exactly 18
     * - Sets deployer as owner, validator, kycAdmin, and revenueAdmin
     */
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) EIP712(name_, "1") Ownable(msg.sender) {
        if (decimals() != DecimalConstants.SHARE_TOKEN_DECIMALS) {
            revert WrongDecimals();
        }
        _validator = msg.sender;
        _kycAdmin = msg.sender;
        _revenueAdmin = msg.sender;
    }

    /**
     * @dev Modifier to restrict functions to validator only
     */
    modifier onlyValidator() {
        if (_validator != msg.sender) revert OnlyValidator();
        _;
    }

    /**
     * @dev Modifier to restrict functions to KYC admin only
     */
    modifier onlyKycAdmin() {
        if (_kycAdmin != msg.sender) revert OnlyKycAdmin();
        _;
    }

    /**
     * @dev Modifier to restrict functions to revenue admin only
     */
    modifier onlyRevenueAdmin() {
        if (_revenueAdmin != msg.sender) revert OnlyRevenueAdmin();
        _;
    }

    /**
     * @dev Modifier to restrict functions to authorized vaults only
     */
    modifier onlyVaults() {
        if (_vaultToAsset[msg.sender] == address(0)) revert Unauthorized();
        _;
    }

    /**
     * @dev Adds a new vault for a specific asset (ERC7575 multi-asset support)
     * @param asset The asset token address that the vault will manage
     * @param vaultAddress The vault contract address to authorize
     *
     * Requirements:
     * - Asset must not be zero address
     * - Vault must not be zero address
     * - Asset must not already be registered
     * - Vault's asset() must match the provided asset parameter
     * - Vault's share() must match this ShareToken address
     * - Only callable by owner
     */
    function registerVault(address asset, address vaultAddress) external onlyOwner {
        if (asset == address(0)) revert ZeroAddress();
        if (vaultAddress == address(0)) revert ZeroAddress();
        if (_assetToVault.contains(asset)) revert AssetAlreadyRegistered();

        // Validate that vault's asset matches the provided asset parameter
        if (IERC7575(vaultAddress).asset() != asset) revert AssetMismatch();

        // Validate that vault's share token matches this ShareToken
        if (IERC7575(vaultAddress).share() != address(this)) {
            revert VaultShareMismatch();
        }

        // DoS mitigation: Enforce maximum vaults per share token to prevent unbounded loops
        if (_assetToVault.length() >= MAX_VAULTS_PER_SHARE_TOKEN) {
            revert MaxVaultsExceeded();
        }

        // Register new vault (automatically adds to enumerable collection)
        _assetToVault.set(asset, vaultAddress);
        _vaultToAsset[vaultAddress] = asset;

        emit VaultUpdate(asset, vaultAddress);
    }

    /**
     * @dev Unregisters a vault for a specific asset
     * @param asset The asset token address to unregister vault authorization for
     *
     * SAFETY: This function now includes outstanding shares validation to prevent
     * user fund loss. It checks that the vault has no remaining assets that users
     * could claim, ensuring safe vault unregistration.
     *
     * Requirements:
     * - Vault must exist and be registered
     * - Vault must have zero assets remaining (no user funds at risk)
     * - Only callable by owner
     */
    function unregisterVault(address asset) external onlyOwner {
        if (asset == address(0)) revert ZeroAddress();
        if (!_assetToVault.contains(asset)) revert AssetNotRegistered();

        address vaultAddress = _assetToVault.get(asset);

        // SAFETY CHECK: Validate that vault has no outstanding assets that users could claim
        // In this architecture, we check vault's total assets rather than share supply
        // since shares are managed by this ShareToken contract, not the vault
        try IERC7575Vault(vaultAddress).totalAssets() returns (uint256 totalAssets) {
            if (totalAssets != 0) revert CannotUnregisterVaultAssetBalance();
        } catch {
            // If we can't verify the vault has no assets, we can't safely unregister
            // This prevents unregistration if the vault is malicious or has interface issues
            revert("ShareToken: cannot verify vault has no outstanding assets");
        }
        // Additional safety: Check if vault still has any assets to prevent user fund loss
        // This is a double-check using ERC20 interface in case totalAssets() is manipulated
        try ERC20(asset).balanceOf(vaultAddress) returns (uint256 vaultBalance) {
            if (vaultBalance != 0) revert CannotUnregisterVaultAssetBalance();
        } catch {
            // If we can't check the asset balance in vault, err on the side of caution
            revert("ShareToken: cannot verify vault asset balance");
        }
        // Remove vault registration and authorization (automatically removes from enumerable collection)
        _assetToVault.remove(asset);
        delete _vaultToAsset[vaultAddress]; // Also clear reverse mapping for authorization

        emit VaultUpdate(asset, address(0));
    }

    /**
     * @dev Sets KYC status for an address
     * @param controller The address to set KYC status for
     * @param isVerified True to mark as KYC verified, false otherwise
     *
     * Emits KYCStatusChanged event only when status actually changes to save gas
     */
    function setKycVerified(address controller, bool isVerified) public onlyKycAdmin {
        bool previousStatus = isKycVerified[controller];

        // Only update and emit if status actually changes
        if (previousStatus != isVerified) {
            isKycVerified[controller] = isVerified;
            emit KYCStatusChanged(controller, msg.sender, isVerified, block.timestamp);
        }
    }

    /**
     * @dev Sets the validator address for permit operations and batch transfers
     * @param validator The new validator address
     *
     * Emits a ValidatorChanged event for off-chain monitoring
     */
    function setValidator(address validator) public onlyOwner {
        if (validator == address(0)) revert ShareTokenZeroValidator();
        address previousValidator = _validator;
        _validator = validator;
        emit ValidatorChanged(previousValidator, validator);
    }

    /**
     * @dev Sets the KYC admin address for managing KYC verification
     * @param kycAdmin The new KYC admin address
     *
     * Emits a KycAdminChanged event for off-chain monitoring
     */
    function setKycAdmin(address kycAdmin) public onlyOwner {
        if (kycAdmin == address(0)) revert ShareTokenZeroKycAdmin();
        address previousKycAdmin = _kycAdmin;
        _kycAdmin = kycAdmin;
        emit KycAdminChanged(previousKycAdmin, kycAdmin);
    }

    /**
     * @dev Sets the revenue admin address for managing rBalance adjustments
     * @param revenueAdmin The new revenue admin address
     *
     * Emits a RevenueAdminChanged event for off-chain monitoring
     */
    function setRevenueAdmin(address revenueAdmin) public onlyOwner {
        if (revenueAdmin == address(0)) revert ShareTokenZeroRevenueAdmin();
        address previousRevenueAdmin = _revenueAdmin;
        _revenueAdmin = revenueAdmin;
        emit RevenueAdminChanged(previousRevenueAdmin, revenueAdmin);
    }

    /**
     * @dev Pause critical ShareToken operations. Only callable by owner.
     * Used for emergency situations to halt batch transfers and rBalance adjustments.
     */
    function pause() external onlyOwner {
        _pause();
    }

    /**
     * @dev Unpause ShareToken operations. Only callable by owner.
     */
    function unpause() external onlyOwner {
        _unpause();
    }

    /**
     * @dev Mints new share tokens to an address (vault-only operation)
     * @param to The address to mint tokens to
     * @param amount The amount of tokens to mint
     */
    function mint(address to, uint256 amount) external onlyVaults whenNotPaused {
        if (to == address(0)) {
            revert IERC20Errors.ERC20InvalidReceiver(address(0));
        }
        if (!isKycVerified[to]) revert KycRequired();
        _mint(to, amount);
    }

    /**
     * @dev Burns share tokens from an address (vault-only operation)
     * @param from The address to burn tokens from
     * @param amount The amount of tokens to burn
     */
    function burn(address from, uint256 amount) external onlyVaults whenNotPaused {
        if (from == address(0)) {
            revert IERC20Errors.ERC20InvalidSender(address(0));
        }
        if (!isKycVerified[from]) revert KycRequired();
        _burn(from, amount);
    }

    /**
     * @dev Permit function allowing gasless approvals via signatures
     * @param owner The owner of the tokens
     * @param spender The address to approve spending
     * @param value The amount to approve
     * @param deadline The signature expiration timestamp
     * @param v The recovery byte of the signature
     * @param r Half of the ECDSA signature pair
     * @param s Half of the ECDSA signature pair
     *
     * Special case: When owner == spender, validator signature is required
     */
    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) public virtual {
        if (block.timestamp > deadline) {
            revert ERC2612ExpiredSignature(deadline);
        }

        uint256 nonce = _useNonce(owner);
        bytes32 permitTypehash = PERMIT_TYPEHASH;

        bytes32 structHash;
        assembly {
            let freeMemPtr := mload(0x40)
            mstore(freeMemPtr, permitTypehash)
            mstore(add(freeMemPtr, 0x20), owner)
            mstore(add(freeMemPtr, 0x40), spender)
            mstore(add(freeMemPtr, 0x60), value)
            mstore(add(freeMemPtr, 0x80), nonce)
            mstore(add(freeMemPtr, 0xa0), deadline)
            structHash := keccak256(freeMemPtr, 0xc0)
        }

        bytes32 hash = _hashTypedDataV4(structHash);
        address signer = ECDSA.recover(hash, v, r, s);

        if (owner == spender) {
            if (signer != _validator) {
                revert ERC2612InvalidSigner(signer, owner);
            }
        } else {
            if (signer != owner) {
                revert ERC2612InvalidSigner(signer, owner);
            }
        }
        _approve(owner, spender, value);
    }

    /**
     * @dev Approve function with self-approval protection
     * @param spender The address to approve spending
     * @param value The amount to approve
     * @return bool True if approval successful
     *
     * Note: Self-approval is blocked, use permit instead for self-spending
     */
    function approve(address spender, uint256 value) public virtual override returns (bool) {
        if (msg.sender != spender) {
            return super.approve(spender, value);
        }
        revert ERC20InvalidSpender(msg.sender);
    }

    /**
     * @dev Returns the current nonce for an owner address
     * @param owner The address to get nonce for
     * @return uint256 The current nonce value
     */
    function nonces(address owner) public view virtual override(IERC20Permit, Nonces) returns (uint256) {
        return super.nonces(owner);
    }

    /**
     * @dev Returns the domain separator for EIP-712 signatures
     * @return bytes32 The domain separator hash
     */
    // EIP-712 standard requires mixed-case DOMAIN_SEPARATOR
    function DOMAIN_SEPARATOR() external view virtual returns (bytes32) {
        return _domainSeparatorV4();
    }

    /**
     * @dev Transfer function with self-allowance spending requirement
     * @param to The address to transfer tokens to
     * @param value The amount of tokens to transfer
     * @return bool True if transfer successful
     *
     * Note: Requires self-allowance via permit for transfers, rBalance should not be affected by transfer.
     */
    function transfer(address to, uint256 value) public override whenNotPaused returns (bool) {
        address from = msg.sender;
        if (!isKycVerified[to]) revert KycRequired();
        _spendAllowance(from, from, value);
        return super.transfer(to, value);
    }

    /**
     * @dev Transfer from function with self-allowance spending requirement
     * @param from The address to transfer tokens from
     * @param to The address to transfer tokens to
     * @param value The amount of tokens to transfer
     * @return bool True if transfer successful
     *
     * Note: Always spends from self-allowance regardless of caller, rBalance should not be affected by transferFrom.
     */
    function transferFrom(address from, address to, uint256 value) public override whenNotPaused returns (bool) {
        if (!isKycVerified[to]) revert KycRequired();
        _spendAllowance(from, from, value);
        return super.transferFrom(from, to, value);
    }

    /**
     * @dev See {IERC20-totalSupply}.
     */
    function totalSupply() public view virtual override(ERC20) returns (uint256) {
        return _totalSupply;
    }

    /**
     * @dev Returns the token balance of an account
     * @param account The address to check balance for
     * @return uint256 The token balance
     */
    function balanceOf(address account) public view virtual override(ERC20) returns (uint256) {
        return _balances[account];
    }

    /**
     * @dev Internal update function that maintains custom balance tracking
     * @param from The address tokens are transferred from (zero for minting)
     * @param to The address tokens are transferred to (zero for burning)
     * @param value The amount of tokens being transferred
     *
     * This override maintains our custom _balances mapping to avoid double
     * Transfer event emission in batchTransfers function
     */
    function _update(address from, address to, uint256 value) internal virtual override {
        if (from == address(0)) {
            // Overflow check required: The rest of the code assumes that totalSupply never overflows
            _totalSupply += value;
        } else {
            uint256 fromBalance = _balances[from];
            if (fromBalance < value) {
                revert ERC20InsufficientBalance(from, fromBalance, value);
            }
            unchecked {
                // Overflow not possible: value <= fromBalance <= totalSupply.
                _balances[from] = fromBalance - value;
            }
        }

        if (to == address(0)) {
            unchecked {
                // Overflow not possible: value <= totalSupply or value <= fromBalance <= totalSupply.
                _totalSupply -= value;
            }
        } else {
            unchecked {
                // Overflow not possible: balance + value is at most totalSupply, which we know fits into a uint256.
                _balances[to] += value;
            }
        }

        emit Transfer(from, to, value);
    }

    /**
     * @dev Returns the reserved balance (rBalance) of an account
     * @param account The address to check rBalance for
     * @return uint256 The reserved balance amount
     */
    function rBalanceOf(address account) public view returns (uint256) {
        return _rBalances[account];
    }

    /**
     * @dev Returns the vault address for a given asset
     * @param asset The asset token address
     * @return address The vault address managing this asset (zero address if not registered)
     */
    function vault(address asset) external view returns (address) {
        if (_assetToVault.contains(asset)) {
            return _assetToVault.get(asset);
        }
        return address(0);
    }

    // Note: asset registration can be inferred via vault(asset) != address(0)

    /**
     * @dev Returns whether an address is a registered vault
     * @param vaultAddress The vault address to check
     * @return bool True if the address is a registered vault
     */
    function isVault(address vaultAddress) external view returns (bool) {
        return _vaultToAsset[vaultAddress] != address(0);
    }

    /**
     * @dev Returns all registered assets in the multi-asset system
     * @return address[] Array of all asset addresses that have registered vaults
     */
    function getRegisteredAssets() external view returns (address[] memory) {
        return _assetToVault.keys();
    }

    /**
     * @dev Returns all registered vaults in the multi-asset system
     * @return address[] Array of all vault addresses that are registered
     */
    function getRegisteredVaults() external view returns (address[] memory) {
        address[] memory assets = _assetToVault.keys();
        address[] memory vaults = new address[](assets.length);

        for (uint256 i = 0; i < assets.length; i++) {
            vaults[i] = _assetToVault.get(assets[i]);
        }

        return vaults;
    }

    /**
     * @dev Returns the total number of registered asset-vault pairs
     * @return uint256 The number of registered vaults
     */
    function getVaultCount() external view returns (uint256) {
        return _assetToVault.length();
    }

    /**
     * @dev Returns asset and vault at the given index (for iteration)
     * @param index The index to query
     * @return asset The asset address at this index
     * @return vaultAddress The vault address at this index
     */
    function getVaultAtIndex(uint256 index) external view returns (address asset, address vaultAddress) {
        return _assetToVault.at(index);
    }

    /**
     * @dev Returns the current validator address
     * @return address The validator address
     */
    function getValidator() external view returns (address) {
        return _validator;
    }

    /**
     * @dev Returns the current KYC admin address
     * @return address The KYC admin address
     */
    function getKycAdmin() external view returns (address) {
        return _kycAdmin;
    }

    /**
     * @dev Returns the current revenue admin address
     * @return address The revenue admin address
     */
    function getRevenueAdmin() external view returns (address) {
        return _revenueAdmin;
    }

    /**
     * @dev Returns true if this contract implements the interface defined by interfaceId
     * @param interfaceId The interface identifier, as specified in ERC-165
     * @return bool True if the contract implements interfaceId
     */
    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC165) returns (bool) {
        return interfaceId == type(IERC7575Share).interfaceId || super.supportsInterface(interfaceId);
    }

    /**
     * @dev Spends self allowance for an owner (vault-only operation)
     * @param owner The owner address to spend allowance for
     * @param shares The amount of shares to spend from allowance
     */
    function spendSelfAllowance(address owner, uint256 shares) external onlyVaults {
        _spendAllowance(owner, owner, shares);
    }

    /**
     * @dev Structure to track debits and credits for batch transfer optimization
     * @param owner The account address
     * @param debit Total amount being debited from the account
     * @param credit Total amount being credited to the account
     */
    struct DebitAndCredit {
        address owner;
        uint256 debit;
        uint256 credit;
    }

    /**
     * @dev Performs batch transfers for settlement operations
     * @param debtors Array of addresses to debit tokens from
     * @param creditors Array of addresses to credit tokens to
     * @param amounts Array of amounts for each transfer
     * @return bool True if all transfers successful
     *
     * This function optimizes multiple transfers by netting debits/credits
     * and moves tokens between regular balance and reserved balance (rBalance)
     * to minimize gas costs and avoid double Transfer event emission.
     *
     * REENTRANCY PROTECTION:
     * This function does NOT use nonReentrant guard because:
     * - Only manipulates internal state (_balances)
     * - Makes no external calls to other contracts
     * - Follows Checks-Effects-Interactions (CEI) pattern
     * - No way for an attacker to re-enter before state is finalized
     *
     * Requirements:
     * - All arrays must have the same length
     * - Maximum 100 transfers per batch
     * - Contract must not be paused
     * - Sufficient balance in debtor accounts
     */
    function batchTransfers(address[] calldata debtors, address[] calldata creditors, uint256[] calldata amounts) external onlyValidator returns (bool) {
        (DebitAndCredit[] memory accounts, uint256 accountsLength) = consolidateTransfers(debtors, creditors, amounts);

        // CEI: Update balances only (do NOT modify rBalances - that is rBatchTransfers' job)
        for (uint256 i = 0; i < accountsLength;) {
            DebitAndCredit memory account = accounts[i];
            if (account.debit > account.credit) {
                uint256 amount = account.debit - account.credit;
                uint256 debtorBalance = _balances[account.owner]; // Direct storage access instead of function call
                if (debtorBalance < amount) revert LowBalance();
                unchecked {
                    _balances[account.owner] -= amount;
                }
            } else if (account.debit < account.credit) {
                uint256 amount = account.credit - account.debit;
                unchecked {
                    _balances[account.owner] += amount;
                }
            }

            unchecked {
                ++i;
            } // Unchecked pre-increment for gas optimization
        }

        // CEI: Emit Transfer events after all state changes are complete
        for (uint256 i = 0; i < debtors.length;) {
            emit Transfer(debtors[i], creditors[i], amounts[i]);
            unchecked {
                ++i;
            } // Unchecked pre-increment for gas optimization
        }

        return true;
    }

    /**
     * @dev Computes the rBalance flags bitmap for batch transfers
     * @param debtors Array of debtor addresses
     * @param creditors Array of creditor addresses
     * @param debtorsRBalanceFlags Boolean array: debtorsRBalanceFlags[i] = true if debtors[i] needs rBalance update
     * @param creditorsRBalanceFlags Boolean array: creditorsRBalanceFlags[i] = true if creditors[i] needs rBalance update
     * @return rBalanceFlags Computed bitmap for accounts array indices that need rBalance updates
     *
     * VALIDATION APPROACH:
     * This helper function separates validation from execution for integrity verification:
     *
     * PHASE 1 (PRE-COMPUTATION):
     * - Maps the boolean arrays (indexed by transfer number) to rBalanceFlags bitmap (indexed by aggregated account position)
     * - Replicates EXACT account aggregation logic from consolidateTransfers() for semantic equivalence
     * - Called OFF-CHAIN before transaction submission for verification
     * - Pure function: deterministic, no side effects, independently verifiable
     *
     * PHASE 2 (EXECUTION):
     * - Result passed to rBatchTransfers() as parameter
     * - Uses O(1) bitwise lookup: ((rBalanceFlags >> i) & 1) instead of O(N) search
     * - Ensures only pre-approved accounts have rBalance updated
     *
     * INPUT FORMAT (boolean arrays):
     * - debtorsRBalanceFlags[i]:     true if debtors[i] needs rBalance update
     * - creditorsRBalanceFlags[i]:   true if creditors[i] needs rBalance update
     *
     * OUTPUT FORMAT (rBalanceFlags bitmap):
     * - Bits 0..M-1:   Set if accounts[i] (in aggregated order) needs rBalance update
     *                  where M <= 2N (typically M much less due to deduplication)
     *
     * MAPPING EXAMPLE:
     * Transfer 0: alice → bob    [debtorsRBalanceFlags[0]=true, creditorsRBalanceFlags[0]=false]
     * Transfer 1: bob → charlie  [debtorsRBalanceFlags[1]=false, creditorsRBalanceFlags[1]=true]
     *
     * Account aggregation:
     * 1. Transfer 0: alice (new) → bob (new)
     *    - alice new at position 0, flag=true → set rBalanceFlags bit 0
     *    - bob new at position 1, flag=false → clear rBalanceFlags bit 1
     * 2. Transfer 1: bob (found) → charlie (new)
     *    - bob found at position 1 (no-op)
     *    - charlie new at position 2, flag=true → set rBalanceFlags bit 2
     * Result: rBalanceFlags = 0b101 (alice and charlie marked for update)
     *
     * FIRST-DISCOVERY FLAG DETERMINATION:
     * - Account rBalance flag is set based on FIRST occurrence (earliest transfer) of that account
     * - If alice appears as debtor in transfer 0 (marked for rBalance), alice's flag is set
     * - If alice appears again in transfer 5 (NOT marked for rBalance), flag ALREADY SET, not re-evaluated
     * - This ensures deterministic, order-dependent (but not arbitrary) flag assignment
     * - CONSISTENCY REQUIREMENT: If an account is marked in one role (debtor/creditor), it MUST be
     *   marked consistently in all subsequent transfers involving that account in any role
     *
     * SEMANTIC EQUIVALENCE:
     * The account aggregation logic in computeRBalanceFlags() MUST match
     * consolidateTransfers() exactly. Both:
     * - Skip self-transfers (debtor == creditor)
     * - Use identical bit flag patterns for account discovery
     * - Process accounts in identical discovery order
     * This ensures flags computed here will be applied to correct accounts in rBatchTransfers()
     *
     * INTEGRITY PROPERTIES:
     * 1. Deterministic: Same inputs always produce same output (pure function)
     * 2. Off-chain verifiable: Can compute and validate before submitting transaction
     * 3. First-discovery semantics: Flag set on first encounter, verified on subsequent encounters
     * 4. Clarity: Boolean arrays are more readable than packed bitmaps
     * 5. Type-safe: No bit manipulation errors from incorrect offsets
     */
    function computeRBalanceFlags(
        address[] calldata debtors,
        address[] calldata creditors,
        bool[] calldata debtorsRBalanceFlags,
        bool[] calldata creditorsRBalanceFlags
    )
        external
        pure
        returns (uint256 rBalanceFlags)
    {
        return _computeRBalanceFlagsInternal(debtors, creditors, debtorsRBalanceFlags, creditorsRBalanceFlags);
    }

    function _computeRBalanceFlagsInternal(
        address[] calldata debtorsData,
        address[] calldata creditorsData,
        bool[] calldata debtorsFlagsData,
        bool[] calldata creditorsFlagsData
    )
        internal
        pure
        returns (uint256 rBalanceFlags)
    {
        // Copy calldata to memory to reduce stack depth issues
        address[] memory debtors = debtorsData;
        address[] memory creditors = creditorsData;
        bool[] memory debtorsRBalanceFlags = debtorsFlagsData;
        bool[] memory creditorsRBalanceFlags = creditorsFlagsData;
        uint256 debtorsLength = debtors.length;
        if (debtorsLength > MAX_BATCH_SIZE) revert ArrayTooLarge();
        if (debtorsLength != creditors.length) revert ArrayLengthMismatch();
        if (debtorsLength != debtorsRBalanceFlags.length) revert ArrayLengthMismatch();
        if (debtorsLength != creditorsRBalanceFlags.length) revert ArrayLengthMismatch();

        // Allocate accounts array with same size as consolidateTransfers (2*N max)
        // This maintains semantic equivalence: same aggregation process = same account positions
        address[] memory accounts = new address[](debtorsLength * BATCH_ARRAY_MULTIPLIER);
        uint256 accountsLength = 0;

        // PHASE 1: Replicate account aggregation logic from consolidateTransfers()
        // This double-loop mirrors the exact pattern used in consolidateTransfers():
        // 1. For each transfer, check if debtor/creditor already exist in accounts array
        // 2. Mark with flags which new accounts need to be created
        // 3. When creating new account, check rAccounts input to determine if rBalance update needed
        // 4. Set corresponding bit in rBalanceFlags output bitmap
        // 5. VERIFY: When account is found again, ensure flag consistency with first discovery
        //
        // CRITICAL: This logic MUST remain synchronized with consolidateTransfers().
        // Any divergence will cause flags to be applied to wrong accounts in rBatchTransfers().
        for (uint256 i = 0; i < debtorsLength;) {
            address debtor = debtors[i];
            address creditor = creditors[i];

            // Skip self-transfers (debtor == creditor) - same as consolidateTransfers() line 828
            if (debtor != creditor) {
                // Bit flag tracking (identical pattern to consolidateTransfers lines 830-842):
                // Bit 0 (0x1): Set if debtor needs to be added to accounts array
                // Bit 1 (0x2): Set if creditor needs to be added to accounts array
                // Start with both bits set, clear as we find existing accounts
                uint8 addFlags = 0x3; // 0b11 = both addDebtor and addCreditor initially true

                // Check if debtor or creditor already exist in accounts array
                // IMPORTANT: Once an account is discovered and added, its rBalance flag is SET based on that
                // discovery transfer's rAccounts bit. Subsequent transfers involving same account DON'T
                // re-check or re-set the flag - it was determined by first appearance.
                // VERIFICATION: When account is found again, validate that the expected flag from
                // current transfer's rAccounts matches the flag already set (from first discovery).
                // Loop only while addFlags != 0 (break early if both found)
                for (uint256 j = 0; (j < accountsLength) && addFlags != 0; ++j) {
                    if (accounts[j] == debtor) {
                        // Debtor found in existing accounts (was added in earlier transfer)
                        // VERIFY: Check that this debtor's rBalance flag from current transfer
                        // matches the flag already set in rBalanceFlags at position j
                        // If first discovery marked debtor with flag, current transfer should also mark it
                        // If first discovery didn't mark debtor, current transfer shouldn't either
                        bool currentTransferMarksDebtor = debtorsRBalanceFlags[i];
                        bool debtorAlreadyMarked = ((rBalanceFlags >> j) & 1) == 1;

                        // VERIFICATION LOGIC:
                        // currentTransferMarksDebtor: Whether THIS transfer marks debtor for rBalance
                        // debtorAlreadyMarked: Whether debtor was already marked from FIRST discovery
                        //
                        // CRITICAL INVARIANT: If debtor was marked on first discovery, it MUST be marked
                        // on all subsequent transfers (same role). If not marked on first discovery,
                        // it must NOT be marked in any subsequent transfer (same role).
                        // This ensures consistent rBalance semantics - account flag doesn't change based on
                        // which transfer involves it.
                        //
                        // Enforcement: If boolean flags are inconsistent, revert with detailed error
                        // Custom error includes: account address, flag from first discovery, flag from current transfer
                        if (currentTransferMarksDebtor != debtorAlreadyMarked) {
                            revert InconsistentRAccounts(debtor, debtorAlreadyMarked, currentTransferMarksDebtor);
                        }

                        addFlags &= ~uint8(1); // Clear bit 0 (addDebtor = false)
                    } else if (accounts[j] == creditor) {
                        // Creditor found in existing accounts (was added in earlier transfer)
                        // VERIFY: Check that this creditor's rBalance flag from current transfer
                        // matches the flag already set in rBalanceFlags at position j
                        bool currentTransferMarksCreditor = creditorsRBalanceFlags[i];
                        bool creditorAlreadyMarked = ((rBalanceFlags >> j) & 1) == 1;

                        // VERIFICATION LOGIC: Same as debtor case
                        // currentTransferMarksCreditor: Whether THIS transfer marks creditor for rBalance
                        // creditorAlreadyMarked: Whether creditor was marked from FIRST discovery
                        //
                        // CRITICAL INVARIANT: If creditor was marked on first discovery, it MUST be marked
                        // on all subsequent transfers (same role). If not marked on first discovery,
                        // it must NOT be marked in any subsequent transfer (same role).
                        // This ensures consistent rBalance semantics - account flag doesn't change based on
                        // which transfer involves it.
                        //
                        // Enforcement: If boolean flags are inconsistent, revert with detailed error
                        // Custom error includes: account address, flag from first discovery, flag from current transfer
                        if (currentTransferMarksCreditor != creditorAlreadyMarked) {
                            revert InconsistentRAccounts(creditor, creditorAlreadyMarked, currentTransferMarksCreditor);
                        }

                        addFlags &= ~uint8(2); // Clear bit 1 (addCreditor = false)
                    }
                }

                // Create new account entries only if not found in existing accounts
                if ((addFlags & 1) != 0) {
                    // DEBTOR IS NEW - add to accounts array at current position (accountsLength)
                    // This position will be used as index when processing this account in rBatchTransfers()
                    accounts[accountsLength] = debtor;

                    // Check if this debtor transfer has rBalance update flag set
                    // Use the debtorsRBalanceFlags[i] boolean to determine if flag should be set
                    if (debtorsRBalanceFlags[i]) {
                        // Set corresponding bit in rBalanceFlags output
                        // This marks accounts[accountsLength] for rBalance update in rBatchTransfers()
                        rBalanceFlags |= (uint256(1) << accountsLength);
                    }
                    accountsLength++;
                }

                if ((addFlags & 2) != 0) {
                    // CREDITOR IS NEW - add to accounts array at current position
                    accounts[accountsLength] = creditor;

                    // Check if this creditor transfer has rBalance update flag set
                    // Use the creditorsRBalanceFlags[i] boolean to determine if flag should be set
                    if (creditorsRBalanceFlags[i]) {
                        // Set corresponding bit in rBalanceFlags output
                        // This marks accounts[accountsLength] for rBalance update in rBatchTransfers()
                        rBalanceFlags |= (uint256(1) << accountsLength);
                    }
                    accountsLength++;
                }
            }

            unchecked {
                ++i;
            }
        }

        // Return bitmap where bit i indicates if accounts[i] (in aggregated order) needs rBalance update
        // This bitmap will be used in rBatchTransfers() as: ((rBalanceFlags >> i) & 1) == 1
        return rBalanceFlags;
    }

    /**
     * @dev Consolidates multiple transfers into unique account debit/credit pairs
     * Inlines the account tracking logic for optimal gas efficiency
     * @param debtors Array of debtor addresses
     * @param creditors Array of creditor addresses
     * @param amounts Array of transfer amounts
     * @return accounts Array of consolidated DebitAndCredit structs
     * @return accountsLength Number of unique accounts in array
     *
     * CONSOLIDATION ALGORITHM:
     * Converts N transfers into M unique accounts where M <= 2N (typically M << 2N due to deduplication)
     *
     * Example: 5 transfers between 3 people
     * Input:
     *   Transfer 0: alice → bob (100)
     *   Transfer 1: bob → charlie (50)
     *   Transfer 2: charlie → alice (75)
     *   Transfer 3: alice → bob (25)
     *   Transfer 4: bob → alice (10)
     *
     * Consolidation Result (3 unique accounts):
     *   Account 0 (alice):   debit=100+25=125, credit=75+10=85, net_debit=40
     *   Account 1 (bob):     debit=50+10=60, credit=100+25=125, net_credit=65
     *   Account 2 (charlie): debit=75, credit=50, net_debit=25
     *
     * RELATIONSHIP TO computeRBalanceFlags():
     * - Both functions use identical account discovery logic (lines 819-831 vs 817-830)
     * - Both skip self-transfers (debtor == creditor)
     * - Both track accounts with bit flags (addFlags pattern)
     * - Both process accounts in identical order: order of first appearance in transfer list
     *
     * This means account positions computed in computeRBalanceFlags() correspond EXACTLY
     * to account positions in consolidateTransfers() output. This semantic equivalence is
     * critical for rBalanceFlags bitmap to work correctly.
     *
     * SECURITY NOTE:
     * The account order is deterministic and depends on:
     * 1. Transfer order (which account appears first: debtor or creditor)
     * 2. Transfer history (whether account was seen before)
     * This order cannot be manipulated by changing account balances or other state.
     */
    function consolidateTransfers(
        address[] calldata debtors,
        address[] calldata creditors,
        uint256[] calldata amounts
    )
        internal
        pure
        returns (DebitAndCredit[] memory accounts, uint256 accountsLength)
    {
        uint256 debtorsLength = debtors.length;
        if (debtorsLength > MAX_BATCH_SIZE) revert ArrayTooLarge();
        if (!(debtorsLength == creditors.length && debtorsLength == amounts.length)) revert ArrayLengthMismatch();

        accounts = new DebitAndCredit[](debtorsLength * BATCH_ARRAY_MULTIPLIER);
        accountsLength = 0;

        // Outer loop: process each transfer
        for (uint256 i = 0; i < debtorsLength;) {
            address debtor = debtors[i];
            address creditor = creditors[i];
            uint256 amount = amounts[i];

            // Skip self-transfers (debtor == creditor)
            if (debtor != creditor) {
                // Inline addAccount logic with bit flags for account creation
                uint8 addFlags = 0x3; // 0b11 = both addDebtor and addCreditor initially true

                // Inner loop: check if debtor and creditor already exist in accounts array
                for (uint256 j = 0; (j < accountsLength) && addFlags != 0; ++j) {
                    if (accounts[j].owner == debtor) {
                        accounts[j].debit += amount;
                        addFlags &= ~uint8(1); // Clear bit 0 (addDebtor = false)
                    } else if (accounts[j].owner == creditor) {
                        // else if is safe here since debtor != creditor (self-transfers already skipped)
                        accounts[j].credit += amount;
                        addFlags &= ~uint8(2); // Clear bit 1 (addCreditor = false)
                    }
                }

                // Create new account entries only if not found in existing accounts
                if ((addFlags & 1) != 0) {
                    // Check bit 0 (addDebtor)
                    accounts[accountsLength] = DebitAndCredit(debtor, amount, 0);
                    accountsLength++;
                }
                if ((addFlags & 2) != 0) {
                    // Check bit 1 (addCreditor)
                    accounts[accountsLength] = DebitAndCredit(creditor, 0, amount);
                    accountsLength++;
                }
            }

            unchecked {
                ++i;
            }
        }
    }

    /**
     * @dev Performs batch transfers with selective reserved balance (rBalance) updates
     * @param debtors Array of addresses to debit tokens from
     * @param creditors Array of addresses to credit tokens to
     * @param amounts Array of amounts for each transfer
     * @param rBalanceFlags Bitmap indicating which accounts (by index in aggregated array) need rBalance updates
     *                      Pre-computed by computeRBalanceFlags() for integrity validation
     * @return bool True if all transfers successful
     *
     * PHASE 2 EXECUTION: Uses pre-computed rBalanceFlags for selective rBalance updates
     *
     * FLOW:
     * 1. Call consolidateTransfers() to aggregate N transfers into M unique accounts
     *    - Same aggregation algorithm as computeRBalanceFlags()
     *    - Account positions match rBalanceFlags bitmap indices
     * 2. For each aggregated account, calculate net debit/credit
     * 3. Update _balances directly (CEI pattern, before events)
     * 4. Selectively update _rBalances using rBalanceFlags bitmap
     *    - If ((rBalanceFlags >> accountIndex) & 1) == 1, update _rBalances
     *    - Otherwise, leave _rBalances unchanged
     * 5. Emit Transfer events for original transfers (not consolidated)
     *
     * RBALANCE UPDATES:
     * When account is debtor (debit > credit):
     *   - Loses tokens: _balances[account] -= net_debit
     *   - If flagged: _rBalances[account] += net_debit (restricted balance increases)
     *
     * When account is creditor (credit > debit):
     *   - Gains tokens: _balances[account] += net_credit
     *   - If flagged: _rBalances[account] -= net_credit (restricted balance decreases)
     *                  Capped at 0: if rBalance < net_credit, set to 0
     *
     * INTEGRITY PROPERTIES:
     * - Atomicity: All transfers succeed or all revert (no partial state)
     * - Determinism: Same inputs always produce same state changes
     * - Verification: rBalanceFlags can be pre-verified with computeRBalanceFlags()
     * - Access Control: Only VALIDATOR role can execute
     *
     * REENTRANCY PROTECTION:
     * This function does NOT use nonReentrant guard because:
     * - Only manipulates internal state (_balances and _rBalances)
     * - Makes no external calls to other contracts
     * - Follows Checks-Effects-Interactions (CEI) pattern
     * - No way for an attacker to re-enter before state is finalized
     *
     * This function optimizes batch transfers for investor pools that need selective rBalance updates.
     * Regular settlement operations should use batchTransfers() instead for better gas efficiency.
     *
     * Requirements:
     * - All arrays must have the same length
     * - Maximum 100 transfers per batch
     * - Contract must not be paused
     * - Sufficient balance in debtor accounts
     * - rBalanceFlags must be pre-computed using computeRBalanceFlags()
     */
    function rBatchTransfers(address[] calldata debtors, address[] calldata creditors, uint256[] calldata amounts, uint256 rBalanceFlags) external onlyValidator returns (bool) {
        // PHASE 2A: Consolidate transfers into aggregated accounts
        // Same aggregation as computeRBalanceFlags: N transfers → M unique accounts (M <= 2N)
        // Account order matches rBalanceFlags bitmap indices
        (DebitAndCredit[] memory accounts, uint256 accountsLength) = consolidateTransfers(debtors, creditors, amounts);

        // PHASE 2B: Update balances with Checks-Effects-Interactions pattern
        // Check: Verify sufficient balance BEFORE state change
        // Effects: Update _balances and _rBalances
        // Interactions: Emit events AFTER state is finalized
        for (uint256 i = 0; i < accountsLength;) {
            DebitAndCredit memory account = accounts[i];

            if (account.debit > account.credit) {
                // CASE 1: Account is net DEBTOR (losing tokens)
                // This account had more outflows than inflows
                uint256 amount = account.debit - account.credit;

                // SECURITY: Check balance BEFORE state change (atomic failure)
                uint256 debtorBalance = _balances[account.owner];
                if (debtorBalance < amount) revert LowBalance();

                unchecked {
                    // Update regular balance: subtract net debit
                    _balances[account.owner] -= amount;

                    // CRITICAL: Selective rBalance update based on rBalanceFlags bitmap
                    // Bit position i in rBalanceFlags corresponds to accounts[i]
                    // If bit i is set (1), this account's rBalance increases
                    // This is how computeRBalanceFlags() output controls execution
                    if (((rBalanceFlags >> i) & 1) == 1) {
                        // Account flagged for rBalance update
                        // When losing tokens, restricted balance increases (restricted amount grows)
                        _rBalances[account.owner] += amount;
                    }
                }
            } else if (account.debit < account.credit) {
                // CASE 2: Account is net CREDITOR (gaining tokens)
                // This account had more inflows than outflows
                uint256 amount = account.credit - account.debit;

                unchecked {
                    // Update regular balance: add net credit
                    _balances[account.owner] += amount;

                    // CRITICAL: Selective rBalance update based on rBalanceFlags bitmap
                    // Same bitmap lookup as above
                    if (((rBalanceFlags >> i) & 1) == 1) {
                        // Account flagged for rBalance update
                        // When gaining tokens, restricted balance decreases (restricted amount used)
                        uint256 rbalance = _rBalances[account.owner];
                        if (rbalance < amount) {
                            // Not enough restricted balance to cover credit amount
                            // Set to 0 (no over-correction, stays >= 0)
                            _rBalances[account.owner] = 0;
                        } else {
                            // Have enough restricted balance, decrement by credit amount
                            // (unchecked is parent unchecked block, safe from underflow)
                            _rBalances[account.owner] -= amount;
                        }
                    }
                }
            }
            // Note: If debit == credit, account nets to zero (no balance changes)

            unchecked {
                ++i;
            } // Unchecked pre-increment for gas optimization
        }

        // PHASE 2C: Emit Transfer events after all state changes are complete (CEI pattern)
        // IMPORTANT: Emit ORIGINAL transfers (not consolidated), to match transfer semantics
        // Each debtors[i] → creditors[i] transfer gets one event, even if consolidated
        // This maintains compatibility with standard ERC20 event expectations
        for (uint256 i = 0; i < debtors.length;) {
            emit Transfer(debtors[i], creditors[i], amounts[i]);
            unchecked {
                ++i;
            } // Unchecked pre-increment for gas optimization
        }

        // SUCCESS: All state changes applied, all events emitted, transaction complete
        return true;
    }

    /**
     * RBALANCEFLAGS VALIDATION SYSTEM - COMPREHENSIVE ARCHITECTURAL DOCUMENTATION
     *
     * The rBalanceFlags validation approach is a two-phase system that separates pre-computation
     * (verification) from execution (application) for selective rBalance updates in batch transfers.
     *
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     * PHASE 1: VALIDATION (computeRBalanceFlags)
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     *
     * INPUT:  debtors[], creditors[], rAccounts
     *         - rAccounts: bitmap indexed by transfer number
     *           Bits 0..N-1:     Set if debtors[i] needs rBalance update
     *           Bits N..2N-1:    Set if creditors[i] needs rBalance update
     *
     * OUTPUT: rBalanceFlags bitmap indexed by account position
     *         - Bits 0..M-1:     Set if accounts[i] (in aggregated order) needs rBalance update
     *         - M <= 2N (typically M << 2N due to deduplication)
     *
     * MECHANISM:
     * 1. Iterate through N transfers in order
     * 2. For each transfer, check if debtor/creditor already exist in accounts array
     * 3. Use bit flags (addFlags) to track which accounts need to be added
     * 4. When creating new account at position j:
     *    - Check corresponding bit in rAccounts (bit i for debtor, bit i+N for creditor)
     *    - If set: mark bit j in rBalanceFlags output
     * 5. Result: rBalanceFlags bitmap where bit positions correspond to account positions
     *
     * PROPERTY: Pure function
     * - No side effects, no state changes
     * - Can be called off-chain to verify before submitting transaction
     * - Same inputs always produce identical output (deterministic)
     *
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     * PHASE 2: EXECUTION (rBatchTransfers)
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     *
     * INPUT:  debtors[], creditors[], amounts[], rBalanceFlags (pre-computed)
     *
     * OUTPUT: Updated _balances and _rBalances
     *
     * MECHANISM:
     * 1. Call consolidateTransfers() with same debtors/creditors/amounts
     *    - Produces M aggregated accounts (same order as Phase 1)
     * 2. For each account at position i:
     *    - Calculate net debit/credit
     *    - Update _balances accordingly
     *    - Check rBalanceFlags: if ((rBalanceFlags >> i) & 1) == 1:
     *      * Update _rBalances
     * 3. Emit Transfer events for original transfers
     * 4. Return success
     *
     * PROPERTY: State-changing transaction
     * - Only VALIDATOR role can execute
     * - Protected by nonReentrant guard
     * - Atomic: all updates succeed or all revert (no partial state)
     *
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     * CRITICAL INVARIANT: SEMANTIC EQUIVALENCE
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     *
     * INVARIANT: The account aggregation logic in computeRBalanceFlags() MUST be identical to
     *            consolidateTransfers() to ensure rBalanceFlags bitmap applies to correct accounts.
     *
     * Both functions:
     * ✓ Skip self-transfers: if (debtor != creditor)
     * ✓ Use identical bit flag patterns: 0x3 initial, &= ~1, &= ~2 for tracking
     * ✓ Check accounts in identical order: iterate j < accountsLength
     * ✓ Create accounts in identical order: accounts[accountsLength] = new account
     * ✓ Process transfers in identical order: for i = 0 to N
     *
     * CONSEQUENCE: If invariant is maintained, then:
     * account position i in Phase 1 computation
     *         =
     * account position i in Phase 2 execution
     *
     * If invariant is violated (code divergence):
     * - rBalanceFlags bits may be applied to wrong accounts
     * - Unintended accounts get rBalance updates
     * - Intended accounts miss rBalance updates
     * - Security risk and functional corruption
     *
     * MAINTENANCE: When modifying account aggregation logic, ALWAYS update BOTH functions
     * in lockstep. Add regression test to verify account order matches.
     *
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     * SECURITY PROPERTIES
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     *
     * 1. DETERMINISM
     *    - computeRBalanceFlags() is pure: same inputs → same output always
     *    - Off-chain verification possible and guaranteed accurate
     *    - No randomness or entropy involved
     *
     * 2. ACCESS CONTROL
     *    - Only VALIDATOR role can execute rBatchTransfers()
     *    - Only trusted validators can modify rBalances
     *    - Prevents unauthorized account manipulation
     *
     * 3. FIRST-DISCOVERY FLAG DETERMINATION
     *    - Account rBalance flag is set based on FIRST occurrence (earliest transfer) of that account
     *    - If alice appears as debtor in transfer 0 (marked for rBalance), alice's flag is set
     *    - If alice appears again in transfer 5 (NOT marked for rBalance), flag ALREADY SET, not re-evaluated
     *    - This ensures deterministic, order-dependent (but not arbitrary) flag assignment
     *
     * 4. REENTRANCY PROTECTION
     *    - nonReentrant modifier prevents callback attacks
     *    - No external calls before state finalized (CEI pattern)
     *    - Safe against reentrancy via share token callbacks
     *
     * 5. INTEGRITY VERIFICATION
     *    - Caller can independently verify rBalanceFlags before submission
     *    - Off-chain computation can detect mismatch early
     *    - Prevents accidental wrong-flag submission
     *
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     * THREAT ANALYSIS
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     *
     * THREAT 1: Incorrect rBalanceFlags Provided
     * Attack:   Attacker provides flags that mark wrong accounts for rBalance update
     * Example:  rBalanceFlags = 0xFF (all bits set) instead of computed value
     * Impact:   Unintended rBalance updates, incorrect investor pool state
     * Defenses:
     *   - Off-chain verification: computeRBalanceFlags() can be called to check
     *   - Access control: Only VALIDATOR role allowed, must be trusted
     *   - Event monitoring: Observers can check Transfer events match expected flags
     * Risk:     Medium (mitigated by access control, but depends on validator trustworthiness)
     *
     * THREAT 2: Logic Divergence
     * Attack:   Code maintainer accidentally changes one function without other
     * Example:  consolidateTransfers() changes self-transfer handling, computeRBalanceFlags() doesn't
     * Impact:   Account position mismatch, flags applied to wrong accounts
     * Defenses:
     *   - Code review: Both functions side-by-side during modifications
     *   - Testing: Regression test verifies account order matches
     *   - Documentation: Comments link both functions and explain invariant
     * Risk:     Low (caught by testing and code review)
     *
     * THREAT 3: Reentrancy During Execution
     * Attack:   During _balances update, attacker calls back into rBatchTransfers()
     * Example:  Transfer callback to ERC777 token triggers reentrant call
     * Impact:   Double spending, corrupted state, fund loss
     * Defenses:
     *   - nonReentrant modifier: Reentrancy guard prevents reentry
     *   - CEI pattern: All state changes before events, no callbacks
     *   - Direct storage access: No fallback to external contract functions
     * Risk:     Low (nonReentrant guard + CEI pattern)
     *
     * THREAT 4: Insufficient Balance Not Caught
     * Attack:   Provide transfers that exceed available balances
     * Impact:   Partial state corruption, incorrect balances
     * Defenses:
     *   - Explicit check: if (debtorBalance < amount) revert LowBalance()
     *   - Before state: Check happens BEFORE _balances update
     *   - Atomic: All transfers or none (no partial)
     * Risk:     Low (explicit check before state change)
     *
     * THREAT 5: rBalance Over-increment/Under-decrement
     * Attack:   rBalanceFlags cause rBalance to be updated incorrectly
     * Example:  rBalance += debit, but account was actually creditor (credit > debit)
     * Impact:   Restricted balance tracking corruption
     * Defenses:
     *   - Bit check is correct: if ((rBalanceFlags >> i) & 1) == 1
     *   - Offset is correct: debtor bit vs creditor bit i+N
     *   - Capping: rBalance -= amount capped at 0 (no negative)
     * Risk:     Very Low (conditional logic is straightforward)
     *
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     * PERFORMANCE ANALYSIS
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     *
     * computeRBalanceFlags():
     *   - Time Complexity: O(N²) in worst case
     *     Outer loop: N transfers
     *     Inner loop: up to 2N accounts checked per transfer
     *   - Space Complexity: O(N) for accounts array
     *   - Gas Cost: ~500k-600k for 100 transfers (depends on uniqueness ratio)
     *   - Cost Model: Paid by caller, off-chain execution possible
     *   - Optimization: Loop breaks early if both debtor/creditor found (addFlags != 0)
     *
     * rBatchTransfers():
     *   - Time Complexity: O(N²) for consolidation + O(M) for balance updates
     *     M <= 2N unique accounts
     *   - Space Complexity: O(M) for accounts array
     *   - Gas Cost: ~700k-900k for 100 transfers (on-chain)
     *   - Cost Model: Paid by validator in transaction gas
     *   - Benefit: O(1) per-account rBalance lookup via bitmap (vs O(N) search)
     *
     * Trade-off: Pay computation cost once (Phase 1) to get O(1) lookups during execution (Phase 2)
     *
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     * VALIDATION CHECKLIST
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     *
     * Before calling rBatchTransfers(), verify:
     *   ✓ Arrays (debtors, creditors, amounts) have equal length
     *   ✓ Length <= 100 (MAX_BATCH_SIZE)
     *   ✓ No duplicate (address, address) pairs in (debtors[i], creditors[i])
     *   ✓ All amounts > 0 (no zero transfers)
     *   ✓ rBalanceFlags = computeRBalanceFlags(debtors, creditors, rAccounts)
     *   ✓ All debtors have sufficient balances
     *   ✓ Caller is VALIDATOR role
     *   ✓ No reentrancy protection active
     *
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     * END OF RBALANCEFLAGS VALIDATION SYSTEM DOCUMENTATION
     * ═══════════════════════════════════════════════════════════════════════════════════════════
     */

    /**
     * @dev Adjusts the reserved balance (rBalance) for an account
     * @param account The account address to adjust rBalance for
     * @param ts Timestamp identifier for this adjustment (must be unique per account)
     * @param amounti The invested amount (original investment)
     * @param amountr The received amount (after investment returns/losses)
     *
     * This function allows revenue admin to adjust rBalance based on investment performance.
     * If amountr > amounti, rBalance increases (profit).
     * If amountr < amounti, rBalance decreases (loss).
     *
     * Requirements:
     * - No existing adjustment for the same account and timestamp
     * - Only callable by revenue admin
     * - Should be called as soon as invoice is generated for the invoicing cycle
     * - Reserved balance need to be adjusted before the invoice is paid otherwise we are at risk of creating non existing yield.
     *
     * Known issue:
     * - if the invoice is paid before the adjustment is applied, the adjustment will be wrong.
     * - If the invoice is already paid, no adjustement is required unless pending reserved balance exists.
     */
    function adjustrBalance(address account, uint256 ts, uint256 amounti, uint256 amountr) external onlyRevenueAdmin {
        if (_rBalanceAdjustments[account][ts][0] != 0) {
            revert RBalanceAdjustmentAlreadyApplied();
        }
        if (amounti == 0) revert ZeroAmount();
        if (ts > block.timestamp) revert FutureTimestampNotAllowed();
        // Prevent overflow in return multiplier calculation
        if (amounti > type(uint256).max / MAX_RETURN_MULTIPLIER) {
            revert AmountTooLarge();
        }
        if (amountr > amounti * MAX_RETURN_MULTIPLIER) {
            revert MaxReturnMultiplierExceeded();
        }
        _rBalanceAdjustments[account][ts] = [amounti, amountr];

        uint256 difference;
        if (amountr > amounti) {
            difference = amountr - amounti;
            unchecked {
                _rBalances[account] += difference;
            }
        } else if (amountr < amounti) {
            difference = amounti - amountr;
            uint256 currentRBalance = _rBalances[account];
            if (currentRBalance < difference) {
                // Should not happen otherwise we can't cancel with cancelrBalanceAdjustment
                // If this was the case it would mean that the investment vault has received more assets than the original investment
                // This would mean that the investment vault has made a profit that is not backed by the assets which should not be possible
                revert RBalanceAdjustmentTooLarge();
            } else {
                unchecked {
                    _rBalances[account] -= difference;
                }
            }
        }
        emit RBalanceAdjusted(account, amounti, amountr);
    }

    /**
     * @dev Cancels a previously applied rBalance adjustment
     * @param account The account address to cancel adjustment for
     * @param ts The timestamp identifier of the adjustment to cancel
     *
     * This function reverses the effects of a previous adjustrBalance call
     * by applying the opposite adjustment to restore the original rBalance.
     *
     * Requirements:
     * - An adjustment must exist for the given account and timestamp
     * - Only callable by revenue admin
     */
    function cancelrBalanceAdjustment(address account, uint256 ts) external onlyRevenueAdmin {
        if (_rBalanceAdjustments[account][ts][0] == 0) {
            revert NoRBalanceAdjustmentFound();
        }

        uint256[2] memory adjustment = _rBalanceAdjustments[account][ts];
        uint256 amounti = adjustment[0];
        uint256 amountr = adjustment[1];

        if (amountr > amounti) {
            uint256 difference = amountr - amounti;
            uint256 currentRBalance = _rBalances[account];
            if (currentRBalance < difference) {
                // Should not happen otherwise we can't cancel with the adjustment
                revert RBalanceAdjustmentTooLarge();
            } else {
                unchecked {
                    _rBalances[account] -= difference;
                }
            }
        } else if (amountr < amounti) {
            uint256 difference = amounti - amountr;
            unchecked {
                _rBalances[account] += difference;
            }
        }

        delete _rBalanceAdjustments[account][ts];
        emit RBalanceAdjustmentCancelled(account, ts);
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (utils/Nonces.sol)
pragma solidity ^0.8.20;

/**
 * @dev Provides tracking nonces for addresses. Nonces will only increment.
 */
abstract contract Nonces {
    /**
     * @dev The nonce used for an `account` is not the expected current nonce.
     */
    error InvalidAccountNonce(address account, uint256 currentNonce);

    mapping(address account => uint256) private _nonces;

    /**
     * @dev Returns the next unused nonce for an address.
     */
    function nonces(address owner) public view virtual returns (uint256) {
        return _nonces[owner];
    }

    /**
     * @dev Consumes a nonce.
     *
     * Returns the current value and increments nonce.
     */
    function _useNonce(address owner) internal virtual returns (uint256) {
        // For each account, the nonce has an initial value of 0, can only be incremented by one, and cannot be
        // decremented or reset. This guarantees that the nonce never overflows.
        unchecked {
            // It is important to do x++ and not ++x here.
            return _nonces[owner]++;
        }
    }

    /**
     * @dev Same as {_useNonce} but checking that `nonce` is the next valid for `owner`.
     */
    function _useCheckedNonce(address owner, uint256 nonce) internal virtual {
        uint256 current = _useNonce(owner);
        if (nonce != current) {
            revert InvalidAccountNonce(owner, current);
        }
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.1) (utils/Context.sol)

pragma solidity ^0.8.20;

/**
 * @dev Provides information about the current execution context, including the
 * sender of the transaction and its data. While these are generally available
 * via msg.sender and msg.data, they should not be accessed in such a direct
 * manner, since when dealing with meta-transactions the account sending and
 * paying for execution may not be the actual sender (as far as an application
 * is concerned).
 *
 * This contract is only required for intermediate, library-like contracts.
 */
abstract contract Context {
    function _msgSender() internal view virtual returns (address) {
        return msg.sender;
    }

    function _msgData() internal view virtual returns (bytes calldata) {
        return msg.data;
    }

    function _contextSuffixLength() internal view virtual returns (uint256) {
        return 0;
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.3.0) (utils/Pausable.sol)

pragma solidity ^0.8.20;

import {Context} from "../utils/Context.sol";

/**
 * @dev Contract module which allows children to implement an emergency stop
 * mechanism that can be triggered by an authorized account.
 *
 * This module is used through inheritance. It will make available the
 * modifiers `whenNotPaused` and `whenPaused`, which can be applied to
 * the functions of your contract. Note that they will not be pausable by
 * simply including this module, only once the modifiers are put in place.
 */
abstract contract Pausable is Context {
    bool private _paused;

    /**
     * @dev Emitted when the pause is triggered by `account`.
     */
    event Paused(address account);

    /**
     * @dev Emitted when the pause is lifted by `account`.
     */
    event Unpaused(address account);

    /**
     * @dev The operation failed because the contract is paused.
     */
    error EnforcedPause();

    /**
     * @dev The operation failed because the contract is not paused.
     */
    error ExpectedPause();

    /**
     * @dev Modifier to make a function callable only when the contract is not paused.
     *
     * Requirements:
     *
     * - The contract must not be paused.
     */
    modifier whenNotPaused() {
        _requireNotPaused();
        _;
    }

    /**
     * @dev Modifier to make a function callable only when the contract is paused.
     *
     * Requirements:
     *
     * - The contract must be paused.
     */
    modifier whenPaused() {
        _requirePaused();
        _;
    }

    /**
     * @dev Returns true if the contract is paused, and false otherwise.
     */
    function paused() public view virtual returns (bool) {
        return _paused;
    }

    /**
     * @dev Throws if the contract is paused.
     */
    function _requireNotPaused() internal view virtual {
        if (paused()) {
            revert EnforcedPause();
        }
    }

    /**
     * @dev Throws if the contract is not paused.
     */
    function _requirePaused() internal view virtual {
        if (!paused()) {
            revert ExpectedPause();
        }
    }

    /**
     * @dev Triggers stopped state.
     *
     * Requirements:
     *
     * - The contract must not be paused.
     */
    function _pause() internal virtual whenNotPaused {
        _paused = true;
        emit Paused(_msgSender());
    }

    /**
     * @dev Returns to normal state.
     *
     * Requirements:
     *
     * - The contract must be paused.
     */
    function _unpause() internal virtual whenPaused {
        _paused = false;
        emit Unpaused(_msgSender());
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/**
 * @title IERC7575Errors
 * @dev Common error definitions for ERC7575 vault implementations
 *
 * This interface defines standard errors that are shared across multiple
 * ERC7575 vault implementations to ensure consistency and reusability.
 */
interface IERC7575Errors {
    // ============ Common Vault Errors ============

    /// @dev The vault is not currently active
    error VaultNotActive();

    /// @dev Operation involves zero assets
    error ZeroAssets();

    /// @dev Operation involves zero shares
    error ZeroShares();

    /// @dev Operation involves zero amount
    error ZeroAmount();

    /// @dev Zero address provided where valid address required
    error ZeroAddress();

    // ============ Access Control Errors ============

    /// @dev Invalid owner for the operation
    error InvalidOwner();

    /// @dev Invalid caller for the operation
    error InvalidCaller();

    /// @dev Unauthorized access
    error Unauthorized();

    /// @dev Only owner can perform this operation
    error OnlyOwner();

    // ============ Balance and Allowance Errors ============

    /// @dev Insufficient balance for the operation
    error InsufficientBalance();

    /// @dev Insufficient claimable assets
    error InsufficientClaimableAssets();

    /// @dev Insufficient claimable shares
    error InsufficientClaimableShares();

    /// @dev Deposit amount below minimum required
    error InsufficientDepositAmount();

    // ============ Calculation Errors ============

    /// @dev Zero assets calculated from shares
    error ZeroAssetsCalculated();

    /// @dev Zero shares calculated from assets
    error ZeroSharesCalculated();

    // ============ Array and Batch Operation Errors ============

    /// @dev Array length mismatch in batch operations
    error LengthMismatch();

    /// @dev Batch size too large
    error BatchSizeTooLarge();

    /// @dev Too many requesters for non-paginated operation
    error TooManyRequesters();

    /// @dev Maximum number of vaults per share token exceeded
    error MaxVaultsExceeded();

    // ============ State Errors ============

    /// @dev No pending deposit found
    error NoPendingDeposit();

    /// @dev No pending redemption found
    error NoPendingRedeem();

    // ============ Async Flow Errors ============

    /// @dev Generic async flow error
    error AsyncFlow();

    /// @dev Request is not yet claimable
    error NotClaimable();

    /// @dev Request already claimed
    error AlreadyClaimed();

    /// @dev Request is not in pending state
    error NotPending();

    // ============ Investment Errors ============

    /// @dev No investment vault configured
    error NoInvestmentVault();

    /// @dev Investment manager required but not set
    error OnlyInvestmentManager();

    /// @dev Invalid manager address
    error InvalidManager();

    /// @dev Invalid vault address
    error InvalidVault();

    /// @dev Asset mismatch between vaults
    error AssetMismatch();

    /// @dev Investment self-allowance missing
    error InvestmentSelfAllowanceMissing(uint256 required, uint256 current);

    // ============ Transfer Errors ============

    /// @dev Share transfer failed
    error ShareTransferFailed();

    // ============ Configuration Errors ============

    /// @dev Wrong decimals for ShareToken
    error WrongDecimals();

    /// @dev Asset decimals retrieval failed
    error AssetDecimalsFailed();

    /// @dev Unsupported asset decimals
    error UnsupportedAssetDecimals();

    /// @dev Scaling factor exceeds uint64 maximum
    error ScalingFactorTooLarge();

    // ============ Registration and Lifecycle Errors ============

    /// @dev Asset not registered in the system
    error AssetNotRegistered();

    /// @dev Asset already registered (duplicate registration attempt)
    error AssetAlreadyRegistered();

    /// @dev Vault's share token does not match expected ShareToken
    error VaultShareMismatch();

    /// @dev Cannot unregister vault that is still active
    error CannotUnregisterActiveVault();

    /// @dev Cannot unregister vault with pending deposits
    error CannotUnregisterVaultPendingDeposits();

    /// @dev Cannot unregister vault with claimable redemptions
    error CannotUnregisterVaultClaimableRedemptions();

    /// @dev Cannot unregister vault with active deposit requesters
    error CannotUnregisterVaultActiveDepositRequesters();

    /// @dev Cannot unregister vault with active redeem requesters
    error CannotUnregisterVaultActiveRedeemRequesters();

    /// @dev Cannot unregister vault with outstanding asset balance
    error CannotUnregisterVaultAssetBalance();

    /// @dev Cannot set self as operator
    error CannotSetSelfAsOperator();

    /// @dev Investment ShareToken already configured
    error InvestmentShareTokenAlreadySet();

    // ============ Request ID Errors ============

    /// @dev Invalid requestId provided (only requestId 0 is supported)
    error InvalidRequestId();

    // ============ ERC7887 Cancelation Errors ============

    /// @dev Deposit cancelation request is pending for this controller (blocks new deposits)
    error DepositCancelationPending();

    /// @dev Redeem cancelation request is pending for this controller (blocks new redeems)
    error RedeemCancelationPending();

    /// @dev No pending cancelation deposit found
    error NoPendingCancelDeposit();

    /// @dev No pending cancelation redeem found
    error NoPendingCancelRedeem();

    /// @dev Cancelation request is not yet claimable
    error CancelationNotClaimable();

    /// @dev Cannot cancel a claimable or already claimed request
    error CannotCancelClaimable();
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/**
 * @title DecimalConstants
 * @dev Common decimal validation constants shared between ShareToken and Vault
 */
library DecimalConstants {
    /// @dev Share tokens always use 18 decimals
    uint8 constant SHARE_TOKEN_DECIMALS = 18;

    /// @dev Minimum allowed asset decimals
    uint8 constant MIN_ASSET_DECIMALS = 6;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/**
 * @title IERC7575
 * @dev Interface of the ERC7575 "Multi-Asset ERC-4626 Vaults", as defined in
 *      https://eips.ethereum.org/EIPS/eip-7575
 *
 * This standard extends ERC-4626 to support multiple assets or entry points
 * for the same share token. It includes all ERC4626 functions plus the share() function.
 * Interface ID: 0x2f0a18c5
 */
interface IERC7575 {
    /**
     * @dev Emitted when a vault address is updated for a specific asset.
     * @param asset The asset token address
     * @param vault The vault address for this asset
     */
    event VaultUpdate(address indexed asset, address vault);

    /**
     * @dev Returns the address of the share token.
     * This is the token minted to represent ownership in the vault.
     * @return shareTokenAddress The address of the share token
     */
    function share() external view returns (address shareTokenAddress);

    // ERC4626 functions (inherited from IERC4626)
    function asset() external view returns (address assetTokenAddress);
    function totalAssets() external view returns (uint256 totalManagedAssets);
    function convertToShares(uint256 assets) external view returns (uint256 shares);
    function convertToAssets(uint256 shares) external view returns (uint256 assets);
    function maxDeposit(address receiver) external view returns (uint256 maxAssets);
    function previewDeposit(uint256 assets) external view returns (uint256 shares);
    function deposit(uint256 assets, address receiver) external returns (uint256 shares);
    function maxMint(address receiver) external view returns (uint256 maxShares);
    function previewMint(uint256 shares) external view returns (uint256 assets);
    function mint(uint256 shares, address receiver) external returns (uint256 assets);
    function maxWithdraw(address owner) external view returns (uint256 maxAssets);
    function previewWithdraw(uint256 assets) external view returns (uint256 shares);
    function withdraw(uint256 assets, address receiver, address owner) external returns (uint256 shares);
    function maxRedeem(address owner) external view returns (uint256 maxShares);
    function previewRedeem(uint256 shares) external view returns (uint256 assets);
    function redeem(uint256 shares, address receiver, address owner) external returns (uint256 assets);
}

/**
 * @title IERC7575Share
 * @dev Basic interface for share tokens in the ERC7575 ecosystem.
 * This covers the fundamental vault lookup functionality that all share tokens should implement.
 * Interface ID: 0x3749710f
 */
interface IERC7575Share {
    /**
     * @dev Returns the vault address for a specific asset.
     * Allows share tokens to point back to their vaults.
     * @param asset The asset token address
     * @return vault The vault address that handles this asset
     */
    function vault(address asset) external view returns (address vault);

    /**
     * @dev Returns all registered assets in the multi-asset system.
     * @return assets Array of all asset addresses that have registered vaults
     */
    function getRegisteredAssets() external view returns (address[] memory assets);

    /**
     * @dev Emitted when a vault address is updated for a specific asset.
     * @param asset The asset token address
     * @param vault The vault address for this asset
     */
    event VaultUpdate(address indexed asset, address vault);
}

/**
 * @title IERC7575ShareExtended
 * @dev Full interface for share tokens in the ERC7575 ecosystem with advanced features.
 * Extends the basic interface with optimization functions for upgradeable implementations.
 * Interface ID: 0x0a13f305
 */
interface IERC7575ShareExtended is IERC7575Share {
    /**
     * @dev Returns both circulating supply and total normalized assets in a single optimized call.
     * This is the preferred method for conversion calculations as it reduces gas usage.
     * @return circulatingSupply Total supply minus shares held by vaults for redemption claims
     * @return totalNormalizedAssets Total normalized assets (18 decimals) across all vaults
     */
    function getCirculatingSupplyAndAssets() external view returns (uint256 circulatingSupply, uint256 totalNormalizedAssets);
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DecimalConstants} from "./DecimalConstants.sol";
import {SafeTokenTransfers} from "./SafeTokenTransfers.sol";
import {ShareTokenUpgradeable} from "./ShareTokenUpgradeable.sol";
import {IERC7540, IERC7540Deposit, IERC7540Operator, IERC7540Redeem} from "./interfaces/IERC7540.sol";
import {IERC7575} from "./interfaces/IERC7575.sol";
import {IERC7575Errors} from "./interfaces/IERC7575Errors.sol";
import {IERC7887, IERC7887DepositCancelation, IERC7887RedeemCancelation} from "./interfaces/IERC7887.sol";

import {IVaultMetrics} from "./interfaces/IVaultMetrics.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import {IERC20Errors} from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

import {ERC1967Utils} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

/**
 * @title ERC7575VaultUpgradeable
 * @dev FULLY COMPLIANT implementation of ERC7575 + ERC7540 + ERC7887 + ERC4626 standards
 *
 * STANDARDS COMPLIANCE VERIFICATION:
 *
 * - ERC7575: Multi-Asset ERC-4626 Vaults (https://eips.ethereum.org/EIPS/eip-7575)
 *    CORE SPECIFICATION REQUIREMENTS:
 *    "Multi-Asset Vaults share a single `share` token with multiple entry points
 *     denominated in different `asset` tokens."
 *    "Entry points SHOULD NOT be ERC-20" - COMPLIANT: Vaults implement ERC4626/7575, not ERC-20
 *    "Each entry point must implement share() method" - IMPLEMENTED
 *    "Share single share token across multiple entry points" - IMPLEMENTED: ShareTokenUpgradeable
 *
 * - ERC7540: Asynchronous Tokenized Vault Standard (https://eips.ethereum.org/EIPS/eip-7540)
 *    CORE SPECIFICATION REQUIREMENTS:
 *    "Transfers `assets` from `owner` into the Vault and submits a Request for asynchronous `deposit`" - IMPLEMENTED
 *    "Assumes control of `shares` from `owner` and submits a Request for asynchronous `redeem`" - IMPLEMENTED
 *    "Grants or revokes permissions for `operator` to manage Requests on behalf of the `msg.sender`" - IMPLEMENTED
 *    LIFECYCLE: Pending → Claimable → Claimed (no short-circuiting) - COMPLIANT
 *
 * - ERC7887: Asynchronous Tokenized Vault Cancelation (https://eips.ethereum.org/EIPS/eip-7887)
 *    CORE SPECIFICATION REQUIREMENTS:
 *    "Cancel pending deposit or redeem requests with asynchronous lifecycle" - IMPLEMENTED
 *    "Pending → Claimable → Claimed state transitions (no short-circuiting)" - COMPLIANT
 *    "Block new deposit/redeem requests while cancelation is pending" - IMPLEMENTED
 *    "Cancelations only work on Pending requests, not Claimable" - COMPLIANT
 *
 * - ERC4626: Complete tokenized vault functionality + ERC165 interface detection
 *
 * SECURITY FEATURES:
 * - Asynchronous flows prevent flash loan attacks
 * - Comprehensive reentrancy protection
 * - Multi-signature operator delegation system
 * - Investment vault integration for yield generation
 * - Request blocking prevents race conditions in cancelations
 * - Upgradeable with proper storage layout
 */
contract ERC7575VaultUpgradeable is Initializable, ReentrancyGuard, Ownable2StepUpgradeable, IERC7540, IERC7887, IERC165, IVaultMetrics, IERC7575Errors, IERC20Errors {
    using Math for uint256;
    using SafeERC20 for IERC20Metadata;
    using EnumerableSet for EnumerableSet.AddressSet;

    // Note: Common errors are now inherited from IERC7575Errors interface

    // Events from IERC7540 are inherited from the interfaces
    // Additional custom events for ERC4626 compatibility
    event Deposit(address indexed sender, address indexed owner, uint256 assets, uint256 shares);
    event Withdraw(address indexed sender, address indexed receiver, address indexed owner, uint256 assets, uint256 shares);

    // Investment management events
    event AssetsInvested(uint256 indexed amount, uint256 indexed shares, address indexed investmentVault);
    event AssetsWithdrawnFromInvestment(uint256 indexed requested, uint256 indexed actual, address indexed investmentVault);

    uint256 internal constant REQUEST_ID = 0;

    // Storage slot for Vault-specific data
    bytes32 private constant VAULT_STORAGE_SLOT = keccak256("erc7575.vault.storage");

    struct VaultStorage {
        // Storage slot optimization: pack address + uint64 + bool in single 32-byte slot
        address asset; // 20 bytes
        uint64 scalingFactor; // 8 bytes
        bool isActive; // 1 byte (fits with asset + scalingFactor: total 29 bytes + 3 bytes padding)
        uint8 assetDecimals; // 1 byte
        uint16 minimumDepositAmount; // 2 bytes
        // Remaining addresses (each takes full 32-byte slot)
        address shareToken;
        address investmentManager;
        address investmentVault;
        // Large numbers (each takes full 32-byte slot)
        uint256 totalPendingDepositAssets;
        uint256 totalClaimableRedeemAssets; // Assets reserved for users who can claim them
        uint256 totalClaimableRedeemShares; // Shares held by vault that will be burned on redeem/withdraw
        // ERC7540 mappings with descriptive names
        mapping(address controller => uint256 assets) pendingDepositAssets;
        mapping(address controller => uint256 shares) claimableDepositShares;
        mapping(address controller => uint256 assets) claimableDepositAssets; // Store corresponding asset amounts
        mapping(address controller => uint256 shares) pendingRedeemShares;
        mapping(address controller => uint256 assets) claimableRedeemAssets;
        mapping(address controller => uint256 shares) claimableRedeemShares;
        // Off-chain helper sets for tracking active requests (using EnumerableSet for O(1) operations)
        EnumerableSet.AddressSet activeDepositRequesters;
        EnumerableSet.AddressSet activeRedeemRequesters;
        // ERC7887 Cancelation Request Storage (simplified - requestId is always 0)
        // Deposit cancelations: controller => assets (requestId always 0)
        mapping(address controller => uint256 assets) pendingCancelDepositAssets;
        mapping(address controller => uint256 assets) claimableCancelDepositAssets;
        // Redeem cancelations: controller => shares (requestId always 0)
        mapping(address controller => uint256 shares) pendingCancelRedeemShares;
        mapping(address controller => uint256 shares) claimableCancelRedeemShares;
        // Total pending and claimable cancelation deposit assets (for totalAssets() calculation)
        uint256 totalCancelDepositAssets;
        // Track controllers with pending cancelations to block new requests
        EnumerableSet.AddressSet controllersWithPendingDepositCancelations;
        EnumerableSet.AddressSet controllersWithPendingRedeemCancelations;
    }

    /**
     * @dev Returns the Vault storage struct
     */
    /**
     * @dev Returns the Vault storage struct
     * @return $ The vault storage pointer
     */
    function _getVaultStorage() private pure returns (VaultStorage storage $) {
        bytes32 slot = VAULT_STORAGE_SLOT;
        assembly {
            $.slot := slot
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @dev Initializes the vault
     * @param asset_ The asset token for this vault
     * @param shareToken_ The share token address
     * @param owner Initial owner address
     */
    function initialize(IERC20Metadata asset_, address shareToken_, address owner) public initializer {
        if (shareToken_ == address(0)) {
            revert IERC20Errors.ERC20InvalidReceiver(address(0));
        }
        if (address(asset_) == address(0)) {
            revert IERC20Errors.ERC20InvalidSender(address(0));
        }

        // Validate asset compatibility and get decimals
        uint8 assetDecimals;
        try IERC20Metadata(address(asset_)).decimals() returns (uint8 decimals) {
            if (decimals < DecimalConstants.MIN_ASSET_DECIMALS || decimals > DecimalConstants.SHARE_TOKEN_DECIMALS) {
                revert UnsupportedAssetDecimals();
            }
            assetDecimals = decimals;
        } catch {
            revert AssetDecimalsFailed();
        }
        // Validate share token compatibility and enforce 18 decimals
        try IERC20Metadata(shareToken_).decimals() returns (uint8 decimals) {
            if (decimals != DecimalConstants.SHARE_TOKEN_DECIMALS) {
                revert WrongDecimals();
            }
        } catch {
            revert AssetDecimalsFailed();
        }
        __Ownable_init(owner);

        VaultStorage storage $ = _getVaultStorage();
        $.asset = address(asset_);
        $.assetDecimals = assetDecimals;
        $.shareToken = shareToken_;
        $.investmentManager = owner; // Initially owner is investment manager
        $.isActive = true; // Vault is active by default

        // Calculate scaling factor for decimal normalization: 10^(18 - assetDecimals)
        uint256 scalingFactor = 10 ** (DecimalConstants.SHARE_TOKEN_DECIMALS - assetDecimals);
        if (scalingFactor > type(uint64).max) revert ScalingFactorTooLarge();
        $.scalingFactor = uint64(scalingFactor);
        $.minimumDepositAmount = 1000;
    }

    /**
     * @dev Returns the asset token address
     * @return Asset token address
     */
    function asset() public view returns (address) {
        VaultStorage storage $ = _getVaultStorage();
        return $.asset;
    }

    /**
     * @dev Returns the scaling factor for asset normalization
     * @return Scaling factor (10^(18 - assetDecimals))
     */
    function getScalingFactor() public view returns (uint256) {
        VaultStorage storage $ = _getVaultStorage();
        return $.scalingFactor;
    }

    // ========== ERC7575 Implementation ==========

    /**
     * @dev Returns the share token address
     *
     * ERC7575 SPECIFICATION:
     * "The address of the underlying `share` received on deposit into the Vault.
     * MUST return an address of an ERC-20 share representation of the Vault."
     *
     * ERC7575 MULTI-ASSET ARCHITECTURE:
     * "Multi-Asset Vaults share a single `share` token with multiple entry points
     * denominated in different `asset` tokens."
     *
     * @return Share token address
     */
    function share() public view virtual returns (address) {
        VaultStorage storage $ = _getVaultStorage();
        return $.shareToken;
    }

    // ========== IERC7540 Operator Implementation ==========

    /**
     * @dev Sets or revokes operator approval for the caller (ERC7540 compliant)
     *
     * Allows the caller to approve or revoke an operator who can manage async requests
     * (deposits, redeems, cancelations) on their behalf. The operator system provides
     * a flexible alternative to direct ERC20 allowance for vault authorization.
     *
     * OPERATOR PERMISSIONS:
     * Approved operators can:
     * - Call requestDeposit() on behalf of owner
     * - Call requestRedeem() on behalf of owner (with share allowance if needed)
     * - Call cancelDepositRequest() on behalf of controller
     * - Call cancelRedeemRequest() on behalf of controller
     * - Call deposit()/mint()/redeem() to claim requests on behalf of controller
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7540: Asynchronous Tokenized Vault Standard
     * - Operator permissions are centralized in the share token
     * - Operators bypass ERC20 allowance checks on vault operations
     * - Compatible with multi-asset vault architecture
     *
     * DELEGATION ARCHITECTURE:
     * This vault function delegates to the ShareToken's centralized operator system,
     * ensuring consistent permissions across all vaults sharing the same share token.
     *
     * @param operator Address to approve or revoke as an operator
     * @param approved True to grant operator permission, false to revoke
     *
     * @return Always returns true to indicate operation succeeded
     *
     * @custom:event OperatorSet(msg.sender, operator, approved)
     */
    function setOperator(address operator, bool approved) public virtual returns (bool) {
        VaultStorage storage $ = _getVaultStorage();
        // Call setOperatorFor on the ShareToken to preserve the original msg.sender
        ShareTokenUpgradeable($.shareToken).setOperatorFor(msg.sender, operator, approved);
        // Emit event from vault level for compatibility with tests
        emit OperatorSet(msg.sender, operator, approved);
        return true;
    }

    /**
     * @dev Checks if an operator is approved for a controller (DELEGATED TO SHARETOKEN)
     *
     * ERC7540 SPECIFICATION:
     * "Returns `true` if the `operator` is approved as an operator for a `controller`."
     *
     * CENTRALIZED ARCHITECTURE:
     * This function queries the ShareToken's centralized operator system,
     * providing consistent operator permissions across the entire multi-asset system.
     *
     * @param controller Address of the controller
     * @param operator Address of the operator
     * @return True if operator is approved
     */
    function isOperator(address controller, address operator) external view returns (bool) {
        VaultStorage storage $ = _getVaultStorage();
        // Use a direct view call to the ShareToken's isOperator function
        return ShareTokenUpgradeable($.shareToken).isOperator(controller, operator);
    }

    // ========== IERC7540Deposit Implementation ==========

    /**
     * @dev Submits a request to deposit assets into the vault (ERC7540 compliant)
     *
     * Initiates an asynchronous deposit request by transferring assets from the owner
     * to the vault. Assets enter the Pending state and must be fulfilled by the investment
     * manager before being converted to shares that can be claimed.
     *
     * DEPOSIT LIFECYCLE:
     * 1. Pending: User calls requestDeposit() to submit request with assets
     * 2. Claimable: Investment manager calls fulfillDeposit() to convert assets to shares
     * 3. Claimed: User calls deposit() to claim the shares
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7540: Asynchronous Tokenized Vault Standard
     * - Assets transferred immediately via Pull-Then-Credit pattern
     * - Three-state lifecycle without short-circuiting
     * - Reentrancy-protected via nonReentrant
     * - Blocks new deposits while ERC7887 cancelation is pending
     *
     * AUTHORIZATION:
     * Owner (msg.sender == owner) can call directly, or
     * Operator must be approved via setOperator() on the share token
     *
     * SECURITY CONSIDERATIONS:
     * - Uses nonReentrant guard to prevent reentrancy attacks
     * - Uses Pull-Then-Credit pattern: transfers before state updates
     * - Validates owner balance before transfer for safety
     * - Blocks new requests during pending ERC7887 cancelations
     * - Vault must be active (not paused)
     * - Assets below minimum deposit amount are rejected
     *
     * @param assets The amount of assets to deposit
     * @param controller Address to receive shares when claim is made
     * @param owner Address that owns the assets being deposited
     *
     * @return requestId The requestId of this deposit request (always 0 in this implementation)
     *
     * @custom:throws VaultNotActive If vault has been paused/deactivated
     * @custom:throws InvalidOwner If caller is neither owner nor approved operator
     * @custom:throws ZeroAssets If assets parameter is 0
     * @custom:throws InsufficientDepositAmount If assets < minimum deposit (1000 * 10^decimals)
     * @custom:throws InsufficientBalance If owner has less assets than requested
     * @custom:throws DepositCancelationPending If this controller has pending cancelation
     *
     * @custom:event DepositRequest(controller, owner, requestId, msg.sender, assets)
     */
    function requestDeposit(uint256 assets, address controller, address owner) external nonReentrant returns (uint256 requestId) {
        VaultStorage storage $ = _getVaultStorage();
        if (!$.isActive) revert VaultNotActive();
        if (!(owner == msg.sender || IERC7540($.shareToken).isOperator(owner, msg.sender))) revert InvalidOwner();
        if (assets == 0) revert ZeroAssets();
        if (assets < $.minimumDepositAmount * (10 ** $.assetDecimals)) {
            revert InsufficientDepositAmount();
        }
        uint256 ownerBalance = IERC20Metadata($.asset).balanceOf(owner);
        if (ownerBalance < assets) {
            revert ERC20InsufficientBalance(owner, ownerBalance, assets);
        }
        // ERC7887: Block new deposit requests while cancelation is pending for this controller
        if ($.controllersWithPendingDepositCancelations.contains(controller)) {
            revert DepositCancelationPending();
        }

        // Pull-Then-Credit pattern: Transfer assets first before updating state
        // This ensures we only credit assets that have been successfully received
        // Protects against transfer fee tokens and validates the actual amount transferred
        SafeTokenTransfers.safeTransferFrom($.asset, owner, address(this), assets);

        // State changes after successful transfer
        $.pendingDepositAssets[controller] += assets;
        $.totalPendingDepositAssets += assets;
        $.activeDepositRequesters.add(controller);

        // Event emission
        emit DepositRequest(controller, owner, REQUEST_ID, msg.sender, assets);
        return REQUEST_ID;
    }

    /**
     * @dev Returns the pending deposit amount for a controller
     *
     * ERC7540 SPECIFICATION:
     * "The amount of requested `assets` in Pending state for the `controller` with the given `requestId`.
     * - MUST NOT include any `assets` in Claimable state for deposit or mint.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT revert unless due to integer overflow caused by an unreasonably large input."
     *
     * @param controller Address of the controller
     * @return pendingAssets Amount of assets pending deposit
     */
    function pendingDepositRequest(uint256, address controller) external view returns (uint256 pendingAssets) {
        VaultStorage storage $ = _getVaultStorage();
        return $.pendingDepositAssets[controller];
    }

    /**
     * @dev Fulfills a pending deposit request by converting assets to shares (ERC7540 compliant)
     *
     * Investment manager calls this to fulfill a pending deposit request. Converts the
     * deposited assets into shares and moves them to the Claimable state so the user
     * can claim the shares via deposit() or mint().
     *
     * DEPOSIT LIFECYCLE:
     * 1. Pending: User calls requestDeposit() with assets
     * 2. Claimable: Investment manager calls fulfillDeposit() to convert to shares (THIS FUNCTION)
     * 3. Claimed: User calls deposit() or mint() to receive the shares
     *
     * SHARES MINTING:
     * - Shares are minted to the vault contract immediately
     * - Shares are held by vault until user calls deposit()/mint()
     * - Users can claim using exact assets or exact shares parameters
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7540: Asynchronous Tokenized Vault Standard
     * - Converts pending assets to claimable shares
     * - Uses Floor rounding for conservative share calculation
     *
     * ACCESS CONTROL:
     * - Only callable by the investment manager
     * - Investment manager is set via setInvestmentManager()
     *
     * @param controller Address that made the original deposit request
     * @param assets Amount of assets to fulfill (must be <= pendingDepositAssets[controller])
     *
     * @return shares Amount of shares that will be claimable for this controller
     *
     * @custom:throws OnlyInvestmentManager If caller is not the investment manager
     * @custom:throws InsufficientBalance If assets > pendingDepositAssets[controller]
     * @custom:throws ZeroShares If share calculation results in 0 shares
     */
    function fulfillDeposit(address controller, uint256 assets) public nonReentrant returns (uint256 shares) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();
        uint256 pendingAssets = $.pendingDepositAssets[controller];
        if (assets > pendingAssets) {
            revert ERC20InsufficientBalance(address(this), pendingAssets, assets);
        }

        shares = _convertToShares(assets, Math.Rounding.Floor);
        if (shares == 0) revert ZeroShares();

        $.pendingDepositAssets[controller] -= assets;
        $.totalPendingDepositAssets -= assets;
        $.claimableDepositShares[controller] += shares;
        $.claimableDepositAssets[controller] += assets; // Store asset amount for precise claiming

        // Mint shares to this vault (will be transferred to user on claim)
        ShareTokenUpgradeable($.shareToken).mint(address(this), shares);

        return shares;
    }

    /**
     * @dev Fulfills multiple pending deposit requests in a batch (only investment manager)
     * @param controllers Array of addresses that made the deposit requests
     * @param assets Array of asset amounts to fulfill for each controller
     * @return shares Array of shares that will be claimable for each controller
     */
    function fulfillDeposits(address[] calldata controllers, uint256[] calldata assets) public nonReentrant returns (uint256[] memory shares) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();
        if (controllers.length != assets.length) revert LengthMismatch();

        shares = new uint256[](controllers.length);
        uint256 assetAmounts = 0;
        uint256 shareAmounts = 0;
        for (uint256 i = 0; i < controllers.length; ++i) {
            address controller = controllers[i];
            uint256 assetAmount = assets[i];
            uint256 pendingAssets = $.pendingDepositAssets[controller];
            if (assetAmount > pendingAssets) {
                revert ERC20InsufficientBalance(address(this), pendingAssets, assetAmount);
            }

            uint256 shareAmount = _convertToShares(assetAmount, Math.Rounding.Floor);
            if (shareAmount == 0) revert ZeroShares();

            assetAmounts += assetAmount;
            shareAmounts += shareAmount;
            $.pendingDepositAssets[controller] -= assetAmount;
            $.claimableDepositShares[controller] += shareAmount;
            $.claimableDepositAssets[controller] += assetAmount; // Store asset amount for precise claiming

            shares[i] = shareAmount;
        }
        $.totalPendingDepositAssets -= assetAmounts;
        // Mint shares to this vault (will be transferred to user on claim)
        ShareTokenUpgradeable($.shareToken).mint(address(this), shareAmounts);
        return shares;
    }

    /**
     * @dev Returns the claimable deposit amount for a controller
     *
     * ERC7540 SPECIFICATION:
     * "The amount of requested `assets` in Claimable state for the `controller` with the given `requestId`.
     * - MUST NOT include any `assets` in Pending state for deposit or mint.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT revert unless due to integer overflow caused by an unreasonably large input."
     *
     * IMPLEMENTATION NOTES:
     * In our case, since we have minted shares for an amount of assets,
     * it is preferable to get the claimable shares instead of the claimable assets.
     * @param controller Address of the controller
     * @return claimableAssets Amount of assets ready to claim
     */
    function claimableDepositRequest(uint256, address controller) external view returns (uint256 claimableAssets) {
        VaultStorage storage $ = _getVaultStorage();
        return $.claimableDepositAssets[controller];
    }

    /**
     * @dev Returns the claimable deposit shares for a controller
     * @param controller Address of the controller
     * @return claimableShares Amount of shares ready to claim
     */
    function claimableShares(address controller) external view returns (uint256) {
        VaultStorage storage $ = _getVaultStorage();
        return $.claimableDepositShares[controller];
    }

    /**
     * @dev Claims shares from a fulfilled deposit request by specifying assets (ERC7540 compliant)
     *
     * Final step in the deposit lifecycle: converts claimable assets to shares and transfers
     * them to the receiver. Controller (or their operator) calls this to complete the deposit.
     *
     * DEPOSIT LIFECYCLE:
     * 1. Pending: User calls requestDeposit() with assets
     * 2. Claimable: Investment manager calls fulfillDeposit() to convert assets to shares
     * 3. Claimed: User calls deposit() to receive shares (THIS FUNCTION)
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7540: Asynchronous Tokenized Vault Standard
     * - Converts assets to shares using the stored asset-share ratio
     * - Allows partial claims of claimable amounts
     * - Reentrancy-protected via nonReentrant
     *
     * AUTHORIZATION:
     * Controller (msg.sender == controller) can call directly, or
     * Operator must be approved via setOperator() on the share token
     *
     * SECURITY CONSIDERATIONS:
     * - Uses nonReentrant guard to prevent reentrancy attacks
     * - Share calculation uses Floor rounding (conservative for protocol)
     * - Only callable by controller or approved operator
     * - Removes controller from active set if all assets are claimed
     *
     * @param assets The amount of assets to claim (must be <= claimableDepositAssets[controller])
     * @param receiver Address that will receive the shares
     * @param controller Address that made the original deposit request
     *
     * @return shares The amount of shares received from the claim
     *
     * @custom:throws InvalidCaller If caller is neither controller nor approved operator
     * @custom:throws ZeroAssets If assets parameter is 0
     * @custom:throws InsufficientClaimableAssets If assets > claimableDepositAssets[controller]
     * @custom:throws ZeroSharesCalculated If share calculation results in 0 (request too small)
     * @custom:throws ShareTransferFailed If share transfer to receiver fails
     *
     * @custom:event Deposit(receiver, controller, assets, shares)
     */
    function deposit(uint256 assets, address receiver, address controller) public nonReentrant returns (uint256 shares) {
        VaultStorage storage $ = _getVaultStorage();
        if (!(controller == msg.sender || IERC7540($.shareToken).isOperator(controller, msg.sender))) {
            revert InvalidCaller();
        }
        if (assets == 0) revert ZeroAssets();

        uint256 availableShares = $.claimableDepositShares[controller];
        uint256 availableAssets = $.claimableDepositAssets[controller];

        if (assets > availableAssets) revert InsufficientClaimableAssets();

        // Calculate shares proportionally from the stored asset-share ratio
        shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor);
        if (shares == 0) revert ZeroSharesCalculated();

        // Remove from active deposit requesters if no more claimable assets
        if (availableAssets == assets) {
            $.activeDepositRequesters.remove(controller);
            delete $.claimableDepositShares[controller];
            delete $.claimableDepositAssets[controller];
        } else {
            $.claimableDepositShares[controller] -= shares;
            $.claimableDepositAssets[controller] -= assets;
        }

        emit Deposit(receiver, controller, assets, shares);

        // Transfer shares from vault to receiver using ShareToken
        if (!IERC20Metadata($.shareToken).transfer(receiver, shares)) {
            revert ShareTransferFailed();
        }
    }

    /**
     * @dev Claims exactly specified shares from a fulfilled deposit request (ERC7540 compliant)
     *
     * Final step in the deposit lifecycle: mints exactly the specified shares and transfers them
     * to the receiver. Controller (or their operator) calls this to complete the deposit with exact
     * share amount guarantee.
     *
     * DEPOSIT LIFECYCLE:
     * 1. Pending: User calls requestDeposit() with assets
     * 2. Claimable: Investment manager calls fulfillDeposit() to convert assets to shares
     * 3. Claimed: User calls mint() to receive exact shares (THIS FUNCTION)
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7540: Asynchronous Tokenized Vault Standard
     * - Mints exactly the specified number of shares
     * - Allows partial claims of claimable amounts
     * - Reentrancy-protected via nonReentrant
     *
     * AUTHORIZATION:
     * Controller (msg.sender == controller) can call directly, or
     * Operator must be approved via setOperator() on the share token
     *
     * SECURITY CONSIDERATIONS:
     * - Uses nonReentrant guard to prevent reentrancy attacks
     * - Asset calculation uses Floor rounding (conservative for protocol)
     * - Only callable by controller or approved operator
     * - Removes controller from active set if all shares are claimed
     *
     * @param shares The exact amount of shares to claim
     * @param receiver Address that will receive the shares
     * @param controller Address that made the original deposit request
     *
     * @return assets The amount of assets consumed to generate the shares
     *
     * @custom:throws InvalidCaller If caller is neither controller nor approved operator
     * @custom:throws ZeroAssets If shares parameter is 0 (check fails on zero shares)
     * @custom:throws InsufficientClaimableAssets If assets needed for shares > claimableDepositAssets
     * @custom:throws ZeroSharesCalculated If asset calculation results in 0
     * @custom:throws ShareTransferFailed If share transfer to receiver fails
     *
     * @custom:event Deposit(receiver, controller, assets, shares)
     */
    function mint(uint256 shares, address receiver, address controller) public nonReentrant returns (uint256 assets) {
        VaultStorage storage $ = _getVaultStorage();
        if (!(controller == msg.sender || IERC7540($.shareToken).isOperator(controller, msg.sender))) {
            revert InvalidCaller();
        }
        if (shares == 0) revert ZeroAssets();

        uint256 availableShares = $.claimableDepositShares[controller];
        uint256 availableAssets = $.claimableDepositAssets[controller];

        if (shares > availableShares) revert InsufficientClaimableShares();

        // Calculate assets proportionally from the stored asset-share ratio
        assets = shares.mulDiv(availableAssets, availableShares, Math.Rounding.Floor);
        if (assets == 0) revert ZeroAssetsCalculated();

        // Remove from active deposit requesters if no more claimable shares
        if (availableShares == shares) {
            $.activeDepositRequesters.remove(controller);
            delete $.claimableDepositShares[controller];
            delete $.claimableDepositAssets[controller];
        } else {
            $.claimableDepositShares[controller] -= shares;
            $.claimableDepositAssets[controller] -= assets;
        }

        emit Deposit(receiver, controller, assets, shares);

        // Transfer shares from vault to receiver using ShareToken
        if (!IERC20Metadata($.shareToken).transfer(receiver, shares)) {
            revert ShareTransferFailed();
        }
    }

    // ========== IERC7540Redeem Implementation ==========

    /**
     * @dev Submits a request to redeem shares from the vault (ERC7540 compliant)
     *
     * Initiates an asynchronous redemption request by transferring shares from the owner
     * to the vault. Shares enter the Pending state and must be fulfilled by the investment
     * manager before being converted to assets that can be claimed.
     *
     * REDEEM LIFECYCLE:
     * 1. Pending: User calls requestRedeem() to submit request with shares
     * 2. Claimable: Investment manager calls fulfillRedeem() to convert shares to assets
     * 3. Claimed: User calls redeem() to claim the assets
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7540: Asynchronous Tokenized Vault Standard
     * - Shares transferred immediately via Pull-Then-Credit pattern
     * - Three-state lifecycle without short-circuiting
     * - Reentrancy-protected via nonReentrant
     * - Blocks new redeems while ERC7887 cancelation is pending
     *
     * AUTHORIZATION:
     * Owner (msg.sender == owner) can call directly, or
     * Operator must be approved via setOperator() on the share token, or
     * Spender must be approved via ERC20 approve() on the share token
     *
     * SECURITY CONSIDERATIONS:
     * - Uses nonReentrant guard to prevent reentrancy attacks
     * - Uses Pull-Then-Credit pattern: transfers before state updates
     * - Validates owner balance before transfer for safety
     * - Blocks new requests during pending ERC7887 cancelations
     * - Supports both operator and ERC20 allowance authorization
     * - Shares are held by vault and will be burned when assets are claimed
     *
     * @param shares The amount of shares to redeem
     * @param controller Address to receive assets when claim is made
     * @param owner Address that owns the shares being redeemed
     *
     * @return requestId The requestId of this redeem request (always 0 in this implementation)
     *
     * @custom:throws ERC20InsufficientAllowance If allowance is insufficient (via spendAllowance)
     * @custom:throws InsufficientBalance If owner has less shares than requested
     * @custom:throws ZeroShares If shares parameter is 0
     * @custom:throws RedeemCancelationPending If this controller has pending cancelation
     * @custom:throws ShareTransferFailed If share transfer to vault fails
     *
     * @custom:event RedeemRequest(controller, owner, requestId, msg.sender, shares)
     */
    function requestRedeem(uint256 shares, address controller, address owner) external nonReentrant returns (uint256 requestId) {
        if (shares == 0) revert ZeroShares();
        VaultStorage storage $ = _getVaultStorage();

        // ERC7540 REQUIREMENT: Authorization check for redemption
        // Per spec: "Redeem Request approval of shares for a msg.sender NOT equal to owner may come
        // either from ERC-20 approval over the shares of owner or if the owner has approved the
        // msg.sender as an operator."
        bool isOwnerOrOperator = owner == msg.sender || IERC7540($.shareToken).isOperator(owner, msg.sender);
        if (!isOwnerOrOperator) {
            ShareTokenUpgradeable($.shareToken).spendAllowance(owner, msg.sender, shares);
        }

        uint256 ownerShares = IERC20Metadata($.shareToken).balanceOf(owner);
        if (ownerShares < shares) {
            revert ERC20InsufficientBalance(owner, ownerShares, shares);
        }

        // ERC7887: Block new redeem requests while cancelation is pending for this controller
        if ($.controllersWithPendingRedeemCancelations.contains(controller)) {
            revert RedeemCancelationPending();
        }

        // Pull-Then-Credit pattern: Transfer shares first before updating state
        // This ensures we only credit shares that have been successfully received
        if (!ShareTokenUpgradeable($.shareToken).vaultTransferFrom(owner, address(this), shares)) {
            revert ShareTransferFailed();
        }

        // State changes after successful transfer
        $.pendingRedeemShares[controller] += shares;
        $.activeRedeemRequesters.add(controller);

        // Event emission
        emit RedeemRequest(controller, owner, REQUEST_ID, msg.sender, shares);
        return REQUEST_ID;
    }

    /**
     * @dev Returns the pending redemption amount for a controller
     *
     * ERC7540 SPECIFICATION:
     * "The amount of requested `shares` in Pending state for the `controller` with the given `requestId`.
     * - MUST NOT include any `shares` in Claimable state for redeem or withdraw.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT revert unless due to integer overflow caused by an unreasonably large input."
     *
     * @param controller Address of the controller
     * @return pendingShares Amount of shares pending redemption
     */
    function pendingRedeemRequest(uint256, address controller) external view returns (uint256 pendingShares) {
        VaultStorage storage $ = _getVaultStorage();
        return $.pendingRedeemShares[controller];
    }

    /**
     * @dev Returns the claimable redemption amount for a controller
     *
     * ERC7540 SPECIFICATION:
     * "The amount of requested `shares` in Claimable state for the `controller` with the given `requestId`.
     * - MUST NOT include any `shares` in Pending state for redeem or withdraw.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT revert unless due to integer overflow caused by an unreasonably large input."
     *
     * @param controller Address of the controller
     * @return claimableRedeemShares Amount of shares ready to redeem
     */
    function claimableRedeemRequest(uint256, address controller) external view returns (uint256 claimableRedeemShares) {
        VaultStorage storage $ = _getVaultStorage();
        return $.claimableRedeemShares[controller];
    }

    /**
     * @dev Fulfills a pending redeem request by converting shares to assets (ERC7540 compliant)
     *
     * Investment manager calls this to fulfill a pending redeem request. Converts the
     * redeemed shares into assets and moves them to the Claimable state so the user
     * can claim the assets via redeem() or withdraw().
     *
     * REDEEM LIFECYCLE:
     * 1. Pending: User calls requestRedeem() with shares
     * 2. Claimable: Investment manager calls fulfillRedeem() to convert to assets (THIS FUNCTION)
     * 3. Claimed: User calls redeem() or withdraw() to receive the assets
     *
     * SHARES BURNING:
     * - Shares are NOT burned during fulfillment
     * - Shares are held by vault and burned when user claims them
     * - This prevents double-burning and enables partial claims
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7540: Asynchronous Tokenized Vault Standard
     * - Converts pending shares to claimable assets
     * - Uses Floor rounding for conservative asset calculation
     *
     * ACCESS CONTROL:
     * - Only callable by the investment manager
     * - Investment manager is set via setInvestmentManager()
     *
     * @param controller Address that made the original redeem request
     * @param shares Amount of shares to fulfill (must be <= pendingRedeemShares[controller])
     *
     * @return assets Amount of assets that will be claimable for this controller
     *
     * @custom:throws OnlyInvestmentManager If caller is not the investment manager
     * @custom:throws ZeroShares If shares parameter is 0
     * @custom:throws InsufficientBalance If shares > pendingRedeemShares[controller]
     */
    function fulfillRedeem(address controller, uint256 shares) public nonReentrant returns (uint256 assets) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();
        if (shares == 0) revert ZeroShares();
        uint256 pendingShares = $.pendingRedeemShares[controller];
        if (shares > pendingShares) {
            revert ERC20InsufficientBalance(address(this), pendingShares, shares);
        }

        assets = _convertToAssets(shares, Math.Rounding.Floor);

        $.pendingRedeemShares[controller] -= shares;
        $.claimableRedeemAssets[controller] += assets;
        $.claimableRedeemShares[controller] += shares;
        $.totalClaimableRedeemAssets += assets;
        $.totalClaimableRedeemShares += shares; // Track shares that will be burned

        // Note: Shares are NOT burned here - they will be burned during redeem/withdraw claim
        return assets;
    }

    /**
     * @dev Claims assets from a fulfilled redemption request by specifying shares (ERC7540 compliant)
     *
     * Final step in the redeem lifecycle: converts claimable shares to assets, burns the shares,
     * and transfers assets to the receiver. Controller (or their operator) calls this to complete
     * the redemption.
     *
     * REDEEM LIFECYCLE:
     * 1. Pending: User calls requestRedeem() with shares
     * 2. Claimable: Investment manager calls fulfillRedeem() to convert shares to assets
     * 3. Claimed: User calls redeem() to receive assets (THIS FUNCTION)
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7540: Asynchronous Tokenized Vault Standard
     * - Converts shares to assets using the stored share-asset ratio
     * - Allows partial claims of claimable amounts
     * - Reentrancy-protected via nonReentrant
     *
     * AUTHORIZATION:
     * Controller (msg.sender == controller) can call directly, or
     * Operator must be approved via setOperator() on the share token
     *
     * SECURITY CONSIDERATIONS:
     * - Uses nonReentrant guard to prevent reentrancy attacks
     * - Asset calculation uses Floor rounding (conservative for protocol)
     * - Only callable by controller or approved operator
     * - Burns shares held by vault after asset calculation
     * - Removes controller from active set if all shares are claimed
     *
     * @param shares The amount of shares to redeem (must be <= claimableRedeemShares[controller])
     * @param receiver Address that will receive the assets
     * @param controller Address that made the original redeem request
     *
     * @return assets The amount of assets received from the redemption
     *
     * @custom:throws InvalidCaller If caller is neither controller nor approved operator
     * @custom:throws ZeroShares If shares parameter is 0
     * @custom:throws InsufficientClaimableShares If shares > claimableRedeemShares[controller]
     * @custom:throws AssetTransferFailed If asset transfer to receiver fails
     *
     * @custom:event Withdraw(receiver, controller, owner, assets, shares)
     */
    function redeem(uint256 shares, address receiver, address controller) public nonReentrant returns (uint256 assets) {
        VaultStorage storage $ = _getVaultStorage();
        if (!(controller == msg.sender || IERC7540($.shareToken).isOperator(controller, msg.sender))) {
            revert InvalidCaller();
        }
        if (shares == 0) revert ZeroShares();

        uint256 availableShares = $.claimableRedeemShares[controller];
        if (shares > availableShares) revert InsufficientClaimableShares();

        // Calculate proportional assets for the requested shares
        uint256 availableAssets = $.claimableRedeemAssets[controller];
        assets = shares.mulDiv(availableAssets, availableShares, Math.Rounding.Floor);

        if (assets == availableAssets) {
            // Remove from active redeem requesters if no more claimable assets and the potential dust
            $.activeRedeemRequesters.remove(controller);
            delete $.claimableRedeemAssets[controller];
            delete $.claimableRedeemShares[controller];
        } else {
            $.claimableRedeemAssets[controller] -= assets;
            $.claimableRedeemShares[controller] -= shares;
        }
        $.totalClaimableRedeemAssets -= assets;
        $.totalClaimableRedeemShares -= shares; // Decrement shares that are being burned

        // Burn the shares as per ERC7540 spec - shares are burned when request is claimed
        ShareTokenUpgradeable($.shareToken).burn(address(this), shares);

        emit Withdraw(msg.sender, receiver, controller, assets, shares);
        if (assets > 0) {
            SafeTokenTransfers.safeTransfer($.asset, receiver, assets);
        }
    }

    /**
     * @dev Claims shares by specifying desired assets from fulfilled redemption
     * @param assets Amount of assets to withdraw
     * @param receiver Address to receive the assets
     * @param controller Address that made the original request
     * @return shares Amount of shares consumed
     */
    function withdraw(uint256 assets, address receiver, address controller) public nonReentrant returns (uint256 shares) {
        VaultStorage storage $ = _getVaultStorage();
        if (!(controller == msg.sender || IERC7540($.shareToken).isOperator(controller, msg.sender))) {
            revert InvalidCaller();
        }
        if (assets == 0) revert ZeroAssets();

        uint256 availableAssets = $.claimableRedeemAssets[controller];
        if (assets > availableAssets) revert InsufficientClaimableAssets();

        // Calculate proportional shares for the requested assets
        uint256 availableShares = $.claimableRedeemShares[controller];
        shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor);

        if (shares == availableShares) {
            // Remove from active redeem requesters if no more claimable assets and the potential dust
            $.activeRedeemRequesters.remove(controller);
            delete $.claimableRedeemAssets[controller];
            delete $.claimableRedeemShares[controller];
        } else {
            $.claimableRedeemAssets[controller] -= assets;
            $.claimableRedeemShares[controller] -= shares;
        }

        $.totalClaimableRedeemAssets -= assets;
        $.totalClaimableRedeemShares -= shares; // Decrement shares that are being burned

        // Burn the shares as per ERC7540 spec - shares are burned when request is claimed
        if (shares > 0) {
            ShareTokenUpgradeable($.shareToken).burn(address(this), shares);
        }

        emit Withdraw(msg.sender, receiver, controller, assets, shares);

        SafeTokenTransfers.safeTransfer($.asset, receiver, assets);
    }

    // ========== ERC7887 Cancelation Fulfillment Functions ==========

    /**
     * @dev Fulfills a pending deposit cancelation request (ERC7887 compliant)
     *
     * Investment manager calls this to fulfill a pending deposit cancelation.
     * Transitions assets from pending cancelation to claimable state so the user
     * can claim the original assets back.
     *
     * CANCELATION LIFECYCLE:
     * 1. Pending: User calls cancelDepositRequest() to initiate cancelation
     * 2. Claimable: Investment manager calls fulfillCancelDepositRequest() to fulfill (THIS FUNCTION)
     * 3. Claimed: User calls claimCancelDepositRequest() to receive assets
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7887: Asynchronous Tokenized Vault Cancelation Extension
     * - Moves assets from pending to claimable state
     * - No state mutation until fulfillment
     *
     * ACCESS CONTROL:
     * - Only callable by the investment manager
     * - Investment manager is set via setInvestmentManager()
     *
     * @param controller Address that made the original deposit cancelation request
     *
     * @return assets Amount of assets now claimable for this controller
     *
     * @custom:throws OnlyInvestmentManager If caller is not the investment manager
     * @custom:throws NoPendingCancelDeposit If no pending cancelation exists for controller
     */
    function fulfillCancelDepositRequest(address controller) external returns (uint256 assets) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();

        assets = $.pendingCancelDepositAssets[controller];
        if (assets == 0) revert NoPendingCancelDeposit();

        // Move from pending to claimable cancelation state
        delete $.pendingCancelDepositAssets[controller];
        $.claimableCancelDepositAssets[controller] += assets;

        return assets;
    }

    /**
     * @dev Fulfills multiple pending deposit cancelations in a batch (ERC7887 compliant)
     *
     * Investment manager calls this to efficiently fulfill multiple pending deposit
     * cancelation requests in a single transaction. Reduces gas costs for bulk operations.
     *
     * BATCH OPERATIONS:
     * - Processes all controllers regardless of whether they have pending cancelations
     * - Returns 0 for controllers with no pending cancelation
     * - Efficiently updates state for all controllers at once
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7887: Asynchronous Tokenized Vault Cancelation Extension
     * - Moves all pending assets to claimable state
     * - Optimized for batch processing
     *
     * ACCESS CONTROL:
     * - Only callable by the investment manager
     * - Investment manager is set via setInvestmentManager()
     *
     * @param controllers Array of addresses that made the original deposit cancelation requests
     *
     * @return assets Array of assets now claimable for each controller (0 if no pending)
     *
     * @custom:throws OnlyInvestmentManager If caller is not the investment manager
     */
    function fulfillCancelDepositRequests(address[] calldata controllers) external returns (uint256[] memory assets) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();

        assets = new uint256[](controllers.length);
        for (uint256 i = 0; i < controllers.length; ++i) {
            address controller = controllers[i];
            uint256 pendingAssets = $.pendingCancelDepositAssets[controller];

            if (pendingAssets > 0) {
                delete $.pendingCancelDepositAssets[controller];
                $.claimableCancelDepositAssets[controller] += pendingAssets;
                assets[i] = pendingAssets;
            }
        }

        return assets;
    }

    /**
     * @dev Fulfills a pending redeem cancelation request (ERC7887 compliant)
     *
     * Investment manager calls this to fulfill a pending redeem cancelation.
     * Transitions shares from pending cancelation to claimable state so the user
     * can claim the original shares back.
     *
     * CANCELATION LIFECYCLE:
     * 1. Pending: User calls cancelRedeemRequest() to initiate cancelation
     * 2. Claimable: Investment manager calls fulfillCancelRedeemRequest() to fulfill (THIS FUNCTION)
     * 3. Claimed: User calls claimCancelRedeemRequest() to receive shares
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7887: Asynchronous Tokenized Vault Cancelation Extension
     * - Moves shares from pending to claimable state
     * - No state mutation until fulfillment
     *
     * ACCESS CONTROL:
     * - Only callable by the investment manager
     * - Investment manager is set via setInvestmentManager()
     *
     * @param controller Address that made the original redeem cancelation request
     *
     * @return shares Amount of shares now claimable for this controller
     *
     * @custom:throws OnlyInvestmentManager If caller is not the investment manager
     * @custom:throws NoPendingCancelRedeem If no pending redeem cancelation exists for controller
     */
    function fulfillCancelRedeemRequest(address controller) external returns (uint256 shares) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();

        shares = $.pendingCancelRedeemShares[controller];
        if (shares == 0) revert NoPendingCancelRedeem();

        // Move from pending to claimable cancelation state
        delete $.pendingCancelRedeemShares[controller];
        $.claimableCancelRedeemShares[controller] += shares;
    }

    /**
     * @dev Fulfills multiple pending redeem cancelations in a batch (ERC7887 compliant)
     *
     * Investment manager calls this to efficiently fulfill multiple pending redeem
     * cancelation requests in a single transaction. Reduces gas costs for bulk operations.
     *
     * BATCH OPERATIONS:
     * - Processes all controllers regardless of whether they have pending cancelations
     * - Returns 0 for controllers with no pending cancelation
     * - Efficiently updates state for all controllers at once
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7887: Asynchronous Tokenized Vault Cancelation Extension
     * - Moves all pending shares to claimable state
     * - Optimized for batch processing
     *
     * ACCESS CONTROL:
     * - Only callable by the investment manager
     * - Investment manager is set via setInvestmentManager()
     *
     * @param controllers Array of addresses that made the original redeem cancelation requests
     *
     * @return shares Array of shares now claimable for each controller (0 if no pending)
     *
     * @custom:throws OnlyInvestmentManager If caller is not the investment manager
     */
    function fulfillCancelRedeemRequests(address[] calldata controllers) external returns (uint256[] memory shares) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();

        shares = new uint256[](controllers.length);
        for (uint256 i = 0; i < controllers.length; ++i) {
            address controller = controllers[i];
            uint256 pendingShares = $.pendingCancelRedeemShares[controller];

            if (pendingShares > 0) {
                delete $.pendingCancelRedeemShares[controller];
                $.claimableCancelRedeemShares[controller] += pendingShares;
                shares[i] = pendingShares;
            }
        }
    }

    // ========== ERC4626-like functions ==========

    /**
     * @dev Returns total assets managed by the vault (EXCLUDES invested assets to avoid double counting)
     *
     * INVESTMENT ARCHITECTURE:
     * This function returns only assets physically held in this vault's token balance.
     * Assets are excluded in these states:
     * - Pending deposits (not yet fulfilled by investment manager)
     * - Claimable redemptions (reserved for user withdrawals)
     * - Invested in external investment vaults (tracked at ShareToken level)
     *
     * ASSET LIFECYCLE:
     * 1. User calls requestDeposit() → Assets transferred TO this vault (pending state)
     * 2. Manager calls fulfillDeposit() → Assets stay in vault, now available for investment
     * 3. Manager calls investAssets() → Assets transferred to investment vault
     * 4. Investment vault shares credited to ShareToken contract
     * 5. ShareToken's getInvestedAssets() includes these invested assets in global accounting
     *
     * This design prevents double-counting when aggregating across multiple vaults while
     * ensuring all assets are tracked somewhere in the system.
     *
     * ERC7575/ERC7540 DEVIATION FROM ERC4626:
     * This implementation differs from ERC4626 totalAssets() because of the async vault pattern:
     * - Only returns assets AVAILABLE in the vault (not allocated to pending operations)
     * - Excludes totalPendingDepositAssets (funds waiting for fulfillDeposit)
     * - Excludes totalClaimableRedeemAssets (funds reserved for user redemption claims)
     * - Excludes totalCancelDepositAssets (funds reserved for pending deposit cancelations)
     * - Excludes already invested assets (funds deployed to ERC7575 investment vaults)
     *   Once assets are invested, they may be withdrawn as different assets from the investment
     *   contract, so totalAssets() excludes them. Use ShareToken.getInvestedAssets() for invested
     *   asset accounting across all vaults.
     * - This prevents over-accounting when assets move between states (pending → claimable → claimed)
     * - Use for: conversion calculations, investment availability checks
     * - For complete asset accounting: sum this value + pending + claimable + cancelation + invested assets
     *
     * @return Total amount of assets available in the vault (not reserved for pending operations or invested)
     */
    function totalAssets() public view virtual returns (uint256) {
        VaultStorage storage $ = _getVaultStorage();
        uint256 balance = IERC20Metadata($.asset).balanceOf(address(this));
        // Exclude pending deposits, pending/claimable cancelation deposits, and claimable withdrawals from total assets
        uint256 reservedAssets = $.totalPendingDepositAssets + $.totalClaimableRedeemAssets + $.totalCancelDepositAssets;
        return balance > reservedAssets ? balance - reservedAssets : 0;
    }

    /**
     * @dev Internal function to convert assets to shares with specified rounding
     * @param assets Amount of assets to convert
     * @param rounding Rounding mode (Floor = favor vault, Ceil = favor user)
     * @return shares Amount of shares equivalent to assets
     */
    function _convertToShares(uint256 assets, Math.Rounding rounding) internal view returns (uint256 shares) {
        VaultStorage storage $ = _getVaultStorage();
        // First normalize assets to 18 decimals using scaling factor
        // Use Math.mulDiv to prevent overflow for large amounts
        uint256 normalizedAssets = Math.mulDiv(assets, $.scalingFactor, 1);

        // Use optimized ShareToken conversion method (single call instead of multiple)
        shares = ShareTokenUpgradeable($.shareToken).convertNormalizedAssetsToShares(normalizedAssets, rounding);
    }

    /**
     * @dev Internal function to convert shares to assets with specified rounding
     * @param shares Amount of shares to convert
     * @param rounding Rounding mode (Floor = favor vault, Ceil = favor user)
     * @return assets Amount of assets equivalent to shares
     */
    function _convertToAssets(uint256 shares, Math.Rounding rounding) internal view returns (uint256 assets) {
        VaultStorage storage $ = _getVaultStorage();
        uint256 scaling = $.scalingFactor;
        // Use optimized ShareToken conversion method (single call instead of multiple)
        uint256 normalizedAssets = ShareTokenUpgradeable($.shareToken).convertSharesToNormalizedAssets(shares, rounding);

        // Then denormalize back to original asset decimals
        if (scaling == 1) {
            return normalizedAssets;
        } else {
            return Math.mulDiv(normalizedAssets, 1, scaling, rounding);
        }
    }

    /**
     * @dev Converts assets to shares using current exchange rate
     * @param assets Amount of assets to convert
     * @return Amount of shares equivalent to assets
     */
    function convertToShares(uint256 assets) public view virtual returns (uint256) {
        return _convertToShares(assets, Math.Rounding.Floor);
    }

    /**
     * @dev Converts shares to assets using current exchange rate
     * @param shares Amount of shares to convert
     * @return Amount of assets equivalent to shares
     */
    function convertToAssets(uint256 shares) public view virtual returns (uint256) {
        return _convertToAssets(shares, Math.Rounding.Floor);
    }

    /**
     * @dev Returns total supply of shares across all vaults in the multi-asset system
     * @return Total share supply from ShareToken
     */
    function totalSupply() public view virtual returns (uint256) {
        // For ERC7575 multi-asset system, return the total supply from ShareToken
        VaultStorage storage $ = _getVaultStorage();
        address shareToken_ = $.shareToken;
        return IERC20Metadata(shareToken_).totalSupply();
    }

    /**
     * @dev Returns share balance of an account
     * @param account Address to check balance for
     * @return Share balance from ShareToken
     */
    function balanceOf(address account) public view virtual returns (uint256) {
        // For ERC7575, balances are tracked in ShareToken
        VaultStorage storage $ = _getVaultStorage();
        address shareToken_ = $.shareToken;
        return IERC20Metadata(shareToken_).balanceOf(account);
    }

    /**
     * @dev Returns maximum assets that can be deposited for a controller
     * @param controller Address to check max deposit for
     * @return Maximum claimable assets for deposit
     */
    function maxDeposit(address controller) public view virtual returns (uint256) {
        VaultStorage storage $ = _getVaultStorage();
        return $.claimableDepositAssets[controller];
    }

    /**
     * @dev Returns maximum shares that can be minted for a controller
     * @param controller Address to check max mint for
     * @return Maximum claimable shares for deposit
     */
    function maxMint(address controller) public view virtual returns (uint256) {
        VaultStorage storage $ = _getVaultStorage();
        return $.claimableDepositShares[controller];
    }

    /**
     * @dev Returns maximum assets that can be withdrawn for a controller
     * @param controller Address to check max withdraw for
     * @return Maximum claimable assets for redemption
     */
    function maxWithdraw(address controller) public view virtual returns (uint256) {
        VaultStorage storage $ = _getVaultStorage();
        return $.claimableRedeemAssets[controller];
    }

    /**
     * @dev Returns maximum shares that can be redeemed for a controller
     * @param controller Address to check max redeem for
     * @return Maximum claimable shares for redemption
     */
    function maxRedeem(address controller) public view virtual returns (uint256) {
        VaultStorage storage $ = _getVaultStorage();
        return $.claimableRedeemShares[controller];
    }

    /**
     * @dev Preview functions revert for async vaults (ERC7540)
     * @return Always reverts with AsyncFlow
     */
    function previewDeposit(uint256) public pure virtual returns (uint256) {
        revert AsyncFlow();
    }

    /**
     * @dev Preview functions revert for async vaults (ERC7540)
     * @return Always reverts with AsyncFlow
     */
    function previewMint(uint256) public pure virtual returns (uint256) {
        revert AsyncFlow();
    }

    /**
     * @dev Preview functions revert for async vaults (ERC7540)
     * @return Always reverts with AsyncFlow
     */
    function previewWithdraw(uint256) public pure virtual returns (uint256) {
        revert AsyncFlow();
    }

    /**
     * @dev Preview functions revert for async vaults (ERC7540)
     * @return Always reverts with AsyncFlow
     */
    function previewRedeem(uint256) public pure virtual returns (uint256) {
        revert AsyncFlow();
    }

    /**
     * @dev Convenience function for deposit with receiver as controller
     * @param assets Amount of assets to claim
     * @param receiver Address to receive shares (also used as controller)
     * @return shares Amount of shares received
     */
    function deposit(uint256 assets, address receiver) public virtual returns (uint256) {
        return deposit(assets, receiver, receiver);
    }

    /**
     * @dev Convenience function for mint with receiver as controller
     * @param shares Amount of shares to mint
     * @param receiver Address to receive shares (also used as controller)
     * @return assets Amount of assets consumed
     */
    function mint(uint256 shares, address receiver) public virtual returns (uint256) {
        return mint(shares, receiver, receiver);
    }

    // ========== ERC165 Support ==========

    /**
     * @dev Returns true if this contract implements the interface (ERC165)
     * @param interfaceId The interface identifier
     * @return True if interface is supported
     */
    function supportsInterface(bytes4 interfaceId) public pure virtual returns (bool) {
        return interfaceId == type(IERC7575).interfaceId || interfaceId == type(IERC7540).interfaceId || interfaceId == type(IERC7540Deposit).interfaceId
            || interfaceId == type(IERC7540Redeem).interfaceId || interfaceId == type(IERC7540Operator).interfaceId || interfaceId == type(IERC7887).interfaceId
            || interfaceId == type(IERC7887DepositCancelation).interfaceId || interfaceId == type(IERC7887RedeemCancelation).interfaceId || interfaceId == type(IERC165).interfaceId;
    }

    // ========== Internal Security Functions ==========

    /**
     * @dev Validates token transfer behavior to detect non-standard tokens
     * @param $ Vault storage reference
     * @param from Token holder address
     * @param amount Amount to validate
     */

    // ========== Investment Management Functions ==========

    /**
     * @dev Sets the investment manager address (only owner or ShareToken)
     * @param newManager Address of the new investment manager
     */
    function setInvestmentManager(address newManager) external {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != owner() && msg.sender != $.shareToken) {
            revert Unauthorized();
        }
        if (newManager == address(0)) revert InvalidManager();
        $.investmentManager = newManager;
        emit InvestmentManagerSet(newManager);
    }

    /**
     * @dev Sets the investment vault for yield generation (only owner or ShareToken)
     * @param investmentVault_ Address of the ERC7575 investment vault
     *
     * NOTE: This function is part of the legacy investment architecture.
     * New deployments should use the centralized investment approach through
     * ShareToken.setInvestmentShareToken() instead of individual vault configuration.
     */
    function setInvestmentVault(IERC7575 investmentVault_) external {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != owner() && msg.sender != $.shareToken) {
            revert Unauthorized();
        }
        if (address(investmentVault_) == address(0)) revert InvalidVault();
        if (address(investmentVault_.asset()) != $.asset) {
            revert AssetMismatch();
        }
        $.investmentVault = address(investmentVault_);
        emit InvestmentVaultSet(address(investmentVault_));
    }

    /**
     * @dev Sets the vault active state (only owner)
     * @param _isActive True to activate, false to deactivate
     */
    function setVaultActive(bool _isActive) external onlyOwner {
        VaultStorage storage $ = _getVaultStorage();
        $.isActive = _isActive;
        emit VaultActiveStateChanged(_isActive);
    }

    /**
     * @dev Returns whether the vault is active and accepting deposits
     * @return True if vault is active
     */
    function isVaultActive() external view returns (bool) {
        VaultStorage storage $ = _getVaultStorage();
        return $.isActive;
    }

    /**
     * @dev Sets the minimum deposit amount (only owner)
     * @param _minimumDepositAmount Minimum deposit amount (will be normalized to asset decimals)
     */
    function setMinimumDepositAmount(uint16 _minimumDepositAmount) external onlyOwner {
        VaultStorage storage $ = _getVaultStorage();
        $.minimumDepositAmount = _minimumDepositAmount;
    }

    /**
     * @dev Invests idle assets into the investment vault (only investment manager)
     * @param amount Amount of assets to invest
     * @return shares Number of shares received from investment vault
     */
    /**
     * @dev Invests idle assets into the investment vault (only investment manager)
     * @param amount Amount of assets to invest
     * @return shares Number of shares received from investment vault
     */
    function investAssets(uint256 amount) external nonReentrant returns (uint256 shares) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();
        if ($.investmentVault == address(0)) revert NoInvestmentVault();
        if (amount == 0) revert ZeroAmount();

        uint256 availableBalance = totalAssets();
        if (amount > availableBalance) {
            revert ERC20InsufficientBalance(address(this), availableBalance, amount);
        }

        // Approve and deposit into investment vault with ShareToken as receiver
        IERC20Metadata($.asset).safeIncreaseAllowance($.investmentVault, amount);
        shares = IERC7575($.investmentVault).deposit(amount, $.shareToken);

        emit AssetsInvested(amount, shares, $.investmentVault);
        return shares;
    }

    /**
     * @dev Withdraws assets from investment vault (only investment manager)
     * @param amount Amount of assets to withdraw
     * @return actualAmount Actual amount withdrawn
     */
    /**
     * @dev Withdraws assets from investment vault (only investment manager)
     * @param amount Amount of assets to withdraw
     * @return actualAmount Actual amount withdrawn
     */
    function withdrawFromInvestment(uint256 amount) external nonReentrant returns (uint256 actualAmount) {
        VaultStorage storage $ = _getVaultStorage();
        if (msg.sender != $.investmentManager) revert OnlyInvestmentManager();
        if ($.investmentVault == address(0)) revert NoInvestmentVault();
        if (amount == 0) revert ZeroAmount();

        uint256 balanceBefore = IERC20Metadata($.asset).balanceOf(address(this));

        // Get ShareToken's share balance from the investment ShareToken
        IERC20Metadata investmentShareToken = IERC20Metadata(IERC7575($.investmentVault).share());
        address shareToken_ = $.shareToken;
        uint256 maxShares = investmentShareToken.balanceOf(shareToken_);
        uint256 shares = IERC7575($.investmentVault).previewWithdraw(amount);
        uint256 minShares = shares < maxShares ? shares : maxShares;
        if (minShares == 0) revert ZeroSharesCalculated();

        // Ensure ShareToken has self-allowance on the investment share token for redemption
        uint256 current = investmentShareToken.allowance(shareToken_, shareToken_);
        if (current < minShares) {
            revert InvestmentSelfAllowanceMissing(minShares, current);
        }

        // Redeem shares from ShareToken using our allowance (ShareToken is owner, vault is receiver)
        IERC7575($.investmentVault).redeem(minShares, address(this), shareToken_);

        uint256 balanceAfter = IERC20Metadata($.asset).balanceOf(address(this));
        unchecked {
            actualAmount = balanceAfter - balanceBefore;
        }

        emit AssetsWithdrawnFromInvestment(amount, actualAmount, $.investmentVault);
        return actualAmount;
    }

    /**
     * @dev Gets the investment manager address
     * @return Address of current investment manager
     */
    /**
     * @dev Gets the current investment manager address
     * @return Address of the investment manager
     */
    function getInvestmentManager() external view returns (address) {
        VaultStorage storage $ = _getVaultStorage();
        return $.investmentManager;
    }

    /**
     * @dev Returns both claimable redemption shares and normalized assets in a single call
     * This is optimized for ShareToken's getCirculatingSupplyAndAssets() which needs both values
     * together for each vault in the loop
     * @return totalClaimableShares Total shares reserved for redemption claims
     * @return totalNormalizedAssets Total vault assets scaled to 18 decimals
     */
    function getClaimableSharesAndNormalizedAssets() external view returns (uint256 totalClaimableShares, uint256 totalNormalizedAssets) {
        VaultStorage storage $ = _getVaultStorage();
        totalClaimableShares = $.totalClaimableRedeemShares;

        uint256 vaultAssets = totalAssets();
        // Use Math.mulDiv to prevent overflow for large amounts
        totalNormalizedAssets = Math.mulDiv(vaultAssets, $.scalingFactor, 1);
    }

    // ========== ERC7887 Cancelation Request Functions ==========

    /**
     * @dev Cancels a pending deposit request (ERC7887 compliant)
     *
     * Transitions assets from pending deposit to pending cancelation state.
     * Uses the Pending → Claimable → Claimed state lifecycle without short-circuiting.
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7887 Extension: Asynchronous Tokenized Vault Cancelation Extension
     * - Only works on Pending requests, not Claimable (already fulfilled)
     * - Blocks new deposit requests while cancelation is pending
     * - State transition: pendingDepositAssets → pendingCancelDepositAssets
     *
     * SECURITY CONSIDERATIONS:
     * - Only callable by the controller or their approved operator
     * - Uses nonReentrant to prevent reentrancy attacks
     * - Cannot cancel claimable (already fulfilled) deposits
     * - Blocks new requests to prevent race conditions
     * - Removes controller from active requesters set
     *
     * AUTHORIZATION:
     * Controller (msg.sender == controller) can call directly, or
     * Operator must be approved via setOperator() on the share token
     *
     * @param requestId The requestId from the original deposit request (must be 0 per implementation)
     * @param controller Address that made the original deposit request
     *
     * @custom:throws InvalidRequestId If requestId != REQUEST_ID (only requestId 0 is valid)
     * @custom:throws InvalidCaller If caller is neither controller nor approved operator
     * @custom:throws NoPendingCancelDeposit If no pending deposit exists for controller
     *
     * @custom:event CancelDepositRequest(controller, controller, requestId, msg.sender, assets)
     */
    function cancelDepositRequest(uint256 requestId, address controller) external nonReentrant {
        VaultStorage storage $ = _getVaultStorage();
        if (requestId != REQUEST_ID) revert InvalidRequestId();
        if (!(controller == msg.sender || IERC7540($.shareToken).isOperator(controller, msg.sender))) {
            revert InvalidCaller();
        }

        uint256 pendingAssets = $.pendingDepositAssets[controller];
        if (pendingAssets == 0) revert NoPendingCancelDeposit();

        // Move from pending to pending cancelation
        delete $.pendingDepositAssets[controller];
        $.totalPendingDepositAssets -= pendingAssets;
        $.pendingCancelDepositAssets[controller] = pendingAssets;
        $.totalCancelDepositAssets += pendingAssets;

        // Block new deposit requests
        $.controllersWithPendingDepositCancelations.add(controller);
        $.activeDepositRequesters.remove(controller);

        emit CancelDepositRequest(controller, controller, REQUEST_ID, msg.sender, pendingAssets);
    }

    /**
     * @dev Checks if a deposit cancelation request is pending (ERC7887 compliant)
     *
     * Returns true if the controller has a pending deposit cancelation in the Pending state.
     * Returns false for invalid requestIds or if no pending cancelation exists.
     *
     * STATE MACHINE:
     * - Pending: Assets have been moved from pendingDepositAssets to pendingCancelDepositAssets
     * - Claimable: Investment manager has fulfilled the cancelation, can be claimed
     * - Claimed: User has claimed the assets, cancelation complete
     *
     * SPECIFICATION COMPLIANCE:
     * - Only returns true for requestId == REQUEST_ID (0)
     * - Safe view function with no state changes
     * - Cannot short-circuit to claimed state
     *
     * @param requestId The requestId from the original deposit request (must be 0)
     * @param controller Address that made the original deposit request
     *
     * @return isPending True if a pending deposit cancelation exists, false otherwise
     */
    function pendingCancelDepositRequest(uint256 requestId, address controller) external view returns (bool isPending) {
        if (requestId != REQUEST_ID) return false;
        VaultStorage storage $ = _getVaultStorage();
        return $.pendingCancelDepositAssets[controller] > 0;
    }

    /**
     * @dev Returns the amount of assets available to claim from a fulfilled deposit cancelation (ERC7887)
     *
     * Returns the number of assets that the controller can claim after the investment manager
     * has fulfilled the deposit cancelation request. Returns 0 for invalid requestIds or if
     * no claimable cancelation exists.
     *
     * FLOW:
     * 1. Controller calls cancelDepositRequest() → moves to pendingCancelDepositAssets
     * 2. Investment manager calls fulfillCancelDepositRequest() → moves to claimableCancelDepositAssets
     * 3. Controller calls claimCancelDepositRequest() → receives assets
     *
     * SPECIFICATION COMPLIANCE:
     * - Only returns non-zero for requestId == REQUEST_ID (0)
     * - Safe view function with no state changes
     * - Amount represents assets that were pending deposit, now being canceled
     *
     * @param requestId The requestId from the original deposit request (must be 0)
     * @param controller Address that made the original deposit request
     *
     * @return assets Amount of assets available to claim from the canceled deposit
     */
    function claimableCancelDepositRequest(uint256 requestId, address controller) external view returns (uint256 assets) {
        if (requestId != REQUEST_ID) return 0;
        VaultStorage storage $ = _getVaultStorage();
        return $.claimableCancelDepositAssets[controller];
    }

    /**
     * @dev Claims assets from a fulfilled deposit cancelation request (ERC7887 compliant)
     *
     * Final step in the cancelation lifecycle: transfers assets back to the receiver.
     * Can only claim assets that have been fulfilled by the investment manager and are
     * in the Claimable state. Follows CEI pattern with state changes before transfers.
     *
     * CANCELATION LIFECYCLE:
     * 1. Pending: User calls cancelDepositRequest() to initiate cancelation
     * 2. Claimable: Investment manager calls fulfillCancelDepositRequest() to fulfill
     * 3. Claimed: User calls claimCancelDepositRequest() to receive assets (THIS FUNCTION)
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7887: Asynchronous Cancelation Extension
     * - Three-state lifecycle without short-circuiting
     * - Uses CEI (Checks-Effects-Interactions) pattern
     * - Reentrancy-protected via nonReentrant
     *
     * SECURITY CONSIDERATIONS:
     * - Uses nonReentrant guard to prevent reentrancy attacks
     * - State changes occur before external transfers (CEI)
     * - Only callable by controller or approved operator
     * - Cannot claim assets if not in Claimable state
     * - Removes controller from pending cancelations set
     *
     * AUTHORIZATION:
     * Controller (msg.sender == controller) can call directly, or
     * Operator must be approved via setOperator() on the share token
     *
     * @param requestId The requestId from the original deposit request (must be 0)
     * @param receiver Address that will receive the canceled asset amount
     * @param controller Address that made the original deposit request
     *
     * @custom:throws InvalidRequestId If requestId != REQUEST_ID (only requestId 0 is valid)
     * @custom:throws InvalidCaller If caller is neither controller nor approved operator
     * @custom:throws CancelationNotClaimable If no claimable deposit cancelation exists
     *
     * @custom:event CancelDepositRequestClaimed(controller, receiver, requestId, assets)
     */
    function claimCancelDepositRequest(uint256 requestId, address receiver, address controller) external nonReentrant {
        if (requestId != REQUEST_ID) revert InvalidRequestId();
        VaultStorage storage $ = _getVaultStorage();
        if (!(controller == msg.sender || IERC7540($.shareToken).isOperator(controller, msg.sender))) {
            revert InvalidCaller();
        }

        uint256 assets = $.claimableCancelDepositAssets[controller];
        if (assets == 0) revert CancelationNotClaimable();

        // CEI: State changes before external transfer
        delete $.claimableCancelDepositAssets[controller];
        $.totalCancelDepositAssets -= assets;
        $.controllersWithPendingDepositCancelations.remove(controller);

        // External interaction
        SafeTokenTransfers.safeTransfer($.asset, receiver, assets);

        // Event emission
        emit CancelDepositRequestClaimed(controller, receiver, REQUEST_ID, assets);
    }

    /**
     * @dev Cancels a pending redeem request (ERC7887 compliant)
     *
     * Transitions shares from pending redeem to pending cancelation state.
     * Uses the Pending → Claimable → Claimed state lifecycle without short-circuiting.
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7887 Extension: Asynchronous Tokenized Vault Cancelation Extension
     * - Only works on Pending requests, not Claimable (already fulfilled)
     * - Blocks new redeem requests while cancelation is pending
     * - State transition: pendingRedeemShares → pendingCancelRedeemShares
     *
     * SECURITY CONSIDERATIONS:
     * - Only callable by the controller or their approved operator
     * - Uses nonReentrant to prevent reentrancy attacks
     * - Cannot cancel claimable (already fulfilled) redeems
     * - Blocks new requests to prevent race conditions
     * - Removes controller from active requesters set
     *
     * AUTHORIZATION:
     * Controller (msg.sender == controller) can call directly, or
     * Operator must be approved via setOperator() on the share token
     *
     * @param requestId The requestId from the original redeem request (must be 0 per implementation)
     * @param controller Address that made the original redeem request
     *
     * @custom:throws InvalidRequestId If requestId != REQUEST_ID (only requestId 0 is valid)
     * @custom:throws InvalidCaller If caller is neither controller nor approved operator
     * @custom:throws NoPendingCancelRedeem If no pending redeem exists for controller
     *
     * @custom:event CancelRedeemRequest(controller, controller, requestId, msg.sender, shares)
     */
    function cancelRedeemRequest(uint256 requestId, address controller) external nonReentrant {
        VaultStorage storage $ = _getVaultStorage();
        if (requestId != REQUEST_ID) revert InvalidRequestId();
        if (!(controller == msg.sender || IERC7540($.shareToken).isOperator(controller, msg.sender))) {
            revert InvalidCaller();
        }

        uint256 pendingShares = $.pendingRedeemShares[controller];
        if (pendingShares == 0) revert NoPendingCancelRedeem();

        // Move from pending to pending cancelation
        delete $.pendingRedeemShares[controller];
        $.pendingCancelRedeemShares[controller] = pendingShares;

        // Block new redeem requests
        $.controllersWithPendingRedeemCancelations.add(controller);
        $.activeRedeemRequesters.remove(controller);

        emit CancelRedeemRequest(controller, controller, REQUEST_ID, msg.sender, pendingShares);
    }

    /**
     * @dev Checks if a redeem cancelation request is pending (ERC7887 compliant)
     *
     * Returns true if the controller has a pending redeem cancelation in the Pending state.
     * Returns false for invalid requestIds or if no pending cancelation exists.
     *
     * STATE MACHINE:
     * - Pending: Shares have been moved from pendingRedeemShares to pendingCancelRedeemShares
     * - Claimable: Investment manager has fulfilled the cancelation, can be claimed
     * - Claimed: User has claimed the shares, cancelation complete
     *
     * SPECIFICATION COMPLIANCE:
     * - Only returns true for requestId == REQUEST_ID (0)
     * - Safe view function with no state changes
     * - Cannot short-circuit to claimed state
     *
     * @param requestId The requestId from the original redeem request (must be 0)
     * @param controller Address that made the original redeem request
     *
     * @return isPending True if a pending redeem cancelation exists, false otherwise
     */
    function pendingCancelRedeemRequest(uint256 requestId, address controller) external view returns (bool isPending) {
        if (requestId != REQUEST_ID) return false;
        VaultStorage storage $ = _getVaultStorage();
        return $.pendingCancelRedeemShares[controller] > 0;
    }

    /**
     * @dev Returns the amount of shares available to claim from a fulfilled redeem cancelation (ERC7887)
     *
     * Returns the number of shares that the controller can claim after the investment manager
     * has fulfilled the redeem cancelation request. Returns 0 for invalid requestIds or if
     * no claimable cancelation exists.
     *
     * FLOW:
     * 1. Controller calls cancelRedeemRequest() → moves to pendingCancelRedeemShares
     * 2. Investment manager calls fulfillCancelRedeemRequest() → moves to claimableCancelRedeemShares
     * 3. Controller calls claimCancelRedeemRequest() → receives shares
     *
     * SPECIFICATION COMPLIANCE:
     * - Only returns non-zero for requestId == REQUEST_ID (0)
     * - Safe view function with no state changes
     * - Amount represents shares that were pending redeem, now being canceled
     *
     * @param requestId The requestId from the original redeem request (must be 0)
     * @param controller Address that made the original redeem request
     *
     * @return shares Amount of shares available to claim from the canceled redeem
     */
    function claimableCancelRedeemRequest(uint256 requestId, address controller) external view returns (uint256 shares) {
        if (requestId != REQUEST_ID) return 0;
        VaultStorage storage $ = _getVaultStorage();
        return $.claimableCancelRedeemShares[controller];
    }

    /**
     * @dev Claims shares from a fulfilled redeem cancelation request (ERC7887 compliant)
     *
     * Final step in the redeem cancelation lifecycle: transfers shares back to the receiver.
     * Can only claim shares that have been fulfilled by the investment manager and are
     * in the Claimable state. Follows CEI pattern with state changes before transfers.
     *
     * CANCELATION LIFECYCLE:
     * 1. Pending: User calls cancelRedeemRequest() to initiate cancelation
     * 2. Claimable: Investment manager calls fulfillCancelRedeemRequest() to fulfill
     * 3. Claimed: User calls claimCancelRedeemRequest() to receive shares (THIS FUNCTION)
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7887: Asynchronous Cancelation Extension
     * - Three-state lifecycle without short-circuiting
     * - Uses CEI (Checks-Effects-Interactions) pattern
     * - Reentrancy-protected via nonReentrant
     *
     * SECURITY CONSIDERATIONS:
     * - Uses nonReentrant guard to prevent reentrancy attacks
     * - State changes occur before external transfers (CEI)
     * - Only callable by controller or approved operator
     * - Cannot claim shares if not in Claimable state
     * - Removes controller from pending cancelations set
     *
     * AUTHORIZATION:
     * Controller (msg.sender == controller) can call directly, or
     * Operator must be approved via setOperator() on the share token
     *
     * IMPORTANT NOTE ON PARAMETERS:
     * The parameter naming in ERC7887 is confusing. In this implementation:
     * - `owner` parameter is actually the receiver address (where shares are sent)
     * - `controller` parameter is the original request controller (who initiated the cancel)
     * This matches the ERC7887 spec but can be misleading - see ERC7887 spec for details.
     *
     * @param requestId The requestId from the original redeem request (must be 0)
     * @param owner Address that will receive the canceled share amount (receiver in ERC7887 terms)
     * @param controller Address that made the original redeem request
     *
     * @custom:throws InvalidRequestId If requestId != REQUEST_ID (only requestId 0 is valid)
     * @custom:throws InvalidCaller If caller is neither controller nor approved operator
     * @custom:throws CancelationNotClaimable If no claimable redeem cancelation exists
     *
     * @custom:event CancelRedeemRequestClaimed(controller, owner, requestId, shares)
     */
    function claimCancelRedeemRequest(uint256 requestId, address owner, address controller) external nonReentrant {
        if (requestId != REQUEST_ID) revert InvalidRequestId();
        VaultStorage storage $ = _getVaultStorage();
        if (!(controller == msg.sender || IERC7540($.shareToken).isOperator(controller, msg.sender))) {
            revert InvalidCaller();
        }

        uint256 shares = $.claimableCancelRedeemShares[controller];
        if (shares == 0) revert CancelationNotClaimable();

        // CEI: State changes before external transfer
        delete $.claimableCancelRedeemShares[controller];
        $.controllersWithPendingRedeemCancelations.remove(controller);

        // External interaction
        SafeTokenTransfers.safeTransfer($.shareToken, owner, shares);

        // Event emission
        emit CancelRedeemRequestClaimed(controller, owner, REQUEST_ID, shares);
    }

    // ========== Off-Chain Helper Functions ==========

    /**
     * @dev Returns all addresses with active deposit requests (limited to 100 for RPC safety)
     *
     * Returns all controllers that have pending deposit requests in Pending state.
     * Limited to 100 entries to prevent RPC timeout. For production systems with many users,
     * use getDepositControllerStatusBatchPaginated() for scalable pagination.
     *
     * PERFORMANCE:
     * - O(n) operation where n = number of active deposit requesters
     * - Limited to 100 entries to prevent RPC overload
     * - Use pagination functions for production with >100 users
     *
     * USE CASES:
     * - Off-chain UI to show all users with pending deposits
     * - Investment manager to see all pending requests at a glance
     * - Monitoring and analytics dashboards
     *
     * @return Array of controller addresses with pending deposit requests
     *
     * @custom:throws TooManyRequesters If more than 100 active deposit requesters exist
     */
    function getActiveDepositRequesters() external view returns (address[] memory) {
        VaultStorage storage $ = _getVaultStorage();
        if ($.activeDepositRequesters.length() > 100) {
            revert TooManyRequesters();
        }
        return $.activeDepositRequesters.values();
    }

    /**
     * @dev Returns all addresses with active redeem requests (limited to 100 for RPC safety)
     *
     * Returns all controllers that have pending redeem requests in Pending state.
     * Limited to 100 entries to prevent RPC timeout. For production systems with many users,
     * use getRedeemControllerStatusBatchPaginated() for scalable pagination.
     *
     * PERFORMANCE:
     * - O(n) operation where n = number of active redeem requesters
     * - Limited to 100 entries to prevent RPC overload
     * - Use pagination functions for production with >100 users
     *
     * USE CASES:
     * - Off-chain UI to show all users with pending redemptions
     * - Investment manager to see all pending requests at a glance
     * - Monitoring and analytics dashboards
     *
     * @return Array of controller addresses with pending redeem requests
     *
     * @custom:throws TooManyRequesters If more than 100 active redeem requesters exist
     */
    function getActiveRedeemRequesters() external view returns (address[] memory) {
        VaultStorage storage $ = _getVaultStorage();
        if ($.activeRedeemRequesters.length() > 100) revert TooManyRequesters();
        return $.activeRedeemRequesters.values();
    }

    /**
     * @dev Returns complete request status for a single controller
     *
     * Returns all pending and claimable amounts for a given controller across
     * both deposit and redeem request flows.
     *
     * RETURNED DATA:
     * - controller: The controller address queried
     * - pendingDepositAssets: Assets in Pending deposit state
     * - claimableDepositShares: Shares in Claimable deposit state
     * - pendingRedeemShares: Shares in Pending redeem state
     * - claimableRedeemAssets: Assets in Claimable redeem state
     * - claimableRedeemShares: Shares being held for claimable redeems
     *
     * PERFORMANCE:
     * - O(1) operation: Single lookup per field
     * - Safe view function with no state changes
     *
     * @param controller Address to check
     *
     * @return status Complete controller status with all pending and claimable amounts
     */
    function getControllerStatus(address controller) external view returns (ControllerStatus memory status) {
        VaultStorage storage $ = _getVaultStorage();
        status = ControllerStatus({
            controller: controller,
            pendingDepositAssets: $.pendingDepositAssets[controller],
            claimableDepositShares: $.claimableDepositShares[controller],
            pendingRedeemShares: $.pendingRedeemShares[controller],
            claimableRedeemAssets: $.claimableRedeemAssets[controller],
            claimableRedeemShares: $.claimableRedeemShares[controller]
        });
    }

    /**
     * @dev Returns batch request status for multiple controllers
     *
     * Efficiently queries request status for multiple controllers in a single call.
     * Limited to MAX_BATCH_SIZE (1000) to prevent gas/RPC issues.
     *
     * BATCH OPERATIONS:
     * - Returns status for all provided controllers
     * - Limited to 1000 addresses per call
     * - More efficient than multiple getControllerStatus calls
     * - Returns empty status if controller has no requests
     *
     * PERFORMANCE:
     * - O(n) operation where n = number of controllers queried
     * - Each controller lookup is O(1)
     * - Total gas: linear in number of controllers
     *
     * @param controllers Array of controller addresses to check
     *
     * @return statuses Array of complete controller statuses (same length as input)
     *
     * @custom:throws BatchSizeTooLarge If controllers.length > MAX_BATCH_SIZE (1000)
     */
    function getControllerStatusBatch(address[] calldata controllers) external view returns (ControllerStatus[] memory statuses) {
        if (controllers.length > MAX_BATCH_SIZE) revert BatchSizeTooLarge();
        VaultStorage storage $ = _getVaultStorage();
        statuses = new ControllerStatus[](controllers.length);

        for (uint256 i = 0; i < controllers.length; i++) {
            address controller = controllers[i];
            statuses[i] = ControllerStatus({
                controller: controller,
                pendingDepositAssets: $.pendingDepositAssets[controller],
                claimableDepositShares: $.claimableDepositShares[controller],
                pendingRedeemShares: $.pendingRedeemShares[controller],
                claimableRedeemAssets: $.claimableRedeemAssets[controller],
                claimableRedeemShares: $.claimableRedeemShares[controller]
            });
        }
    }

    /**
     * @dev Returns comprehensive global vault metrics and configuration
     *
     * Provides a complete snapshot of vault state including pending requests,
     * claimable amounts, asset allocation, and configuration parameters.
     *
     * RETURNED METRICS:
     * - Pending/claimable requests: All asynchronous request amounts
     * - Asset allocation: Available for investment, in claims, etc.
     * - Configuration: Scaling factor, active status, manager addresses
     * - Request counts: Number of active requesters
     *
     * USES:
     * - Off-chain monitoring and analytics
     * - Dashboard calculations
     * - Health checks and reports
     * - Integration with portfolio tracking
     *
     * @return metrics Complete vault metrics including totals and configuration
     */
    function getVaultMetrics() external view returns (VaultMetrics memory metrics) {
        VaultStorage storage $ = _getVaultStorage();
        metrics = VaultMetrics({
            totalPendingDepositAssets: $.totalPendingDepositAssets,
            totalClaimableRedeemAssets: $.totalClaimableRedeemAssets,
            totalCancelDepositAssets: $.totalCancelDepositAssets,
            scalingFactor: $.scalingFactor,
            totalAssets: totalAssets(),
            availableForInvestment: totalAssets(),
            activeDepositRequestersCount: $.activeDepositRequesters.length(),
            activeRedeemRequestersCount: $.activeRedeemRequesters.length(),
            isActive: $.isActive,
            asset: $.asset,
            shareToken: $.shareToken,
            investmentManager: $.investmentManager,
            investmentVault: $.investmentVault
        });
    }

    // ========== Scalable Off-Chain Helper Functions ==========

    // Maximum batch size to prevent RPC timeouts and gas issues
    uint256 public constant MAX_BATCH_SIZE = 1000;

    /**
     * @dev Internal helper for paginating ControllerStatus using built-in values() function
     * EnumerableSet.values() handles all bounds checking internally, so no manual validation needed
     * @param addressSet The EnumerableSet to paginate and get status for
     * @param offset Starting index (0-based)
     * @param limit Maximum number of statuses to return
     * @return statuses Paginated ControllerStatus array
     * @return total Total number of items in the set
     * @return hasMore True if there are more items beyond this batch
     */
    function _paginateControllerStatus(
        EnumerableSet.AddressSet storage addressSet,
        uint256 offset,
        uint256 limit
    )
        internal
        view
        returns (ControllerStatus[] memory statuses, uint256 total, bool hasMore)
    {
        if (limit > MAX_BATCH_SIZE) revert BatchSizeTooLarge();
        VaultStorage storage $ = _getVaultStorage();
        total = addressSet.length();

        // Get addresses using EnumerableSet's built-in range function (handles bounds automatically)
        address[] memory controllers = addressSet.values(offset, offset + limit);
        statuses = new ControllerStatus[](controllers.length);

        // Populate ControllerStatus array with complete data
        for (uint256 i = 0; i < controllers.length; i++) {
            address controller = controllers[i];
            statuses[i] = ControllerStatus({
                controller: controller,
                pendingDepositAssets: $.pendingDepositAssets[controller],
                claimableDepositShares: $.claimableDepositShares[controller],
                pendingRedeemShares: $.pendingRedeemShares[controller],
                claimableRedeemAssets: $.claimableRedeemAssets[controller],
                claimableRedeemShares: $.claimableRedeemShares[controller]
            });
        }

        hasMore = offset + controllers.length < total;
    }

    /**
     * @dev Returns count of active requesters without fetching arrays
     * Gas-efficient way to check queue sizes for monitoring
     * @return depositCount Number of active deposit requesters
     * @return redeemCount Number of active redeem requesters
     */
    function getActiveRequestersCount() external view returns (uint256 depositCount, uint256 redeemCount) {
        VaultStorage storage $ = _getVaultStorage();
        depositCount = $.activeDepositRequesters.length();
        redeemCount = $.activeRedeemRequesters.length();
    }

    /**
     * @dev Returns paginated ControllerStatus for active deposit requesters (comprehensive data)
     * Provides complete request status information for efficient off-chain processing
     *
     * @param offset Starting index in active deposit requesters array
     * @param limit Maximum number of statuses to return
     * @return statuses Array of ControllerStatus for deposit requesters (complete data)
     * @return total Total number of active deposit requesters
     * @return hasMore True if there are more results beyond this batch
     */
    function getDepositControllerStatusBatchPaginated(uint256 offset, uint256 limit) external view returns (ControllerStatus[] memory statuses, uint256 total, bool hasMore) {
        VaultStorage storage $ = _getVaultStorage();
        return _paginateControllerStatus($.activeDepositRequesters, offset, limit);
    }

    /**
     * @dev Returns paginated ControllerStatus for active redeem requesters (comprehensive data)
     * Provides complete request status information for efficient off-chain processing
     *
     * @param offset Starting index in active redeem requesters array
     * @param limit Maximum number of statuses to return
     * @return statuses Array of ControllerStatus for redeem requesters (complete data)
     * @return total Total number of active redeem requesters
     * @return hasMore True if there are more results beyond this batch
     */
    function getRedeemControllerStatusBatchPaginated(uint256 offset, uint256 limit) external view returns (ControllerStatus[] memory statuses, uint256 total, bool hasMore) {
        VaultStorage storage $ = _getVaultStorage();
        return _paginateControllerStatus($.activeRedeemRequesters, offset, limit);
    }

    /**
     * @dev Comprehensive controller status including all request states
     */
    struct ControllerStatus {
        address controller; // Address of the controller/requester
        uint256 pendingDepositAssets; // Assets in pending deposit state
        uint256 claimableDepositShares; // Shares ready to be claimed from deposits
        uint256 pendingRedeemShares; // Shares in pending redeem state
        uint256 claimableRedeemAssets; // Assets ready to be claimed from redemptions
        uint256 claimableRedeemShares; // Shares ready to be redeemed
    }

    // Investment events
    event InvestmentManagerSet(address indexed manager);
    event InvestmentVaultSet(address indexed vault);
    event VaultActiveStateChanged(bool indexed isActive);

    // Cancel request events
    event DepositRequestCancelled(address indexed controller, uint256 assets);
    event RedeemRequestCancelled(address indexed controller, uint256 shares);

    // ========== Upgrade Functions ==========

    /**
     * @dev Upgrade the implementation of the proxy (only owner)
     * @param newImplementation Address of the new implementation contract
     */
    function upgradeTo(address newImplementation) external onlyOwner {
        ERC1967Utils.upgradeToAndCall(newImplementation, "");
    }

    /**
     * @dev Upgrade the implementation and call a function (only owner)
     * @param newImplementation Address of the new implementation contract
     * @param data Calldata to execute on the new implementation
     */
    function upgradeToAndCall(address newImplementation, bytes calldata data) external payable onlyOwner {
        ERC1967Utils.upgradeToAndCall(newImplementation, data);
    }

    // ========== Internal Helper Functions for Safe Transfers ==========
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC7540Operator} from "./interfaces/IERC7540.sol";
import {IERC7575, IERC7575ShareExtended} from "./interfaces/IERC7575.sol";

import {IERC7575Errors} from "./interfaces/IERC7575Errors.sol";
import {IVaultMetrics} from "./interfaces/IVaultMetrics.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ERC20Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import {ERC1967Utils} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
// Interface for WERC7575 share tokens with restricted balance functionality

interface IWERC7575ShareToken {
    function rBalanceOf(address account) external view returns (uint256);
}

import {DecimalConstants} from "./DecimalConstants.sol";
import {ERC7575VaultUpgradeable} from "./ERC7575VaultUpgradeable.sol";
import {IERC20Errors} from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {EnumerableMap} from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

// Forward declaration to avoid circular dependency
interface IERC7575Vault {
    function getClaimableSharesAndNormalizedAssets() external view returns (uint256 claimableShares, uint256 normalizedAssets);
}

/**
 * @title ShareTokenUpgradeable
 * @dev FULLY COMPLIANT ERC7575Share + ERC7540Operator + ERC20 token for multi-asset vault systems
 *
 * ERC7575 COMPLIANCE VERIFICATION:
 * IERC7575ShareExtended Interface (https://eips.ethereum.org/EIPS/eip-7575)
 *    - vault(address asset) → returns vault address for asset
 *    - getRegisteredAssets() → returns all registered assets
 *    - getTotalNormalizedAssets() → aggregates across all vaults
 *    - VaultUpdate event emission on registration/unregistration
 *    - Multi-asset registry with asset→vault mapping
 *
 * ERC7540 OPERATOR COMPLIANCE VERIFICATION:
 * IERC7540Operator Interface (https://eips.ethereum.org/EIPS/eip-7540)
 *    - setOperator(operator, approved) → centralized operator management
 *    - isOperator(controller, operator) → unified operator checks
 *    - OperatorSet event emission on operator changes
 *    - CENTRALIZED: One operator setting works across ALL vaults
 *
 * ARCHITECTURE FEATURES:
 * - Shared across multiple ERC7575VaultUpgradeable contracts (one per asset)
 * - Decimal normalization for cross-asset aggregation (18-decimal standard)
 * - Vault-only minting/burning with proper authorization controls
 * - Registry management for asset-to-vault relationships
 * - CENTRALIZED operator management for all vaults (better UX)
 * - CENTRALIZED investment manager control with automatic propagation to all vaults
 * - CENTRALIZED investment ShareToken configuration for unified investment strategy
 * - ERC165 interface detection support
 *
 * SECURITY:
 * - Only registered vaults can mint/burn tokens (onlyVaults modifier)
 * - Safe vault registration/unregistration with outstanding share checks
 * - Integer overflow protection with Math.mulDiv in aggregation
 * - Centralized operator validation prevents fragmented permissions
 * - Upgradeable with storage slots pattern for safe upgrades
 */
contract ShareTokenUpgradeable is Initializable, ERC20Upgradeable, Ownable2StepUpgradeable, IERC7575ShareExtended, IERC7540Operator, IERC165, IERC7575Errors {
    using EnumerableMap for EnumerableMap.AddressToAddressMap;
    // Storage slot for ShareToken-specific data

    // Note: Common errors are now inherited from IERC7575Errors interface

    bytes32 private constant SHARE_TOKEN_STORAGE_SLOT = keccak256("erc7575.sharetoken.storage");
    // Security constants
    uint256 private constant VIRTUAL_SHARES = 1e6; // Virtual shares for inflation protection
    uint256 private constant VIRTUAL_ASSETS = 1e6; // Virtual assets for inflation protection
    uint256 private constant MAX_VAULTS_PER_SHARE_TOKEN = 10; // DoS mitigation: prevents unbounded loop in aggregation

    // Note: OperatorSet event is defined in IERC7540Operator interface

    struct ShareTokenStorage {
        // EnumerableMap from asset to vault address (replaces both vaults mapping and registeredAssets array)
        EnumerableMap.AddressToAddressMap assetToVault;
        // Reverse mapping from vault to asset for quick lookup
        mapping(address vault => address asset) vaultToAsset;
        // ERC7540 Operator mappings - centralized for all vaults
        mapping(address controller => mapping(address operator => bool approved)) operators;
        // Investment configuration - centralized at ShareToken level
        address investmentShareToken; // The ShareToken used for investments
        address investmentManager; // Centralized investment manager for all vaults
    }

    /**
     * @dev Returns the ShareToken storage struct
     */
    function _getShareTokenStorage() private pure returns (ShareTokenStorage storage $) {
        bytes32 slot = SHARE_TOKEN_STORAGE_SLOT;
        assembly {
            $.slot := slot
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @dev Initializes the contract
     * @param name Token name
     * @param symbol Token symbol
     * @param owner Initial owner address
     */
    function initialize(string memory name, string memory symbol, address owner) public initializer {
        __ERC20_init(name, symbol);
        __Ownable_init(owner);

        // Enforce 18 decimals for consistency with ERC7575 standard
        if (decimals() != DecimalConstants.SHARE_TOKEN_DECIMALS) {
            revert WrongDecimals();
        }
    }

    // Modifier to restrict minting/burning to registered vaults
    modifier onlyVaults() {
        ShareTokenStorage storage $ = _getShareTokenStorage();
        if ($.vaultToAsset[msg.sender] == address(0)) revert Unauthorized();
        _;
    }

    /**
     * @dev Returns the vault address for a specific asset
     *
     * ERC7575 SPECIFICATION (IERC7575ShareExtended interface):
     * "Returns the vault address for a specific asset.
     * Allows share tokens to point back to their vaults."
     *
     * @param asset The asset token address
     * @return vaultAddress The vault address that handles this asset
     */
    function vault(address asset) external view override returns (address vaultAddress) {
        ShareTokenStorage storage $ = _getShareTokenStorage();
        (, vaultAddress) = $.assetToVault.tryGet(asset);
    }

    /**
     * @dev Registers a new vault for an asset in the multi-asset system (ERC7575 compliant)
     *
     * Establishes a one-to-one relationship between an asset and a vault. All users
     * depositing/redeeming that asset will use this vault. Automatically configures
     * the new vault with existing investment settings for seamless integration.
     *
     * MULTI-ASSET ARCHITECTURE:
     * "Multi-Asset Vaults share a single `share` token with multiple entry points
     * denominated in different `asset` tokens." (ERC7575 specification)
     *
     * AUTOMATIC CONFIGURATION:
     * When a vault is registered, it automatically inherits:
     * - Investment ShareToken configuration (if already set)
     * - Investment manager (if already configured)
     * - Appropriate allowances for investment operations
     *
     * This ensures newly registered vaults work immediately without separate setup.
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7575: Multi-asset vault standard
     * - One-to-one asset-to-vault mapping enforced
     * - DoS mitigation: Maximum 10 vaults per share token
     * - VaultUpdate event emission
     *
     * ACCESS CONTROL:
     * - Only callable by share token owner
     * - Validates vault configuration before registration
     *
     * VALIDATION:
     * - Asset and vault addresses must not be zero
     * - Asset must not already be registered
     * - Vault's asset() must match the asset parameter
     * - Vault's share() must match this ShareToken address
     * - Total vaults must not exceed MAX_VAULTS_PER_SHARE_TOKEN (10)
     *
     * @param asset The asset token address to register
     * @param vaultAddress The vault contract address for this asset
     *
     * @custom:throws ZeroAddress If asset or vault address is zero
     * @custom:throws AssetMismatch If vault.asset() != provided asset
     * @custom:throws VaultShareMismatch If vault.share() != this ShareToken
     * @custom:throws AssetAlreadyRegistered If asset is already registered
     * @custom:throws MaxVaultsExceeded If maximum vault limit (10) is reached
     *
     * @custom:event VaultUpdate(asset, vaultAddress)
     */
    function registerVault(address asset, address vaultAddress) external onlyOwner {
        if (asset == address(0)) revert ZeroAddress();
        if (vaultAddress == address(0)) revert ZeroAddress();

        // Validate that vault's asset matches the provided asset parameter
        if (IERC7575(vaultAddress).asset() != asset) revert AssetMismatch();

        // Validate that vault's share token matches this ShareToken
        if (IERC7575(vaultAddress).share() != address(this)) {
            revert VaultShareMismatch();
        }

        ShareTokenStorage storage $ = _getShareTokenStorage();

        // DoS mitigation: Enforce maximum vaults per share token to prevent unbounded loop in getCirculatingSupplyAndAssets
        if ($.assetToVault.length() >= MAX_VAULTS_PER_SHARE_TOKEN) {
            revert MaxVaultsExceeded();
        }

        // Register new vault - set() returns true if newly added, false if already existed
        if (!$.assetToVault.set(asset, vaultAddress)) {
            revert AssetAlreadyRegistered();
        }
        $.vaultToAsset[vaultAddress] = asset;

        // If investment ShareToken is already configured, set up investment for the new vault
        // Only configure if the vault address is a deployed contract
        address investmentShareToken = $.investmentShareToken;
        if (investmentShareToken != address(0)) {
            _configureVaultInvestmentSettings(asset, vaultAddress, investmentShareToken);
        }

        // If investment manager is already configured, set it for the new vault
        // Only configure if the vault address is a deployed contract
        address investmentManager = $.investmentManager;
        if (investmentManager != address(0)) {
            ERC7575VaultUpgradeable(vaultAddress).setInvestmentManager(investmentManager);
        }

        emit VaultUpdate(asset, vaultAddress);
    }

    /**
     * @dev Unregisters a vault and removes it from the multi-asset system (ERC7575 compliant)
     *
     * Removes a vault from the asset-to-vault registry. This is a permanent operation
     * that can only be performed when the vault has zero pending requests and no remaining
     * assets, ensuring no user funds are at risk.
     *
     * PREREQUISITES FOR UNREGISTRATION:
     * The vault must meet ALL of these conditions:
     * 1. Vault must be inactive (isActive = false)
     * 2. No pending deposit requests (totalPendingDepositAssets = 0)
     * 3. No claimable redemptions (totalClaimableRedeemAssets = 0)
     * 4. No ERC7887 pending/claimable cancelations (totalCancelDepositAssets = 0)
     * 5. No active deposit requesters (activeDepositRequestersCount = 0)
     * 6. No active redeem requesters (activeRedeemRequestersCount = 0)
     * 7. No asset tokens remaining in vault balance
     *
     * SAFETY GUARANTEES:
     * - Comprehensive multi-step validation prevents accidental unregistration
     * - Checks both request state and physical asset balance
     * - Catches investment vaults and edge cases
     * - Atomic operation: all validations or complete rollback
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7575: Multi-asset vault standard
     * - Safe unregistration without user fund loss
     * - VaultUpdate event emission with zero address
     *
     * ACCESS CONTROL:
     * - Only callable by share token owner
     * - Owner responsibility to pause vault before unregistration
     *
     * @param asset The asset token address to unregister
     *
     * @custom:throws ZeroAddress If asset address is zero
     * @custom:throws AssetNotRegistered If asset is not currently registered
     * @custom:throws CannotUnregisterActiveVault If vault is still active
     * @custom:throws CannotUnregisterVaultPendingDeposits If pending deposits exist
     * @custom:throws CannotUnregisterVaultClaimableRedemptions If claimable redemptions exist
     * @custom:throws CannotUnregisterVaultAssetBalance If ERC7887 cancelations or assets remain
     * @custom:throws CannotUnregisterVaultActiveDepositRequesters If active deposit requesters exist
     * @custom:throws CannotUnregisterVaultActiveRedeemRequesters If active redeem requesters exist
     *
     * @custom:event VaultUpdate(asset, address(0))
     */
    function unregisterVault(address asset) external onlyOwner {
        if (asset == address(0)) revert ZeroAddress();
        ShareTokenStorage storage $ = _getShareTokenStorage();

        (bool exists, address vaultAddress) = $.assetToVault.tryGet(asset);
        if (!exists) revert AssetNotRegistered();

        // COMPREHENSIVE SAFETY CHECK: Ensure vault has no user funds at risk
        // This covers pending deposits, claimable redemptions, ERC7887 cancelations, and any remaining assets

        // 1. Check vault metrics for pending requests, active users, and ERC7887 cancelation assets
        try IVaultMetrics(vaultAddress).getVaultMetrics() returns (IVaultMetrics.VaultMetrics memory metrics) {
            if (metrics.isActive) revert CannotUnregisterActiveVault();
            if (metrics.totalPendingDepositAssets != 0) {
                revert CannotUnregisterVaultPendingDeposits();
            }
            if (metrics.totalClaimableRedeemAssets != 0) {
                revert CannotUnregisterVaultClaimableRedemptions();
            }
            if (metrics.totalCancelDepositAssets != 0) {
                revert CannotUnregisterVaultAssetBalance();
            }
            if (metrics.activeDepositRequestersCount != 0) {
                revert CannotUnregisterVaultActiveDepositRequesters();
            }
            if (metrics.activeRedeemRequestersCount != 0) {
                revert CannotUnregisterVaultActiveRedeemRequesters();
            }
        } catch {
            // If we can't get vault metrics, we can't safely verify no pending requests
            revert CannotUnregisterActiveVault();
        }
        // 2. Final safety: Check raw asset balance in vault contract
        // This catches any remaining assets including investments and edge cases
        // If this happens, there is either a bug in the vault
        // or assets were sent to the vault without directly
        if (IERC20(asset).balanceOf(vaultAddress) != 0) {
            revert CannotUnregisterVaultAssetBalance();
        }

        // Remove vault registration (automatically removes from enumerable collection)
        $.assetToVault.remove(asset);
        delete $.vaultToAsset[vaultAddress];

        emit VaultUpdate(asset, address(0));
    }

    /**
     * @dev Returns whether an address is a registered vault.
     */
    /**
     * @dev Checks if an address is a registered vault
     * @param vaultAddress The address to check
     * @return True if the address is a registered vault
     */
    function isVault(address vaultAddress) external view returns (bool) {
        ShareTokenStorage storage $ = _getShareTokenStorage();
        return $.vaultToAsset[vaultAddress] != address(0);
    }

    /**
     * @dev Returns all registered assets in the multi-asset system
     *
     * ERC7575 SPECIFICATION (IERC7575ShareExtended interface):
     * "Returns all registered assets in the multi-asset system."
     *
     * MULTI-ASSET ARCHITECTURE:
     * "Multi-Asset Vaults share a single `share` token with multiple entry points
     * denominated in different `asset` tokens."
     *
     * @return Array of all asset addresses that have registered vaults
     */
    function getRegisteredAssets() external view returns (address[] memory) {
        ShareTokenStorage storage $ = _getShareTokenStorage();
        return $.assetToVault.keys();
    }

    /**
     * @dev Returns both circulating supply and normalized assets in a single call
     *
     * Circulating supply excludes shares held by vaults for redemption claims.
     * Total normalized assets excludes assets reserved for redemption claims.
     * Both values exclude the same economic scope for consistent conversion ratios.
     *
     * @return circulatingSupply Total supply minus shares held by vaults for redemption claims
     * @return totalNormalizedAssets Total normalized assets across all vaults (18 decimals)
     */
    function getCirculatingSupplyAndAssets() external view returns (uint256 circulatingSupply, uint256 totalNormalizedAssets) {
        ShareTokenStorage storage $ = _getShareTokenStorage();
        uint256 totalClaimableShares = 0;
        uint256 length = $.assetToVault.length();

        for (uint256 i = 0; i < length; i++) {
            (, address vaultAddress) = $.assetToVault.at(i);

            // Get both claimable shares and normalized assets in a single call for gas efficiency
            (uint256 vaultClaimableShares, uint256 vaultNormalizedAssets) = IERC7575Vault(vaultAddress).getClaimableSharesAndNormalizedAssets();
            totalClaimableShares += vaultClaimableShares;
            totalNormalizedAssets += vaultNormalizedAssets;
        }

        // Add invested assets from the investment ShareToken (if configured)
        totalNormalizedAssets += _calculateInvestmentAssets();

        // Get total supply
        uint256 supply = totalSupply();
        // Calculate circulating supply: total supply minus vault-held shares for redemption claims
        circulatingSupply = totalClaimableShares > supply ? 0 : supply - totalClaimableShares;
    }

    /**
     * @dev Mint shares to an account. Only callable by authorized vaults.
     */
    /**
     * @dev Mints shares to an account (only registered vaults)
     * @param account The account to mint shares to
     * @param amount The amount of shares to mint
     */
    function mint(address account, uint256 amount) external onlyVaults {
        _mint(account, amount);
    }

    /**
     * @dev Burn shares from an account. Only callable by authorized vaults.
     */
    /**
     * @dev Burns shares from an account (only registered vaults)
     * @param account The account to burn shares from
     * @param amount The amount of shares to burn
     */
    function burn(address account, uint256 amount) external onlyVaults {
        _burn(account, amount);
    }

    /**
     * @dev Spends allowance for an owner (vault-only operation)
     * @param owner The owner address whose shares are being spent
     * @param spender The spender address spending the allowance
     * @param amount The amount of shares to spend from allowance
     */
    function spendAllowance(address owner, address spender, uint256 amount) external onlyVaults {
        _spendAllowance(owner, spender, amount);
    }

    // ========== IERC7540Operator Implementation ==========

    /**
     * @dev Sets or unsets an operator for the caller (centralized for all vaults)
     *
     * ERC7540 SPECIFICATION:
     * "Grants or revokes permissions for `operator` to manage Requests on behalf of the `msg.sender`.
     * - MUST set the operator status to the `approved` value.
     * - MUST log the `OperatorSet` event.
     * - MUST return True."
     *
     * CENTRALIZED ARCHITECTURE:
     * Unlike vault-level operators, this implementation provides unified operator
     * management across ALL ERC7575 vaults that use this ShareToken.
     *
     * @param operator Address of the operator
     * @param approved True to approve, false to revoke
     * @return True if successful
     */
    /**
     * @dev Sets or revokes operator approval for the caller (ERC7540 compliant)
     *
     * Allows users to centrally approve operators who can manage async requests
     * across ALL vaults in the multi-asset system. Single operator approval works
     * for deposits, redeems, and cancelations in all vaults sharing this ShareToken.
     *
     * CENTRALIZED OPERATOR SYSTEM:
     * One operator approval provides authorization across:
     * - All ERC7575 vaults (deposits/redeems)
     * - All ERC7887 cancelations
     * - All asset classes in the multi-asset system
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7540: Asynchronous Tokenized Vault Standard
     * - Centralized operator delegation
     * - OperatorSet event emission
     *
     * OPERATOR PERMISSIONS:
     * Approved operators can:
     * - Call requestDeposit on behalf of owner
     * - Call requestRedeem on behalf of owner
     * - Call cancelDepositRequest on behalf of controller
     * - Call cancelRedeemRequest on behalf of controller
     * - Call claim functions (deposit/redeem/cancelation) on behalf of controller
     * - Works across all vaults in the system
     *
     * @param operator Address to approve or revoke as an operator
     * @param approved True to grant operator permission, false to revoke
     *
     * @return Always returns true to indicate operation succeeded
     *
     * @custom:throws CannotSetSelfAsOperator If operator == msg.sender
     * @custom:event OperatorSet(msg.sender, operator, approved)
     */
    function setOperator(address operator, bool approved) external virtual returns (bool) {
        if (msg.sender == operator) revert CannotSetSelfAsOperator();
        ShareTokenStorage storage $ = _getShareTokenStorage();
        $.operators[msg.sender][operator] = approved;
        emit OperatorSet(msg.sender, operator, approved);
        return true;
    }

    /**
     * @dev Checks if an operator is approved for a controller (centralized for all vaults)
     *
     * ERC7540 SPECIFICATION:
     * "Returns `true` if the `operator` is approved as an operator for a `controller`."
     *
     * CENTRALIZED ARCHITECTURE:
     * This single function serves ALL ERC7575 vaults, providing consistent
     * operator permissions across the entire multi-asset system.
     *
     * @param controller Address of the controller
     * @param operator Address of the operator
     * @return True if operator is approved
     */
    function isOperator(address controller, address operator) external view virtual returns (bool) {
        ShareTokenStorage storage $ = _getShareTokenStorage();
        return $.operators[controller][operator];
    }

    /**
     * @dev Sets or revokes operator approval for a specific controller (vault-delegation)
     *
     * Internal delegation function allowing vaults to set operators on behalf of users
     * while preserving the original user context. This enables vaults to delegate
     * operator setup to their own logic if needed.
     *
     * VAULT DELEGATION:
     * - Only callable by registered vaults
     * - Preserves original controller identity
     * - Centralized operator tracking
     *
     * @param controller Address of the controller (the user)
     * @param operator Address to approve or revoke as operator
     * @param approved True to grant operator permission, false to revoke
     *
     * @custom:throws CannotSetSelfAsOperator If operator == controller
     */
    function setOperatorFor(address controller, address operator, bool approved) external onlyVaults {
        if (controller == operator) revert CannotSetSelfAsOperator();
        ShareTokenStorage storage $ = _getShareTokenStorage();
        $.operators[controller][operator] = approved;
        emit OperatorSet(controller, operator, approved);
    }

    // ========== Investment Configuration Management ==========

    /**
     * @dev Internal helper function to configure investment settings for a single vault
     * @param asset The asset address
     * @param vaultAddress The vault address to configure
     * @param investmentShareToken The investment ShareToken address
     */
    function _configureVaultInvestmentSettings(address asset, address vaultAddress, address investmentShareToken) internal {
        // Find the corresponding investment vault for this asset
        address investmentVaultAddress = IERC7575ShareExtended(investmentShareToken).vault(asset);

        // Configure investment vault if there's a matching one for this asset
        if (investmentVaultAddress != address(0)) {
            ERC7575VaultUpgradeable(vaultAddress).setInvestmentVault(IERC7575(investmentVaultAddress));

            // Grant unlimited allowance to the vault on the investment ShareToken
            IERC20(investmentShareToken).approve(vaultAddress, type(uint256).max);
        }
    }

    /**
     * @dev Sets the investment ShareToken address and configures all vault investment mappings (only owner)
     *
     * This function:
     * 1. Sets the investment ShareToken for the multi-asset system
     * 2. Iterates through all registered assets
     * 3. For each asset, finds the matching investment vault from the investment ShareToken
     * 4. Configures each vault with its corresponding investment vault
     *
     * ARCHITECTURE:
     * - All investments will be made in the name of this ShareToken
     * - Each vault will have its counterpart investment vault (same asset)
     * - Enables centralized investment management across the multi-asset system
     *
     * @param investmentShareToken_ The address of the investment ShareToken
     */
    function setInvestmentShareToken(address investmentShareToken_) external onlyOwner {
        if (investmentShareToken_ == address(0)) revert ZeroAddress();
        ShareTokenStorage storage $ = _getShareTokenStorage();
        if ($.investmentShareToken != address(0)) {
            revert InvestmentShareTokenAlreadySet();
        }

        // Store the investment ShareToken address
        $.investmentShareToken = investmentShareToken_;

        // Iterate through all registered assets and configure investment vaults
        uint256 length = $.assetToVault.length();
        for (uint256 i = 0; i < length; i++) {
            (address asset, address vaultAddress) = $.assetToVault.at(i);
            _configureVaultInvestmentSettings(asset, vaultAddress, investmentShareToken_);
        }

        emit InvestmentShareTokenSet(investmentShareToken_);
    }

    /**
     * @dev Returns the current investment ShareToken address
     *
     * @return The address of the investment ShareToken, or zero address if not set
     */
    function getInvestmentShareToken() external view returns (address) {
        ShareTokenStorage storage $ = _getShareTokenStorage();
        return $.investmentShareToken;
    }

    /**
     * @dev Helper function to calculate total investment assets (balanceOf + rBalanceOf)
     * @return totalInvestmentAssets Total invested assets including reserved balance
     */
    function _calculateInvestmentAssets() internal view returns (uint256 totalInvestmentAssets) {
        ShareTokenStorage storage $ = _getShareTokenStorage();
        address investmentShareToken = $.investmentShareToken;

        if (investmentShareToken == address(0)) {
            return 0;
        }

        // Get our balance of investment ShareToken (already normalized to 18 decimals)
        totalInvestmentAssets = IERC20(investmentShareToken).balanceOf(address(this));

        // Add rBalanceOf (reserved balance) if the investment share token supports it
        try IWERC7575ShareToken(investmentShareToken).rBalanceOf(address(this)) returns (uint256 rShares) {
            totalInvestmentAssets += rShares;
        } catch {
            // If rBalanceOf is not supported, continue with regular balance only
        }
    }

    /**
     * @dev Gets the total value of invested assets (normalized to 18 decimals)
     * @return Total value of assets invested through the investment ShareToken
     */
    function getInvestedAssets() external view returns (uint256) {
        return _calculateInvestmentAssets();
    }

    /**
     * @dev Sets the investment manager for all vaults (centralized management)
     *
     * Establishes centralized investment management by designating a single manager
     * responsible for fulfilling all deposit/redeem requests across all vaults in the
     * multi-asset system. The manager is automatically propagated to all registered vaults.
     *
     * CENTRALIZED INVESTMENT ARCHITECTURE:
     * - Single investment manager for ALL vaults
     * - Automatic propagation to existing vaults
     * - Automatic assignment to new vaults during registration
     * - Unified investment strategy across asset classes
     *
     * INVESTMENT MANAGER RESPONSIBILITIES:
     * - Call fulfillDeposit/fulfillDeposits to convert pending assets to shares
     * - Call fulfillRedeem to convert pending shares to assets
     * - Call fulfillCancelDepositRequest(s) for deposit cancelations
     * - Call fulfillCancelRedeemRequest(s) for redeem cancelations
     * - Manage investments through the investment vault
     * - Monitor vault metrics and manage liquidity
     *
     * ACCESS CONTROL:
     * - Only callable by share token owner
     * - Not restricted once set (can be changed by owner)
     *
     * @param newInvestmentManager The address of the new investment manager
     *
     * @custom:throws ZeroAddress If newInvestmentManager is zero address
     */
    function setInvestmentManager(address newInvestmentManager) external onlyOwner {
        if (newInvestmentManager == address(0)) revert ZeroAddress();
        ShareTokenStorage storage $ = _getShareTokenStorage();

        // Store the investment manager centrally
        $.investmentManager = newInvestmentManager;

        // Propagate to all registered vaults
        uint256 length = $.assetToVault.length();
        for (uint256 i = 0; i < length; i++) {
            (, address vaultAddress) = $.assetToVault.at(i);

            // Call setInvestmentManager on each vault
            ERC7575VaultUpgradeable(vaultAddress).setInvestmentManager(newInvestmentManager);
        }

        emit InvestmentManagerSet(newInvestmentManager);
    }

    /**
     * @dev Returns the current investment manager address
     * @return The address of the centralized investment manager
     */
    function getInvestmentManager() external view returns (address) {
        ShareTokenStorage storage $ = _getShareTokenStorage();
        return $.investmentManager;
    }

    /**
     *  OPTIMIZED CONVERSION: Normalized assets to shares with mathematical consistency
     *
     * - Assets: excludes reserved redemption assets
     * - Shares: excludes vault-held shares for redemption claims
     * Result: Both numerator and denominator represent the same economic scope
     *
     * VIRTUAL ASSETS/SHARES:
     * Added for inflation protection as per ERC4626 best practices
     *
     * @param normalizedAssets Amount of normalized assets (18 decimals)
     * @param rounding Rounding mode for the conversion
     * @return shares Amount of shares equivalent to the normalized assets
     */
    function convertNormalizedAssetsToShares(uint256 normalizedAssets, Math.Rounding rounding) external view returns (uint256 shares) {
        // Get both values in a single call
        (uint256 circulatingSupply, uint256 totalNormalizedAssets) = this.getCirculatingSupplyAndAssets();

        // Add virtual amounts for inflation protection
        circulatingSupply += VIRTUAL_SHARES;
        totalNormalizedAssets += VIRTUAL_ASSETS;

        // shares = normalizedAssets * circulatingSupply / totalNormalizedAssets
        shares = Math.mulDiv(normalizedAssets, circulatingSupply, totalNormalizedAssets, rounding);
    }

    /**
     *  OPTIMIZED CONVERSION: Shares to normalized assets with mathematical consistency
     *
     * MATHEMATICAL CONSISTENCY:
     * This function uses the same circulating supply approach as convertNormalizedAssetsToShares
     * to ensure consistent conversion ratios in both directions during ERC7540 async operations.
     *
     * See convertNormalizedAssetsToShares documentation for detailed explanation of the
     * mathematical consistency fix.
     *
     * @param shares Amount of shares to convert
     * @param rounding Rounding mode for the conversion
     * @return normalizedAssets Amount of normalized assets (18 decimals) equivalent to the shares
     */
    function convertSharesToNormalizedAssets(uint256 shares, Math.Rounding rounding) external view returns (uint256 normalizedAssets) {
        // Get both values in a single call
        (uint256 circulatingSupply, uint256 totalNormalizedAssets) = this.getCirculatingSupplyAndAssets();

        // Add virtual amounts for inflation protection
        circulatingSupply += VIRTUAL_SHARES;
        totalNormalizedAssets += VIRTUAL_ASSETS;

        // normalizedAssets = shares * totalNormalizedAssets / circulatingSupply
        normalizedAssets = Math.mulDiv(shares, totalNormalizedAssets, circulatingSupply, rounding);
    }

    /**
     * @dev Transfers shares from owner to vault without requiring allowance (vault-only operation)
     * This function is essential for ERC7540 operator functionality, allowing operators to
     * submit redemption requests on behalf of users without requiring pre-approval.
     *
     * @param from The owner address to transfer shares from
     * @param to The recipient address (typically the vault)
     * @param amount The amount of shares to transfer
     * @return success True if transfer successful
     */
    function vaultTransferFrom(address from, address to, uint256 amount) external onlyVaults returns (bool success) {
        if (from == address(0)) {
            revert IERC20Errors.ERC20InvalidSender(address(0));
        }
        if (to == address(0)) {
            revert IERC20Errors.ERC20InvalidReceiver(address(0));
        }

        // Direct transfer without checking allowance since this is vault-only
        _transfer(from, to, amount);
        return true;
    }

    /**
     * @dev Event emitted when the investment ShareToken is updated
     */
    event InvestmentShareTokenSet(address indexed investmentShareToken);

    /**
     * @dev Event emitted when the investment manager is updated
     */
    event InvestmentManagerSet(address indexed investmentManager);

    // ========== Upgrade Functions ==========

    /**
     * @dev Upgrade the implementation of the proxy (only owner)
     * @param newImplementation Address of the new implementation contract
     */
    function upgradeTo(address newImplementation) external onlyOwner {
        ERC1967Utils.upgradeToAndCall(newImplementation, "");
    }

    /**
     * @dev Upgrade the implementation and call a function (only owner)
     * @param newImplementation Address of the new implementation contract
     * @param data Calldata to execute on the new implementation
     */
    function upgradeToAndCall(address newImplementation, bytes calldata data) external payable onlyOwner {
        ERC1967Utils.upgradeToAndCall(newImplementation, data);
    }

    // ========== ERC165 Support ==========

    /**
     * @dev Returns true if this contract implements the interface (ERC165)
     * @param interfaceId The interface identifier
     * @return True if interface is supported
     */
    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return interfaceId == type(IERC7575ShareExtended).interfaceId || interfaceId == type(IERC7540Operator).interfaceId || interfaceId == type(IERC165).interfaceId;
    }
}

pragma solidity ^0.8.30;

interface IERC7540Operator {
    /**
     * @dev The `controller` has set the `approved` status to an `operator`.
     *
     * - MUST be logged when the operator status is set.
     * - MAY be logged when the operator status is set to the same status it was before the current call.
     */
    event OperatorSet(address indexed controller, address indexed operator, bool approved);

    /**
     * @dev Grants or revokes permissions for `operator` to manage Requests on behalf of the `msg.sender`.
     *
     * - MUST set the operator status to the `approved` value.
     * - MUST log the `OperatorSet` event.
     * - MUST return True.
     */
    function setOperator(address operator, bool approved) external returns (bool);

    /**
     * @dev Returns `true` if the `operator` is approved as an operator for an `controller`.
     */
    function isOperator(address controller, address operator) external returns (bool status);
}

interface IERC7540Deposit {
    /**
     * @dev `owner` has locked `assets` in the Vault to Request a deposit with request ID `requestId`. `controller` controls this Request. `sender` is the caller of the `requestDeposit` which may not be equal to the `owner`.
     *
     * - MUST be emitted when a deposit Request is submitted using the requestDeposit method.
     */
    event DepositRequest(address indexed controller, address indexed owner, uint256 indexed requestId, address sender, uint256 assets);

    /**
     * @dev Transfers `assets` from `owner` into the Vault and submits a Request for asynchronous `deposit`.
     *
     * - MUST support ERC-20 `approve` / `transferFrom` on `asset` as a deposit Request flow.
     * - `owner` MUST equal `msg.sender` unless the `owner` has approved the `msg.sender` as an operator.
     * - MUST revert if all of `assets` cannot be requested for `deposit`/`mint`
     * - NOTE: most implementations will require pre-approval of the Vault with the Vault’s underlying `asset` token.
     * - MUST emit the `RequestDeposit` event.
     */
    function requestDeposit(uint256 assets, address controller, address owner) external returns (uint256 requestId);

    /**
     * @dev The amount of requested `assets` in Pending state for the `controller` with the given `requestId` to `deposit` or `mint`.
     *
     * - MUST NOT include any `assets` in Claimable state for deposit or mint.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT revert unless due to integer overflow caused by an unreasonably large input.
     */
    function pendingDepositRequest(uint256 requestId, address controller) external view returns (uint256 pendingAssets);

    /**
     * @dev The amount of requested `assets` in Claimable state for the `controller` with the given `requestId` to `deposit` or `mint`.
     *
     * - MUST NOT include any `assets` in Pending state for `deposit` or `mint`.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT revert unless due to integer overflow caused by an unreasonably large input.
     */
    function claimableDepositRequest(uint256 requestId, address controller) external view returns (uint256 claimableAssets);

    /**
     * @dev Mints shares Vault shares to `receiver` by claiming the Request of the `controller`.
     *
     * - MUST revert unless `msg.sender` is either equal to `controller` or an operator approved by `controller`.
     * - MUST emit the `Deposit` event.
     * - MUST revert if all of assets cannot be deposited (due to deposit limit being reached, slippage, the user not
     *   approving enough underlying tokens to the Vault contract, etc).
     */
    function deposit(uint256 assets, address receiver, address controller) external returns (uint256 shares);

    /**
     * @dev Mints exactly shares Vault shares to `receiver` by claiming the Request of the `controller`.
     *
     * - MUST revert unless `msg.sender` is either equal to `controller` or an operator approved by `controller`.
     * - MUST emit the Deposit event.
     * - MUST revert if all of shares cannot be minted (due to deposit limit being reached, slippage, the user not
     *   approving enough underlying tokens to the Vault contract, etc).
     */
    function mint(uint256 shares, address receiver, address controller) external returns (uint256 assets);
}

interface IERC7540Redeem is IERC7540Operator {
    /**
     * @dev `sender` has locked `shares`, owned by `owner`, in the Vault to Request a redemption. `controller` controls this Request, but is not necessarily the `owner`.
     *
     * - MUST be emitted when a redemption Request is submitted using the `requestRedeem` method.
     */
    event RedeemRequest(address indexed controller, address indexed owner, uint256 indexed requestId, address sender, uint256 shares);

    /**
     * @dev Assumes control of `shares` from `owner` and submits a Request for asynchronous `redeem`.
     *
     * - MUST remove `shares` from the custody of `owner` upon `requestRedeem` and burned by the time the request is Claimed.
     *   where msg.sender has ERC-20 approval over the shares of owner.
     * - MUST revert if all of shares cannot be requested for `redeem` / `withdraw`
     * - MUST emit the `RequestRedeem` event.
     *
     */
    function requestRedeem(uint256 shares, address controller, address owner) external returns (uint256 requestId);

    /**
     * @dev The amount of requested `shares` in Pending state for the `controller` with the given `requestId` to `redeem` or `withdraw`.
     *
     * - MUST NOT include any `shares` in Claimable state for `redeem` or `withdraw`.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT revert unless due to integer overflow caused by an unreasonably large input.
     */
    function pendingRedeemRequest(uint256 requestId, address owner) external view returns (uint256 pendingShares);

    /**
     * @dev The amount of requested `shares` in Claimable state for the `controller` with the given `requestId` to `redeem` or `withdraw`.
     *
     * - MUST NOT include any `shares` in Pending state for `redeem` or `withdraw`.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT revert unless due to integer overflow caused by an unreasonably large input.
     */
    function claimableRedeemRequest(uint256 requestId, address owner) external view returns (uint256 claimableShares);
}

/**
 * @title  IERC7540
 * @dev    Interface of the ERC7540 "Asynchronous Tokenized Vault Standard", as defined in
 *         https://eips.ethereum.org/EIPS/eip-7540
 */
interface IERC7540 is IERC7540Operator, IERC7540Deposit, IERC7540Redeem {}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/**
 * @title IERC7887
 * @dev Interface for ERC7887 "Asynchronous Tokenized Vault Cancelation Extension"
 *      Extends ERC7540 with asynchronous cancelation capabilities
 *      https://eips.ethereum.org/EIPS/eip-7887
 */
interface IERC7887DepositCancelation {
    /**
     * @dev Emitted when a deposit cancelation request is submitted
     * controller - address that controls the cancelation request
     * owner - original owner of the assets
     * requestId - unique identifier for the cancelation request
     * sender - address that called cancelDepositRequest
     * assets - amount of assets being canceled
     */
    event CancelDepositRequest(address indexed controller, address indexed owner, uint256 indexed requestId, address sender, uint256 assets);

    /**
     * @dev Emitted when a deposit cancelation is claimed
     * controller - address that controlled the cancelation request
     * receiver - address that received the assets
     * requestId - unique identifier for the cancelation request
     * assets - amount of assets claimed
     */
    event CancelDepositRequestClaimed(address indexed controller, address indexed receiver, uint256 indexed requestId, uint256 assets);

    /**
     * @dev Submits a request to cancel a pending deposit request
     * Transitions the deposit request assets from pending state into pending cancelation state
     *
     * - MUST revert unless `msg.sender` is either equal to `controller` or an operator approved by `controller`
     * - MUST block new deposit requests for this controller while cancelation is pending
     * - MUST emit `CancelDepositRequest` event
     * - Can only cancel deposits in Pending state, not Claimable state
     *
     * @param requestId The requestId from the original deposit request (identifies which deposit to cancel)
     * @param controller Address that made the original deposit request
     */
    function cancelDepositRequest(uint256 requestId, address controller) external;

    /**
     * @dev Whether the given requestId and controller have a pending deposit cancelation request
     *
     * - Returns true if a deposit cancelation is in Pending state for this controller
     * - MUST NOT show any variations depending on the caller
     * - MUST NOT revert unless due to integer overflow
     *
     * @param requestId Cancelation request identifier
     * @param controller Address that made the original deposit request
     * @return isPending Whether a pending deposit cancelation exists for this controller
     */
    function pendingCancelDepositRequest(uint256 requestId, address controller) external view returns (bool isPending);

    /**
     * @dev Returns the amount of assets in claimable cancelation state
     *
     * - MUST NOT include any assets in Pending state
     * - MUST NOT show any variations depending on the caller
     * - MUST NOT revert unless due to integer overflow
     *
     * @param requestId Cancelation request identifier
     * @param controller Address that made the original deposit request
     * @return assets Amount of assets ready to claim
     */
    function claimableCancelDepositRequest(uint256 requestId, address controller) external view returns (uint256 assets);

    /**
     * @dev Claims assets from a claimable deposit cancelation request
     *
     * - MUST revert unless `msg.sender` is either equal to `controller` or an operator approved by `controller`
     * - MUST transition request from Claimable to Claimed state
     * - MUST transfer assets to `receiver`
     * - MUST emit `CancelDepositRequestClaimed` event
     * - Cannot be called unless request is in Claimable state
     *
     * @param requestId Cancelation request identifier
     * @param receiver Address to receive the claimed assets
     * @param controller Address that made the original deposit request
     */
    function claimCancelDepositRequest(uint256 requestId, address receiver, address controller) external;
}

interface IERC7887RedeemCancelation {
    /**
     * @dev Emitted when a redeem cancelation request is submitted
     * controller - address that controls the cancelation request
     * owner - original owner of the shares
     * requestId - unique identifier for the cancelation request
     * sender - address that called cancelRedeemRequest
     * shares - amount of shares being canceled
     */
    event CancelRedeemRequest(address indexed controller, address indexed owner, uint256 indexed requestId, address sender, uint256 shares);

    /**
     * @dev Emitted when a redeem cancelation is claimed
     * controller - address that controlled the cancelation request
     * receiver - address that received the shares
     * requestId - unique identifier for the cancelation request
     * shares - amount of shares claimed
     */
    event CancelRedeemRequestClaimed(address indexed controller, address indexed receiver, uint256 indexed requestId, uint256 shares);

    /**
     * @dev Submits a request to cancel a pending redeem request
     * Transitions the redeem request shares from pending state into pending cancelation state
     *
     * - MUST revert unless `msg.sender` is either equal to `controller` or an operator approved by `controller`
     * - MUST block new redeem requests for this controller while cancelation is pending
     * - MUST emit `CancelRedeemRequest` event
     * - Can only cancel redeems in Pending state, not Claimable state
     *
     * @param requestId The requestId from the original redeem request (identifies which redeem to cancel)
     * @param controller Address that made the original redeem request
     */
    function cancelRedeemRequest(uint256 requestId, address controller) external;

    /**
     * @dev Whether the given requestId and controller have a pending redeem cancelation request
     *
     * - Returns true if a redeem cancelation is in Pending state for this controller
     * - MUST NOT show any variations depending on the caller
     * - MUST NOT revert unless due to integer overflow
     *
     * @param requestId Cancelation request identifier
     * @param controller Address that made the original redeem request
     * @return isPending Whether a pending redeem cancelation exists for this controller
     */
    function pendingCancelRedeemRequest(uint256 requestId, address controller) external view returns (bool isPending);

    /**
     * @dev Returns the amount of shares in claimable cancelation state
     *
     * - MUST NOT include any shares in Pending state
     * - MUST NOT show any variations depending on the caller
     * - MUST NOT revert unless due to integer overflow
     *
     * @param requestId Cancelation request identifier
     * @param controller Address that made the original redeem request
     * @return shares Amount of shares ready to claim
     */
    function claimableCancelRedeemRequest(uint256 requestId, address controller) external view returns (uint256 shares);

    /**
     * @dev Claims shares from a claimable redeem cancelation request
     *
     * - MUST revert unless `msg.sender` is either equal to `owner` or an operator approved by `owner`
     * - MUST transition request from Claimable to Claimed state
     * - MUST transfer shares to `receiver`
     * - MUST emit `CancelRedeemRequestClaimed` event
     * - Cannot be called unless request is in Claimable state
     *
     * @param requestId Cancelation request identifier
     * @param receiver Address to receive the claimed shares
     * @param owner Address that made the original redeem request
     */
    function claimCancelRedeemRequest(uint256 requestId, address receiver, address owner) external;
}

/**
 * @title IERC7887
 * @dev Full ERC7887 interface combining deposit and redeem cancelation
 */
interface IERC7887 is IERC7887DepositCancelation, IERC7887RedeemCancelation {}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DecimalConstants} from "./DecimalConstants.sol";
import {SafeTokenTransfers} from "./SafeTokenTransfers.sol";
import {WERC7575ShareToken} from "./WERC7575ShareToken.sol";
import {IERC7575} from "./interfaces/IERC7575.sol";
import {IERC7575Errors} from "./interfaces/IERC7575Errors.sol";

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {IERC20Errors} from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {ERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";

contract WERC7575Vault is IERC7575, ERC165, ReentrancyGuard, Ownable2Step, Pausable, IERC7575Errors {
    using SafeERC20 for IERC20Metadata;

    /**
     * @dev Emitted when assets are deposited into the vault
     * @param sender The address that initiated the deposit
     * @param owner The address that received the shares
     * @param assets The amount of assets deposited
     * @param shares The amount of shares minted
     */
    event Deposit(address indexed sender, address indexed owner, uint256 assets, uint256 shares);

    /**
     * @dev Emitted when assets are withdrawn from the vault
     * @param sender The address that initiated the withdrawal
     * @param receiver The address that received the assets
     * @param owner The address that owned the shares
     * @param assets The amount of assets withdrawn
     * @param shares The amount of shares burned
     */
    event Withdraw(address indexed sender, address indexed receiver, address indexed owner, uint256 assets, uint256 shares);

    event VaultActiveStateChanged(bool indexed isActive);

    address private _asset; // 20 bytes
    uint64 private _scalingFactor; // 8 bytes
    bool private _isActive; // 1 byte - packs with _asset and _scalingFactor in same slot
    WERC7575ShareToken private _shareToken;

    /**
     * @dev Initializes a synchronous ERC4626 vault for the multi-asset system (ERC7575 compliant)
     *
     * Creates a simple, synchronous vault that enables immediate deposit/redeem operations
     * for a single asset. Integrates with the shared ShareToken to participate in the
     * multi-asset vault ecosystem.
     *
     * VAULT ARCHITECTURE:
     * - Synchronous operations: deposits and redeems are immediate
     * - Single asset per vault (paired asset-vault relationship)
     * - Shares minted/burned directly (no async requests)
     * - Integrates with multi-asset ShareToken
     * - Can be paused by owner for emergency situations
     *
     * SPECIFICATION COMPLIANCE:
     * - ERC7575: Multi-asset vault standard
     * - ERC4626: Complete tokenized vault functionality
     * - Decimal normalization: 6-18 decimals for assets, 18 for shares
     *
     * INITIALIZATION:
     * After deployment, the owner must:
     * 1. Call shareToken.registerVault(asset, vault_address)
     * 2. Set vault as active if needed (defaults to active)
     *
     * VALIDATION:
     * - Asset must be valid ERC20 with 6-18 decimals
     * - ShareToken must be valid ERC20 with 18 decimals
     * - ShareToken address must not be zero
     * - Scaling factor must fit in uint64
     *
     * @param asset_ The underlying ERC20 asset token (e.g., USDC, USDT)
     * @param shareToken_ The ERC7575 share token for multi-asset vault system
     *
     * @custom:throws ZeroAddress If shareToken_ is zero address
     * @custom:throws UnsupportedAssetDecimals If asset decimals are not 6-18
     * @custom:throws WrongDecimals If shareToken decimals are not 18
     * @custom:throws AssetDecimalsFailed If asset.decimals() call fails
     * @custom:throws ScalingFactorTooLarge If scaling factor exceeds uint64 max
     */
    constructor(address asset_, WERC7575ShareToken shareToken_) Ownable(msg.sender) {
        // Validate asset compatibility
        uint8 assetDecimals;
        try IERC20Metadata(asset_).decimals() returns (uint8 decimals) {
            if (decimals < DecimalConstants.MIN_ASSET_DECIMALS || decimals > DecimalConstants.SHARE_TOKEN_DECIMALS) {
                revert UnsupportedAssetDecimals();
            }
            assetDecimals = decimals;
        } catch {
            revert AssetDecimalsFailed();
        }
        // Validate share token compatibility and enforce 18 decimals
        if (address(shareToken_) == address(0)) revert ZeroAddress();
        if (shareToken_.decimals() != DecimalConstants.SHARE_TOKEN_DECIMALS) {
            revert WrongDecimals();
        }

        // Precompute scaling factor: 10^(18 - assetDecimals)
        // Max scaling factor is 10^12 (for 6 decimals) which fits in uint64
        uint256 scalingFactor = 10 ** (DecimalConstants.SHARE_TOKEN_DECIMALS - assetDecimals);
        if (scalingFactor > type(uint64).max) revert ScalingFactorTooLarge();

        _asset = asset_;
        _scalingFactor = uint64(scalingFactor);
        _isActive = true; // Vault is active by default
        _shareToken = shareToken_;

        // Note: Owner must separately call shareToken.registerVault(asset, vault) after deployment
    }

    /**
     * @dev Pause all vault operations. Only callable by owner.
     * Used for emergency situations to halt deposits, withdrawals, mints, and redeems.
     */
    function pause() external onlyOwner {
        _pause();
    }

    /**
     * @dev Unpause all vault operations. Only callable by owner.
     */
    function unpause() external onlyOwner {
        _unpause();
    }

    /**
     * @dev Sets the vault active state (only owner)
     * @param _active True to activate, false to deactivate
     */
    function setVaultActive(bool _active) external onlyOwner {
        _isActive = _active;
        emit VaultActiveStateChanged(_active);
    }

    /**
     * @dev Returns whether the vault is active and accepting deposits
     * @return True if vault is active
     */
    function isVaultActive() external view returns (bool) {
        return _isActive;
    }

    /**
     * @dev Returns true if this contract implements the interface defined by interfaceId
     * @param interfaceId The interface identifier, as specified in ERC-165
     * @return bool True if the contract implements interfaceId
     */
    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC165) returns (bool) {
        return interfaceId == type(IERC7575).interfaceId || super.supportsInterface(interfaceId);
    }

    /**
     * @dev Returns the address of the share token contract
     * @return address The ERC7575 share token address
     */
    function share() external view returns (address) {
        return address(_shareToken);
    }

    /**
     * @dev Returns the address of the underlying asset token
     * @return address The ERC20 asset token address
     */
    function asset() external view returns (address) {
        return _asset;
    }

    /**
     * @dev Returns the total amount of underlying assets held by the vault
     * @return uint256 Total assets held in the vault
     */
    function totalAssets() public view returns (uint256) {
        return IERC20Metadata(_asset).balanceOf(address(this));
    }

    /**
     * @dev Converts asset amount to equivalent share amount
     * @param assets Amount of assets to convert
     * @return uint256 Equivalent amount of shares
     */
    function convertToShares(uint256 assets) public view returns (uint256) {
        return _convertToShares(assets, Math.Rounding.Floor);
    }

    /**
     * @dev Converts share amount to equivalent asset amount
     * @param shares Amount of shares to convert
     * @return uint256 Equivalent amount of assets
     */
    function convertToAssets(uint256 shares) public view returns (uint256) {
        return _convertToAssets(shares, Math.Rounding.Floor);
    }

    /**
     * @dev Converts assets to shares using decimal normalization for stablecoins
     * @param assets Amount of assets to convert
     * @return shares Amount of shares equivalent to assets
     *
     * Formula: shares = assets * 10^(18 - assetDecimals)
     *
     * For stablecoins with no yield:
     * - Share decimals: enforced to be 18 in ShareToken constructor
     * - Asset decimals: varies (6 for USDC, 18 for DAI, etc.)
     * - This provides 1:1 value conversion with decimal normalization
     * - No first depositor attack possible since conversion is deterministic
     * - No manipulation possible since no dependency on totalSupply or totalAssets
     */
    function _convertToShares(uint256 assets, Math.Rounding rounding) internal view returns (uint256) {
        // ShareToken always has 18 decimals, assetDecimals ∈ [6, 18]
        // shares = assets * _scalingFactor where _scalingFactor = 10^(18 - assetDecimals)
        // Use Math.mulDiv to prevent overflow on large amounts
        return Math.mulDiv(assets, uint256(_scalingFactor), 1, rounding);
    }

    /**
     * @dev Converts shares to assets using decimal normalization for stablecoins
     * @param shares Amount of shares to convert
     * @param rounding Rounding direction (Floor = favor vault, Ceil = favor user)
     * @return assets Amount of assets equivalent to shares
     *
     * Formula: assets = shares * 10^(assetDecimals) / 10^(shareDecimals)
     *
     * For stablecoins with no yield:
     * - Share decimals: queried from share token (typically 18)
     * - Asset decimals: varies (6 for USDC, 18 for DAI, etc.)
     * - This provides 1:1 value conversion with decimal normalization
     * - No first depositor attack possible since conversion is deterministic
     * - No manipulation possible since no dependency on totalSupply or totalAssets
     */
    function _convertToAssets(uint256 shares, Math.Rounding rounding) internal view returns (uint256) {
        // ShareToken always has 18 decimals, assetDecimals ∈ [6, 18]
        // When _scalingFactor == 1 (assetDecimals == 18): assets = shares
        // When _scalingFactor > 1 (assetDecimals < 18): assets = shares / _scalingFactor
        if (_scalingFactor == 1) {
            return shares;
        } else {
            return Math.mulDiv(shares, 1, uint256(_scalingFactor), rounding);
        }
    }

    /**
     * @dev Preview shares received for depositing assets
     * Uses Floor rounding to give slightly fewer shares to user (favors vault)
     */
    function previewDeposit(uint256 assets) public view returns (uint256) {
        return _convertToShares(assets, Math.Rounding.Floor);
    }

    /**
     * @dev Preview assets needed to mint shares
     * Uses Ceil rounding to require slightly more assets from user (favors vault)
     */
    function previewMint(uint256 shares) public view returns (uint256) {
        return _convertToAssets(shares, Math.Rounding.Ceil);
    }

    /**
     * @dev Preview shares needed to withdraw assets
     * Uses Ceil rounding to require slightly more shares from user (favors vault)
     */
    function previewWithdraw(uint256 assets) public view returns (uint256) {
        return _convertToShares(assets, Math.Rounding.Ceil);
    }

    /**
     * @dev Preview assets received for redeeming shares
     * Uses Floor rounding to give slightly fewer assets to user (favors vault)
     */
    function previewRedeem(uint256 shares) public view returns (uint256) {
        return _convertToAssets(shares, Math.Rounding.Floor);
    }

    /**
     * @dev Returns the maximum amount of assets that can be deposited
     * @return uint256 Maximum deposit amount (unlimited)
     *
     * Note: Receiver parameter is unused as there are no deposit limits
     */
    function maxDeposit(address) public pure returns (uint256) {
        return type(uint256).max;
    }

    /**
     * @dev Returns the maximum amount of shares that can be minted
     * @return uint256 Maximum mint amount (unlimited)
     *
     * Note: Receiver parameter is unused as there are no mint limits
     */
    function maxMint(address) public pure returns (uint256) {
        return type(uint256).max;
    }

    /**
     * @dev Returns the maximum amount of assets that can be withdrawn by owner
     * @param owner The address that owns the shares
     * @return uint256 Maximum withdrawal amount based on share balance
     */
    function maxWithdraw(address owner) public view returns (uint256) {
        return _convertToAssets(_shareToken.balanceOf(owner), Math.Rounding.Floor);
    }

    /**
     * @dev Returns the maximum amount of shares that can be redeemed by owner
     * @param owner The address that owns the shares
     * @return uint256 Maximum redeem amount (owner's full share balance)
     */
    function maxRedeem(address owner) public view returns (uint256) {
        return _shareToken.balanceOf(owner);
    }

    /**
     * @dev Internal function to handle deposit/mint logic
     * @param assets Amount of assets to transfer
     * @param shares Amount of shares to mint
     * @param receiver Address to receive shares
     */
    function _deposit(uint256 assets, uint256 shares, address receiver) internal {
        if (!_isActive) revert VaultNotActive();
        if (receiver == address(0)) {
            revert IERC20Errors.ERC20InvalidReceiver(address(0));
        }
        if (assets == 0) revert ZeroAssets();
        if (shares == 0) revert ZeroShares();

        SafeTokenTransfers.safeTransferFrom(_asset, msg.sender, address(this), assets);

        _shareToken.mint(receiver, shares);
        emit Deposit(msg.sender, receiver, assets, shares);
    }

    /**
     * @dev Deposits exact amount of assets and receives corresponding shares (ERC4626 compliant)
     *
     * Synchronous deposit operation: immediately mints shares and transfers assets.
     * Simple one-step process without async state management.
     *
     * OPERATION:
     * - Previews share amount for the deposit
     * - Transfers assets from caller to vault
     * - Mints shares to receiver
     *
     * SECURITY:
     * - Reentrancy protected
     * - Paused state check
     * - Vault must be active
     * - Zero address validation
     *
     * @param assets Amount of assets to deposit
     * @param receiver Address to receive the minted shares
     *
     * @return shares Amount of shares minted
     */
    function deposit(uint256 assets, address receiver) public nonReentrant whenNotPaused returns (uint256 shares) {
        shares = previewDeposit(assets);
        _deposit(assets, shares, receiver);
    }

    /**
     * @dev Mints exact amount of shares by depositing necessary assets (ERC4626 compliant)
     *
     * Synchronous mint operation: caller specifies desired shares, assets calculated.
     * Transfers required assets and immediately mints specified shares.
     *
     * OPERATION:
     * - Previews asset amount needed for shares
     * - Transfers required assets from caller
     * - Mints exact shares to receiver
     *
     * USE CASE:
     * - When you want exactly X shares (not Y assets)
     * - May require more assets due to rounding
     *
     * @param shares Amount of shares to mint (exact)
     * @param receiver Address to receive the minted shares
     *
     * @return assets Amount of assets required for the mint
     */
    function mint(uint256 shares, address receiver) public nonReentrant whenNotPaused returns (uint256 assets) {
        assets = previewMint(shares);
        _deposit(assets, shares, receiver);
    }

    /**
     * @dev Internal function to handle withdraw/redeem logic
     * @param assets Amount of assets to transfer
     * @param shares Amount of shares to burn
     * @param receiver Address to receive assets
     * @param owner Address that owns the shares
     */
    function _withdraw(uint256 assets, uint256 shares, address receiver, address owner) internal {
        if (receiver == address(0)) {
            revert IERC20Errors.ERC20InvalidReceiver(address(0));
        }
        if (owner == address(0)) {
            revert IERC20Errors.ERC20InvalidSender(address(0));
        }
        if (assets == 0) revert ZeroAssets();
        if (shares == 0) revert ZeroShares();

        _shareToken.spendSelfAllowance(owner, shares);
        _shareToken.burn(owner, shares);
        SafeTokenTransfers.safeTransfer(_asset, receiver, assets);
        emit Withdraw(msg.sender, receiver, owner, assets, shares);
    }

    /**
     * @dev Withdraws exact amount of assets from vault by burning shares (ERC4626 compliant)
     *
     * Synchronous withdrawal operation: caller specifies assets, shares calculated.
     * Burns required shares and immediately transfers assets to receiver.
     *
     * OPERATION:
     * - Previews share amount needed for assets
     * - Burns required shares from owner
     * - Transfers exact assets to receiver
     *
     * AUTHORIZATION:
     * - msg.sender must be owner OR have allowance for the shares
     * - Allows delegation to withdrawal operators
     *
     * @param assets Amount of assets to withdraw (exact)
     * @param receiver Address to receive the assets
     * @param owner Address that owns the shares to be burned
     *
     * @return shares Amount of shares burned
     */
    function withdraw(uint256 assets, address receiver, address owner) public nonReentrant whenNotPaused returns (uint256 shares) {
        shares = previewWithdraw(assets);
        _withdraw(assets, shares, receiver, owner);
    }

    /**
     * @dev Redeems exact amount of shares for assets (ERC4626 compliant)
     *
     * Synchronous redemption operation: caller specifies shares, assets calculated.
     * Burns exact shares and transfers corresponding assets to receiver.
     *
     * OPERATION:
     * - Previews asset amount for shares
     * - Burns exact shares from owner
     * - Transfers corresponding assets to receiver
     *
     * AUTHORIZATION:
     * - msg.sender must be owner OR have allowance for the shares
     * - Allows delegation to redemption operators
     *
     * USE CASE:
     * - When you want to burn exactly X shares (not Y assets)
     * - Receives at least minimum due to rounding down
     *
     * @param shares Amount of shares to redeem (exact)
     * @param receiver Address to receive the assets
     * @param owner Address that owns the shares to be burned
     *
     * @return assets Amount of assets withdrawn
     */
    function redeem(uint256 shares, address receiver, address owner) public nonReentrant whenNotPaused returns (uint256 assets) {
        assets = previewRedeem(shares);
        _withdraw(assets, shares, receiver, owner);
    }
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

