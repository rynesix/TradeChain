use std::str::FromStr;

use crate::config::CONFIG;
use anyhow::Context;
use log::info;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{bs58, signature::Keypair, signer::Signer};
use solana_sdk::{instruction::Instruction, transaction::Transaction};
use tokio::task;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloseTokenAccountResult {
    pub success: bool,
    pub token_mint: String,
    pub token_account: Option<String>,
    pub transaction_signature: Option<String>,
    pub message: String,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloseTokenAccountBatchRequest {
    pub wallet_pk: String,
    pub token_mints: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloseTokenAccountBatchResult {
    pub total_requested: usize,
    pub successful_closures: usize,
    pub failed_closures: usize,
    pub results: Vec<CloseTokenAccountResult>,
}

fn parse_keypair(private_key: &str) -> anyhow::Result<Keypair> {
    let bytes = bs58::decode(private_key)
        .into_vec()
        .context("invalid base58 key")?;
    if bytes.len() == 64 {
        Keypair::try_from(&bytes[..]).context("invalid keypair bytes")
    } else {
        anyhow::bail!("unexpected key length: {} bytes", bytes.len());
    }
}

/// Extract token account info from parsed data
fn extract_token_info(
    data: &solana_account_decoder_client_types::UiAccountData,
) -> Option<(String, f64)> {
    if let solana_account_decoder_client_types::UiAccountData::Json(parsed) = data {
        if let Some(parsed_account) = parsed.parsed.as_object() {
            if let Some(info) = parsed_account.get("info").and_then(|v| v.as_object()) {
                let mint = info.get("mint")?.as_str()?.to_string();
                let token_amount = info.get("tokenAmount")?.as_object()?;
                let ui_amount = token_amount.get("uiAmount")?.as_f64()?;
                return Some((mint, ui_amount));
            }
        }
    }
    None
}

/// Create close account instruction for SPL token account
fn create_close_instruction(
    token_account: &Pubkey,
    owner: &Pubkey,
    destination: &Pubkey,
) -> Instruction {
    // SPL Token close account instruction
    let program_id = spl_token::id();

    Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(*token_account, false),
            solana_sdk::instruction::AccountMeta::new(*destination, false),
            solana_sdk::instruction::AccountMeta::new_readonly(*owner, true),
        ],
        data: vec![9], // Close account instruction discriminator
    }
}

