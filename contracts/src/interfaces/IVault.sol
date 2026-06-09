// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IVaultMain {
    function unlock(bytes calldata data) external returns (bytes memory result);
    function sendTo(IERC20 token, address recipient, uint256 amount) external;
    function settle(IERC20 token, uint256 amountHint) external returns (uint256 credit);
}
