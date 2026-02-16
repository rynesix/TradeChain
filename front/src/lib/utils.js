import { Keypair, PublicKey } from "@solana/web3.js";
import bs58 from "bs58";

export const isHttpUrl = (u) => /^https?:\/\/[\w.-]/i.test(u);
export const isWsUrl = (u) => /^wss?:\/\/[\w.-]/i.test(u);
export const isHex = (s) => /^[0-9a-fA-F]*$/.test(s);

export const pad5 = (arr, fill = 0) =>
    [...arr, fill, fill, fill, fill, fill].slice(0, 5);

// Try to derive a public key (base58) from various inputs:
// - already a public key string
// - hex seed (32 bytes) -> Keypair.fromSeed
// - hex secret key (64 bytes) -> Keypair.fromSecretKey
// - base58 public key (32 bytes)
// - base58 secret key (64 bytes)
export function derivePublicKey(input) {
    if (!input) return null;
    try {
        const pk = new PublicKey(input);
        return pk.toBase58();
    } catch (e) {
        // not a direct public key
    }

    const hexToU8 = (hex) => {
        let h = hex.replace(/^0x/, "");
        if (h.length % 2 === 1) h = "0" + h;
        const out = new Uint8Array(h.length / 2);
        for (let i = 0; i < out.length; i++) {
            out[i] = parseInt(h.substr(i * 2, 2), 16);
        }
        return out;
    };

    // hex handling
    try {
        const hex = String(input).trim();
        if (/^[0-9a-fA-F]+$/.test(hex)) {
            const u8 = hexToU8(hex);
            if (u8.length === 32) {
                try {
                    const kp = Keypair.fromSeed(u8);
                    return kp.publicKey.toBase58();
                } catch (e) { }
            }
            if (u8.length === 64) {
                try {
                    const kp = Keypair.fromSecretKey(u8);
                    return kp.publicKey.toBase58();
                } catch (e) { }
            }
        }
    } catch (e) { }

    // base58 handling
    try {
        const decoded = bs58.decode(String(input).trim());
        if (decoded && decoded.length === 32) {
            try {
                return new PublicKey(decoded).toBase58();
            } catch (e) { }
        }
        if (decoded && decoded.length === 64) {
            try {
                const kp = Keypair.fromSecretKey(decoded);
                return kp.publicKey.toBase58();
            } catch (e) { }
        }
    } catch (e) { }

    return null;
}

export default {
    isHttpUrl,
    isWsUrl,
    isHex,
    pad5,
    derivePublicKey,
};

// Load arbitrary JSON from localStorage safely
export function loadJsonFromLocalStorage(key, fallback = null) {
    try {
        const raw = localStorage.getItem(key);
        if (!raw) return fallback;
        return JSON.parse(raw);
    } catch (e) {
        console.warn(`Failed to parse localStorage key ${key}`, e);
        return fallback;
    }
}

// Load bundler/settings style with defaults and padding for arrays
export function loadSettingsFromLocalStorage(key, defaults) {
    try {
        const raw = localStorage.getItem(key);
        if (!raw) return { ...(defaults || {}) };
        const parsed = JSON.parse(raw);
        return {
            ...(defaults || {}),
            ...(parsed || {}),
            buyPresets: Array.isArray(parsed?.buyPresets)
                ? [...parsed.buyPresets]
                    .slice(0, 6)
                    .concat(Array(6 - Math.min(parsed.buyPresets.length, 6)).fill(0))
                : [...(defaults?.buyPresets || [])],
            sellPercents: Array.isArray(parsed?.sellPercents)
                ? [...parsed.sellPercents]
                    .slice(0, 6)
                    .concat(Array(6 - Math.min(parsed.sellPercents.length, 6)).fill(0))
                : [...(defaults?.sellPercents || [])],
        };
    } catch (e) {
        console.warn("Failed to load settings, using defaults", e);
        return { ...(defaults || {}) };
    }
}
