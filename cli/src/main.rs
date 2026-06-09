use clap::{Parser, Subcommand};
use std::time::Duration;
use tokio::time;

mod blockchain;
mod config;
mod scanner;

use config::AppConfig;

#[derive(Parser)]
#[command(name = "balancer-arb")]
#[command(about = "Balancer V3 Flash Loan Arbitrage Bot for Polygon")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(short, long)]
        interval: Option<u64>,
        #[arg(short = 'a', long)]
        amount: Option<f64>,
    },
    Stop,
    Scan {
        #[arg(short = 'a', long)]
        amount: Option<f64>,
    },
    Simulate {
        #[arg(short = 'a', long)]
        amount: f64,
    },
    Execute {
        #[arg(short = 'a', long)]
        amount: f64,
        #[arg(short, long)]
        dry_run: bool,
    },
    Status,
    Balance,
    Withdraw {
        #[arg(short, long)]
        token: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cli = Cli::parse();
    let config = AppConfig::load();

    match cli.command {
        Commands::Start { interval, amount } => {
            let scan_interval = interval.unwrap_or(config.scan_interval_ms);
            let borrow_amount = amount.unwrap_or(config.borrow_amount);
            log::info!("Starting arbitrage bot with {}ms scan interval", scan_interval);
            log::info!("Flash loan amount: ${:.2}", borrow_amount);
            log::info!("Direct pool calls: {}", config.use_direct_pool_calls);
            log::info!("Simulate before send: {}", config.simulate_before_send);
            log::info!("Max slippage: {} bps", config.max_slippage_bps);
            log::info!("Max price impact: {}%", config.max_price_impact_pct);

            let mut interval = time::interval(Duration::from_millis(scan_interval));

            loop {
                interval.tick().await;

                let current_hour = chrono::Utc::now().hour();
                if config.auto_trade && (current_hour < config.daily_start_hour || current_hour > config.daily_end_hour) {
                    log::info!("Outside trading hours ({}:00 - {}:00 UTC)", config.daily_start_hour, config.daily_end_hour);
                    continue;
                }

                let mut scan_config = config.clone();
                scan_config.borrow_amount = borrow_amount;

                match scanner::scan_opportunity_with_amount(&scan_config, Some(borrow_amount)).await {
                    Ok(Some(opportunity)) => {
                        log::info!(
                            "Opportunity: {} | Route: {} | Gross: ${:.4} | Impact: {}bps | Net: ${:.4}",
                            opportunity.token_pair,
                            opportunity.route_description,
                            opportunity.expected_profit,
                            opportunity.price_impact_bps,
                            opportunity.net_profit_after_costs,
                        );

                        if config.auto_trade {
                            match scanner::execute_trade(&config, &opportunity).await {
                                Ok(result) => {
                                    log::info!(
                                        "Trade executed! Profit: ${:.4} | Gas: ${:.4} | Slippage: {}bps | Simulated: {}",
                                        result.profit,
                                        result.gas_cost,
                                        result.actual_slippage_bps,
                                        result.simulated,
                                    );
                                }
                                Err(e) => {
                                    log::error!("Trade execution failed: {}", e);
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        log::debug!("No opportunity found");
                    }
                    Err(e) => {
                        log::warn!("Scan error: {}", e);
                    }
                }
            }
        }
        Commands::Stop => {
            log::info!("Bot stopped");
        }
        Commands::Scan { amount } => {
            let borrow_amount = amount.unwrap_or(config.borrow_amount);
            log::info!("Scanning for opportunities with ${:.2}...", borrow_amount);
            log::info!("Fee tiers: {:?}", config.fee_tiers);
            log::info!("Direct pool calls: {}", config.use_direct_pool_calls);

            let mut scan_config = config.clone();
            scan_config.borrow_amount = borrow_amount;

            match scanner::scan_opportunity_with_amount(&scan_config, Some(borrow_amount)).await {
                Ok(Some(opportunity)) => {
                    println!("=== Opportunity Found ===");
                    println!("  Token Pair: {}", opportunity.token_pair);
                    println!("  Buy on: {}", opportunity.dex_from);
                    println!("  Sell on: {}", opportunity.dex_to);
                    println!("  Borrow Amount: ${:.2}", opportunity.amount);
                    println!("  Gross Profit: ${:.4}", opportunity.expected_profit);
                    println!("  Price Impact: {} bps ({:.2}%)", opportunity.price_impact_bps, opportunity.price_impact_bps as f64 / 100.0);
                    println!("  Slippage: {} bps ({:.2}%)", opportunity.slippage_bps, opportunity.slippage_bps as f64 / 100.0);
                    println!("  Net Profit: ${:.4}", opportunity.net_profit_after_costs);
                    println!("  Route: {}", opportunity.route_description);
                    println!("  Pool Liquidity: ${:.2}", opportunity.pool_liquidity_usd);
                }
                Ok(None) => {
                    println!("No profitable opportunity found");
                    println!("Current fee tiers: {:?}", config.fee_tiers);
                    println!("Max price impact: {}%", config.max_price_impact_pct);
                }
                Err(e) => {
                    eprintln!("Scan error: {}", e);
                }
            }
        }
        Commands::Simulate { amount } => {
            log::info!("Simulating trade with ${:.2}...", amount);

            let mut test_config = config.clone();
            test_config.borrow_amount = amount;

            match scanner::scan_opportunity_with_amount(&test_config, Some(amount)).await {
                Ok(Some(opportunity)) => {
                    println!("=== Simulation Results ===");
                    println!("  Input: ${:.2} USDC", amount);
                    println!("  Route: {}", opportunity.route_description);
                    println!("  Buy: {} at {}", opportunity.amount, opportunity.dex_from);
                    println!("  Sell at: {}", opportunity.dex_to);
                    println!("  Expected Gross Profit: ${:.4}", opportunity.expected_profit);
                    println!("  Price Impact: {} bps ({:.2}%)", opportunity.price_impact_bps, opportunity.price_impact_bps as f64 / 100.0);
                    println!("  Slippage Limit: {} bps ({:.2}%)", opportunity.slippage_bps, opportunity.slippage_bps as f64 / 100.0);
                    println!("  Net Profit (after gas+slippage): ${:.4}", opportunity.net_profit_after_costs);

                    if opportunity.net_profit_after_costs < test_config.min_profit_usd {
                        println!("\n  ⚠ Warning: Net profit below minimum ${:.2}", test_config.min_profit_usd);
                    } else {
                        println!("\n  ✓ Trade would be profitable");
                    }

                    if opportunity.price_impact_bps > (test_config.max_price_impact_pct * 100.0) as u32 {
                        println!("  ⚠ Warning: Price impact exceeds maximum {}%", test_config.max_price_impact_pct);
                    }
                }
                Ok(None) => {
                    println!("No profitable opportunity found for ${:.2}", amount);
                    println!("\nPossible reasons:");
                    println!("  - Spread too small after gas costs");
                    println!("  - Price impact exceeds {}%", test_config.max_price_impact_pct);
                    println!("  - Pools have insufficient liquidity");
                }
                Err(e) => {
                    eprintln!("Simulation error: {}", e);
                }
            }
        }
        Commands::Execute { amount, dry_run } => {
            if dry_run {
                log::info!("DRY RUN: Would execute trade with ${:.2}...", amount);
                let mut test_config = config.clone();
                test_config.borrow_amount = amount;

                match scanner::scan_opportunity_with_amount(&test_config, Some(amount)).await {
                    Ok(Some(opportunity)) => {
                        println!("=== Dry Run Results ===");
                        println!("  Would execute: {}", opportunity.route_description);
                        println!("  Expected net profit: ${:.4}", opportunity.net_profit_after_costs);
                        println!("  Gas cost included: yes");
                        println!("  Slippage protection: {} bps", opportunity.slippage_bps);
                    }
                    Ok(None) => println!("No opportunity found"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                log::info!("Executing trade with ${:.2}...", amount);

                let mut exec_config = config.clone();
                exec_config.borrow_amount = amount;

                match scanner::scan_opportunity_with_amount(&exec_config, Some(amount)).await {
                    Ok(Some(opportunity)) => {
                        match scanner::execute_trade(&exec_config, &opportunity).await {
                            Ok(result) => {
                                println!("=== Trade Executed ===");
                                println!("  Profit: ${:.4}", result.profit);
                                println!("  Gas Cost: ${:.4}", result.gas_cost);
                                println!("  Actual Slippage: {} bps", result.actual_slippage_bps);
                                println!("  Simulated: {}", result.simulated);
                            }
                            Err(e) => {
                                eprintln!("Trade failed: {}", e);
                            }
                        }
                    }
                    Ok(None) => {
                        println!("No profitable opportunity found for ${:.2}", amount);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
        Commands::Status => {
            let balance = blockchain::get_contract_balance(&config).await?;
            println!("=== Bot Status ===");
            println!("  Network: Polygon");
            println!("  Contract: {}", config.contract_address);
            println!("  Contract Balance: ${:.2} USDC", balance);
            println!("  Flash Loan Amount: ${:.2}", config.borrow_amount);
            println!("  Min Borrow: ${:.2}", config.min_borrow_amount);
            println!("  Max Borrow: ${:.2}", config.max_borrow_amount);
            println!("  Borrow Step: ${:.2}", config.borrow_step);
            println!("  Min Profit: ${:.2}", config.min_profit_usd);
            println!("  Max Gas: {} Gwei", config.max_gas_price_gwei);
            println!("  Max Slippage: {} bps", config.max_slippage_bps);
            println!("  Max Price Impact: {}%", config.max_price_impact_pct);
            println!("  Auto-trade: {}", config.auto_trade);
            println!("  Trading Hours: {}:00 - {}:00 UTC", config.daily_start_hour, config.daily_end_hour);
            println!("  Direct Pool Calls: {}", config.use_direct_pool_calls);
            println!("  Simulate Before Send: {}", config.simulate_before_send);
            println!("  Fee Tiers: {:?}", config.fee_tiers);
            println!("  Reverse Route: {}", config.reverse_route);
        }
        Commands::Balance => {
            let balance = blockchain::get_contract_balance(&config).await?;
            println!("Contract USDC Balance: ${:.2}", balance);
        }
        Commands::Withdraw { token } => {
            log::info!("Withdrawing profits for token {}...", token);
            blockchain::withdraw_profits(&config, &token).await?;
            println!("Withdrawal successful!");
        }
    }

    Ok(())
}