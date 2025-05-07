import { useWallet } from "../context/WalletContext";
import { ellipsify } from "../utils/ellipsify";

export default function ConnectWalletButton() {
  const { account, isConnecting, connect } = useWallet();

  return (
    <div className="flex flex-col items-center space-y-4">
      <button
        onClick={connect}
        className="px-6 py-2 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition-all duration-300 disabled:opacity-50"
        disabled={isConnecting || !!account?.address}
      >
        {account?.address
          ? ellipsify(account?.address)
          : isConnecting
          ? "Connecting..."
          : "Connect Wallet"}
      </button>
    </div>
  );
}
