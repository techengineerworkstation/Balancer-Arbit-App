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
    TraderJoeV2,
    ZyberV3,
    RamsesV3,
    SushiSwapV3,
    Ambient,
    AerodromeV2,
    BaseSwapV2,
    VelodromeV2,
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
    #[serde(default)]
    pub init_code_hash: Option<String>,
    #[serde(default)]
    pub quoter_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub name: String,
    pub chain_id: u64,
    pub rpc_url: String,
    pub weth_address: String,
    pub native_symbol: String,
    pub native_price_usd: f64,
    pub tokens: Vec<TokenInfo>,
    pub dexes: Vec<DexInfo>,
    pub base_pairs: Vec<String>,
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
    #[serde(default)]
    pub networks: Vec<NetworkConfig>,
    #[serde(default)]
    pub active_network: String,
}

fn default_tokens() -> Vec<TokenInfo> {
    vec![
        TokenInfo {
            symbol: "AAVE".to_string(),
            address: "0xba5ddd1f9d7F570dc94a51479a000e3bce967196".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "SUSHI".to_string(),
            address: "0xd4d42f0b6def4ce0383636770ef773390d85c61a".to_string(),
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
            address: "0x18c11FD286C5EC11c3b683Caa813B77f5163A122".to_string(),
            decimals: 18,
            is_base: false,
        },
        TokenInfo {
            symbol: "BAL".to_string(),
            address: "0x040d1EdC9569d4Bab2D15287Dc5A4F10F56a56B8".to_string(),
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
            address: "0x6C2C06790b3E3E3c38e12Ee22F8183b37a13EE55".to_string(),
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
            address: "0x0C4681e6C0235179ec3D4F4fc4DF3d14FDD96017".to_string(),
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
            init_code_hash: Some("0xe18a34eb0e04b04f7a0ac29a6e80748dca96319b42c54d679cb821dca90c6303".to_string()),
            quoter_address: None,
        },
        DexInfo {
            name: "PancakeSwap V3".to_string(),
            dex_type: DexType::PancakeV3,
            router_address: "0x13f4EA83D0bd40E0A6C33c274740244243D0FC24".to_string(),
            factory_address: "0x0BFbCF9fa4f9C56B0F40a671Ad40E38852d245B0".to_string(),
            fee_tier: Some(500),
            init_code_hash: None,
            quoter_address: Some("0xB048Bbc1Ee6b733FFfCFb9e9CeF7375518e25997".to_string()),
        },
        DexInfo {
            name: "Uniswap V3".to_string(),
            dex_type: DexType::UniswapV3,
            router_address: "0xE592427A0AEce92De3Edee1F18E0157C05861564".to_string(),
            factory_address: "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string(),
            fee_tier: Some(3000),
            init_code_hash: None,
            quoter_address: Some("0x61fFE014bA17989E743c5F6cB21bF9697530B21e".to_string()),
        },
        DexInfo {
            name: "Curve".to_string(),
            dex_type: DexType::Curve,
            router_address: "0x2191718CD32d02B8E60BAdFFeA33E4B5DD9A0A0D".to_string(),
            factory_address: "0x9AF14D26075f142eb3F292D5065EB3faa646167b".to_string(),
            fee_tier: None,
            init_code_hash: None,
            quoter_address: None,
        },
        DexInfo {
            name: "Camelot V2".to_string(),
            dex_type: DexType::CamelotV2,
            router_address: "0xc873fEcbd354f5A56E00E710B90EF4201db2448d".to_string(),
            factory_address: "0x6EcCab422D763aC031210895C81787E87B43A652".to_string(),
            fee_tier: None,
            init_code_hash: Some("0xa856464ae65f7619087bc369daaf7e387dae1e5af69cfa7935850ebf754b04c1".to_string()),
            quoter_address: None,
        },
        DexInfo {
            name: "Camelot V4".to_string(),
            dex_type: DexType::CamelotV4,
            router_address: "0x4ee15342d6Deb297c3A2aA7CFFd451f788675F53".to_string(),
            factory_address: "0xBefC4b405041c5833f53412fF997ed2f697a2f37".to_string(),
            fee_tier: Some(500),
            init_code_hash: None,
            quoter_address: None,
        },
        DexInfo {
            name: "Trader Joe V2.1".to_string(),
            dex_type: DexType::TraderJoeV2,
            router_address: "0xb4315e873dBcf96Ffd0acd8EA43f689D8c20fB30".to_string(),
            factory_address: "0x8e42f2F4101563bF679975178e880FD87d3eFd4e".to_string(),
            fee_tier: None,
            init_code_hash: None,
            quoter_address: None,
        },
        DexInfo {
            name: "ZyberSwap V3".to_string(),
            dex_type: DexType::ZyberV3,
            router_address: "0xFa58b8024B49836772180f2Df902f231ba712F72".to_string(),
            factory_address: "0x9C2ABD632771b433E5E7507BcaA41cA3b25D8544".to_string(),
            fee_tier: Some(3000),
            init_code_hash: None,
            quoter_address: Some("0xAeD211346Fa2E6A5063b4f273BCf7DDbD0368d62".to_string()),
        },
        DexInfo {
            name: "Ramses V3".to_string(),
            dex_type: DexType::RamsesV3,
            router_address: "0x76D91074B46fF76E04FE59a90526a40009943fd2".to_string(),
            factory_address: "0x07E60782535752be279929e2DFfDd136Db2e6b45".to_string(),
            fee_tier: Some(3000),
            init_code_hash: Some("0x892f127ed4b26ca352056c8fb54585a3268f76f97fdd84d5836ef4bda8d8c685".to_string()),
            quoter_address: Some("0x403Bf94fe505cA0F0b1563C350B57dCeC8303ECd".to_string()),
        },
        DexInfo {
            name: "SushiSwap V3".to_string(),
            dex_type: DexType::SushiSwapV3,
            router_address: "0xE3BFF5cD5D85749a3C7164C66bB5a7d67b04a1aC".to_string(),
            factory_address: "0x1af415a1EbA07a4986a52B6f2e7dE7003D82231e".to_string(),
            fee_tier: Some(3000),
            init_code_hash: None,
            quoter_address: Some("0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54".to_string()),
        },
        DexInfo {
            name: "Ambient".to_string(),
            dex_type: DexType::Ambient,
            router_address: "0x533E164ded63f4c55E83E1f409BDf2BaC5278035".to_string(),
            factory_address: "0xAaAaAAAaA24eEeb8d57D431224f73832bC34f688".to_string(),
            fee_tier: None,
            init_code_hash: None,
            quoter_address: None,
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

fn default_networks() -> Vec<NetworkConfig> {
    vec![
        // Arbitrum One
        NetworkConfig {
            name: "Arbitrum One".to_string(),
            chain_id: 42161,
            rpc_url: "https://arb1.arbitrum.io/rpc".to_string(),
            weth_address: "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".to_string(),
            native_symbol: "ETH".to_string(),
            native_price_usd: 2500.0,
            tokens: vec![
                TokenInfo { symbol: "WETH".to_string(), address: "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".to_string(), decimals: 18, is_base: true },
                TokenInfo { symbol: "USDC".to_string(), address: "0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string(), decimals: 6, is_base: true },
                TokenInfo { symbol: "USDT".to_string(), address: "0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9".to_string(), decimals: 6, is_base: true },
                TokenInfo { symbol: "WBTC".to_string(), address: "0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f".to_string(), decimals: 8, is_base: true },
                TokenInfo { symbol: "ARB".to_string(), address: "0x912CE59144191C1204E64559FE8253a0e49E6548".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "GMX".to_string(), address: "0xfc5A1A6EB076a2C7aD06eD22C90d7E710E35ad0a".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "PENDLE".to_string(), address: "0x0c880f6761F1af8d9Aa9C466984b80DAb9a8c9e8".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "LINK".to_string(), address: "0xf97f4df75117a78c1A5a0DBb814Af92458539FB4".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "UNI".to_string(), address: "0xFa7F8980b0f1E64A2062791cc3b0871572f1F7f0".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "AAVE".to_string(), address: "0xba5ddd1f9d7F570dc94a51479a000e3bce967196".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "SUSHI".to_string(), address: "0xd4d42f0b6def4ce0383636770ef773390d85c61a".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "GNS".to_string(), address: "0x18c11FD286C5EC11c3b683Caa813B77f5163A122".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "BAL".to_string(), address: "0x040d1EdC9569d4Bab2D15287Dc5A4F10F56a56B8".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "MAGIC".to_string(), address: "0x539bdE0d7Dbd336b79148AA742883198BBF60342".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "GRAIL".to_string(), address: "0x3d9907F9a368ad0a51Be60f7Da3b97cf940982D8".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "DPEX".to_string(), address: "0x6C2C06790b3E3E3c38e12Ee22F8183b37a13EE55".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "RDNT".to_string(), address: "0x0C4681e6C0235179ec3D4F4fc4DF3d14FDD96017".to_string(), decimals: 18, is_base: false },
            ],
            dexes: default_dexes(),
            base_pairs: vec!["USDC".to_string(), "WETH".to_string(), "WBTC".to_string(), "USDT".to_string()],
        },
        // Base
        NetworkConfig {
            name: "Base".to_string(),
            chain_id: 8453,
            rpc_url: "https://mainnet.base.org".to_string(),
            weth_address: "0x4200000000000000000000000000000000000006".to_string(),
            native_symbol: "ETH".to_string(),
            native_price_usd: 2500.0,
            tokens: vec![
                TokenInfo { symbol: "WETH".to_string(), address: "0x4200000000000000000000000000000000000006".to_string(), decimals: 18, is_base: true },
                TokenInfo { symbol: "USDC".to_string(), address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".to_string(), decimals: 6, is_base: true },
                TokenInfo { symbol: "DAI".to_string(), address: "0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb".to_string(), decimals: 18, is_base: true },
                TokenInfo { symbol: "cbETH".to_string(), address: "0x2Ae3F1Ec7F1F5012CFEab0185bfc7aa3cf0DEc22".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "wstETH".to_string(), address: "0xc1CBa3fCea344f92D9239c08C0568f6F2F0ee452".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "AERO".to_string(), address: "0x940181a94A35A4569E4529A3CDfB74e38FD98631".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "VIRTUAL".to_string(), address: "0x0b3e3c8e9f63630d9576150d059736f6c93592e6".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "DEGEN".to_string(), address: "0x4ed4E862860beD51a9570b96d89aF5E1B0Efefed".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "BRETT".to_string(), address: "0x532f27101965dd16442E59d40670FaF5eBB142E4".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "ANDY".to_string(), address: "0xf5Cb8DBA2a7BE282b268520F3B0c41C4d5A1d3c3".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "MORPHO".to_string(), address: "0xbaa0FDFc42031DAcE800384dD3C9C4A0C10c6D57".to_string(), decimals: 18, is_base: false },
            ],
            dexes: vec![
                DexInfo {
                    name: "Uniswap V3".to_string(),
                    dex_type: DexType::UniswapV3,
                    router_address: "0x2626664c2603336E57B271c5C0b26F421741e481".to_string(),
                    factory_address: "0x33128a8fC17869897dcE68Ed026d694621f6FDfD".to_string(),
                    fee_tier: Some(3000),
                    init_code_hash: None,
                    quoter_address: Some("0x3d4e44Eb1374240CE5F1B871ab261CD16335B76a".to_string()),
                },
                DexInfo {
                    name: "Aerodrome V2".to_string(),
                    dex_type: DexType::AerodromeV2,
                    router_address: "0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43".to_string(),
                    factory_address: "0x420DD381b31aEf6683db6B902084cB0FFECe40Da".to_string(),
                    fee_tier: None,
                    init_code_hash: None,
                    quoter_address: None,
                },
                DexInfo {
                    name: "BaseSwap V2".to_string(),
                    dex_type: DexType::BaseSwapV2,
                    router_address: "0x327Df1E6de251d1327b645ea38278831f6aBeD33".to_string(),
                    factory_address: "0xfDa135A83F25E62882C7a5A6C4e1cC7c1e84d4C3".to_string(),
                    fee_tier: None,
                    init_code_hash: None,
                    quoter_address: None,
                },
            ],
            base_pairs: vec!["USDC".to_string(), "WETH".to_string(), "DAI".to_string()],
        },
        // Optimism
        NetworkConfig {
            name: "Optimism".to_string(),
            chain_id: 10,
            rpc_url: "https://mainnet.optimism.io".to_string(),
            weth_address: "0x4200000000000000000000000000000000000006".to_string(),
            native_symbol: "ETH".to_string(),
            native_price_usd: 2500.0,
            tokens: vec![
                TokenInfo { symbol: "WETH".to_string(), address: "0x4200000000000000000000000000000000000006".to_string(), decimals: 18, is_base: true },
                TokenInfo { symbol: "USDC".to_string(), address: "0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85".to_string(), decimals: 6, is_base: true },
                TokenInfo { symbol: "USDT".to_string(), address: "0x94b008aA00579c1307B0EF2c499aD98a8ce58e58".to_string(), decimals: 6, is_base: true },
                TokenInfo { symbol: "DAI".to_string(), address: "0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1".to_string(), decimals: 18, is_base: true },
                TokenInfo { symbol: "OP".to_string(), address: "0x4200000000000000000000000000000000000042".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "VELO".to_string(), address: "0x9560e827aF36c94D2Ac33a39bCE1Fe78631088Db".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "wstETH".to_string(), address: "0x1F32b1c2345538c0c6f582fCB022739c4A194Ebb".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "SNX".to_string(), address: "0x8700dAec35aF8Ff88c16BdF0418774CB3D7599B4".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "AAVE".to_string(), address: "0x76FB31fb4af56892A25e32cFC43De717950c9278".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "LINK".to_string(), address: "0x350a791Bfc6C61f2c36F2E10bc31c720766892cE".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "UNI".to_string(), address: "0x6fd9d7AD17242c41f7131d257212c54A0e816691".to_string(), decimals: 18, is_base: false },
                TokenInfo { symbol: "PERP".to_string(), address: "0x9e1028F5F1D5eDE59748FFceE5532509976840E0".to_string(), decimals: 18, is_base: false },
            ],
            dexes: vec![
                DexInfo {
                    name: "Uniswap V3".to_string(),
                    dex_type: DexType::UniswapV3,
                    router_address: "0xE592427A0AEce92De3Edee1F18E0157C05861564".to_string(),
                    factory_address: "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string(),
                    fee_tier: Some(3000),
                    init_code_hash: None,
                    quoter_address: Some("0x61fFE014bA17989E743c5F6cB21bF9697530B21e".to_string()),
                },
                DexInfo {
                    name: "Velodrome V2".to_string(),
                    dex_type: DexType::VelodromeV2,
                    router_address: "0xa062aE8A9c5e11aaA026fc2670B0D65cCc8B2858".to_string(),
                    factory_address: "0xF1046053aa5682b4F9a81b5481394DA16BE5FF5a".to_string(),
                    fee_tier: None,
                    init_code_hash: None,
                    quoter_address: None,
                },
                DexInfo {
                    name: "SushiSwap V2".to_string(),
                    dex_type: DexType::SushiV2,
                    router_address: "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506".to_string(),
                    factory_address: "0xc35DADB65012eC5796536bD9864eD8773aBc74C4".to_string(),
                    fee_tier: None,
                    init_code_hash: Some("0xe18a34eb0e04b04f7a0ac29a6e80748dca96319b42c54d679cb821dca90c6303".to_string()),
                    quoter_address: None,
                },
            ],
            base_pairs: vec!["USDC".to_string(), "WETH".to_string(), "USDT".to_string(), "DAI".to_string()],
        },
    ]
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://arb1.arbitrum.io/rpc".to_string(),
            private_key: String::new(),
            contract_address: String::new(),
            borrow_amount: 10000.0,
            min_profit_usd: 0.005,
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
            networks: default_networks(),
            active_network: "Arbitrum One".to_string(),
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
