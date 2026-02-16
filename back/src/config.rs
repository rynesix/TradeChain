use log::{info, warn};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

// Configuration for external APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub helius_https: String,
    pub helius_ws: String,
    pub mongodb_uri: String,
}

// Configuration for transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionConfig {
    pub private_key1: String,
    pub private_key2: String,
    pub private_key3: String,
    pub private_key4: String,
    pub private_key5: String,
    pub private_key6: String,
    pub private_key7: String,
    pub sol_to_spend: f64,
    pub jwt_token: String,
    pub slippage: f64,
}

// Configuration for transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeesLamports {
    pub micro_lamports: u64,
    pub units_limit: u32,
}

// Configuration for metrics and monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub metrics_min_token_fund: u64,
    pub metrics_max_token_fund: u64,
    pub min_sol_variation_percent: f64,
    pub max_stable_iterations: u32,
}

// Configuration for trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingConfig {
    pub threads_sniper: u32,
    pub threads_holders_watcher: u32,
    pub threads_tx_watcher: u32,

    pub buy_fees_lamports: FeesLamports,
    pub sell_fees_lamports: FeesLamports,

    pub min_token_fund: u64,
    pub min_buy_amount: u64,
    pub max_buy_amount: u64,

    pub holders_watcher_timeout: u64,
    pub holders_watcher_min_count: u64,
    pub holders_min_token_fund: u64,
    pub holders_max_token_fund: u64,
    pub min_holders: u64,

    pub profit_percentage: f64,
    pub lost_profit_pourcentage: f64,

    pub sell_iteration_max: u32,
    pub sell_iteration_sleep: u64,
}

// Main structure grouping all configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub transaction: TransactionConfig,
    pub metrics: MetricsConfig,
    pub trading: TradingConfig,
}

// Globally accessible configuration singleton
pub static CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    Arc::new(AppConfig::load_from_env().unwrap_or_else(|e| {
        warn!(
            "Error loading config from environment: {}. Using defaults.",
            e
        );
        AppConfig::default()
    }))
});

