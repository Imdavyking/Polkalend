import { useState } from "react";
import { tokens } from "../../utils/constants";
import { toast } from "react-toastify";
import { FaSpinner } from "react-icons/fa";
import { createLoan } from "../../services/blockchain.services";
import { useWallet } from "../../context/WalletContext";
export default function CreateLoan() {
  const [selectedToken, setSelectedToken] = useState(tokens[0]);
  const [amount, setAmount] = useState("");
  const [duration, setDuration] = useState("");
  const [creatingLoan, setCreatingLoan] = useState(false);
  const { account } = useWallet();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    if (!amount) {
      toast.error("Please enter a valid amount");
      return;
    }

    if (!duration) {
      toast.error("Please enter a valid duration");
      return;
    }

    onSubmit({
      token: selectedToken.address,
      amount,
      duration,
    });
  };

  const onSubmit = async (data: {
    token: string;
    amount: string;
    duration: string;
  }) => {
    try {
      const { token, amount, duration } = data;
      if (!token) {
        toast.error("Please select a token");
        return;
      }
      setCreatingLoan(true);

      if (!account) {
        toast.error("Please connect your wallet");
        return;
      }

      await createLoan({
        account: account,
        token,
        amount: Number(amount),
        duration: BigInt(duration),
      });
    } catch (error) {
      console.error("Error creating loan offer:", error);
      toast.error("Failed to create loan offer. Please try again.");
    } finally {
      setCreatingLoan(false);
    }
  };

  return (
    <div className="max-w-md mx-auto bg-white shadow-lg rounded-xl p-6 space-y-4">
      <h2 className="text-xl font-bold text-gray-800">Create Loan Offer</h2>

      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Token Dropdown */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Select Token
          </label>
          <select
            onChange={(e) =>
              setSelectedToken(
                tokens.find((t) => t.address === e.target.value)!
              )
            }
            className="w-full p-2 border rounded-md focus:ring focus:ring-indigo-500"
          >
            {tokens.map((token) => (
              <option key={token.address} value={token.address}>
                {token.name}
              </option>
            ))}
          </select>

          <div className="flex items-center mt-2 gap-2">
            <img
              src={selectedToken.image}
              alt={selectedToken.name}
              className="w-6 h-6"
            />
          </div>
        </div>

        {/* Amount */}
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Amount
          </label>
          <input
            type="number"
            min="1"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="w-full p-2 border rounded-md focus:ring focus:ring-indigo-500"
            placeholder="Enter loan amount"
          />
        </div>

        {/* Duration */}
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Duration (seconds)
          </label>
          <input
            type="number"
            min="1"
            value={duration}
            onChange={(e) => setDuration(e.target.value)}
            className="w-full p-2 border rounded-md focus:ring focus:ring-indigo-500"
            placeholder="e.g. 86400 for 1 day"
          />
        </div>

        <button
          type="submit"
          className="w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 flex items-center justify-center transition duration-200"
        >
          {creatingLoan ? (
            <FaSpinner className="w-5 h-5 animate-spin" />
          ) : (
            "Submit Loan Offer"
          )}
        </button>
      </form>
    </div>
  );
}
