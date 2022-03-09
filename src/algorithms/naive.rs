use roget::{Correctness, Guess, Guesser, Word};

pub struct Naive;

impl Guesser for Naive {
    fn guess(&mut self, past_guesses: &[Guess]) -> Word {
        past_guesses;
        return b"which".to_owned();
    }
}
