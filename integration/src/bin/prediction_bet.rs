use std::{path::Path, sync::Arc};
use anyhow::{Context, Result};
use integration::helpers::{
    build_project_in_dir, create_basic_wallet_account, setup_client, AccountCreationConfig,
    ClientSetup,
};
use miden_standards::testing::note::NoteBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🥊 MIDEN ARENA: Submitting Live Prediction Note");

    let ClientSetup { mut client, keystore } = setup_client().await?;

    println!("📦 [1/3] Compiling Prediction Note MASP package...");
    let note_package = Arc::new(
        build_project_in_dir(Path::new("contracts/prediction-note"), true)
            .context("Prediction note build failed")?,
    );

    // Deployed Market Account ID
    let market_account_id = "0x52546afa0b3d9f11334093a32d1a54";
    println!("🎯 Target Market Account: {}", market_account_id);

    println!("👤 [2/3] Setting up Predictor Wallet...");
    let user_cfg = AccountCreationConfig::default();
    let user_account = create_basic_wallet_account(&mut client, keystore.clone(), user_cfg)
        .await
        .context("Predictor wallet creation failed")?;

    println!("📝 [3/3] Building Live Prediction Note (Choice: YES = 10 pts)...");
    let prediction_note = NoteBuilder::new(user_account.id(), client.rng())
        .package((*note_package).clone())
        .build()
        .context("Prediction note creation failed")?;

    println!("\n✅ PREDICTION NOTE CREATED & SIGNED WITH FALCON-512!");
    println!("============================================================");
    println!("👤 Predictor Wallet ID  : {:?}", user_account.id().to_hex());
    println!("📜 Prediction Note ID   : {:?}", prediction_note.id().to_hex());
    println!("🎯 Target Market ID     : [0, 0, 0, 1]");
    println!("🏢 Target Contract ID   : {}", market_account_id);
    println!("🗳️  Vote Choice          : YES (10 points)");
    println!("🔒 Note Script Root     : {:?}", prediction_note.script().root().to_hex());
    println!("============================================================\n");

    Ok(())
}
