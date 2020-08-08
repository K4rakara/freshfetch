extern crate clml_rs;

use std::fs;
use std::path::{ Path, PathBuf };

use clml_rs::{ clml };

type BuildList = Vec<(PathBuf, PathBuf)>;

fn get_buildlist(base: &Path) -> BuildList {
	let mut to_return: BuildList = Vec::new();
	let buildlist_string = fs::read_to_string(base.join("buildlist")).expect("Failed to read buildlist file!");
	let buildlist_lines = {
		let split: Vec<&str> = buildlist_string.split("\n").collect();
		let mut to_return = Vec::new();
		for line in split.iter() {
			if !line.starts_with("#") {
				to_return.push(line.clone());
			}
		}
		to_return
	};
	for line in buildlist_lines.iter() {
		let split: Vec<&str> = line.split(" -> ").collect();
		if split.len() != 2 { panic!("Expected only one ... -> ... statement per line!"); }
		to_return.push((base.join(split[0]), base.join(split[1])));
	}
	to_return
}

fn main() {
	let base = Path::new("./src/assets/ascii_art/");
	let buildlist = get_buildlist(&base);
	for target in buildlist {
		let input = fs::read_to_string(&target.0).expect(&format!("Failed to read the file \"{:?}\"!", &target.0));
		let output = clml(&input);
		fs::write(&target.1, &output).expect(&format!("Failed to write to the file \"{:?}\"!", &target.1));
	}
}
