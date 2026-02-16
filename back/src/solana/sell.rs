use log::{error, info};
use pumpfun::common::types::PriorityFee;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{
    pubkey::Pubkey, signature::Keypair, signer::Signer,
};
use std::{str::FromStr, sync::Arc};

pub async fn sell_task(
    fee: Option<PriorityFee>,
    wallet: Arc<Keypair>,
    pumpfun_sdk: Arc<pumpfun::PumpFun>,
    mint: Pubkey,
    percent: u8,
    slippage_bps: Option<u64>,
) -> Result<(), anyhow::Error> {
    let pumpfun_sdk_wallet: Arc<pumpfun::PumpFun> = Arc::new(pumpfun::PumpFun::new(
        Arc::new(wallet.insecure_clone()),
        pumpfun_sdk.cluster.clone(),
    ));
    let mint_clone = mint.clone();
    // let token_amount_to_sell = percent as f64 / 100.0
    //     * pumpfun_sdk_wallet.rpc.get_balance(&wallet.pubkey()).await? as f64
    //     / 1_000_000_000.0;

    let token_account = pumpfun_sdk_wallet
        .rpc
        .get_token_accounts_by_owner(&wallet.pubkey(), TokenAccountsFilter::Mint(mint_clone))
        .await?;

    let token_account_balance = pumpfun_sdk_wallet
        .rpc
        .get_token_account_balance(&Pubkey::from_str(&token_account[0].pubkey).unwrap())
        .await?;

    let token_amount_to_sell = percent as f64 / 100.0 * token_account_balance.ui_amount.unwrap();
    info!(
        "Selling {}% of token balance: {} tokens",
        percent, token_amount_to_sell
    );

    let token_amount_to_sell =
        (token_amount_to_sell * 10f64.powi(token_account_balance.decimals as i32)) as u64;
    info!(
        "Token amount to sell (in smallest units): {}",
        token_amount_to_sell
    );

    let sell_sig = pumpfun_sdk_wallet
        .sell(
            mint_clone,
            Some(token_amount_to_sell),
            slippage_bps,
            fee.clone(),
        )
        .await;
    if sell_sig.is_err() {
        error!(
            "Error selling token with wallet {}: {}",
            wallet.pubkey(),
            sell_sig.unwrap_err()
        );

        return Err(anyhow::anyhow!("failed to sell token"));
    }
    info!(
        "Sold with wallet {}: {} SOL, signature: {}",
        wallet.pubkey(),
        token_amount_to_sell,
        sell_sig.unwrap()
    );
    Ok(())
}
