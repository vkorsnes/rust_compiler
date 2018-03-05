use std::error::Error;
use std::fs::File;
use std::env;
use std::io;
use std::io::prelude::*;

fn read_file(target: &str) -> String {
	let mut f = File::open(target).expect("File not found.");

	let mut content = String::new();

	f.read_to_string(&mut content)
		.expect("Failed to read file.");

	content
}

fn parse_command_line_arguments() -> Result<Vec<String>, ()> {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		Err(())
	}
	else {
		Ok(args)
	}
}

fn main() {

	let args = match parse_command_line_arguments() {
		Ok(arg) => arg,
		Err(()) => {
			println!("Usage: simple_compiler FILENAME FLAG");
			return
		}
	};

	let filename = &args[1];
	let content = read_file(filename);

	println!("{}",content);
}