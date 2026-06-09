use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod api;
mod components;
mod pages;

use components::*;
use pages::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotStatus {
    pub is_running: bool,
    pub total_trades: u64,
    pub total_profit: f64,
    pub last_trade_time: Option<String>,
    pub gas_spent: f64,
    pub balance: f64,
    pub network: String,
    pub avg_slippage_bps: f64,
    pub avg_price_impact_bps: f64,
    pub simulate_before_send: bool,
    pub use_direct_pool_calls: bool,
}

impl Default for BotStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            total_trades: 0,
            total_profit: 0.0,
            last_trade_time: None,
            gas_spent: 0.0,
            balance: 0.0,
            network: "Polygon".to_string(),
            avg_slippage_bps: 0.0,
            avg_price_impact_bps: 0.0,
            simulate_before_send: true,
            use_direct_pool_calls: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeLog {
    pub id: u64,
    pub timestamp: String,
    pub token_pair: String,
    pub dex_from: String,
    pub dex_to: String,
    pub amount: f64,
    pub profit: f64,
    pub gas_cost: f64,
    pub status: String,
    pub slippage_bps: u32,
    pub price_impact_bps: u32,
    pub net_profit_after_costs: f64,
    pub simulated: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_url: "https://polygon-rpc.com".to_string(),
            private_key: String::new(),
            contract_address: String::new(),
            borrow_amount: 10000.0,
            min_profit_usd: 5.0,
            max_gas_price_gwei: 100.0,
            scan_interval_ms: 1000,
            auto_trade: false,
            daily_start_hour: 0,
            daily_end_hour: 23,
            max_slippage_bps: 50,
            max_price_impact_pct: 1.0,
            tenderly_api_key: String::new(),
            tenderly_project_slug: String::new(),
            simulate_before_send: true,
            use_direct_pool_calls: true,
            fee_tiers: vec![100, 500, 2500, 10000],
            reverse_route: false,
            min_borrow_amount: 100.0,
            max_borrow_amount: 100000.0,
            borrow_step: 100.0,
        }
    }
}

#[component]
fn App() -> impl IntoView {
    provide_context(create_signal(BotStatus::default()));
    provide_context(create_signal(Config::default()));
    provide_context(create_signal(Vec::<TradeLog>::new()));

    view! {
        <Router>
            <main class="app-container">
                <NavBar/>
                <Routes>
                    <Route path="/" view=Dashboard/>
                    <Route path="/config" view=ConfigPage/>
                    <Route path="/trades" view=TradeHistory/>
                    <Route path="/contract" view=ContractDeploy/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    leptos::mount::mount_to_body(App);
}