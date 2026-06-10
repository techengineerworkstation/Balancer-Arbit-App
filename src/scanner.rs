use crate::blockchain::{BlockchainClient, SwapLeg, ARBITRUM_GAS_PRICE_GWEI};
use crate::config::{AppConfig, DexInfo, DexType, TokenInfo};
use ethers::abi::Abi;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

pub const V2_FACTORY_ABI: &str = r#"[
    {
        "inputs": [
            { "internalType": "address", "name": "", "type": "address" },
            { "internalType": "address", "name": "", "type": "address" }
        ],
        "name": "getPair",
        "outputs": [
            { "internalType": "address", "name": "", "type": "address" }
        ],
        "stateMutability": "view",
        "type": "function"
    }
]"#;

pub const V2_PAIR_ABI: &str = r#"[
    {
        "inputs": [],
        "name": "getReserves",
        "outputs": [
            { "internalType": "uint112", "name": "_reserve0", "type": "uint112" },
            { "internalType": "uint112", "name": "_reserve1", "type": "uint112" },
            { "internalType": "uint32", "name": "_blockTimestampLast", "type": "uint32" }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "token0",
        "outputs": [
            { "internalType": "address", "name": "", "type": "address" }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "token1",
        "outputs": [
            { "internalType": "address", "name": "", "type": "address" }
        ],
        "stateMutability": "view",
        "type": "function"
    }
]"#;

pub const V3_FACTORY_ABI: &str = r#"[
    {
        "inputs": [
            { "internalType": "address", "name": "", "type": "address" },
            { "internalType": "address", "name": "", "type": "address" },
            { "internalType": "uint24", "name": "", "type": "uint24" }
        ],
        "name": "getPool",
        "outputs": [
            { "internalType": "address", "name": "", "type": "address" }
        ],
        "stateMutability": "view",
        "type": "function"
    }
]"#;

