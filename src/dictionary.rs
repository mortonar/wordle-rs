use curl::easy::Easy;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_json::Value;
use serde_json::Value::Array;

#[derive(Debug)]
pub struct Dictionary {
    words: Vec<String>,
}

impl Dictionary {
    // TODO add error handling when building the dictionary from the REST API fails
    pub fn initialize() -> Self {
        Dictionary {
            words: fetch_words(),
        }
    }

    pub fn fetch_random_word(&self) -> &str {
        let mut rng = thread_rng();
        self.words.choose(&mut rng).unwrap()
    }

    pub fn is_valid_word(&self, word: &str) -> bool {
        self.words.binary_search(&word.to_owned()).is_ok()
    }
}

fn fetch_words() -> Vec<String> {
    let mut return_val = Vec::new();
    let mut dst = Vec::new();
    {
        let mut easy_curl = Easy::new();
        easy_curl
            .url("https://random-word-api.herokuapp.com/all")
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
        for w in inner_vec {
            if let Value::String(word) = w {
                if word.len() == 5 {
                    return_val.push(word);
                }
            } else {
                panic!("Didn't get JSON ['word'] back from random-word-api")
            }
        }
    } else {
        panic!("Didn't get JSON ['word'] back from random-word-api")
    }
    return_val.sort();
    return_val
}
