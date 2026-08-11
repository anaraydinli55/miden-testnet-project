#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, component_storage, Felt, StorageMap, Word};

// 1. Səsvermə deposunu müəyyən edirik
#[component_storage]
pub struct BallotBoxStorage {
    #[storage(description = "Proposal vote tallies inside the ballot box")]
    pub votes: StorageMap<Word, Felt>,
}

// 2. Səsvermə funksiyalarının arayüzü (Trait)
#[component]
pub trait BallotContract {
    fn cast_vote(&mut self, candidate_id: Word) -> Felt;
    fn get_votes(&self, candidate_id: Word) -> Felt;
}

// 3. Trait-in depolama alanımız üzərində icrası
#[component]
impl BallotContract for BallotBoxStorage {
    /// Müvafiq namizəd ID-sinə (candidate_id) 1 səs əlavə edir.
    /// Güncəl ümumi səs sayını geri qaytarır.
    fn cast_vote(&mut self, candidate_id: Word) -> Felt {
        // Namizədin mövcud səs sayını depodan çək
        let current_votes: Felt = self.votes.get(candidate_id);
        
        // Səs sayını 1 artır
        let next_votes = current_votes + Felt::from_u32(1);
        
        // Yeni səs sayını depoya qeyd et
        self.votes.set(candidate_id, next_votes);
        
        next_votes
    }

    /// Namizədin hazırda neçə səs aldığını sorğulayır.
    fn get_votes(&self, candidate_id: Word) -> Felt {
        self.votes.get(candidate_id)
    }
}