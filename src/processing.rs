use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Record {
    pub text: String,
    pub spans: Vec<Range<usize>>,
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
}
