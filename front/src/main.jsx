// File: src/main.jsx
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.jsx";
import { AppProvider } from "./context/AppContext.jsx";
import { JobProvider } from "./context/JobContext.jsx";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <AppProvider>
      <JobProvider>
        <App />
      </JobProvider>
    </AppProvider>
  </React.StrictMode>
);
