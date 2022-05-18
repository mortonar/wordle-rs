use clap::Parser;
use curl::easy::Easy;
use serde_json::Value::{self, Array};
use std::io::{self, Write};
use std::process::exit;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    let stdin = io::stdin();
    // TODO replace with random word from dictionary
    let word = fetch_word();
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

        // TODO make sure the user enters a valid word, don't let it through if it's invalid
        if in_buffer.trim().eq(&word) {
            println!("correct!");
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            writeln!(&mut stdout, "{}", &word).unwrap();
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
                write!(&mut stdout, "{}", &c).unwrap();
            }
            writeln!(&mut stdout).unwrap();
        }
    }
}

fn fetch_word() -> String {
    let mut dst = Vec::new();
    {
        let mut easy_curl = Easy::new();
        easy_curl
            .url("https://random-word-api.herokuapp.com/word?length=5")
            .unwrap();
        let mut transfer = easy_curl.transfer();
        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    let content = std::str::from_utf8(&dst).unwrap();
    let parsed: Value = serde_json::from_str(content).unwrap();
    if let Array(inner_vec) = parsed {
        if let Some(Value::String(word)) = inner_vec.first() {
            word.clone()
        } else {
            panic!("Didn't get JSON ['word'] back from random-word-api")
        }
    } else {
        panic!("Didn't get JSON ['word'] back from random-word-api")
    }
}
