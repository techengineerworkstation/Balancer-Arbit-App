// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import "forge-std/Test.sol";
import "../src/BalancerFlashLoanArbitrage.sol";
import "../src/interfaces/IERC20.sol";

contract MockBalancerVault {
    bool public isUnlocked;
    address public lastBorrower;

    function unlock(bytes calldata data) external returns (bytes memory) {
        isUnlocked = true;
        lastBorrower = msg.sender;

        (address recipient, address tokenBorrow, uint256 amount, bytes memory userData) = abi.decode(
            data,
            (address, address, uint256, bytes)
        );

        IERC20(tokenBorrow).transfer(recipient, amount);

        BalancerFlashLoanArbitrage(recipient).receiveFlashLoan(
            tokenBorrow,
            amount,
            abi.decode(userData, (BalancerFlashLoanArbitrage.SwapPath[], uint256))
        );

        isUnlocked = false;
        return "";
    }

    function settle(IERC20 token, uint256 amountHint) external returns (uint256) {
        uint256 balance = token.balanceOf(address(this));
        if (balance >= amountHint) {
            token.transfer(msg.sender, amountHint);
            return amountHint;
        }
        return balance;
    }

    function sendTo(IERC20 token, address to, uint256 amount) external {
        token.transfer(to, amount);
    }

    receive() external payable {}
}

contract MockSushiRouter {
    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts) {
        amounts = new uint256[](path.length);
        amounts[path.length - 1] = amountIn * 102 / 100;
        IERC20(path[path.length - 1]).transfer(to, amounts[path.length - 1]);
    }

    function getAmountsOut(uint256 amountIn, address[] calldata path) external view returns (uint256[] memory amounts) {
        amounts = new uint256[](path.length);
        amounts[path.length - 1] = amountIn * 102 / 100;
    }
}

contract MockPancakeV3Router {
    function exactInputSingle(
        IPancakeV3Router.ExactInputSingleParams calldata params
    ) external payable returns (uint256 amountOut) {
        amountOut = params.amountIn * 103 / 100;
        IERC20(params.tokenOut).transfer(params.recipient, amountOut);
    }
}

contract MockERC20 is IERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;

    mapping(address => uint256) private _balances;
    mapping(address => mapping(address => uint256)) private _allowances;

    constructor(string memory _name, string memory _symbol, uint8 _decimals) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
    }

    function mint(address to, uint256 amount) external {
        _balances[to] += amount;
        totalSupply += amount;
    }

    function balanceOf(address account) external view override returns (uint256) {
        return _balances[account];
    }

    function transfer(address recipient, uint256 amount) external override returns (bool) {
        _balances[msg.sender] -= amount;
        _balances[recipient] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        _allowances[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address sender, address recipient, uint256 amount) external override returns (bool) {
        _allowances[sender][msg.sender] -= amount;
        _balances[sender] -= amount;
        _balances[recipient] += amount;
        return true;
    }

    function allowance(address owner, address spender) external view override returns (uint256) {
        return _allowances[owner][spender];
    }
}

contract BalancerFlashLoanArbitrageTest is Test {
    BalancerFlashLoanArbitrage public arb;
    MockBalancerVault public vault;
    MockSushiRouter public sushi;
    MockPancakeV3Router public pancake;
    MockERC20 public bal;
    MockERC20 public usdc;
    address public deployer = address(this);

    function setUp() public {
        vault = new MockBalancerVault();
        sushi = new MockSushiRouter();
        pancake = new MockPancakeV3Router();
        bal = new MockERC20("Balancer", "BAL", 18);
        usdc = new MockERC20("USD Coin", "USDC", 6);

        arb = new BalancerFlashLoanArbitrage(
            address(vault),
            address(sushi),
            address(pancake)
        );

        vault.mint(address(vault), 1000000 * 1e18);
        bal.mint(address(vault), 1000000 * 1e18);
        usdc.mint(address(vault), 1000000 * 1e6);
    }

    function testOwnership() public {
        assertEq(arb.owner(), deployer);
    }

    function testPause() public {
        arb.pause();
        assertTrue(arb.paused());
    }

    function testUnpause() public {
        arb.pause();
        arb.unpause();
        assertFalse(arb.paused());
    }

    function testWithdraw() public {
        bal.mint(address(arb), 100 * 1e18);
        arb.withdraw(address(bal));
        assertEq(bal.balanceOf(deployer), 100 * 1e18);
    }

    function testGetStats() public {
        (uint256 trades, uint256 profit) = arb.getStats();
        assertEq(trades, 0);
        assertEq(profit, 0);
    }

    function testNonOwnerCannotExecute() public {
        address nonOwner = address(0xBEEF);
        vm.prank(nonOwner);
        vm.expectRevert("Not owner");
        arb.executeFlashLoan(
            address(usdc),
            1000 * 1e6,
            new BalancerFlashLoanArbitrage.SwapPath[](0),
            0
        );
    }

    function testPausedCannotExecute() public {
        arb.pause();
        vm.expectRevert("Contract is paused");
        arb.executeFlashLoan(
            address(usdc),
            1000 * 1e6,
            new BalancerFlashLoanArbitrage.SwapPath[](0),
            0
        );
    }

    function testReceiveFlashLoanUnauthorized() public {
        vm.expectRevert("Unauthorized callback");
        BalancerFlashLoanArbitrage.SwapPath[] memory swaps = new BalancerFlashLoanArbitrage.SwapPath[](0);
        arb.receiveFlashLoan(address(usdc), 1000 * 1e6, swaps, 0);
    }

    function testTransferOwnership() public {
        address newOwner = address(0xCAFE);
        arb.transferOwnership(newOwner);
        assertEq(arb.owner(), newOwner);
    }

    function testWithdrawZeroFails() public {
        vm.expectRevert("No balance");
        arb.withdraw(address(bal));
    }

    function testETHBalance() public {
        assertEq(arb.getETHBalance(), 0);
    }

    receive() external payable {}
}