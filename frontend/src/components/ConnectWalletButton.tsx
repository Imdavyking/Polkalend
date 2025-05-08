import { useWallet } from "../context/WalletContext";
import { ellipsify } from "../utils/ellipsify";

export default function ConnectWalletButton() {
  const { account, isConnecting, connect, disconnect } = useWallet();

  const handleClick = () => {
    if (account?.address) {
      disconnect();
    } else {
      connect();
    }
  };

  return (
    <div className="flex flex-col items-center space-y-2">
      <button
        onClick={handleClick}
        className={`cursor-pointer px-6 py-2 ${
          account?.address
            ? "bg-red-600 hover:bg-red-700"
            : "bg-blue-600 hover:bg-blue-700"
        } text-white font-semibold rounded-lg transition-all duration-300 disabled:opacity-50`}
        disabled={isConnecting}
      >
        {isConnecting
          ? "Connecting..."
          : account?.address
          ? `Disconnect (${ellipsify(account.address)})`
          : "Connect Wallet"}
      </button>
    </div>
  );
}
