
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

pragma solidity ^0.8.4;
pragma experimental ABIEncoderV2;

import "../root/Ownable.sol";
import "./PublicSuffixList.sol";

contract SimplePublicSuffixList is PublicSuffixList, Ownable {
    mapping(bytes => bool) suffixes;

    event SuffixAdded(bytes suffix);

    function addPublicSuffixes(bytes[] memory names) public onlyOwner {
        for (uint256 i = 0; i < names.length; i++) {
            suffixes[names[i]] = true;
            emit SuffixAdded(names[i]);
        }
    }

    function isPublicSuffix(
        bytes calldata name
    ) external view override returns (bool) {
        return suffixes[name];
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
pragma solidity ^0.8.4;

contract Ownable {
    address public owner;

    event OwnershipTransferred(
        address indexed previousOwner,
        address indexed newOwner
    );

    modifier onlyOwner() {
        require(isOwner(msg.sender));
        _;
    }

    constructor() public {
        owner = msg.sender;
    }

    function transferOwnership(address newOwner) public onlyOwner {
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }

    function isOwner(address addr) public view returns (bool) {
        return owner == addr;
    }
}

pragma solidity ^0.8.4;

interface PublicSuffixList {
    function isPublicSuffix(bytes calldata name) external view returns (bool);
}

pragma solidity ^0.8.4;

contract Ownable {
    address public owner;

    event OwnershipTransferred(
        address indexed previousOwner,
        address indexed newOwner
    );

    modifier onlyOwner() {
        require(isOwner(msg.sender));
        _;
    }

    constructor() public {
        owner = msg.sender;
    }

    function transferOwnership(address newOwner) public onlyOwner {
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }

    function isOwner(address addr) public view returns (bool) {
        return owner == addr;
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

