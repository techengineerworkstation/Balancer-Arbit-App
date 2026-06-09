mod auth;
mod blockchain;
mod config;
mod scanner;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, middleware};
use actix_web::body::MessageBody;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::Error;
use chrono::Timelike;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use tokio::time::{interval, Duration};

use auth::AuthStore;
use blockchain::BlockchainClient;
use config::AppConfig;
use scanner::{BotStatus, Opportunity, Scanner};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeInfo {
    pub gas_price_gwei: f64,
    pub l1_data_fee_usd: f64,
    pub swap_fee_usd: f64,
    pub flash_loan_fee_usd: f64,
    pub deployment_cost_usd: f64,
    pub total_fee_per_trade_usd: f64,
}

#[derive(Clone)]
pub struct AppState {
    pub status: Arc<Mutex<BotStatus>>,
    pub config: Arc<Mutex<AppConfig>>,
    pub trades: Arc<Mutex<Vec<TradeRecord>>>,
    pub stop_tx: Arc<Mutex<Option<tokio::sync::oneshot::Sender<()>>>>,
    pub auth_store: Arc<AuthStore>,
    pub fee_cache: Arc<Mutex<FeeInfo>>,
    pub sse_broadcaster: broadcast::Sender<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRecord {
    pub id: String,
    pub timestamp: String,
    pub opportunity: Opportunity,
    pub result: Option<blockchain::TradeResult>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ConfigUpdateRequest {
    pub rpc_url: Option<String>,
    pub private_key: Option<String>,
    pub contract_address: Option<String>,
    pub borrow_amount: Option<f64>,
    pub min_profit_usd: Option<f64>,
    pub max_gas_price_gwei: Option<f64>,
    pub scan_interval_ms: Option<u64>,
    pub auto_trade: Option<bool>,
    pub daily_start_hour: Option<u32>,
    pub daily_end_hour: Option<u32>,
    pub max_slippage_bps: Option<u32>,
    pub max_price_impact_pct: Option<f64>,
    pub simulate_before_send: Option<bool>,
    pub reverse_route: Option<bool>,
    pub min_borrow_amount: Option<f64>,
    pub max_borrow_amount: Option<f64>,
    pub borrow_step: Option<f64>,
}

#[derive(Deserialize)]
pub struct ExecuteRequest {
    pub borrow_amount: Option<f64>,
}

#[derive(Deserialize)]
pub struct WithdrawRequest {
    pub token: String,
    pub amount: f64,
}

async fn validate_auth(
    req: &HttpRequest,
    auth_store: &AuthStore,
) -> Option<String> {
    let no_auth_paths = vec![
        "/api/auth/login",
        "/api/auth/register",
        "/api/events",
    ];

    let path = req.path();
    if no_auth_paths.contains(&path) || path.starts_with("/static") || path == "/" {
        return Some(String::new());
    }

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token_str) = auth_header.to_str() {
            let token = token_str.strip_prefix("Bearer ").unwrap_or(token_str);
            return auth_store.get_username(token);
        }
    }

    None
}

async fn auth_middleware(
    req: ServiceRequest,
    next: middleware::Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let no_auth_paths = ["/api/auth/login", "/api/auth/register", "/api/events"];

    let path = req.path();
    let should_skip = no_auth_paths.iter().any(|p| path.starts_with(p))
        || path.starts_with("/static")
        || path == "/";

    if should_skip {
        return next.call(req).await;
    }

    let token = req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    match token {
        Some(t) => {
            let auth_store = req.app_data::<Data<AppState>>().map(|s| s.auth_store.clone());
            if let Some(store) = auth_store {
                if store.get_username(&t).is_some() {
                    return next.call(req).await;
                }
            }
            Err(ErrorUnauthorized("Unauthorized"))
        }
        None => Err(ErrorUnauthorized("Unauthorized")),
    }
}

async fn register_handler(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    match state.auth_store.register(&body.username, &body.password) {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "token": token
        })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": e
        })),
    }
}

async fn login_handler(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    match state.auth_store.login(&body.username, &body.password) {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "token": token
        })),
        Err(e) => HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "error": e
        })),
    }
}

async fn logout_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token_str) = auth_header.to_str() {
            let token = token_str.strip_prefix("Bearer ").unwrap_or(token_str);
            let _ = state.auth_store.logout(token);
        }
    }
    HttpResponse::Ok().json(serde_json::json!({
        "success": true
    }))
}

