#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, Felt, StorageMap, Word};

// 1. Kasanın iki fərqli depolama sahəsini (balances və unlock_blocks) müəyyən edirik
#[component_storage]
pub struct TimeLockVaultStorage {
    #[storage(description = "User token balances")]
    pub balances: StorageMap<Word, Felt>,
    
    #[storage(description = "Block height at which the user balance unlocks")]
    pub unlock_blocks: StorageMap<Word, Felt>,
}

// 2. Kasa funksiyalarının arayüzü (Trait)
#[component]
pub trait VaultContract {
    fn deposit(&mut self, user_key: Word, amount: Felt, current_block: Felt, lock_duration: Felt) -> Felt;
    fn withdraw(&mut self, user_key: Word, amount: Felt, current_block: Felt) -> Felt;
    fn check_balance(&self, user_key: Word) -> Felt;
    fn get_unlock_block(&self, user_key: Word) -> Felt;
}

// 3. Trait-in depolama alanımız üzərində icrası
#[component]
impl VaultContract for TimeLockVaultStorage {
    /// İstifadəçinin əmanətini qəbul edir və hədəf kilid açılış blokunu təyin edir.
    fn deposit(&mut self, user_key: Word, amount: Felt, current_block: Felt, lock_duration: Felt) -> Felt {
        assert!(amount > Felt::ZERO, "Depozit miktari sifirdan buyuk olmalidir!");

        // İstifadəçinin mövcud balansını artır
        let current_balance: Felt = self.balances.get(user_key);
        let next_balance = current_balance + amount;
        self.balances.set(user_key, next_balance);

        // Hədəf kilid açılış blokunu hesablayırıq (current_block + lock_duration)
        let unlock_block = current_block + lock_duration;
        self.unlock_blocks.set(user_key, unlock_block);

        next_balance
    }

    /// Zaman kilidi dolduqda istifadəçinin pulunu çəkməsini təmin edir.
    fn withdraw(&mut self, user_key: Word, amount: Felt, current_block: Felt) -> Felt {
        assert!(amount > Felt::ZERO, "Cekilmek istenen miktar sifirdan buyuk olmalidir!");

        // 1. Hata Kontrolü: Zaman kilidinin açılıb-açılmadığını ZK səviyyəsində yoxlayırıq!
        let unlock_block: Felt = self.unlock_blocks.get(user_key);
        assert!(current_block >= unlock_block, "Zaman kilidi hele dolmayib! Pul kilidlidir.");

        // 2. Hata Kontrolü: Balans yoxlaması
        let current_balance: Felt = self.balances.get(user_key);
        assert!(current_balance >= amount, "Yetersiz kasa bakiyesi!");

        // Balansı yenilə və qeyd et
        let next_balance = current_balance - amount;
        self.balances.set(user_key, next_balance);

        next_balance
    }

    fn check_balance(&self, user_key: Word) -> Felt {
        self.balances.get(user_key)
    }

    fn get_unlock_block(&self, user_key: Word) -> Felt {
        self.unlock_blocks.get(user_key)
    }
}