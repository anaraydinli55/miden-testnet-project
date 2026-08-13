#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

/// Notun bağlandığı hədəf hesab: escrow-contract-dakı EscrowContract arayüzünü qoşduq
#[account(escrow_contract::EscrowContract)]
pub struct Escrow;

#[note]
struct SwapNote;

#[note]
impl SwapNote {
    #[note_script]
    fn run(self, _arg: Word, escrow: &mut Escrow) {
        // Tərəflərin sembolik açarları (Party A və Party B)
        let party_a = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);
        let party_b = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::from_u32(2)]);
        
        // 1. Tərəf A SKS əmanət depozit edir: 50 birim
        let amount_a = Felt::from_u32(50);
        escrow.deposit_sks(party_a, amount_a);
        
        // 2. Tərəf B USDCx əmanət depozit edir: 5 birim
        let amount_b = Felt::from_u32(5);
        escrow.deposit_usdcx(party_b, amount_b);
        
        // 3. Mübadilə (Swap) əmrini icra edirik!
        let result = escrow.execute_sks_usdcx_swap(party_a, party_b);
        
        // 4. Mübadilənin uğurlu olduğunu (1 döndüyünü) ZK səviyyəsində doğrulayırıq
        assert_eq(result, Felt::from_u32(1));
    }
}
