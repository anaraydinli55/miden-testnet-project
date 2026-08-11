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
async fn test_time_lock_vault() -> anyhow::Result<()> {
    // 1. Lokal MockChain test zənciri qurucusunu başladırıq
    let mut builder = MockChain::builder();

    // 2. İşlemi gönderecek olan kullanıcı cüzdanını (Wallet) ekliyoruz
    let sender = builder.add_existing_wallet(Auth::BasicAuth {
        auth_scheme: AuthSchemeId::Falcon512Poseidon2,
    })?;

    // 3. time-lock-vault, vault-deposit-note ve vault-withdraw-note sözleşmelerini RELEASE modunda derliyoruz
    // (Böylece FPI linker uyuşmazlıkları otomatik olarak çözülür!)
    let contract_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/time-lock-vault"),
        true,
    )?);
    let deposit_note_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/vault-deposit-note"),
        true,
    )?);
    let withdraw_note_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/vault-withdraw-note"),
        true,
    )?);

    // 4. Kasa depolama alanlarının (balances ve unlock_blocks) slot isimlerini tanımlıyoruz
    let balances_slot = StorageSlotName::new("time_lock_vault::vault_contract::balances")
        .context("invalid balances storage slot name")?;
    let unlock_blocks_slot = StorageSlotName::new("time_lock_vault::vault_contract::unlock_blocks")
        .context("invalid unlock_blocks storage slot name")?;

    // Test için sembolik bir kullanıcı anahtarı (User Key Word) oluşturuyoruz
    let user_key = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);

    // 5. Kasa depolarını başlangıçta 0 bakiye ve 0 kilitleme süresiyle ilklendiriyoruz
    let mut init_storage_data = InitStorageData::default();
    init_storage_data.insert_map_entry(balances_slot.clone(), user_key, 0_u64)?;
    init_storage_data.insert_map_entry(unlock_blocks_slot.clone(), user_key, 0_u64)?;

    // 6. Derlediğimiz paketten (MASP) kasa bileşenini ayağa kaldırıyoruz
    let vault_component = AccountComponent::from_package(&contract_package, &init_storage_data)
        .context("failed to build account component from vault package")?;

    // 7. Bu bileşeni barındıran halka açık (public) yeni bir Zaman Kilidli Kasa Hesabı oluşturuyoruz
    let vault_account = builder.add_account_from_builder(
        Auth::BasicAuth {
            auth_scheme: AuthSchemeId::Falcon512Poseidon2,
        },
        AccountBuilder::new([8_u8; 32])
            .account_type(AccountType::Public)
            .with_component(vault_component),
        AccountState::Exists,
    )?;

    // 8. Tüketilecek Depozit Notunu (Deposit Note) oluşturuyoruz (Block 1000'de 100 yatıracak ve 50 blok kilitleyecek)
    let mut deposit_note_rng = RandomCoin::new(Word::from(
        NoteScript::from_package(deposit_note_package.as_ref())
            .context("failed to build note script from package")?
            .root(),
    ));
    let deposit_note = NoteBuilder::new(sender.id(), &mut deposit_note_rng)
        .package((*deposit_note_package).clone())
        .build()
        .context("failed to build deposit note from package")?;

    // 9. Tüketilecek Çekme Notunu (Withdraw Note) oluşturuyoruz (40 birim çekme talebi)
    let mut withdraw_note_rng = RandomCoin::new(Word::from(
        NoteScript::from_package(withdraw_note_package.as_ref())
            .context("failed to build note script from package")?
            .root(),
    ));
    let withdraw_note = NoteBuilder::new(sender.id(), &mut withdraw_note_rng)
        .package((*withdraw_note_package).clone())
        .build()
        .context("failed to build withdraw note from package")?;

    // 10. Oluşturulan her iki notu MockChain'e çıktı notu olarak ekliyoruz
    builder.add_output_note(RawOutputNote::Full(deposit_note.clone()));
    builder.add_output_note(RawOutputNote::Full(withdraw_note.clone()));

    // 11. Sanal zinciri inşa ediyoruz
    let mut mock_chain = builder.build()?;

    // -------------------------------------------------------------
    // ADIM A: Para Yatırma İşlemi (Deposit)
    // -------------------------------------------------------------
    println!("Step A: Executing deposit transaction...");
    let tx_context_a = mock_chain
        .build_tx_context(vault_account.clone(), &[deposit_note.id()], &[])?
        .build()?;

    let executed_transaction_a = tx_context_a.execute().await?;
    mock_chain.add_pending_executed_transaction(&executed_transaction_a)?;
    mock_chain.prove_next_block()?;

    // Kilid açılış blok nömrəsinin 1050 olduğunu doğrulayırıq
    let unlock_block = mock_chain
        .committed_account(vault_account.id())?
        .storage()
        .get_map_item(&unlock_blocks_slot, user_key)
        .expect("Failed to get unlock block from storage slot");
    assert_eq!(unlock_block[0].as_canonical_u64(), 1050, "Unlock block is not equal to 1050!");
    println!("SUCCESS: Deposit executed. 100 SKS locked until block 1050!");

    // -------------------------------------------------------------
// ADIM B: Para Çekme İşlemi (Withdraw) - Başarı Durumu (Block 1060)
// -------------------------------------------------------------
println!("Step B: Executing withdraw transaction after lock expiration...");

// ZƏNCİRDƏN GÜNCƏL VƏ YENİLƏNMİŞ HESAB NÜSXƏSİNİ ÇƏKİRİK VƏ CLONE EDİRİK!
let updated_vault_account = mock_chain.committed_account(vault_account.id())?.clone();

let tx_context_b = mock_chain
    .build_tx_context(updated_vault_account, &[withdraw_note.id()], &[])?
    .build()?;

    let executed_transaction_b = tx_context_b.execute().await?;
    mock_chain.add_pending_executed_transaction(&executed_transaction_b)?;
    mock_chain.prove_next_block()?;

    // Son bakiyenin tam olarak 60 kaldığını doğruluyoruz (100 - 40 = 60)
    let final_balance = mock_chain
        .committed_account(vault_account.id())?
        .storage()
        .get_map_item(&balances_slot, user_key)
        .expect("Failed to get balance from storage slot");
    assert_eq!(final_balance[0].as_canonical_u64(), 60, "Final balance is not equal to 60!");
    
    println!("SUCCESS! Time-Locked Vault deposit and unlock withdraw simulation completed perfectly!");
    Ok(())
}