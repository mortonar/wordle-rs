use clap::Parser;
use std::io;
use wordle_rs::guess_reader::GuessReader;
use wordle_rs::guess_writer::{ConsoleGuessWriter, GuessWriter};
use wordle_rs::{dictionary, game_state};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// print debugging info (spoils the answer!)
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    println!("Enter a 5-letter word: ");
    // TODO read one char at a time and disallow non english alphabet characters
    let mut guess_reader = GuessReader::new(Box::new(io::stdin().lock()));
    let mut guess_writer = ConsoleGuessWriter::new();
    let word = dictionary::fetch_random_word();
    let mut game_state = game_state::GameState::new(word.clone());
    let cli = Cli::parse();
    if cli.debug {
        println!("DEBUG: word: {}", &word);
    }
    loop {
        let guess = guess_reader.get_guess();
        let guess = game_state.evaluate(guess.trim());
        guess_writer.write(&guess);
        if guess.is_terminal() {
            break;
        }
    }
}
