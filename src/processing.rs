use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read};
use std::ops::Range;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Record {
    pub text: String,
    pub spans: Vec<Range<usize>>,
}

pub struct Records<R> {
    input: BufReader<R>,
}

impl<R: Read> Records<R> {
    pub fn new(input: R) -> Self {
        Records {
            input: BufReader::new(input),
        }
    }
}

impl<R: Read> Iterator for Records<R> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.input.read_line(&mut line) {
            Ok(0) => None,
            Err(e) => panic!("{:?}", e),
            Ok(_) => match serde_json::from_str(&line) {
                Ok(result) => Some(result),
                Err(e) => panic!("{:?}", e),
            },
        }
    }
}

mod tests {

    use super::*;
    use serde_json::json;

    #[test]
    fn serialize() {
        let r = Record {
            text: String::from("Hello"),
            spans: vec![],
        };
        let expected = r#"{"text":"Hello","spans":[]}"#;
        let value = serde_json::to_string(&r).unwrap();
        assert_eq!(value, expected);
    }

    #[test]
    fn deserialize() {
        let expected = Record {
            text: String::from("Русский текст"),
            spans: vec![(13..15)],
        };
        let json = json!({
            "text": "Русский текст",
            "spans": [[13, 15]]
        });
        let value: Record = serde_json::from_value(json).unwrap();
        assert_eq!(value, expected)
    }

    #[test]
    fn iterate() {
        let string = r#"{"text": "Hello", "spans": []}"#;
        let mut records = Records::new(string.as_bytes());

        assert_eq!(records.next().map(|i| i.text), Some(String::from("Hello")));
        assert_eq!(records.next(), None);
    }
}
