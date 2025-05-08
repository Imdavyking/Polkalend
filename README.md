# **Interest-Free P2P Lending Protocol**

## **Overview**

This is a decentralized protocol built on the **Polkadot** blockchain, offering **interest-free loans** backed by collateral. Inspired by the **Liquity** protocol, our goal is to provide a stable and decentralized way for users to borrow funds without the burden of interest rates, using a **collateralized debt position (CDP)** model.

### **Problem Statement**

Traditional lending often involves high interest rates, adding significant financial strain on borrowers. This protocol aims to provide a better alternative by offering **interest-free loans** while maintaining protocol stability through a **collateralized** mechanism.

---

## **How It Works**

The protocol enables users to **borrow stablecoins** without paying interest by locking collateral. The loan is secured by the collateral, and if the value of the collateral falls below a set threshold, it can be liquidated to repay the loan. This ensures that the protocol remains solvent without charging interest to borrowers.

### **Core Features**

- **Interest-Free Loans**: Loans are offered without interest, relying on collateral to secure the loan value.
- **Collateralized Debt Positions (CDPs)**: Users lock collateral to take out loans. The collateral value must exceed the loan amount by a predetermined ratio (e.g., 110%).
- **Liquidation Mechanism**: If the collateral value drops below the required threshold, it is liquidated to repay the loan.
- **Stability Pool**: A reserve pool that absorbs liquidated collateral to maintain the protocol's stability.

---

## **How to Use**

### **For Borrowers**

1. Deposit collateral (e.g., DOT) to secure a loan.
2. Specify the amount you wish to borrow (in stablecoins).
3. Borrow funds at no interest, ensuring your collateral remains above the required ratio.
4. Repay the loan when ready to unlock your collateral.

### **For Lenders**

1. Provide collateral to the **Stability Pool**.
2. Earn rewards through **liquidation fees** or other incentives for participating in the protocol.
3. Help maintain protocol stability by absorbing collateral when borrowers fail to maintain collateral ratios.

---

## **Future Improvements**

- **Governance**: Introduce decentralized governance to allow users to vote on protocol changes (e.g., adjusting collateral ratios).
- **Interest-free Loan Modifications**: Implement more advanced mechanisms to reduce liquidation risks, such as multiple collateral types or a dynamic collateral ratio.
- **Collateralized Token**: Introduce a new collateral-backed token for additional stability.

---

## **Contributing**

We welcome contributions! If you’d like to contribute to this project, please fork the repository and submit a pull request.

1. Fork the repository
2. Create a new branch for your changes
3. Make your changes
4. Submit a pull request

---

## Prerequisites

1. **Rust Setup**:

   - Make sure you have Rust installed. You can install it using:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

2. **Cargo Contract**:

   - Install `cargo-contract@v6` which is required to build Ink! smart contracts:
     ```bash
     cargo install cargo-contract --version 6.0.0-alpha --locked
     ```

3. **Substrate Node**:
   - You'll need a local Substrate-based blockchain or Polkadot development environment like a `Canvas` node or `substrate-node-template` to deploy and interact with the smart contract.

### Running the Local Development Chain

1. Run a local Substrate development chain:
   - If you have `substrate-contracts-node`, you can start it by running:
     `bash
substrate-contracts-node --dev --tmp
`

---

## ✅ Build and Map Your Account for the Marketplace Contract

### 1. Compile the Smart Contract

Use the following command to compile your ink! contract in release mode:

```bash
cargo contract build --release
```

This will generate a `.contract` file in the `target/ink/` directory. This file contains:

- The contract’s compiled WebAssembly bytecode (`.wasm`)
- Metadata (`.json`)
- Everything needed to deploy the contract

---

### 2. Map Your Account (Required Before Using the Contract)

Before interacting with the contract, you must **register (map) your account** using a runtime extrinsic. This step prevents the `AccountUnmapped` error.

#### 🧭 How to Do It in Polkadot.js Apps

1. Navigate to [**Developer → Extrinsics**](https://polkadot.js.org/apps/#/extrinsics)
2. **Select your account** in the "using the selected account" dropdown
3. For the "submit the following extrinsic" section:

   - Choose the **pallet** (`revive`)
   - Select the **`mapAccount`** function

4. Leave any required parameters (if any) blank or fill as needed
5. Click **Submit Transaction**
6. Sign and confirm the transaction

Once complete, your account will be mapped and you can interact with the contract.

---

### Deploying the Contract

https://faucet.polkadot.io/

1. Go to the PolkadotJS Apps portal:

   - Open [UI ink](https://inkv6alpha.netlify.app/).

2. Upload and Deploy the Contract:
   - Navigate to the "Contracts" tab and click on "Deploy Contract."
   - Upload the `.contract` file that was generated during the build process.
   - Set the initial parameters (if any) and deploy the contract.

### Interacting with the Contract

1. Once deployed, you can interact with the contract via the PolkadotJS UI
2. You can also run tests or write scripts to interact with the contract programmatically.

### Testing

- If you want to test the contract logic before deployment, write unit tests inside the contract using `#[ink::test]` and run:
  ```bash
  cargo test
  ```

### Known Issues & Troubleshooting

- **WebSocket Compatibility (Brave Browser):**
  WebSocket connections may not function reliably in the **Brave browser** due to strict security settings. Switching to **Chrome** or another browser resolves the issue.

- **Contract Event Emission Issue on `ink! v6` Playground:**
  When deploying contracts via the [ink! v6 Alpha Playground](https://inkv6alpha.netlify.app/), attempting to emit events results in the following error:

  ```
  CodeRejected: The contract failed to compile or is missing the correct entry points.
  A more detailed error can be found on the node console if debug messages are enabled by supplying -lruntime::revive=debug
  ```

  This error typically occurs because:

  - The emitted event isn't recognized by the runtime.
  - The contract does not meet expected structure or metadata for `ink! v6`.

  **Impact:**
  Due to this limitation, tools like **SubQuery** cannot reliably index contract events, making it difficult to query them using GraphQL.

---

## **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## **Acknowledgments**

- Inspired by **Liquity Protocol** (interest-free loan model).
- Built using **Polkadot** and **Ink!** for decentralized smart contracts.
