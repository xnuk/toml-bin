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

macro_rules! pretty {
	($from:tt -> $to:tt , $x:expr) => {
		$to::ser::to_string_pretty(
			unwrap(&{
				$from::de::from_str::<$to::value::Value>($x)
			})
		).expect("")
	}
}

fn run() -> std::io::Result<()> {
	let mut buf = String::new();

	let path = args_os().nth(1).filter(|path| { path != "-" });

	match path {
		Some(path) => {
			File::open(path)?.read_to_string(&mut buf)
		},
		None => stdin().read_to_string(&mut buf),
	}?;

	let buf = buf.trim_start();
	let is_json = buf.chars().next() == Some('{');

	stdout().write_all(
		if is_json {
			pretty!(serde_json -> toml, &buf)
		} else {
			pretty!(toml -> serde_json, &buf)
		}.as_bytes()
	)
}

fn main() { *unwrap(&run()) }
