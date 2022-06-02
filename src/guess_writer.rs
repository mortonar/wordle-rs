use crate::game_state::{Guess, LetterPosition};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub trait GuessWriter {
    fn write(&mut self, guess: &Guess);
}

pub struct ConsoleGuessWriter {
    stdout: StandardStream,
}

impl ConsoleGuessWriter {
    pub fn new() -> Self {
        ConsoleGuessWriter {
            stdout: StandardStream::stdout(ColorChoice::Always),
        }
    }
}

impl GuessWriter for ConsoleGuessWriter {
    fn write(&mut self, guess: &Guess) {
        match guess {
            // TODO Format this into a generic guess writer and implementing console-based guess writer.
            //   What if we wanted to have a GUI or network-based guess writer?
            Guess::Correct => {
                self.stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                    .unwrap();
                println!("correct!");
            }
            Guess::Incorrect(attempts_left, position_status, guessed_word) => {
                self.stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                    .unwrap();
                println!("incorrect! {} tries remaining", attempts_left);
                let mut chars = guessed_word.chars();
                for ps in position_status {
                    let color = match ps {
                        LetterPosition::Correct => Color::Green,
                        LetterPosition::Incorrect => Color::Yellow,
                        LetterPosition::NotFound => Color::Red,
                    };
                    self.stdout
                        .set_color(ColorSpec::new().set_fg(Some(color)))
                        .unwrap();
                    write!(&mut self.stdout, "{}", chars.next().unwrap()).unwrap();
                }
                writeln!(&mut self.stdout).unwrap();
            }
            Guess::InvalidWord => {}
            Guess::GameOver => {
                self.stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                write!(&mut self.stdout, "Game Over").unwrap();
            }
        }
    }
}
