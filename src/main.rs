use clap::Parser;
use std::io::{self, Write};
use std::process::exit;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use wordle_rs::game_state::{Guess, LetterPosition};
use wordle_rs::guess_reader::GuessReader;
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
    let word = dictionary::fetch_random_word();
    let mut game_state = game_state::GameState::new(word.clone());
    let cli = Cli::parse();
    if cli.debug {
        println!("DEBUG: word: {}", &word);
    }
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    loop {
        let guess = guess_reader.get_guess();
        match game_state.evaluate(guess.trim()) {
            Guess::Correct => {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                    .unwrap();
                println!("correct!");
                writeln!(&mut stdout, "{}", &word).unwrap();
                exit(0);
            }
            Guess::Incorrect(attempts_left, position_status) => {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                    .unwrap();
                println!("incorrect! {} tries remaining", attempts_left);
                let mut chars = guess.chars();
                for ps in position_status {
                    let color = match ps {
                        LetterPosition::Correct => Color::Green,
                        LetterPosition::Incorrect => Color::Yellow,
                        LetterPosition::NotFound => Color::Red,
                    };
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(color)))
                        .unwrap();
                    write!(&mut stdout, "{}", chars.next().unwrap()).unwrap();
                }
                writeln!(&mut stdout).unwrap();
            }
            Guess::InvalidWord => {}
            Guess::GameOver => {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                write!(&mut stdout, "Game Over").unwrap();
                break;
            }
        }
    }
}
