import { withPolkadotSdkCompat } from "polkadot-api/polkadot-sdk-compat";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import { createClient, FixedSizeBinary } from "polkadot-api";
import { contracts, MultiAddress, westend } from "@polkadot-api/descriptors";
import { getInkClient } from "polkadot-api/ink";
import { CONTRACT_ADDRESS } from "../utils/constants";
import { connectWallet } from "./connect.wallet.services";
const client = createClient(
  withPolkadotSdkCompat(
    getWsProvider("wss://westend-asset-hub-rpc.polkadot.io")
  )
);
const typedApi = client.getTypedApi(westend);
const polkalend = getInkClient(contracts.polkalend);
const WESTEND_ASSETHUB_DECIMALS = 18;
export const getLiquidity = async ({
  lender,
  token,
}: {
  lender: string;
  token: string;
}) => {
  const getLiquidity = polkalend.message("get_liquidity");
  const account = await connectWallet();
  const data = getLiquidity.encode({
    lender: FixedSizeBinary.fromHex(lender),
    token: FixedSizeBinary.fromHex(token),
  });

  const response = await typedApi.apis.ReviveApi.call(
    account.address,
    FixedSizeBinary.fromHex(CONTRACT_ADDRESS),
    0n,
    undefined,
    undefined,
    data
  );
  if (response.result.success) {
    const responseMessage = getLiquidity.decode(response.result.value);
    console.log("Result response", responseMessage);
    return responseMessage;
  }
};

export const createLoan = async ({
  token,
  amount,
  duration,
}: {
  token: string;
  amount: bigint;
  duration: bigint;
}) => {
  const createLoan = polkalend.message("create_loan");
  const data = createLoan.encode({
    token: FixedSizeBinary.fromHex(token),
    amount,
    duration,
  });

  const value = amount * BigInt(10 ** WESTEND_ASSETHUB_DECIMALS);
  const account = await connectWallet();
  const response = await typedApi.apis.ReviveApi.call(
    account.address,
    FixedSizeBinary.fromHex(CONTRACT_ADDRESS),
    value,
    undefined,
    undefined,
    data
  );

  const result = await typedApi.tx.Revive.call({
    value,
    data,
    dest: FixedSizeBinary.fromHex(CONTRACT_ADDRESS),
    gas_limit: response.gas_required,
    storage_deposit_limit: response.storage_deposit.value,
  }).signAndSubmit(account.polkadotSigner);
  console.log(
    "tx events",
    polkalend.event.filter(CONTRACT_ADDRESS, result.events)
  );
  return result;
};
