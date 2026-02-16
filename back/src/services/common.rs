use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct TransactionRes {
    pub ok: bool,
    pub tx_sig: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateTokenTransactionRes {
    pub ok: bool,
    pub tx_sig: Option<String>,
    pub mint: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateTokenReq {
    pub name: String,
    pub symbol: String,
    pub decimals: f64,
    pub description: String,
    pub website: String,
    pub telegram: String,
    pub twitter: String,
    pub supply_human: u64,
    pub supply_base_units: f64,
    pub dev_wallet: String,
    pub dev_token_amount: u64,
    pub dev_sol_amount: f64,
    pub sniper_wallet_one: String,
    pub amount_sol_sniper_one: f64,
    pub creation_tip_sol: f64,
    pub tip_sol: f64,
    pub slippage_bps: u64,
    pub cu_price_microlamports: u64,
    pub max_unit_price_microlamports: u64,
    // pub description: String,
    // pub file: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct QuickBuyReq {
    pub pk: String,
    pub amount_sol: f64,
    pub mint: String,
    pub slippage_bps: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct QuickSellReq {
    pub wallet: String,
    pub pk: String,
    pub percent: u8,
    pub mint: String,
    pub slippage_bps: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshBalancesReq {
    pub group_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct WalletBalance {
    pub wallet: String,
    pub balance: f64,
    pub tokens: u64,
}
