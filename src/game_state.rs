use crate::game_state::LetterPosition::{Correct, Incorrect, NotFound};
use std::collections::HashMap;

pub struct GameState {
    guesses_made: u8,
    total_guesses: u8,
    word_to_guess: String,
    processed_word: HashMap<char, Vec<usize>>,
}

impl GameState {
    pub fn new(word_to_guess: String) -> Self {
        let mut processed_word = HashMap::new();
        for (idx, c) in word_to_guess.chars().enumerate() {
            if processed_word.get_mut(&c).is_none() {
                processed_word.insert(c, Vec::with_capacity(6));
            }
            let vec = processed_word.get_mut(&c).unwrap();
            vec.push(idx);
        }

        GameState {
            guesses_made: 0,
            total_guesses: 6,
            word_to_guess,
            processed_word,
        }
    }

    pub fn evaluate(&mut self, guess: &str) -> Guess {
        if guess.eq(&self.word_to_guess) {
            Guess::Correct
        } else {
            // TODO check if valid word

            self.guesses_made += 1;
            if self.guesses_made == self.total_guesses {
                return Guess::GameOver;
            }

            // map: char => [0, 1, 2, 3]
            let pos_array: [LetterPosition; 5] = guess
                .chars()
                .enumerate()
                .map(|(idx, c)| match self.processed_word.get(&c) {
                    None => NotFound,
                    Some(vec) => {
                        if vec.contains(&idx) {
                            Correct
                        } else {
                            Incorrect
                        }
                    }
                })
                .collect::<Vec<LetterPosition>>()
                .try_into()
                .unwrap();
            Guess::Incorrect(self.total_guesses - self.guesses_made, pos_array)
        }
    }
}

#[derive(Debug)]
pub enum Guess {
    Correct,
    Incorrect(u8, [LetterPosition; 5]),
    InvalidWord,
    GameOver,
}

#[derive(Debug)]
pub enum LetterPosition {
    Correct,
    Incorrect,
    NotFound,
}
