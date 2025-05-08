import { withPolkadotSdkCompat } from "polkadot-api/polkadot-sdk-compat";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import {
  createClient,
  FixedSizeBinary,
  PolkadotClient,
  TxFinalizedPayload,
} from "polkadot-api";
import { contracts, westend } from "@polkadot-api/descriptors";
import { getInkClient } from "polkadot-api/ink";
import { CONTRACT_ADDRESS, WS_URL } from "../utils/constants";
import {
  bigintToFixedSizeArray4,
  convertPublicKeyToSs58,
  fixedSizeArray4ToBigint,
  ss58ToH160,
} from "../utils/helpers";
import { InjectedPolkadotAccount } from "polkadot-api/pjs-signer";
import { Binary } from "polkadot-api";
import { ethers } from "ethers";

let clientInstance: PolkadotClient | null = null;

function getClient() {
  if (!clientInstance) {
    console.log("Creating new client instance");

    clientInstance = createClient(withPolkadotSdkCompat(getWsProvider(WS_URL)));
  }
  return clientInstance;
}

// Usage
const client = getClient();

const typedApi = client.getTypedApi(westend);
const polkalend = getInkClient(contracts.polkalend);
const WESTEND_ASSETHUB_H160_DECIMALS = 18;
const WESTEND_ASSETHUB_SS58_DECIMALS = 12;

