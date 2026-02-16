// File: src/context/AppContext.jsx
import { createContext, useContext, useState, useEffect } from "react";
import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";

const AppContext = createContext(null);

const LS_GROUPS_KEY = "mc_groups_v1";
const LS_ACTIVE_GROUP_KEY = "mc_active_group_v1";

// Private code