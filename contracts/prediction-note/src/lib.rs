#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

#[account(prediction_market::PredictionMarketContract)]
pub struct PredictionMarket;

#[note]
struct PredictionNote;

#[note]
impl PredictionNote {
    #[note_script]
    fn run(self, _arg: Word, market: &mut PredictionMarket) {
        let market_id = Word::new([
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
            Felt::ONE,
        ]);

        let position = Felt::from_u32(1); // YES
        let amount = Felt::from_u32(10);

        let initial = market.get_position_total(market_id, position);

        let final_total =
            market.place_prediction(market_id, position, amount);

        assert_eq!(final_total, initial + amount);
    }
}
