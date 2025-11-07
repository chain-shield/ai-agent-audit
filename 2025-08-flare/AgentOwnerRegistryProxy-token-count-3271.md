
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {AgentOwnerRegistry} from "./AgentOwnerRegistry.sol";


contract AgentOwnerRegistryProxy is ERC1967Proxy {
    constructor(
        address _implementationAddress,
        IGovernanceSettings _governanceSettings,
        address _initialGovernance
    )
        ERC1967Proxy(_implementationAddress,
            abi.encodeCall(AgentOwnerRegistry.initialize, (_governanceSettings, _initialGovernance)))
    {
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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
pragma solidity ^0.8.27;

import { UUPSUpgradeable } from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import { GovernedProxyImplementation } from "./GovernedProxyImplementation.sol";
import { IUUPSUpgradeable } from "../../utils/interfaces/IUUPSUpgradeable.sol";

/**
 * Implementation of UUPS proxy that uses Flare governance with timelock.
 **/
abstract contract GovernedUUPSProxyImplementation is
    UUPSUpgradeable,
    GovernedProxyImplementation,
    IUUPSUpgradeable
{
    constructor()
        GovernedProxyImplementation()
    {}

    /**
     * See UUPSUpgradeable.upgradeTo
     */
    function upgradeTo(address newImplementation)
        public override (IUUPSUpgradeable, UUPSUpgradeable)
        onlyGovernance
        onlyProxy
    {
        _upgradeToAndCallUUPS(newImplementation, new bytes(0), false);
    }

    /**
     * See UUPSUpgradeable.upgradeToAndCall
     */
    function upgradeToAndCall(address newImplementation, bytes memory data)
        public payable override (IUUPSUpgradeable, UUPSUpgradeable)
        onlyGovernance
        onlyProxy
    {
        _upgradeToAndCallUUPS(newImplementation, data, true);
    }

    /**
     * Unused. Only present to satisfy UUPSUpgradeable requirement.
     * The real check is in onlyGovernance modifier on upgradeTo and upgradeToAndCall.
     */
    function _authorizeUpgrade(address  /* _newImplementation */)
        internal pure override
    {
        assert(false);
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

/**
 * A special contract that holds Flare governance address.
 * This contract enables updating governance address and timelock only by hard forking the network,
 * meaning only by updating validator code.
 */
interface IGovernanceSettings {
    /**
     * Get the governance account address.
     * The governance address can only be changed by a hardfork.
     */
    function getGovernanceAddress() external view returns (address);

    /**
     * Get the time in seconds that must pass between a governance call and execution.
     * The timelock value can only be changed by a hardfork.
     */
    function getTimelock() external view returns (uint256);

    /**
     * Get the addresses of the accounts that are allowed to execute the timelocked governance calls
     * once the timelock period expires.
     * Executors can be changed without a hardfork, via a normal governance call.
     */
    function getExecutors() external view returns (address[] memory);

    /**
     * Check whether an address is one of the executors.
     */
    function isExecutor(address _address) external view returns (bool);
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

