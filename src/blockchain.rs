use crate::config::DexType;
use ethers::abi::{Abi, Token as AbiToken};
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

pub const VAULT_ADDRESS: &str = "0xba1333333333a1BA1108E8412f11850A5C319bA9";
pub const ETH_PRICE_USD: f64 = 2500.0;
pub const BTC_PRICE_USD: f64 = 100_000.0;

pub const V3_VAULT_ABI: &str = r#"[
    {
        "inputs": [
            { "internalType": "bytes", "name": "data", "type": "bytes" }
        ],
        "name": "unlock",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            { "internalType": "contract IERC20", "name": "token", "type": "address" },
            { "internalType": "address", "name": "to", "type": "address" },
            { "internalType": "uint256", "name": "amount", "type": "uint256" }
        ],
        "name": "sendTo",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            { "internalType": "contract IERC20", "name": "token", "type": "address" },
            { "internalType": "uint256", "name": "amount", "type": "uint256" }
        ],
        "name": "settle",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    }
]"#;

pub const EXECUTE_FLASH_LOAN_ABI: &str = r#"[
    {
        "inputs": [
            { "internalType": "address", "name": "token", "type": "address" },
            { "internalType": "uint256", "name": "amount", "type": "uint256" },
            {
                "components": [
                    { "internalType": "address", "name": "tokenIn", "type": "address" },
                    { "internalType": "address", "name": "tokenOut", "type": "address" },
                    { "internalType": "uint256", "name": "amountIn", "type": "uint256" },
                    { "internalType": "uint256", "name": "minAmountOut", "type": "uint256" },
                    { "internalType": "address", "name": "dexRouter", "type": "address" }
                ],
                "internalType": "struct SwapLeg[]",
                "name": "legs",
                "type": "tuple[]"
            },
            { "internalType": "uint256", "name": "deadline", "type": "uint256" }
        ],
        "name": "executeFlashLoan",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    }
]"#;

pub const IERC20_ABI: &str = r#"[
    {
        "inputs": [
            { "internalType": "address", "name": "account", "type": "address" }
        ],
        "name": "balanceOf",
        "outputs": [
            { "internalType": "uint256", "name": "", "type": "uint256" }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            { "internalType": "address", "name": "recipient", "type": "address" },
            { "internalType": "uint256", "name": "amount", "type": "uint256" }
        ],
        "name": "transfer",
        "outputs": [
            { "internalType": "bool", "name": "", "type": "bool" }
        ],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "decimals",
        "outputs": [
            { "internalType": "uint8", "name": "", "type": "uint8" }
        ],
        "stateMutability": "view",
        "type": "function"
    }
]"#;

pub const ARBITRUM_GAS_PRICE_GWEI: f64 = 0.02;
pub const ARBITRUM_L1_DATA_FEE_USD: f64 = 0.05;
pub const GAS_PER_SWAP: u64 = 500000;

