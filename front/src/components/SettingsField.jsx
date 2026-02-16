import React from "react";

export default function SettingsField({ label, error, hint, children }) {
  return (
    <div className="space-y-1">
      <label className="text-xs text-white/70">{label}</label>
      {children}
      {error ? (
        <p className="text-xs text-red-400">{error}</p>
      ) : hint ? (
        <p className="text-xs text-white/50">{hint}</p>
      ) : null}
    </div>
  );
}
