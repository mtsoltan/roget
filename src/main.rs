extern crate roget;

mod algorithms;
use roget::RepresentableAsWord;

const GAMES: &'static str = include_str!("../answers.txt");
const GAMES_LENGTH: usize = 2309;

fn main() {
    let mut guesser = algorithms::Naive {};
    let mut answers = [None; GAMES_LENGTH];
    for (i, answer) in GAMES.split_whitespace().enumerate() {
        answers[i] = roget::play(&answer.as_word(), &mut guesser);
    }

    println!("{:?}", answers);

    ()
}
