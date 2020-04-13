use toml::de::from_str;
use serde_json::value::Value;
use serde_json::ser::to_string_pretty;
use std::io::{Read, Write, stdin, stdout};
use std::process::exit;

fn main() {
	let json = to_string_pretty(&from_str::<Value>(&{
		let mut buf = String::new();
		stdin().read_to_string(&mut buf).unwrap();
		buf
	}).unwrap_or_else(|e| {
		eprintln!("{}", e);
		exit(1);
	})).unwrap();

	stdout().write_all(json.as_bytes()).unwrap();
}