async fn me_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token_str) = auth_header.to_str() {
            let token = token_str.strip_prefix("Bearer ").unwrap_or(token_str);
            if let Some(session) = state.auth_store.validate_token(token) {
                return HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "username": session.username,
                    "created_at": session.created_at,
                    "expires_at": session.expires_at
                }));
            }
        }
    }
    HttpResponse::Unauthorized().json(serde_json::json!({
        "success": false,
        "error": "Invalid token"
    }))
}

async fn status_handler(state: web::Data<AppState>) -> HttpResponse {
    let status = state.status.lock().await;
    HttpResponse::Ok().json(&*status)
}

async fn config_get_handler(state: web::Data<AppState>) -> HttpResponse {
    let config = state.config.lock().await;
    let mut safe_config = config.clone();
    safe_config.private_key = if safe_config.private_key.is_empty() {
        String::new()
    } else {
        "••••••••".to_string()
    };
    safe_config.tenderly_api_key = if safe_config.tenderly_api_key.is_empty() {
        String::new()
    } else {
        "••••••••".to_string()
    };
    HttpResponse::Ok().json(&safe_config)
}

async fn config_update_handler(
    state: web::Data<AppState>,
    body: web::Json<ConfigUpdateRequest>,
) -> HttpResponse {
    let mut config = state.config.lock().await;

    if let Some(v) = &body.rpc_url {
        config.rpc_url = v.clone();
    }
    if let Some(v) = &body.private_key {
        if !v.is_empty() && v != "••••••••" {
            config.private_key = v.clone();
        }
    }
    if let Some(v) = &body.contract_address {
        config.contract_address = v.clone();
    }
    if let Some(v) = body.borrow_amount {
        config.borrow_amount = v;
    }
    if let Some(v) = body.min_profit_usd {
        config.min_profit_usd = v;
    }
    if let Some(v) = body.max_gas_price_gwei {
        config.max_gas_price_gwei = v;
    }
    if let Some(v) = body.scan_interval_ms {
        config.scan_interval_ms = v;
    }
    if let Some(v) = body.auto_trade {
        config.auto_trade = v;
    }
    if let Some(v) = body.daily_start_hour {
        config.daily_start_hour = v;
    }
    if let Some(v) = body.daily_end_hour {
        config.daily_end_hour = v;
    }
    if let Some(v) = body.max_slippage_bps {
        config.max_slippage_bps = v;
    }
    if let Some(v) = body.max_price_impact_pct {
        config.max_price_impact_pct = v;
    }
    if let Some(v) = body.simulate_before_send {
        config.simulate_before_send = v;
    }
    if let Some(v) = body.reverse_route {
        config.reverse_route = v;
    }
    if let Some(v) = body.min_borrow_amount {
        config.min_borrow_amount = v;
    }
    if let Some(v) = body.max_borrow_amount {
        config.max_borrow_amount = v;
    }
    if let Some(v) = body.borrow_step {
        config.borrow_step = v;
    }

    config.validate_borrow_amount();
    config.save();

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Config updated"
    }))
}

