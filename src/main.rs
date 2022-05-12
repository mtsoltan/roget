extern crate roget;

mod algorithms;

use algorithms::Unoptimized;
use roget::{RepresentableAsWord, Wordle};
use std::collections::HashMap;
use std::time::Instant;

const GAMES: &'static str = include_str!("../answers.txt");
const DICTIONARY: &'static str = include_str!("../dictionary.txt");
const JOINED: &'static str = include_str!("../joined.txt");
const GAMES_LENGTH: usize = 2309;

fn main() {
    let wordle = Wordle::new(DICTIONARY.lines().map(|word_str| word_str.as_word()));

    let initial_remaining = HashMap::from_iter(JOINED.lines().map(|line| {
        let (word, count) = line
            .split_once(' ')
            .expect("Each line should have a word and a count");
        let word = word.as_word();
        let count = count.parse().expect("The count should be parse-able");
        (word, count)
    }));

    let mut guesses_required = [None; GAMES_LENGTH];

    let start = Instant::now();
    for (i, answer) in GAMES.lines().enumerate() {
        let guesser = Unoptimized::new(wordle.get_dictionary(), initial_remaining.clone());
        guesses_required[i] = wordle.play(&answer.as_word(), guesser);
    }
    let end = Instant::now();

    // println!("{:?}", guesses_required);

    println!(
        "Took {:?} for an average guess score of {}",
        end.duration_since(start),
        guesses_required
            .iter()
            .map(|item| item.unwrap())
            .sum::<usize>() as f64
            / guesses_required.len() as f64
    );

    ()
}
