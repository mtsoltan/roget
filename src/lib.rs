#![feature(slice_as_chunks)]

pub type Word = [u8; 5];

pub trait RepresentableAsWord {
    fn as_word(&self) -> &Word;
}

impl RepresentableAsWord for str {
    fn as_word(&self) -> &Word {
        let (chunks, _): (&[[u8; 5]], &[u8]) = self.as_bytes().as_chunks();

        return &chunks[0];
    }
}

/// A function play that takes a generic G that implements the trait Guesser.
pub fn play<G: Guesser>(answer: &'static Word, guesser: &mut G) -> Option<usize> {
    // play six rounds where it invokes the guesser each round
    let mut past_guesses = Vec::new();

    // Wordle only allows six guesses. We allow more to avoid chopping off the score distribution
    // for stats purposes.
    for i in 1..=32 {
        let guessed_word = guesser.guess(&past_guesses[..]);

        if guessed_word.eq(answer) {
            return Some(i);
        }

        let correctness_mask = Correctness::check(answer, guessed_word);
        past_guesses.push(Guess {
            word: guessed_word,
            mask: correctness_mask,
        });
    }

    None
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    fn check(answer: &'static Word, guessed_word: Word) -> [Self; 5] {
        let mut rv = [Self::Wrong; 5];
        for i in 0..5 {
            if answer[i] == guessed_word[i] {
                rv[i] = Self::Correct;
            } else if answer.contains(&guessed_word[i]) {
                rv[i] = Self::Misplaced;
            }
        }

        return rv;
    }
}

pub struct Guess {
    word: Word,
    mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, past_guesses: &[Guess]) -> Word;
}

// We don't need this anymore for some reason, we dont want an impl for the trait Guesser.
//
// /// An implementation, Guesser taking a generic, T, where T implements the trait Guesser.
// impl<T> Guesser for &mut T
// where
//     T: Guesser,
// {
//     fn guess(&mut self, past_guesses: &[Guess]) -> String {}
// }
