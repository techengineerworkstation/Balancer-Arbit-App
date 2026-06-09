use leptos::*;

use crate::api;
use crate::components::*;
use crate::TradeLog;

#[component]
pub fn TradeHistory() -> impl IntoView {
    let (trades, set_trades) = use_context::<(ReadSignal<Vec<TradeLog>>, WriteSignal<Vec<TradeLog>>)>
        .unwrap();
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(true);

    create_effect(move |_| {
        let set_trades = set_trades.clone();
        let set_loading = set_loading.clone();
        let set_error = set_error.clone();

        spawn_local(async move {
            match api::get_trade_history().await {
                Ok(new_trades) => {
                    set_trades.set(new_trades);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    });

    let total_profit = move || {
        trades.get().iter().map(|t| t.profit).sum::<f64>()
    };

    let total_gas = move || {
        trades.get().iter().map(|t| t.gas_cost).sum::<f64>()
    };

    let success_rate = move || {
        let t = trades.get();
        if t.is_empty() {
            0.0
        } else {
            let successes = t.iter().filter(|t| t.status == "success").count() as f64;
            (successes / t.len() as f64) * 100.0
        }
    };

    view! {
        <div class="trade-history">
            <h1>"Trade History"</h1>

            {move || error.get().map(|e| view! { <ErrorBanner error=e/> })}

            <div class="stats-row">
                <StatusCard
                    title="Total Trades".to_string()
                    value=move || trades.get().len().to_string()
                    subtitle=None
                    class="card-info".to_string()
                />
                <StatusCard
                    title="Net Profit".to_string()
                    value=move || format!("${:.4}", total_profit())
                    subtitle=Some("USDC".to_string())
                    class=move || if total_profit() > 0.0 { "card-profit".to_string() } else { "card-loss".to_string() }
                />
                <StatusCard
                    title="Total Gas".to_string()
                    value=move || format!("${:.4}", total_gas())
                    subtitle=Some("MATIC".to_string())
                    class="card-info".to_string()
                />
                <StatusCard
                    title="Success Rate".to_string()
                    value=move || format!("{:.1}%", success_rate())
                    subtitle=None
                    class="card-info".to_string()
                />
            </div>

            {move || if loading.get() {
                view! { <LoadingSpinner/> }.into_view()
            } else {
                view! {
                    <table class="trades-table full-table">
                        <thead>
                            <tr>
                                <th>"ID"</th>
                                <th>"Timestamp"</th>
                                <th>"Pair"</th>
                                <th>"Buy From"</th>
                                <th>"Sell To"</th>
                                <th>"Amount"</th>
                                <th>"Profit"</th>
                                <th>"Gas Cost"</th>
                                <th>"Status"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                trades.get().iter().map(|t| {
                                    view! { <TradeRow trade=t.clone()/> }
                                }).collect::<Vec<_>>()
                            }}
                        </tbody>
                    </table>
                }.into_view()
            }}
        </div>
    }
}