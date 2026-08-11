#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

#[account(time_lock_vault::VaultContract)]
pub struct Vault;

#[note]
struct VaultDepositNote;

#[note]
impl VaultDepositNote {
    #[note_script]
    fn run(self, _arg: Word, vault: &mut Vault) {
        let user_key = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);
        
        let initial_balance = vault.check_balance(user_key);
        let amount = Felt::from_u32(100);
        let current_block = Felt::from_u32(1000);  // Blok 1000-də yatırırıq
        let lock_duration = Felt::from_u32(50);     // 50 blok boyunca kilidlə (Açılış: 1050)
        
        let final_balance = vault.deposit(user_key, amount, current_block, lock_duration);
        
        let expected_balance = initial_balance + amount;
        assert_eq(final_balance, expected_balance);
        
        let unlock_block = vault.get_unlock_block(user_key);
        assert_eq(unlock_block, current_block + lock_duration);
    }
}