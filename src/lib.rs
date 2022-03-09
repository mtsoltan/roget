#![feature(slice_as_chunks)]

use std::collections::HashSet;

const WORD_SIZE: usize = 5;

/// Wordle only allows six guesses. We allow more to avoid chopping off the score distribution
/// for stats purposes.
const TRIES_BEFORE_LOSS: usize = 32;

pub type Word = [u8; WORD_SIZE];
pub type Dictionary = HashSet<&'static Word>;

pub trait RepresentableAsWord {
    fn as_word(&self) -> &Word;
}

impl RepresentableAsWord for str {
    fn as_word(&self) -> &Word {
        let (chunks, _): (&[[u8; WORD_SIZE]], &[u8]) = self.as_bytes().as_chunks();

        return &chunks[0];
    }
}

pub struct Wordle {
    dictionary: Dictionary,
}

impl Wordle {
    pub fn new<I: IntoIterator<Item = &'static Word>>(iter: I) -> Self {
        Self {
            dictionary: Dictionary::from_iter(iter),
        }
    }

    pub fn get_dictionary(&self) -> &Dictionary {
        return &self.dictionary;
    }

    /// A function play that takes a generic G that implements the trait Guesser.
    pub fn play<G: Guesser>(&self, answer: &'static Word, mut guesser: G) -> Option<usize> {
        // play six rounds where it invokes the guesser each round
        let mut past_guesses = Vec::new();

        for i in 1..=TRIES_BEFORE_LOSS {
            let guessed_word = guesser.guess(&past_guesses[..]);
            assert!(self.dictionary.contains(&guessed_word));

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
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    /// Check can't just check for misplaced using:
    /// ```
    /// answer.contains(&guessed_word[i])
    /// ```
    /// because it takes care of counts when deciding whether it is misplaced or wrong.
    fn check(answer: &'static Word, guessed_word: Word) -> [Self; WORD_SIZE] {
        let mut rv = [Self::Wrong; WORD_SIZE];
        let mut used = [false; WORD_SIZE];
        for i in 0..WORD_SIZE {
            if answer[i] == guessed_word[i] {
                rv[i] = Self::Correct;
                used[i] = true;
            };
        }

        for i in 0..WORD_SIZE {
            for j in 0..WORD_SIZE {
                if rv[i] != Self::Correct && !used[j] && answer[j] == guessed_word[i] {
                    rv[i] = Self::Misplaced;
                    used[j] = true;
                    break;
                }
            }
        }

        rv
    }
}

pub struct Guess {
    word: Word,
    mask: [Correctness; WORD_SIZE],
}

pub trait Guesser {
    fn guess(&mut self, past_guesses: &[Guess]) -> Word;
}

/// We want to allow functions to be guessers, which just calls `self` on `past_guesses`.
impl Guesser for fn(past_guesses: &[Guess]) -> Word {
    fn guess(&mut self, past_guesses: &[Guess]) -> Word {
        (*self)(past_guesses)
    }
}

macro_rules! guesser {
    ($func:expr) => {
        ($func) as fn(past_guesses: &[Guess]) -> Word
    };
}

#[cfg(test)]
mod tests {
    mod play_wordle {
        use crate::{Guess, Guesser, RepresentableAsWord, Word, Wordle};

        const DICTIONARY: &'static str = include_str!("../dictionary.txt");

        #[test]
        fn guess_first_time_correctly() {
            let wordle: Wordle = Wordle::new(
                DICTIONARY
                    .split_ascii_whitespace()
                    .map(|word_str| word_str.as_word()),
            );

            assert_eq!(
                wordle.play(b"moved", guesser!(|_past: &[Guess]| b"moved".to_owned())),
                Some(1)
            );
        }

        #[test]
        fn guess_second_time_correctly() {
            let wordle: Wordle = Wordle::new(
                DICTIONARY
                    .split_ascii_whitespace()
                    .map(|word_str| word_str.as_word()),
            );

            pub struct GuessesThirdTimeCorrectly {
                guesses_made: usize,
            }

            impl Guesser for GuessesThirdTimeCorrectly {
                fn guess(&mut self, _past_guesses: &[Guess]) -> Word {
                    self.guesses_made += 1;
                    if self.guesses_made == 3 {
                        return b"moved".to_owned();
                    }

                    b"which".to_owned()
                }
            }

            let guesser = assert_eq!(
                wordle.play(b"moved", GuessesThirdTimeCorrectly { guesses_made: 0 }),
                Some(3)
            );
        }

        #[test]
        fn dont_guess_correctly() {
            let wordle: Wordle = Wordle::new(
                DICTIONARY
                    .split_ascii_whitespace()
                    .map(|word_str| word_str.as_word()),
            );

            pub struct DoesNotGuessCorrectly;

            impl Guesser for DoesNotGuessCorrectly {
                fn guess(&mut self, _past_guesses: &[Guess]) -> Word {
                    b"which".to_owned()
                }
            }

            assert_eq!(wordle.play(b"moved", DoesNotGuessCorrectly {}), None);
        }
    }

    mod check_correctness {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[$(mask!($c)),+]}
        }

        #[test]
        fn all_green() {
            assert_eq!(
                Correctness::check(b"hello", b"hello".to_owned()),
                mask![C C C C C]
            );
        }

        #[test]
        fn all_gray() {
            assert_eq!(
                Correctness::check(b"hello", b"pqrst".to_owned()),
                mask![W W W W W]
            );
        }

        #[test]
        fn all_yellow() {
            assert_eq!(
                Correctness::check(b"hello", b"llohe".to_owned()),
                mask![M M M M M]
            );
        }

        #[test]
        fn actual_words() {
            assert_eq!(
                Correctness::check(b"hello", b"world".to_owned()),
                mask![W M W C W]
            );
        }

        #[test]
        fn guess_single_letter() {
            assert_eq!(
                Correctness::check(b"hello", b"lllll".to_owned()),
                mask![W W C C W]
            );
        }

        #[test]
        fn guess_with_more_of_a_letter_than_needed() {
            assert_eq!(
                Correctness::check(b"azzaz", b"aaabb".to_owned()),
                mask![C M W W W]
            );
        }
    }
}
