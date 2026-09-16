#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, Felt, StorageMap, Word};

#[component_storage]
pub struct PredictionMarketStorage {
    #[storage(description = "Total prediction amount for each market")]
    pub market_totals: StorageMap<Word, Felt>,

    #[storage(description = "YES prediction amount for each market")]
    pub yes_totals: StorageMap<Word, Felt>,

    #[storage(description = "NO prediction amount for each market")]
    pub no_totals: StorageMap<Word, Felt>,
}

#[component]
pub trait PredictionMarketContract {
    fn place_prediction(
        &mut self,
        market_id: Word,
        position: Felt,
        amount: Felt,
    ) -> Felt;

    fn get_position_total(
        &self,
        market_id: Word,
        position: Felt,
    ) -> Felt;

    fn get_market_total(&self, market_id: Word) -> Felt;
}

#[component]
impl PredictionMarketContract for PredictionMarketStorage {
    fn place_prediction(
        &mut self,
        market_id: Word,
        position: Felt,
        amount: Felt,
    ) -> Felt {
        assert!(
            position == Felt::from_u32(1)
                || position == Felt::from_u32(2),
            "Position must be YES(1) or NO(2)"
        );

        assert!(
            amount > Felt::ZERO,
            "Prediction amount must be greater than zero"
        );

        let current_market_total = self.market_totals.get(market_id);
        let next_market_total = current_market_total + amount;

        self.market_totals.set(market_id, next_market_total);

        if position == Felt::from_u32(1) {
            let current_yes = self.yes_totals.get(market_id);
            let next_yes = current_yes + amount;

            self.yes_totals.set(market_id, next_yes);

            next_yes
        } else {
            let current_no = self.no_totals.get(market_id);
            let next_no = current_no + amount;

            self.no_totals.set(market_id, next_no);

            next_no
        }
    }

    fn get_position_total(
        &self,
        market_id: Word,
        position: Felt,
    ) -> Felt {
        assert!(
            position == Felt::from_u32(1)
                || position == Felt::from_u32(2),
            "Position must be YES(1) or NO(2)"
        );

        if position == Felt::from_u32(1) {
            self.yes_totals.get(market_id)
        } else {
            self.no_totals.get(market_id)
        }
    }

    fn get_market_total(&self, market_id: Word) -> Felt {
        self.market_totals.get(market_id)
    }
}
