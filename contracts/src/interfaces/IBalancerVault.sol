// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

interface IBalancerVault {
    function flashLoan(
        address recipient,
        IERC20[] calldata tokens,
        uint256[] calldata amounts,
        bytes calldata userData
    ) external;

    function sendTo(IERC20 token, address to, uint256 amount) external;
    function settle(IERC20 token, uint256 amountHint) external returns (uint256 credit);
}