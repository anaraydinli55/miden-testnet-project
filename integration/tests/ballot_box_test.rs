use std::{path::Path, sync::Arc};

use anyhow::Context;
use integration::helpers::build_project_in_dir;
use miden_client::{
    account::{component::InitStorageData, AccountBuilder, AccountComponent, AccountType, StorageSlotName},
    auth::AuthSchemeId,
    crypto::RandomCoin,
    note::NoteScript,
    transaction::RawOutputNote,
    Word, Felt,
};
use miden_standards::testing::note::NoteBuilder;
use miden_testing::{AccountState, Auth, MockChain};

#[tokio::test]
async fn test_private_voting() -> anyhow::Result<()> {
    // 1. Lokal MockChain test zənciri qurucusunu başladırıq
    let mut builder = MockChain::builder();

    // 2. Səs verəcək olan anonim istifadəçi cüzdanını (Wallet) əlavə edirik
    let voter = builder.add_existing_wallet(Auth::BasicAuth {
        auth_scheme: AuthSchemeId::Falcon512Poseidon2,
    })?;

    // 3. ballot-box (hesab) və vote-note (səs notu) sözləşmələrini RELEASE modunda dərcləyirik
    // (Bunun sayəsində linker və FPI çətinlikləri avtomatik həll olunur!)
    let contract_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/ballot-box"),
        true, // release = true
    )?);
    let note_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/vote-note"),
        true, // release = true
    )?);

    // 4. ballot-box-da təyin etdiyimiz votes StorageMap-inin slot adını müəyyən edirik
    let ballot_storage_slot = StorageSlotName::new("ballot_box::ballot_contract::votes")
        .context("invalid ballot storage slot name")?;

    // Səs veriləcək namizədin ID-si (Sembolik Word olaraq 1)
    let candidate_id = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);

    // 5. Səsvermə deposunu başlanğıcda 0 səs ilə ilkləndiririk
    let mut init_storage_data = InitStorageData::default();
    init_storage_data.insert_map_entry(ballot_storage_slot.clone(), candidate_id, 0_u64)?;

    // 6. Dərclədiyimiz pakətdən (MASP) səsvermə komponentini ayağa qaldırırıq
    let ballot_component = AccountComponent::from_package(&contract_package, &init_storage_data)
        .context("failed to build account component from ballot package")?;

    // 7. Bu səsvermə komponentini saxlayan yeni bir Ballot Box Hesabı yaradırıq
    let ballot_account = builder.add_account_from_builder(
        Auth::BasicAuth {
            auth_scheme: AuthSchemeId::Falcon512Poseidon2,
        },
        AccountBuilder::new([7_u8; 32])
            .account_type(AccountType::Public)
            .with_component(ballot_component),
        AccountState::Exists,
    )?;

    // 8. Tükədiləcək Səsvermə Notunu (Vote Note) yaradırıq
    let mut note_rng = RandomCoin::new(Word::from(
        NoteScript::from_package(note_package.as_ref())
            .context("failed to build note script from package")?
            .root(),
    ));
    let vote_note = NoteBuilder::new(voter.id(), &mut note_rng)
        .package((*note_package).clone())
        .build()
        .context("failed to build vote note from package")?;

    // 9. Yaradılan səs notunu MockChain-ə çıxış notu olaraq əlavə edirik
    builder.add_output_note(RawOutputNote::Full(vote_note.clone()));

    // 10. Sanal zənciri inşa edirik
    let mut mock_chain = builder.build()?;

    // 11. Tranzaksiya kontekstini yaradırıq
    let tx_context = mock_chain
        .build_tx_context(ballot_account.clone(), &[vote_note.id()], &[])?
        .build()?;

    // 12. Səsvermə işləmini icra edirik (cast_vote tetiklenecek və səs sayı 1 olacaqdır!)
    let executed_transaction = tx_context.execute().await?;

    // 13. İşləmi zəncirə işləyib yeni bloku bağlayırıq (Proving)
    mock_chain.add_pending_executed_transaction(&executed_transaction)?;
    mock_chain.prove_next_block()?;

    // 14. Yenilənmiş səsvermə hesabından namizədin ümumi səs sayını çəkirik
    let final_votes = mock_chain
        .committed_account(ballot_account.id())?
        .storage()
        .get_map_item(&ballot_storage_slot, candidate_id)
        .expect("Failed to get vote count from storage slot");

    // 15. Səsin uğurla 1-ə bərabər olduğunu test edirik!
    assert_eq!(
        final_votes[0].as_canonical_u64(),
        1,
        "Namizədin səs sayı 1 olaraq yenilənmədi!"
    );

    println!("SUCCESS! Private Voting (Ballot Box) ZK-tranzaksiyası lokal simulyasiyada uğurla tamamlandı!");
    Ok(())
}