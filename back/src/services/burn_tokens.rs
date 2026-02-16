use crate::config::CONFIG;
use crate::jobs::JobManager;
use anyhow::Context;
use log::{error, info};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{bs58, signature::Keypair, signer::Signer};
use solana_sdk::{instruction::Instruction, transaction::Transaction};
use std::str::FromStr;
use tokio::task;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BurnEachTokenResult {
    pub success: bool,
    pub total_mints: u32,
    pub successful_burns: u32,
    pub failed_burns: u32,
    pub burn_results: Vec<BurnTokenResult>,
    pub message: String,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BurnTokenResult {
    pub mint: String,
    pub success: bool,
    pub token_accounts_processed: u32,
    pub total_tokens_burned: f64,
    pub transaction_signatures: Vec<String>,
    pub error: Option<String>,
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
) -> Option<(String, f64, u8)> {
    if let solana_account_decoder_client_types::UiAccountData::Json(parsed) = data {
        if let Some(parsed_account) = parsed.parsed.as_object() {
            if let Some(info) = parsed_account.get("info").and_then(|v| v.as_object()) {
                let mint = info.get("mint")?.as_str()?.to_string();
                let token_amount = info.get("tokenAmount")?.as_object()?;
                let ui_amount = token_amount.get("uiAmount")?.as_f64()?;
                let decimals = token_amount.get("decimals")?.as_u64()? as u8;
                return Some((mint, ui_amount, decimals));
            }
        }
    }
    None
}

/// Create burn instruction for SPL token
fn create_burn_instruction(
    token_account: &Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
    amount: u64,
) -> Instruction {
    // SPL Token burn instruction
    let program_id = spl_token::id();

   // This is private code
   // Please contact me if you need it.
}

pub async fn burn_tokens(
    wallet_pk: String,
    mint_address: String,
    burn_percentage: f64,
) -> Result<String, String> {
    if burn_percentage <= 0.0 || burn_percentage > 100.0 {
        return Err("Burn percentage must be between 0.1 and 100.0".to_string());
    }

    info!(
        "Starting burn_tokens for wallet {} - burning {}% of mint {}",
        wallet_pk, burn_percentage, mint_address
    );

    let rpc_url = CONFIG.api.helius_https.clone();
    let wallet_pk_clone = wallet_pk.clone();
    let mint_address_clone = mint_address.clone();

    // This is private code.
    // Please contact me if you need it.

    // Return summary of all transaction signatures
    Ok(format!(
        "Successfully burned {}% of tokens for mint {}. Transactions: {}",
        burn_percentage,
        mint_address,
        res.join(", ")
    ))
}

/// Burn tokens with progress tracking
pub async fn burn_tokens_with_progress(
    wallet_pk: String,
    mint_address: String,
    burn_percentage: f64,
    job_id: String,
    job_manager: JobManager,
) -> Result<String, String> {
    if burn_percentage <= 0.0 || burn_percentage > 100.0 {
        return Err("Burn percentage must be between 0.1 and 100.0".to_string());
    }

    info!(
        "Starting burn_tokens_with_progress for wallet {} - burning {}% of mint {}",
        wallet_pk, burn_percentage, mint_address
    );

    // Update progress - starting
    
    // This is private code


    let rpc_url = CONFIG.api.helius_https.clone();
    let wallet_pk_clone = wallet_pk.clone();
    let mint_address_clone = mint_address.clone();
    let job_id_clone = job_id.clone();
    let job_manager_clone = job_manager.clone();

    // This is private code.
    // Please contact me if you need it.
}

/// Burn tokens for multiple mint addresses
pub async fn burn_each_tokens(
    wallet_pk: String,
    mint_addresses: Vec<String>,
    burn_percentage: f64,
) -> Result<String, String> {
    if burn_percentage <= 0.0 || burn_percentage > 100.0 {
        return Err("Burn percentage must be between 0.1 and 100.0".to_string());
    }

    if mint_addresses.is_empty() {
        return Err("No mint addresses provided".to_string());
    }

    info!(
        "Starting burn_each_tokens for wallet {} - burning {}% of {} different mints",
        wallet_pk,
        burn_percentage,
        mint_addresses.len()
    );

    let rpc_url = CONFIG.api.helius_https.clone();
    let wallet_pk_clone = wallet_pk.clone();
    let mint_addresses_clone = mint_addresses.clone();

    // Private code.

    // Serialize result as JSON for job system
    match serde_json::to_string(&res) {
        Ok(json_result) => Ok(json_result),
        Err(e) => Err(format!("Failed to serialize result: {}", e)),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BurnTokensBatchRequest {
    pub wallet_pk: String,
    pub token_mints: Vec<String>,
}

/// Burn tokens for multiple mints with progress tracking

// Private code.
