#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

/// Notun bağlandığı hədəf hesab: amm-pool paketindəki AmmContract arayüzünü qoşduq
#[account(amm_pool::AmmContract)]
pub struct AmmPool;

#[note]
struct AmmSwapNote;

#[note]
impl AmmSwapNote {
    #[note_script]
    fn run(self, _arg: Word, pool: &mut AmmPool) {
        // 1. Havuza ilk likiditeyi ekliyoruz: 10 SKS ve 100 USDCx
        let init_sks = Felt::from_u32(10);
        let init_usdcx = Felt::from_u32(100);
        pool.add_liquidity(init_sks, init_usdcx);
        
        // 2. Takas yapacağımız SKS miktarı: 3 SKS
        let swap_sks = Felt::from_u32(3);
        
        // 3. SKS'leri verip USDCx stablecoin'i alıyoruz (1 SKS = 10 USDCx oranına göre)
        let payout_usdcx = pool.swap_sks_for_usdcx(swap_sks);
        
        // 4. Alacağımız miktarın tam olarak 30 USDCx (3 * 10) olduğunu ZK seviyesinde doğruluyoruz!
        let expected_usdcx = Felt::from_u32(30);
        assert_eq(payout_usdcx, expected_usdcx);
    }
}
