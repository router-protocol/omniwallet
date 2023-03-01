//SPDX-License-Identifier: MIT
pragma solidity >=0.8.0 <0.9.0;

import "evm-gateway-contract/contracts/IGateway.sol";
import "evm-gateway-contract/contracts/Utils.sol";
import "evm-gateway-contract/contracts/IApplication.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract AbstractAccount is IApplication {
    IGateway public gateway;
    string public forwarderAddress;

    constructor(address gatewayAddress, string memory _forwarderAddress) {
        gateway = IGateway(gatewayAddress);
        forwarderAddress = _forwarderAddress;
    }

    event SendToEvent(
        string indexed sender,
        address indexed recipient,
        address token,
        uint256 amount
    );

    /// @notice function to handle the request received from router chain
    /// @param sender is a forwarder Address (middleware contract)
    /// @param payload consists of {recipient,token,amount,isNative}
    function handleRequestFromRouter(string memory sender, bytes memory payload)
        public
    {
        require(msg.sender == address(gateway));
        require(
            keccak256(abi.encode(sender)) ==
                keccak256(abi.encode(forwarderAddress)),
            "Only user's forwarder can call"
        );
        (
            address payable recipient,
            address token,
            uint256 amount,
            bool isNative
        ) = abi.decode(payload, (address, address, uint256, bool));
        _handleSendTo(recipient, token, amount, isNative);
        emit SendToEvent(sender, recipient, token, amount);
    }

    /// @notice function to handle transfer logic it will handle seprately for native and ERC20 tokens
    /// @param recipient is where funds needs to transfer
    /// @param tokenAddress is token thst needs to transfer
    /// @param amount how much token needs to transfer
    /// @param isNative bool true if instruction is for native token
    function _handleSendTo(
        address payable recipient,
        address tokenAddress,
        uint256 amount,
        bool isNative
    ) internal {
        if (isNative) {
            (bool success, ) = recipient.call{value: amount}(new bytes(0));
            require(success, "Native transfer failed");
        } else {
            IERC20 token = IERC20(tokenAddress);
            require(token.transfer(recipient, amount), "Transfer failed");
        }
    }

     receive() external payable{}
}
