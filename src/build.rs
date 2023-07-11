#![allow(dead_code)]

extern crate clml_rs;

//mod assets;

use std::fs;
use std::path::{Path, PathBuf};

use clml_rs::clml;

//use assets::ascii_art::{ ASCII_ART };

type BuildList = Vec<(PathBuf, PathBuf)>;

fn get_buildlist(base: &Path) -> BuildList {
    let mut to_return: BuildList = Vec::new();
    let buildlist_string =
        fs::read_to_string(base.join("buildlist")).expect("Failed to read buildlist file!");
    let buildlist_lines = {
        let split: Vec<&str> = buildlist_string.split('\n').collect();
        let mut to_return = Vec::new();
        for line in split.iter() {
            if !line.starts_with('#') {
                to_return.push(line.to_owned());
            }
        }
        to_return
    };
    for line in buildlist_lines.iter() {
        let split: Vec<&str> = line.split(" -> ").collect();
        if split.len() != 2 {
            panic!("Expected only one ... -> ... statement per line!");
        }
        to_return.push((base.join(split[0]), base.join(split[1])));
    }
    to_return
}

fn progress(min: usize, max: usize) -> String {
    let mut bar = String::from(" [");
    let complete = ((min as f32 / max as f32) * 57.0).floor() as usize;
    if complete >= 1 {
        bar = format!("{}{}", bar, String::from("=").repeat(complete - 1));
    }
    bar = format!("{}>", bar);
    let remaining = 57 - complete;
    if complete >= 1 {
        bar = format!("{}{}]", bar, String::from(" ").repeat(remaining));
    } else {
        bar = format!("{}{}]", bar, String::from(" ").repeat(remaining - 1));
    }
    format!("{bar} {min:03}/{max:03}: ", bar = bar, min = min, max = max)
}

fn main() {
    let base = Path::new("./src/assets/ascii_art/");
    let buildlist = get_buildlist(base);
    let len = buildlist.len();
    for (i, target) in buildlist.iter().enumerate() {
        println!(
            "\u{001b}[1A\r\u{001b}[K    \u{001b}[1m\u{001b}[36mBuilding\u{001b}[0m{}ASCII art",
            progress(i, len)
        );
        let input = fs::read_to_string(&target.0)
            .unwrap_or_else(|_| panic!("Failed to read the file \"{:?}\"!", &target.0));
        let output = clml(&input);
        fs::write(&target.1, &output)
            .unwrap_or_else(|_| panic!("Failed to write to the file \"{:?}\"!", &target.1));
    }
    println!("\u{001b}[1A\r\u{001b}[K    \u{001b}[1m\u{001b}[32mFinished\u{001b}[0m ASCII art");
    {
        let input = fs::read_to_string("./src/assets/help.clml")
            .expect("Failed to read the file \"./src/assets/help.clml\"!");
        let /*mut*/ output = clml(&input);
        /*// This is removed because it causes a compile issue-- Once 0.2.0 is released
          // this will be fixed, because I'll be reworking how the ASCII art is compiled
          // into the final build.
         let ascii_distro_list = {
        let mut output = clml(&input);
        let ascii_distro_list = {
            let mut to_use = Vec::new();
            for art in ASCII_ART.iter() {
                if art.0 != "" {
                    to_use.push(String::from(art.0));
                }
            }
            let mut to_return = String::new();
            let mut this_line = String::new();
            for (i, name) in to_use.iter().enumerate() {
                this_line += name;
                if i != to_use.len() {
                    this_line += ", "
                }
                if this_line.len() >= 64 {
                    to_return += &this_line;
                    to_return += "\n";
                    this_line = String::new();
                }
            }
            to_return += &this_line;
            to_return += "\n";
            to_return = to_return.replace("\n", &(String::from("\n") + &" ".repeat(4)));
            to_return
        };
        output = output.replace("ASCII_DISTRO_LIST", &ascii_distro_list);*/
        fs::write("./src/assets/.help.clml", output)
            .expect("Failed to write to the file \"./src/assets/.help.clml\"!");
    }
}