/// Close specific token account for a given token mint
pub async fn close_token_account(
    wallet_pk: String,
    token_mint: String,
    job_id: Option<String>,
) -> Result<String, String> {
    info!(
        "Starting close_token_account for wallet - closing token account for mint: {}",
        token_mint
    );

    let rpc_url = CONFIG.api.helius_https.clone();
    let wallet_pk_clone = wallet_pk.clone();
    let token_mint_clone = token_mint.clone();

    let res = task::spawn_blocking(move || -> anyhow::Result<CloseTokenAccountResult> {
        let client = RpcClient::new(rpc_url);

        // Parse the wallet keypair
        let wallet_keypair = parse_keypair(&wallet_pk_clone)?;
        let wallet_pubkey = wallet_keypair.pubkey();

        // Parse token mint
        let mint_pubkey =
            Pubkey::from_str(&token_mint_clone).context("invalid token mint pubkey")?;

        // Update job progress if job_id provided - Starting (Step 1/5)
        if let Some(ref job_id) = job_id {
            let _ = crate::update_job_progress(
                &job_id,
                1,
                5,
                "Starting token account closure".to_string(),
            );
        }

        info!(
            "Processing wallet: {} for mint: {}",
            wallet_pubkey, mint_pubkey
        );
        info!(
            "Rent will be refunded to the same wallet: {}",
            wallet_pubkey
        );

        // Get all token accounts for this wallet
        let token_accounts = client
            .get_token_accounts_by_owner(
                &wallet_pubkey,
                TokenAccountsFilter::ProgramId(spl_token::id()),
            )
            .context("failed to get token accounts")?;

        // Update job progress if job_id provided - Searching for token account (Step 2/5)
        if let Some(ref job_id) = job_id {
            let _ = crate::update_job_progress(
                &job_id,
                2,
                5,
                format!(
                    "Found {} token accounts, searching for target mint",
                    token_accounts.len()
                ),
            );
        }

        info!(
            "Found {} token accounts for wallet {}",
            token_accounts.len(),
            wallet_pubkey
        );

        // Find the specific token account for the given mint
        let mut target_account_pubkey = None;
        let mut account_balance = 0.0;

        for token_account in token_accounts {
            let account_pubkey =
                Pubkey::from_str(&token_account.pubkey).context("invalid token account pubkey")?;

            if let Some((mint, ui_amount)) = extract_token_info(&token_account.account.data) {
                info!(
                    "Token account {} - Mint: {}, Balance: {}",
                    account_pubkey, mint, ui_amount
                );

                // Check if this is the token account for our target mint
                if mint == token_mint_clone {
                    info!(
                        "Found target token account {} for mint {} with balance {}",
                        account_pubkey, mint, ui_amount
                    );
                    target_account_pubkey = Some(account_pubkey);
                    account_balance = ui_amount;

                    // Update job progress if job_id provided - Found target account (Step 3/5)
                    if let Some(ref job_id) = job_id {
                        let _ = crate::update_job_progress(
                            &job_id,
                            3,
                            5,
                            format!("Found target token account with balance: {}", ui_amount),
                        );
                    }
                    break;
                }
            } else {
                info!("Could not parse token account data for {}", account_pubkey);
            }
        }

        let account_to_close = match target_account_pubkey {
            Some(account) => account,
            None => {
                let message = format!(
                    "No token account found for mint {} in wallet {}",
                    token_mint_clone, wallet_pubkey
                );
                info!("{}", message);

                // Update job progress if job_id provided - Account not found (Step 5/5 - Error)
                if let Some(ref job_id) = job_id {
                    let _ = crate::update_job_progress(
                        &job_id,
                        5,
                        5,
                        "Token account not found".to_string(),
                    );
                }

                return Ok(CloseTokenAccountResult {
                    success: false,
                    token_mint: token_mint_clone,
                    token_account: None,
                    transaction_signature: None,
                    message,
                    error: Some("Token account not found".to_string()),
                });
            }
        };

        info!(
            "Closing token account {} for mint {} from wallet {} (balance: {})",
            account_to_close, token_mint_clone, wallet_pubkey, account_balance
        );

        // Update job progress if job_id provided - Creating transaction (Step 4/5)
        if let Some(ref job_id) = job_id {
            let _ =
                crate::update_job_progress(&job_id, 4, 5, "Creating close transaction".to_string());
        }

        // Create close instruction (refund rent to same wallet)
        let close_instruction = create_close_instruction(
            &account_to_close,
            &wallet_pubkey,
            &wallet_pubkey, // Refund rent to the same wallet
        );

        let recent_blockhash = client
            .get_latest_blockhash()
            .context("failed to fetch blockhash")?;

        let signer_refs: Vec<&dyn Signer> = vec![&wallet_keypair as &dyn Signer];

        let tx = Transaction::new_signed_with_payer(
            &[close_instruction],
            Some(&wallet_pubkey),
            &signer_refs,
            recent_blockhash,
        );

        // Send transaction
        match client.send_and_confirm_transaction(&tx) {
            Ok(sig) => {
                let signature = sig.to_string();
                let message = format!(
                    "Token account {} for mint {} closed successfully (balance: {})",
                    account_to_close, token_mint_clone, account_balance
                );
                info!(
                    "Token account closure completed with signature: {} - {}",
                    signature, message
                );

                // Update job progress if job_id provided - Success (Step 5/5 - Complete)
                if let Some(ref job_id) = job_id {
                    let _ = crate::update_job_progress(
                        &job_id,
                        5,
                        5,
                        format!("Successfully closed token account: {}", signature),
                    );
                }

                Ok(CloseTokenAccountResult {
                    success: true,
                    token_mint: token_mint_clone,
                    token_account: Some(account_to_close.to_string()),
                    transaction_signature: Some(signature),
                    message,
                    error: None,
                })
            }
            Err(e) => {
                let error_msg = format!(
                    "Failed to close token account {} for mint {}: {}",
                    account_to_close, token_mint_clone, e
                );
                info!("{}", error_msg);

                // Update job progress if job_id provided - Error (Step 5/5 - Failed)
                if let Some(ref job_id) = job_id {
                    let _ = crate::update_job_progress(
                        &job_id,
                        5,
                        5,
                        format!("Failed to close account: {}", error_msg),
                    );
                }

                Ok(CloseTokenAccountResult {
                    success: false,
                    token_mint: token_mint_clone,
                    token_account: Some(account_to_close.to_string()),
                    transaction_signature: None,
                    message: error_msg.clone(),
                    error: Some(error_msg),
                })
            }
        }
    })
    .await
    .map_err(|e| format!("join error: {}", e))
    .and_then(|r| r.map_err(|e| format!("rpc error: {}", e)))?;

    // Serialize result as JSON for job system
    match serde_json::to_string(&res) {
        Ok(json_result) => Ok(json_result),
        Err(e) => Err(format!("Failed to serialize result: {}", e)),
    }
}

