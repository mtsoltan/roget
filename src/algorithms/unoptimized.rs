use roget::{Correctness, Dictionary, DictionaryWithCounts, Guess, Guesser, Word, WORD_SIZE};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Candidate {
    /// The word of this candidate.
    word: &'static Word,

    /// The count coming form the DictionaryWithCounts value parameter. This lets us know how
    /// frequent this word is in the English language.
    occurrence_count: usize,

    /// How much this candidate will reduce the space of possible states.
    /// Information of 2 bits means that the candidate will cut the remaining space to one fourth
    /// of it's current size.
    expected_information: f64,
}

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
    /// Applying information theory, we try to guess the word. Guessing is a two-step procedure:
    /// First, we try to limit our space of remaining words to only those that could be possible
    /// given the last mask. Then, we loop over the remaining words to figure out which provides
    /// the largest information, and return that.
    fn guess(&mut self, past_guesses: &[Guess]) -> &'static Word {
        if let Some(last) = past_guesses.last() {
            // We retain words in `remaining` that are guessable after the last word we guessed.
            // Since this process happens once per guess, we don't need to iterate over al past
            // guesses, as those have been filtered out when those past guesses were made.
            self.remaining.retain(|word, _| {
                let mask = Correctness::check(word, last.word);
                for i in 0..WORD_SIZE {
                    if mask[i] != last.mask[i] {
                        return false;
                    }
                }

                return true;
            });
        }

        let mut best: Option<Candidate> = None;

        // We loop over every remaining guess, borrowing words and counts:
        let total_occurrence_count: usize = self.remaining.values().sum();
        let current_event_space_size = self.remaining.len();

        for (&word, &occurrence_count) in &self.remaining {
            // We need to find all the masks that can result from using this word, calculate
            // the probability of each as the amount of words in the remaining dictionary that
            // satisfy this mask, take the negative log (the information of the mask), then
            // calculate the expected value across all masks to get a measure of the quality of
            // the word.
            let masks_with_counts = self
                .remaining
                .keys()
                .map(|future_guess| Correctness::check(word, &future_guess))
                .fold(
                    HashMap::new(),
                    |mut histogram: HashMap<[Correctness; 5], usize>, mask| {
                        let counter = histogram.entry(mask).or_insert(0);
                        *counter += 1;
                        histogram
                    },
                );

            let expected_information = masks_with_counts
                .values()
                .map(|&count| {
                    let p = count as f64 / current_event_space_size as f64;
                    -p * f64::log2(p)
                })
                .sum::<f64>();

            // A new guess is better if no guess was previously made, or if the new guess has more
            // information, or has the same exact information but is more common.
            if best.is_none()
                || expected_information > best.unwrap().expected_information
                || (expected_information == best.unwrap().expected_information
                    && occurrence_count > best.unwrap().occurrence_count)
            {
                best = Some(Candidate {
                    word,
                    occurrence_count,
                    expected_information,
                });
            }
        }

        let best = best.expect("Our guesser has to find at least one word");

        best.word
    }
}

#[cfg(test)]
mod tests {
    mod play_wordle {
        use crate::Unoptimized;
        use roget::{DictionaryWithCounts, RepresentableAsWord, Word, Wordle};

        const DICTIONARY: &'static str = include_str!("../../dictionary.txt");

        const DICTIONARY_WITH_COUNTS: &'static str = include_str!("../../joined.txt");

        #[test]
        fn unoptimized_tries_highest_information_words() {
            let wordle: Wordle = Wordle::new(
                DICTIONARY
                    .split_ascii_whitespace()
                    .map(|word_str| word_str.as_word()),
            );

            let mut dictionary_with_counts_iter = DICTIONARY_WITH_COUNTS.split_ascii_whitespace();
            let mut dictionary_with_counts: Vec<(&'static Word, usize)> = Vec::new();

            while let Some(word) = dictionary_with_counts_iter.next() {
                let count = dictionary_with_counts_iter.next().unwrap().parse().unwrap();
                dictionary_with_counts.push((word.as_word(), count))
            }

            assert_eq!(
                wordle.play(
                    b"moved",
                    Unoptimized {
                        dictionary: wordle.get_dictionary(),
                        remaining: DictionaryWithCounts::from_iter(
                            dictionary_with_counts.into_iter()
                        )
                    }
                ),
                Some(1)
            );
        }
    }
}
