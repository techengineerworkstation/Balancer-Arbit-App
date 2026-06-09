use leptos::*;
use leptos_router::*;

use crate::BotStatus;

#[component]
pub fn NavBar() -> impl IntoView {
    let (status, _) = use_context::<(ReadSignal<BotStatus>, WriteSignal<BotStatus>)>()
        .unwrap();

    view! {
        <nav class="navbar">
            <div class="navbar-brand">
                <A href="/" class="navbar-logo">
                    "Balancer Arbitrage Bot"
                </A>
            </div>
            <div class="navbar-menu">
                <A href="/" class="navbar-item">"Dashboard"</A>
                <A href="/config" class="navbar-item">"Configuration"</A>
                <A href="/trades" class="navbar-item">"Trade History"</A>
                <A href="/contract" class="navbar-item">"Contract"</A>
            </div>
            <div class="navbar-status">
                <span class=move || if status.get().is_running {
                    "status-indicator running"
                } else {
                    "status-indicator stopped"
                }>
                    {move || if status.get().is_running { "● Running" } else { "● Stopped" }}
                </span>
            </div>
        </nav>
    }
}

#[component]
pub fn StatusCard(
    title: String,
    value: String,
    subtitle: Option<String>,
    class: Option<String>,
) -> impl IntoView {
    view! {
        <div class=move || format!("status-card {}", class.unwrap_or_default())>
            <h3 class="status-card-title">{title}</h3>
            <div class="status-card-value">{value}</div>
            {subtitle.map(|s| view! { <div class="status-card-subtitle">{s}</div> })}
        </div>
    }
}

#[component]
pub fn TradeRow(trade: crate::TradeLog) -> impl IntoView {
    let status_class = match trade.status.as_str() {
        "success" => "trade-success",
        "failed" => "trade-failed",
        "pending" => "trade-pending",
        _ => "",
    };

    view! {
        <tr>
            <td>{trade.timestamp}</td>
            <td>{trade.token_pair}</td>
            <td>{trade.dex_from}</td>
            <td>{trade.dex_to}</td>
            <td>{format!("{:.2}", trade.amount)}</td>
            <td class=move || if trade.profit > 0.0 { "profit-positive" } else { "profit-negative" }>
                {format!("{:.4}", trade.profit)}
            </td>
            <td>{format!("{:.4}", trade.gas_cost)}</td>
            <td class=status_class>{trade.status}</td>
        </tr>
    }
}

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="loading-spinner">
            <div class="spinner"></div>
            <span>"Loading..."</span>
        </div>
    }
}

#[component]
pub fn ErrorBanner(error: String) -> impl IntoView {
    view! {
        <div class="error-banner">
            <span class="error-icon">"⚠"</span>
            <span class="error-text">{error}</span>
        </div>
    }
}

#[component]
pub fn ToggleSwitch(
    checked: Signal<bool>,
    on_change: impl Fn(bool) + 'static,
    label: String,
) -> impl IntoView {
    view! {
        <label class="toggle-switch">
            <span class="toggle-label">{label}</span>
            <input
                type="checkbox"
                prop:checked=move || checked.get()
                on:change=move |ev| {
                    let checked = event_target_checked(&ev);
                    on_change(checked);
                }
            />
            <span class="toggle-slider"></span>
        </label>
    }
}