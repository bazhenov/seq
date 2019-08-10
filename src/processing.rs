use std::ops::Range;
use serde_json::{json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Record {
	text: String,
	spans: Vec<Range<usize>>
}

mod tests {

	use super::*;

	#[test]
	fn serialize() {
		let r = Record { text: String::from("Hello"), spans: vec![] };
		let expected = json!({
			"text": "Hello",
			"spans": []
		});

		let value = serde_json::to_value(r).unwrap();
		assert_eq!(value, expected);
	}

	#[test]
	fn deserialize() {
		let expected = Record { text: String::from("Русский текст"), spans: vec![(13..15)] };
		let json = json!({
			"text": "Русский текст",
			"spans": [[13, 15]]
		});
		let value: Record = serde_json::from_value(json).unwrap();
		assert_eq!(value, expected)
	}
}
