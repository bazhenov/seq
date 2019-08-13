use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read};
use std::ops::Range;

type Span = Range<usize>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Record {
    pub text: String,
    pub spans: Vec<Span>,
}

impl Record {
    pub fn new(text: &str) -> Self {
        Record {
            text: String::from(text),
            spans: vec![],
        }
    }

    pub fn add_match(&mut self, re: &Regex) -> usize {
        let mut found_spans = re
            .find_iter(&self.text)
            .map(|m| char_span(&self.text, m.start()..m.end()))
            .collect::<Vec<_>>();

        let spans_found = found_spans.len();
        self.spans.append(&mut found_spans);

        spans_found
    }
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

/// Returns character span for a byte span and given string
///
/// This is convinient method for converting byte span (for example from a regex search) to a charatcter
/// span for a given string.
/// Input span should be correct byte offsets for a given string
fn char_span(string: &str, span: Span) -> Span {
    let (start, end) = (span.start, span.end);

    let char_start = string[0..start].chars().count();
    let char_end = char_start + string[start..end].chars().count();

    char_start..char_end
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

    #[test]
    fn mark_records() {
        let mut r = Record::new("Вот тебе 2 яблочка");
        let regex = Regex::new("[0-9]+").unwrap();

        assert_eq!(r.add_match(&regex), 1);
        assert_eq!(r.spans[0], 9..10);
    }
}
