use integration::helpers::{
    build_project_in_dir, create_account_from_package, setup_client, AccountCreationConfig, ClientSetup,
};
use anyhow::{Context, Result};
use miden_client::account::component::InitStorageData;
use std::{path::Path, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 SAKASENA Bridge: Deploy bridge-lockbox to Testnet");
    
    let ClientSetup { mut client, .. } = setup_client().await?;
    let sync = client.sync_state().await?;
    println!("✅ Synced at block: {}", sync.block_num);

    let bridge_package = Arc::new(
        build_project_in_dir(Path::new("../contracts/bridge-lockbox"), true)
            .context("Bridge lockbox build failed")?,
    );

    let bridge_cfg = AccountCreationConfig {
        init_storage_data: InitStorageData::default(),
        ..Default::default()
    };

    let bridge_account = create_account_from_package(&mut client, bridge_package.clone(), bridge_cfg)
        .await
        .context("Bridge account creation failed")?;

    println!("🔐 Bridge account deployed!");
    println!("   Account ID: {:?}", bridge_account.id().to_hex());
    println!("   Save this ID — you'll need it for the relayer config.");

    Ok(())
}
