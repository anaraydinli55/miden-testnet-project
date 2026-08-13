#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, Felt, StorageMap, Word};

// 1. Mübadilə əmanət deposunu StorageMap olaraq müəyyən edirik
#[component_storage]
pub struct EscrowStorage {
    #[storage(description = "Deposited assets inside the escrow")]
    pub deposits: StorageMap<Word, Felt>,
}

// 2. Mübadilə müqaviləsinin arayüzü (Trait) - SKS ve USDCx fonksiyonları eklendi
#[component]
pub trait EscrowContract {
    fn deposit_sks(&mut self, party_key: Word, amount: Felt) -> Felt;
    fn deposit_usdcx(&mut self, party_key: Word, amount: Felt) -> Felt;
    fn execute_sks_usdcx_swap(&mut self, party_a: Word, party_b: Word) -> Felt;
    fn get_sks_deposit(&self, party_key: Word) -> Felt;
    fn get_usdcx_deposit(&self, party_key: Word) -> Felt;
}

// 3. Trait-in depolama sahəmiz üzərində icrası
#[component]
impl EscrowContract for EscrowStorage {
    /// Tərəfin SKS əmanətini qəbul edir.
    fn deposit_sks(&mut self, party_key: Word, amount: Felt) -> Felt {
        assert!(amount > Felt::ZERO, "Emanet miktari sifirdan buyuk olmalidir!");

        // SKS için unikal bir anahtar silsiləsi qururuq (Dizinin son elemanını 1 yapıyoruz)
        let mut key = party_key;
        key[3] = Felt::from_u32(1);

        let current: Felt = self.deposits.get(key);
        let next = current + amount;
        self.deposits.set(key, next);

        next
    }

    /// Tərəfin USDCx əmanətini qəbul edir.
    fn deposit_usdcx(&mut self, party_key: Word, amount: Felt) -> Felt {
        assert!(amount > Felt::ZERO, "Emanet miktari sifirdan buyuk olmalidir!");

        // USDCx için unikal bir anahtar silsiləsi qururuq (Dizinin son elemanını 2 yapıyoruz)
        let mut key = party_key;
        key[3] = Felt::from_u32(2);

        let current: Felt = self.deposits.get(key);
        let next = current + amount;
        self.deposits.set(key, next);

        next
    }

    /// Hər iki tərəf əmanəti (SKS ve USDCx) yatırdıqda gizli mübadiləni icra edir.
    fn execute_sks_usdcx_swap(&mut self, party_a: Word, party_b: Word) -> Felt {
        let mut key_a_sks = party_a;
        key_a_sks[3] = Felt::from_u32(1);

        let mut key_b_usdcx = party_b;
        key_b_usdcx[3] = Felt::from_u32(2);

        let deposit_a = self.deposits.get(key_a_sks);
        let deposit_b = self.deposits.get(key_b_usdcx);

        // ZK-Yoxlaması: A tərəfi SKS, B tərəfi isə USDCx yatırmalıdır!
        assert!(deposit_a > Felt::ZERO, "Taraf A henüz SKS emanet etmedi!");
        assert!(deposit_b > Felt::ZERO, "Taraf B henüz USDCx emanet etmedi!");

        // Mübadiləni depoda icra edirik (əmanətləri sıfırlayırıq)
        self.deposits.set(key_a_sks, Felt::ZERO);
        self.deposits.set(key_b_usdcx, Felt::ZERO);

        Felt::from_u32(1) // 1 = Uğurlu ZK mübadilə təsdiqi!
    }

    fn get_sks_deposit(&self, party_key: Word) -> Felt {
        let mut key = party_key;
        key[3] = Felt::from_u32(1);
        self.deposits.get(key)
    }

    fn get_usdcx_deposit(&self, party_key: Word) -> Felt {
        let mut key = party_key;
        key[3] = Felt::from_u32(2);
        self.deposits.get(key)
    }
}