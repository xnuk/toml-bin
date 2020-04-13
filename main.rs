use std::io::{Read, Write, stdin, stdout};
use std::process::exit;
use std::env::args_os;
use std::fs::File;
use core::fmt::Display;

fn unwrap<'a, T, E>(x: &'a Result<T, E>) -> &'a T where E: Display {
	x.as_ref().unwrap_or_else(|e| {
		eprintln!("{}", e);
		exit(1);
	})
}

fn main() -> std::io::Result<()> {
	let path = args_os().nth(1);

	let mut buf = String::new();

	match path {
		Some(p) => {
			let a = File::open(p);
			unwrap(&a).read_to_string(&mut buf)?
		},
		None => stdin().read_to_string(&mut buf)?,
	};

	let is_json = buf.trim_start().chars().next() == Some('{');

	let output = if is_json {
		let a = serde_json::de::from_str::<toml::value::Value>(&buf);
		toml::ser::to_string_pretty(unwrap(&a)).unwrap()
	} else {
		let a = toml::de::from_str::<serde_json::value::Value>(&buf);
		serde_json::ser::to_string_pretty(unwrap(&a)).unwrap()
	};

	stdout().write_all(output.as_bytes())
}
