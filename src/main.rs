use std::io::{self, Write};
use std::ops::Index;
use std::process::exit;
use clap::Parser;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// print debugging info (spoils the answer!)
    #[clap(short, long)]
    debug: bool
}

fn main() {
    println!("Enter a 5-letter word: ");
    // TODO read one char at a time and disallow non english alphabet characters
    let stdin = io::stdin();
    // TODO replace with random word from dictionary
    let word = "wordl";
    let cli = Cli::parse();
    if cli.debug {
        println!("DEBUG: word: {}", word);
    }
    // TODO display word at the beginning if debug flag is set
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for n in (0..6).rev() {
        let mut in_buffer = String::new();
        if let Err(error) = stdin.read_line(&mut in_buffer) {
            eprintln!("error: {}", error);
            exit(1);
        }
        // TODO don't count these errors towards the tries
        // +1 for newline character
        if in_buffer.len() != 6 {
            eprintln!("error: must input a 5-letter word: {}", &in_buffer);
            exit(2);
        }
        if in_buffer.chars().take(5).any(|c| !c.is_alphabetic()) {
            eprintln!("error: mut input an 5-letter word: {}", &in_buffer);
            exit(3);
        }

        if in_buffer.trim().eq(word) {
            println!("correct!");
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            writeln!(&mut stdout, "{}", &word);
            exit(0);
        } else {
            println!("incorrect! {} tries remaining", n);
            for (idx, c) in in_buffer.chars().enumerate() {
                let color = match word.find(c) {
                    None => Color::Red,
                    Some(p) => {
                        if p == idx {
                            Color::Green
                        } else {
                            Color::Yellow
                        }
                    }
                };
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(color)))
                    .unwrap();
                write!(&mut stdout, "{}", &c);
            }
            writeln!(&mut stdout, "");
        }
    }
}
