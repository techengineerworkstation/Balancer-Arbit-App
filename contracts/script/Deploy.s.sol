// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import "forge-std/Script.sol";
import "../src/BalancerFlashLoanArbitrage.sol";
import "../src/PolygonAddresses.sol";

contract DeployBalancerArb is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);

        BalancerFlashLoanArbitrage arb = new BalancerFlashLoanArbitrage(
            PolygonAddresses.BALANCER_V3_VAULT,
            PolygonAddresses.SUSHI_ROUTER,
            PolygonAddresses.PANCAKE_V3_ROUTER,
            PolygonAddresses.PANCAKE_V3_FACTORY,
            PolygonAddresses.SUSHI_V3_FACTORY
        );

        vm.stopBroadcast();

        console.log("Arbitrage contract deployed at:", address(arb));
        console.log("Owner:", arb.owner());
        console.log("Balancer Vault:", address(arb.balancerVault()));
        console.log("Sushi Router:", address(arb.sushiRouter()));
        console.log("Pancake V3 Router:", address(arb.pancakeV3Router()));
        console.log("Pancake V3 Factory:", address(arb.pancakeV3Factory()));
    }
}