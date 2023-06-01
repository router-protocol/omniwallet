//SPDX-License-Identifier: MIT
pragma solidity >=0.8.0 <0.9.0;

import "@routerprotocol/evm-gateway-contracts/contracts/IGateway.sol";
import "@routerprotocol/evm-gateway-contracts/contracts/IDapp.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract AbstractAccount is IDapp {
    IGateway public gateway;
    string public forwarderAddress;

    constructor(
        address gatewayAddress,
        string memory _forwarderAddress,
        string memory feePayer
    ) {
        gateway = IGateway(gatewayAddress);
        forwarderAddress = _forwarderAddress;
        gateway.setDappMetadata(feePayer);
    }

    event SendToEvent(
        string indexed sender,
        address indexed recipient,
        address token,
        uint256 amount
    );

    /// @notice function to handle the request received from router chain
    /// @param requestSender is a forwarder Address (middleware contract)
    /// @param packet consists of {recipient,token,amount,isNative}
    function iReceive(
        string memory requestSender,
        bytes memory packet,
        string memory srcChainId
    ) external override returns (bytes memory) {
        require(msg.sender == address(gateway));
        require(
            keccak256(abi.encode(requestSender)) ==
                keccak256(abi.encode(forwarderAddress)),
            "Only user's forwarder can call"
        );
        (
            address payable recipient,
            address token,
            uint256 amount,
            bool isNative
        ) = abi.decode(packet, (address, address, uint256, bool));
        _handleSendTo(recipient, token, amount, isNative);
        emit SendToEvent(requestSender, recipient, token, amount);
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

    function iAck(
        uint256 requestIdentifier,
        bool execFlags,
        bytes memory execData
    ) external {}

    receive() external payable {}
}
