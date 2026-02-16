// File: src/tabs/registry.jsx
import Bundler from "../views/Bundler";
import Swap from "../views/Swap";
import Wallets from "../views/Wallets";
import Settings from "../views/Settings";

const TABS = {
  "Pump.fun": {
    label: "Pump.fun bundler",
    rightPanel: false,
    render: (props) => <Bundler {...props} />,
  },
  Swap: {
    label: "Swap",
    rightPanel: false,
    render: (props) => <Swap {...props} />,
  },
  Wallets: {
    label: "Wallets",
    rightPanel: true, // -> adds right sidebar margin
    render: (props) => <Wallets {...props} />,
  },
  Settings: {
    label: "Settings",
    rightPanel: false,
    render: () => <Settings />,
  },
};

export default TABS;
