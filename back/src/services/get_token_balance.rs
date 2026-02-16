use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json;
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenBalance {
    pub balance: f64,
    pub balance_raw: u64,
    pub decimals: u8,
    pub token_account: String,
    pub mint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTokenBalanceResult {
    pub wallet: String,
    pub balances: Vec<TokenBalance>,
    pub total_tokens: usize,
}

/// Extract mint from token account debug string (fallback method)
fn extract_mint_from_debug_string(debug_str: &str) -> Option<String> {
    // Look for mint pattern in debug string: "mint": String("...")
    if let Some(start) = debug_str.find(r#""mint": String(""#) {
        let after_start = &debug_str[start + r#""mint": String(""#.len()..];
        if let Some(end) = after_start.find('"') {
            return Some(after_start[..end].to_string());
        }
    }
    None
}

/// Extract mint from parsed JSON token account data
fn extract_mint_from_token_account(
    token_account: &solana_client::rpc_response::RpcKeyedAccount,
) -> Option<String> {
    // Extract mint from debug string representation
    let debug_str = format!("{:?}", token_account);
    extract_mint_from_debug_string(&debug_str)
}

async fn get_token_balance(
    wallet: String,
    mint: Option<String>,
) -> Result<GetTokenBalanceResult, anyhow::Error> {
    let wallet_pubkey = Pubkey::from_str(&wallet).unwrap();

    info!("Getting token balance for wallet: {}", wallet_pubkey);

    let mut token_balances = Vec::new();

    match mint {
        Some(specific_mint) => {
            let mint = Pubkey::from_str(&specific_mint).ok();
            let mint_clone = mint.unwrap().clone();
            info!("Getting balance for specific token: {}", mint_clone);
            let rpc_url = "https://api.mainnet-beta.solana.com";
            let client = RpcClient::new(rpc_url.to_string());
            let token_accounts = client.get_token_accounts_by_owner(
                &wallet_pubkey,
                TokenAccountsFilter::Mint(mint_clone),
            )?;

            if token_accounts.is_empty() {
                info!("No token account found for mint: {}", mint_clone);
                return Ok(GetTokenBalanceResult {
                    wallet: wallet_pubkey.to_string(),
                    balances: vec![],
                    total_tokens: 0,
                });
            }

            for token_account in token_accounts {
                let token_account_pubkey = Pubkey::from_str(&token_account.pubkey)?;

                let token_account_balance =
                    client.get_token_account_balance(&token_account_pubkey)?;

                let balance_ui = token_account_balance.ui_amount.unwrap_or(0.0);
                let balance_raw = token_account_balance.amount.parse::<u64>().unwrap_or(0);
                let decimals = token_account_balance.decimals;

                // Extract mint from parsed JSON data
                let extracted_mint = extract_mint_from_token_account(&token_account);

                info!("token account : {:?}", token_account);
                if let Some(mint) = &extracted_mint {
                    info!("Extracted mint from JSON: {}", mint);
                }

                info!(
                    "Token account {}: {} tokens ({} raw units, {} decimals)",
                    token_account_pubkey, balance_ui, balance_raw, decimals
                );

                token_balances.push(TokenBalance {
                    balance: balance_ui,
                    balance_raw,
                    decimals,
                    token_account: token_account_pubkey.to_string(),
                    mint: extracted_mint,
                });
            }
        }

        None => {
            info!("Getting all token balances for wallet");
            let rpc_url = "https://api.mainnet-beta.solana.com";
            let client = RpcClient::new(rpc_url.to_string());
            let token_accounts = client.get_token_accounts_by_owner(
                &wallet_pubkey,
                TokenAccountsFilter::ProgramId(spl_token::id()),
            )?;

            if token_accounts.is_empty() {
                info!("No token account found");
                return Ok(GetTokenBalanceResult {
                    wallet: wallet_pubkey.to_string(),
                    balances: vec![],
                    total_tokens: 0,
                });
            }
            for token_account in token_accounts {
                let token_account_pubkey = Pubkey::from_str(&token_account.pubkey)?;

                let token_account_balance =
                    client.get_token_account_balance(&token_account_pubkey)?;
                let balance_ui = token_account_balance.ui_amount.unwrap_or(0.0);
                let balance_raw = token_account_balance.amount.parse::<u64>().unwrap_or(0);
                let decimals = token_account_balance.decimals;

                // Extract mint from parsed JSON data
                let extracted_mint = extract_mint_from_token_account(&token_account);

                info!("token account : {:?}", token_account);
                if let Some(mint) = &extracted_mint {
                    info!("Extracted mint from JSON: {}", mint);
                }

                info!(
                    "Token account {}: {} tokens ({} raw units, {} decimals)",
                    token_account_pubkey, balance_ui, balance_raw, decimals
                );

                token_balances.push(TokenBalance {
                    balance: balance_ui,
                    balance_raw,
                    decimals,
                    token_account: token_account_pubkey.to_string(),
                    mint: extracted_mint,
                });
            }
        }
    }

    let result = GetTokenBalanceResult {
        wallet: wallet_pubkey.to_string(),
        balances: token_balances.clone(),
        total_tokens: token_balances.len(),
    };

    info!(
        "Token balance summary for wallet {}: {} different tokens found",
        wallet_pubkey, result.total_tokens
    );

    Ok(result)
}
async fn get_tokens_to(wallet: String) -> Result<GetTokenBalanceResult, anyhow::Error> {
    let wallet_pubkey = Pubkey::from_str(&wallet).unwrap();

    info!("Getting token balance for wallet: {}", wallet_pubkey);

    let mut token_balances = Vec::new();

    info!("Getting all token balances for wallet");
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());
    let token_accounts = client.get_token_accounts_by_owner(
        &wallet_pubkey,
        TokenAccountsFilter::ProgramId(spl_token::id()),
    )?;

    if token_accounts.is_empty() {
        info!("No token account found");
        return Ok(GetTokenBalanceResult {
            wallet: wallet_pubkey.to_string(),
            balances: vec![],
            total_tokens: 0,
        });
    }
    for token_account in token_accounts {
        // Extract mint from parsed JSON data
        let extracted_mint = extract_mint_from_token_account(&token_account);

        info!("token account : {:?}", token_account);
        if let Some(mint) = &extracted_mint {
            info!("Extracted mint from JSON: {}", mint);
        }

        token_balances.push(TokenBalance {
            balance: 0.0,
            balance_raw: 0,
            decimals: 6,
            token_account: "".to_string(),
            mint: extracted_mint,
        });
    }

    let result = GetTokenBalanceResult {
        wallet: wallet_pubkey.to_string(),
        balances: token_balances.clone(),
        total_tokens: token_balances.len(),
    };

    info!(
        "Token balance summary for wallet {}: {} different tokens found",
        wallet_pubkey, result.total_tokens
    );

    Ok(result)
}

pub async fn get_single_token_balance(wallet: String, mint: Option<String>) -> Result<f64, String> {
    let result = get_token_balance(wallet, mint)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(token_balance) = result.balances.first() {
        Ok(token_balance.clone().balance)
    } else {
        Err("No token balance found for the specified mint".to_string())
    }
}
pub async fn get_tokens_balances(wallet: String) -> Result<GetTokenBalanceResult, String> {
    get_token_balance(wallet, None)
        .await
        .map_err(|e| e.to_string())
}
pub async fn get_tokens(wallet: String) -> Result<GetTokenBalanceResult, String> {
    get_tokens_to(wallet).await.map_err(|e| e.to_string())
}

/// Public function to extract mint from a token account
pub fn extract_mint_from_account_debug(account_debug: &str) -> Option<String> {
    extract_mint_from_debug_string(account_debug)
}

/// Get tokens balances with progress tracking for multiple wallets
pub async fn get_tokens_balances_batch_with_progress(
    wallets: Vec<String>,
    job_id: String,
    job_manager: crate::jobs::JobManager,
) -> Result<std::collections::HashMap<String, GetTokenBalanceResult>, String> {
    use std::collections::HashMap;

    if wallets.is_empty() {
        return Err("No wallets provided".to_string());
    }

    info!(
        "Starting batch token balances fetch for {} wallets",
        wallets.len()
    );

    // Initialize progress
    job_manager.set_total_items(&job_id, wallets.len() as u32);
    job_manager.update_progress(
        &job_id,
        0.0,
        Some("Starting batch token balance fetch".to_string()),
    );

    let mut results = HashMap::new();

    for (idx, wallet) in wallets.iter().enumerate() {
        let step_msg = format!(
            "Fetching tokens for wallet {} of {}",
            idx + 1,
            wallets.len()
        );
        job_manager.update_progress_items(
            &job_id,
            idx as u32,
            wallets.len() as u32,
            Some(step_msg),
        );

        info!("Fetching tokens for wallet: {}", wallet);

        match get_tokens_balances(wallet.clone()).await {
            Ok(result) => {
                results.insert(wallet.clone(), result);
                info!("✅ Successfully fetched tokens for wallet: {}", wallet);
            }
            Err(error) => {
                warn!("❌ Failed to fetch tokens for wallet {}: {}", wallet, error);
                // Insert empty result for failed wallets
                results.insert(
                    wallet.clone(),
                    GetTokenBalanceResult {
                        wallet: wallet.clone(),
                        balances: vec![],
                        total_tokens: 0,
                    },
                );
            }
        }
    }

    // Final progress update with results in the job result
    let results_json = serde_json::to_string(&results).unwrap_or_else(|e| {
        warn!("Failed to serialize results: {}", e);
        format!("{{\"error\": \"Failed to serialize results: {}\"}}", e)
    });

    job_manager.update_progress(
        &job_id,
        100.0,
        Some("Batch token balance fetch completed".to_string()),
    );

    // Store results in job result for frontend to access
    job_manager.set_job_result(&job_id, results_json);

    info!(
        "Completed batch token balances fetch for {} wallets",
        wallets.len()
    );
    Ok(results)
}
