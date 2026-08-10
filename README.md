
# Miden Project

A workspace structure for building Miden smart contract applications.

## **Installation**

Before getting started, ensure you have the following prerequisites:

1. **Install Rust** - Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/)

2. **Install midenup toolchain** - Follow the installation instructions at: <https://github.com/0xMiden/midenup>

---

## **How to Test This Project (Quick Start)**

If you are a developer looking to clone, build, and test this project locally or interact with the live Miden Testnet, follow these steps:

### 1. Install Required Toolchains
This project compiles from Rust to Miden Assembly (MASM) using the `cargo-miden` compiler extension:
```bash
# Install the Rust nightly toolchain
rustup toolchain install nightly-2025-12-10

# Install the Miden cargo compiler
cargo install cargo-miden --locked
2. Clone and Navigate to the Repository
code
Bash
git clone https://github.com/anaraydinli55/miden-testnet-project.git
cd miden-testnet-project
3. Run Local Integration Tests
Run the integration tests to simulate account state changes on a local mock chain:
code
Bash
cd integration
cargo test
4. Run Testnet Interaction Script
Execute the interaction script to connect to the live Miden Testnet, fetch the latest block data, register your account, and generate transaction notes on-chain:
code
Bash
cd integration
cargo run --bin increment_count
Structure
code
Text
miden-project/
├── contracts/                   # Each contract as individual crate
│   ├── counter-account/         # Example: Counter account contract
│   └── increment-note/          # Example: Increment note contract
├── integration/                 # Integration crate (scripts + tests)
│   ├── src/
│   │   ├── bin/                 # Rust binaries for on-chain interactions
│   │   ├── config.rs            # Temporary config file (do not modify!)
│   │   ├── helpers.rs           # Temporary helper file (do not modify!)
│   │   └── lib.rs
│   └── tests/                   # Test files
├── Cargo.toml                   # Workspace root
└── rust-toolchain.toml          # Temporary Rust toolchain specification
Design Philosophy
This workspace follows a clean separation of concerns:
Contracts Folder - Miden Development
The contracts/ folder is your primary working directory when writing Miden smart contract code. Each contract is organized as its own individual crate, allowing for:
Independent versioning and dependencies
Clear isolation between different contracts
Easy contract management and modularization
When you're working on Miden Rust code (writing smart contracts), you'll be working in the contracts/ directory.
Integration Crate - Scripts and Testing
The integration/ crate is your working directory for interacting with compiled contracts. All on-chain interactions, scripts, and tests are housed within this single crate. This includes:
Binaries (src/bin/): Rust executables for deploying and interacting with your contracts on-chain
Tests (tests/): Integration tests for validating your contract behavior
This structure provides flexibility as your application grows, allowing you to add custom dependencies, sophisticated tooling, and independent configuration specific to your deployment and testing needs.
Important Note: The helpers.rs file inside the integration/ crate is temporary and exists only to facilitate current development workflows. Do not modify this file unless you know what you are doing! It will be removed in future versions.
Adding New Contracts
To create a new contract crate, run the following command from the workspace root:
code
Bash
miden new --account contracts/my-account
This will scaffold a new contract crate inside the contracts/ directory with all the necessary boilerplate.
Adding Binaries for On-Chain Interactions
Binaries are used for deploying contracts and performing on-chain interactions. To add a new binary:
Create a new .rs file in integration/src/bin/ (e.g., deploy_contract.rs)
Write your binary code as a standard Rust executable with a main() function
Run the binary using the commands shown below
Testing Your Contracts
Tests are located in integration/tests/. To add a new test:
Create a new test file in integration/tests/ (e.g., my_contract_test.rs)
Write your test functions using the standard Rust testing framework
Run tests using the commands shown below
Commands
Compile a Contract
code
Bash
# Compile a specific contract using miden CLI
miden build --manifest-path contracts/counter-account/Cargo.toml

# Or compile using cargo-miden extension directly
cargo miden build --manifest-path contracts/counter-account/Cargo.toml
Run a Binary
code
Bash
# Navigate to integration crate and run a binary
cd integration
cargo run --bin increment_count
Run Tests
code
Bash
# Navigate to integration crate and run tests
cd integration
cargo test                      # Run all tests
cargo test counter_test         # Run specific test file
Extending the Workspace
If you need to extend the workspace with new crates (for example, to add libraries or additional tools), it is recommended to add these new crates in the root of the project directory. This helps keep the project structure clean and makes it easier to manage dependencies and workspace configuration.
To add a new crate to the workspace:
From the project root, run:
code
Bash
cargo new my-new-crate
Then add the crate path (e.g., my-new-crate) to the [workspace].members section of your Cargo.toml.
Note: Avoid adding new crates as subdirectories under contracts/ or integration/, unless they are intended to be contract crates or part of integration specifically. Keeping new crates at the root makes the project easier to understand and maintain.
AI Developer Experience
This template includes resources for AI-assisted development:
CLAUDE.md — Project context loaded automatically by Claude Code
.cursorrules — Project-level guidance for Cursor
.claude/skills/ — On-demand skill files for Miden SDK patterns, pitfalls, testing, and source exploration
.claude/hooks/build-contracts.sh — Automatic build verification after contract edits
code
Code
