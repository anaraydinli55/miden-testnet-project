#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, Felt, StorageMap, Word};

// 1. Havuz rezervlerini StorageMap daxilində saxlayırıq
#[component_storage]
pub struct AmmPoolStorage {
    #[storage(description = "Reserves of SKS and USDCx in the pool")]
    pub reserves: StorageMap<Word, Felt>,
}

// 2. Havuz funksiyalarının arayüzü (Trait)
#[component]
pub trait AmmContract {
    fn add_liquidity(&mut self, amount_sks: Felt, amount_usdcx: Felt) -> Felt;
    fn swap_sks_for_usdcx(&mut self, amount_sks: Felt) -> Felt;
    fn get_reserves(&self, token_key: Word) -> Felt;
}

// 3. Trait-in depolama alanımız üzərində icrası
#[component]
impl AmmContract for AmmPoolStorage {
    /// Havuza SKS ve USDCx likiditesi ekler.
    fn add_liquidity(&mut self, amount_sks: Felt, amount_usdcx: Felt) -> Felt {
        assert!(amount_sks > Felt::ZERO && amount_usdcx > Felt::ZERO, "Miktarlar sifirdan buyuk olmalidir!");

        let key_sks = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::from_u32(1)]);
        let key_usdcx = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::from_u32(2)]);

        // Mevcut rezervleri çek
        let current_sks = self.reserves.get(key_sks);
        let current_usdcx = self.reserves.get(key_usdcx);

        // Rezervleri güncelle
        self.reserves.set(key_sks, current_sks + amount_sks);
        self.reserves.set(key_usdcx, current_usdcx + amount_usdcx);

        Felt::from_u32(1) // 1 = Başarılı likidite ekleme onayı
    }

    /// Kullanıcının getirdiği SKS miktarını alır ve havuzdan ona 1 SKS = 10 USDCx oranında stablecoin öder.
    fn swap_sks_for_usdcx(&mut self, amount_sks: Felt) -> Felt {
        assert!(amount_sks > Felt::ZERO, "Takas miktari sifirdan buyuk olmalidir!");

        let key_sks = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::from_u32(1)]);
        let key_usdcx = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::from_u32(2)]);

        let current_sks = self.reserves.get(key_sks);
        let current_usdcx = self.reserves.get(key_usdcx);

        // Kullanıcıya ödenecek USDCx miktarını hesapla (1 SKS = 10 USDCx)
        let payout_usdcx = amount_sks * Felt::from_u32(10);

        // Hata Kontrolü: Havuzda yeterli USDCx likiditesi var mı?
        assert!(current_usdcx >= payout_usdcx, "Havuzda yeterli USDCx likiditesi yok!");

        // Rezervleri güncelle
        self.reserves.set(key_sks, current_sks + amount_sks);
        self.reserves.set(key_usdcx, current_usdcx - payout_usdcx);

        payout_usdcx // Kullanıcıya ödenecek USDCx miktarını dön
    }

    fn get_reserves(&self, token_key: Word) -> Felt {
        self.reserves.get(token_key)
    }
}