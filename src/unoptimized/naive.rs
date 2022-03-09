use roget::{Dictionary, Guess, Guesser, Word};

pub struct Unoptimized<'l> {
    dictionary: &'l Dictionary,
}

impl<'l> Unoptimized<'l> {
    /// Takes a borrowed Dictionary that it uses to guess from.
    pub fn new(dictionary: &'l Dictionary) -> Self {
        Self { dictionary }
    }
}

impl<'l> Guesser for Unoptimized<'l> {
    fn guess(&mut self, _past_guesses: &[Guess]) -> Word {
        b"which".to_owned()
    }
}
