use leptos::*;

use crate::api;
use crate::components::*;
use crate::Config;

#[component]
pub fn ConfigPage() -> impl IntoView {
    let (config, set_config) = use_context::<(ReadSignal<Config>, WriteSignal<Config>)>()
        .unwrap();
    let (error, set_error) = create_signal(None::<String>);
    let (saving, set_saving) = create_signal(false);
    let (saved, set_saved) = create_signal(false);

    create_effect(move |_| {
        let set_config = set_config.clone();
        spawn_local(async move {
            if let Ok(new_config) = api::get_config().await {
                set_config.set(new_config);
            }
        });
    });

    let save_config = create_action(move |_| {
        let config = config.get();
        let set_error = set_error.clone();
        let set_saving = set_saving.clone();
        let set_saved = set_saved.clone();
        async move {
            set_saving.set(true);
            match api::update_config(&config).await {
                Ok(()) => {
                    set_saving.set(false);
                    set_saved.set(true);
                    set_error.set(None);
                }
                Err(e) => {
                    set_saving.set(false);
                    set_error.set(Some(e));
                }
            }
        }
    });

    let update_rpc = move |ev: web_sys::Event| {
        let val = event_target_value(&ev);
        set_config.update(|c| c.rpc_url = val);
    };

    let update_private_key = move |ev: web_sys::Event| {
        let val = event_target_value(&ev);
        set_config.update(|c| c.private_key = val);
    };

    let update_contract_address = move |ev: web_sys::Event| {
        let val = event_target_value(&ev);
        set_config.update(|c| c.contract_address = val);
    };

    let update_borrow_amount = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
            set_config.update(|c| c.borrow_amount = val);
        }
    };

    let update_min_profit = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
            set_config.update(|c| c.min_profit_usd = val);
        }
    };

    let update_max_gas = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
            set_config.update(|c| c.max_gas_price_gwei = val);
        }
    };

    let update_scan_interval = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<u64>() {
            set_config.update(|c| c.scan_interval_ms = val);
        }
    };

    let update_max_slippage = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<u32>() {
            set_config.update(|c| c.max_slippage_bps = val);
        }
    };

    let update_max_price_impact = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
            set_config.update(|c| c.max_price_impact_pct = val);
        }
    };

    let update_auto_trade = move |checked: bool| {
        set_config.update(|c| c.auto_trade = checked);
    };

    let update_simulate = move |checked: bool| {
        set_config.update(|c| c.simulate_before_send = checked);
    };

    let update_direct_pool = move |checked: bool| {
        set_config.update(|c| c.use_direct_pool_calls = checked);
    };

    let update_reverse_route = move |checked: bool| {
        set_config.update(|c| c.reverse_route = checked);
    };

    let update_start_hour = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<u32>() {
            set_config.update(|c| c.daily_start_hour = val);
        }
    };

    let update_end_hour = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<u32>() {
            set_config.update(|c| c.daily_end_hour = val);
        }
    };

    let update_tenderly_key = move |ev: web_sys::Event| {
        let val = event_target_value(&ev);
        set_config.update(|c| c.tenderly_api_key = val);
    };

    let update_tenderly_project = move |ev: web_sys::Event| {
        let val = event_target_value(&ev);
        set_config.update(|c| c.tenderly_project_slug = val);
    };

    let update_min_borrow = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
            set_config.update(|c| c.min_borrow_amount = val);
        }
    };

    let update_max_borrow = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
            set_config.update(|c| c.max_borrow_amount = val);
        }
    };

    let update_borrow_step = move |ev: web_sys::Event| {
        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
            set_config.update(|c| c.borrow_step = val);
        }
    };

    view! {
        <div class="config-page">
            <h1>"Configuration"</h1>

            {move || error.get().map(|e| view! { <ErrorBanner error=e/> })}

            <div class="config-section">
                <h2>"Network Settings"</h2>
                <div class="config-field">
                    <label>"RPC URL"</label>
                    <input
                        type="text"
                        prop:value=move || config.get().rpc_url
                        on:input=update_rpc
                        placeholder="https://polygon-rpc.com"
                    />
                </div>
                <div class="config-field">
                    <label>"Private Key"</label>
                    <input
                        type="password"
                        prop:value=move || config.get().private_key
                        on:input=update_private_key
                        placeholder="0x..."
                    />
                </div>
                <div class="config-field">
                    <label>"Contract Address"</label>
                    <input
                        type="text"
                        prop:value=move || config.get().contract_address
                        on:input=update_contract_address
                        placeholder="0x..."
                    />
                </div>
            </div>

            <div class="config-section">
                <h2>"Trading Settings"</h2>
                <div class="config-field">
                    <label>"Default Borrow Amount (USDC)"</label>
                    <input
                        type="number"
                        prop:value=move || config.get().borrow_amount.to_string()
                        on:input=update_borrow_amount
                        min=move || config.get().min_borrow_amount.to_string()
                        step=move || config.get().borrow_step.to_string()
                    />
                </div>
                <div class="config-row">
                    <div class="config-field">
                        <label>"Min Borrow (USDC)"</label>
                        <input
                            type="number"
                            prop:value=move || config.get().min_borrow_amount.to_string()
                            on:input=update_min_borrow
                            min="10"
                            step="10"
                        />
                    </div>
                    <div class="config-field">
                        <label>"Max Borrow (USDC)"</label>
                        <input
                            type="number"
                            prop:value=move || config.get().max_borrow_amount.to_string()
                            on:input=update_max_borrow
                            min="100"
                            step="1000"
                        />
                    </div>
                    <div class="config-field">
                        <label>"Step Size (USDC)"</label>
                        <input
                            type="number"
                            prop:value=move || config.get().borrow_step.to_string()
                            on:input=update_borrow_step
                            min="10"
                            step="10"
                        />
                    </div>
                </div>
                <div class="config-field">
                    <label>"Minimum Profit (USD)"</label>
                    <input
                        type="number"
                        prop:value=move || config.get().min_profit_usd.to_string()
                        on:input=update_min_profit
                        min="0.1"
                        step="0.1"
                    />
                </div>
                <div class="config-field">
                    <label>"Max Gas Price (Gwei)"</label>
                    <input
                        type="number"
                        prop:value=move || config.get().max_gas_price_gwei.to_string()
                        on:input=update_max_gas
                        min="10"
                        step="10"
                    />
                </div>
                <div class="config-field">
                    <label>"Scan Interval (ms)"</label>
                    <input
                        type="number"
                        prop:value=move || config.get().scan_interval_ms.to_string()
                        on:input=update_scan_interval
                        min="500"
                        step="500"
                    />
                </div>
            </div>

            <div class="config-section">
                <h2>"Slippage & Price Impact"</h2>
                <p class="config-help">
                    "Maximum slippage tolerance in basis points (1 bps = 0.01%). Higher values allow more price movement but increase risk."
                </p>
                <div class="config-field">
                    <label>"Max Slippage (bps)"</label>
                    <input
                        type="number"
                        prop:value=move || config.get().max_slippage_bps.to_string()
                        on:input=update_max_slippage
                        min="1"
                        max="500"
                        step="5"
                    />
                    <span class="config-hint">
                        {move || format!("({:.2}%)", config.get().max_slippage_bps as f64 / 100.0)}
                    </span>
                </div>
                <p class="config-help">
                    "Maximum allowed price impact as percentage of trade size vs pool liquidity. Larger trades on small pools will exceed this."
                </p>
                <div class="config-field">
                    <label>"Max Price Impact (%)"</label>
                    <input
                        type="number"
                        prop:value=move || config.get().max_price_impact_pct.to_string()
                        on:input=update_max_price_impact
                        min="0.1"
                        max="5.0"
                        step="0.1"
                    />
                    <span class="config-hint">
                        {move || format!("({:.1}%)", config.get().max_price_impact_pct)}
                    </span>
                </div>
            </div>

            <div class="config-section">
                <h2>"Routing Settings"</h2>
                <p class="config-help">
                    "Direct pool calls bypass the router contract, saving ~0.3% in routing fees. Requires accurate pool state reading."
                </p>
                <ToggleSwitch
                    checked=move || config.get().use_direct_pool_calls
                    on_change=update_direct_pool
                    label="Use Direct Pool Calls (skip router fees)".to_string()
                />
                <p class="config-help">
                    "Try both directions: SUSHI->PANCAKE and PANCAKE->SUSHI to find better spreads."
                </p>
                <ToggleSwitch
                    checked=move || config.get().reverse_route
                    on_change=update_reverse_route
                    label="Enable Reverse Route Scanning".to_string()
                />
            </div>

            <div class="config-section">
                <h2>"Simulation"</h2>
                <p class="config-help">
                    "Simulate the transaction via Tenderly before broadcasting to catch reverts and estimate gas accurately."
                </p>
                <ToggleSwitch
                    checked=move || config.get().simulate_before_send
                    on_change=update_simulate
                    label="Simulate Before Send (Tenderly)".to_string()
                />
                <div class="config-field">
                    <label>"Tenderly API Key"</label>
                    <input
                        type="password"
                        prop:value=move || config.get().tenderly_api_key
                        on:input=update_tenderly_key
                        placeholder="tvly-..."
                    />
                </div>
                <div class="config-field">
                    <label>"Tenderly Project Slug"</label>
                    <input
                        type="text"
                        prop:value=move || config.get().tenderly_project_slug
                        on:input=update_tenderly_project
                        placeholder="my-project"
                    />
                </div>
            </div>

            <div class="config-section">
                <h2>"Schedule Settings"</h2>
                <ToggleSwitch
                    checked=move || config.get().auto_trade
                    on_change=update_auto_trade
                    label="Auto-trade Enabled".to_string()
                />
                <div class="config-row">
                    <div class="config-field">
                        <label>"Start Hour (UTC)"</label>
                        <input
                            type="number"
                            prop:value=move || config.get().daily_start_hour.to_string()
                            on:input=update_start_hour
                            min="0"
                            max="23"
                        />
                    </div>
                    <div class="config-field">
                        <label>"End Hour (UTC)"</label>
                        <input
                            type="number"
                            prop:value=move || config.get().daily_end_hour.to_string()
                            on:input=update_end_hour
                            min="0"
                            max="23"
                        />
                    </div>
                </div>
            </div>

            <div class="config-actions">
                <button
                    class="btn btn-primary"
                    on:click=move |_| save_config.dispatch(())
                    disabled=move || saving.get()
                >
                    {move || if saving.get() { "Saving..." } else { "Save Configuration" }}
                </button>
                {move || saved.get().then(|| view! {
                    <span class="save-indicator">"✓ Saved"</span>
                })}
            </div>
        </div>
    }
}