use leptos::*;

use crate::api;
use crate::components::*;

#[component]
pub fn ContractDeploy() -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);
    let (deploying, set_deploying) = create_signal(false);
    let (deployed, set_deployed) = create_signal(false);
    let (contract_addr, set_contract_addr) = create_signal(String::new());
    let (balance, set_balance) = create_signal(0.0_f64);

    let deploy_contract = create_action(move |_| {
        let set_deploying = set_deploying.clone();
        let set_deployed = set_deployed.clone();
        let set_contract_addr = set_contract_addr.clone();
        let set_error = set_error.clone();
        async move {
            set_deploying.set(true);
            match api::get_contract_balance().await {
                Ok(bal) => {
                    set_balance.set(bal);
                    set_deployed.set(true);
                    set_deploying.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_deploying.set(false);
                }
            }
        }
    });

    let refresh_balance = create_action(move |_| {
        let set_balance = set_balance.clone();
        let set_error = set_error.clone();
        async move {
            match api::get_contract_balance().await {
                Ok(bal) => set_balance.set(bal),
                Err(e) => set_error.set(Some(e)),
            }
        }
    });

    let withdraw_profits = create_action(move |_| {
        let set_error = set_error.clone();
        async move {
            if let Err(e) = api::withdraw_profits("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174").await {
                set_error.set(Some(e));
            }
        }
    });

    view! {
        <div class="contract-page">
            <h1>"Contract Management"</h1>

            {move || error.get().map(|e| view! { <ErrorBanner error=e/> })}

            <div class="contract-info">
                <h2>"Deployed Contract"</h2>
                <div class="config-field">
                    <label>"Contract Address"</label>
                    <input
                        type="text"
                        prop:value=move || contract_addr.get()
                        on:input=move |ev| set_contract_addr.set(event_target_value(&ev))
                        placeholder="Enter deployed contract address"
                    />
                </div>
            </div>

            <div class="contract-actions">
                <button
                    class="btn btn-primary"
                    on:click=move |_| deploy_contract.dispatch(())
                    disabled=move || deploying.get()
                >
                    {move || if deploying.get() { "Deploying..." } else { "Check Balance" }}
                </button>
                <button
                    class="btn btn-secondary"
                    on:click=move |_| refresh_balance.dispatch(())
                >
                    "Refresh Balance"
                </button>
                <button
                    class="btn btn-warning"
                    on:click=move |_| withdraw_profits.dispatch(())
                >
                    "Withdraw Profits"
                </button>
            </div>

            <div class="contract-balance">
                <StatusCard
                    title="USDC Balance".to_string()
                    value=move || format!("${:.2}", balance.get())
                    subtitle=Some("In Contract".to_string())
                    class="card-info".to_string()
                />
            </div>

            <div class="contract-details">
                <h2>"Deployment Guide"</h2>
                <div class="guide-steps">
                    <div class="guide-step">
                        <h3>"1. Install Foundry"</h3>
                        <code>"curl -L https://foundry.paradigm.xyz | bash && foundryup"</code>
                    </div>
                    <div class="guide-step">
                        <h3>"2. Deploy Contract"</h3>
                        <code>"PRIVATE_KEY=0x... forge script script/Deploy.s.sol --rpc-url https://polygon-rpc.com --broadcast"</code>
                    </div>
                    <div class="guide-step">
                        <h3>"3. Verify Contract"</h3>
                        <code>"forge verify-contract <address> BalancerFlashLoanArbitrage --chain-id 137"</code>
                    </div>
                    <div class="guide-step">
                        <h3>"4. Fund Contract"</h3>
                        <code>"Send MATIC to the contract for gas, then use the bot to execute flash loans"</code>
                    </div>
                </div>
            </div>

            <div class="gas-estimates">
                <h2>"Gas Cost Estimates (Polygon)"</h2>
                <table class="gas-table">
                    <thead>
                        <tr>
                            <th>"Operation"</th>
                            <th>"Estimated Gas"</th>
                            <th>"Est. Cost (USD)"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Contract Deployment"</td>
                            <td>"~3,000,000"</td>
                            <td>"$0.10 - $0.30"</td>
                        </tr>
                        <tr>
                            <td>"Flash Loan + Swap"</td>
                            <td>"~500,000"</td>
                            <td>"$0.02 - $0.05"</td>
                        </tr>
                        <tr>
                            <td>"Flash Loan + 2 Swaps"</td>
                            <td>"~750,000"</td>
                            <td>"$0.03 - $0.08"</td>
                        </tr>
                        <tr>
                            <td>"Withdraw Profits"</td>
                            <td>"~100,000"</td>
                            <td>"$0.005 - $0.01"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}