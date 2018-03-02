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

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let content = read_file(filename);

	println!("{}",content);
}