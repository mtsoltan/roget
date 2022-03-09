use roget::{Dictionary, DictionaryWithCounts, Guess, Guesser, Word};
use std::collections::HashMap;

pub struct Unoptimized<'l> {
    dictionary: &'l Dictionary,
    remaining: DictionaryWithCounts,
}

impl<'l> Unoptimized<'l> {
    /// Takes a borrowed Dictionary that it uses to guess from.
    pub fn new(dictionary: &'l Dictionary, remaining: DictionaryWithCounts) -> Self {
        Self {
            dictionary,
            remaining,
        }
    }
}

impl<'l> Guesser for Unoptimized<'l> {
    fn guess(&mut self, _past_guesses: &[Guess]) -> Word {
        b"which".to_owned()
    }
}
