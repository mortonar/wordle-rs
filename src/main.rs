use clap::Parser;
use std::io;
use wordle_rs::guess_reader::{Error, GuessReader};
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
    // TODO read one char at a time and disallow non english alphabet characters
    let mut guess_reader = GuessReader::new(Box::new(io::stdin().lock()));
    let mut guess_writer = ConsoleGuessWriter::new();
    println!("Building dictionary...");
    let dictionary = dictionary::Dictionary::initialize();
    let word = dictionary.fetch_random_word();
    let mut game_state = game_state::GameState::new(word.to_owned());
    let cli = Cli::parse();
    if cli.debug {
        println!("DEBUG: word: {}", &word);
    }
    println!("Enter a 5-letter word: ");
    loop {
        match guess_reader.get_guess() {
            Ok(guess) => {
                if dictionary.is_valid_word(guess.trim()) {
                    let guess = game_state.evaluate(guess.trim());
                    guess_writer.write(&guess);
                    if guess.is_terminal() {
                        break;
                    }
                } else {
                    println!("{} is not a valid word", &guess);
                }
            }
            // TODO Print these in red / add to the guess writer to print these?
            Err(e) => match e {
                Error::IOErrror(ioe) => {
                    println!("Error entering word: {}", ioe)
                }
                Error::InvalidSize => {
                    println!("Must enter a 5-letter word")
                }
                Error::InvalidCharacters => {
                    println!("Word has invalid characters.")
                }
            },
        }
    }
}
