use integration::helpers::{
    build_project_in_dir, create_account_from_package, create_basic_wallet_account,
    setup_client, AccountCreationConfig, ClientSetup,
};
use anyhow::{Context, Result};
use miden_client::{account::component::InitStorageData, transaction::TransactionRequestBuilder};
use miden_standards::testing::note::NoteBuilder;
use std::{path::Path, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌉 SAKASENA Bridge: Miden -> EVM Deposit");
    
    let ClientSetup { mut client, keystore } = setup_client().await?;
    let sync = client.sync_state().await?;
    println!("✅ Synced at block: {}", sync.block_num);

    let bridge_package = Arc::new(
        build_project_in_dir(Path::new("../contracts/bridge-lockbox"), true)
            .context("Bridge lockbox build failed")?,
    );
    let note_package = Arc::new(
        build_project_in_dir(Path::new("../contracts/bridge-note"), true)
            .context("Bridge note build failed")?,
    );

    // Bridge account ID (testnet deploy sonucu)
    let bridge_account_id = "0x5eb65e512ab979911ec04e6798ead0";
    println!("🔐 Bridge account: {}", bridge_account_id);

    let user_cfg = AccountCreationConfig::default();
    let user_account = create_basic_wallet_account(&mut client, keystore.clone(), user_cfg)
        .await.context("User wallet creation failed")?;
    println!("👤 User account: {:?}", user_account.id().to_hex());

    let amount = 100u64;
    let evm_dest = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb";
    println!("🔒 Locking {} SKS to EVM: {}", amount, evm_dest);

    let lock_note = NoteBuilder::new(user_account.id(), client.rng())
        .package((*note_package).clone())
        .tag(0)
        .build()
        .context("Lock note creation failed")?;

    let publish_req = TransactionRequestBuilder::new()
        .own_output_notes(vec![lock_note.clone()])
        .build()?;

    let publish_tx = client.submit_new_transaction(user_account.id(), publish_req).await?;
    client.sync_state().await?;
    println!("📤 Note published: {:?}", publish_tx.to_hex());

    // Bridge account'tan note'u consume et
    // Not: Gerçekte bridge account'ın auth'u gerekli, bu test script'i
    println!("⚠️  Note consume requires bridge account auth — manual step for now");
    println!("📋 Relayer should call SKSBridge.mint() on EVM with:");
    println!("   - amount: {}", amount);
    println!("   - evm_dest: {}", evm_dest);
    println!("   - miden_account: {:?}", user_account.id().to_hex());

    Ok(())
}
