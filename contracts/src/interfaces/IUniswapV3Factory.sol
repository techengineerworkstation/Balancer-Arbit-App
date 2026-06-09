// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

interface IUniswapV3Factory {
    function getPool(address tokenA, address tokenB, uint24 fee) external view returns (address pool);
    function createPool(address tokenA, address tokenB, uint24 fee) external returns (address pool);

    function getPool(
        address tokenA,
        address tokenB,
        uint24 fee,
        address deployer
    ) external view returns (address pool);

    event PoolCreated(
        address indexed token0,
        address indexed token1,
        uint24 indexed fee,
        int24 tickSpacing,
        address pool
    );
}