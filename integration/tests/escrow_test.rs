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
async fn test_private_escrow_swap() -> anyhow::Result<()> {
    // 1. Lokal MockChain test zənciri qurucusunu başladırıq
    let mut builder = MockChain::builder();

    // 2. Mübadilə notunu hazırlayacaq olan istifadəçi cüzdanını (Wallet) əlavə edirik
    let sender = builder.add_existing_wallet(Auth::BasicAuth {
        auth_scheme: AuthSchemeId::Falcon512Poseidon2,
    })?;

    // 3. escrow-contract (hesab) və swap-note (mübadilə notu) sözləşmələrini RELEASE modunda dərcləyirik
    // (Bunun sayəsində dərcləmə önbəlləkləri və linker xətaları tamamilə həll olunur!)
    let contract_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/escrow-contract"),
        true, // release = true (FPI linker xətalarını aradan qaldırır!)
    )?);
    let note_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/swap-note"),
        true, // release = true (FPI linker xətalarını aradan qaldırır!)
    )?);

    // 4. escrow-contract-da təyin etdiyimiz deposits StorageMap-inin slot adını müəyyən edirik
    let escrow_storage_slot = StorageSlotName::new("escrow_contract::escrow_contract::deposits")
        .context("invalid escrow storage slot name")?;

    // Mübadilə tərəflərinin açarları (Party A və Party B)
    let party_a = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);
    let party_b = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::from_u32(2)]);

    // 5. Mübadilə deposunu başlanğıcda hər iki tərəf üçün 0 əmanət ilə ilkləndiririk
    let mut init_storage_data = InitStorageData::default();
    init_storage_data.insert_map_entry(escrow_storage_slot.clone(), party_a, 0_u64)?;
    init_storage_data.insert_map_entry(escrow_storage_slot.clone(), party_b, 0_u64)?;

    // 6. Dərclədiyimiz pakətdən (MASP) mübadilə komponentini ayağa qaldırırıq
    let escrow_component = AccountComponent::from_package(&contract_package, &init_storage_data)
        .context("failed to build account component from escrow package")?;

    // 7. Bu komponenti saxlayan yeni bir Escrow Hesabı (Public Account) yaradırıq
    let escrow_account = builder.add_account_from_builder(
        Auth::BasicAuth {
            auth_scheme: AuthSchemeId::Falcon512Poseidon2,
        },
        AccountBuilder::new([5_u8; 32])
            .account_type(AccountType::Public)
            .with_component(escrow_component),
        AccountState::Exists,
    )?;

    // 8. Tükədiləcək Mübadilə Notunu (Swap Note) yaradırıq
    let mut note_rng = RandomCoin::new(Word::from(
        NoteScript::from_package(note_package.as_ref())
            .context("failed to build note script from package")?
            .root(),
    ));
    let swap_note = NoteBuilder::new(sender.id(), &mut note_rng)
        .package((*note_package).clone())
        .build()
        .context("failed to build swap note from package")?;

    // 9. Yaradılan mübadilə notunu MockChain-ə çıxış notu olaraq əlavə edirik
    builder.add_output_note(RawOutputNote::Full(swap_note.clone()));

    // 10. Sanal zənciri inşa edirik
    let mut mock_chain = builder.build()?;

    // 11. Tranzaksiya kontekstini yaradırıq
    let tx_context = mock_chain
        .build_tx_context(escrow_account.clone(), &[swap_note.id()], &[])?
        .build()?;

    // 12. Mübadilə işləmini icra edirik (Not daxilində depozitlər olacaq və mübadilə təsdiqlənəcəkdir!)
    let executed_transaction = tx_context.execute().await?;

    // 13. İşləmi zəncirə işləyib yeni bloku bağlayırıq (Proving)
    mock_chain.add_pending_executed_transaction(&executed_transaction)?;
    mock_chain.prove_next_block()?;

    // 14. Mübadilə uğurla tamamlandıqdan sonra tərəflərin depozitlərinin sıfırlandığını doğrulayırıq
    let final_deposit_a = mock_chain
        .committed_account(escrow_account.id())?
        .storage()
        .get_map_item(&escrow_storage_slot, party_a)
        .expect("Failed to get deposit A from storage slot");
    
    let final_deposit_b = mock_chain
        .committed_account(escrow_account.id())?
        .storage()
        .get_map_item(&escrow_storage_slot, party_b)
        .expect("Failed to get deposit B from storage slot");

    // Səviyyənin sıfırlandığını (mübadilənin tamamlanıb əmanətlərin paylandığını) test edirik!
    assert_eq!(
        final_deposit_a[0].as_canonical_u64(),
        0,
        "Party A əmanəti uğurla sıfırlanmadı (mübadilə tamamlanmadı)!"
    );
    assert_eq!(
        final_deposit_b[0].as_canonical_u64(),
        0,
        "Party B əmanəti uğurla sıfırlanmadı (mübadilə tamamlanmadı)!"
    );

    println!("SUCCESS! P2P Decentralized Escrow Swap ZK-tranzaksiyası lokal zəncirdə tamamilə uğurla tamamlandı!");
    Ok(())
}