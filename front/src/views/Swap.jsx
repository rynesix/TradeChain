// File: src/views/Swap.jsx
import { useCallback, useEffect, useMemo, useState } from "react";
import { useApp } from "../context/AppContext";
import { api, isTauriEnv } from "../lib/tauriClient";
import { loadSettingsFromLocalStorage as loadSettingsFromLS } from "../lib/utils";
import SwapPanel from "../components/bundler/SwapPanel";
import { useBalanceRefresh } from "../hooks/useBalanceRefresh";
import Toast from "../components/Toast";

const LS_KEY = "mc_settings_v1";

const SETTINGS_DEFAULTS = {
  slippageBps: 100,
  tipSol: 0.0005,
  cuPriceMicrolamports: 0,
  maxUnitPriceMicrolamports: 0,
  buyPresets: [0.05, 0.1, 0.2, 0.5, 1, 2],
  sellPercents: [5, 10, 25, 50, 75, 100],
  enableToasts: true,
};

export default function Swap() {
  const {
    tokens,
    setTokens,
    mint: appMint,
    setMint,
    groups,
    setGroups,
    activeTab,
  } = useApp();
  const [settings, setSettings] = useState(() =>
    loadSettingsFromLS(LS_KEY, SETTINGS_DEFAULTS)
  );
  const [toast, setToast] = useState(null);
  const tauriAvailable = isTauriEnv();

  // ensure dev group exists and is first
  const groupsWithDev = useMemo(() => {
    const out = [...(groups ?? [])];
    const devIdx = out.findIndex((g) => String(g.name).toLowerCase() === "dev");
    if (devIdx === -1) {
      out.unshift({ name: "dev", wallets: [] });
    } else {
      const dev = out.splice(devIdx, 1)[0];
      out.unshift(dev);
    }
    return out;
  }, [groups]);

  const devGroup = groupsWithDev.find(
    (g) => String(g.name).toLowerCase() === "dev"
  );
  const devWalletItem = devGroup?.wallets?.[0];
  const sniperGroup = groupsWithDev.find(
    (g) => String(g.name).toLowerCase() === "sniper"
  );
  const sniperWalletItem = sniperGroup?.wallets?.[0];

  // Balance refresher (SOL + token balances for current appMint)
  const { refreshGroupBalances } = useBalanceRefresh(
    groupsWithDev,
    setGroups,
    settings,
    appMint
  );

  // Private code
}
