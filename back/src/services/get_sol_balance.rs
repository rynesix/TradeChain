use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub async fn get_sol_balance(wallet: String) -> Result<f64, String> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let pubkey = Pubkey::from_str(&wallet).map_err(|e| e.to_string())?;
    let balance = client
        .get_balance(&pubkey)
        .map_err(|e| format!("RPC error: {}", e))?;

    Ok(balance as f64 / 1_000_000_000.0)
}
