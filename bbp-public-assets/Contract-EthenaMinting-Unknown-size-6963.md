
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

/* solhint-disable private-vars-leading-underscore */
/* solhint-disable var-name-mixedcase */

import "./SingleAdminAccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

import "./interfaces/IUSDe.sol";
import "./interfaces/IEthenaMinting.sol";
import "./interfaces/IWETH9.sol";

/**
 * @title Ethena Minting
 * @notice This contract mints and redeems USDe, the first staked Ethereum delta-neutral backed synthetic dollar
 */
contract EthenaMinting is IEthenaMinting, SingleAdminAccessControl, ReentrancyGuard {
  using SafeERC20 for IERC20;
  using EnumerableSet for EnumerableSet.AddressSet;

  /* --------------- CONSTANTS --------------- */

  /// @notice EIP712 domain
  bytes32 private constant EIP712_DOMAIN =
    keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

  /// @notice route type
  bytes32 private constant ROUTE_TYPE = keccak256("Route(address[] addresses,uint256[] ratios)");

  /// @notice order type
  bytes32 private constant ORDER_TYPE = keccak256(
    "Order(uint8 order_type,uint256 expiry,uint256 nonce,address benefactor,address beneficiary,address collateral_asset,uint256 collateral_amount,uint256 usde_amount)"
  );

  /// @notice role enabling to invoke mint
  bytes32 private constant MINTER_ROLE = keccak256("MINTER_ROLE");

  /// @notice role enabling to invoke redeem
  bytes32 private constant REDEEMER_ROLE = keccak256("REDEEMER_ROLE");

  /// @notice role enabling to transfer collateral to custody wallets
  bytes32 private constant COLLATERAL_MANAGER_ROLE = keccak256("COLLATERAL_MANAGER_ROLE");

  /// @notice role enabling to disable mint and redeem and remove minters and redeemers in an emergency
  bytes32 private constant GATEKEEPER_ROLE = keccak256("GATEKEEPER_ROLE");

  /// @notice EIP712 domain hash
  bytes32 private constant EIP712_DOMAIN_TYPEHASH = keccak256(abi.encodePacked(EIP712_DOMAIN));

  /// @notice address denoting native ether
  address private constant NATIVE_TOKEN = 0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE;

  /// @notice EIP712 name
  bytes32 private constant EIP_712_NAME = keccak256("EthenaMinting");

  /// @notice holds EIP712 revision
  bytes32 private constant EIP712_REVISION = keccak256("1");

  /// @notice required ratio for route
  uint256 private constant ROUTE_REQUIRED_RATIO = 10_000;

  IWETH9 private immutable WETH;

  /* --------------- STATE VARIABLES --------------- */

  /// @notice usde stablecoin
  IUSDe public immutable usde;

  /// @notice Supported assets
  EnumerableSet.AddressSet internal _supportedAssets;

  // @notice custodian addresses
  EnumerableSet.AddressSet internal _custodianAddresses;

  /// @notice holds computable chain id
  uint256 private immutable _chainId;

  /// @notice holds computable domain separator
  bytes32 private immutable _domainSeparator;

  /// @notice user deduplication
  mapping(address => mapping(uint256 => uint256)) private _orderBitmaps;

  /// @notice USDe minted per block
  mapping(uint256 => uint256) public mintedPerBlock;
  /// @notice USDe redeemed per block
  mapping(uint256 => uint256) public redeemedPerBlock;

  /// @notice For smart contracts to delegate signing to EOA address
  mapping(address => mapping(address => DelegatedSignerStatus)) public delegatedSigner;

  /// @notice max minted USDe allowed per block
  uint256 public maxMintPerBlock;
  /// @notice max redeemed USDe allowed per block
  uint256 public maxRedeemPerBlock;

  /* --------------- MODIFIERS --------------- */

  /// @notice ensure that the already minted USDe in the actual block plus the amount to be minted is below the maxMintPerBlock var
  /// @param mintAmount The USDe amount to be minted
  modifier belowMaxMintPerBlock(uint256 mintAmount) {
    if (mintedPerBlock[block.number] + mintAmount > maxMintPerBlock) revert MaxMintPerBlockExceeded();
    _;
  }

  /// @notice ensure that the already redeemed USDe in the actual block plus the amount to be redeemed is below the maxRedeemPerBlock var
  /// @param redeemAmount The USDe amount to be redeemed
  modifier belowMaxRedeemPerBlock(uint256 redeemAmount) {
    if (redeemedPerBlock[block.number] + redeemAmount > maxRedeemPerBlock) revert MaxRedeemPerBlockExceeded();
    _;
  }

  /* --------------- CONSTRUCTOR --------------- */

  constructor(
    IUSDe _usde,
    IWETH9 _weth,
    address[] memory _assets,
    address[] memory _custodians,
    address _admin,
    uint256 _maxMintPerBlock,
    uint256 _maxRedeemPerBlock
  ) {
    if (address(_usde) == address(0)) revert InvalidUSDeAddress();
    if (address(_weth) == address(0)) revert InvalidZeroAddress();
    if (_assets.length == 0) revert NoAssetsProvided();
    if (_admin == address(0)) revert InvalidZeroAddress();
    usde = _usde;
    WETH = _weth;

    _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);

    for (uint256 i = 0; i < _assets.length;) {
      addSupportedAsset(_assets[i]);
      unchecked {
        ++i;
      }
    }

    for (uint256 j = 0; j < _custodians.length;) {
      addCustodianAddress(_custodians[j]);
      unchecked {
        ++j;
      }
    }

    // Set the max mint/redeem limits per block
    _setMaxMintPerBlock(_maxMintPerBlock);
    _setMaxRedeemPerBlock(_maxRedeemPerBlock);

    if (msg.sender != _admin) {
      _grantRole(DEFAULT_ADMIN_ROLE, _admin);
    }

    _chainId = block.chainid;
    _domainSeparator = _computeDomainSeparator();

    emit USDeSet(address(_usde));
  }

  /* --------------- EXTERNAL --------------- */

  /**
   * @notice Fallback function to receive ether
   */
  receive() external payable {
    emit Received(msg.sender, msg.value);
  }

  /**
   * @notice Mint stablecoins from assets
   * @param order struct containing order details and confirmation from server
   * @param signature signature of the taker
   */
  function mint(Order calldata order, Route calldata route, Signature calldata signature)
    external
    override
    nonReentrant
    onlyRole(MINTER_ROLE)
    belowMaxMintPerBlock(order.usde_amount)
  {
    if (order.order_type != OrderType.MINT) revert InvalidOrder();
    verifyOrder(order, signature);
    if (!verifyRoute(route)) revert InvalidRoute();
    _deduplicateOrder(order.benefactor, order.nonce);
    // Add to the minted amount in this block
    mintedPerBlock[block.number] += order.usde_amount;
    _transferCollateral(
      order.collateral_amount, order.collateral_asset, order.benefactor, route.addresses, route.ratios
    );
    usde.mint(order.beneficiary, order.usde_amount);
    emit Mint(
      msg.sender,
      order.benefactor,
      order.beneficiary,
      order.collateral_asset,
      order.collateral_amount,
      order.usde_amount
    );
  }

  /**
   * @notice Mint stablecoins from assets
   * @param order struct containing order details and confirmation from server
   * @param signature signature of the taker
   */
  function mintWETH(Order calldata order, Route calldata route, Signature calldata signature)
    external
    nonReentrant
    onlyRole(MINTER_ROLE)
    belowMaxMintPerBlock(order.usde_amount)
  {
    if (order.order_type != OrderType.MINT) revert InvalidOrder();
    verifyOrder(order, signature);
    if (!verifyRoute(route)) revert InvalidRoute();
    _deduplicateOrder(order.benefactor, order.nonce);
    // Add to the minted amount in this block
    mintedPerBlock[block.number] += order.usde_amount;
    // Checks that the collateral asset is WETH also
    _transferEthCollateral(
      order.collateral_amount, order.collateral_asset, order.benefactor, route.addresses, route.ratios
    );
    usde.mint(order.beneficiary, order.usde_amount);
    emit Mint(
      msg.sender,
      order.benefactor,
      order.beneficiary,
      order.collateral_asset,
      order.collateral_amount,
      order.usde_amount
    );
  }

  /**
   * @notice Redeem stablecoins for assets
   * @param order struct containing order details and confirmation from server
   * @param signature signature of the taker
   */
  function redeem(Order calldata order, Signature calldata signature)
    external
    override
    nonReentrant
    onlyRole(REDEEMER_ROLE)
    belowMaxRedeemPerBlock(order.usde_amount)
  {
    if (order.order_type != OrderType.REDEEM) revert InvalidOrder();
    verifyOrder(order, signature);
    _deduplicateOrder(order.benefactor, order.nonce);
    // Add to the redeemed amount in this block
    redeemedPerBlock[block.number] += order.usde_amount;
    usde.burnFrom(order.benefactor, order.usde_amount);
    _transferToBeneficiary(order.beneficiary, order.collateral_asset, order.collateral_amount);
    emit Redeem(
      msg.sender,
      order.benefactor,
      order.beneficiary,
      order.collateral_asset,
      order.collateral_amount,
      order.usde_amount
    );
  }

  /// @notice Sets the max mintPerBlock limit
  function setMaxMintPerBlock(uint256 _maxMintPerBlock) external onlyRole(DEFAULT_ADMIN_ROLE) {
    _setMaxMintPerBlock(_maxMintPerBlock);
  }

  /// @notice Sets the max redeemPerBlock limit
  function setMaxRedeemPerBlock(uint256 _maxRedeemPerBlock) external onlyRole(DEFAULT_ADMIN_ROLE) {
    _setMaxRedeemPerBlock(_maxRedeemPerBlock);
  }

  /// @notice Disables the mint and redeem
  function disableMintRedeem() external onlyRole(GATEKEEPER_ROLE) {
    _setMaxMintPerBlock(0);
    _setMaxRedeemPerBlock(0);
  }

  /// @notice Enables smart contracts to delegate an address for signing
  function setDelegatedSigner(address _delegateTo) external {
    delegatedSigner[_delegateTo][msg.sender] = DelegatedSignerStatus.PENDING;
    emit DelegatedSignerInitiated(_delegateTo, msg.sender);
  }

  /// @notice The delegated address to confirm delegation
  function confirmDelegatedSigner(address _delegatedBy) external {
    if (delegatedSigner[msg.sender][_delegatedBy] != DelegatedSignerStatus.PENDING) {
      revert DelegationNotInitiated();
    }
    delegatedSigner[msg.sender][_delegatedBy] = DelegatedSignerStatus.ACCEPTED;
    emit DelegatedSignerAdded(msg.sender, _delegatedBy);
  }

  /// @notice Enables smart contracts to undelegate an address for signing
  function removeDelegatedSigner(address _removedSigner) external {
    delegatedSigner[_removedSigner][msg.sender] = DelegatedSignerStatus.REJECTED;
    emit DelegatedSignerRemoved(_removedSigner, msg.sender);
  }

  /// @notice transfers an asset to a custody wallet
  function transferToCustody(address wallet, address asset, uint256 amount)
    external
    nonReentrant
    onlyRole(COLLATERAL_MANAGER_ROLE)
  {
    if (wallet == address(0) || !_custodianAddresses.contains(wallet)) revert InvalidAddress();
    if (asset == NATIVE_TOKEN) {
      (bool success,) = wallet.call{value: amount}("");
      if (!success) revert TransferFailed();
    } else {
      IERC20(asset).safeTransfer(wallet, amount);
    }
    emit CustodyTransfer(wallet, asset, amount);
  }

  /// @notice Removes an asset from the supported assets list
  function removeSupportedAsset(address asset) external onlyRole(DEFAULT_ADMIN_ROLE) {
    if (!_supportedAssets.remove(asset)) revert InvalidAssetAddress();
    emit AssetRemoved(asset);
  }

  /// @notice Checks if an asset is supported.
  function isSupportedAsset(address asset) external view returns (bool) {
    return _supportedAssets.contains(asset);
  }

  /// @notice Removes an custodian from the custodian address list
  function removeCustodianAddress(address custodian) external onlyRole(DEFAULT_ADMIN_ROLE) {
    if (!_custodianAddresses.remove(custodian)) revert InvalidCustodianAddress();
    emit CustodianAddressRemoved(custodian);
  }

  /// @notice Removes the minter role from an account, this can ONLY be executed by the gatekeeper role
  /// @param minter The address to remove the minter role from
  function removeMinterRole(address minter) external onlyRole(GATEKEEPER_ROLE) {
    _revokeRole(MINTER_ROLE, minter);
  }

  /// @notice Removes the redeemer role from an account, this can ONLY be executed by the gatekeeper role
  /// @param redeemer The address to remove the redeemer role from
  function removeRedeemerRole(address redeemer) external onlyRole(GATEKEEPER_ROLE) {
    _revokeRole(REDEEMER_ROLE, redeemer);
  }

  /// @notice Removes the collateral manager role from an account, this can ONLY be executed by the gatekeeper role
  /// @param collateralManager The address to remove the collateralManager role from
  function removeCollateralManagerRole(address collateralManager) external onlyRole(GATEKEEPER_ROLE) {
    _revokeRole(COLLATERAL_MANAGER_ROLE, collateralManager);
  }

  /* --------------- PUBLIC --------------- */

  /// @notice Adds an asset to the supported assets list.
  function addSupportedAsset(address asset) public onlyRole(DEFAULT_ADMIN_ROLE) {
    if (asset == address(0) || asset == address(usde) || !_supportedAssets.add(asset)) {
      revert InvalidAssetAddress();
    }
    emit AssetAdded(asset);
  }

  /// @notice Adds an custodian to the supported custodians list.
  function addCustodianAddress(address custodian) public onlyRole(DEFAULT_ADMIN_ROLE) {
    if (custodian == address(0) || custodian == address(usde) || !_custodianAddresses.add(custodian)) {
      revert InvalidCustodianAddress();
    }
    emit CustodianAddressAdded(custodian);
  }

  /// @notice Get the domain separator for the token
  /// @dev Return cached value if chainId matches cache, otherwise recomputes separator, to prevent replay attack across forks
  /// @return The domain separator of the token at current chain
  function getDomainSeparator() public view returns (bytes32) {
    if (block.chainid == _chainId) {
      return _domainSeparator;
    }
    return _computeDomainSeparator();
  }

  /// @notice hash an Order struct
  function hashOrder(Order calldata order) public view override returns (bytes32) {
    return ECDSA.toTypedDataHash(getDomainSeparator(), keccak256(encodeOrder(order)));
  }

  function encodeOrder(Order calldata order) public pure returns (bytes memory) {
    return abi.encode(
      ORDER_TYPE,
      order.order_type,
      order.expiry,
      order.nonce,
      order.benefactor,
      order.beneficiary,
      order.collateral_asset,
      order.collateral_amount,
      order.usde_amount
    );
  }

  /// @notice assert validity of signed order
  function verifyOrder(Order calldata order, Signature calldata signature)
    public
    view
    override
    returns (bytes32 taker_order_hash)
  {
    taker_order_hash = hashOrder(order);
    address signer = ECDSA.recover(taker_order_hash, signature.signature_bytes);
    if (!(signer == order.benefactor || delegatedSigner[signer][order.benefactor] == DelegatedSignerStatus.ACCEPTED)) {
      revert InvalidSignature();
    }
    if (order.beneficiary == address(0)) revert InvalidAddress();
    if (order.collateral_amount == 0) revert InvalidAmount();
    if (order.usde_amount == 0) revert InvalidAmount();
    if (block.timestamp > order.expiry) revert SignatureExpired();
  }

  /// @notice assert validity of route object per type
  function verifyRoute(Route calldata route) public view override returns (bool) {
    uint256 totalRatio = 0;
    if (route.addresses.length != route.ratios.length) {
      return false;
    }
    if (route.addresses.length == 0) {
      return false;
    }
    for (uint256 i = 0; i < route.addresses.length;) {
      if (!_custodianAddresses.contains(route.addresses[i]) || route.addresses[i] == address(0) || route.ratios[i] == 0)
      {
        return false;
      }
      totalRatio += route.ratios[i];
      unchecked {
        ++i;
      }
    }
    return (totalRatio == ROUTE_REQUIRED_RATIO);
  }

  /// @notice verify validity of nonce by checking its presence
  function verifyNonce(address sender, uint256 nonce) public view override returns (uint256, uint256, uint256) {
    if (nonce == 0) revert InvalidNonce();
    uint256 invalidatorSlot = uint64(nonce) >> 8;
    uint256 invalidatorBit = 1 << uint8(nonce);
    uint256 invalidator = _orderBitmaps[sender][invalidatorSlot];
    if (invalidator & invalidatorBit != 0) revert InvalidNonce();

    return (invalidatorSlot, invalidator, invalidatorBit);
  }

  /* --------------- PRIVATE --------------- */

  /// @notice deduplication of taker order
  function _deduplicateOrder(address sender, uint256 nonce) private {
    (uint256 invalidatorSlot, uint256 invalidator, uint256 invalidatorBit) = verifyNonce(sender, nonce);
    _orderBitmaps[sender][invalidatorSlot] = invalidator | invalidatorBit;
  }

  /* --------------- INTERNAL --------------- */

  /// @notice transfer supported asset to beneficiary address
  function _transferToBeneficiary(address beneficiary, address asset, uint256 amount) internal {
    if (asset == NATIVE_TOKEN) {
      if (address(this).balance < amount) revert InvalidAmount();
      (bool success,) = (beneficiary).call{value: amount}("");
      if (!success) revert TransferFailed();
    } else {
      if (!_supportedAssets.contains(asset)) revert UnsupportedAsset();
      IERC20(asset).safeTransfer(beneficiary, amount);
    }
  }

  /// @notice transfer supported asset to array of custody addresses per defined ratio
  function _transferCollateral(
    uint256 amount,
    address asset,
    address benefactor,
    address[] calldata addresses,
    uint256[] calldata ratios
  ) internal {
    // cannot mint using unsupported asset or native ETH even if it is supported for redemptions
    if (!_supportedAssets.contains(asset) || asset == NATIVE_TOKEN) revert UnsupportedAsset();
    IERC20 token = IERC20(asset);
    uint256 totalTransferred = 0;
    for (uint256 i = 0; i < addresses.length;) {
      uint256 amountToTransfer = (amount * ratios[i]) / ROUTE_REQUIRED_RATIO;
      token.safeTransferFrom(benefactor, addresses[i], amountToTransfer);
      totalTransferred += amountToTransfer;
      unchecked {
        ++i;
      }
    }
    uint256 remainingBalance = amount - totalTransferred;
    if (remainingBalance > 0) {
      token.safeTransferFrom(benefactor, addresses[addresses.length - 1], remainingBalance);
    }
  }

  /// @notice transfer supported asset to array of custody addresses per defined ratio
  function _transferEthCollateral(
    uint256 amount,
    address asset,
    address benefactor,
    address[] calldata addresses,
    uint256[] calldata ratios
  ) internal {
    if (!_supportedAssets.contains(asset) || asset == NATIVE_TOKEN || asset != address(WETH)) revert UnsupportedAsset();
    IERC20 token = IERC20(asset);
    token.safeTransferFrom(benefactor, address(this), amount);

    WETH.withdraw(amount);

    uint256 totalTransferred = 0;
    for (uint256 i = 0; i < addresses.length;) {
      uint256 amountToTransfer = (amount * ratios[i]) / ROUTE_REQUIRED_RATIO;
      (bool success,) = addresses[i].call{value: amountToTransfer}("");
      if (!success) revert TransferFailed();
      totalTransferred += amountToTransfer;
      unchecked {
        ++i;
      }
    }
    uint256 remainingBalance = amount - totalTransferred;
    if (remainingBalance > 0) {
      (bool success,) = addresses[addresses.length - 1].call{value: remainingBalance}("");
      if (!success) revert TransferFailed();
    }
  }

  /// @notice Sets the max mintPerBlock limit
  function _setMaxMintPerBlock(uint256 _maxMintPerBlock) internal {
    uint256 oldMaxMintPerBlock = maxMintPerBlock;
    maxMintPerBlock = _maxMintPerBlock;
    emit MaxMintPerBlockChanged(oldMaxMintPerBlock, maxMintPerBlock);
  }

  /// @notice Sets the max redeemPerBlock limit
  function _setMaxRedeemPerBlock(uint256 _maxRedeemPerBlock) internal {
    uint256 oldMaxRedeemPerBlock = maxRedeemPerBlock;
    maxRedeemPerBlock = _maxRedeemPerBlock;
    emit MaxRedeemPerBlockChanged(oldMaxRedeemPerBlock, maxRedeemPerBlock);
  }

  /// @notice Compute the current domain separator
  /// @return The domain separator for the token
  function _computeDomainSeparator() internal view returns (bytes32) {
    return keccak256(abi.encode(EIP712_DOMAIN, EIP_712_NAME, EIP712_REVISION, block.chainid, address(this)));
  }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/interfaces/IERC5313.sol";
