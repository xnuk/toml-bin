use core::fmt::Display;
use std::env::args_os;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::exit;

fn unwrap<T, E>(x: &Result<T, E>) -> &T
where
	E: Display,
{
	x.as_ref().unwrap_or_else(|e| {
		eprintln!("{}", e);
		exit(1);
	})
}

macro_rules! pretty {
	($from:tt -> $to:tt , $x:expr) => {
		$to::ser::to_string_pretty(unwrap(&{
			$from::de::from_str::<$to::value::Value>($x)
		}))
		.expect("")
	};
}

fn run() -> io::Result<()> {
	let mut buf = String::new();

	let path = args_os().nth(1).filter(|path| path != "-");

	match path {
		Some(path) => File::open(path)?.read_to_string(&mut buf),
		None => io::stdin().read_to_string(&mut buf),
	}?;

	let buf = buf.trim_start();
	let is_json = buf.starts_with('{');

	io::stdout().write_all(
		if is_json {
			pretty!(serde_json -> toml, &buf)
		} else {
			pretty!(toml -> serde_json, &buf)
		}
		.as_bytes(),
	)
}

fn main() {
	*unwrap(&run())
}
