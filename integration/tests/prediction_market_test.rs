use std::{path::Path, sync::Arc};

use anyhow::Context;
use integration::helpers::build_project_in_dir;
use miden_client::{
    account::{
        component::InitStorageData,
        AccountBuilder,
        AccountComponent,
        AccountType,
        StorageSlotName,
    },
    auth::AuthSchemeId,
    crypto::RandomCoin,
    note::NoteScript,
    transaction::RawOutputNote,
    Felt,
    Word,
};
use miden_standards::testing::note::NoteBuilder;
use miden_testing::{AccountState, Auth, MockChain};

#[tokio::test]
async fn test_prediction_market() -> anyhow::Result<()> {
    let mut builder = MockChain::builder();

    let predictor = builder.add_existing_wallet(Auth::BasicAuth {
        auth_scheme: AuthSchemeId::Falcon512Poseidon2,
    })?;

    let contract_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/prediction-market"),
        true,
    )?);

    let note_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/prediction-note"),
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

    let market_id = Word::new([
        Felt::ZERO,
        Felt::ZERO,
        Felt::ZERO,
        Felt::ONE,
    ]);

    let mut init_storage_data = InitStorageData::default();

    init_storage_data.insert_map_entry(
        market_totals_slot.clone(),
        market_id,
        0_u64,
    )?;

    init_storage_data.insert_map_entry(
        yes_totals_slot.clone(),
        market_id,
        0_u64,
    )?;

    init_storage_data.insert_map_entry(
        no_totals_slot.clone(),
        market_id,
        0_u64,
    )?;

    let market_component =
        AccountComponent::from_package(
            &contract_package,
            &init_storage_data,
        )
        .context("failed to build prediction market component")?;

    let market_account = builder.add_account_from_builder(
        Auth::BasicAuth {
            auth_scheme: AuthSchemeId::Falcon512Poseidon2,
        },
        AccountBuilder::new([8_u8; 32])
            .account_type(AccountType::Public)
            .with_component(market_component),
        AccountState::Exists,
    )?;

    let mut note_rng = RandomCoin::new(Word::from(
        NoteScript::from_package(note_package.as_ref())
            .context("failed to build prediction note script")?
            .root(),
    ));

    let prediction_note = NoteBuilder::new(
        predictor.id(),
        &mut note_rng,
    )
    .package((*note_package).clone())
    .build()
    .context("failed to build prediction note")?;

    builder.add_output_note(
        RawOutputNote::Full(prediction_note.clone()),
    );

    let mut mock_chain = builder.build()?;

    let tx_context = mock_chain
        .build_tx_context(
            market_account.clone(),
            &[prediction_note.id()],
            &[],
        )?
        .build()?;

    let executed_transaction =
        tx_context.execute().await?;

    mock_chain.add_pending_executed_transaction(
        &executed_transaction,
    )?;

    mock_chain.prove_next_block()?;

    let committed_market =
        mock_chain.committed_account(market_account.id())?;

    let market_total = committed_market
        .storage()
        .get_map_item(
            &market_totals_slot,
            market_id,
        )
        .expect("Failed to read market total");

    let yes_total = committed_market
        .storage()
        .get_map_item(
            &yes_totals_slot,
            market_id,
        )
        .expect("Failed to read YES total");

    let no_total = committed_market
        .storage()
        .get_map_item(
            &no_totals_slot,
            market_id,
        )
        .expect("Failed to read NO total");

    assert_eq!(
        market_total[0].as_canonical_u64(),
        10,
        "Market total should be 10"
    );

    assert_eq!(
        yes_total[0].as_canonical_u64(),
        10,
        "YES total should be 10"
    );

    assert_eq!(
        no_total[0].as_canonical_u64(),
        0,
        "NO total should remain 0"
    );

    println!(
        "SUCCESS! Miden Arena prediction ZK transaction completed: market=10, YES=10, NO=0"
    );

    Ok(())
}
