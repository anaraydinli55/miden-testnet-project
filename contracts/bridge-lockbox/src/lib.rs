#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, felt, Felt, StorageMap, Word};

#[component_storage]
pub struct BridgeLockboxStorage {
    #[storage(description = "Locked SKS balances per account")]
    pub locked_balances: StorageMap<Word, Felt>,
    #[storage(description = "Nonce counter per account")]
    pub nonces: StorageMap<Word, Felt>,
}

#[component]
pub trait BridgeLockbox {
    fn lock_sks(&mut self, amount: Felt, evm_dest: Word) -> Felt;
    fn unlock_sks(&mut self, account_id: Word, amount: Felt, nonce: Felt) -> Felt;
    fn get_locked(&self, account_id: Word) -> Felt;
}

#[component]
impl BridgeLockbox for BridgeLockboxStorage {
    fn lock_sks(&mut self, amount: Felt, evm_dest: Word) -> Felt {
        let current = self.locked_balances.get(evm_dest);
        let new_amount = current + amount;
        self.locked_balances.set(evm_dest, new_amount);
        let nonce = self.nonces.get(evm_dest) + felt!(1);
        self.nonces.set(evm_dest, nonce);
        nonce
    }
    
    fn unlock_sks(&mut self, account_id: Word, amount: Felt, nonce: Felt) -> Felt {
        let current = self.locked_balances.get(account_id);
        assert!(current >= amount, "Insufficient locked balance");
        let new_amount = current - amount;
        self.locked_balances.set(account_id, new_amount);
        nonce
    }
    
    fn get_locked(&self, account_id: Word) -> Felt {
        self.locked_balances.get(account_id)
    }
}
