use integration::helpers::{
    build_project_in_dir, create_basic_wallet_account,
    setup_client, AccountCreationConfig, ClientSetup,
};
use anyhow::{Context, Result};
use miden_client::{account::component::InitStorageData, transaction::TransactionRequestBuilder};
use miden_standards::testing::note::NoteBuilder;
use std::{path::Path, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌉 SAKASENA Bridge: EVM -> Miden Withdraw");
    
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

    let user_cfg = AccountCreationConfig::default();
    let user_account = create_basic_wallet_account(&mut client, keystore.clone(), user_cfg)
        .await.context("User wallet creation failed")?;
    println!("👤 User account: {:?}", user_account.id().to_hex());

    let amount = 50u64;
    println!("🔓 Unlocking {} SKS for: {:?}", amount, user_account.id().to_hex());

    // Bridge claim note oluştur (relayer tarafından yapılır)
    let claim_note = NoteBuilder::new(user_account.id(), client.rng())
        .package((*note_package).clone())
        .tag(0)
        .build()
        .context("Claim note creation failed")?;

    let publish_req = TransactionRequestBuilder::new()
        .own_output_notes(vec![claim_note.clone()])
        .build()?;

    let publish_tx = client.submit_new_transaction(user_account.id(), publish_req).await?;
    client.sync_state().await?;
    println!("📤 Claim note published: {:?}", publish_tx.to_hex());

    println!("⚠️  Note consume requires bridge account auth — manual step");
    println!("📋 In production, relayer would call unlock_sks() on bridge-lockbox");

    Ok(())
}
