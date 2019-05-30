use std::env;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::path::Path;

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("words.rs");
	let mut f = File::create(&dest_path).unwrap();

	let f_in = File::open("top-10000-words.txt").unwrap();
	let r = BufReader::new(f_in);

	f.write(b"\nstatic WORDS: &'static [&'static str] = &[\n").unwrap();
	
	for line in r.lines() {
		write!(&f, " 	\"{}\",\n", line.unwrap()).unwrap();
	}

	f.write(b"];").unwrap();
}