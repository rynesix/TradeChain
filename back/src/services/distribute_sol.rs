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

pub async fn distribute_sol(
    source_private_key: String,
    destination_wallets: Vec<String>,
    total_amount_sol: f64,
    job_id: Option<String>,
) -> Result<String, String> {
    const MAX_DESTINATION_WALLETS: usize = 200;
    const MAX_TRANSFERS_PER_TX: usize = 10;

    if destination_wallets.len() > MAX_DESTINATION_WALLETS {
        return Err(format!(
            "Too many destination wallets provided: {} (max: {})",
            destination_wallets.len(),
            MAX_DESTINATION_WALLETS
        ));
    }

    if destination_wallets.is_empty() {
        return Err("No destination wallets provided".to_string());
    }

    info!(
        "Starting distribute_sol from source wallet to {} destinations with {} SOL total",
        destination_wallets.len(),
        total_amount_sol
    );

    let rpc_url = CONFIG.api.helius_https.clone();
    let source_pk_clone = source_private_key.clone();
    let destinations_clone = destination_wallets.clone();

    let res = task::spawn_blocking(move || -> anyhow::Result<Vec<String>> {
        let client = RpcClient::new(rpc_url);

        // Parse source keypair
        let source_bytes = bs58::decode(&source_pk_clone).into_vec().context("invalid base58 source key")?;
        let source_keypair = if source_bytes.len() == 64 {
            Keypair::try_from(&source_bytes[..]).context("invalid source keypair bytes")?
        } else {
            anyhow::bail!("unexpected source key length: {} bytes", source_bytes.len());
        };

        let source_pubkey = source_keypair.pubkey();
        info!("Distributing from source wallet: {}", source_pubkey);

        // Check source wallet balance
        let source_balance = client
            .get_balance(&source_pubkey)
            .context("failed to get source wallet balance")?;

        let total_amount_lamports = (total_amount_sol * 1_000_000_000.0) as u64;

        if source_balance < total_amount_lamports {
            anyhow::bail!(
                "Insufficient balance in source wallet. Required: {} lamports, Available: {} lamports",
                total_amount_lamports,
                source_balance
            );
        }

        // Parse destination pubkeys
        let mut dest_pubkeys = Vec::with_capacity(destinations_clone.len());
        for dest_str in destinations_clone.iter() {
            let dest_pubkey = Pubkey::from_str(dest_str).context("invalid destination pubkey")?;
            dest_pubkeys.push(dest_pubkey);
        }

        // Calculate amount per destination
        let amount_per_dest = total_amount_lamports / dest_pubkeys.len() as u64;
        info!(
            "Distributing {} lamports ({} SOL) to each of {} destinations",
            amount_per_dest,
            amount_per_dest as f64 / 1_000_000_000.0,
            dest_pubkeys.len()
        );

        // Update job progress if job_id provided
        if let Some(ref job_id) = job_id {
            let _ = crate::update_job_progress(&job_id, 0, dest_pubkeys.len() as u32, "Starting SOL distribution".to_string());
        }

        // Process destinations in batches of 10
        let mut transaction_signatures = Vec::new();
        let mut completed_transfers = 0;

        for (batch_idx, batch_destinations) in dest_pubkeys.chunks(MAX_TRANSFERS_PER_TX).enumerate() {
            info!(
                "Processing batch {} with {} destination wallets",
                batch_idx + 1,
                batch_destinations.len()
            );

            let mut batch_instructions = Vec::new();

            // Create transfer instructions for this batch (from source to each destination)
            for dest_pubkey in batch_destinations.iter() {
                batch_instructions.push(system_instruction::transfer(
                    &source_pubkey,
                    dest_pubkey,
                    amount_per_dest
                ));

                info!("Will transfer {} lamports to {}", amount_per_dest, dest_pubkey);
            }

            let recent_blockhash = client
                .get_latest_blockhash()
                .context("failed to fetch blockhash")?;

            // Only the source keypair needs to sign (it's paying for all transfers)
            let signer_refs: Vec<&dyn Signer> = vec![&source_keypair as &dyn Signer];

            let tx = Transaction::new_signed_with_payer(
                &batch_instructions,
                Some(&source_pubkey),
                &signer_refs,
                recent_blockhash,
            );

            // Send transaction
            let sig = client
                .send_and_confirm_transaction(&tx)
                .context("send_and_confirm_transaction failed")?;

            info!("Batch {} completed with signature: {}", batch_idx + 1, sig);
            transaction_signatures.push(sig.to_string());

            // Update progress after each batch
            completed_transfers += batch_destinations.len();
            if let Some(ref job_id) = job_id {
                let _ = crate::update_job_progress(
                    &job_id,
                    completed_transfers as u32,
                    dest_pubkeys.len() as u32,
                    format!("Completed batch {} of {} (transfers: {}/{})", batch_idx + 1,
                           (dest_pubkeys.len() + MAX_TRANSFERS_PER_TX - 1) / MAX_TRANSFERS_PER_TX,
                           completed_transfers, dest_pubkeys.len())
                );
            }            // Small delay between batches to avoid overwhelming the RPC
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        info!(
            "All batches completed. Total transactions: {}",
            transaction_signatures.len()
        );
        Ok(transaction_signatures)
    })
    .await
    .map_err(|e| format!("join error: {}", e))
    .and_then(|r| r.map_err(|e| format!("rpc error: {}", e)))?;

    // Return summary of all transaction signatures
    Ok(format!(
        "Distributed {} SOL to {} wallets in {} transactions: {}",
        total_amount_sol,
        destination_wallets.len(),
        res.len(),
        res.join(", ")
    ))
}
