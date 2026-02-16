use crate::config::CONFIG;
use anyhow::Context;
use log::info;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{bs58, signature::Keypair, signer::Signer};
use std::str::FromStr;
use tokio::task;

// ============= HELPER FUNCTIONS =============

/// Parse a base58 private key string into a Keypair
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

/// Parse multiple private keys into keypairs
fn parse_keypairs(private_keys: &[String]) -> anyhow::Result<Vec<Keypair>> {
    let mut keypairs = Vec::with_capacity(private_keys.len());
    for pk in private_keys.iter() {
        keypairs.push(parse_keypair(pk)?);
    }
    if keypairs.is_empty() {
        anyhow::bail!("no keypairs provided");
    }
    Ok(keypairs)
}

/// Create and send a single SOL transfer transaction
fn send_transfer(
    client: &RpcClient,
    source_kp: &Keypair,
    dest_pubkey: &Pubkey,
    amount_lamports: u64,
) -> anyhow::Result<String> {
    let source_pubkey = source_kp.pubkey();

    // Create transfer instruction
    let instruction = system_instruction::transfer(&source_pubkey, dest_pubkey, amount_lamports);

    // Get recent blockhash
    let recent_blockhash = client
        .get_latest_blockhash()
        .context("failed to fetch blockhash")?;

    // Create signer references
    let signer_refs: Vec<&dyn Signer> = vec![source_kp as &dyn Signer];

    // Create and send transaction
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&source_pubkey),
        &signer_refs,
        recent_blockhash,
    );

    let sig = client
        .send_and_confirm_transaction(&tx)
        .context("send_and_confirm_transaction failed")?;

    Ok(sig.to_string())
}

/// Check if wallet has sufficient balance for a transfer
fn check_balance_sufficient(
    client: &RpcClient,
    wallet_pubkey: &Pubkey,
    required_amount: u64,
    min_reserve: u64,
) -> anyhow::Result<bool> {
    let balance = client
        .get_balance(wallet_pubkey)
        .context("RPC get_balance failed")?;

    let required_total = required_amount + min_reserve + 5_000; // extra for transaction fees
    Ok(balance >= required_total)
}

/// Get transferable balance (total - reserve)
fn get_transferable_balance(
    client: &RpcClient,
    wallet_pubkey: &Pubkey,
    min_reserve: u64,
) -> anyhow::Result<Option<u64>> {
    let balance = client
        .get_balance(wallet_pubkey)
        .context("RPC get_balance failed")?;

    if balance <= min_reserve {
        return Ok(None);
    }

    let transferable = balance.saturating_sub(min_reserve);
    if transferable == 0 {
        Ok(None)
    } else {
        Ok(Some(transferable))
    }
}

// ============= PUBLIC FUNCTIONS =============

