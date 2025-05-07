import { useState } from "react";
import { connectWallet } from "../services/connect.wallet.services";
import { ellipsify } from "../utils/ellipsify";

export default function ConnectWalletButton() {
  const [address, setAddress] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState("");

  const handleConnect = async () => {
    setIsConnecting(true);
    setError("");
    try {
      const account = await connectWallet();
      setAddress(account.address);
    } catch (err) {
      console.error(err);
      setError("Failed to connect wallet.");
    } finally {
      setIsConnecting(false);
    }
  };

  return (
    <div className="flex flex-col items-center space-y-4">
      <button
        onClick={handleConnect}
        className="px-6 py-2 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition-all duration-300 disabled:opacity-50"
        disabled={isConnecting || !!address}
      >
        {address
          ? ellipsify(address)
          : isConnecting
          ? "Connecting..."
          : "Connect Wallet"}
      </button>
      {error && <p className="text-red-500">{error}</p>}
    </div>
  );
}
