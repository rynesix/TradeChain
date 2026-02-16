use tracing_subscriber::{fmt::time::ChronoLocal, FmtSubscriber};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ----------------- Logger -----------------
pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_owned()))
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Error initializing logs");
}

// ----------------- Job Progress Helper -----------------
use std::sync::Arc;
use std::sync::OnceLock;

static JOB_MANAGER: OnceLock<Arc<jobs::JobManager>> = OnceLock::new();

pub fn set_job_manager_instance(manager: Arc<jobs::JobManager>) {
    let _ = JOB_MANAGER.set(manager);
}

pub fn update_job_progress_items(job_id: &str, completed: u32, total: u32, step: String) -> bool {
    if let Some(manager) = JOB_MANAGER.get() {
        manager.update_progress_items(job_id, completed, total, Some(step))
    } else {
        false
    }
}

// Helper function for services to update job progress with different name to avoid conflict
pub fn update_job_progress(job_id: &str, completed: u32, total: u32, step: String) -> bool {
    update_job_progress_items(job_id, completed, total, step)
}
pub mod config;
pub mod jobs;
pub mod solana;
pub mod services {
    pub mod burn_tokens;
    pub mod close_accounts;
    pub mod close_token_account;
    pub mod common;
    pub mod create_meme_token;
    pub mod distribute_sol;
    pub mod get_sol_balance;
    pub mod get_token_balance;
    pub mod long_polling;
    pub mod quick_buy;
    pub mod quick_sell;
    pub mod refund_wallets;
    pub mod sniper_bot;
}
