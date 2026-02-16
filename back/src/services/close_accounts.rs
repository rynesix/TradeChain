use std::str::FromStr;

use crate::config::CONFIG;
use anyhow::Context;
use log::info;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{bs58, signature::Keypair, signer::Signer};
use solana_sdk::{instruction::Instruction, transaction::Transaction};
use tokio::task;

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

pub async fn close_accounts(wallet_pk: String) -> Result<String, String> {
    const MAX_CLOSES_PER_TX: usize = 5; // Limit closes per transaction

    info!(
        "Starting close_accounts for wallet - closing empty token accounts and refunding rent to same wallet"
    );

    let rpc_url = CONFIG.api.helius_https.clone();
    let wallet_pk_clone = wallet_pk.clone();

    let res = task::spawn_blocking(move || -> anyhow::Result<Vec<String>> {
        let client = RpcClient::new(rpc_url);

        // Parse the wallet keypair
        let wallet_keypair = parse_keypair(&wallet_pk_clone)?;
        let wallet_pubkey = wallet_keypair.pubkey();

        info!("Processing wallet: {}", wallet_pubkey);
        info!(
            "Rent will be refunded to the same wallet: {}",
            wallet_pubkey
        );

        let mut transaction_signatures = Vec::new();
        let mut total_closed_accounts = 0;

        // Get all token accounts for this wallet
        let token_accounts = client
            .get_token_accounts_by_owner(
                &wallet_pubkey,
                TokenAccountsFilter::ProgramId(spl_token::id()),
            )
            .context("failed to get token accounts")?;

        info!(
            "Found {} token accounts for wallet {}",
            token_accounts.len(),
            wallet_pubkey
        );

        // Find token accounts with 0 balance
        let mut accounts_to_close = Vec::new();

        for token_account in token_accounts {
            let account_pubkey =
                Pubkey::from_str(&token_account.pubkey).context("invalid token account pubkey")?;

            if let Some((mint, ui_amount)) = extract_token_info(&token_account.account.data) {
                info!(
                    "Token account {} - Mint: {}, Balance: {}",
                    account_pubkey, mint, ui_amount
                );

                if ui_amount == 0.0 {
                    info!(
                        "Adding token account {} to close list (balance: 0)",
                        account_pubkey
                    );
                    accounts_to_close.push(account_pubkey);
                }
            } else {
                info!("Could not parse token account data for {}", account_pubkey);
            }
        }

        if accounts_to_close.is_empty() {
            info!("No empty token accounts found for wallet {}", wallet_pubkey);
            return Ok(vec!["No empty token accounts to close".to_string()]);
        }

        info!(
            "Found {} empty token accounts to close for wallet {}",
            accounts_to_close.len(),
            wallet_pubkey
        );

        // Process accounts in batches
        for (batch_idx, batch_accounts) in accounts_to_close.chunks(MAX_CLOSES_PER_TX).enumerate() {
            info!(
                "Processing batch {} with {} accounts to close",
                batch_idx + 1,
                batch_accounts.len()
            );

            let mut instructions = Vec::new();

            // Create close instructions for this batch (refund rent to same wallet)
            for account_pubkey in batch_accounts {
                let close_instruction = create_close_instruction(
                    account_pubkey,
                    &wallet_pubkey,
                    &wallet_pubkey, // Refund rent to the same wallet
                );
                instructions.push(close_instruction);
            }

            let recent_blockhash = client
                .get_latest_blockhash()
                .context("failed to fetch blockhash")?;

            let signer_refs: Vec<&dyn Signer> = vec![&wallet_keypair as &dyn Signer];

            let tx = Transaction::new_signed_with_payer(
                &instructions,
                Some(&wallet_pubkey),
                &signer_refs,
                recent_blockhash,
            );

            // Send transaction
            match client.send_and_confirm_transaction(&tx) {
                Ok(sig) => {
                    info!(
                        "Batch {} completed with signature: {} (closed {} accounts)",
                        batch_idx + 1,
                        sig,
                        batch_accounts.len()
                    );
                    transaction_signatures.push(sig.to_string());
                    total_closed_accounts += batch_accounts.len();
                }
                Err(e) => {
                    info!("Failed to close batch {}: {}", batch_idx + 1, e);
                    continue;
                }
            }

            // Small delay between batches
            std::thread::sleep(std::time::Duration::from_millis(300));
        }

        info!(
            "All account closures completed. Total closed accounts: {}, Total transactions: {}",
            total_closed_accounts,
            transaction_signatures.len()
        );
        Ok(transaction_signatures)
    })
    .await
    .map_err(|e| format!("join error: {}", e))
    .and_then(|r| r.map_err(|e| format!("rpc error: {}", e)))?;

    // Return summary of all transaction signatures
    Ok(format!(
        "Closed {} empty token accounts. Transactions: {}",
        res.len(),
        res.join(", ")
    ))
}
