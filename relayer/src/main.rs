use anyhow::Result;
use log::info;

/// SAKASENA Bridge Relayer
/// Miden Testnet <-> EVM Sepolia

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("🚀 SAKASENA Bridge Relayer starting...");

    let miden_rpc = std::env::var("MIDEN_RPC")
        .unwrap_or_else(|_| "https://rpc.testnet.miden.io".to_string());
    let evm_rpc = std::env::var("SEPOLIA_RPC_URL")
        .unwrap_or_else(|_| "https://rpc.sepolia.org".to_string());
    let evm_bridge = std::env::var("EVM_BRIDGE_ADDRESS")
        .unwrap_or_else(|_| "0x90cbAe500C2c008B58656f474d4e35F5B7A7996a".to_string());
    let bridge_account_id = std::env::var("BRIDGE_ACCOUNT_ID")
        .unwrap_or_else(|_| "0x5eb65e512ab979911ec04e6798ead0".to_string());

    info!("Config:");
    info!("  Miden RPC: {}", miden_rpc);
    info!("  EVM RPC: {}", evm_rpc);
    info!("  EVM Bridge: {}", evm_bridge);
    info!("  Miden Bridge: {}", bridge_account_id);
    info!("  Status: LISTENING MODE");

    // TODO: Implement Miden note polling
    // TODO: Implement EVM event filtering
    // TODO: Implement cross-chain message relay

    info!("Relayer is running. Press Ctrl+C to stop.");
    tokio::signal::ctrl_c().await?;
    info!("👋 Relayer shutting down.");
    Ok(())
}
