import Sidebar from "./components/Sidebar";
import { useApp } from "./context/AppContext";
import TABS from "./tabs/registry";
import "./App.css";
import "./Sidebar.css";

export default function App() {
  const { activeTab, groups, activeGroupName } = useApp();
  const tabEntry = TABS[activeTab] || TABS["Pump.fun"];
  const hasRightPanel = !!tabEntry.rightPanel;
  const View = tabEntry.render;

  return (
    <div className="root-mc bg-zinc-900 text-white min-h-screen font-sans">
      <Sidebar />
      <main
        className={[
          "min-h-screen",
          "pl-[var(--left-sidebar-w)]",
          hasRightPanel ? "pr-[var(--right-sidebar-w)]" : "",
          "transition-[padding] duration-200 ease-out",
        ].join(" ")}
      >
        <View groups={groups} activeGroupName={activeGroupName} />
      </main>
    </div>
  );
}
