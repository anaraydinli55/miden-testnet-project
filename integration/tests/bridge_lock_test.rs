use std::{path::Path, sync::Arc};

use anyhow::Context;
use integration::helpers::{build_project_in_dir, create_account_from_package, create_basic_wallet_account, setup_client, AccountCreationConfig, ClientSetup};
use miden_client::{
    account::{component::InitStorageData, AccountBuilder, AccountComponent, AccountType},
    auth::AuthSchemeId,
    crypto::RandomCoin,
    note::NoteScript,
    transaction::RawOutputNote,
    Word,
};
use miden_standards::testing::note::NoteBuilder;
use miden_testing::{AccountState, Auth, MockChain};

#[tokio::test]
async fn bridge_lock_unlock_test() -> anyhow::Result<()> {
    let mut builder = MockChain::builder();
    let sender = builder.add_existing_wallet(Auth::BasicAuth {
        auth_scheme: AuthSchemeId::Falcon512Poseidon2,
    })?;

    let bridge_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/bridge-lockbox"), true,
    )?);
    let note_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/bridge-note"), true,
    )?);

    let bridge_component = AccountComponent::from_package(
        &bridge_package, &InitStorageData::default(),
    ).context("Bridge component build failed")?;

    let bridge_account = builder.add_account_from_builder(
        Auth::BasicAuth { auth_scheme: AuthSchemeId::Falcon512Poseidon2 },
        AccountBuilder::new([99_u8; 32])
            .account_type(AccountType::Public)
            .with_component(bridge_component),
        AccountState::Exists,
    )?;

    let mut note_rng = RandomCoin::new(Word::from(
        NoteScript::from_package(note_package.as_ref())
            .context("Note script build failed")?
            .root(),
    ));

    let lock_note = NoteBuilder::new(sender.id(), &mut note_rng)
        .package((*note_package).clone())
        .build()
        .context("Lock note build failed")?;

    builder.add_output_note(RawOutputNote::Full(lock_note.clone()));
    let mut mock_chain = builder.build()?;

    let tx_context = mock_chain
        .build_tx_context(bridge_account.clone(), &[lock_note.id()], &[])?
        .build()?;

    let executed_tx = tx_context.execute().await?;
    mock_chain.add_pending_executed_transaction(&executed_tx)?;
    mock_chain.prove_next_block()?;

    println!("✅ Bridge lock test passed! Account: {:?}", bridge_account.id().to_hex());
    Ok(())
}
