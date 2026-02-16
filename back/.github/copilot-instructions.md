# Copilot Instructions for trading-back

## Project Overview

This is a Rust backend for a Tauri desktop app focused on Solana blockchain trading automation. The codebase is organized for modularity, async operations, and integration with external APIs (Helius, MongoDB, PumpFun SDK).

## Architecture & Key Components

- **src/main.rs**: Entry point, Tauri command handlers, job orchestration, and Solana transaction logic.
- **src/jobs.rs**: Background job manager for async tasks, with support for job state tracking and cancellation.
- **src/config.rs**: Loads configuration from environment variables and `.env` files. Use `CONFIG` singleton for global settings (API endpoints, keys, trading params).
- **src/solana/**: Solana-specific logic (buy, sell, token creation, price, IPFS integration). Use these modules for blockchain operations.
- **src/utils/**: Utility functions and design helpers.
- **gen/schemas/**: JSON schemas for desktop and windows integration.

## Developer Workflows

- **Build**: Use `cargo build` for compilation. Tauri integration is handled via `tauri-build`.
- **Run**: Use `cargo run` to start the backend. For Tauri desktop, use `tauri dev` (see Tauri docs).
- **Environment**: Set up `.env` with required keys (see `src/config.rs` for expected variables).
- **Logging**: Uses both `env_logger` and `tracing_subscriber`. Logs are initialized in `main.rs` via `init_logger()`.
- **Async Jobs**: Spawn long-running tasks using `JobManager::spawn_job`. Job state is tracked and can be queried/cancelled.
- **Solana Integration**: All blockchain actions use the `pumpfun` SDK and Solana crates. RPC endpoints are configurable via environment.

## Patterns & Conventions

- **Tauri Commands**: Expose backend functions to the frontend using `#[tauri::command]`. All commands are registered in `main.rs`.
- **Job Management**: Use `JobManager` for background jobs. Each job has a unique ID and state (`Pending`, `Running`, `Completed`, `Failed`, `Cancelled`).
- **Config Access**: Always use the `CONFIG` singleton for accessing settings. Do not hardcode API endpoints or keys.
- **Error Handling**: Use `anyhow::Result` for error propagation. Log errors with `log::error!` or `tracing`.
- **Solana Keys**: Keys are loaded from environment and parsed using `Keypair::from_base58_string`.
- **External APIs**: Helius and MongoDB endpoints are set via environment. See `src/config.rs` for details.

## Integration Points

- **Frontend**: Communicates via Tauri commands. All backend actions should be exposed as commands.
- **Solana**: Uses `solana-sdk`, `solana-client`, and custom logic in `src/solana/`.
- **PumpFun SDK**: Used for token creation and trading logic.
- **MongoDB**: Connection string managed via config, but actual DB logic may be elsewhere.

## Examples

- **Spawning a job**:
  ```rust
  let job_id = job_manager.spawn_job("create_token", app_handle, |app, job_id| {
      tauri::async_runtime::spawn(async move {
          // ... async logic ...
      })
  });
  ```
- **Accessing config**:
  ```rust
  let api_url = CONFIG.api.helius_https.clone();
  ```
- **Registering a Tauri command**:
  ```rust
  #[tauri::command]
  async fn quick_buy(req: QuickBuyReq) -> TransactionRes { ... }
  ```

## Key Files & Directories

- `src/main.rs`: Tauri commands, job orchestration
- `src/jobs.rs`: Job manager implementation
- `src/config.rs`: Environment/config loading
- `src/solana/`: Blockchain logic
- `gen/schemas/`: Integration schemas

---

If any section is unclear or missing, please specify which workflows, conventions, or integration points need more detail.
