use integration::helpers::{
    build_project_in_dir, create_account_from_package,
    create_basic_wallet_account, setup_client, AccountCreationConfig, ClientSetup,
};

use anyhow::{Context, Result};
use miden_client::{
    account::{component::InitStorageData, StorageSlotName}, 
    transaction::TransactionRequestBuilder,
    Felt, Word,
};
use miden_standards::testing::note::NoteBuilder;
use std::{path::Path, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Miden testnet istemcisini (client) ve keystore'u başlatıyoruz
    let ClientSetup {
        mut client,
        keystore,
    } = setup_client().await?;

    let sync_summary = client.sync_state().await?;
    println!("Latest block on Miden Testnet: {}", sync_summary.block_num);

    // 2. Sözleşmelerimizi (bank-account ve deposit-note) dinamik olarak RELEASE modunda derliyoruz
    let bank_package = Arc::new(
        build_project_in_dir(Path::new("../contracts/bank-account"), true)
            .context("Failed to build bank account contract")?,
    );
    let note_package = Arc::new(
        build_project_in_dir(Path::new("../contracts/deposit-note"), true)
            .context("Failed to build deposit note contract")?,
    );

    // 3. bank-account'taki balances StorageMap'inin slot ismini tanımlıyoruz
    let bank_storage_slot = StorageSlotName::new("bank_account::bank_contract::balances")
        .context("invalid bank storage slot name")?;

    // Test için sembolik bir kullanıcı anahtarı (User Key Word) oluşturuyoruz
    let user_key = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);

    // 4. Banka depolama alanını ilk durum (0 bakiye) ile ilklendiriyoruz
    let mut init_storage_data = InitStorageData::default();
    init_storage_data
        .insert_map_entry(bank_storage_slot, user_key, 0_u64)
        .context("Failed to seed bank storage")?;
    let bank_cfg = AccountCreationConfig {
        init_storage_data,
        ..Default::default()
    };

    // 5. Canlı ağda yeni bir Banka Hesabı (Contract Account) oluşturup kaydediyoruz
    println!("Deploying bank account contract to Miden Testnet...");
    let bank_account =
        create_account_from_package(&mut client, bank_package.clone(), bank_cfg)
            .await
            .context("Failed to create bank account")?;
    println!("Bank Account ID: {:?}", bank_account.id().to_hex());

    // 6. İşlemi gönderecek olan kullanıcı için ayrı bir cüzdan (Wallet) hesabı oluşturuyoruz
    let sender_cfg = AccountCreationConfig::default();
    let sender_account = create_basic_wallet_account(&mut client, keystore.clone(), sender_cfg)
        .await
        .context("Failed to create sender wallet account")?;
    println!("Sender Account ID: {:?}", sender_account.id().to_hex());

    // 7. Depolama alanını tetikleyecek Depozit Notumuzu (Deposit Note) hazırlıyoruz
    let deposit_note = NoteBuilder::new(sender_account.id(), client.rng())
        .package((*note_package).clone())
        .tag(0)
        .build()
        .context("Failed to create deposit note from package")?;
    println!("Deposit note hash: {:?}", deposit_note.id().to_hex());

    // 8. Notu yayınlamak (publish) için işlemi hazırlayıp gönderici hesaptan ağa gönderiyoruz
    println!("Publishing deposit note to the network...");
    let note_publish_request = TransactionRequestBuilder::new()
        .own_output_notes(vec![deposit_note.clone()])
        .build()
        .context("Failed to build note publish transaction request")?;

    let note_publish_tx_id = client
        .submit_new_transaction(sender_account.id(), note_publish_request)
        .await
        .context("Failed to create note publish transaction")?;

    // İşlem kesinleştikten sonra ağ durumunu senkronize ediyoruz
    client
        .sync_state()
        .await
        .context("Failed to sync state after publishing note")?;

    println!(
        "Note publish transaction ID: {:?}",
        note_publish_tx_id.to_hex()
    );

    // 9. Yayınlanan bu notu banka hesabı (bank_account) üzerinden tüketiyoruz (consume)
    println!("Consuming note on the bank account to complete deposit...");
    let consume_note_request = TransactionRequestBuilder::new()
        .input_notes([(deposit_note.clone(), None)])
        .build()
        .context("Failed to build consume note transaction request")?;

    let consume_tx_id = client
        .submit_new_transaction(bank_account.id(), consume_note_request)
        .await
        .context("Failed to create consume note transaction")?;

    println!("Consume transaction ID: {:?}", consume_tx_id.to_hex());
    println!("SUCCESS! Miden Bank deposit flow completed on live Miden Testnet!");

    Ok(())
}