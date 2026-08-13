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
async fn test_amm_liquidity_and_swap() -> anyhow::Result<()> {
    // 1. Lokal MockChain test zənciri qurucusunu başladırıq
    let mut builder = MockChain::builder();

    // 2. Havuzla etkileşime girecek olan kullanıcı cüzdanını ekliyoruz
    let sender = builder.add_existing_wallet(Auth::BasicAuth {
        auth_scheme: AuthSchemeId::Falcon512Poseidon2,
    })?;

    // 3. amm-pool ve amm-swap-note sözleşmelerini RELEASE modunda derliyoruz
    let contract_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/amm-pool"),
        true, // release = true (FPI derleyici sorunlarını çözer!)
    )?);
    let note_package = Arc::new(build_project_in_dir(
        Path::new("../contracts/amm-swap-note"),
        true, // release = true (FPI derleyici sorunlarını çözer!)
    )?);

    // 4. amm-pool'da tanımladığımız reserves StorageMap'inin slot adını tanımlıyoruz
    let reserves_slot = StorageSlotName::new("amm_pool::amm_contract::reserves")
        .context("invalid reserves storage slot name")?;

    // SKS ve USDCx rezervlerini tutacak anahtarlar (SKS için son eleman 1, USDCx için 2)
    let key_sks = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::from_u32(1)]);
    let key_usdcx = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::from_u32(2)]);

    // 5. Havuz rezervlerini başlangıçta 0 olarak ilklendiriyoruz
    let mut init_storage_data = InitStorageData::default();
    init_storage_data.insert_map_entry(reserves_slot.clone(), key_sks, 0_u64)?;
    init_storage_data.insert_map_entry(reserves_slot.clone(), key_usdcx, 0_u64)?;

    // 6. Derlediğimiz paketten (MASP) havuz bileşenini ayağa kaldırıyoruz
    let amm_component = AccountComponent::from_package(&contract_package, &init_storage_data)
        .context("failed to build account component from amm package")?;

    // 7. Bu bileşeni barındıran halka açık (public) yeni bir ZK-AMM Havuz Hesabı oluşturuyoruz
    let amm_account = builder.add_account_from_builder(
        Auth::BasicAuth {
            auth_scheme: AuthSchemeId::Falcon512Poseidon2,
        },
        AccountBuilder::new([6_u8; 32])
            .account_type(AccountType::Public)
            .with_component(amm_component),
        AccountState::Exists,
    )?;

    // 8. Tüketilecek Likidite ve Takas Notunu (AMM Swap Note) oluşturuyoruz
    let mut note_rng = RandomCoin::new(Word::from(
        NoteScript::from_package(note_package.as_ref())
            .context("failed to build note script from package")?
            .root(),
    ));
    let swap_note = NoteBuilder::new(sender.id(), &mut note_rng)
        .package((*note_package).clone())
        .build()
        .context("failed to build swap note from package")?;

    // 9. Oluşturulan notu MockChain'e çıktı notu olarak ekliyoruz
    builder.add_output_note(RawOutputNote::Full(swap_note.clone()));

    // 10. Sanal zinciri inşa ediyoruz
    let mut mock_chain = builder.build()?;

    // 11. Tranzaksiya bağlamını oluşturuyoruz
    let tx_context = mock_chain
        .build_tx_context(amm_account.clone(), &[swap_note.id()], &[])?
        .build()?;

    // 12. İşlemi yürütüyoruz (Havuza 10 SKS/100 USDCx eklenecek ve ardından 3 SKS verilip 30 USDCx çekilecektir!)
    let executed_transaction = tx_context.execute().await?;

    // 13. İşlemi zincire işleyip bloğu kapatıyoruz (Proving)
    mock_chain.add_pending_executed_transaction(&executed_transaction)?;
    mock_chain.prove_next_block()?;

    // 14. Güncellenmiş havuz hesabından SKS rezervini çekiyoruz (10 + 3 = 13 olmalıdır)
    let sks_reserves = mock_chain
        .committed_account(amm_account.id())?
        .storage()
        .get_map_item(&reserves_slot, key_sks)
        .expect("Failed to get SKS reserves from storage slot");
    
    // 15. Güncellenmiş havuz hesabından USDCx rezervini çekiyoruz (100 - 30 = 70 olmalıdır)
    let usdcx_reserves = mock_chain
        .committed_account(amm_account.id())?
        .storage()
        .get_map_item(&reserves_slot, key_usdcx)
        .expect("Failed to get USDCx reserves from storage slot");

    // 16. Rezervlerin ZK-VM üzerinde tam olarak doğru güncellendiğini doğruluyoruz!
    assert_eq!(sks_reserves[0].as_canonical_u64(), 13, "SKS rezervi 13 olmalıdır!");
    assert_eq!(usdcx_reserves[0].as_canonical_u64(), 70, "USDCx rezervi 70 olmalıdır!");

    println!("SUCCESS! Miden ZK-AMM Liquidity Pool and Swap simulation completed successfully!");
    Ok(())
}