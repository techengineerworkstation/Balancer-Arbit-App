// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

library PolygonAddresses {
    // Balancer V2
    address internal constant BALANCER_V2_VAULT = 0xBA12222222228d8Ba445958a75a0704d566BF2C8;

    // SushiSwap V2
    address internal constant SUSHI_ROUTER = 0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506;
    address internal constant SUSHI_V2_FACTORY = 0xc35DADB65012eC5796536bD9864eD8773aBc74C4;

    // PancakeSwap V3
    address internal constant PANCAKE_V3_ROUTER = 0x1a1A2E343144B65fd481352a44b8bFc61dc97507;
    address internal constant PANCAKE_V3_FACTORY = 0x0BFbCF9fa4f9C56B0F40a671Ad40E38852d245B0;

    // Uniswap V3
    address internal constant UNI_V3_ROUTER = 0xE592427A0AEce92De3Edee1F18E0157C05861564;
    address internal constant UNI_V3_FACTORY = 0x1F98431c8aD98523631AE4a59f267346ea31F984;

    // Tokens
    address internal constant BAL_TOKEN = 0x9a71012B13CA4d3D0Cdc72A315f260ac2810CfD6;
    address internal constant USDC_E_TOKEN = 0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174;
    address internal constant USDC_NATIVE = 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359;
    address internal constant USDT_TOKEN = 0xc2132D05D31c914a87C6611C10748AEb04B58e8F;
    address internal constant AAVE_TOKEN = 0xD6DF932A45C0f255f85145f286eA0b292B21C90B;
    address internal constant SUSHI_TOKEN = 0x0b3F86A2651c15778aDB8cE2eA1B961cfc6914F9;
    address internal constant GNS_TOKEN = 0xE5417Af564e4bFDA1c483642db72007871397896;
    address internal constant MATIC_TOKEN = 0x0000000000000000000000000000000000001010;
    address internal constant WMATIC_TOKEN = 0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270;
    address internal constant WETH_TOKEN = 0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619;

    // PancakeSwap V3 Fee Tiers
    uint24 internal constant PANCAKE_FEE_LOWEST = 100;
    uint24 internal constant PANCAKE_FEE_LOW = 500;
    uint24 internal constant PANCAKE_FEE_MEDIUM = 2500;
    uint24 internal constant PANCAKE_FEE_HIGH = 10000;

    // Uniswap V3 Fee Tiers
    uint24 internal constant UNI_FEE_LOWEST = 100;
    uint24 internal constant UNI_FEE_LOW = 500;
    uint24 internal constant UNI_FEE_MEDIUM = 3000;
    uint24 internal constant UNI_FEE_HIGH = 10000;
}
