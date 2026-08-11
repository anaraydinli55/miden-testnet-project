#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, Felt, StorageMap, Word};

// 1. Depolama alanı
#[component_storage]
pub struct BankAccountStorage {
    #[storage(description = "User balances inside the bank")]
    pub balances: StorageMap<Word, Felt>,
}

// 2. Trait arayüzüne withdraw fonksiyonunu ekledik
#[component]
pub trait BankContract {
    fn bank_deposit(&mut self, user_key: Word, amount: Felt) -> Felt;
    fn withdraw(&mut self, user_key: Word, amount: Felt) -> Felt; // <-- Yeni eklendi!
    fn check_balance(&self, user_key: Word) -> Felt;
}

// 3. Trait uygulaması
#[component]
impl BankContract for BankAccountStorage {
    /// Kullanıcının banka hesabına belirli bir miktarda varlık depozit eder.
    fn bank_deposit(&mut self, user_key: Word, amount: Felt) -> Felt {
        assert!(amount > Felt::ZERO, "Depozit miktari sifirdan buyuk olmalidir!");

        let current_balance: Felt = self.balances.get(user_key);
        let next_balance = current_balance + amount;
        self.balances.set(user_key, next_balance);

        next_balance
    }

    /// Kullanıcının bankadaki bakiyesinden para çekmesini sağlar.
    fn withdraw(&mut self, user_key: Word, amount: Felt) -> Felt { // <-- Yeni eklendi!
        assert!(amount > Felt::ZERO, "Cekilmek istenen miktar sifirdan buyuk olmalidir!");

        // Kullanıcının mevcut bakiyesini al
        let current_balance: Felt = self.balances.get(user_key);

        // Hata Kontrolü: Bankada çekilmek istenenden az para varsa işlemi reddet (assert)
        assert!(current_balance >= amount, "Yetersiz banka bakiyesi!");

        // Yeni bakiyeyi hesapla ve kaydet
        let next_balance = current_balance - amount;
        self.balances.set(user_key, next_balance);

        next_balance
    }

    fn check_balance(&self, user_key: Word) -> Felt {
        self.balances.get(user_key)
    }
}