fn parse_abi(json: &str) -> Abi {
    serde_json::from_str(json).expect("Invalid ABI JSON")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    pub success: bool,
    pub profit: f64,
    pub gas_cost: f64,
    pub actual_slippage_bps: u32,
    pub simulated: bool,
    pub tx_hash: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SwapLeg {
    pub token_in: H160,
    pub token_out: H160,
    pub amount_in: U256,
    pub min_amount_out: U256,
    pub dex_router: H160,
}

pub struct BlockchainClient {
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
    chain_id: u64,
}

impl BlockchainClient {
    pub async fn new(rpc_url: &str, private_key: &str, chain_id: u64) -> Result<Self, String> {
        let provider = Arc::new(
            Provider::<Http>::try_from(rpc_url)
                .map_err(|e| format!("Failed to create provider: {}", e))?,
        );

        let wallet = private_key
            .parse::<LocalWallet>()
            .map_err(|e| format!("Failed to parse private key: {}", e))?
            .with_chain_id(chain_id);

        Ok(Self {
            provider,
            wallet,
            chain_id,
        })
    }

    pub async fn new_readonly(rpc_url: &str, chain_id: u64) -> Result<Self, String> {
        let provider = Arc::new(
            Provider::<Http>::try_from(rpc_url)
                .map_err(|e| format!("Failed to create provider: {}", e))?,
        );

        let wallet = LocalWallet::new(&mut ethers::core::rand::thread_rng()).with_chain_id(chain_id);

        Ok(Self {
            provider,
            wallet,
            chain_id,
        })
    }

    pub fn address(&self) -> H160 {
        self.wallet.address()
    }

    pub fn provider(&self) -> &Arc<Provider<Http>> {
        &self.provider
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn signer(&self) -> SignerMiddleware<Arc<Provider<Http>>, LocalWallet> {
        SignerMiddleware::new(self.provider.clone(), self.wallet.clone())
    }

    pub async fn get_gas_price(&self) -> Result<f64, String> {
        let gas_price = self
            .provider
            .get_gas_price()
            .await
            .map_err(|e| format!("Failed to get gas price: {}", e))?;
        Ok(gas_price.as_u64() as f64 / 1e9)
    }

    pub async fn get_eth_balance(&self) -> Result<f64, String> {
        let balance = self
            .provider
            .get_balance(self.wallet.address(), None)
            .await
            .map_err(|e| format!("Failed to get ETH balance: {}", e))?;
        Ok(format!("{}", balance).parse::<f64>().unwrap_or(0.0) / 1e18)
    }

    pub async fn get_token_balance(
        &self,
        token_address: &str,
        account: H160,
    ) -> Result<U256, String> {
        let address = H160::from_str(token_address)
            .map_err(|e| format!("Invalid token address: {}", e))?;
        let contract = Contract::new(address, parse_abi(IERC20_ABI), self.provider.clone());
        let balance: U256 = contract
            .method("balanceOf", account)
            .map_err(|e| format!("Failed to build balanceOf call: {}", e))?
            .call()
            .await
            .map_err(|e| format!("Failed to get token balance: {}", e))?;
        Ok(balance)
    }

    pub async fn get_contract_balance(
        &self,
        token_address: &str,
        contract_address: &str,
    ) -> Result<f64, String> {
        let contract_addr = H160::from_str(contract_address)
            .map_err(|e| format!("Invalid contract address: {}", e))?;
        let balance = self
            .get_token_balance(token_address, contract_addr)
            .await?;

        let token_addr = H160::from_str(token_address)
            .map_err(|e| format!("Invalid token address: {}", e))?;
        let contract = Contract::new(token_addr, parse_abi(IERC20_ABI), self.provider.clone());
        let decimals: u8 = contract
            .method("decimals", ())
            .map_err(|e| format!("Failed to build decimals call: {}", e))?
            .call()
            .await
            .map_err(|e| format!("Failed to get decimals: {}", e))?;

        let divisor = 10f64.powi(decimals as i32);
        let raw = format!("{}", balance).parse::<f64>().unwrap_or(0.0);
        Ok(raw / divisor)
    }

    pub fn calculate_gas_cost_usd(&self, gas_price_gwei: f64, gas_used: u64) -> f64 {
        let gas_cost_eth = (gas_price_gwei * gas_used as f64) / 1e9;
        gas_cost_eth * ETH_PRICE_USD
    }

    pub fn estimate_swap_cost_usd(&self, gas_price_gwei: f64) -> f64 {
        let gas_cost_eth = (gas_price_gwei * GAS_PER_SWAP as f64) / 1e9;
        gas_cost_eth * ETH_PRICE_USD
    }

    pub fn estimate_total_trade_cost(
        &self,
        gas_price_gwei: f64,
        num_swaps: u32,
    ) -> f64 {
        let swap_cost = self.estimate_swap_cost_usd(gas_price_gwei) * num_swaps as f64;
        let l1_data_fee = ARBITRUM_L1_DATA_FEE_USD * num_swaps as f64;
        let flash_loan_fee = 0.0;
        swap_cost + l1_data_fee + flash_loan_fee
    }

    pub fn estimate_deployment_cost(&self, gas_price_gwei: f64) -> f64 {
        let deploy_gas: u64 = 2_000_000;
        self.calculate_gas_cost_usd(gas_price_gwei, deploy_gas)
    }

    pub fn estimate_gas_cost(&self, gas_price_gwei: f64, gas_limit: u64) -> f64 {
        self.calculate_gas_cost_usd(gas_price_gwei, gas_limit)
    }

    pub fn calculate_slippage(
        &self,
        amount_in_wei: U256,
        reserve_in: U256,
        reserve_out: U256,
        dex_type: &DexType,
    ) -> f64 {
        if reserve_in.is_zero() || reserve_out.is_zero() {
            return 100.0;
        }

        let amount_in_f64 = format!("{}", amount_in_wei).parse::<f64>().unwrap_or(0.0);
        let reserve_in_f64 = format!("{}", reserve_in).parse::<f64>().unwrap_or(1.0);
        let reserve_out_f64 = format!("{}", reserve_out).parse::<f64>().unwrap_or(1.0);

        let trade_size_ratio = amount_in_f64 / reserve_in_f64;

        let impact_pct = match dex_type {
            DexType::SushiV2 | DexType::CamelotV2 => {
                let k = reserve_in_f64 * reserve_out_f64;
                let new_reserve_in = reserve_in_f64 + amount_in_f64;
                let new_reserve_out = k / new_reserve_in;
                let amount_out = reserve_out_f64 - new_reserve_out;
                let ideal_out = amount_in_f64 * (reserve_out_f64 / reserve_in_f64);
                if ideal_out > 0.0 {
                    ((ideal_out - amount_out) / ideal_out * 100.0).abs()
                } else {
                    100.0
                }
            }
            DexType::PancakeV3 | DexType::UniswapV3 | DexType::CamelotV4 => {
                if trade_size_ratio > 0.1 {
                    trade_size_ratio * 100.0
                } else {
                    trade_size_ratio * trade_size_ratio * 1000.0
                }
            }
            DexType::Curve => {
                if trade_size_ratio > 0.05 {
                    trade_size_ratio * 50.0
                } else {
                    trade_size_ratio * trade_size_ratio * 200.0
                }
            }
        };

        impact_pct.min(100.0)
    }

    pub fn build_swap_calldata(
        &self,
        token_in: H160,
        token_out: H160,
        amount_in: U256,
        min_amount_out: U256,
        dex_type: &DexType,
        dex_router: H160,
        fee_tier: Option<u32>,
    ) -> Result<Vec<u8>, String> {
        match dex_type {
            DexType::SushiV2 | DexType::CamelotV2 => {
                self.encode_v2_swap_calldata(
                    token_in,
                    token_out,
                    amount_in,
                    min_amount_out,
                    dex_router,
                )
            }
            DexType::PancakeV3 | DexType::UniswapV3 => {
                let fee = fee_tier.unwrap_or(3000);
                self.encode_v3_swap_calldata(
                    token_in,
                    token_out,
                    amount_in,
                    min_amount_out,
                    dex_router,
                    fee,
                )
            }
            DexType::Curve => {
                self.encode_curve_swap_calldata(
                    token_in,
                    token_out,
                    amount_in,
                    min_amount_out,
                    dex_router,
                )
            }
            DexType::CamelotV4 => {
                let fee = fee_tier.unwrap_or(500);
                self.encode_algebra_swap_calldata(
                    token_in,
                    token_out,
                    amount_in,
                    min_amount_out,
                    dex_router,
                    fee,
                )
            }
        }
    }

    fn encode_v2_swap_calldata(
        &self,
        token_in: H160,
        token_out: H160,
        amount_in: U256,
        min_amount_out: U256,
        _dex_router: H160,
    ) -> Result<Vec<u8>, String> {
        let deadline = U256::from(chrono::Utc::now().timestamp() + 300);

        let path = vec![
            AbiToken::Address(token_in),
            AbiToken::Address(token_out),
        ];

        let mut tokens = Vec::new();
        tokens.push(AbiToken::Uint(amount_in));
        tokens.push(AbiToken::Uint(min_amount_out));
        tokens.push(AbiToken::Array(path));
        tokens.push(AbiToken::Address(self.wallet.address()));
        tokens.push(AbiToken::Uint(deadline));

        let encoded = ethers::abi::encode(&tokens);

        let selector = hex::decode("38ed1739")
            .map_err(|e| format!("Failed to decode selector: {}", e))?;

        let mut calldata = selector;
        calldata.extend_from_slice(&encoded);
        Ok(calldata)
    }

    fn encode_v3_swap_calldata(
        &self,
        token_in: H160,
        token_out: H160,
        amount_in: U256,
        min_amount_out: U256,
        _dex_router: H160,
        fee: u32,
    ) -> Result<Vec<u8>, String> {
        let deadline = U256::from(chrono::Utc::now().timestamp() + 300);

        let mut tuple_tokens = Vec::new();
        tuple_tokens.push(AbiToken::Address(token_in));
        tuple_tokens.push(AbiToken::Address(token_out));
        tuple_tokens.push(AbiToken::Uint(U256::from(fee)));
        tuple_tokens.push(AbiToken::Address(self.wallet.address()));
        tuple_tokens.push(AbiToken::Uint(deadline));
        tuple_tokens.push(AbiToken::Uint(amount_in));
        tuple_tokens.push(AbiToken::Uint(min_amount_out));
        tuple_tokens.push(AbiToken::Uint(U256::zero()));

        let mut tokens = Vec::new();
        tokens.push(AbiToken::Tuple(tuple_tokens));

        let encoded = ethers::abi::encode(&tokens);

        let selector = hex::decode("c04b8d59")
            .map_err(|e| format!("Failed to decode selector: {}", e))?;

        let mut calldata = selector;
        calldata.extend_from_slice(&encoded);
        Ok(calldata)
    }

    fn encode_curve_swap_calldata(
        &self,
        _token_in: H160,
        _token_out: H160,
        amount_in: U256,
        min_amount_out: U256,
        _dex_router: H160,
    ) -> Result<Vec<u8>, String> {
        let mut tokens = Vec::new();
        tokens.push(AbiToken::Uint(U256::zero()));
        tokens.push(AbiToken::Uint(U256::one()));
        tokens.push(AbiToken::Uint(amount_in));
        tokens.push(AbiToken::Uint(min_amount_out));

        let encoded = ethers::abi::encode(&tokens);

        let selector = hex::decode("3df02124")
            .map_err(|e| format!("Failed to decode selector: {}", e))?;

        let mut calldata = selector;
        calldata.extend_from_slice(&encoded);
        Ok(calldata)
    }

    fn encode_algebra_swap_calldata(
        &self,
        token_in: H160,
        token_out: H160,
        amount_in: U256,
        min_amount_out: U256,
        _dex_router: H160,
        fee: u32,
    ) -> Result<Vec<u8>, String> {
        let deadline = U256::from(chrono::Utc::now().timestamp() + 300);

        let mut tuple_tokens = Vec::new();
        tuple_tokens.push(AbiToken::Address(token_in));
        tuple_tokens.push(AbiToken::Address(token_out));
        tuple_tokens.push(AbiToken::Uint(U256::from(fee)));
        tuple_tokens.push(AbiToken::Address(self.wallet.address()));
        tuple_tokens.push(AbiToken::Uint(deadline));
        tuple_tokens.push(AbiToken::Uint(amount_in));
        tuple_tokens.push(AbiToken::Uint(min_amount_out));
        tuple_tokens.push(AbiToken::Uint(U256::zero()));

        let mut tokens = Vec::new();
        tokens.push(AbiToken::Tuple(tuple_tokens));

        let encoded = ethers::abi::encode(&tokens);

        let selector = hex::decode("c04b8d59")
            .map_err(|e| format!("Failed to decode selector: {}", e))?;

        let mut calldata = selector;
        calldata.extend_from_slice(&encoded);
        Ok(calldata)
    }

    pub fn encode_flash_loan_calldata(
        &self,
        token: H160,
        amount: U256,
        legs: &[SwapLeg],
        deadline: U256,
    ) -> Result<Vec<u8>, String> {
        let mut tokens = Vec::new();
        tokens.push(AbiToken::Address(token));
        tokens.push(AbiToken::Uint(amount));

        let mut legs_tokens = Vec::new();
        for leg in legs {
            let mut leg_tuple = Vec::new();
            leg_tuple.push(AbiToken::Address(leg.token_in));
            leg_tuple.push(AbiToken::Address(leg.token_out));
            leg_tuple.push(AbiToken::Uint(leg.amount_in));
            leg_tuple.push(AbiToken::Uint(leg.min_amount_out));
            leg_tuple.push(AbiToken::Address(leg.dex_router));
            legs_tokens.push(AbiToken::Tuple(leg_tuple));
        }

        tokens.push(AbiToken::Array(legs_tokens));
        tokens.push(AbiToken::Uint(deadline));

        let encoded = ethers::abi::encode(&tokens);

        let sig = ethers::utils::id("executeFlashLoan(address,uint256,tuple[],uint256)");
        let function_selector = sig[..4].to_vec();

        let mut calldata = function_selector;
        calldata.extend_from_slice(&encoded);
        Ok(calldata)
    }

    pub fn encode_vault_unlock_calldata(
        &self,
        token: H160,
        amount: U256,
        recipient: H160,
    ) -> Result<Vec<u8>, String> {
        let mut inner_calldata = Vec::new();

        let send_to_sig = ethers::utils::id("sendTo(address,address,uint256)");
        let send_to_selector = send_to_sig[..4].to_vec();

        let mut send_to_tokens = Vec::new();
        send_to_tokens.push(AbiToken::Address(token));
        send_to_tokens.push(AbiToken::Address(recipient));
        send_to_tokens.push(AbiToken::Uint(amount));

        let send_to_encoded = ethers::abi::encode(&send_to_tokens);
        inner_calldata.extend_from_slice(&send_to_selector);
        inner_calldata.extend_from_slice(&send_to_encoded);

        let mut tokens = Vec::new();
        tokens.push(AbiToken::Bytes(inner_calldata));

        let encoded = ethers::abi::encode(&tokens);

        let unlock_sig = ethers::utils::id("unlock(bytes)");
        let unlock_selector = unlock_sig[..4].to_vec();

        let mut calldata = unlock_selector;
        calldata.extend_from_slice(&encoded);
        Ok(calldata)
    }

    pub fn encode_settle_calldata(
        &self,
        token: H160,
        amount: U256,
    ) -> Result<Vec<u8>, String> {
        let mut tokens = Vec::new();
        tokens.push(AbiToken::Address(token));
        tokens.push(AbiToken::Uint(amount));

        let encoded = ethers::abi::encode(&tokens);

        let sig = ethers::utils::id("settle(address,uint256)");
        let selector = sig[..4].to_vec();

        let mut calldata = selector;
        calldata.extend_from_slice(&encoded);
        Ok(calldata)
    }

    pub async fn simulate_swap(
        &self,
        token_in: H160,
        token_out: H160,
        amount_in: U256,
        dex_type: &DexType,
        router_address: H160,
        fee_tier: Option<u32>,
    ) -> Result<U256, String> {
        let calldata = self.build_swap_calldata(
            token_in,
            token_out,
            amount_in,
            U256::zero(),
            dex_type,
            router_address,
            fee_tier,
        )?;

        let mut tx = TypedTransaction::default();
        tx.set_to(NameOrAddress::Address(router_address));
        tx.set_data(calldata.into());

        let result = self
            .provider
            .call(&tx, None)
            .await
            .map_err(|e| format!("Static call failed: {}", e))?;

        if result.len() >= 32 {
            let amount_out = U256::from_big_endian(&result[result.len() - 32..]);
            Ok(amount_out)
        } else {
            Err("Invalid response length".to_string())
        }
    }

    pub async fn execute_flash_loan(
        &self,
        contract_address: &str,
        token: &str,
        amount: U256,
        legs: Vec<SwapLeg>,
    ) -> Result<TradeResult, String> {
        let contract_addr = H160::from_str(contract_address)
            .map_err(|e| format!("Invalid contract address: {}", e))?;
        let token_addr = H160::from_str(token)
            .map_err(|e| format!("Invalid token address: {}", e))?;

        let gas_price = self.get_gas_price().await?;
        let gas_price_wei = U256::from((gas_price * 1e9) as u64);
        let deadline = U256::from(chrono::Utc::now().timestamp() + 60);

        let calldata =
            self.encode_flash_loan_calldata(token_addr, amount, &legs, deadline)?;

        let mut tx = TypedTransaction::default();
        tx.set_to(NameOrAddress::Address(contract_addr));
        tx.set_data(calldata.into());
        tx.set_value(U256::zero());
        tx.set_gas(U256::from(3_000_000));
        tx.set_gas_price(gas_price_wei);

        let signer = self.signer();
        let pending_tx = signer
            .send_transaction(tx, None)
            .await
            .map_err(|e| format!("Failed to send tx: {}", e))?;

        let receipt = pending_tx
            .await
            .map_err(|e| format!("Tx failed: {}", e))?;

        match receipt {
            Some(r) => {
                let gas_cost =
                    self.calculate_gas_cost_usd(gas_price, r.gas_used.unwrap_or_default().as_u64());
                Ok(TradeResult {
                    success: r.status == Some(U64::from(1)),
                    profit: 0.0,
                    gas_cost,
                    actual_slippage_bps: 0,
                    simulated: false,
                    tx_hash: Some(format!("{:?}", r.transaction_hash)),
                    error: if r.status != Some(U64::from(1)) {
                        Some("Transaction reverted".to_string())
                    } else {
                        None
                    },
                })
            }
            None => Ok(TradeResult {
                success: false,
                profit: 0.0,
                gas_cost: 0.0,
                actual_slippage_bps: 0,
                simulated: false,
                tx_hash: None,
                error: Some("No receipt received".to_string()),
            }),
        }
    }

    pub async fn withdraw_profits(
        &self,
        contract_address: &str,
        token: &str,
        _amount: U256,
    ) -> Result<TradeResult, String> {
        let contract_addr = H160::from_str(contract_address)
            .map_err(|e| format!("Invalid contract address: {}", e))?;
        let _token_addr = H160::from_str(token)
            .map_err(|e| format!("Invalid token address: {}", e))?;

        let gas_price = self.get_gas_price().await?;
        let gas_price_wei = U256::from((gas_price * 1e9) as u64);

        let mut tx = TypedTransaction::default();
        tx.set_to(NameOrAddress::Address(contract_addr));
        tx.set_data(Vec::new().into());
        tx.set_value(U256::zero());
        tx.set_gas(U256::from(100_000));
        tx.set_gas_price(gas_price_wei);

        let signer = self.signer();
        let pending_tx = signer
            .send_transaction(tx, None)
            .await
            .map_err(|e| format!("Failed to send withdraw tx: {}", e))?;

        let receipt = pending_tx
            .await
            .map_err(|e| format!("Withdraw tx failed: {}", e))?;

        match receipt {
            Some(r) => {
                let gas_cost =
                    self.calculate_gas_cost_usd(gas_price, r.gas_used.unwrap_or_default().as_u64());
                Ok(TradeResult {
                    success: r.status == Some(U64::from(1)),
                    profit: 0.0,
                    gas_cost,
                    actual_slippage_bps: 0,
                    simulated: false,
                    tx_hash: Some(format!("{:?}", r.transaction_hash)),
                    error: None,
                })
            }
            None => Ok(TradeResult {
                success: false,
                profit: 0.0,
                gas_cost: 0.0,
                actual_slippage_bps: 0,
                simulated: false,
                tx_hash: None,
                error: Some("No receipt".to_string()),
            }),
        }
    }
}