const getContractCode = async () => {
  const url = "/polkalend.polkavm";
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch contract code: ${response.statusText}`);
  }
  const data = await response.arrayBuffer();
  const binaryData = new Uint8Array(data);
  return binaryData;
};

export const instantiateContract = async (account: InjectedPolkadotAccount) => {
  const binaryData = await getContractCode();
  const tx = typedApi.tx.Revive.instantiate_with_code({
    data: Binary.fromHex("0x00"),
    value: BigInt(0),
    gas_limit: {
      ref_time: BigInt(1e5),
      proof_size: BigInt(1e5),
    },
    storage_deposit_limit: BigInt(1e10),
    code: Binary.fromBytes(binaryData),
    salt: undefined,
  });
  let result = await tx.signAndSubmit(account.polkadotSigner);
  console.log("Instantiate contract result", result);
};

export const instantiateUser = async (account: InjectedPolkadotAccount) => {
  try {
    const ss58Address = convertPublicKeyToSs58(
      account.polkadotSigner.publicKey
    );

    const h160Account = ss58ToH160(ss58Address);

    const mapped = await typedApi.query.Revive.OriginalAccount.getValue(
      h160Account
    );
    console.log("Mapped account", mapped);
    if (mapped) {
      console.log("Already mapped");
      return;
    }
    await typedApi.tx.Revive.map_account().signAndSubmit(
      account.polkadotSigner
    );
  } catch (error) {
    console.error("Error instantiating user:", error);
  }
};

export const getUserBalance = async (
  account: InjectedPolkadotAccount,
  token: string
): Promise<number> => {
  await instantiateUser(account);

  let userBalance = 0;

  if (token === ethers.ZeroAddress) {
    const userAccount = await typedApi.query.System.Account.getValue(
      account.address
    );
    const balance = userAccount.data.free;
    console.log(Number(balance) / 10 ** WESTEND_ASSETHUB_SS58_DECIMALS);
    userBalance = Number(balance) / 10 ** WESTEND_ASSETHUB_SS58_DECIMALS;
  }

  return userBalance;
};

export const getLiquidity = async ({
  lender,
  token,
  account,
}: {
  lender: string;
  token: string;
  account: InjectedPolkadotAccount;
}) => {
  await instantiateUser(account);

  const getLiquidity = polkalend.message("get_liquidity");
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
    const liquidity = responseMessage.value;
    return (
      fixedSizeArray4ToBigint(
        liquidity as unknown as [bigint, bigint, bigint, bigint]
      ) /
      10n ** BigInt(WESTEND_ASSETHUB_H160_DECIMALS)
    );
  }
};

export const createLoan = async ({
  account,
  token,
  amount,
  duration,
}: {
  token: string;
  amount: number;
  duration: bigint;
  account: InjectedPolkadotAccount;
}) => {
  await instantiateUser(account);
  const createLoan = polkalend.message("create_loan");
  let value = 0n;
  let amountWithDecimals = 0n;
  if (token === ethers.ZeroAddress) {
    value = BigInt(Math.trunc(amount * 10 ** WESTEND_ASSETHUB_SS58_DECIMALS));
    amountWithDecimals = BigInt(
      Math.trunc(amount * 10 ** WESTEND_ASSETHUB_H160_DECIMALS)
    );
  } else {
    const ERC20_DECIMALS = 18; // TODO: get from contract
    amountWithDecimals = BigInt(Math.trunc(amount * 10 ** ERC20_DECIMALS));
  }

  const data = createLoan.encode({
    token: FixedSizeBinary.fromHex(token),
    amount: bigintToFixedSizeArray4(amountWithDecimals),
    duration: bigintToFixedSizeArray4(duration),
  });

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
  rethrowContractError(result);
  console.log(
    "tx events",
    polkalend.event.filter(CONTRACT_ADDRESS, result.events)
  );
  return result;
};

export const acceptLoan = async ({
  account,
  lender,
  token,
  amount,
}: {
  lender: string;
  token: string;
  amount: number;
  account: InjectedPolkadotAccount;
}) => {
  await instantiateUser(account);

  let amountToLoan = 0n;

  if (token === ethers.ZeroAddress) {
    amountToLoan = BigInt(
      Math.trunc(amount * 10 ** WESTEND_ASSETHUB_H160_DECIMALS)
    );
  } else {
    const ERC20_DECIMALS = 18; // TODO: get from contract
    amountToLoan = BigInt(amount * 10 ** ERC20_DECIMALS);
  }

  const acceptLoan = polkalend.message("accept_loan");
  const data = acceptLoan.encode({
    token: FixedSizeBinary.fromHex(token),
    amount: bigintToFixedSizeArray4(amountToLoan),
    lender: FixedSizeBinary.fromHex(lender),
  });

  const response = await typedApi.apis.ReviveApi.call(
    account.address,
    FixedSizeBinary.fromHex(CONTRACT_ADDRESS),
    0n,
    undefined,
    undefined,
    data
  );

  const result = await typedApi.tx.Revive.call({
    value: 0n,
    data,
    dest: FixedSizeBinary.fromHex(CONTRACT_ADDRESS),
    gas_limit: response.gas_required,
    storage_deposit_limit: response.storage_deposit.value,
  }).signAndSubmit(account.polkadotSigner);

  rethrowContractError(result);
  console.log(
    "tx events",
    polkalend.event.filter(CONTRACT_ADDRESS, result.events)
  );
  return result;
};

export const payLoan = async ({
  account,
  lender,
  token,
  amount,
}: {
  lender: string;
  token: string;
  amount: number;
  account: InjectedPolkadotAccount;
}) => {
  await instantiateUser(account);

  let amountToPay = 0n;

  if (token === ethers.ZeroAddress) {
    amountToPay = BigInt(
      Math.trunc(amount * 10 ** WESTEND_ASSETHUB_H160_DECIMALS)
    );
  } else {
    const ERC20_DECIMALS = 18; // TODO: get from contract
    amountToPay = BigInt(amount * 10 ** ERC20_DECIMALS);
  }

  const acceptLoan = polkalend.message("pay_loan");
  const data = acceptLoan.encode({
    token: FixedSizeBinary.fromHex(token),
    amount: bigintToFixedSizeArray4(amountToPay),
    lender: FixedSizeBinary.fromHex(lender),
  });

  const response = await typedApi.apis.ReviveApi.call(
    account.address,
    FixedSizeBinary.fromHex(CONTRACT_ADDRESS),
    0n,
    undefined,
    undefined,
    data
  );

  const result = await typedApi.tx.Revive.call({
    value: 0n,
    data,
    dest: FixedSizeBinary.fromHex(CONTRACT_ADDRESS),
    gas_limit: response.gas_required,
    storage_deposit_limit: response.storage_deposit.value,
  }).signAndSubmit(account.polkadotSigner);
  rethrowContractError(result);
  console.log(
    "tx events",
    polkalend.event.filter(CONTRACT_ADDRESS, result.events)
  );
  return result;
};

export const lockCollateral = async ({
  account,
  token,
  amount,
}: {
  token: string;
  amount: number;
  account: InjectedPolkadotAccount;
}) => {
  await instantiateUser(account);

  let amountToLock = 0n;

  if (token === ethers.ZeroAddress) {
    amountToLock = BigInt(
      Math.trunc(amount * 10 ** WESTEND_ASSETHUB_H160_DECIMALS)
    );
  } else {
    const ERC20_DECIMALS = 18; // TODO: get from contract
    amountToLock = BigInt(amount * 10 ** ERC20_DECIMALS);
  }

  const acceptLoan = polkalend.message("lock_collateral");
  const data = acceptLoan.encode({
    collateral_token: FixedSizeBinary.fromHex(token),
    collateral_amount: bigintToFixedSizeArray4(amountToLock),
  });

  const response = await typedApi.apis.ReviveApi.call(
    account.address,
    FixedSizeBinary.fromHex(CONTRACT_ADDRESS),
    0n,
    undefined,
    undefined,
    data
  );

  const result = await typedApi.tx.Revive.call({
    value: 0n,
    data,
    dest: FixedSizeBinary.fromHex(CONTRACT_ADDRESS),
    gas_limit: response.gas_required,
    storage_deposit_limit: response.storage_deposit.value,
  }).signAndSubmit(account.polkadotSigner);

  rethrowContractError(result);
  console.log(
    "tx events",
    polkalend.event.filter(CONTRACT_ADDRESS, result.events)
  );
  return result;
};

const rethrowContractError = (result: TxFinalizedPayload) => {
  if (result.dispatchError) {
    // {"display":{"type":"Module","value":{"type":"Revive","value":{"type":"TransferFailed"}}}}
    console.log(JSON.stringify({ display: result.dispatchError }));
    throw new Error(`${result.dispatchError.type}`);
  }
};
