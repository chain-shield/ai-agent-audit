
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.13;

import "./interfaces/IHybra.sol";

contract HYBR is IHybra {

    string public constant name = "HYBR";
    string public constant symbol = "HYBR";
    uint8 public constant decimals = 18;
    uint public totalSupply = 0;

    mapping(address => uint) public balanceOf;
    mapping(address => mapping(address => uint)) public allowance;

    bool public initialMinted;
    address public minter;

    event Transfer(address indexed from, address indexed to, uint value);
    event Approval(address indexed owner, address indexed spender, uint value);

    constructor() {
        minter = msg.sender;
        _mint(msg.sender, 0);
    }

    // No checks as its meant to be once off to set minting rights to BaseV1 Minter
    function setMinter(address _minter) external {
        require(msg.sender == minter);
        minter = _minter;
    }

    // Initial mint: total 50M    
    function initialMint(address _recipient) external {
        require(msg.sender == minter && !initialMinted);
        initialMinted = true;
        _mint(_recipient, 500 * 1e6 * 1e18);
    }

    function approve(address _spender, uint _value) external returns (bool) {
        allowance[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }

    function _mint(address _to, uint _amount) internal returns (bool) {
        totalSupply += _amount;
        unchecked {
            balanceOf[_to] += _amount;
        }
        emit Transfer(address(0x0), _to, _amount);
        return true;
    }

    function _transfer(address _from, address _to, uint _value) internal returns (bool) {
        balanceOf[_from] -= _value;
        unchecked {
            balanceOf[_to] += _value;
        }
        emit Transfer(_from, _to, _value);
        return true;
    }

    function transfer(address _to, uint _value) external returns (bool) {
        return _transfer(msg.sender, _to, _value);
    }

    function transferFrom(address _from, address _to, uint _value) external returns (bool) {
        uint allowed_from = allowance[_from][msg.sender];
        if (allowed_from != type(uint).max) {
            allowance[_from][msg.sender] -= _value;
        }
        return _transfer(_from, _to, _value);
    }

    function mint(address account, uint amount) external returns (bool) {
        require(msg.sender == minter, 'not allowed');
        _mint(account, amount);
        return true;
    }

    function burn(uint256 value) external returns (bool) {
        _burn(msg.sender, value);
        return true;
    }

    function burnFrom(address _from, uint _value) external returns (bool) {
        uint allowed_from = allowance[_from][msg.sender];
        if (allowed_from != type(uint).max) {
            allowance[_from][msg.sender] -= _value;
        }
        _burn(_from, _value);
        return true;
    }

    function _burn(address _from, uint _amount) internal returns (bool) {
        totalSupply -= _amount;
        balanceOf[_from] -= _amount;
        emit Transfer(_from, address(0x0), _amount);
        return true;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IHybra {
    function totalSupply() external view returns (uint);
    function balanceOf(address) external view returns (uint);
    function approve(address spender, uint value) external returns (bool);
    function transfer(address, uint) external returns (bool);
    function transferFrom(address,address,uint) external returns (bool);
    function mint(address, uint) external returns (bool);
    function minter() external returns (address);
    function burn(uint) external returns (bool);
    function burnFrom(address, uint) external returns (bool);
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "./BaseDeployScript.sol";

import {MinterUpgradeable} from "../contracts/MinterUpgradeable.sol";
import {HYBR} from "../contracts/HYBR.sol";
import {RewardHYBR} from "../contracts/RewardHYBR.sol";

contract InitMinter is BaseDeployScript {
    using stdJson for string;
    
    function run() external {
        uint256 deployerKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.rememberKey(deployerKey);
        
        // Load deployed contracts
        string memory tokenPath = getInputPath("Deploy2_TokenSystem");
        string memory minterPath = getInputPath("Deploy3b1_MinterRewards");
        string memory gaugeManagerPath = getInputPath("Deploy3a_GaugeFactories");
        string memory tokenJson = vm.readFile(tokenPath);
        string memory minterJson = vm.readFile(minterPath);
        string memory gaugeManagerJson = vm.readFile(gaugeManagerPath);

        address hybr = abi.decode(vm.parseJson(tokenJson, ".HYBR"), (address));
        address rewardHybr = abi.decode(vm.parseJson(tokenJson, ".RewardHYBR"), (address));
        address minter = abi.decode(vm.parseJson(minterJson, ".Minter"), (address));
        address gaugeManager = abi.decode(vm.parseJson(gaugeManagerJson, ".GaugeManager"), (address));
        address gHYBR = abi.decode(vm.parseJson(tokenJson, ".GrowthHYBR"), (address));
        console.log("=== Initialize Minter ===");
        console.log("HYBR:", hybr);
        console.log("RewardHYBR:", rewardHybr);
        console.log("Minter:", minter);
        console.log("Deployer:", deployer);
        
        // Check if initial mint was already done
        bool initialMinted = HYBR(hybr).initialMinted();
        console.log("Initial mint already done:", initialMinted);
        
        if (!initialMinted) {
            console.log("Performing initial mint first...");
            vm.startBroadcast(deployer);
            HYBR(hybr).initialMint(deployer);
            HYBR(hybr).setMinter(minter);
            vm.stopBroadcast();
            console.log("Initial mint completed - 500M HYBR minted to deployer");
        }
        
        vm.startBroadcast(deployer);
        
        // Initialize the minter with initial distribution
        // For this example, we'll just do a simple initialization with empty arrays
        // You can modify these arrays to include specific claimants and amounts
        address[] memory claimants = new address[](0);
        uint[] memory amounts = new uint[](0);
        uint max = 0; // Set to 0 for no additional minting during initialization
        
        console.log("Initializing Minter...");
        MinterUpgradeable(minter)._initialize(claimants, amounts, max);
        console.log("Minter initialized successfully");
        
        // Set minter on RewardHYBR (rHYBR)
        console.log("Setting minter on RewardHYBR...");
        RewardHYBR(rewardHybr).setGaugeManager(gaugeManager);
        RewardHYBR(rewardHybr).setGHYBR(gHYBR);
        console.log("Minter set on RewardHYBR successfully");
        
        vm.stopBroadcast();
        
        console.log("");
        console.log("=== Verification ===");
        
        // Verify the minter can now be used
        bool canMint = MinterUpgradeable(minter).check();
        uint256 period = MinterUpgradeable(minter).period();
        uint256 activePeriod = MinterUpgradeable(minter).active_period();
        
        console.log("Can mint now:", canMint);
        console.log("Current period:", period);
        console.log("Active period:", activePeriod);
        
        // Check HYBR supply and balance
        uint256 totalSupply = HYBR(hybr).totalSupply();
        uint256 deployerBalance = HYBR(hybr).balanceOf(deployer);
        
        console.log("Total HYBR supply:", totalSupply);
        console.log("Deployer HYBR balance:", deployerBalance);
        
        // Verify RewardHYBR minter setup
        address rHybrGHYBR = RewardHYBR(rewardHybr).gHYBR();
        console.log("RewardHYBR gHYBR:", rHybrGHYBR);
        console.log("Minter matches:", rHybrGHYBR == gHYBR);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "./BaseDeployScript.sol";

import {HYBR} from "../contracts/HYBR.sol";
import {RewardHYBR} from "../contracts/RewardHYBR.sol";
import {GrowthHYBR} from "../contracts/GovernanceHYBR.sol";
import {VotingEscrow} from "../contracts/VotingEscrow.sol";

contract Deploy2_TokenSystem is BaseDeployScript {
    using stdJson for string;
    
    function run() external {
        uint256 deployPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.rememberKey(deployPrivateKey);
        
        // Load infrastructure addresses
        string memory infraPath = getInputPath("Deploy1_Infrastructure");
        string memory infraJson = vm.readFile(infraPath);
        
        address veArtProxy = abi.decode(vm.parseJson(infraJson, ".VeArtProxy"), (address));
        
        console.log("=== Phase 2: Deploy Token System ===");
        console.log("Deployer:", deployerAddress);
        console.log("Using VeArtProxy:", veArtProxy);
        vm.startBroadcast(deployerAddress);
        
        // 1. Deploy HYBR Token
        HYBR hybr = new HYBR();
        console.log("HYBR:", address(hybr));
        
      
      
        
        // 2. Deploy VotingEscrow
        VotingEscrow votingEscrow = new VotingEscrow(
            address(hybr),
            veArtProxy
        );
        console.log("VotingEscrow:", address(votingEscrow));
        
          // 3. Deploy RewardHYBR Token
        RewardHYBR rewardHybr = new RewardHYBR(address(hybr), address(votingEscrow));
        console.log("RewardHYBR:", address(rewardHybr));
        
          // 4. Deploy GovernanceHYBR Token (will need additional addresses, deploy with placeholders for now)
        GrowthHYBR gHybr = new GrowthHYBR(
            address(hybr), 
            address(votingEscrow)
        );
        console.log("GrowthHYBR:", address(gHybr));
        vm.stopBroadcast();
        
        // Save to JSON
        string memory path = getOutputPath("Deploy2_TokenSystem");
        
        string memory json = "";
        json = vm.serializeAddress("tokenSystem", "HYBR", address(hybr));
        json = vm.serializeAddress("tokenSystem", "RewardHYBR", address(rewardHybr));
        json = vm.serializeAddress("tokenSystem", "GrowthHYBR", address(gHybr));
        json = vm.serializeAddress("tokenSystem", "VotingEscrow", address(votingEscrow));
        
        vm.writeJson(json, path);
        console.log("Addresses saved to:", path);
        
        console.log("\n=== Token System Deployment Complete ===");
    }
}
