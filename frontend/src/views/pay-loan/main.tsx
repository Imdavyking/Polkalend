import { useEffect, useState } from "react";
import { tokens } from "../../utils/constants";
import TokenDropdown from "../../components/TokenDropdown";
import TextInput from "../../components/TextInput";
import NumberInput from "../../components/DurationInput";
import SubmitButton from "../../components/SubmitButton";
import { useWallet } from "../../context/WalletContext";
import { acceptLoan, getLiquidity } from "../../services/blockchain.services";
import { toast } from "react-toastify";

interface Token {
  name: string;
  address: string;
  image: string;
}

export default function PayLoan() {
  const lender = "0x95f5af38f10492ad29ac06086846b8c6f9509f51";
  const [amount, setAmount] = useState("");
  const [selectedLoanToken, setSelectedLoanToken] = useState<Token>(tokens[0]);
  const [loading, setLoading] = useState(false);
  const [balance, setBalance] = useState("");
  const { account } = useWallet();

  useEffect(() => {
    (async () => {
      if (!account) return;
      const balance = await getLiquidity({
        lender,
        token: selectedLoanToken.address,
        account,
      });

      if (typeof balance === "undefined") {
        return;
      }
      setBalance(balance.toString());
    })();
  }, [account, selectedLoanToken]);

  const handlePayLoan = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      if (!account) {
        toast.error("Please connect your wallet");
        return;
      }
      await acceptLoan({
        lender,
        token: selectedLoanToken.address,
        amount: +amount,
        account: account,
      });
      toast.success("Loan accepted successfully!");
    } catch (error) {
      console.error("Error:", error);
      if (error instanceof Error) {
        toast.error(error.message);
      } else {
        toast.error("An error occurred");
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <div className="max-w-md mx-auto bg-white shadow-lg rounded-xl p-6 space-y-4">
        <h2 className="text-xl font-bold text-gray-800">Pay Loan</h2>
        <form onSubmit={handlePayLoan} className="space-y-4">
          <TextInput
            defaultValue={lender}
            onChange={() => {}}
            placeholder="0xLender..."
            label="Lender Address"
            disabled={true}
          />
          <TokenDropdown
            label="Loan Token"
            tokens={tokens}
            selectedToken={selectedLoanToken}
            setSelectedToken={setSelectedLoanToken}
            balance={balance}
          />
          <NumberInput
            label="Loan Amount"
            placeholder="1000"
            defaultValue={amount}
            onChange={(value) => setAmount(value)}
          />
          <SubmitButton isSubmitting={loading} />
        </form>
      </div>
    </>
  );
}
