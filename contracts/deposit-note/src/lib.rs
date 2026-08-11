#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

/// Notun bağlandığı hedef hesap: BankContract arayüzünü bağladık
#[account(bank_account::BankContract)]
pub struct Bank;

#[note]
struct DepositNote;

#[note]
impl DepositNote {
    #[note_script]
    fn run(self, _arg: Word, bank: &mut Bank) {
        let user_key = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);
        
        let initial_balance = bank.check_balance(user_key);
        let amount = Felt::from_u32(50);
        
        // bank_deposit çağrısı yapacak şekilde güncelledik
        let final_balance = bank.bank_deposit(user_key, amount);
        
        let expected_balance = initial_balance + amount;
        assert_eq(final_balance, expected_balance);
    }
}