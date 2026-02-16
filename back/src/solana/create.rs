use std::sync::Arc;

use log::{error, info};
use pumpfun::{common::types::PriorityFee, utils::CreateTokenMetadata};
use solana_sdk::{signature::Keypair, signer::Signer};

pub async fn create_and_buy_task(
    metadata: CreateTokenMetadata,
    pumpfun_sdk_clone: Arc<pumpfun::PumpFun>,
    mint_keypair_clone: Keypair,
    fee_create_and_buy: PriorityFee,
    dev_token_amount: u64,
    dev_sol_amount: u64,
    slippage: u64,
) -> Result<(String, String), anyhow::Error> {
    let create_and_buy_signature = pumpfun_sdk_clone
        .create_and_buy(
            mint_keypair_clone.insecure_clone(),
            metadata,
            dev_token_amount,
            dev_sol_amount,
            Some(slippage),
            Some(fee_create_and_buy),
        )
        .await;
    if create_and_buy_signature.is_err() {
        error!(
            "Error creating and buying token: {}",
            create_and_buy_signature.unwrap_err()
        );
        return Err(anyhow::anyhow!("failed to create and buy token"));
    }
    let create_and_buy_signature = create_and_buy_signature.unwrap();
    // info!("Mint keypair: {:?}", mint_keypair_clone);
    info!(
        "Created and bought token: signature {}",
        create_and_buy_signature
    );
    Ok((
        create_and_buy_signature.to_string(),
        mint_keypair_clone.pubkey().to_string(),
    ))
}
