use std::{path::Path, sync::Arc};
use anyhow::Context;
use integration::helpers::{build_project_in_dir, create_account_from_package, setup_client, AccountCreationConfig};
use miden_client::{
    account::{component::InitStorageData, AccountType, StorageSlotName},
    Felt, Word,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🌐 [1/3] Initializing Miden Client & Keystore...");
    let mut setup = setup_client().await.context("Failed to initialize client")?;

    println!("📦 [2/3] Compiling Prediction Market MASP package...");
    let contract_package = Arc::new(build_project_in_dir(
        Path::new("contracts/prediction-market"),
        true,
    )?);

    let market_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::market_totals")
            .context("invalid market_totals storage slot name")?;
    let yes_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::yes_totals")
            .context("invalid yes_totals storage slot name")?;
    let no_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::no_totals")
            .context("invalid no_totals storage slot name")?;

    // Market ID = 1
    let market_id = Word::new([
        Felt::ZERO,
        Felt::ZERO,
        Felt::ZERO,
        Felt::ONE,
    ]);

    let mut init_storage_data = InitStorageData::default();
    init_storage_data.insert_map_entry(market_totals_slot, market_id, 0_u64)?;
    init_storage_data.insert_map_entry(yes_totals_slot, market_id, 0_u64)?;
    init_storage_data.insert_map_entry(no_totals_slot, market_id, 0_u64)?;

    println!("🚀 [3/3] Creating and registering Prediction Market Account...");
    let config = AccountCreationConfig {
        account_type: AccountType::Public,
        init_storage_data,
    };

    let market_account = create_account_from_package(&mut setup.client, contract_package, config)
        .await
        .context("Failed to create prediction market account")?;

    println!("\n✅ PREDICTION MARKET ACCOUNT CREATED!");
    println!("============================================================");
    println!("📍 Market Contract Account ID : {}", market_account.id());
    println!("🎯 Initialized Market ID      : [0, 0, 0, 1]");
    println!("📊 Initial Status             : YES=0, NO=0, TOTAL=0");
    println!("============================================================\n");

    Ok(())
}
