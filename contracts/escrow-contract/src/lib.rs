#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, Felt, StorageMap, Word};

// 1. Mübadilə əmanət deposunu StorageMap olaraq müəyyən edirik
#[component_storage]
pub struct EscrowStorage {
    #[storage(description = "Deposited assets inside the escrow")]
    pub deposits: StorageMap<Word, Felt>,
}

// 2. Mübadilə müqaviləsinin arayüzü (Trait)
#[component]
pub trait EscrowContract {
    fn deposit_asset(&mut self, party_key: Word, amount: Felt) -> Felt;
    fn execute_swap(&mut self, party_a: Word, party_b: Word) -> Felt;
    fn get_deposit(&self, party_key: Word) -> Felt;
}

// 3. Trait-in depolama sahəmiz üzərində icrası
#[component]
impl EscrowContract for EscrowStorage {
    /// İstifadəçinin əmanətini qəbul edir və StorageMap-ə qeyd edir.
    fn deposit_asset(&mut self, party_key: Word, amount: Felt) -> Felt {
        assert!(amount > Felt::ZERO, "Emanet miktari sifirdan buyuk olmalidir!");

        let current: Felt = self.deposits.get(party_key);
        let next = current + amount;
        self.deposits.set(party_key, next);

        next
    }

    /// Hər iki tərəf əmanəti yatırdıqda gizli mübadiləni icra edir.
    /// Mübadilə uğurlu olduqda 1 (Felt) geri qaytarır.
    fn execute_swap(&mut self, party_a: Word, party_b: Word) -> Felt {
        let deposit_a = self.deposits.get(party_a);
        let deposit_b = self.deposits.get(party_b);

        // Hata Kontrolü: İki tərəfin də pul yatırdığını ZK səviyyəsində yoxlayırıq!
        assert!(deposit_a > Felt::ZERO, "Taraf A henüz emanet yatirmadi!");
        assert!(deposit_b > Felt::ZERO, "Taraf B henüz emanet yatirmadi!");

        // Mübadiləni depoda icra edirik (Simulyasiya olaraq emanetləri sıfırlayırıq)
        self.deposits.set(party_a, Felt::ZERO);
        self.deposits.set(party_b, Felt::ZERO);

        Felt::from_u32(1) // 1 = Uğurlu mübadilə təsdiqi!
    }

    fn get_deposit(&self, party_key: Word) -> Felt {
        self.deposits.get(party_key)
    }
}