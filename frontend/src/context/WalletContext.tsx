// contexts/WalletContext.tsx
import { createContext, useContext, useState, useEffect } from "react";
import {
  connectWallet,
  hasConnected,
} from "../services/connect.wallet.services";
import { InjectedPolkadotAccount } from "polkadot-api/pjs-signer";

interface WalletContextType {
  account: InjectedPolkadotAccount | null;
  isConnecting: boolean;
  connect: () => Promise<void>;
}

const WalletContext = createContext<WalletContextType>({
  account: null,
  isConnecting: false,
  connect: async () => {},
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

  return (
    <WalletContext.Provider value={{ account, isConnecting, connect }}>
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = () => useContext(WalletContext);
