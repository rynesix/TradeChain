// File: src/lib/tauriClient.js

// Detect Tauri (v2 or v1)
export function isTauriEnv() {
  if (typeof window === "undefined") return false;
  return Boolean(window.__TAURI_INTERNALS__ || window.__TAURI__);
}

let _invoke = null;

// Load invoke (prefer v2, fallback v1). Mark dynamic imports with @vite-ignore.
async function loadInvoke() {
  if (_invoke) return _invoke;
  if (!isTauriEnv()) {
    throw new Error("Tauri IPC not available. Launch the app with `tauri dev`.");
  }

  // Try Tauri v2 (@tauri-apps/api/core)
  try {
    const mod = await import(/* @vite-ignore */ "@tauri-apps/api/core");
    _invoke = mod.invoke;
    return _invoke;
  } catch (_) {
    // continue
  }

  // Fallback Tauri v1 (@tauri-apps/api/tauri)
  try {
    const v1Path = "@tauri-apps/api/tauri"; // keep in a var so Vite won't prebundle
    const modOld = await import(/* @vite-ignore */ v1Path);
    _invoke = modOld.invoke;
    return _invoke;
  } catch (e) {
    throw new Error(
      "Failed to load Tauri API. Ensure `@tauri-apps/api` is installed (v2 uses '@tauri-apps/api/core')."
    );
  }
}

// Simple timeout helper
function withTimeout(promise, ms, label = "invoke") {
  return new Promise((resolve, reject) => {
    const t = setTimeout(() => reject(new Error(`Timeout ${ms}ms for ${label}`)), ms);
    promise.then((v) => { clearTimeout(t); resolve(v); })
      .catch((e) => { clearTimeout(t); reject(e); });
  });
}

// Public invoke wrapper
export async function tauriInvoke(cmd, payload = {}, timeoutMs = 15000) {
  const invoke = await loadInvoke();
  return withTimeout(invoke(cmd, payload), timeoutMs, `command "${cmd}"`);
}

// Convenience API
export const api = {
  ping: () => tauriInvoke("ping"),
  quickBuy: (req) => tauriInvoke("quick_buy", { req }),
  quickSell: (req) => tauriInvoke("quick_sell", { req }),
  refreshBalances: (groupName) => tauriInvoke("refresh_balances", { req: { group_name: groupName } }),
  // Returns list of tokens for a given wallet address with optional details
  // Expected backend response shape example:
  // { ok: true, tokens: [{ mint, name, symbol } ...] } or directly [{...}]
  // Only call this from WalletTokensModal and on Swap view launch
  getTokensBalances: (wallet) => tauriInvoke("get_tokens_balances", { wallet }),
  // Returns the token balance for a specific mint in a wallet
  getTokenBalance: (wallet, mint) => tauriInvoke("get_token_balance", { wallet, mint }),
};
