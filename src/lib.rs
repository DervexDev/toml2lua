//     __                  __ ___   __
//    / /_____  ____ ___  / /|__ \ / /_  __ _____
//   / __/ __ \/ __  __ \/ / __/ // / / / / __  /
//  / /_/ /_/ / / / / / / / / __// / /_/ / /_/ /
//  \__/\____/_/ /_/ /_/_/ /____/_/\____/\____/
//
//! # toml2lua
//!
//! Convert TOML to Lua table
//!
//! ## Example:
//! ```rust
//! use toml2lua::parse;
//!
//! let toml = r#"
//! string = "toml2lua",
//! int = 420,
//! bool = true,
//!
//! [object]
//! key = "value"
//! "#;
//!
//! let lua = parse(toml).unwrap();
//! // Output:
//! // {
//! //   ["string"] = "toml2lua",
//! //   ["int"] = 420,
//! //   ["bool"] = true,
//! //   ["object"] = {
//! //	    ["key"] = "value",
//! //   },
//! // }
//! ```
//!
//! Made with <3 by Dervex

#![allow(clippy::tabs_in_doc_comments)]

use indexmap::IndexMap;
use toml::{de::Error, from_str, Value};

/// Parse TOML string into a Lua table
///
/// ```rust
/// use toml2lua::parse;
///
/// let toml = r#"
/// string = "abc"
/// int = 123
/// bool = true
///
/// [object]
/// key = "value"
/// "#;
///
/// let lua = r#"{
/// 	["string"] = "abc",
/// 	["int"] = 123,
/// 	["bool"] = true,
/// 	["object"] = {
/// 		["key"] = "value",
/// 	},
/// }"#;
///
/// assert_eq!(parse(toml).unwrap(), lua);
/// ```
pub fn parse(toml: &str) -> Result<String, Error> {
	let toml: IndexMap<String, Value> = from_str(toml)?;
	let mut lua = String::from("{\n");

	for (key, value) in toml {
		lua.push_str(&walk(Some(&validate_string(&key)), &value, 1));
	}

	lua.push('}');

	Ok(lua)
}

fn walk(key: Option<&str>, value: &Value, depth: usize) -> String {
	let mut lua = String::new();

	lua.push_str(&get_indent(depth));

	if let Some(key) = key {
		lua.push_str(&format!("[\"{}\"] = ", validate_string(key)));
	}

	match value {
		Value::String(s) => lua.push_str(&format!("\"{}\"", &validate_string(s))),
		Value::Integer(i) => lua.push_str(&i.to_string()),
		Value::Float(f) => lua.push_str(&f.to_string()),
		Value::Boolean(b) => lua.push_str(&b.to_string()),
		Value::Datetime(d) => lua.push_str(&format!("\"{}\"", d)),
		Value::Array(a) => {
			lua.push_str("{\n");

			for v in a {
				lua.push_str(&walk(None, v, depth + 1));
			}

			lua.push_str(&get_indent(depth));
			lua.push('}');
		}
		Value::Table(t) => {
			lua.push_str("{\n");

			for (k, v) in t {
				lua.push_str(&walk(Some(k), v, depth + 1));
			}

			lua.push_str(&get_indent(depth));
			lua.push('}');
		}
	}

	lua.push_str(",\n");

	lua
}

fn get_indent(depth: usize) -> String {
	let mut indent = String::new();

	for _ in 0..depth {
		indent.push('\t');
	}

	indent
}

fn validate_string(string: &str) -> String {
	let mut validated = String::new();

	for char in string.chars() {
		match char {
			'\n' => validated.push_str("\\n"),
			'\t' => validated.push_str("\\t"),
			'\r' => validated.push_str("\\r"),
			'\\' => validated.push_str("\\\\"),
			'"' => validated.push_str("\\\""),
			_ => validated.push(char),
		}
	}

	validated
}

#[cfg(test)]
mod test {
	#[test]
	fn all_values() {
		use crate::parse;

		let toml = r#"
		string = "str"
		int = 420
		float = 6.9
		bool = true
		datetime = 1979-05-27T00:32:00.999999-07:00
		array = [
			"string",
			12345,
			false
		]

		[object]
		key = "value"
"#;

		let lua = r#"{
	["string"] = "str",
	["int"] = 420,
	["float"] = 6.9,
	["bool"] = true,
	["datetime"] = "1979-05-27T00:32:00.999999-07:00",
	["array"] = {
		"string",
		12345,
		false,
	},
	["object"] = {
		["key"] = "value",
	},
}"#;

		assert_eq!(parse(toml).unwrap(), lua);
	}

	#[test]
	fn malformed_strings() {
		use crate::parse;

		let toml = r#"
  1 = "..\n.."
  2 = "..\t.."
  3 = "..\r.."
  4 = "..\\.."
  5 = "..\".."
"#;

		let lua = r#"{
	["1"] = "..\n..",
	["2"] = "..\t..",
	["3"] = "..\r..",
	["4"] = "..\\..",
	["5"] = "..\"..",
}"#;

		assert_eq!(parse(toml).unwrap(), lua);
	}
}
