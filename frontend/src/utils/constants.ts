import usdc from "../assets/images/usdc.png";
import polkadot from "../assets/images/polkadot.png";
export const tokens = [
  {
    name: "DOT",
    address: "0x0000000000000000000000000000000000000000", // native
    image: polkadot,
  },
  {
    name: "USDC",
    address: "0x2222222222222222222222222222222222222222",
    image: usdc,
  },
];
export const CONTRACT_ADDRESS = import.meta.env.VITE_CONTRACT_ADDRESS;