async fn bot_start_handler(state: web::Data<AppState>) -> HttpResponse {
    let mut status = state.status.lock().await;
    if status.is_running {
        return HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": "Bot is already running"
        }));
    }
    status.is_running = true;
    drop(status);

    let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel::<()>();

    {
        let mut stop_tx_store = state.stop_tx.lock().await;
        *stop_tx_store = Some(stop_tx);
    }

    let state_clone = state.clone();
    let config = state.config.lock().await.clone();
    drop(state.status.lock().await);

    tokio::spawn(async move {
        let provider = if config.private_key.is_empty() {
            match BlockchainClient::new_readonly(&config.rpc_url, config.chain_id).await {
                Ok(p) => Arc::new(p),
                Err(e) => {
                    log::error!("Failed to create provider: {}", e);
                    let mut status = state_clone.status.lock().await;
                    status.is_running = false;
                    return;
                }
            }
        } else {
            match BlockchainClient::new(
                &config.rpc_url,
                &config.private_key,
                config.chain_id,
            )
            .await
            {
                Ok(p) => Arc::new(p),
                Err(e) => {
                    log::error!("Failed to create provider: {}", e);
                    let mut status = state_clone.status.lock().await;
                    status.is_running = false;
                    return;
                }
            }
        };

        let scanner = Scanner::new(provider.clone(), config.clone());

        let gas_price = provider.get_gas_price().await.unwrap_or(blockchain::ARBITRUM_GAS_PRICE_GWEI);
        let _eth_price = blockchain::ETH_PRICE_USD;
        let swap_cost = provider.estimate_swap_cost_usd(gas_price);
        let deploy_cost = provider.estimate_deployment_cost(gas_price);
        let total_per_trade = swap_cost * 2.0 + 0.05;

        {
            let mut fee_cache = state_clone.fee_cache.lock().await;
            *fee_cache = FeeInfo {
                gas_price_gwei: gas_price,
                l1_data_fee_usd: blockchain::ARBITRUM_L1_DATA_FEE_USD,
                swap_fee_usd: swap_cost,
                flash_loan_fee_usd: 0.0,
                deployment_cost_usd: deploy_cost,
                total_fee_per_trade_usd: total_per_trade,
            };
        }

        log::info!("Bot started, scanning every {}ms", config.scan_interval_ms);

        let mut scan_interval = interval(Duration::from_millis(config.scan_interval_ms));

        loop {
            tokio::select! {
                _ = &mut stop_rx => {
                    log::info!("Bot stopped");
                    break;
                }
                _ = scan_interval.tick() => {
                    let now = Utc::now();
                    let hour = now.hour();
                    if hour < config.daily_start_hour || hour > config.daily_end_hour {
                        continue;
                    }

                    match scanner.scan_all().await {
                        opportunities if !opportunities.is_empty() => {
                            if let Some(best) = opportunities.first() {
                                if best.net_profit_after_costs > config.min_profit_usd {
                                    log::info!(
                                        "Found opportunity: {} on {} -> {} profit ${:.4}",
                                        best.token_pair, best.dex_to,
                                        best.route_description, best.net_profit_after_costs
                                    );

                                    let mut status = state_clone.status.lock().await;
                                    status.last_trade_time = Some(Utc::now().to_rfc3339());
                                    status.total_trades += 1;
                                    status.total_profit += best.net_profit_after_costs;
                                    status.avg_slippage_bps = (status.avg_slippage_bps
                                        * (status.total_trades - 1) as f64
                                        + best.slippage_bps as f64)
                                        / status.total_trades as f64;
                                    status.avg_price_impact_bps = (status.avg_price_impact_bps
                                        * (status.total_trades - 1) as f64
                                        + best.price_impact_bps as f64)
                                        / status.total_trades as f64;

                                    let trade_record = TradeRecord {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        timestamp: Utc::now().to_rfc3339(),
                                        opportunity: best.clone(),
                                        result: None,
                                        status: "scanned".to_string(),
                                    };

                                    let mut trades = state_clone.trades.lock().await;
                                    trades.push(trade_record);
                                    let len = trades.len();
                                    if len > 1000 {
                                        trades.drain(0..len - 1000);
                                    }
                                }
                            }

                            {
                                let mut status = state_clone.status.lock().await;
                                let new_gas = provider.get_gas_price().await.unwrap_or(gas_price);
                                status.current_gas_price_gwei = new_gas;
                                status.estimated_fee_per_trade =
                                    provider.estimate_swap_cost_usd(new_gas) * 2.0 + 0.05;

                                let balance = provider.get_eth_balance().await.unwrap_or(0.0);
                                status.balance = balance;
                            }
                        }
                        _ => {}
                    }

                    let status = state_clone.status.lock().await.clone();
                    let status_json = serde_json::to_string(&status).unwrap_or_default();
                    let _ = state_clone.sse_broadcaster.send(status_json);
                }
            }
        }

        let mut status = state_clone.status.lock().await;
        status.is_running = false;
    });

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Bot started"
    }))
}

async fn bot_stop_handler(state: web::Data<AppState>) -> HttpResponse {
    let mut stop_tx_store = state.stop_tx.lock().await;
    if let Some(stop_tx) = stop_tx_store.take() {
        let _ = stop_tx.send(());
    }

    let mut status = state.status.lock().await;
    status.is_running = false;

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Bot stopped"
    }))
}

