# Token Claim Smart Contract

## Overview

This project is a smart contract written in Rust for the MultiversX blockchain platform. It allows users to claim a fixed amount of tokens by calling the `claimTokens` endpoint. The contract is designed to demonstrate basic smart contract functionality, including token transfer operations.

## Design

- **Language**: The contract is implemented in Rust and compiled to WebAssembly (Wasm), which is compatible with the MultiversX platform.
- **Smart Contract Library**: Utilizes `multiversx-sc` to facilitate smart contract development.
- **Functionality**: Provides a single endpoint, `claimTokens`, which sends a fixed amount of tokens to the caller.
- **Token Amount**: The contract is configured to transfer 100 tokens to the caller by default. This amount can be adjusted in the source code.

## Prerequisites

- **Rust**: Make sure you have Rust installed on your system. Follow the installation guide at [rust-lang.org](https://www.rust-lang.org/tools/install).
- **MultiversX SDK**: Install the MultiversX SDK and CLI tools. Refer to the [MultiversX SDK documentation](https://docs.multiversx.com/sdk-and-tools/overview) for detailed installation instructions.

## Setup

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/yourusername/token_claim_contract.git
   cd token_claim_contract
Install Dependencies: Ensure that the required Rust crates are available by running:
cargo build
Building and Deployment
Compile to WebAssembly: Compile the smart contract to Wasm using the following command:

cargo build --release --target wasm32-unknown-unknown
Deploy to MultiversX: Use the MultiversX CLI tools to deploy the contract to the Devnet. Follow the instructions in the MultiversX deployment guide for details.

Verify Deployment: After deploying, verify the contract's status and interactions using the Devnet Explorer.

Interaction
Calling the claimTokens Endpoint: To claim tokens, interact with the deployed contract by calling the claimTokens endpoint. This can be done using the MultiversX API or SDK.

Devnet API: Use the Devnet API to programmatically interact with the contract, send transactions, and query blockchain data.

File Structure
Copy
token_claim_contract/
├── Cargo.toml
├── src/
│   └── lib.rs
├── README.md
├── LICENSE
└── .gitignore
Cargo.toml: Contains package metadata and dependencies.
src/lib.rs: Main source file containing the smart contract code.
README.md: Documentation file providing an overview and instructions.
LICENSE: License file specifying usage terms.
.gitignore: Specifies files and directories to be ignored by Git.
License
This project is licensed under the MIT License. See the LICENSE file for details.

Resources
MultiversX Developer Reference
Devnet Explorer
Devnet API
Rust Language