impl AppConfig {
    // Load configuration from environment variables
    pub fn load_from_env() -> Result<Self, anyhow::Error> {
        // Load dotenv for local tests
        if let Err(e) = dotenv::dotenv() {
            warn!("Failed to load .env file: {}", e);
        }

        let config = AppConfig {
            api: ApiConfig {
                helius_https: env::var("HELIUS_HTTPS")
                    .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
                helius_ws: env::var("HELIUS_WS")
                    .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string()),
                mongodb_uri: env::var("MONGODB_URI")
                    .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            },
            transaction: TransactionConfig {
                jwt_token: env::var("JWT_TOKEN").unwrap_or_else(|_| {
                    warn!("JWT_TOKEN not set in environment variables!");
                    "".to_string()
                }),
                private_key1: env::var("PRIVATE_KEY").unwrap_or_else(|_| {
                    warn!("PRIVATE_KEY not set in environment variables!");
                    "".to_string()
                }),
                private_key2: env::var("PRIVATE_KEY2").unwrap_or_else(|_| {
                    warn!("PRIVATE_KEY2 not set in environment variables!");
                    "".to_string()
                }),
                private_key3: env::var("PRIVATE_KEY3").unwrap_or_else(|_| {
                    warn!("PRIVATE_KEY3 not set in environment variables!");
                    "".to_string()
                }),
                private_key4: env::var("PRIVATE_KEY4").unwrap_or_else(|_| {
                    warn!("PRIVATE_KEY4 not set in environment variables!");
                    "".to_string()
                }),
                private_key5: env::var("PRIVATE_KEY5").unwrap_or_else(|_| {
                    warn!("PRIVATE_KEY5 not set in environment variables!");
                    "".to_string()
                }),
                private_key6: env::var("PRIVATE_KEY6").unwrap_or_else(|_| {
                    warn!("PRIVATE_KEY6 not set in environment variables!");
                    "".to_string()
                }),
                private_key7: env::var("PRIVATE_KEY7").unwrap_or_else(|_| {
                    warn!("PRIVATE_KEY7 not set in environment variables!");
                    "".to_string()
                }),
                sol_to_spend: env::var("SOL_TO_SPEND")
                    .unwrap_or_else(|_| "0.01".to_string())
                    .parse::<f64>()
                    .unwrap_or(0.01),
                slippage: env::var("SLIPPAGE")
                    .unwrap_or_else(|_| "0.05".to_string())
                    .parse::<f64>()
                    .unwrap_or(0.05),
            },
            metrics: MetricsConfig {
                metrics_min_token_fund: env::var("METRICS_MIN_TOKEN_FUND")
                    .unwrap_or_else(|_| "5000000000".to_string())
                    .parse::<u64>()
                    .unwrap_or(5000000000),
                metrics_max_token_fund: env::var("METRICS_MAX_TOKEN_FUND")
                    .unwrap_or_else(|_| "70000000000".to_string())
                    .parse::<u64>()
                    .unwrap_or(70000000000),
                min_sol_variation_percent: env::var("MIN_SOL_VARIATION_PERCENT")
                    .unwrap_or_else(|_| "0.001".to_string())
                    .parse::<f64>()
                    .unwrap_or(0.001),
                max_stable_iterations: env::var("MAX_STABLE_ITERATIONS")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse::<u32>()
                    .unwrap_or(20),
            },
            trading: TradingConfig {
                threads_sniper: env::var("THREADS_SNIPER")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse::<u32>()
                    .unwrap_or(5),
                threads_holders_watcher: env::var("THREADS_HOLDERS_WATCHER")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse::<u32>()
                    .unwrap_or(10),
                threads_tx_watcher: env::var("THREADS_TX_WATCHER")
                    .unwrap_or_else(|_| "15".to_string())
                    .parse::<u32>()
                    .unwrap_or(15),

                buy_fees_lamports: FeesLamports {
                    micro_lamports: env::var("BUY_MICRO_LAMPORTS")
                        .unwrap_or_else(|_| "10000".to_string())
                        .parse::<u64>()
                        .unwrap_or(10000),
                    units_limit: env::var("BUY_UNITS_LIMIT")
                        .unwrap_or_else(|_| "10000".to_string())
                        .parse::<u32>()
                        .unwrap_or(10000),
                },
                sell_fees_lamports: FeesLamports {
                    micro_lamports: env::var("SELL_MICRO_LAMPORTS")
                        .unwrap_or_else(|_| "10000".to_string())
                        .parse::<u64>()
                        .unwrap_or(10000),
                    units_limit: env::var("SELL_UNITS_LIMIT")
                        .unwrap_or_else(|_| "10000".to_string())
                        .parse::<u32>()
                        .unwrap_or(10000),
                },

                holders_watcher_min_count: env::var("HOLDERS_WATCHER_MIN_COUNT")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse::<u64>()
                    .unwrap_or(10),
                holders_watcher_timeout: env::var("HOLDERS_WATCHER_TIMEOUT")
                    .unwrap_or_else(|_| "50".to_string())
                    .parse::<u64>()
                    .unwrap_or(50),

                holders_min_token_fund: env::var("HOLDERS_MIN_TOKEN_FUND")
                    .unwrap_or_else(|_| "5000000000".to_string())
                    .parse::<u64>()
                    .unwrap_or(5000000000),
                holders_max_token_fund: env::var("HOLDERS_MAX_TOKEN_FUND")
                    .unwrap_or_else(|_| "70000000000".to_string())
                    .parse::<u64>()
                    .unwrap_or(70000000000),

                min_token_fund: env::var("MIN_TOKEN_FUND")
                    .unwrap_or_else(|_| "3000000000".to_string())
                    .parse::<u64>()
                    .unwrap_or(3000000000),
                min_buy_amount: env::var("MIN_BUY_AMOUNT")
                    .unwrap_or_else(|_| "3000000000".to_string())
                    .parse::<u64>()
                    .unwrap_or(3000000000),
                max_buy_amount: env::var("MAX_BUY_AMOUNT")
                    .unwrap_or_else(|_| "70000000000".to_string())
                    .parse::<u64>()
                    .unwrap_or(70000000000),
                profit_percentage: env::var("PROFIT_PERCENTAGE")
                    .unwrap_or_else(|_| "15".to_string())
                    .parse::<f64>()
                    .unwrap_or(15.0),
                lost_profit_pourcentage: env::var("LOST_PROFIT_PERCENTAGE")
                    .unwrap_or_else(|_| "15".to_string())
                    .parse::<f64>()
                    .unwrap_or(15.0),
                sell_iteration_max: env::var("SELL_ITERATION_MAX")
                    .unwrap_or_else(|_| "250".to_string())
                    .parse::<u32>()
                    .unwrap_or(250),
                sell_iteration_sleep: env::var("SELL_ITERATION_SLEEP")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse::<u64>()
                    .unwrap_or(100),
                min_holders: env::var("MIN_HOLDERS")
                    .unwrap_or_else(|_| "12".to_string())
                    .parse::<u64>()
                    .unwrap_or(12),
            },
        };

        info!("Configuration loaded successfully");
        Ok(config)
    }

    // Provides a default configuration in case of error
    fn default() -> Self {
        AppConfig {
            api: ApiConfig {
                helius_https: "https://mainnet.helius-rpc.com/?api-key=default".to_string(),
                helius_ws: "wss://mainnet.helius-rpc.com/?api-key=default".to_string(),
                mongodb_uri: "mongodb://localhost:27017".to_string(),
            },
            transaction: TransactionConfig {
                jwt_token: "".to_string(),
                private_key1: "".to_string(),
                private_key2: "".to_string(),
                private_key3: "".to_string(),
                private_key4: "".to_string(),
                private_key5: "".to_string(),
                private_key6: "".to_string(),
                private_key7: "".to_string(),
                sol_to_spend: 0.01,
                slippage: 0.05,
            },
            metrics: MetricsConfig {
                metrics_min_token_fund: 5000000000,
                metrics_max_token_fund: 70000000000,
                min_sol_variation_percent: 0.001,
                max_stable_iterations: 20,
            },
            trading: TradingConfig {
                threads_sniper: 5,
                threads_holders_watcher: 10,
                threads_tx_watcher: 15,
                buy_fees_lamports: FeesLamports {
                    micro_lamports: 10000,
                    units_limit: 10000,
                },
                sell_fees_lamports: FeesLamports {
                    micro_lamports: 10000,
                    units_limit: 10000,
                },
                holders_watcher_min_count: 10,
                holders_watcher_timeout: 50,
                holders_min_token_fund: 5000000000,
                holders_max_token_fund: 70000000000,
                min_token_fund: 3000000000,
                min_buy_amount: 3000000000,
                max_buy_amount: 70000000000,
                profit_percentage: 15.0,
                sell_iteration_max: 250,
                sell_iteration_sleep: 100,
                min_holders: 12,
                lost_profit_pourcentage: 10.0,
            },
        }
    }
}
