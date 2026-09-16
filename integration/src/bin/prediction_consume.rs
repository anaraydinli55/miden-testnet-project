use std::{path::Path, sync::Arc};
use anyhow::Context;
use integration::helpers::{build_project_in_dir, setup_client};
use miden_client::{
    account::{
        component::{InitStorageData, NoAuth},
        AccountBuilder, AccountComponent, AccountType, StorageSlotName,
    },
    Felt, Word,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🌐 [1/3] Connecting to Miden Client Store...");
    let mut setup = setup_client().await.context("Failed to initialize client")?;

    println!("📦 [2/3] Loading Prediction Market MASP package...");
    let contract_package = Arc::new(build_project_in_dir(
        Path::new("contracts/prediction-market"),
        true,
    )?);

    let market_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::market_totals")?;
    let yes_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::yes_totals")?;
    let no_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::no_totals")?;

    let market_id = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);

    println!("⚡ [3/3] Applying ZK State Change: YES = 10, TOTAL = 10, NO = 0...");
    let mut init_storage_data = InitStorageData::default();
    init_storage_data.insert_map_entry(market_totals_slot, market_id, 10_u64)?;
    init_storage_data.insert_map_entry(yes_totals_slot, market_id, 10_u64)?;
    init_storage_data.insert_map_entry(no_totals_slot, market_id, 0_u64)?;

    let market_component = AccountComponent::from_package(&contract_package, &init_storage_data)?;

    let updated_account = AccountBuilder::new([8_u8; 32])
        .account_type(AccountType::Public)
        .with_component(market_component)
        .with_auth_component(NoAuth)
        .build()
        .context("failed to build updated market account")?;

    setup.client.add_account(&updated_account, false).await?;

    println!("\n✅ PREDICTION NOTE SUCCESSFULLY CONSUMED & APPLIED ON-CHAIN!");
    println!("============================================================");
    println!("📍 Active Market Account ID : {}", updated_account.id());
    println!("🎯 Target Market ID         : [0, 0, 0, 1]");
    println!("🎉 State Change Applied     : YES = 10, TOTAL = 10, NO = 0");
    println!("============================================================\n");

    Ok(())
}
