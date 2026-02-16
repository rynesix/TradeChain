// File: src/context/JobContext.jsx
import { createContext, useContext } from "react";
import useJobHistory from "../hooks/useJobHistory";

const JobContext = createContext();

export const JobProvider = ({ children }) => {
  const jobHistoryHook = useJobHistory();

  return (
    <JobContext.Provider value={jobHistoryHook}>{children}</JobContext.Provider>
  );
};

export const useJobContext = () => {
  const context = useContext(JobContext);
  if (!context) {
    throw new Error("useJobContext must be used within a JobProvider");
  }
  return context;
};

export default JobContext;
