use crate::{
    config::CONFIG,
    services::{
        common::{CreateTokenReq, QuickBuyReq},
        quick_buy,
    },
};
use chrono::{self, TimeDelta};
use log::info;
use pumpfun::common::types::{Cluster, PriorityFee, RpcEndpoint};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair};
use std::{sync::Arc, time::Duration};

pub async fn sniper_buy(req: CreateTokenReq, mint_keypair: Pubkey) -> Result<(), String> {
    info!("Sniper buy for {:?}", req);

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

    // info!("Main Wallet Public Key: {}", main_wallet.pubkey());

    let start = chrono::Utc::now();

    // Private code
}
