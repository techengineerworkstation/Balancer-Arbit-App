use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DexType {
    SushiV2,
    PancakeV3,
    UniswapV3,
    Curve,
    CamelotV2,
    CamelotV4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub symbol: String,
    pub address: String,
    pub decimals: u32,
    pub is_base: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexInfo {
    pub name: String,
    pub dex_type: DexType,
    pub router_address: String,
    pub factory_address: String,
    pub fee_tier: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub rpc_url: String,
    pub private_key: String,
    pub contract_address: String,
    pub borrow_amount: f64,
    pub min_profit_usd: f64,
    pub max_gas_price_gwei: f64,
    pub scan_interval_ms: u64,
    pub auto_trade: bool,
    pub daily_start_hour: u32,
    pub daily_end_hour: u32,
    pub max_slippage_bps: u32,
    pub max_price_impact_pct: f64,
    pub tenderly_api_key: String,
    pub tenderly_project_slug: String,
    pub simulate_before_send: bool,
    pub use_direct_pool_calls: bool,
    pub fee_tiers: Vec<u32>,
    pub reverse_route: bool,
    pub min_borrow_amount: f64,
    pub max_borrow_amount: f64,
    pub borrow_step: f64,
    #[serde(default = "default_tokens")]
    pub tokens: Vec<TokenInfo>,
    #[serde(default = "default_dexes")]
    pub dexes: Vec<DexInfo>,
    #[serde(default = "default_base_pairs")]
    pub base_pairs: Vec<String>,
    pub quote_tokens: Vec<String>,
    pub network_name: String,
    pub chain_id: u64,
}

fn default_tokens() -> Vec<TokenInfo> {
    vec![
        TokenInfo {
            symbol: "AAVE".to_string(),
            address: "0xba5DdD1f9d7F3bE546e97a4C2D7b34c26c435bF9".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "SUSHI".to_string(),
            address: "0xd4d42Fca40609F09a632F85185db06415470AD69".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "LINK".to_string(),
            address: "0xf97f4df75117a78c1A5a0dbb814af92458539FB4".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "GNS".to_string(),
            address: "0x18c11FD286C5EC11c3b683Caa813B93f77155Ac9".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "BAL".to_string(),
            address: "0x040d1EdC9461295F50CBDb630f6C3cAb6ab31104".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "MAGIC".to_string(),
            address: "0x539bdE0d7Dbd336b79148AA742883198BBF60342".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "PENDLE".to_string(),
            address: "0x0c880f6761F1af8d9Aa9C466984b80DAb9a8c9e8".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "WETH".to_string(),
            address: "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".to_string(),
            decimals: 18,
            is_base: true,
        },
        TokenInfo {
            symbol: "USDC".to_string(),
            address: "0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string(),
            decimals: 6,
            is_base: true,
        },
        TokenInfo {
            symbol: "USDT".to_string(),
            address: "0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9".to_string(),
            decimals: 6,
            is_base: true,
        },
        TokenInfo {
            symbol: "WBTC".to_string(),
            address: "0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f".to_string(),
            decimals: 8,
            is_base: true,
        },
        TokenInfo {
            symbol: "GRAIL".to_string(),
            address: "0x3d9907F9a368ad0a51Be60f7Da3b97cf940982D8".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "DPEX".to_string(),
            address: "0x6C2C2649d712c27D7405D35d9aD6b1C2233cBbae".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "UNI".to_string(),
            address: "0xFa7F8980b0f1E64A2062791cc3b0871572f1F7f0".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "ARB".to_string(),
            address: "0x912CE59144191C1204E64559FE8253a0e49E6548".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "RDNT".to_string(),
            address: "0x3082CC23568eA640225c2467663441610403D183".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "GMX".to_string(),
            address: "0xfc5A1A6EB076a2C7aD06eD22C90d7E710E35ad0a".to_string(),
            decimals: 18,
            is_base: false,
        },
    ]
}

fn default_dexes() -> Vec<DexInfo> {
    vec![
        DexInfo {
            name: "SushiSwap V2".to_string(),
            dex_type: DexType::SushiV2,
            router_address: "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506".to_string(),
            factory_address: "0xc35DADB65012eC5796536bD9864eD8773aBc74C4".to_string(),
            fee_tier: None,
        },
        DexInfo {
            name: "PancakeSwap V3".to_string(),
            dex_type: DexType::PancakeV3,
            router_address: "0x13f4EA83D0bd40E0A6C33c274740244243D0FC24".to_string(),
            factory_address: "0x0BFbCF9fa4f9C56B0F40a671Ad40E38852d245B0".to_string(),
            fee_tier: Some(500),
        },
        DexInfo {
            name: "Uniswap V3".to_string(),
            dex_type: DexType::UniswapV3,
            router_address: "0xE592427A0AEce92De3Edee1F18E0157C05861564".to_string(),
            factory_address: "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string(),
            fee_tier: Some(3000),
        },
        DexInfo {
            name: "Curve".to_string(),
            dex_type: DexType::Curve,
            router_address: "0x2191718CD32d02B8E60BAdFFeA33E4B5DD9A0A0D".to_string(),
            factory_address: "0x9AF14D26075f142eb3F292D5065EB3faa646167b".to_string(),
            fee_tier: None,
        },
        DexInfo {
            name: "Camelot V2".to_string(),
            dex_type: DexType::CamelotV2,
            router_address: "0xc873fEcbd354f5A56E00E710B90EF4201db2448d".to_string(),
            factory_address: "0x6EcCab422D763aC031210895C81787E87B43A652".to_string(),
            fee_tier: None,
        },
        DexInfo {
            name: "Camelot V4".to_string(),
            dex_type: DexType::CamelotV4,
            router_address: "0x4ee15342d6Deb297c3A2aA7CFFd451f788675F53".to_string(),
            factory_address: "0xBefC4b405041c5833f53412fF997ed2f697a2f37".to_string(),
            fee_tier: Some(500),
        },
    ]
}

fn default_base_pairs() -> Vec<String> {
    vec![
        "USDC".to_string(),
        "WETH".to_string(),
        "WBTC".to_string(),
        "USDT".to_string(),
    ]
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://arb1.arbitrum.io/rpc".to_string(),
            private_key: String::new(),
            contract_address: String::new(),
            borrow_amount: 10000.0,
            min_profit_usd: 1.0,
            max_gas_price_gwei: 1.0,
            scan_interval_ms: 100,
            auto_trade: false,
            daily_start_hour: 0,
            daily_end_hour: 23,
            max_slippage_bps: 50,
            max_price_impact_pct: 1.0,
            tenderly_api_key: String::new(),
            tenderly_project_slug: String::new(),
            simulate_before_send: true,
            use_direct_pool_calls: true,
            fee_tiers: vec![100, 500, 3000, 10000],
            reverse_route: false,
            min_borrow_amount: 100.0,
            max_borrow_amount: 500000.0,
            borrow_step: 100.0,
            tokens: default_tokens(),
            dexes: default_dexes(),
            base_pairs: default_base_pairs(),
            quote_tokens: vec!["USDC".to_string(), "USDT".to_string()],
            network_name: "Arbitrum One".to_string(),
            chain_id: 42161,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        let path = "config.json";
        if Path::new(path).exists() {
            match fs::read_to_string(path) {
                Ok(content) => {
                    match serde_json::from_str::<AppConfig>(&content) {
                        Ok(mut config) => {
                            if config.tokens.is_empty() {
                                config.tokens = default_tokens();
                            }
                            if config.dexes.is_empty() {
                                config.dexes = default_dexes();
                            }
                            if config.base_pairs.is_empty() {
                                config.base_pairs = default_base_pairs();
                            }
                            config.validate_borrow_amount();
                            config
                        }
                        Err(e) => {
                            log::warn!("Failed to parse config.json: {}. Using defaults.", e);
                            let config = AppConfig::default();
                            config.save();
                            config
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to read config.json: {}. Using defaults.", e);
                    let config = AppConfig::default();
                    config.save();
                    config
                }
            }
        } else {
            log::info!("No config.json found, creating default config");
            let config = AppConfig::default();
            config.save();
            config
        }
    }

    pub fn save(&self) {
        let path = "config.json";
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = fs::write(path, json) {
                    log::error!("Failed to save config.json: {}", e);
                }
            }
            Err(e) => {
                log::error!("Failed to serialize config: {}", e);
            }
        }
    }

    pub fn validate_borrow_amount(&mut self) {
        if self.borrow_amount < self.min_borrow_amount {
            self.borrow_amount = self.min_borrow_amount;
        }
        if self.borrow_amount > self.max_borrow_amount {
            self.borrow_amount = self.max_borrow_amount;
        }
        if self.min_borrow_amount >= self.max_borrow_amount {
            self.min_borrow_amount = 100.0;
            self.max_borrow_amount = 500000.0;
        }
    }

    pub fn get_token(&self, symbol: &str) -> Option<&TokenInfo> {
        self.tokens.iter().find(|t| t.symbol == symbol)
    }

    pub fn get_token_by_address(&self, address: &str) -> Option<&TokenInfo> {
        self.tokens
            .iter()
            .find(|t| t.address.eq_ignore_ascii_case(address))
    }

    pub fn get_dex(&self, name: &str) -> Option<&DexInfo> {
        self.dexes.iter().find(|d| d.name == name)
    }
}
