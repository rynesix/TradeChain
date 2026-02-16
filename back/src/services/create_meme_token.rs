use crate::{
    config::CONFIG,
    services::common::CreateTokenReq,
    solana::{self, create::create_and_buy_task},
};
use log::{error, info};
use pumpfun::common::types::{Cluster, PriorityFee, RpcEndpoint};
use rand::Rng;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::sol_str_to_lamports, signature::Keypair,
    signer::Signer,
};
use std::{ops::Deref, sync::Arc};

pub async fn create_meme_token(
    req: CreateTokenReq,
    mint_keypair: Keypair,
) -> anyhow::Result<String> {
    info!("Simulating create_token for {:?}", req);

    let cluster = Cluster {
        rpc: RpcEndpoint {
            http: CONFIG.api.helius_https.clone(),
            ws: CONFIG.api.helius_ws.clone(),
        },
        commitment: CommitmentConfig::confirmed(),
        priority_fee: PriorityFee::new(
            Some(req.cu_price_microlamports as u32),
            Some(req.max_unit_price_microlamports),
        ),
    };

    info!("Using cluster: {:?}", cluster);
    let main_wallet: Arc<Keypair> = Arc::new(Keypair::from_base58_string(&req.dev_wallet));

    info!("Using main wallet: {}", req.dev_wallet);
    let pumpfun_sdk = Arc::new(pumpfun::PumpFun::new(main_wallet.clone(), cluster));

    info!("Main Wallet Public Key: {}", main_wallet.pubkey());

    let metadata = generate_metadata(&req, mint_keypair.insecure_clone());
    let fee_create_and_buy = PriorityFee::new(
        Some(req.cu_price_microlamports as u32),
        Some(req.max_unit_price_microlamports),
    );

    // Private code.

    Ok(result.0)
}

pub fn generate_metadata(
    req: &CreateTokenReq,
    _mint_keypair: solana_sdk::signature::Keypair,
) -> pumpfun::utils::CreateTokenMetadata {
    let random_index = rand::thread_rng().gen_range(0..=10);
    let metadata: pumpfun::utils::CreateTokenMetadata =
        solana::tokens::TOKEN_COLLECTION.deref().tokens[random_index].clone();
    info!("Creating token metadata for {:?}", metadata);

    // Private code
}
