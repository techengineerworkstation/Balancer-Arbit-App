# Balancer V3 Flash Loan Arbitrage Bot

Production-grade arbitrage bot for Polygon network using Balancer V3 flash loans to trade BAL/USDC between Sushiswap and PancakeSwap V3.

## Architecture

```
├── contracts/          # Solidity smart contracts (Foundry)
├── src/                # Rust backend server (Actix-web)
├── web-app/            # Leptos web dashboard (WASM)
├── cli/                # Command-line interface bot
├── mobile-app/         # Flutter mobile app
├── tauri.conf.json     # Tauri desktop app config
├── Dockerfile          # Container deployment
└── docker-compose.yml  # Local development
```

## Quick Start

### 1. Deploy Smart Contract

```bash
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash && foundryup

# Deploy to Polygon
cd contracts
PRIVATE_KEY=0x... forge script script/Deploy.s.sol --rpc-url https://polygon-rpc.com --broadcast

# Verify contract
forge verify-contract <address> BalancerFlashLoanArbitrage --chain-id 137
```

### 2. Configure Environment

```bash
cp .env.example .env
# Edit .env with your private key and contract address
```

### 3. Run Web Dashboard

```bash
# Build Leptos app
cd web-app
trunk build --release

# Start server
cd ..
cargo run --release
```

### 4. Run CLI Bot

```bash
cd cli
cargo run --release -- start --interval 1000
```

## Docker Deployment

```bash
docker-compose up -d
```

## Railway Deployment

1. Push to GitHub
2. Connect repo to Railway
3. Set environment variables:
   - `RPC_URL`
   - `PRIVATE_KEY`
   - `CONTRACT_ADDRESS`
4. Deploy

## Gas Cost Estimates (Polygon)

| Operation | Gas | Cost (USD) |
|-----------|-----|------------|
| Contract Deployment | ~3M | $0.10-0.30 |
| Flash Loan + Swap | ~500K | $0.02-0.05 |
| Flash Loan + 2 Swaps | ~750K | $0.03-0.08 |
| Withdraw Profits | ~100K | $0.005-0.01 |

## Token Addresses (Polygon)

| Token | Address |
|-------|---------|
| BAL | 0x9a71012B13CA4d3D0Cdc72A315f260ac2810CfD6 |
| USDC | 0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174 |
| WETH | 0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619 |
| WMATIC | 0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270 |

## DEX Addresses (Polygon)

| DEX | Router |
|-----|--------|
| Sushiswap V2 | 0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506 |
| PancakeSwap V3 | 0x10ED43C718714eb63d5aA57B78B54704E256024E |
| Balancer V3 Vault | 0xbA1333333333a1BA1108E8412f11850A5C319bA9 |

## Safety Notes

- **Testnet First**: Always test on Mumbai/Amoy testnet before mainnet
- **Private Key Security**: Never commit private keys; use `.env` files
- **Start Small**: Begin with small borrow amounts
- **Monitor Gas**: Set `MAX_GAS_PRICE_GWEI` to avoid high gas costs
- **Daily Limits**: Configure `DAILY_START_HOUR` and `DAILY_END_HOUR` for safe hours