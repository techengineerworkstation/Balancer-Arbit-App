// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "./interfaces/IVault.sol";

interface IUniswapV2Router {
    function getAmountsOut(uint256 amountIn, address[] calldata path)
        external view returns (uint256[] memory amounts);
    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);
}

interface ISwapRouterV3 {
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 sqrtPriceLimitX96;
    }
    function exactInputSingle(ExactInputSingleParams calldata params)
        external payable returns (uint256 amountOut);
}

interface ICurvePool {
    function exchange(int128 i, int128 j, uint256 dx, uint256 min_dy)
        external returns (uint256);
}

interface IAlgebraSwapRouter {
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint256 amountIn;
        uint256 minimumOutput;
        uint160 limitSqrtPrice;
        address recipient;
        uint256 deadline;
    }
    function exactInputSingle(ExactInputSingleParams calldata params)
        external payable returns (uint256 amountOut);
}

contract BalancerFlashLoanArbitrage is Ownable {
    using SafeERC20 for IERC20;

    address constant AAVE = 0xba5DdD1f9d7F3bE546e97a4C2D7b34c26c435bF9;
    address constant SUSHI = 0xd4d42Fca40609F09a632F85185db06415470AD69;
    address constant LINK = 0xf97f4df75117a78c1A5a0dbb814af92458539FB4;
    address constant GNS = 0x18c11FD286C5EC11c3b683Caa813B93f77155Ac9;
    address constant BAL = 0x040d1EdC9461295F50CBDb630f6C3cAb6ab31104;
    address constant MAGIC = 0x539bdE0d7Dbd336b79148AA742883198BBF60342;
    address constant PENDLE = 0x0c880f6761F1af8d9Aa9C466984b80DAb9a8c9e8;
    address constant WETH = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
    address constant USDC = 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;
    address constant USDT = 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9;
    address constant WBTC = 0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f;
    address constant GRAIL = 0x3d9907F9a368ad0a51Be60f7Da3b97cf940982D8;
    address constant DPEX = 0x6C2C2649d712c27D7405D35d9aD6b1C2233cBbae;
    address constant UNI = 0xFa7F8980b0f1E64A2062791cc3b0871572f1F7f0;
    address constant ARB = 0x912CE59144191C1204E64559FE8253a0e49E6548;
    address constant RDNT = 0x3082CC23568eA640225c2467663441610403D183;
    address constant GMX = 0xfc5A1A6EB076a2C7aD06eD22C90d7E710E35ad0a;

    address constant SUSHI_V2_ROUTER = 0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506;
    address constant PANCAKE_V3_ROUTER = 0x13f4EA83D0bd40E0A6C33c274740244243D0FC24;
    address constant UNI_V3_ROUTER = 0xE592427A0AEce92De3Edee1F18E0157C05861564;
    address constant CURVE_ROUTER = 0x2191718CD32d02B8E60BAdFFeA33E4B5DD9A0A0D;
    address constant CAMELOT_V2_ROUTER = 0xc873fEcbd354f5A56E00E710B90EF4201db2448d;
    address constant CAMELOT_V4_ROUTER = 0x4ee15342d6Deb297c3A2aA7CFFd451f788675F53;

    enum DexMode { SushiV2, PancakeV3, UniswapV3, Curve, CamelotV2, CamelotV4 }

    IVaultMain public immutable balancerVault;
    uint256 public slippageBps;
    uint256 public totalTrades;
    uint256 public totalProfit;

    event TradeExecuted(
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        uint256 profit,
        uint256 timestamp
    );
    event TradeReverted(
        address indexed tokenIn,
        uint256 amount,
        string reason,
        uint256 timestamp
    );
    event FundsWithdrawn(address indexed token, address indexed to, uint256 amount);

    modifier onlyOwner() {
        require(msg.sender == owner(), "Caller is not the owner");
        _;
    }

    constructor(address _balancerVault) Ownable(msg.sender) {
        balancerVault = IVaultMain(_balancerVault);
        slippageBps = 50;
    }

    function setSlippage(uint256 _slippageBps) external onlyOwner {
        require(_slippageBps <= 1000, "Slippage too high");
        slippageBps = _slippageBps;
    }

    function executeArbitrage(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        DexMode dexMode
    ) external onlyOwner {
        require(amountIn > 0, "Amount must be > 0");
        bytes memory userData = abi.encode(tokenIn, tokenOut, amountIn, dexMode);
        balancerVault.unlock(
            abi.encodeWithSelector(this.receiveFlashLoan.selector, userData)
        );
    }

    function receiveFlashLoan(bytes calldata userData) external {
        require(msg.sender == address(balancerVault), "Unauthorized callback");

        (
            address tokenIn,
            address tokenOut,
            uint256 amountIn,
            DexMode dexMode
        ) = abi.decode(userData, (address, address, uint256, DexMode));

        balancerVault.sendTo(IERC20(tokenIn), address(this), amountIn);

        uint256 balanceBefore = IERC20(tokenOut).balanceOf(address(this));
        uint256 amountOut = _swap(tokenIn, tokenOut, amountIn, dexMode);
        uint256 profit = IERC20(tokenOut).balanceOf(address(this)) - balanceBefore;

        IERC20(tokenIn).transfer(address(balancerVault), amountIn);
        balancerVault.settle(IERC20(tokenIn), amountIn);

        totalTrades++;
        totalProfit += profit;

        emit TradeExecuted(
            tokenIn,
            tokenOut,
            amountIn,
            amountOut,
            profit,
            block.timestamp
        );
    }

    function _swap(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        DexMode dexMode
    ) internal returns (uint256) {
        if (dexMode == DexMode.SushiV2) {
            return _swapV2(tokenIn, tokenOut, amountIn, SUSHI_V2_ROUTER);
        } else if (dexMode == DexMode.PancakeV3) {
            return _swapV3(tokenIn, tokenOut, amountIn, PANCAKE_V3_ROUTER, 500);
        } else if (dexMode == DexMode.UniswapV3) {
            return _swapV3(tokenIn, tokenOut, amountIn, UNI_V3_ROUTER, 3000);
        } else if (dexMode == DexMode.Curve) {
            return _swapCurve(tokenIn, tokenOut, amountIn);
        } else if (dexMode == DexMode.CamelotV2) {
            return _swapV2(tokenIn, tokenOut, amountIn, CAMELOT_V2_ROUTER);
        } else if (dexMode == DexMode.CamelotV4) {
            return _swapAlgebra(
                tokenIn,
                tokenOut,
                amountIn,
                CAMELOT_V4_ROUTER
            );
        }
        revert("Unknown DEX mode");
    }

    function _swapV2(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        address router
    ) internal returns (uint256) {
        IERC20(tokenIn).safeApprove(router, 0);
        IERC20(tokenIn).safeApprove(router, amountIn);

        address[] memory path = new address[](2);
        path[0] = tokenIn;
        path[1] = tokenOut;

        uint256 quotedOut = IUniswapV2Router(router).getAmountsOut(
            amountIn,
            path
        )[1];
        uint256 amountOutMin = quotedOut * (10000 - slippageBps) / 10000;

        uint256[] memory amounts = IUniswapV2Router(router)
            .swapExactTokensForTokens(
                amountIn,
                amountOutMin,
                path,
                address(this),
                block.timestamp + 300
            );

        return amounts[amounts.length - 1];
    }

    function _swapV3(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        address router,
        uint24 fee
    ) internal returns (uint256) {
        IERC20(tokenIn).safeApprove(router, 0);
        IERC20(tokenIn).safeApprove(router, amountIn);

        ISwapRouterV3.ExactInputSingleParams memory params = ISwapRouterV3
            .ExactInputSingleParams({
                tokenIn: tokenIn,
                tokenOut: tokenOut,
                fee: fee,
                recipient: address(this),
                deadline: block.timestamp + 300,
                amountIn: amountIn,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });

        return ISwapRouterV3(router).exactInputSingle(params);
    }

    function _swapCurve(
        address tokenIn,
        address tokenOut,
        uint256 amountIn
    ) internal returns (uint256) {
        IERC20(tokenIn).safeApprove(CURVE_ROUTER, amountIn);
        return ICurvePool(CURVE_ROUTER).exchange(0, 1, amountIn, 0);
    }

    function _swapAlgebra(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        address router
    ) internal returns (uint256) {
        IERC20(tokenIn).safeApprove(router, 0);
        IERC20(tokenIn).safeApprove(router, amountIn);

        IAlgebraSwapRouter.ExactInputSingleParams memory params = IAlgebraSwapRouter
            .ExactInputSingleParams({
                tokenIn: tokenIn,
                tokenOut: tokenOut,
                amountIn: amountIn,
                minimumOutput: 0,
                limitSqrtPrice: 0,
                recipient: address(this),
                deadline: block.timestamp + 300
            });

        return IAlgebraSwapRouter(router).exactInputSingle(params);
    }

    function withdraw(address token) external onlyOwner {
        uint256 balance = IERC20(token).balanceOf(address(this));
        require(balance > 0, "No balance to withdraw");
        IERC20(token).safeTransfer(owner(), balance);
        emit FundsWithdrawn(token, owner(), balance);
    }

    function withdrawETH() external onlyOwner {
        uint256 balance = address(this).balance;
        require(balance > 0, "No ETH balance");
        (bool success, ) = owner().call{value: balance}("");
        require(success, "ETH transfer failed");
        emit FundsWithdrawn(address(0), owner(), balance);
    }

    function getBalance(address token) external view returns (uint256) {
        return IERC20(token).balanceOf(address(this));
    }

    receive() external payable {}
    fallback() external payable {}
}
