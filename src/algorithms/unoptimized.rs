use roget::{Correctness, Dictionary, DictionaryWithCounts, Guess, Guesser, Word, WORD_SIZE};

#[derive(Debug, Copy, Clone)]
struct Candidate {
    /// The word of this candidate.
    word: &'static Word,

    /// The count coming form the DictionaryWithCounts value parameter. This lets us know how
    /// frequent this word is in the English language.
    count: usize,

    /// How much this candidate will reduce the space of possible states.
    /// Information of 2 bits means that the candidate will cut the remaining space to one fourth
    /// of it's current size.
    information: f64,
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
            // We retain words in `remaining` that, if we check against them with this specific
            // last guess, will return the mask we encountered.
            self.remaining.retain(|word, _count| {
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
        let total_count: usize = self.remaining.values().sum();

        for (&word, &count) in &self.remaining {
            // todo!("This just gets the rarest word. Actually calculate information correctly");
            let information = -f64::log2(count as f64 / total_count as f64);
            // A new guess is better if it has more information.
            if best.is_none() || information > best.unwrap().information {
                best = Some(Candidate {
                    word,
                    count,
                    information,
                })
            }
        }

        assert!(best.is_some(), "Our guesser has to find at least one word");

        let best = best.unwrap();

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