async fn bot_execute_handler(
    state: web::Data<AppState>,
    body: web::Json<ExecuteRequest>,
) -> HttpResponse {
    let config = state.config.lock().await.clone();
    let borrow_amount = body.borrow_amount.unwrap_or(config.borrow_amount);

    let provider = match BlockchainClient::new(
        &config.rpc_url,
        &config.private_key,
        config.chain_id,
    )
    .await
    {
        Ok(p) => Arc::new(p),
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": e
            }));
        }
    };

    let scanner = Scanner::new(provider.clone(), config.clone());

    let opportunity = match scanner.scan_opportunity_with_amount(borrow_amount).await {
        Some(opp) => opp,
        None => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "No profitable opportunity found"
            }));
        }
    };

    let result = scanner.execute_trade(&opportunity).await;

    let mut trades = state.trades.lock().await;
    let trade_record = TradeRecord {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: Utc::now().to_rfc3339(),
        opportunity,
        result: Some(result.clone().unwrap_or(blockchain::TradeResult {
            success: false,
            profit: 0.0,
            gas_cost: 0.0,
            actual_slippage_bps: 0,
            simulated: false,
            tx_hash: None,
            error: Some("Execution failed".to_string()),
        })),
        status: if result.as_ref().map(|r| r.success).unwrap_or(false) {
            "success".to_string()
        } else {
            "failed".to_string()
        },
    };

    trades.push(trade_record);

    HttpResponse::Ok().json(serde_json::json!({
        "success": result.as_ref().map(|r| r.success).unwrap_or(false),
        "result": result,
    }))
}

async fn bot_simulate_handler(
    state: web::Data<AppState>,
    body: web::Json<ExecuteRequest>,
) -> HttpResponse {
    let config = state.config.lock().await.clone();
    let borrow_amount = body.borrow_amount.unwrap_or(config.borrow_amount);

    let provider = match BlockchainClient::new(
        &config.rpc_url,
        &config.private_key,
        config.chain_id,
    )
    .await
    {
        Ok(p) => Arc::new(p),
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": e
            }));
        }
    };

    let scanner = Scanner::new(provider.clone(), config.clone());

    let opportunity = match scanner.scan_opportunity_with_amount(borrow_amount).await {
        Some(opp) => opp,
        None => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "No profitable opportunity found"
            }));
        }
    };

    let gas_price = provider.get_gas_price().await.unwrap_or(blockchain::ARBITRUM_GAS_PRICE_GWEI);
    let total_cost = provider.estimate_total_trade_cost(gas_price, 2);

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "opportunity": opportunity,
        "estimated_cost": total_cost,
        "simulated": true
    }))
}

async fn trades_handler(state: web::Data<AppState>) -> HttpResponse {
    let trades = state.trades.lock().await;
    HttpResponse::Ok().json(&*trades)
}

async fn contract_balance_handler(state: web::Data<AppState>) -> HttpResponse {
    let config = state.config.lock().await.clone();

    if config.contract_address.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Contract address not configured"
        }));
    }

    let provider = match BlockchainClient::new(
        &config.rpc_url,
        &config.private_key,
        config.chain_id,
    )
    .await
    {
        Ok(p) => Arc::new(p),
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": e
            }));
        }
    };

    let mut balances = HashMap::new();
    for token in &config.tokens {
        let balance = provider
            .get_contract_balance(&token.address, &config.contract_address)
            .await
            .unwrap_or(0.0);
        balances.insert(token.symbol.clone(), balance);
    }

    let eth_balance = provider.get_eth_balance().await.unwrap_or(0.0);

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "balances": balances,
        "eth_balance": eth_balance
    }))
}

async fn contract_withdraw_handler(
    state: web::Data<AppState>,
    body: web::Json<WithdrawRequest>,
) -> HttpResponse {
    let config = state.config.lock().await.clone();

    if config.contract_address.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Contract address not configured"
        }));
    }

    let provider = match BlockchainClient::new(
        &config.rpc_url,
        &config.private_key,
        config.chain_id,
    )
    .await
    {
        Ok(p) => Arc::new(p),
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": e
            }));
        }
    };

    let token_config = config.tokens.iter().find(|t| t.symbol == body.token);
    let token_address = match token_config {
        Some(t) => t.address.clone(),
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": format!("Unknown token: {}", body.token)
            }));
        }
    };

    let decimals = token_config.unwrap().decimals;
    let amount_wei = ethers::types::U256::from((body.amount * 10f64.powi(decimals as i32)) as u64);

    let result = provider
        .withdraw_profits(&config.contract_address, &token_address, amount_wei)
        .await;

    match result {
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({
            "success": r.success,
            "result": r
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e
        })),
    }
}

async fn fees_handler(state: web::Data<AppState>) -> HttpResponse {
    let fee_cache = state.fee_cache.lock().await;
    HttpResponse::Ok().json(&*fee_cache)
}

