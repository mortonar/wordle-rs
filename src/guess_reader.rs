use std::io;
use std::io::BufRead;

pub struct GuessReader {
    input: Box<dyn BufRead>,
}

#[derive(Debug)]
pub enum Error {
    IOErrror(io::Error),
    InvalidSize,
    InvalidCharacters,
}

impl GuessReader {
    pub fn new(input: Box<dyn BufRead>) -> Self {
        GuessReader { input }
    }

    pub fn get_guess(&mut self) -> Result<String, Error> {
        let mut in_buffer = String::new();
        if let Err(error) = self.input.read_line(&mut in_buffer) {
            return Err(Error::IOErrror(error));
        }
        // +1 for newline character
        if in_buffer.len() != 6 {
            return Err(Error::InvalidSize);
        }
        if in_buffer.chars().take(5).any(|c| !c.is_alphabetic()) {
            return Err(Error::InvalidCharacters);
        }

        Ok(in_buffer)
    }
}
