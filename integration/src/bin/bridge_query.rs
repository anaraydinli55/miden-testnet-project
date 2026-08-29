use integration::helpers::{setup_client, ClientSetup};
use anyhow::Result;
use miden_client::account::AccountId;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 SAKASENA Bridge: Query bridge-lockbox state");
    
    let ClientSetup { mut client, .. } = setup_client().await?;
    let sync = client.sync_state().await?;
    println!("✅ Synced at block: {}", sync.block_num);

    let bridge_id = AccountId::from_hex("0x5eb65e512ab979911ec04e6798ead0")?;
    println!("🔐 Bridge account: {:?}", bridge_id.to_hex());

    // Account storage'ı getir (unwrap ile)
    let account = client.get_account(bridge_id).await?.expect("Bridge account not found");
    println!("📦 Account found: {:?}", account.id().to_hex());
    
    // Storage slot'ları listele
    println!("🔍 Storage slots:");
    for (i, slot) in account.storage().slots().iter().enumerate() {
        println!("  Slot {}: {:?}", i, slot);
    }

    Ok(())
}
