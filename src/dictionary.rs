use curl::easy::Easy;
use serde_json::Value;
use serde_json::Value::Array;

pub fn fetch_random_word() -> String {
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