async fn sse_handler(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> HttpResponse {
    let rx = state.sse_broadcaster.subscribe();

    let stream = futures_util::stream::unfold(rx, |mut rx| async {
        match rx.recv().await {
            Ok(msg) => {
                let sse_data = format!("data: {}\n\n", msg);
                Some((Ok::<_, actix_web::error::Error>(
                    actix_web::web::Bytes::from(sse_data)
                ), rx))
            }
            Err(broadcast::error::RecvError::Lagged(_)) => {
                let mut rx2 = rx;
                match rx2.recv().await {
                    Ok(msg) => {
                        let sse_data = format!("data: {}\n\n", msg);
                        Some((Ok::<_, actix_web::error::Error>(
                            actix_web::web::Bytes::from(sse_data)
                        ), rx2))
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    });

    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .insert_header(("X-Accel-Buffering", "no"))
        .streaming(stream)
}

async fn refresh_fees(state: &AppState) {
    let config = state.config.lock().await.clone();

    if let Ok(provider) = BlockchainClient::new(
        &config.rpc_url,
        &config.private_key,
        config.chain_id,
    )
    .await
    {
        let gas_price = provider.get_gas_price().await.unwrap_or(blockchain::ARBITRUM_GAS_PRICE_GWEI);
        let swap_cost = provider.estimate_swap_cost_usd(gas_price);
        let deploy_cost = provider.estimate_deployment_cost(gas_price);
        let total_per_trade = swap_cost * 2.0 + 0.05;

        let mut fee_cache = state.fee_cache.lock().await;
        *fee_cache = FeeInfo {
            gas_price_gwei: gas_price,
            l1_data_fee_usd: blockchain::ARBITRUM_L1_DATA_FEE_USD,
            swap_fee_usd: swap_cost,
            flash_loan_fee_usd: 0.0,
            deployment_cost_usd: deploy_cost,
            total_fee_per_trade_usd: total_per_trade,
        };
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let auth_store = Arc::new(AuthStore::new().expect("Failed to initialize auth store"));
    let config = AppConfig::load();

    let default_status = BotStatus {
        is_running: false,
        total_trades: 0,
        total_profit: 0.0,
        last_trade_time: None,
        gas_spent: 0.0,
        balance: 0.0,
        network: "Arbitrum One".to_string(),
        avg_slippage_bps: 0.0,
        avg_price_impact_bps: 0.0,
        active_tokens: config.tokens.iter().map(|t| t.symbol.clone()).collect(),
        active_dexes: config.dexes.iter().map(|d| d.name.clone()).collect(),
        scan_interval_ms: config.scan_interval_ms,
        eth_price: blockchain::ETH_PRICE_USD,
        current_gas_price_gwei: blockchain::ARBITRUM_GAS_PRICE_GWEI,
        estimated_fee_per_trade: 0.0,
        last_scan: None,
        total_scans: 0,
        opportunities_found: 0,
        pairs_scanned: 0,
        scanning: false,
    };

    let default_fee = FeeInfo {
        gas_price_gwei: blockchain::ARBITRUM_GAS_PRICE_GWEI,
        l1_data_fee_usd: blockchain::ARBITRUM_L1_DATA_FEE_USD,
        swap_fee_usd: 0.0,
        flash_loan_fee_usd: 0.0,
        deployment_cost_usd: 0.0,
        total_fee_per_trade_usd: 0.0,
    };

    let (sse_broadcaster, _) = broadcast::channel::<String>(100);

    let app_state = AppState {
        status: Arc::new(Mutex::new(default_status)),
        config: Arc::new(Mutex::new(config)),
        trades: Arc::new(Mutex::new(Vec::new())),
        stop_tx: Arc::new(Mutex::new(None)),
        auth_store,
        fee_cache: Arc::new(Mutex::new(default_fee)),
        sse_broadcaster,
    };

    let state_for_fees = app_state.clone();
    tokio::spawn(async move {
        refresh_fees(&state_for_fees).await;
        let mut fee_interval = interval(Duration::from_secs(60));
        loop {
            fee_interval.tick().await;
            refresh_fees(&state_for_fees).await;
        }
    });

    let state_heartbeat = app_state.clone();
    tokio::spawn(async move {
        let mut hb = interval(Duration::from_secs(2));
        loop {
            hb.tick().await;
            let status = state_heartbeat.status.lock().await.clone();
            let msg = serde_json::json!({"type": "status", "data": status});
            let _ = state_heartbeat.sse_broadcaster.send(
                serde_json::to_string(&msg).unwrap_or_default()
            );
            let fee = state_heartbeat.fee_cache.lock().await.clone();
            let fmsg = serde_json::json!({"type": "fees", "data": fee});
            let _ = state_heartbeat.sse_broadcaster.send(
                serde_json::to_string(&fmsg).unwrap_or_default()
            );
        }
    });

    let state_scanner = app_state.clone();
    tokio::spawn(async move {
        let scan_cfg = state_scanner.config.lock().await.clone();
        let provider = match BlockchainClient::new_readonly(
            &scan_cfg.rpc_url,
            scan_cfg.chain_id,
        ).await {
            Ok(p) => Arc::new(p),
            Err(e) => {
                log::error!("Background scanner provider failed: {}", e);
                return;
            }
        };
        let scanner = Scanner::new(provider, scan_cfg.clone());
        let mut scan_ticker = interval(Duration::from_millis(
            scan_cfg.scan_interval_ms.max(500)
        ));
        log::info!(
            "Background scanner started: {} tokens × {} bases × {} dexes",
            scan_cfg.tokens.len(), scan_cfg.base_pairs.len(), scan_cfg.dexes.len()
        );
        loop {
            scan_ticker.tick().await;
            let now = Utc::now();
            let hour = now.hour();
            if hour < scan_cfg.daily_start_hour || hour > scan_cfg.daily_end_hour {
                continue;
            }
            {
                let mut st = state_scanner.status.lock().await;
                st.scanning = true;
                st.pairs_scanned = (scan_cfg.tokens.len()
                    * scan_cfg.base_pairs.len()
                    * scan_cfg.dexes.len()) as u32;
            }
            let scan_msg = serde_json::json!({
                "type": "scan",
                "data": {
                    "status": "scanning",
                    "timestamp": Utc::now().to_rfc3339(),
                    "pairs": scan_cfg.tokens.len() * scan_cfg.base_pairs.len() * scan_cfg.dexes.len()
                }
            });
            let _ = state_scanner.sse_broadcaster.send(
                serde_json::to_string(&scan_msg).unwrap_or_default()
            );
            match scanner.scan_all().await {
                opportunities if !opportunities.is_empty() => {
                    for opp in opportunities.iter().take(10) {
                        let msg = serde_json::json!({
                            "type": "opportunity",
                            "data": opp
                        });
                        let _ = state_scanner.sse_broadcaster.send(
                            serde_json::to_string(&msg).unwrap_or_default()
                        );
                    }
                    let mut st = state_scanner.status.lock().await;
                    st.last_scan = Some(Utc::now().to_rfc3339());
                    st.total_scans += 1;
                    st.opportunities_found = opportunities.len() as u32;
                    st.scanning = false;
                    log::info!(
                        "Scan #{}: {} opportunities found",
                        st.total_scans, opportunities.len()
                    );
                }
                _ => {
                    let mut st = state_scanner.status.lock().await;
                    st.last_scan = Some(Utc::now().to_rfc3339());
                    st.total_scans += 1;
                    st.opportunities_found = 0;
                    st.scanning = false;
                }
            }
        }
    });

    log::info!("Starting Balancer V3 Arb Server on http://0.0.0.0:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(app_state.clone()))
            .route("/api/events", web::get().to(sse_handler))
            .service(
                web::scope("/api")
                    .wrap(middleware::from_fn(auth_middleware))
                    .route("/auth/register", web::post().to(register_handler))
                    .route("/auth/login", web::post().to(login_handler))
                    .route("/auth/logout", web::post().to(logout_handler))
                    .route("/auth/me", web::get().to(me_handler))
                    .route("/status", web::get().to(status_handler))
                    .route("/config", web::get().to(config_get_handler))
                    .route("/config", web::post().to(config_update_handler))
                    .route("/bot/start", web::post().to(bot_start_handler))
                    .route("/bot/stop", web::post().to(bot_stop_handler))
                    .route("/bot/execute", web::post().to(bot_execute_handler))
                    .route("/bot/simulate", web::post().to(bot_simulate_handler))
                    .route("/trades", web::get().to(trades_handler))
                    .route("/contract/balance", web::get().to(contract_balance_handler))
                    .route("/contract/withdraw", web::post().to(contract_withdraw_handler))
                    .route("/fees", web::get().to(fees_handler))
            )
            .service(
                actix_files::Files::new("/", "./web-app/dist")
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open("./web-app/dist/index.html")
                            .expect("index.html not found")
                    )
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
