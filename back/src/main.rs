#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app_lib::{
    init_logger,
    jobs::{JobInfo, JobManager},
    services::{
        self,
        common::{
            CreateTokenReq, CreateTokenTransactionRes, QuickBuyReq, QuickSellReq, TransactionRes,
        },
        create_meme_token,
        get_token_balance::GetTokenBalanceResult,
        sniper_bot,
    },
};
use env_logger::init;
use log::{error, info, warn};
use serde_json;
use solana_sdk::{signature::Keypair, signer::Signer};
use tauri::{AppHandle, State};

// Helper function for job progress updates
use services::*;

#[tauri::command]
async fn ping() -> String {
    "pong".into()
}

#[tauri::command]
async fn quick_buy(req: QuickBuyReq) -> Result<TransactionRes, String> {
    info!("quick_buy request: {:?}", req);
    services::quick_buy::quick_buy(req).await
}

#[tauri::command]
async fn quick_sell(req: QuickSellReq) -> TransactionRes {
    info!("quick_sell request: {:?}", req);
    services::quick_sell::quick_sell(req).await
}

#[tauri::command]
async fn create_token(req: CreateTokenReq) -> CreateTokenTransactionRes {
    info!("create_token request: {:?}", req);
    let mint_keypair = Keypair::new();
    let mint = mint_keypair.pubkey();

    let copy_req = req.clone();
    // tokio::spawn(async move {
    //     sniper_bot::sniper_buy(copy_req.clone(), mint)
    //         .await
    //         .map(|_tx_sig| ())
    //         .map_err(|e| e.to_string())
    // });
    match create_meme_token::create_meme_token(req, mint_keypair.insecure_clone()).await {
        Ok(tx_sig) => CreateTokenTransactionRes {
            ok: true,
            tx_sig: Some(tx_sig),
            mint: Some(mint_keypair.pubkey().to_string()),
            error: None,
        },
        Err(e) => CreateTokenTransactionRes {
            ok: false,
            tx_sig: None,
            mint: None,
            error: Some(format!("{}", e)),
        },
    }
}

// Command qui spawn un job create_token
#[tauri::command]
fn create_token_spawn(
    req: CreateTokenReq,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    let manager = job_manager.inner().clone();
    manager.spawn_job("create_token", app_handle, async move {
        let mint_keypair = Keypair::new();
        create_meme_token::create_meme_token(req, mint_keypair.insecure_clone())
            .await
            .map(|_tx_sig| ())
            .map_err(|e| e.to_string())
    })
}

#[tauri::command]
fn list_jobs(job_manager: State<'_, JobManager>) -> Vec<JobInfo> {
    job_manager.list_all()
}

#[tauri::command]
fn get_job_status(job_id: String, job_manager: State<'_, JobManager>) -> Option<JobInfo> {
    job_manager.get_info(&job_id)
}

#[tauri::command]
fn update_job_progress(
    job_id: String,
    percentage: f32,
    step: Option<String>,
    job_manager: State<'_, JobManager>,
) -> bool {
    job_manager.update_progress(&job_id, percentage, step)
}

#[tauri::command]
fn update_job_progress_items(
    job_id: String,
    completed: u32,
    total: u32,
    step: Option<String>,
    job_manager: State<'_, JobManager>,
) -> bool {
    job_manager.update_progress_items(&job_id, completed, total, step)
}

#[tauri::command]
async fn get_sol_balance(wallet: String) -> Result<f64, String> {
    services::get_sol_balance::get_sol_balance(wallet).await
}
#[tauri::command]
async fn get_token_balance(wallet: String, mint: String) -> Result<f64, String> {
    services::get_token_balance::get_single_token_balance(wallet, Some(mint)).await
}

#[tauri::command]
async fn get_tokens_balances(wallet: String) -> Result<GetTokenBalanceResult, String> {
    services::get_token_balance::get_tokens_balances(wallet).await
}

#[tauri::command]
async fn get_tokens(wallet: String) -> Result<GetTokenBalanceResult, String> {
    services::get_token_balance::get_tokens(wallet).await
}

#[tauri::command]
async fn refund_wallet(
    pks: Vec<String>,
    refund_to: String,
    funding_pk: String,
) -> Result<String, String> {
    services::refund_wallets::refunds_to(pks, refund_to, funding_pk, None).await
}

#[tauri::command]
async fn refund_specific_amount(
    source_pk: String,
    refund_to: String,
    amount_sol: f64,
) -> Result<String, String> {
    services::refund_wallets::refund_amount_to(source_pk, refund_to, amount_sol, None).await
}

#[tauri::command]
async fn refund_wallets_specific_amount(
    pks: Vec<String>,
    refund_to: String,
    amount_sol: f64,
) -> Result<String, String> {
    services::refund_wallets::refunds_amount_to(pks, refund_to, amount_sol, None).await
}
#[tauri::command]
async fn distribute_sol(
    src: String,
    wallets: Vec<String>,
    total_amount_sol: f64,
) -> Result<String, String> {
    services::distribute_sol::distribute_sol(src, wallets, total_amount_sol, None).await
}

#[tauri::command]
async fn close_accounts(wallet_pk: String) -> Result<String, String> {
    services::close_accounts::close_accounts(wallet_pk).await
}

#[tauri::command]
async fn close_token_account(wallet_pk: String, token_mint: String) -> Result<String, String> {
    services::close_token_account::close_token_account(wallet_pk, token_mint, None).await
}

#[tauri::command]
async fn burn_tokens(
    wallet_pk: String,
    mint_address: String,
    burn_percentage: f64,
) -> Result<String, String> {
    services::burn_tokens::burn_tokens(wallet_pk, mint_address, burn_percentage).await
}

