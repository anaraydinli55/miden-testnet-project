# 🌉 SAKASENA Cross-Chain Bridge

Cross-chain bridge connecting **Polygon Miden** ↔ **EVM** via AggLayer.

## Deployments

| Network | Contract | Address |
|---------|----------|---------|
| Miden Testnet | bridge-lockbox | `0x5eb65e512ab979911ec04e6798ead0` |
| Sepolia | SKSBridge | `0x90cbAe500C2c008B58656f474d4e35F5B7A7996a` |

## Quick Start

```bash
cd integration
cargo test bridge_lock_test
cargo run --bin bridge_deposit
cargo run --bin bridge_query
MIT
