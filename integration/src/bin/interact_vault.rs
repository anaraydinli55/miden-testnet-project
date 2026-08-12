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

    // 2. Sözleşmelerimizi (time-lock-vault ve vault-deposit-note) RELEASE modunda derliyoruz
    let vault_package = Arc::new(
        build_project_in_dir(Path::new("../contracts/time-lock-vault"), true)
            .context("Failed to build time-lock vault contract")?,
    );
    let note_package = Arc::new(
        build_project_in_dir(Path::new("../contracts/vault-deposit-note"), true)
            .context("Failed to build vault deposit note contract")?,
    );

    // 3. Kasa depolama alanlarının (balances ve unlock_blocks) slot isimlerini tanımlıyoruz
    let balances_slot = StorageSlotName::new("time_lock_vault::vault_contract::balances")
        .context("invalid balances storage slot name")?;
    let unlock_blocks_slot = StorageSlotName::new("time_lock_vault::vault_contract::unlock_blocks")
        .context("invalid unlock_blocks storage slot name")?;

    // Test için sembolik bir kullanıcı anahtarı (User Key Word) oluşturuyoruz
    let user_key = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);

    // 4. Kasa depolarını başlangıçta 0 bakiye ve 0 kilitleme süresiyle ilklendiriyoruz
    let mut init_storage_data = InitStorageData::default();
    init_storage_data
        .insert_map_entry(balances_slot, user_key, 0_u64)
        .context("Failed to seed balances storage")?;
    init_storage_data
        .insert_map_entry(unlock_blocks_slot, user_key, 0_u64)
        .context("Failed to seed unlock_blocks storage")?;
    let vault_cfg = AccountCreationConfig {
        init_storage_data,
        ..Default::default()
    };

    // 5. Canlı ağda yeni bir Zaman Kilidli Kasa Hesabı oluşturup kaydediyoruz
    println!("Deploying time-lock vault contract to Miden Testnet...");
    let vault_account =
        create_account_from_package(&mut client, vault_package.clone(), vault_cfg)
            .await
            .context("Failed to create time-lock vault")?;
    println!("Time-Lock Vault Account ID: {:?}", vault_account.id().to_hex());

    // 6. İşlemi gönderecek olan kullanıcı için ayrı bir cüzdan (Wallet) hesabı oluşturuyoruz
    let sender_cfg = AccountCreationConfig::default();
    let sender_account = create_basic_wallet_account(&mut client, keystore.clone(), sender_cfg)
        .await
        .context("Failed to create sender wallet account")?;
    println!("Sender Account ID: {:?}", sender_account.id().to_hex());

    // 7. Depolama alanını tetikleyecek Depozit Notumuzu (Vault Deposit Note) hazırlıyoruz
    let deposit_note = NoteBuilder::new(sender_account.id(), client.rng())
        .package((*note_package).clone())
        .tag(0)
        .build()
        .context("Failed to create vault deposit note from package")?;
    println!("Vault Deposit note hash: {:?}", deposit_note.id().to_hex());

    // 8. Notu yayınlamak (publish) için işlemi hazırlayıp gönderici hesaptan ağa gönderiyoruz
    println!("Publishing vault deposit note to the network...");
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

    // 9. Yayınlanan bu notu kasa hesabı (vault_account) üzerinden tüketiyoruz (consume)
    println!("Consuming note on the vault account to complete time-locked deposit...");
    let consume_note_request = TransactionRequestBuilder::new()
        .input_notes([(deposit_note.clone(), None)])
        .build()
        .context("Failed to build consume note transaction request")?;

    let consume_tx_id = client
        .submit_new_transaction(vault_account.id(), consume_note_request)
        .await
        .context("Failed to create consume note transaction")?;

    println!("Consume transaction ID: {:?}", consume_tx_id.to_hex());
    println!("SUCCESS! Miden Time-Locked Vault deposit flow completed on live Miden Testnet!");

    Ok(())
}