#[tauri::command]
async fn burn_each_tokens(
    wallet_pk: String,
    mint_addresses: Vec<String>,
    burn_percentage: f64,
) -> Result<String, String> {
    services::burn_tokens::burn_each_tokens(wallet_pk, mint_addresses, burn_percentage).await
}

// Job-based commands pour les opérations intensives
#[tauri::command]
fn refund_wallets_job(
    pks: Vec<String>,
    refund_to: String,
    funding_pk: String,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    let manager = job_manager.inner().clone();
    let pks_clone = pks.clone();
    let refund_to_clone = refund_to.clone();
    let funding_pk_clone = funding_pk.clone();

    // Create job manually
    let job_id = manager.create_job("refund_wallets");
    let job_id_clone = job_id.clone();
    let job_id_return = job_id.clone();
    let manager_clone = manager.clone();

    // Private code
    job_id_return
}

#[tauri::command]
fn refund_wallets_specific_amount_job(
    pks: Vec<String>,
    refund_to: String,
    amount_sol: f64,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    let manager = job_manager.inner().clone();
    let pks_clone = pks.clone();
    let refund_to_clone = refund_to.clone();

    // Create job manually
    let job_id = manager.create_job("refund_wallets_specific_amount");
    let job_id_clone = job_id.clone();
    let manager_clone = manager.clone();

    // Private code

    job_id
}

#[tauri::command]
fn distribute_sol_job(
    src: String,
    wallets: Vec<String>,
    total_amount_sol: f64,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    let manager = job_manager.inner().clone();
    let src_clone = src.clone();
    let wallets_clone = wallets.clone();

    // Create job manually
    let job_id = manager.create_job("distribute_sol");
    let job_id_clone = job_id.clone();
    let job_id_return = job_id.clone();
    let manager_clone = manager.clone();

    // Start the actual distribute sol operation
    
    // Private code

    log::info!("Spawned distribute SOL job: {}", job_id_return);
    job_id_return
}

#[tauri::command]
fn close_accounts_job(
    wallet_pk: String,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    let manager = job_manager.inner().clone();
    let wallet_pk_clone = wallet_pk.clone();

    // Create job manually
    let job_id = manager.create_job("close_accounts");

    let job_id_clone = job_id.clone();
    let manager_clone = manager.clone();

    // Start the actual close accounts operation
    
    // Private code

    job_id
}

#[tauri::command]
fn close_token_account_job(
    wallet_pk: String,
    token_mint: String,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    let manager = job_manager.inner().clone();
    let job_name = format!(
        "close_token_account_{}",
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    );
    let job_name_clone = job_name.clone();
    manager.spawn_job(&job_name, app_handle, async move {
        services::close_token_account::close_token_account(
            wallet_pk,
            token_mint,
            Some(job_name_clone),
        )
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
    })
}

#[tauri::command]
fn close_token_accounts_batch_job(
    wallet_pk: String,
    token_mints: Vec<String>,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    info!(
        "Starting close_token_accounts_batch_job for wallet: {}, tokens: {:?}",
        wallet_pk, token_mints
    );

    let manager = job_manager.inner().clone();
    let request = services::close_token_account::CloseTokenAccountBatchRequest {
        wallet_pk,
        token_mints,
    };

    // Create job manually
    let job_id = manager.create_job("close_token_accounts_batch");
    let job_id_clone = job_id.clone();
    let job_id_return = job_id.clone();
    let manager_clone = manager.clone();

    // Private code
    info!("Created close_token_accounts_batch job with ID: {}", job_id);
    job_id
}

#[tauri::command]
fn burn_tokens_batch_job(
    wallet_pk: String,
    token_mints: Vec<String>,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    info!(
        "Starting burn_tokens_batch_job for wallet: {}, tokens: {:?}",
        wallet_pk, token_mints
    );

    let manager = job_manager.inner().clone();
    let job_name = "burn_tokens_batch".to_string();
    let request = services::burn_tokens::BurnTokensBatchRequest {
        wallet_pk,
        token_mints,
    };

    let job_id = manager.create_job(&job_name);
    let job_id_clone = job_id.clone(); // Use the actual job ID, not the job name
    let manager_clone = manager.clone();

    // Private code
    info!("Created burn_tokens_batch job with ID: {}", job_id);
    job_id
}

#[tauri::command]
fn burn_tokens_job(
    wallet_pk: String,
    mint_address: String,
    burn_percentage: f64,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    let manager = job_manager.inner().clone();
    let wallet_pk_clone = wallet_pk.clone();
    let mint_address_clone = mint_address.clone();

    manager.spawn_job("burn_tokens", app_handle, async move {
        services::burn_tokens::burn_tokens(wallet_pk_clone, mint_address_clone, burn_percentage)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    })
}

#[tauri::command]
fn burn_each_tokens_job(
    wallet_pk: String,
    mint_addresses: Vec<String>,
    burn_percentage: f64,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    let manager = job_manager.inner().clone();
    let wallet_pk_clone = wallet_pk.clone();
    let mint_addresses_clone = mint_addresses.clone();

    // Create job manually so we have the job_id available
    let job_id = manager.create_job("burn_each_tokens");

    let job_id_clone = job_id.clone();
    let manager_clone = manager.clone();

    // Start the actual burn each tokens operation
    // Private code
    job_id
}

#[tauri::command]
fn get_tokens_balances_job(
    wallet: String,
    app_handle: AppHandle,
    job_manager: State<'_, JobManager>,
) -> String {
    let manager = job_manager.inner().clone();
    manager.spawn_job("get_tokens_balances", app_handle, async move {
        services::get_token_balance::get_tokens_balances(wallet)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    })
}

// Private code