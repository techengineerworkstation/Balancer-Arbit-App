use gloo_net::http::Request;
use gloo_net::http::Response;
use serde::{Deserialize, Serialize};

use crate::{BotStatus, Config, TradeLog};

const API_BASE: &str = "/api";

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn get_status() -> Result<BotStatus, String> {
    let resp = Request::get(&format!("{}/status", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status: BotStatus = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    Ok(status)
}

pub async fn get_config() -> Result<Config, String> {
    let resp = Request::get(&format!("{}/config", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let config: Config = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    Ok(config)
}

pub async fn update_config(config: &Config) -> Result<(), String> {
    let resp = Request::post(&format!("{}/config", API_BASE))
        .json(config)
        .map_err(|e| format!("JSON serialize failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Update failed: {}", resp.status()))
    }
}

pub async fn start_bot() -> Result<(), String> {
    let resp = Request::post(&format!("{}/bot/start", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Start failed: {}", resp.status()))
    }
}

pub async fn stop_bot() -> Result<(), String> {
    let resp = Request::post(&format!("{}/bot/stop", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Stop failed: {}", resp.status()))
    }
}

pub async fn execute_trade(amount: Option<f64>) -> Result<(), String> {
    let body = match amount {
        Some(a) => serde_json::json!({ "amount": a }),
        None => serde_json::json!({}),
    };

    let resp = Request::post(&format!("{}/bot/execute", API_BASE))
        .json(&body)
        .map_err(|e| format!("JSON serialize failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Execute failed: {}", resp.status()))
    }
}

pub async fn get_trade_history() -> Result<Vec<TradeLog>, String> {
    let resp = Request::get(&format!("{}/trades", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let trades: Vec<TradeLog> = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    Ok(trades)
}

pub async fn get_contract_balance() -> Result<f64, String> {
    let resp = Request::get(&format!("{}/contract/balance", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let balance: f64 = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    Ok(balance)
}

pub async fn withdraw_profits(token_address: &str) -> Result<(), String> {
    let resp = Request::post(&format!("{}/contract/withdraw", API_BASE))
        .json(&serde_json::json!({ "token": token_address }))
        .map_err(|e| format!("JSON serialize failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Withdraw failed: {}", resp.status()))
    }
}

pub async fn simulate_trade(amount: Option<f64>) -> Result<String, String> {
    let body = match amount {
        Some(a) => serde_json::json!({ "amount": a }),
        None => serde_json::json!({}),
    };

    let resp = Request::post(&format!("{}/bot/simulate", API_BASE))
        .json(&body)
        .map_err(|e| format!("JSON serialize failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    Ok(serde_json::to_string_pretty(&body).unwrap_or_default())
}