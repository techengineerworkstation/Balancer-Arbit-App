// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

interface IWETH is IERC20 {
    function deposit() external payable;
    function withdraw(uint256) external;
    function approve(address spender, uint256 amount) external returns (bool);
}