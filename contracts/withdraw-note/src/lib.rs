#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

/// Notun bağlandığı hedef hesap: BankContract arayüzünü bağladık
#[account(bank_account::BankContract)]
pub struct Bank;

#[note]
struct WithdrawNote;

#[note]
impl WithdrawNote {
    #[note_script]
    fn run(self, _arg: Word, bank: &mut Bank) {
        // Test için kullandığımız sembolik kullanıcı anahtarı (User Key Word)
        let user_key = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);
        
        // İşlem öncesindeki bakiye durumunu sorguluyoruz
        let initial_balance = bank.check_balance(user_key);
        
        // Bankadan çekilecek miktar: 20 birim
        let amount = Felt::from_u32(20);
        
        // bank_deposit yerine bu kez withdraw fonksiyonunu çağırıyoruz!
        let final_balance = bank.withdraw(user_key, amount);
        
        // Bakiyenin başarıyla 20 birim azaldığını doğruluyoruz (assert)
        let expected_balance = initial_balance - amount;
        assert_eq(final_balance, expected_balance);
    }
}