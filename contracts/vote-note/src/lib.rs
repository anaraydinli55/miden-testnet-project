#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

/// Notun bağlandığı hədəf hesab: ballot_box paketindəki BallotContract arayüzünü qoşduq
#[account(ballot_box::BallotContract)]
pub struct Ballot;

#[note]
struct VoteNote;

#[note]
impl VoteNote {
    #[note_script]
    fn run(self, _arg: Word, ballot: &mut Ballot) {
        // Səs veriləcək namizədin ID-si (Sembolik Word olaraq 1)
        let candidate_id = Word::new([Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ONE]);
        
        // İşləmdən əvvəl namizədin mövcud səs sayını sorğulayırıq
        let initial_votes = ballot.get_votes(candidate_id);
        
        // Namizədə səs veririk (cast_vote çağırışı)
        let final_votes = ballot.cast_vote(candidate_id);
        
        // Səs sayının uğurla 1 artdığını ZK səviyyəsində doğrulayırıq (assert)
        let expected_votes = initial_votes + Felt::from_u32(1);
        assert_eq(final_votes, expected_votes);
    }
}