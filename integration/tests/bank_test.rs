use std::{path::Path, sync::Arc};

use anyhow::Context;
use integration::helpers::build_project_in_dir;
use miden_client::{
    account::{component::InitStorageData, AccountBuilder, AccountComponent, AccountType, StorageSlotName},
    auth::AuthSchemeId,
    Word, Felt,
};
use miden_testing::{AccountState, Auth, MockChain};

#[tokio::test]
async fn test_bank_deposit() -> anyhow::Result<()> {
    // 1. Lokal MockChain test zənciri kurucusunu başlatıyoruz
    let mut builder = MockChain::builder();

    // 2. bank-account sözleşmemizi (Rust) test esnasında dinamik olarak derliyoruz
    let contract_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/bank-account"),
        true,
    )?);

    // 3. bank-account'ta tanımladığımız balances StorageMap'inin slot adını tanımlıyoruz
    let bank_storage_slot = StorageSlotName::new("bank_account::BankAccountStorage::balances")
        .context("invalid bank storage slot name")?;

    // Test için sembolik bir kullanıcı anahtarı (User Key Word) oluşturuyoruz
    let user_key = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);

    // 4. Banka depolama alanını ilk durum (0 bakiye) ile ilklendiriyoruz
    let mut init_storage_data = InitStorageData::default();
    init_storage_data.insert_map_entry(bank_storage_slot.clone(), user_key, 0_u64)?;

    // 5. Derlediğimiz paketten (MASP) hesap bileşenini ayağa kaldırıyoruz
    let bank_component = AccountComponent::from_package(&contract_package, &init_storage_data)
        .context("failed to build account component from bank package")?;

    // 6. Bu bileşeni barındıran halka açık (public) yeni bir Miden Banka Hesabı oluşturuyoruz
    let bank_account = builder.add_account_from_builder(
        Auth::BasicAuth {
            auth_scheme: AuthSchemeId::Falcon512Poseidon2,
        },
        AccountBuilder::new([9_u8; 32])
            .account_type(AccountType::Public)
            .with_component(bank_component),
        AccountState::Exists,
    )?;

    println!("Success: Bank account compiled and built with ID: {:?}", bank_account.id());

    // Hesabın başarıyla oluştuğunu ve Miden standartlarına uyduğunu doğruluyoruz
    assert_eq!(bank_account.id().is_public(), true);
    
    Ok(())
}