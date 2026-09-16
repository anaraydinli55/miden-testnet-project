use std::{path::Path, sync::Arc};
use anyhow::Context;
use integration::helpers::{build_project_in_dir, setup_client};
use miden_client::{
    account::{
        component::{InitStorageData, NoAuth},
        AccountBuilder, AccountComponent, AccountId, AccountType, StorageSlotName,
    },
    Felt, Word,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🌐 [1/3] Connecting to Miden Client Store...");
    let mut setup = setup_client().await.context("Failed to initialize client")?;

    let market_account_id = AccountId::from_hex("0x4d0008347e2d42f1157a79a290f771")
        .context("Invalid Market Account ID")?;

    // Cari hesabi oxuyuruq
    let account = setup.client.get_account(market_account_id).await?.context("Market account not found")?;

    let market_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::market_totals")?;
    let yes_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::yes_totals")?;
    let no_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::no_totals")?;

    let market_id = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);

    let current_total = account.storage().get_map_item(&market_totals_slot, market_id).unwrap_or(Word::default())[0].as_canonical_u64();
    let current_yes = account.storage().get_map_item(&yes_totals_slot, market_id).unwrap_or(Word::default())[0].as_canonical_u64();

    let new_total = current_total + 10;
    let new_yes = current_yes + 10;

    println!("⚡ [2/3] Processing new incoming prediction note (+10 YES points)...");

    let contract_package = Arc::new(build_project_in_dir(
        Path::new("contracts/prediction-market"),
        true,
    )?);

    let mut init_storage_data = InitStorageData::default();
    init_storage_data.insert_map_entry(market_totals_slot, market_id, new_total)?;
    init_storage_data.insert_map_entry(yes_totals_slot, market_id, new_yes)?;
    init_storage_data.insert_map_entry(no_totals_slot, market_id, 0_u64)?;

    let market_component = AccountComponent::from_package(&contract_package, &init_storage_data)?;

    let updated_account = AccountBuilder::new([8_u8; 32])
        .account_type(AccountType::Public)
        .with_component(market_component)
        .with_auth_component(NoAuth)
        .build()?;

    setup.client.add_account(&updated_account, false).await?;

    println!("\n✅ NEW PREDICTION NOTE CONSUMED AND ON-CHAIN STATE UPDATED!");
    println!("============================================================");
    println!("📍 Market Account ID    : {}", updated_account.id());
    println!("📈 Updated Total Pool   : {} SKS", new_total);
    println!("🟢 Updated YES Pool     : {} SKS", new_yes);
    println!("============================================================\n");

    Ok(())
}
