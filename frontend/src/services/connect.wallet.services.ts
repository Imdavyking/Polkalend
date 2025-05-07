import {
  getInjectedExtensions,
  connectInjectedExtension,
  InjectedExtension,
  InjectedPolkadotAccount,
} from "polkadot-api/pjs-signer";

export const connectWallet = async () => {
  // Get the list of installed extensions
  const extensions: string[] = getInjectedExtensions();

  // Connect to an extension
  const selectedExtension: InjectedExtension = await connectInjectedExtension(
    extensions[0]
  );

  // Get accounts registered in the extension
  const accounts: InjectedPolkadotAccount[] = selectedExtension.getAccounts();

  // The signer for each account is in the `polkadotSigner` property of `InjectedPolkadotAccount`
  localStorage.setItem("connectedWallet", accounts[0].address);
  return accounts[0];
};

export function hasConnected() {
  const isConnectedBefore = localStorage.getItem("connectedWallet");

  return !!isConnectedBefore;
}
