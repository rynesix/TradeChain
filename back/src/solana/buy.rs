use log::{error, info};
use pumpfun::common::types::PriorityFee;
use solana_sdk::{
    native_token::sol_str_to_lamports, pubkey::Pubkey, signature::Keypair, signer::Signer,
};
use std::sync::Arc;

pub async fn buy_task(
    fee: Option<PriorityFee>,
    wallet: Arc<Keypair>,
    pumpfun_sdk: Arc<pumpfun::PumpFun>,
    mint: Pubkey,
    buy_amount_sol: f64,
    slippage_bps: Option<u16>,
) -> Result<String, anyhow::Error> {
    let pumpfun_sdk_wallet: Arc<pumpfun::PumpFun> = Arc::new(pumpfun::PumpFun::new(
        Arc::new(wallet.insecure_clone()),
        pumpfun_sdk.cluster.clone(),
    ));
    let mint_clone = mint.clone();
    let lamports = sol_str_to_lamports(&buy_amount_sol.to_string())
        .ok_or_else(|| anyhow::anyhow!("Failed to convert SOL to lamports"))?;
    let buy_sig = pumpfun_sdk_wallet
        .buy(
            mint_clone,
            lamports,
            Some(slippage_bps.unwrap() as u64),
            fee.clone(),
        )
        .await;
    if buy_sig.is_err() {
        error!(
            "Error buying token with wallet {}: {}",
            wallet.pubkey(),
            buy_sig.unwrap_err()
        );

        return Err(anyhow::anyhow!("failed to buy token"));
    }
    let sig = buy_sig.unwrap();
    info!(
        "Bought with wallet {}: {} SOL, signature: {}",
        wallet.pubkey(),
        buy_amount_sol,
        sig
    );
    Ok((sig.to_string()).into())
}