pub const V3_POOL_ABI: &str = r#"[
    {
        "inputs": [],
        "name": "slot0",
        "outputs": [
            { "internalType": "uint160", "name": "sqrtPriceX96", "type": "uint160" },
            { "internalType": "int24", "name": "tick", "type": "int24" },
            { "internalType": "uint16", "name": "observationIndex", "type": "uint16" },
            { "internalType": "uint16", "name": "observationCardinality", "type": "uint16" },
            { "internalType": "uint16", "name": "observationCardinalityNext", "type": "uint16" },
            { "internalType": "uint8", "name": "feeProtocol", "type": "uint8" },
            { "internalType": "bool", "name": "unlocked", "type": "bool" }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "liquidity",
        "outputs": [
            { "internalType": "uint128", "name": "", "type": "uint128" }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "token0",
        "outputs": [
            { "internalType": "address", "name": "", "type": "address" }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "token1",
        "outputs": [
            { "internalType": "address", "name": "", "type": "address" }
        ],
        "stateMutability": "view",
        "type": "function"
    }
]"#;

fn parse_abi(json: &str) -> Abi {
    serde_json::from_str(json).expect("Invalid ABI JSON")
}

pub fn estimate_swap_cost_usd(gas_price_gwei: f64) -> f64 {
    let gas_units = 150_000.0;
    let eth_price = crate::blockchain::ETH_PRICE_USD;
    (gas_units * gas_price_gwei * 1e-9) * eth_price
}

fn compute_v2_pair_address(
    factory: H160,
    token_a: H160,
    token_b: H160,
    init_code_hash: &str,
) -> H160 {
    let (token0, token1) = if token_a < token_b {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    };

    let salt_input = {
        let mut buf = [0u8; 40];
        buf[..20].copy_from_slice(token0.as_fixed_bytes());
        buf[20..].copy_from_slice(token1.as_fixed_bytes());
        buf
    };

    let salt = H256::from(ethers::utils::keccak256(salt_input));
    let init_hash = H256::from_str(init_code_hash).unwrap_or_default();

    let create2_input = {
        let mut buf = [0u8; 85];
        buf[0] = 0xff;
        buf[1..21].copy_from_slice(factory.as_fixed_bytes());
        buf[21..53].copy_from_slice(salt.as_fixed_bytes());
        buf[53..].copy_from_slice(init_hash.as_fixed_bytes());
        buf
    };

    let hash = ethers::utils::keccak256(create2_input);
    let mut addr_bytes = [0u8; 20];
    addr_bytes.copy_from_slice(&hash[12..]);
    H160::from(addr_bytes)
}

fn v2_get_amount_out(amount_in_wei: U256, reserve_in: U256, reserve_out: U256) -> U256 {
    let amount_in_with_fee = amount_in_wei * 997;
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in * 1000 + amount_in_with_fee;
    numerator / denominator
}

async fn get_v2_swap_output(
    client: &BlockchainClient,
    factory_address: &str,
    token_in: &str,
    token_out: &str,
    amount_in_human: f64,
    decimals_in: u32,
    decimals_out: u32,
    init_code_hash: Option<&str>,
) -> Result<(f64, f64), String> {
    let factory_addr = H160::from_str(factory_address)
        .map_err(|e| format!("Invalid factory: {}", e))?;
    let token_in_addr = H160::from_str(token_in)
        .map_err(|e| format!("Invalid token_in: {}", e))?;
    let token_out_addr = H160::from_str(token_out)
        .map_err(|e| format!("Invalid token_out: {}", e))?;

    let pair = if let Some(hash) = init_code_hash {
        compute_v2_pair_address(factory_addr, token_in_addr, token_out_addr, hash)
    } else {
        let factory = Contract::new(factory_addr, parse_abi(V2_FACTORY_ABI), client.provider().clone());
        factory
            .method("getPair", (token_in_addr, token_out_addr))
            .map_err(|e| format!("getPair method: {}", e))?
            .call()
            .await
            .map_err(|e| format!("getPair call: {}", e))?
    };

    if pair == H160::zero() {
        return Err("No pair".to_string());
    }

    let pair_contract = Contract::new(pair, parse_abi(V2_PAIR_ABI), client.provider().clone());
    let token0: H160 = pair_contract
        .method("token0", ())
        .map_err(|e| format!("token0: {}", e))?
        .call()
        .await
        .map_err(|e| format!("token0 call: {}", e))?;

    let (reserve0, reserve1, _): (U256, U256, u32) = pair_contract
        .method("getReserves", ())
        .map_err(|e| format!("getReserves: {}", e))?
        .call()
        .await
        .map_err(|e| format!("getReserves call: {}", e))?;

    let (reserve_in, reserve_out) = if token_in_addr == token0 {
        (reserve0, reserve1)
    } else {
        (reserve1, reserve0)
    };

    if reserve_in.is_zero() || reserve_out.is_zero() {
        return Err("Zero reserves".to_string());
    }

    let amount_in_wei = U256::from((amount_in_human * 10f64.powi(decimals_in as i32)) as u64);
    let amount_out_wei = v2_get_amount_out(amount_in_wei, reserve_in, reserve_out);

    let amount_out_human = amount_out_wei.as_u128() as f64 / 10f64.powi(decimals_out as i32);

    let spot_price = {
        let r_in = reserve_in.as_u128() as f64 / 10f64.powi(decimals_in as i32);
        let r_out = reserve_out.as_u128() as f64 / 10f64.powi(decimals_out as i32);
        r_out / r_in
    };

    let exec_price = if amount_in_human > 0.0 { amount_out_human / amount_in_human } else { 0.0 };

    let price_impact = if spot_price > 0.0 && exec_price > 0.0 {
        ((spot_price - exec_price) / spot_price * 10000.0).max(0.0)
    } else {
        10000.0
    };

    Ok((amount_out_human, price_impact))
}

async fn get_v3_swap_output(
    client: &BlockchainClient,
    factory_address: &str,
    token_in: &str,
    token_out: &str,
    fee_tier: u32,
    amount_in_human: f64,
    decimals_in: u32,
    decimals_out: u32,
) -> Result<(f64, f64), String> {
    let factory_addr = H160::from_str(factory_address)
        .map_err(|e| format!("Invalid factory: {}", e))?;
    let token_in_addr = H160::from_str(token_in)
        .map_err(|e| format!("Invalid token_in: {}", e))?;
    let token_out_addr = H160::from_str(token_out)
        .map_err(|e| format!("Invalid token_out: {}", e))?;

    let factory = Contract::new(factory_addr, parse_abi(V3_FACTORY_ABI), client.provider().clone());
    let pool: H160 = factory
        .method("getPool", (token_in_addr, token_out_addr, fee_tier))
        .map_err(|e| format!("getPool: {}", e))?
        .call()
        .await
        .map_err(|e| format!("getPool call: {}", e))?;

    if pool == H160::zero() {
        return Err("No V3 pool".to_string());
    }

    let pool_contract = Contract::new(pool, parse_abi(V3_POOL_ABI), client.provider().clone());
    let slot0: (U256, i32, u16, u16, u16, u8, bool) = pool_contract
        .method("slot0", ())
        .map_err(|e| format!("slot0: {}", e))?
        .call()
        .await
        .map_err(|e| format!("slot0 call: {}", e))?;

    let liquidity: U128 = pool_contract
        .method("liquidity", ())
        .map_err(|e| format!("liquidity: {}", e))?
        .call()
        .await
        .map_err(|e| format!("liquidity call: {}", e))?;

    if liquidity.is_zero() {
        return Err("Zero liquidity".to_string());
    }

    let sqrt_price_x96 = slot0.0;
    if sqrt_price_x96.is_zero() {
        return Err("Zero sqrt price".to_string());
    }

    let pool_token0: H160 = pool_contract
        .method("token0", ())
        .map_err(|e| format!("pool token0: {}", e))?
        .call()
        .await
        .map_err(|e| format!("pool token0 call: {}", e))?;

    let (dec_token0, dec_token1) = if pool_token0 == token_in_addr {
        (decimals_in, decimals_out)
    } else {
        (decimals_out, decimals_in)
    };

    let sqrt_price_f64: f64 = sqrt_price_x96.to_string().parse().unwrap_or(0.0);
    let raw_price = (sqrt_price_f64 / 2f64.powi(96)).powi(2);
    let price_token1_per_token0 = raw_price * 10f64.powi(dec_token0 as i32 - dec_token1 as i32);

    let spot_price = if token_in_addr == pool_token0 {
        price_token1_per_token0
    } else {
        if price_token1_per_token0 > 0.0 { 1.0 / price_token1_per_token0 } else { 0.0 }
    };

    let amount_out_human = amount_in_human * spot_price;

    let liq_f64: f64 = liquidity.as_u128() as f64;
    let amount_in_raw = amount_in_human * 10f64.powi(decimals_in as i32);

    let price_impact = if liq_f64 > 0.0 {
        (amount_in_raw / liq_f64 * 10000.0).min(10000.0)
    } else {
        10000.0
    };

    Ok((amount_out_human, price_impact))
}

async fn get_dex_swap_output(
    client: &BlockchainClient,
    dex: &DexInfo,
    token_in: &str,
    token_out: &str,
    amount_in_human: f64,
    decimals_in: u32,
    decimals_out: u32,
) -> Result<(f64, f64), String> {
    match dex.dex_type {
        DexType::SushiV2 | DexType::CamelotV2 => {
            get_v2_swap_output(client, &dex.factory_address, token_in, token_out, amount_in_human, decimals_in, decimals_out, dex.init_code_hash.as_deref()).await
        }
        DexType::PancakeV3 | DexType::UniswapV3 | DexType::CamelotV4 => {
            let fee = dex.fee_tier.unwrap_or(3000);
            get_v3_swap_output(client, &dex.factory_address, token_in, token_out, fee, amount_in_human, decimals_in, decimals_out).await
        }
        DexType::Curve => {
            Err("Curve scanning not implemented".to_string())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapPair {
    pub token_symbol: String,
    pub token_address: String,
    pub base_symbol: String,
    pub base_address: String,
    pub dex_name: String,
    pub dex_type: DexType,
    pub router_address: String,
    pub factory_address: String,
    pub pair_id: String,
    pub fee_tier: Option<u32>,
}

pub fn generate_swap_pairs(config: &AppConfig) -> Vec<SwapPair> {
    let mut pairs = Vec::new();

    let base_tokens: Vec<&TokenInfo> = config
        .tokens
        .iter()
        .filter(|t| config.base_pairs.contains(&t.symbol))
        .collect();

    let _non_base_tokens: Vec<&TokenInfo> = config
        .tokens
        .iter()
        .filter(|t| !config.base_pairs.contains(&t.symbol))
        .collect();

    for token in &config.tokens {
        for base in &base_tokens {
            if token.symbol == base.symbol {
                continue;
            }
            for dex in &config.dexes {
                let pair_id = format!("{}-{}-{}", token.symbol, base.symbol, dex.name);
                pairs.push(SwapPair {
                    token_symbol: token.symbol.clone(),
                    token_address: token.address.clone(),
                    base_symbol: base.symbol.clone(),
                    base_address: base.address.clone(),
                    dex_name: dex.name.clone(),
                    dex_type: dex.dex_type.clone(),
                    router_address: dex.router_address.clone(),
                    factory_address: dex.factory_address.clone(),
                    pair_id,
                    fee_tier: dex.fee_tier,
                });
            }
        }
    }

    pairs.sort_by(|a, b| {
        a.token_symbol
            .cmp(&b.token_symbol)
            .then(a.base_symbol.cmp(&b.base_symbol))
            .then(a.dex_name.cmp(&b.dex_name))
    });

    pairs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub token_pair: String,
    pub dex_from: String,
    pub dex_to: String,
    pub amount: f64,
    pub expected_profit: f64,
    pub price_impact_bps: u32,
    pub slippage_bps: u32,
    pub net_profit_after_costs: f64,
    pub route_description: String,
    pub token_in_address: String,
    pub token_out_address: String,
    pub dex_from_router: String,
    pub dex_to_router: String,
    pub fee_tier: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotStatus {
    pub is_running: bool,
    pub total_trades: u32,
    pub total_profit: f64,
    pub last_trade_time: Option<String>,
    pub gas_spent: f64,
    pub balance: f64,
    pub network: String,
    pub avg_slippage_bps: f64,
    pub avg_price_impact_bps: f64,
    pub active_tokens: Vec<String>,
    pub active_dexes: Vec<String>,
    pub scan_interval_ms: u64,
    pub eth_price: f64,
    pub current_gas_price_gwei: f64,
    pub estimated_fee_per_trade: f64,
    pub last_scan: Option<String>,
    pub total_scans: u64,
    pub opportunities_found: u32,
    pub pairs_scanned: u32,
    pub scanning: bool,
}

pub struct Scanner {
    client: Arc<BlockchainClient>,
    config: AppConfig,
}

impl Scanner {
    pub fn new(client: Arc<BlockchainClient>, config: AppConfig) -> Self {
        Self { client, config }
    }

    async fn get_v2_price(
        &self,
        factory_address: &str,
        token_in: &str,
        token_out: &str,
        init_code_hash: Option<&str>,
    ) -> Result<(f64, f64, f64), String> {
        let factory_addr = H160::from_str(factory_address)
            .map_err(|e| format!("Invalid factory address: {}", e))?;
        let token_in_addr = H160::from_str(token_in)
            .map_err(|e| format!("Invalid token_in address: {}", e))?;
        let token_out_addr = H160::from_str(token_out)
            .map_err(|e| format!("Invalid token_out address: {}", e))?;

        let pair = if let Some(hash) = init_code_hash {
            compute_v2_pair_address(factory_addr, token_in_addr, token_out_addr, hash)
        } else {
            let factory =
                Contract::new(factory_addr, parse_abi(V2_FACTORY_ABI), self.client.provider().clone());
            factory
                .method("getPair", (token_in_addr, token_out_addr))
                .map_err(|e| format!("Failed to get pair: {}", e))?
                .call()
                .await
                .map_err(|e| format!("getPair call failed: {}", e))?
        };

        if pair == H160::zero() {
            return Err("Pair does not exist".to_string());
        }

        let pair_contract =
            Contract::new(pair, parse_abi(V2_PAIR_ABI), self.client.provider().clone());

        let token0: H160 = pair_contract
            .method("token0", ())
            .map_err(|e| format!("Failed to get token0: {}", e))?
            .call()
            .await
            .map_err(|e| format!("token0 call failed: {}", e))?;

        let (reserve0, reserve1, _): (U256, U256, u32) = pair_contract
            .method("getReserves", ())
            .map_err(|e| format!("Failed to get reserves: {}", e))?
            .call()
            .await
            .map_err(|e| format!("getReserves call failed: {}", e))?;

        let (reserve_in, reserve_out) = if token_in_addr == token0 {
            (reserve0, reserve1)
        } else {
            (reserve1, reserve0)
        };

        if reserve_in.is_zero() || reserve_out.is_zero() {
            return Err("Zero reserves".to_string());
        }

        let amount_in_human = 10000.0;
        let token_in_config = self.get_token_config(token_in);
        let token_out_config = self.get_token_config(token_out);
        let decimals_in = token_in_config.map(|t| t.decimals).unwrap_or(18);
        let decimals_out = token_out_config.map(|t| t.decimals).unwrap_or(18);

        let amount_in_raw = amount_in_human * 10f64.powi(decimals_in as i32);
        let amount_in_wei = U256::from(amount_in_raw as u64);

        let amount_in_with_fee = amount_in_wei * 997;
        let numerator = amount_in_with_fee * reserve_out;
        let denominator = reserve_in * 1000 + amount_in_with_fee;
        let amount_out_wei = numerator / denominator;

        let amount_out_human =
            format!("{}", amount_out_wei).parse::<f64>().unwrap_or(0.0)
                / 10f64.powi(decimals_out as i32);

        let price_impact = 1.0 - (amount_out_human / amount_in_human).abs();

        let spot_price = format!("{}", reserve_out).parse::<f64>().unwrap_or(1.0)
            / format!("{}", reserve_in).parse::<f64>().unwrap_or(1.0);

        Ok((spot_price, price_impact * 10000.0, amount_out_human))
    }

    async fn get_v3_price(
        &self,
        factory_address: &str,
        token_in: &str,
        token_out: &str,
        fee_tier: u32,
    ) -> Result<(f64, f64, f64, f64), String> {
        let factory_addr = H160::from_str(factory_address)
            .map_err(|e| format!("Invalid factory address: {}", e))?;
        let token_in_addr = H160::from_str(token_in)
            .map_err(|e| format!("Invalid token_in address: {}", e))?;
        let token_out_addr = H160::from_str(token_out)
            .map_err(|e| format!("Invalid token_out address: {}", e))?;

        let factory =
            Contract::new(factory_addr, parse_abi(V3_FACTORY_ABI), self.client.provider().clone());

        let pool: H160 = factory
            .method("getPool", (token_in_addr, token_out_addr, fee_tier))
            .map_err(|e| format!("Failed to get V3 pool: {}", e))?
            .call()
            .await
            .map_err(|e| format!("getPool call failed: {}", e))?;

        if pool == H160::zero() {
            return Err("V3 pool does not exist".to_string());
        }

        let pool_contract =
            Contract::new(pool, parse_abi(V3_POOL_ABI), self.client.provider().clone());

        let slot0: (U256, i32, u16, u16, u16, u8, bool) = pool_contract
            .method("slot0", ())
            .map_err(|e| format!("Failed to get slot0: {}", e))?
            .call()
            .await
            .map_err(|e| format!("slot0 call failed: {}", e))?;

        let liquidity: U128 = pool_contract
            .method("liquidity", ())
            .map_err(|e| format!("Failed to get liquidity: {}", e))?
            .call()
            .await
            .map_err(|e| format!("liquidity call failed: {}", e))?;

        if liquidity.is_zero() {
            return Err("Zero liquidity".to_string());
        }

        let sqrt_price_x96 = slot0.0;
        if sqrt_price_x96.is_zero() {
            return Err("Zero sqrt price".to_string());
        }

        let token_in_config = self.get_token_config(token_in);
        let token_out_config = self.get_token_config(token_out);
        let decimals_in = token_in_config.map(|t| t.decimals).unwrap_or(18);
        let decimals_out = token_out_config.map(|t| t.decimals).unwrap_or(18);

        let sqrt_price_f64 = format!("{}", sqrt_price_x96).parse::<f64>().unwrap_or(0.0);
        let price = (sqrt_price_f64 / 2f64.powi(96)).powi(2);
        let price_adjusted = price * 10f64.powi((decimals_in as i32) - (decimals_out as i32));

        let amount_in_human = 10000.0;
        let amount_out_human = amount_in_human / price_adjusted;

        let liq_f64 = format!("{}", liquidity).parse::<f64>().unwrap_or(1.0);
        let price_impact_bps = if liq_f64 > 0.0 {
            ((amount_in_human * price_adjusted) / liq_f64 * 10000.0).min(10000.0) as u32
        } else {
            10000
        };

        Ok((
            price_adjusted,
            price_impact_bps as f64,
            amount_out_human,
            liquidity.as_u128() as f64,
        ))
    }

    fn is_v2_dex(dex_type: &DexType) -> bool {
        matches!(dex_type, DexType::SushiV2 | DexType::CamelotV2)
    }

    fn is_v3_dex(dex_type: &DexType) -> bool {
        matches!(
            dex_type,
            DexType::PancakeV3 | DexType::UniswapV3 | DexType::CamelotV4
        )
    }

    async fn get_price_for_dex(
        &self,
        dex: &DexInfo,
        token_in: &str,
        token_out: &str,
    ) -> Result<(f64, f64, f64), String> {
        match dex.dex_type {
            DexType::SushiV2 | DexType::CamelotV2 => {
                self.get_v2_price(&dex.factory_address, token_in, token_out, dex.init_code_hash.as_deref())
                    .await
            }
            DexType::PancakeV3 | DexType::UniswapV3 => {
                let fee = dex.fee_tier.unwrap_or(3000);
                let result = self
                    .get_v3_price(&dex.factory_address, token_in, token_out, fee)
                    .await;
                match result {
                    Ok((price, impact, amount_out, _liquidity)) => Ok((price, impact, amount_out)),
                    Err(e) => Err(e),
                }
            }
            DexType::Curve => {
                self.get_curve_price(&dex.router_address, token_in, token_out)
                    .await
            }
            DexType::CamelotV4 => {
                let fee = dex.fee_tier.unwrap_or(500);
                let result = self
                    .get_v3_price(&dex.factory_address, token_in, token_out, fee)
                    .await;
                match result {
                    Ok((price, impact, amount_out, _liquidity)) => Ok((price, impact, amount_out)),
                    Err(e) => Err(e),
                }
            }
        }
    }

    async fn get_curve_price(
        &self,
        router_address: &str,
        token_in: &str,
        token_out: &str,
    ) -> Result<(f64, f64, f64), String> {
        let _router_addr = H160::from_str(router_address)
            .map_err(|e| format!("Invalid Curve router address: {}", e))?;
        let _token_in_addr = H160::from_str(token_in)
            .map_err(|e| format!("Invalid token_in address: {}", e))?;
        let _token_out_addr = H160::from_str(token_out)
            .map_err(|e| format!("Invalid token_out address: {}", e))?;

        Err("Curve price estimation via static call not implemented".to_string())
    }

    async fn scan_token_pair(
        &self,
        token_in: &TokenInfo,
        token_out: &TokenInfo,
    ) -> Vec<Opportunity> {
        let mut opportunities = Vec::new();
        let gas_price = self.client.get_gas_price().await.unwrap_or(ARBITRUM_GAS_PRICE_GWEI);

        for i in 0..self.config.dexes.len() {
            for j in 0..self.config.dexes.len() {
                if i == j {
                    continue;
                }

                let dex_from = &self.config.dexes[i];
                let dex_to = &self.config.dexes[j];

                let buy_result = self
                    .get_price_for_dex(dex_from, &token_out.address, &token_in.address)
                    .await;

                let sell_result = self
                    .get_price_for_dex(dex_to, &token_in.address, &token_out.address)
                    .await;

                if let (Ok(buy), Ok(sell)) = (buy_result, sell_result) {
                    let buy_price = buy.0;
                    let buy_impact_bps = buy.1 as u32;
                    let sell_price = sell.0;
                    let sell_impact_bps = sell.1 as u32;

                    let price_diff = sell_price - buy_price;
                    let avg_price = (sell_price + buy_price) / 2.0;

                    if avg_price > 0.0 && price_diff > 0.0 {
                        let profit_pct = (price_diff / avg_price) * 100.0;

                        let swap_cost = self.client.estimate_swap_cost_usd(gas_price);
                        let expected_profit = self.config.borrow_amount * profit_pct / 100.0;
                        let total_cost = swap_cost * 2.0 + 0.05;
                        let net_profit = expected_profit - total_cost;

                        if net_profit > self.config.min_profit_usd
                            && buy_impact_bps <= (self.config.max_price_impact_pct * 100.0) as u32
                            && sell_impact_bps <= (self.config.max_price_impact_pct * 100.0) as u32
                        {
                            let total_impact = buy_impact_bps + sell_impact_bps;

                            let fee_tier_info = if Self::is_v3_dex(&dex_from.dex_type) {
                                dex_from.fee_tier
                            } else {
                                None
                            };

                            opportunities.push(Opportunity {
                                token_pair: format!(
                                    "{}/{}",
                                    token_in.symbol, token_out.symbol
                                ),
                                dex_from: dex_from.name.clone(),
                                dex_to: dex_to.name.clone(),
                                amount: self.config.borrow_amount,
                                expected_profit,
                                price_impact_bps: total_impact,
                                slippage_bps: total_impact / 2,
                                net_profit_after_costs: net_profit,
                                route_description: format!(
                                    "Borrow {} on Balancer V3 → Buy {} on {} → Sell {} on {} → Repay",
                                    token_out.symbol,
                                    token_in.symbol, dex_from.name,
                                    token_in.symbol, dex_to.name
                                ),
                                token_in_address: token_in.address.clone(),
                                token_out_address: token_out.address.clone(),
                                dex_from_router: dex_from.router_address.clone(),
                                dex_to_router: dex_to.router_address.clone(),
                                fee_tier: fee_tier_info,
                            });
                        }
                    }
                }
            }
        }

        opportunities
    }

    pub async fn scan_opportunity_with_amount(
        &self,
        borrow_amount: f64,
    ) -> Option<Opportunity> {
        let mut all_opportunities = Vec::new();
        let base_tokens: Vec<&TokenInfo> = self
            .config
            .tokens
            .iter()
            .filter(|t| self.config.base_pairs.contains(&t.symbol))
            .collect();

        let non_base_tokens: Vec<&TokenInfo> = self
            .config
            .tokens
            .iter()
            .filter(|t| !self.config.base_pairs.contains(&t.symbol))
            .collect();

        for non_base in &non_base_tokens {
            for base in &base_tokens {
                let mut opps = self.scan_token_pair(base, non_base).await;
                for opp in &mut opps {
                    opp.amount = borrow_amount;
                }
                all_opportunities.extend(opps);

                if self.config.reverse_route {
                    let mut opps = self.scan_token_pair(non_base, base).await;
                    for opp in &mut opps {
                        opp.amount = borrow_amount;
                    }
                    all_opportunities.extend(opps);
                }
            }
        }

        all_opportunities
            .into_iter()
            .max_by(|a, b| {
                a.net_profit_after_costs
                    .partial_cmp(&b.net_profit_after_costs)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    pub async fn scan_all(&self) -> Vec<Opportunity> {
        use futures::future::join_all;
        let mut all_opportunities = Vec::new();

        let base_tokens: Vec<TokenInfo> = self
            .config
            .tokens
            .iter()
            .filter(|t| self.config.base_pairs.contains(&t.symbol))
            .cloned()
            .collect();

        let non_base_tokens: Vec<TokenInfo> = self
            .config
            .tokens
            .iter()
            .filter(|t| !self.config.base_pairs.contains(&t.symbol))
            .cloned()
            .collect();

        let mut futures = Vec::new();

        for non_base in &non_base_tokens {
            for base in &base_tokens {
                let client = self.client.clone();
                let dexes = self.config.dexes.clone();
                let borrow_amount = self.config.borrow_amount;
                let min_profit = self.config.min_profit_usd;
                let max_impact = self.config.max_price_impact_pct;
                let token_in_addr = base.address.clone();
                let token_out_addr = non_base.address.clone();
                let token_in_sym = base.symbol.clone();
                let token_out_sym = non_base.symbol.clone();
                let base_sym = base.symbol.clone();
                let base_decimals = base.decimals;
                let non_base_decimals = non_base.decimals;

                futures.push(Box::pin(async move {
                    let mut opps = Vec::new();
                    let gas_price = client.get_gas_price().await.unwrap_or(0.02);
                    let swap_cost = estimate_swap_cost_usd(gas_price);
                    let total_cost = swap_cost * 2.0 + 0.05;

                    let base_token_amount = match base_sym.as_str() {
                        "USDC" | "USDT" => borrow_amount,
                        "WETH" => borrow_amount / crate::blockchain::ETH_PRICE_USD,
                        "WBTC" => borrow_amount / crate::blockchain::BTC_PRICE_USD,
                        _ => borrow_amount,
                    };

                    for i in 0..dexes.len() {
                        for j in 0..dexes.len() {
                            if i == j { continue; }
                            let dex_from = &dexes[i];
                            let dex_to = &dexes[j];

                            let buy_result = get_dex_swap_output(
                                &client, dex_from, &token_in_addr, &token_out_addr,
                                base_token_amount, base_decimals, non_base_decimals,
                            ).await;

                            if let Ok((amount_out_1, buy_impact)) = buy_result {
                                if amount_out_1 > 0.0 {
                                    let sell_result = get_dex_swap_output(
                                        &client, dex_to, &token_out_addr, &token_in_addr,
                                        amount_out_1, non_base_decimals, base_decimals,
                                    ).await;

                                    if let Ok((amount_out_2, sell_impact)) = sell_result {
                                        let base_usd_price = match base_sym.as_str() {
                                            "USDC" | "USDT" => 1.0,
                                            "WETH" => crate::blockchain::ETH_PRICE_USD,
                                            "WBTC" => crate::blockchain::BTC_PRICE_USD,
                                            _ => 1.0,
                                        };
                                        let amount_out_2_usd = amount_out_2 * base_usd_price;
                                        let gross_profit_usd = amount_out_2_usd - borrow_amount;
                                        let net_profit = gross_profit_usd - total_cost;

                                        log::debug!(
                                            "{}/{}: borrowed ${} ({:.6} {}) → {} on {} (got {:.4}) → {} on {} (got {:.4} = ${:.2}) | gross: ${:.4} | impact: {:.1}/{:.1}",
                                            token_in_sym, token_out_sym, borrow_amount, base_token_amount, base_sym,
                                            token_out_sym, dex_from.name, amount_out_1,
                                            token_in_sym, dex_to.name, amount_out_2, amount_out_2_usd,
                                            gross_profit_usd, buy_impact, sell_impact
                                        );

                                        if net_profit > min_profit {
                                            let total_impact = buy_impact as u32 + sell_impact as u32;
                                            opps.push(Opportunity {
                                                token_pair: format!("{}/{}", token_in_sym, token_out_sym),
                                                dex_from: dex_from.name.clone(),
                                                dex_to: dex_to.name.clone(),
                                                amount: borrow_amount,
                                                expected_profit: gross_profit_usd,
                                                price_impact_bps: total_impact,
                                                slippage_bps: total_impact / 2,
                                                net_profit_after_costs: net_profit,
                                                route_description: format!(
                                                    "Borrow ${} {} ({:.6}) → Buy {} on {} (got {:.4}) → Sell on {} (got {:.4} = ${:.2}) → Repay",
                                                    borrow_amount, base_sym, base_token_amount,
                                                    token_out_sym, dex_from.name, amount_out_1,
                                                    dex_to.name, amount_out_2, amount_out_2_usd
                                                ),
                                                token_in_address: token_in_addr.clone(),
                                                token_out_address: token_out_addr.clone(),
                                                dex_from_router: dex_from.router_address.clone(),
                                                dex_to_router: dex_to.router_address.clone(),
                                                fee_tier: dex_from.fee_tier,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }

                    opps
                }));
            }
        }

        let results = join_all(futures).await;
        for opps in results {
            all_opportunities.extend(opps);
        }

        all_opportunities.sort_by(|a, b| {
            b.net_profit_after_costs
                .partial_cmp(&a.net_profit_after_costs)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !all_opportunities.is_empty() {
            log::info!(
                "scan_all: {} opportunities, best: {} profit ${:.4} ({})",
                all_opportunities.len(),
                all_opportunities[0].token_pair,
                all_opportunities[0].net_profit_after_costs,
                all_opportunities[0].route_description,
            );
        }

        all_opportunities
    }

    pub async fn execute_trade(
        &self,
        opportunity: &Opportunity,
    ) -> Result<crate::blockchain::TradeResult, String> {
        let amount_wei = U256::from((opportunity.amount * 1e6) as u64);

        let min_amount_out = U256::from(
            (opportunity.amount
                * (1.0 - opportunity.slippage_bps as f64 / 10000.0)
                * 1e6) as u64,
        );

        let legs = vec![SwapLeg {
            token_in: H160::from_str(&opportunity.token_out_address)
                .map_err(|e| format!("Invalid address: {}", e))?,
            token_out: H160::from_str(&opportunity.token_in_address)
                .map_err(|e| format!("Invalid address: {}", e))?,
            amount_in: amount_wei,
            min_amount_out,
            dex_router: H160::from_str(&opportunity.dex_from_router)
                .map_err(|e| format!("Invalid router address: {}", e))?,
        }];

        self.client
            .execute_flash_loan(
                &self.config.contract_address,
                &opportunity.token_out_address,
                amount_wei,
                legs,
            )
            .await
    }

    pub fn get_token_config(&self, address: &str) -> Option<&TokenInfo> {
        self.config
            .tokens
            .iter()
            .find(|t| t.address.eq_ignore_ascii_case(address))
    }
}
