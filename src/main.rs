extern crate roget;

mod unoptimized;

use roget::{RepresentableAsWord, Wordle};
use unoptimized::Unoptimized;

const GAMES: &'static str = include_str!("../answers.txt");
const DICTIONARY: &'static str = include_str!("../dictionary.txt");
const GAMES_LENGTH: usize = 2309;

fn main() {
    let wordle = Wordle::new(
        DICTIONARY
            .split_ascii_whitespace()
            .map(|word_str| word_str.as_word()),
    );

    let mut guesses_required = [None; GAMES_LENGTH];
    for (i, answer) in GAMES.split_ascii_whitespace().enumerate() {
        let guesser = Unoptimized::new(wordle.get_dictionary());
        guesses_required[i] = wordle.play(&answer.as_word(), guesser);
    }

    println!("{:?}", guesses_required);

    ()
}
