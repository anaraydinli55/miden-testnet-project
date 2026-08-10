#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, Felt, StorageMap, Word};

// 1. Depolama alanı struct'ını #[component_storage] ile tanımlıyoruz
#[component_storage]
pub struct BankAccountStorage {
    #[storage(description = "User balances inside the bank")]
    pub balances: StorageMap<Word, Felt>,
}

// 2. Dışarıya açılacak fonksiyon arayüzlerini #[component] trait ile tanımlıyoruz
#[component]
pub trait BankAccount {
    fn deposit(&mut self, user_key: Word, amount: Felt) -> Felt;
    fn get_balance(&self, user_key: Word) -> Felt;
}

// 3. Trait'i depolama alanımız için #[component] impl bloğu ile uyguluyoruz
#[component]
impl BankAccount for BankAccountStorage {
    /// Kullanıcının banka hesabına belirli bir miktarda varlık depozit eder.
    /// Güncellenmiş yeni bakiye değerini döner.
    fn deposit(&mut self, user_key: Word, amount: Felt) -> Felt {
        // Hata Kontrolü: Depozit miktarı sıfırdan büyük olmalıdır
        assert!(amount > Felt::ZERO, "Depozit miktari sifirdan buyuk olmalidir!");

        // Kullanıcının mevcut bakiyesini çek (& işaretini sildik)
        let current_balance: Felt = self.balances.get(user_key);
        
        // Yeni bakiyeyi hesapla
        let next_balance = current_balance + amount;
        
        // Yeni bakiyeyi kaydet
        self.balances.set(user_key, next_balance);
        
        next_balance
    }
    
    /// Belirtilen kullanıcının bankadaki güncel bakiyesini sorgular.
    fn get_balance(&self, user_key: Word) -> Felt {
        // & işaretini sildik
        self.balances.get(user_key)
    }
}