// File: src/views/Bundler.jsx
import { useMemo, useState, useEffect, useCallback, useRef } from "react";
import { ClipboardCopy } from "lucide-react";
import ConfigureBundlerModal from "../components/modals/ConfigureBundlerModal";
import AutoBuyPanel from "../components/bundler/AutoBuyPanel";
import TokenCreatorPanel from "../components/bundler/TokenCreatorPanel";
import { useApp } from "../context/AppContext";
import { useBalanceRefresh } from "../hooks/useBalanceRefresh";
import { api, isTauriEnv } from "../lib/tauriClient";
import {
  loadSettingsFromLocalStorage as loadSettingsFromLS,
  loadJsonFromLocalStorage,
} from "../lib/utils";
import SolanaIcon from "../components/icons/SolanaIcon";
import Toast from "../components/Toast";
import BundlerHeader from "../components/bundler/BundlerHeader";
import BundlesGrid from "../components/bundler/BundlesGrid";
import SwapPanel from "../components/bundler/SwapPanel";

const LS_KEY = "mc_settings_v1";
const LS_GROUP_PRESETS = "mc_group_presets_v1";

const SETTINGS_DEFAULTS = {
  rpcUrl: "https://api.mainnet-beta.solana.com",
  wsUrl: "wss://api.mainnet-beta.solana.com",
  fundingPk: "",
  slippageBps: 100,
  tipSol: 0.0005,
  cuPriceMicrolamports: 0,
  maxUnitPriceMicrolamports: 0,
  buyPresets: [0.05, 0.1, 0.2, 0.5, 1, 2],
  sellPercents: [5, 10, 25, 50, 75, 100],
  autoRefreshSec: 20,
  targetScope: "selected",
  maxSnipers: 3,
  compactRows: false,
  enableToasts: true,
};

// settings loader moved to src/lib/utils

// Local helpers
const loadGroupPresets = () => loadJsonFromLocalStorage(LS_GROUP_PRESETS, {});

export default function Bundler({ groups, activeGroupName }) {
  // Private code
}
