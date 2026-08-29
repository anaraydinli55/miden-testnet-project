#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

#[account(bridge_lockbox::BridgeLockbox)]
pub struct BridgeAccount;

#[note]
struct BridgeLockNote;

#[note]
impl BridgeLockNote {
    #[note_script]
    fn run(self, args: Word, account: &mut BridgeAccount) {
        let amount = args[0];
        let evm_dest = Word::new([args[1], args[2], args[3], felt!(0)]);
        account.lock_sks(amount, evm_dest);
    }
}
