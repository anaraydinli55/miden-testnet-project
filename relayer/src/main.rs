use anyhow::Result;
use log::{info, error, debug};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
struct Config {
    miden_rpc: String,
    evm_rpc: String,
    evm_bridge: String,
    bridge_account_id: String,
    relayer_address: String,
    poll_interval_secs: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Vec<serde_json::Value>,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(default)]
    result: Option<serde_json::Value>,
    #[serde(default)]
    error: Option<serde_json::Value>,
    id: u64,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct EvmLog {
    address: String,
    topics: Vec<String>,
    data: String,
    blockNumber: String,
    transactionHash: String,
}

/// EVM Burn eventlerini dinle (5 block chunk'lar halinde)
async fn poll_evm_events(config: &Config, last_block: Arc<Mutex<u64>>) -> Result<()> {
    let client = reqwest::Client::new();
    
    // Son blocku al
    let block_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_blockNumber".to_string(),
        params: vec![],
        id: 1,
    };
    
    let resp = client.post(&config.evm_rpc)
        .json(&block_req)
        .send()
        .await?;
    
    let block_resp: JsonRpcResponse = resp.json().await?;
    let current_block = block_resp.result
        .and_then(|r| r.as_str().map(|s| u64::from_str_radix(&s[2..], 16).ok()).flatten())
        .unwrap_or(0);
    
    let mut last = last_block.lock().await;
    
    // İlk çalıştırma: burn event'lerin olduğu block'tan başla
    if *last == 0 {
        *last = 11586100; // Burn events at 11586103
    }
    
    // Eğer last_block current_block'tan büyükse (edge case)
    if *last >= current_block {
        debug!("Up to date (last={}, current={})", *last, current_block);
        return Ok(());
    }
    
    // 5 block'luk chunk'lar halinde tara (Alchemy Free Tier: max 10 block)
    let chunk_size = 5;
    let mut from = *last;
    
    while from < current_block {
        let to = std::cmp::min(from + chunk_size, current_block);
        
        debug!("Scanning blocks {}-{}", from, to);
        
        let burn_topic = "0x8e9f676bc0e67c2eff7217a1ba8a325009f2d8adbf22e10f841c381796ae2b01";
        
        let logs_req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_getLogs".to_string(),
            params: vec![json!({
                "fromBlock": format!("0x{:x}", from),
                "toBlock": format!("0x{:x}", to),
                "address": config.evm_bridge,
                "topics": [burn_topic]
            })],
            id: 2,
        };
        
        let resp = client.post(&config.evm_rpc)
            .json(&logs_req)
            .send()
            .await?;
        
        let raw_text = resp.text().await?;
        
        let logs_resp: JsonRpcResponse = serde_json::from_str(&raw_text)?;
        
        if let Some(error) = logs_resp.error {
            error!("RPC error: {:?} (block {}-{})", error, from, to);
            break;
        }
        
        if let Some(logs) = logs_resp.result {
            if let Some(log_array) = logs.as_array() {
                if !log_array.is_empty() {
                    info!("🔥 Found {} burn event(s) in blocks {}-{}!", log_array.len(), from, to);
                    for log in log_array {
                        if let Ok(ev) = serde_json::from_value::<EvmLog>(log.clone()) {
                            let data = ev.data.trim_start_matches("0x");
                            let amount = u64::from_str_radix(&data[0..64], 16).unwrap_or(0);
                            let nonce = u64::from_str_radix(&data[64..128], 16).unwrap_or(0);
                            let dest_miden = format!("0x{}", &data[128..192]);
                            
                            info!("🔥 BURN EVENT:");
                            info!("   TX: {}", ev.transactionHash);
                            info!("   Block: {}", ev.blockNumber);
                            info!("   Amount: {} wSKS", amount);
                            info!("   Nonce: {}", nonce);
                            info!("   Dest Miden: {}", dest_miden);
                            info!("   → Action: Create unlock note on Miden");
                        }
                    }
                }
            }
        }
        
        from = to;
    }
    
    *last = current_block;
    debug!("Updated last_block to {}", current_block);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("🚀 SAKASENA Bridge Relayer starting...");
    
    let config = Config {
        miden_rpc: std::env::var("MIDEN_RPC")
            .unwrap_or_else(|_| "https://rpc.testnet.miden.io".to_string()),
        evm_rpc: std::env::var("SEPOLIA_RPC_URL")
            .unwrap_or_else(|_| "https://rpc.sepolia.org".to_string()),
        evm_bridge: std::env::var("EVM_BRIDGE_ADDRESS")
            .unwrap_or_else(|_| "0x90cbAe500C2c008B58656f474d4e35F5B7A7996a".to_string()),
        bridge_account_id: std::env::var("BRIDGE_ACCOUNT_ID")
            .unwrap_or_else(|_| "0x5eb65e512ab979911ec04e6798ead0".to_string()),
        relayer_address: std::env::var("RELAYER_ADDRESS")
            .unwrap_or_else(|_| "0xf8d59231bD1c74b8878cCF244C4dFFf412C872F5".to_string()),
        poll_interval_secs: 5,
    };
    
    let last_block = Arc::new(Mutex::new(0u64));
    
    info!("Config:");
    info!("  EVM Bridge: {}", config.evm_bridge);
    info!("  Miden Bridge: {}", config.bridge_account_id);
    
    let mut interval = tokio::time::interval(Duration::from_secs(config.poll_interval_secs));
    
    info!("🔄 Starting event polling loop...");
    
    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = poll_evm_events(&config, last_block.clone()).await {
                    error!("EVM poll error: {}", e);
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("👋 Relayer shutting down.");
                break;
            }
        }
    }
    
    Ok(())
}
