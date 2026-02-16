use crate::config::CONFIG;
use crate::services::common::{QuickBuyReq, TransactionRes};
use crate::solana::buy::buy_task;
use log::info;
use pumpfun::common::types::{Cluster, PriorityFee, RpcEndpoint};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair, signer::Signer};
use std::{str::FromStr, sync::Arc};

pub async fn quick_buy(req: QuickBuyReq) -> Result<TransactionRes, String> {
    info!("quick_buy request: {:?}", req);

    // Private code
}
