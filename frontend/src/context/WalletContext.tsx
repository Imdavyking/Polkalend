// contexts/WalletContext.tsx
import { createContext, useContext, useState, useEffect } from "react";
import {
  connectWallet,
  disconnectWallet,
  hasConnected,
} from "../services/connect.wallet.services";
import { InjectedPolkadotAccount } from "polkadot-api/pjs-signer";
import { CONNECT_WALLET_KEY_STORAGE } from "../utils/constants";

interface WalletContextType {
  account: InjectedPolkadotAccount | null;
  isConnecting: boolean;
  connect: () => Promise<void>;
  disconnect: () => void;
}

const WalletContext = createContext<WalletContextType>({
  account: null,
  isConnecting: false,
  connect: async () => {},
  disconnect: () => {},
});

export const WalletProvider = ({ children }: { children: React.ReactNode }) => {
  const [account, setAccount] = useState<InjectedPolkadotAccount | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);

  useEffect(() => {
    const autoConnect = async () => {
      if (hasConnected()) {
        await connect();
      }
    };
    autoConnect();
  }, []);

  const connect = async () => {
    setIsConnecting(true);
    try {
      const account = await connectWallet();
      setAccount(account);
    } catch (err) {
      console.error("Connection error:", err);
    } finally {
      setIsConnecting(false);
    }
  };

  const disconnect = () => {
    setAccount(null);
    localStorage.removeItem(CONNECT_WALLET_KEY_STORAGE);
    disconnectWallet();
  };

  return (
    <WalletContext.Provider
      value={{ account, isConnecting, connect, disconnect }}
    >
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = () => useContext(WalletContext);
