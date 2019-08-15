use regex::{Regex, Error};
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

    pub fn add_match_str(&mut self, re: &str) -> Result<usize, Error> {
        Ok(self.add_match(&Regex::new(&re)?))
    }

    /// Mask all spans in a text with given label.
    /// 
    /// For example:
    /// ```
    /// let r = Record::new("Hello world");
    /// r.add_match_str("world");
    /// assert_eq!(r.mask("<W>"), "Hello <W>");
    /// ```
    pub fn mask(&self, label: &str) -> String {
        let mut result = String::new();
        let mut prev_end = 0;

        for s in &self.spans {
            let span_start = byte_offset(&self.text, s.start);
            if span_start > prev_end {
                result.push_str(&self.text[prev_end..span_start]);
            }
            result.push_str(label);
            prev_end = byte_offset(&self.text, s.end);
        }

        if prev_end < self.text.len() - 1 {
            result.push_str(&self.text[prev_end..]);
        }

        result
    }
}

fn byte_offset(text: &str, char_no: usize) -> usize {
    // Looking for the nth character in a text
    if let Some((offset, _)) = text.char_indices().nth(char_no) {
        offset

    // if char_no equals to the length of the string return the byte after the last one
    } else if text.chars().count() == char_no {
        text.len()
    } else {
        panic!("unable to find offset for char no {no} in string '{text}'", no = char_no, text = text);
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

        assert_eq!(r.add_match_str("[0-9]+"), Ok(1));
        assert_eq!(r.spans[0], 9..10);
    }

    #[test]
    fn build_template() {
        let mut r = Record::new("2 cats have 2 tails 2");
        r.add_match(&Regex::new("[0-9]").unwrap());

        let result = r.mask("<DIGIT>");
        assert_eq!(result, "<DIGIT> cats have <DIGIT> tails <DIGIT>");
    }
}
