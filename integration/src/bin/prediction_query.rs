use anyhow::Context;
use integration::helpers::setup_client;
use miden_client::{
    account::{AccountId, StorageSlotName},
    Felt, Word,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🌐 [1/2] Connecting to Miden Client & Store...");
    let setup = setup_client().await.context("Failed to initialize client")?;

    // Terminal parametrindən və ya ən son yenilənmiş ID-dən oxuyuruq
    let args: Vec<String> = std::env::args().collect();
    let target_id_str = if args.len() > 1 {
        args[1].clone()
    } else {
        "0x4fd1531ea602bd513c5b87df3d8332".to_string()
    };

    let account_id = AccountId::from_hex(&target_id_str)
        .context("Invalid Market Account ID format")?;

    println!("🔍 [2/2] Querying Live Prediction Market Storage ({account_id})...");
    
    let account = setup
        .client
        .get_account(account_id)
        .await
        .context("Failed to query market account")?
        .context("Account not found in store")?;

    let market_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::market_totals")
            .context("invalid market_totals storage slot name")?;
    let yes_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::yes_totals")
            .context("invalid yes_totals storage slot name")?;
    let no_totals_slot =
        StorageSlotName::new("prediction_market::prediction_market_contract::no_totals")
            .context("invalid no_totals storage slot name")?;

    let market_id = Word::new([
        Felt::ZERO,
        Felt::ZERO,
        Felt::ZERO,
        Felt::ONE,
    ]);

    let market_total = account
        .storage()
        .get_map_item(&market_totals_slot, market_id)
        .unwrap_or(Word::default());

    let yes_total = account
        .storage()
        .get_map_item(&yes_totals_slot, market_id)
        .unwrap_or(Word::default());

    let no_total = account
        .storage()
        .get_map_item(&no_totals_slot, market_id)
        .unwrap_or(Word::default());

    println!("\n📊 LIVE PREDICTION MARKET ON-CHAIN STATE");
    println!("============================================================");
    println!("📍 Market Contract ID : {}", account.id());
    println!("🎯 Market ID          : [0, 0, 0, 1]");
    println!("📈 Total Pool Points  : {} SKS", market_total[0].as_canonical_u64());
    println!("🟢 YES Pool Points    : {} SKS", yes_total[0].as_canonical_u64());
    println!("🔴 NO Pool Points     : {} SKS", no_total[0].as_canonical_u64());
    println!("============================================================\n");

    Ok(())
}