/// Close multiple token accounts in batch with progress tracking
pub async fn close_token_accounts_batch(
    request: CloseTokenAccountBatchRequest,
    job_id: Option<String>,
) -> Result<String, String> {
    let wallet_pk = request.wallet_pk;
    let token_mints = request.token_mints;
    let total_accounts = token_mints.len();

    if total_accounts == 0 {
        return Err("No token mints provided".to_string());
    }

    // Update job progress - Starting
    if let Some(ref job_id) = job_id {
        let _ = crate::update_job_progress(
            &job_id,
            0,
            total_accounts as u32,
            format!(
                "Starting batch closure of {} token accounts",
                total_accounts
            ),
        );
    }

    let mut results = Vec::new();
    let mut successful_closures = 0;
    let mut failed_closures = 0;

    // Process each token account
    for (index, token_mint) in token_mints.iter().enumerate() {
        info!(
            "Processing token account {}/{}: {}",
            index + 1,
            total_accounts,
            token_mint
        );

        // Update progress for current account
        if let Some(ref job_id) = job_id {
            let _ = crate::update_job_progress(
                &job_id,
                index as u32,
                total_accounts as u32,
                format!(
                    "Closing token account {}/{}: {}",
                    index + 1,
                    total_accounts,
                    token_mint
                ),
            );
        }

        // Call the individual close function without job_id to avoid nested progress tracking
        match close_token_account(wallet_pk.clone(), token_mint.clone(), None).await {
            Ok(result_json) => {
                match serde_json::from_str::<CloseTokenAccountResult>(&result_json) {
                    Ok(result) => {
                        if result.success {
                            successful_closures += 1;
                            info!("Successfully closed token account for mint: {}", token_mint);
                        } else {
                            failed_closures += 1;
                            info!(
                                "Failed to close token account for mint: {} - {}",
                                token_mint, result.message
                            );
                        }
                        results.push(result);
                    }
                    Err(e) => {
                        failed_closures += 1;
                        let error_result = CloseTokenAccountResult {
                            success: false,
                            token_mint: token_mint.clone(),
                            token_account: None,
                            transaction_signature: None,
                            message: format!("Failed to parse result: {}", e),
                            error: Some(format!("Parse error: {}", e)),
                        };
                        results.push(error_result);
                    }
                }
            }
            Err(e) => {
                failed_closures += 1;
                let error_result = CloseTokenAccountResult {
                    success: false,
                    token_mint: token_mint.clone(),
                    token_account: None,
                    transaction_signature: None,
                    message: format!("Failed to close token account: {}", e),
                    error: Some(e),
                };
                results.push(error_result);
            }
        }
    }

    // Final progress update
    if let Some(ref job_id) = job_id {
        let _ = crate::update_job_progress(
            &job_id,
            total_accounts as u32,
            total_accounts as u32,
            format!(
                "Completed batch closure: {} successful, {} failed",
                successful_closures, failed_closures
            ),
        );
    }

    let batch_result = CloseTokenAccountBatchResult {
        total_requested: total_accounts,
        successful_closures,
        failed_closures,
        results,
    };

    // Serialize result as JSON for job system
    match serde_json::to_string(&batch_result) {
        Ok(json_result) => Ok(json_result),
        Err(e) => Err(format!("Failed to serialize batch result: {}", e)),
    }
}
