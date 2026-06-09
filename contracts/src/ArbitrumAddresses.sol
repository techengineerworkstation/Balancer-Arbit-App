// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

library ArbitrumAddresses {
    address constant BALANCER_V3_VAULT = 0xba1333333333a1BA1108E8412f11850A5C319bA9;

    address constant SUSHI_V2_ROUTER = 0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506;
    address constant SUSHI_V2_FACTORY = 0xc35DADB65012eC5796536bD9864eD8773aBc74C4;
    address constant PANCAKE_V3_ROUTER = 0x1a1A2E343144B65fd481352a44b8bFc61dc97507;
    address constant PANCAKE_V3_FACTORY = 0x0BFbCF9fa4f9C56B0F40a671Ad40E38852d245B0;
    address constant UNI_V3_ROUTER = 0xE592427A0AEce92De3Edee1F18E0157C05861564;
    address constant UNI_V3_FACTORY = 0x1F98431c8aD98523631AE4a59f267346ea31F984;

    address constant USDC_TOKEN = 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;
    address constant USDT_TOKEN = 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9;
    address constant WETH_TOKEN = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
    address constant WBTC_TOKEN = 0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f;

    uint24 constant FEE_LOWEST = 100;
    uint24 constant FEE_LOW = 500;
    uint24 constant FEE_MEDIUM = 3000;
    uint24 constant FEE_HIGH = 10000;
}
