#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, Felt, StorageMap, Word};

// 1. Depolama alanı struct'ını #[component_storage] ile tanımlıyoruz
#[component_storage]
pub struct BankAccountStorage {
    #[storage(description = "User balances inside the bank")]
    pub balances: StorageMap<Word, Felt>,
}

// 2. Trait arayüz ismini BankContract, deposit ismini bank_deposit olarak güncelliyoruz
#[component]
pub trait BankContract {
    fn bank_deposit(&mut self, user_key: Word, amount: Felt) -> Felt; // <-- bank_deposit yapıldı
    fn check_balance(&self, user_key: Word) -> Felt;
}

// 3. Trait'i #[component] impl bloğu ile uyguluyoruz
#[component]
impl BankContract for BankAccountStorage {
    /// Kullanıcının banka hesabına belirli bir miktarda varlık depozit eder.
    fn bank_deposit(&mut self, user_key: Word, amount: Felt) -> Felt { // <-- bank_deposit yapıldı
        assert!(amount > Felt::ZERO, "Depozit miktari sifirdan buyuk olmalidir!");

        let current_balance: Felt = self.balances.get(user_key);
        let next_balance = current_balance + amount;
        self.balances.set(user_key, next_balance);

        next_balance
    }

    fn check_balance(&self, user_key: Word) -> Felt {
        self.balances.get(user_key)
    }
}