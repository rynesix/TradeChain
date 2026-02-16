use crate::config::CONFIG;
use crate::services::common::{QuickSellReq, TransactionRes};
use crate::solana::sell::sell_task;
use log::info;
use pumpfun::common::types::{Cluster, PriorityFee, RpcEndpoint};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair, signer::Signer};
use std::{str::FromStr, sync::Arc};

pub async fn quick_sell(req: QuickSellReq) -> TransactionRes {
    info!("quick_sell request: {:?}", req);

    // Private code
}
