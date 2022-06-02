use std::io::BufRead;
use std::process::exit;

pub struct GuessReader {
    input: Box<dyn BufRead>,
}

impl GuessReader {
    pub fn new(input: Box<dyn BufRead>) -> Self {
        GuessReader { input }
    }

    pub fn get_guess(&mut self) -> String {
        let mut in_buffer = String::new();
        if let Err(error) = self.input.read_line(&mut in_buffer) {
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

        in_buffer
    }
}
