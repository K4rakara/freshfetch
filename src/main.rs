pub(crate) extern crate clml_rs;

pub(crate) mod assets;

use clml_rs::clml;

fn main() {
    println!("{}", clml(assets::ascii_art::get_ascii_art("tux_256")));
}