pub async fn refunds_to(
    pks: Vec<String>,
    refund_to: String,
    _funding_pk: String,
    job_id: Option<String>,
) -> Result<String, String> {
    const MAX_PRIVATE_KEYS: usize = 200;
    const MIN_RESERVE: u64 = 5_000;

    if pks.len() > MAX_PRIVATE_KEYS {
        return Err(format!(
            "Too many private keys provided: {} (max: {})",
            pks.len(),
            MAX_PRIVATE_KEYS
        ));
    }

    info!(
        "Starting refund_wallet for {} keypairs to {}",
        pks.len(),
        refund_to
    );

    let rpc_url = CONFIG.api.helius_https.clone();
    let pks_clone = pks.clone();
    let refund_to_clone = refund_to.clone();

    let res = task::spawn_blocking(move || -> anyhow::Result<Vec<String>> {
        let client = RpcClient::new(rpc_url);

        let dest_pubkey =
            Pubkey::from_str(&refund_to_clone).context("invalid refund destination pubkey")?;
        info!("Refunding to: {}", dest_pubkey);

        // Parse all keypairs using helper function
        let keypairs = parse_keypairs(&pks_clone)?;

        info!("Total transfers to process: {}", keypairs.len());

        // Process transfers ONE BY ONE (no batching)
        let mut transaction_signatures = Vec::new();
        let total_wallets = keypairs.len() as u32;

        // Update job progress if job_id provided
        if let Some(ref job_id) = job_id {
            let _ = crate::update_job_progress(
                &job_id,
                0,
                total_wallets,
                "Starting refund process".to_string(),
            );
        }

        for (transfer_idx, kp) in keypairs.iter().enumerate() {
            let pubkey = kp.pubkey();
            let current_idx = transfer_idx as u32 + 1;

            // Update progress if job_id provided
            if let Some(ref job_id) = job_id {
                let _ = crate::update_job_progress(
                    &job_id,
                    current_idx - 1,
                    total_wallets,
                    format!(
                        "Processing wallet {} of {} ({})...",
                        current_idx, total_wallets, pubkey
                    ),
                );
            }

            // Get transferable balance using helper function
            match get_transferable_balance(&client, &pubkey, MIN_RESERVE)? {
                None => {
                    info!(
                        "Wallet {} has insufficient transferable balance, skipping",
                        pubkey
                    );

                    // Update progress for skipped wallets if job_id provided
                    if let Some(ref job_id) = job_id {
                        let _ = crate::update_job_progress(
                            &job_id,
                            current_idx,
                            total_wallets,
                            format!(
                                "Skipped wallet {} of {} (insufficient balance)",
                                current_idx, total_wallets
                            ),
                        );
                    }
                    continue;
                }
                Some(amount) => {
                    info!(
                        "Processing transfer {} of {}: {} will transfer {} lamports",
                        transfer_idx + 1,
                        keypairs.len(),
                        pubkey,
                        amount
                    );

                    // Send transfer using helper function
                    match send_transfer(&client, kp, &dest_pubkey, amount) {
                        Ok(sig) => {
                            info!("Transfer {} completed with signature: {}", current_idx, sig);
                            let sig_clone = sig.clone();
                            transaction_signatures.push(sig);

                            // Update progress after successful transfer if job_id provided
                            if let Some(ref job_id) = job_id {
                                let _ = crate::update_job_progress(
                                    &job_id,
                                    current_idx,
                                    total_wallets,
                                    format!(
                                        "Completed transfer {} of {} ({})",
                                        current_idx, total_wallets, sig_clone
                                    ),
                                );
                            }
                        }
                        Err(e) => {
                            info!(
                                "Transfer {} failed for wallet {}: {}",
                                current_idx, pubkey, e
                            );

                            // Update progress even for failed transfers if job_id provided
                            if let Some(ref job_id) = job_id {
                                let _ = crate::update_job_progress(
                                    &job_id,
                                    current_idx,
                                    total_wallets,
                                    format!(
                                        "Failed transfer {} of {} ({}): {}",
                                        current_idx, total_wallets, pubkey, e
                                    ),
                                );
                            }
                            // Continue with next transfer instead of failing completely
                            continue;
                        }
                    }

                    // Small delay between individual transfers
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
        }

        info!(
            "All transfers completed. Total successful transactions: {}",
            transaction_signatures.len()
        );
        Ok(transaction_signatures)
    })
    .await
    .map_err(|e| format!("join error: {}", e))
    .and_then(|r| r.map_err(|e| format!("rpc error: {}", e)))?;

    // Return summary of all transaction signatures
    Ok(format!(
        "Completed {} individual transfers: {}",
        res.len(),
        res.join(", ")
    ))
}

pub async fn refund_amount_to(
    source_pk: String,
    refund_to: String,
    amount_sol: f64,
    job_id: Option<String>,
) -> Result<String, String> {
    // Validation des paramètres
    if amount_sol <= 0.0 {
        return Err("Amount must be greater than 0".to_string());
    }

    let amount_lamports = (amount_sol * 1_000_000_000.0) as u64;
    const MIN_RESERVE: u64 = 5_000;

    info!(
        "Starting refund_specific_amount: {} SOL ({} lamports) from source to {}",
        amount_sol, amount_lamports, refund_to
    );

    let rpc_url = CONFIG.api.helius_https.clone();
    let source_pk_clone = source_pk.clone();
    let refund_to_clone = refund_to.clone();

    let res = task::spawn_blocking(move || -> anyhow::Result<String> {
        let client = RpcClient::new(rpc_url);

        // Parse destination pubkey
        let dest_pubkey =
            Pubkey::from_str(&refund_to_clone).context("invalid refund destination pubkey")?;
        info!("Refunding to: {}", dest_pubkey);

        // Parse source keypair using helper function
        let source_kp = parse_keypair(&source_pk_clone)?;
        let source_pubkey = source_kp.pubkey();
        info!("Source wallet: {}", source_pubkey);

        // Check balance using helper function
        if !check_balance_sufficient(&client, &source_pubkey, amount_lamports, MIN_RESERVE)? {
            let balance = client.get_balance(&source_pubkey)?;
            let required_balance = amount_lamports + MIN_RESERVE + 5_000;
            anyhow::bail!(
                "Insufficient balance in source wallet. Required: {} lamports ({} SOL), Available: {} lamports ({} SOL)",
                required_balance,
                required_balance as f64 / 1_000_000_000.0,
                balance,
                balance as f64 / 1_000_000_000.0
            );
        }

        info!(
            "Transferring {} lamports ({} SOL) from {} to {}",
            amount_lamports, amount_sol, source_pubkey, dest_pubkey
        );

        // Send transfer using helper function
        let sig = send_transfer(&client, &source_kp, &dest_pubkey, amount_lamports)?;

        info!(
            "Transfer completed successfully with signature: {}",
            sig
        );

        Ok(sig)
    })
    .await
    .map_err(|e| format!("join error: {}", e))
    .and_then(|r| r.map_err(|e| format!("rpc error: {}", e)))?;

    // Return transaction signature
    Ok(format!(
        "Successfully transferred {} SOL. Transaction signature: {}",
        amount_sol, res
    ))
}

pub async fn refunds_amount_to(
    pks: Vec<String>,
    refund_to: String,
    amount_sol: f64,
    job_id: Option<String>,
) -> Result<String, String> {
    // Security check: limit maximum number of private keys
    const MAX_PRIVATE_KEYS: usize = 200;
    const MIN_RESERVE: u64 = 5_000;

    if pks.len() > MAX_PRIVATE_KEYS {
        return Err(format!(
            "Too many private keys provided: {} (max: {})",
            pks.len(),
            MAX_PRIVATE_KEYS
        ));
    }

    if amount_sol <= 0.0 {
        return Err("Amount must be greater than 0".to_string());
    }

    let amount_lamports = (amount_sol * 1_000_000_000.0) as u64;

    info!(
        "Starting refund_wallets_specific_amount: {} SOL ({} lamports) from {} wallets to {}",
        amount_sol,
        amount_lamports,
        pks.len(),
        refund_to
    );

    let rpc_url = CONFIG.api.helius_https.clone();
    let pks_clone = pks.clone();
    let refund_to_clone = refund_to.clone();

    let res = task::spawn_blocking(move || -> anyhow::Result<Vec<String>> {
        let client = RpcClient::new(rpc_url);

        let dest_pubkey =
            Pubkey::from_str(&refund_to_clone).context("invalid refund destination pubkey")?;
        info!("Refunding to: {}", dest_pubkey);

        // Parse all keypairs using helper function
        let keypairs = parse_keypairs(&pks_clone)?;

        info!("Total wallets to process: {}", keypairs.len());

        // Update job progress if job_id provided
        if let Some(ref job_id) = job_id {
            let _ = crate::update_job_progress(&job_id, 0, keypairs.len() as u32, "Starting specific amount refund process".to_string());
        }

        // Process transfers ONE BY ONE (no batching)
        let mut transaction_signatures = Vec::new();
        let mut successful_transfers = 0;
        let mut failed_transfers = 0;

        for (transfer_idx, kp) in keypairs.iter().enumerate() {
            let pubkey = kp.pubkey();
            let current_idx = transfer_idx as u32 + 1;

            // Update progress if job_id provided
            if let Some(ref job_id) = job_id {
                let _ = crate::update_job_progress(
                    &job_id,
                    current_idx - 1,
                    keypairs.len() as u32,
                    format!("Processing wallet {} of {} ({})...", current_idx, keypairs.len(), pubkey)
                );
            }

            // Check balance using helper function
            if !check_balance_sufficient(&client, &pubkey, amount_lamports, MIN_RESERVE)? {
                let balance = client.get_balance(&pubkey)?;
                let required_balance = amount_lamports + MIN_RESERVE + 5_000;
                info!(
                    "Wallet {} has insufficient balance. Required: {} lamports ({} SOL), Available: {} lamports ({} SOL), skipping",
                    pubkey,
                    required_balance,
                    required_balance as f64 / 1_000_000_000.0,
                    balance,
                    balance as f64 / 1_000_000_000.0
                );
                failed_transfers += 1;

                // Update progress for skipped wallets if job_id provided
                if let Some(ref job_id) = job_id {
                    let _ = crate::update_job_progress(
                        &job_id,
                        current_idx,
                        keypairs.len() as u32,
                        format!("Skipped wallet {} of {} (insufficient balance)", current_idx, keypairs.len())
                    );
                }
                continue;
            }

            info!(
                "Processing transfer {} of {}: {} will transfer {} lamports ({} SOL)",
                transfer_idx + 1,
                keypairs.len(),
                pubkey,
                amount_lamports,
                amount_sol
            );

            // Send transfer using helper function
            match send_transfer(&client, kp, &dest_pubkey, amount_lamports) {
                Ok(sig) => {
                    info!(
                        "Transfer {} completed with signature: {}",
                        transfer_idx + 1,
                        sig
                    );
                    let sig_clone = sig.clone();
                    transaction_signatures.push(sig);
                    successful_transfers += 1;

                    // Update progress after successful transfer if job_id provided
                    if let Some(ref job_id) = job_id {
                        let _ = crate::update_job_progress(
                            &job_id,
                            current_idx,
                            keypairs.len() as u32,
                            format!("Completed transfer {} of {} ({})", current_idx, keypairs.len(), sig_clone)
                        );
                    }
                }
                Err(e) => {
                    info!(
                        "Transfer {} failed for wallet {}: {}",
                        transfer_idx + 1,
                        pubkey,
                        e
                    );
                    failed_transfers += 1;

                    // Update progress even for failed transfers if job_id provided
                    if let Some(ref job_id) = job_id {
                        let _ = crate::update_job_progress(
                            &job_id,
                            current_idx,
                            keypairs.len() as u32,
                            format!("Failed transfer {} of {} ({}): {}", current_idx, keypairs.len(), pubkey, e)
                        );
                    }
                    // Continue with next transfer instead of failing completely
                    continue;
                }
            }

            // Small delay between individual transfers
            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        info!(
            "All specific amount transfers completed. Successful: {}, Failed: {}, Total signatures: {}",
            successful_transfers, failed_transfers, transaction_signatures.len()
        );
        Ok(transaction_signatures)
    })
    .await
    .map_err(|e| format!("join error: {}", e))
    .and_then(|r| r.map_err(|e| format!("rpc error: {}", e)))?;

    // Return summary of all transaction signatures
    Ok(format!(
        "Completed {} transfers of {} SOL each. Transactions: {}",
        res.len(),
        amount_sol,
        res.join(", ")
    ))
}
