use leptos::*;
use gloo_timers::future::IntervalStream;
use futures::StreamExt;

use crate::api;
use crate::components::*;
use crate::{BotStatus, TradeLog};

#[component]
pub fn Dashboard() -> impl IntoView {
    let (status, set_status) = use_context::<(ReadSignal<BotStatus>, WriteSignal<BotStatus>)>()
        .unwrap();
    let (config, set_config) = use_context::<(ReadSignal<crate::Config>, WriteSignal<crate::Config>)>()
        .unwrap();
    let (trades, _) = use_context::<(ReadSignal<Vec<TradeLog>>, WriteSignal<Vec<TradeLog>>)>
        .unwrap();

    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(true);
    let (sim_result, set_sim_result) = create_signal(None::<String>);
    let (quick_amount, set_quick_amount) = create_signal(10000.0_f64);
    let (executing, set_executing) = create_signal(false);

    create_effect(move |_| {
        let set_status = set_status.clone();
        let set_loading = set_loading.clone();
        let set_error = set_error.clone();

        spawn_local(async move {
            match api::get_status().await {
                Ok(new_status) => {
                    set_status.set(new_status);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    });

    let start_bot = create_action(move |_| {
        let set_error = set_error.clone();
        async move {
            if let Err(e) = api::start_bot().await {
                set_error.set(Some(e));
            }
        }
    });

    let stop_bot = create_action(move |_| {
        let set_error = set_error.clone();
        async move {
            if let Err(e) = api::stop_bot().await {
                set_error.set(Some(e));
            }
        }
    });

    let execute_trade = create_action(move |_| {
        let amount = quick_amount.get();
        let set_error = set_error.clone();
        let set_executing = set_executing.clone();
        async move {
            set_executing.set(true);
            match api::execute_trade(Some(amount)).await {
                Ok(()) => set_error.set(None),
                Err(e) => set_error.set(Some(e)),
            }
            set_executing.set(false);
        }
    });

    let simulate_trade = create_action(move |_| {
        let amount = quick_amount.get();
        let set_error = set_error.clone();
        let set_sim_result = set_sim_result.clone();
        async move {
            match api::simulate_trade(Some(amount)).await {
                Ok(result) => set_sim_result.set(Some(result)),
                Err(e) => set_error.set(Some(e)),
            }
        }
    });

    let update_quick_amount = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
            set_quick_amount.set(val);
        }
    };

    view! {
        <div class="dashboard">
            <h1>"Arbitrage Bot Dashboard"</h1>

            {move || error.get().map(|e| view! { <ErrorBanner error=e/> })}

            <div class="loan-amount-bar">
                <div class="loan-amount-header">
                    <span class="loan-amount-label">"Flash Loan Amount (USDC)"</span>
                    <span class="loan-amount-value">{move || format!("${:.0}", quick_amount.get())}</span>
                </div>
                <div class="loan-amount-controls">
                    <input
                        type="range"
                        class="loan-slider"
                        prop:min=move || config.get().min_borrow_amount.to_string()
                        prop:max=move || config.get().max_borrow_amount.to_string()
                        prop:step=move || config.get().borrow_step.to_string()
                        prop:value=move || quick_amount.get().to_string()
                        on:input=update_quick_amount
                    />
                    <input
                        type="number"
                        class="loan-input"
                        prop:value=move || quick_amount.get().to_string()
                        on:input=update_quick_amount
                        min=move || config.get().min_borrow_amount.to_string()
                        max=move || config.get().max_borrow_amount.to_string()
                        step=move || config.get().borrow_step.to_string()
                    />
                </div>
                <div class="loan-range-labels">
                    <span>{move || format!("Min: ${:.0}", config.get().min_borrow_amount)}</span>
                    <span>{move || format!("Max: ${:.0}", config.get().max_borrow_amount)}</span>
                </div>
                <div class="quick-amounts">
                    {move || {
                        let min = config.get().min_borrow_amount;
                        let max = config.get().max_borrow_amount;
                        let mut presets = vec![100.0, 500.0, 1000.0, 5000.0, 10000.0, 25000.0, 50000.0, 100000.0];
                        presets.retain(|&v| v >= min && v <= max);
                        presets.into_iter().map(|preset| {
                            let set_quick = set_quick_amount.clone();
                            view! {
                                <button
                                    class="btn-preset"
                                    class:active=move || (quick_amount.get() - preset).abs() < 1.0
                                    on:click=move |_| set_quick.set(preset)
                                >
                                    {format!("${:.0}", preset)}
                                </button>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>

            <div class="controls">
                <button
                    class="btn btn-success"
                    on:click=move |_| start_bot.dispatch(())
                    disabled=move || status.get().is_running
                >
                    "Start Bot"
                </button>
                <button
                    class="btn btn-danger"
                    on:click=move |_| stop_bot.dispatch(())
                    disabled=move || !status.get().is_running
                >
                    "Stop Bot"
                </button>
                <button
                    class="btn btn-primary"
                    on:click=move |_| execute_trade.dispatch(())
                    disabled=move || executing.get()
                >
                    {move || if executing.get() { "Executing..." } else { "Execute Trade" }}
                </button>
                <button
                    class="btn btn-secondary"
                    on:click=move |_| simulate_trade.dispatch(())
                >
                    "Simulate"
                </button>
            </div>

            {move || sim_result.get().map(|r| view! {
                <div class="simulation-result">
                    <h3>"Simulation Result"</h3>
                    <pre>{r}</pre>
                </div>
            })}

            <div class="status-grid">
                <StatusCard
                    title="Status".to_string()
                    value=move || if status.get().is_running { "Running".to_string() } else { "Stopped".to_string() }
                    subtitle=None
                    class=move || if status.get().is_running { "card-running".to_string() } else { "card-stopped".to_string() }
                />
                <StatusCard
                    title="Total Trades".to_string()
                    value=move || status.get().total_trades.to_string()
                    subtitle=None
                    class="card-info".to_string()
                />
                <StatusCard
                    title="Total Profit".to_string()
                    value=move || format!("${:.2}", status.get().total_profit)
                    subtitle=Some("USDC".to_string())
                    class=move || if status.get().total_profit > 0.0 { "card-profit".to_string() } else { "card-loss".to_string() }
                />
                <StatusCard
                    title="Gas Spent".to_string()
                    value=move || format!("${:.2}", status.get().gas_spent)
                    subtitle=Some("MATIC".to_string())
                    class="card-info".to_string()
                />
                <StatusCard
                    title="Contract Balance".to_string()
                    value=move || format!("${:.2}", status.get().balance)
                    subtitle=Some("USDC".to_string())
                    class="card-info".to_string()
                />
                <StatusCard
                    title="Network".to_string()
                    value=move || status.get().network
                    subtitle=None
                    class="card-info".to_string()
                />
                <StatusCard
                    title="Avg Slippage".to_string()
                    value=move || format!("{} bps", status.get().avg_slippage_bps as u32)
                    subtitle=None
                    class="card-info".to_string()
                />
                <StatusCard
                    title="Avg Price Impact".to_string()
                    value=move || format!("{} bps", status.get().avg_price_impact_bps as u32)
                    subtitle=None
                    class=move || if status.get().avg_price_impact_bps > 50.0 { "card-warning".to_string() } else { "card-info".to_string() }
                />
                <StatusCard
                    title="Direct Pool Calls".to_string()
                    value=move || if status.get().use_direct_pool_calls { "ON".to_string() } else { "OFF".to_string() }
                    subtitle=None
                    class=move || if status.get().use_direct_pool_calls { "card-profit".to_string() } else { "card-loss".to_string() }
                />
                <StatusCard
                    title="Simulation".to_string()
                    value=move || if status.get().simulate_before_send { "ON".to_string() } else { "OFF".to_string() }
                    subtitle=None
                    class=move || if status.get().simulate_before_send { "card-profit".to_string() } else { "card-warning".to_string() }
                />
            </div>

            <div class="recent-trades">
                <h2>"Recent Trades"</h2>
                <table class="trades-table">
                    <thead>
                        <tr>
                            <th>"Time"</th>
                            <th>"Pair"</th>
                            <th>"Route"</th>
                            <th>"Amount"</th>
                            <th>"Profit"</th>
                            <th>"Net"</th>
                            <th>"Impact"</th>
                            <th>"Slip"</th>
                            <th>"Gas"</th>
                            <th>"Sim"</th>
                            <th>"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            trades.get().iter().take(10).map(|t| {
                                view! { <TradeRow trade=t.clone()/> }
                            }).collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>
        </div>
    }
}