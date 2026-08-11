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
async fn test_bank_deposit_and_withdraw() -> anyhow::Result<()> {
    // 1. Lokal MockChain test zənciri kurucusunu başlatıyoruz
    let mut builder = MockChain::builder();

    // 2. İşlemi gönderecek olan kullanıcı cüzdanını (Wallet) ekliyoruz
    let sender = builder.add_existing_wallet(Auth::BasicAuth {
        auth_scheme: AuthSchemeId::Falcon512Poseidon2,
    })?;

    // 3. bank-account, deposit-note ve withdraw-note sözleşmelerini RELEASE modunda derliyoruz
    // (Böylece derleme önbellekleri hatasız ve pürüzsüz çözümlenir!)
    let contract_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/bank-account"),
        true,
    )?);
    let deposit_note_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/deposit-note"),
        true,
    )?);
    let withdraw_note_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/withdraw-note"),
        true,
    )?);

    // 4. bank-account'ta tanımladığımız balances StorageMap'inin slot adını tanımlıyoruz
    let bank_storage_slot = StorageSlotName::new("bank_account::bank_contract::balances")
        .context("invalid bank storage slot name")?;

    // Test için sembolik bir kullanıcı anahtarı (User Key Word) oluşturuyoruz
    let user_key = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);

    // 5. Banka depolama alanını ilk durum (0 bakiye) ile ilklendiriyoruz
    let mut init_storage_data = InitStorageData::default();
    init_storage_data.insert_map_entry(bank_storage_slot.clone(), user_key, 0_u64)?;

    // 6. Derlediğimiz paketten (MASP) hesap bileşenini ayağa kaldırıyoruz
    let bank_component = AccountComponent::from_package(&contract_package, &init_storage_data)
        .context("failed to build account component from bank package")?;

    // 7. Bu bileşeni barındıran halka açık (public) yeni bir Miden Banka Hesabı oluşturuyoruz
    let bank_account = builder.add_account_from_builder(
        Auth::BasicAuth {
            auth_scheme: AuthSchemeId::Falcon512Poseidon2,
        },
        AccountBuilder::new([9_u8; 32])
            .account_type(AccountType::Public)
            .with_component(bank_component),
        AccountState::Exists,
    )?;

    // 8. Tüketilecek Depozit Notunu oluşturuyoruz (50 birim para yatırma)
    let mut deposit_note_rng = RandomCoin::new(Word::from(
        NoteScript::from_package(deposit_note_package.as_ref())
            .context("failed to build note script from package")?
            .root(),
    ));
    let deposit_note = NoteBuilder::new(sender.id(), &mut deposit_note_rng)
        .package((*deposit_note_package).clone())
        .build()
        .context("failed to build deposit note from package")?;

    // 9. Tüketilecek Çekme Notunu oluşturuyoruz (20 birim para çekme)
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

    // 12. Tranzaksiya bağlamını oluşturuyoruz (Aynı anda iki notu birden tüketiyoruz!)
    let tx_context = mock_chain
        .build_tx_context(bank_account.clone(), &[deposit_note.id(), withdraw_note.id()], &[])?
        .build()?;

    // 13. İşlemi tek seferde yürütüyoruz (Yatırılan 50 - Çekilen 20 = Kalan 30 bakiye)
    let executed_transaction = tx_context.execute().await?;

    // 14. İşlemi zincire işleyip bloğu kapatıyoruz (Proving)
    mock_chain.add_pending_executed_transaction(&executed_transaction)?;
    mock_chain.prove_next_block()?;

    // 15. Güncellenmiş banka hesabından kullanıcının bakiyesini çekiyoruz
    let final_balance = mock_chain
        .committed_account(bank_account.id())?
        .storage()
        .get_map_item(&bank_storage_slot, user_key)
        .expect("Failed to get balance from storage slot");

    // 16. Bakiyenin başarıyla tam olarak 30 birim kaldığını doğruluyoruz!
    assert_eq!(
        final_balance[0].as_canonical_u64(),
        30,
        "Banka bakiyesi 30 birim olarak güncellenmedi!"
    );

    println!("SUCCESS! Miden Bank Deposit and Withdraw simulation completed perfectly! Net balance is 30.");
    Ok(())
}