#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

#[account(time_lock_vault::VaultContract)]
pub struct Vault;

#[note]
struct VaultWithdrawNote;

#[note]
impl VaultWithdrawNote {
    #[note_script]
    fn run(self, _arg: Word, vault: &mut Vault) {
        let user_key = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);
        
        let initial_balance = vault.check_balance(user_key);
        let amount = Felt::from_u32(40);
        let current_block = Felt::from_u32(1060); // 1050-dən böyük olduğu üçün kilid açılacaqdır!
        
        let final_balance = vault.withdraw(user_key, amount, current_block);
        
        let expected_balance = initial_balance - amount;
        assert_eq(final_balance, expected_balance);
    }
}