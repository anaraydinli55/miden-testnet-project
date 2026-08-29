// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SKSBridge is ERC20, Ownable {
    address public relayer;
    mapping(bytes32 => bool) public processedNonces;
    
    event Mint(bytes32 indexed midenAccountId, uint256 amount, uint256 nonce, address indexed recipient);
    event Burn(bytes32 indexed midenAccountId, uint256 amount, uint256 nonce, bytes32 destMidenAccount);
    event RelayerUpdated(address indexed oldRelayer, address indexed newRelayer);

    modifier onlyRelayer() {
        require(msg.sender == relayer, "SKSBridge: Only relayer");
        _;
    }

    constructor() ERC20("Wrapped Sakasena", "wSKS") Ownable(msg.sender) {}

    function setRelayer(address _relayer) external onlyOwner {
        address old = relayer;
        relayer = _relayer;
        emit RelayerUpdated(old, _relayer);
    }

    function mint(bytes32 midenAccountId, uint256 amount, uint256 nonce, address recipient) external onlyRelayer {
        bytes32 txHash = keccak256(abi.encodePacked(midenAccountId, amount, nonce));
        require(!processedNonces[txHash], "SKSBridge: Nonce already used");
        processedNonces[txHash] = true;
        _mint(recipient, amount);
        emit Mint(midenAccountId, amount, nonce, recipient);
    }

    function burn(uint256 amount, bytes32 destMidenAccount) external {
        require(amount > 0, "SKSBridge: Amount must be > 0");
        require(destMidenAccount != bytes32(0), "SKSBridge: Invalid destination");
        _burn(msg.sender, amount);
        uint256 nonce = uint256(keccak256(abi.encodePacked(msg.sender, amount, destMidenAccount, block.timestamp)));
        emit Burn(bytes32(uint256(uint160(msg.sender))), amount, nonce, destMidenAccount);
    }
    
    function decimals() public pure override returns (uint8) {
        return 18;
    }
}
