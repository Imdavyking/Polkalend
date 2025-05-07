import {
  getInjectedExtensions,
  connectInjectedExtension,
  InjectedExtension,
  InjectedPolkadotAccount,
} from "polkadot-api/pjs-signer";
import { withPolkadotSdkCompat } from "polkadot-api/polkadot-sdk-compat";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import { createClient, FixedSizeBinary } from "polkadot-api";
import { contracts, westend } from "@polkadot-api/descriptors";
import { getInkClient } from "polkadot-api/ink";

export const connectWallet = async () => {
  const client = createClient(
    withPolkadotSdkCompat(
      getWsProvider("wss://westend-asset-hub-rpc.polkadot.io")
    )
  );

  const typedApi = client.getTypedApi(westend);
  const polkalend = getInkClient(contracts.polkalend);
  // Get the list of installed extensions
  const extensions: string[] = getInjectedExtensions();

  // Connect to an extension
  const selectedExtension: InjectedExtension = await connectInjectedExtension(
    extensions[0]
  );

  // Get accounts registered in the extension
  const accounts: InjectedPolkadotAccount[] = selectedExtension.getAccounts();

  // The signer for each account is in the `polkadotSigner` property of `InjectedPolkadotAccount`
  const polkadotSigner = accounts[0].polkadotSigner;

  const getLiquidity = polkalend.message("get_liquidity");
  const data = getLiquidity.encode({
    lender: FixedSizeBinary.fromHex(
      "0x5801b439a678d9d3a68b8019da6a4abfa507de11"
    ),
    token: FixedSizeBinary.fromHex(
      "0x5801b439a678d9d3a68b8019da6a4abfa507de11"
    ),
  });

  // apis.ContractsApi.call with apis.ReviveApi.call or tx.Contracts.call with tx.Revive.call h

  const response = await typedApi.apis.ReviveApi.call(
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", // Alice account
    FixedSizeBinary.fromHex("0x5801b439a678d9d3a68b8019da6a4abfa507de11"), // SC address
    0n, // value
    undefined,
    undefined,
    data
  );
  if (response.result.success) {
    const responseMessage = getLiquidity.decode(response.result.value);
    console.log("Result response", responseMessage);
  }
};