import "./interfaces/ISingleAdminAccessControl.sol";

/**
 * @title SingleAdminAccessControl
 * @notice SingleAdminAccessControl is a contract that provides a single admin role
 * @notice This contract is a simplified alternative to OpenZeppelin's AccessControlDefaultAdminRules
 */
abstract contract SingleAdminAccessControl is IERC5313, ISingleAdminAccessControl, AccessControl {
  address private _currentDefaultAdmin;
  address private _pendingDefaultAdmin;

  modifier notAdmin(bytes32 role) {
    if (role == DEFAULT_ADMIN_ROLE) revert InvalidAdminChange();
    _;
  }

  /// @notice Transfer the admin role to a new address
  /// @notice This can ONLY be executed by the current admin
  /// @param newAdmin address
  function transferAdmin(address newAdmin) external onlyRole(DEFAULT_ADMIN_ROLE) {
    if (newAdmin == msg.sender) revert InvalidAdminChange();
    _pendingDefaultAdmin = newAdmin;
    emit AdminTransferRequested(_currentDefaultAdmin, newAdmin);
  }

  function acceptAdmin() external {
    if (msg.sender != _pendingDefaultAdmin) revert NotPendingAdmin();
    _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
  }

  /// @notice grant a role
  /// @notice can only be executed by the current single admin
  /// @notice admin role cannot be granted externally
  /// @param role bytes32
  /// @param account address
  function grantRole(bytes32 role, address account) public override onlyRole(DEFAULT_ADMIN_ROLE) notAdmin(role) {
    _grantRole(role, account);
  }

  /// @notice revoke a role
  /// @notice can only be executed by the current admin
  /// @notice admin role cannot be revoked
  /// @param role bytes32
  /// @param account address
  function revokeRole(bytes32 role, address account) public override onlyRole(DEFAULT_ADMIN_ROLE) notAdmin(role) {
    _revokeRole(role, account);
  }

  /// @notice renounce the role of msg.sender
  /// @notice admin role cannot be renounced
  /// @param role bytes32
  /// @param account address
  function renounceRole(bytes32 role, address account) public virtual override notAdmin(role) {
    super.renounceRole(role, account);
  }

  /**
   * @dev See {IERC5313-owner}.
   */
  function owner() public view virtual returns (address) {
    return _currentDefaultAdmin;
  }

  /**
   * @notice no way to change admin without removing old admin first
   */
  function _grantRole(bytes32 role, address account) internal override {
    if (role == DEFAULT_ADMIN_ROLE) {
      emit AdminTransferred(_currentDefaultAdmin, account);
      _revokeRole(DEFAULT_ADMIN_ROLE, _currentDefaultAdmin);
      _currentDefaultAdmin = account;
      delete _pendingDefaultAdmin;
    }
    super._grantRole(role, account);
  }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

/* solhint-disable var-name-mixedcase  */

import "./IEthenaMintingEvents.sol";

interface IEthenaMinting is IEthenaMintingEvents {
  enum Role {
    Minter,
    Redeemer
  }

  enum OrderType {
    MINT,
    REDEEM
  }

  enum SignatureType {EIP712}

  enum DelegatedSignerStatus {
    REJECTED,
    PENDING,
    ACCEPTED
  }

  struct Signature {
    SignatureType signature_type;
    bytes signature_bytes;
  }

  struct Route {
    address[] addresses;
    uint256[] ratios;
  }

  struct Order {
    OrderType order_type;
    uint256 expiry;
    uint256 nonce;
    address benefactor;
    address beneficiary;
    address collateral_asset;
    uint256 collateral_amount;
    uint256 usde_amount;
  }

  error InvalidAddress();
  error InvalidUSDeAddress();
  error InvalidZeroAddress();
  error InvalidAssetAddress();
  error InvalidCustodianAddress();
  error InvalidOrder();
  error InvalidAmount();
  error InvalidRoute();
  error UnsupportedAsset();
  error NoAssetsProvided();
  error InvalidSignature();
  error InvalidNonce();
  error SignatureExpired();
  error TransferFailed();
  error MaxMintPerBlockExceeded();
  error MaxRedeemPerBlockExceeded();
  error DelegationNotInitiated();

  function hashOrder(Order calldata order) external view returns (bytes32);

  function verifyOrder(Order calldata order, Signature calldata signature) external view returns (bytes32);

  function verifyRoute(Route calldata route) external view returns (bool);

  function verifyNonce(address sender, uint256 nonce) external view returns (uint256, uint256, uint256);

  function mint(Order calldata order, Route calldata route, Signature calldata signature) external;

  function mintWETH(Order calldata order, Route calldata route, Signature calldata signature) external;

  function redeem(Order calldata order, Signature calldata signature) external;
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

interface ISingleAdminAccessControl {
  error InvalidAdminChange();
  error NotPendingAdmin();

  event AdminTransferred(address indexed oldAdmin, address indexed newAdmin);
  event AdminTransferRequested(address indexed oldAdmin, address indexed newAdmin);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

/* solhint-disable var-name-mixedcase  */

interface IEthenaMintingEvents {
  /// @notice Event emitted when contract receives ETH
  event Received(address, uint256);

  /// @notice Event emitted when USDe is minted
  event Mint(
    address indexed minter,
    address indexed benefactor,
    address indexed beneficiary,
    address collateral_asset,
    uint256 collateral_amount,
    uint256 usde_amount
  );

  /// @notice Event emitted when funds are redeemed
  event Redeem(
    address indexed redeemer,
    address indexed benefactor,
    address indexed beneficiary,
    address collateral_asset,
    uint256 collateral_amount,
    uint256 usde_amount
  );

  /// @notice Event emitted when a supported asset is added
  event AssetAdded(address indexed asset);

  /// @notice Event emitted when a supported asset is removed
  event AssetRemoved(address indexed asset);

  // @notice Event emitted when a custodian address is added
  event CustodianAddressAdded(address indexed custodian);

  // @notice Event emitted when a custodian address is removed
  event CustodianAddressRemoved(address indexed custodian);

  /// @notice Event emitted when assets are moved to custody provider wallet
  event CustodyTransfer(address indexed wallet, address indexed asset, uint256 amount);

  /// @notice Event emitted when USDe is set
  event USDeSet(address indexed USDe);

  /// @notice Event emitted when the max mint per block is changed
  event MaxMintPerBlockChanged(uint256 oldMaxMintPerBlock, uint256 newMaxMintPerBlock);

  /// @notice Event emitted when the max redeem per block is changed
  event MaxRedeemPerBlockChanged(uint256 oldMaxRedeemPerBlock, uint256 newMaxRedeemPerBlock);

  /// @notice Event emitted when a delegated signer is added, enabling it to sign orders on behalf of another address
  event DelegatedSignerAdded(address indexed signer, address indexed delegator);

  /// @notice Event emitted when a delegated signer is removed
  event DelegatedSignerRemoved(address indexed signer, address indexed delegator);

  /// @notice Event emitted when a delegated signer is initiated
  event DelegatedSignerInitiated(address indexed signer, address indexed delegator);
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";

interface IUSDe is IERC20, IERC20Permit, IERC20Metadata {
  function mint(address _to, uint256 _amount) external;

  function burn(uint256 _amount) external;

  function burnFrom(address account, uint256 amount) external;

  function grantRole(bytes32 role, address account) external;

  function setMinter(address newMinter) external;
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

interface IWETH9 {
  function deposit() external payable;
  function withdraw(uint256 wad) external payable;
  function totalSupply() external returns (uint256);
  function approve(address guy, uint256 wad) external returns (bool);
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

