// File: src/components/Sidebar.jsx
import { useApp } from "../context/AppContext";
import MemeCoreLogo from "../assets/meme-core.png";
import WalletsSidebarPanel from "./wallets/WalletsSidebarPanel";
import TABS from "../tabs/registry";
import "../Sidebar.css";

export default function Sidebar() {
  const { activeTab, setActiveTab } = useApp();
  const items = Object.keys(TABS);

  return (
    <>
      <aside className="sidebar-left fixed left-0 top-0 h-full bg-gray-950 border-r border-gray-800 p-4 flex flex-col gap-6 z-40">
        <div className="flex items-center gap-2">
          <img
            src={MemeCoreLogo}
            alt="MemeCore Logo"
            className="h-7 w-7 drop-shadow-md"
          />
          <h1 className="text-xl font-bold text-white tracking-tight">
            MemeCore
          </h1>
        </div>

        <nav className="flex flex-col gap-1.5 text-sm">
          {items.map((item) => {
            const isActive = activeTab === item;
            return (
              <button
                key={item}
                onClick={() => setActiveTab(item)}
                className={`text-left px-2 py-2 rounded-lg transition duration-200 font-medium ${
                  isActive
                    ? "text-white bg-white/10 border border-white/10 shadow"
                    : "text-gray-300 hover:text-white hover:bg-white/[0.06]"
                }`}
                aria-current={isActive ? "page" : undefined}
              >
                {TABS[item].label}
              </button>
            );
          })}
        </nav>

        <div className="mt-auto text-xs text-gray-400 space-y-1">
          <div className="text-white/80">kama_x2 (rental)</div>
          <div className="flex justify-between text-gray-400 text-[11px]">
            <span>JITO</span>
            <span>RPC</span>
          </div>
        </div>
      </aside>

      {activeTab === "Wallets" && <WalletsSidebarPanel />}
    </>
  );
}
