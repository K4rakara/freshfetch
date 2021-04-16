#[macro_use]
pub(crate) extern crate lazy_static;
pub(crate) extern crate chrono;
pub(crate) extern crate clap;
pub(crate) extern crate cmd_lib;
pub(crate) extern crate mlua;
pub(crate) extern crate regex;
pub(crate) extern crate sysinfo;
pub(crate) extern crate term_size;
pub(crate) extern crate uname;
pub(crate) extern crate users;

pub(crate) mod art;
pub(crate) mod assets;
pub(crate) mod errors;
pub(crate) mod info;
pub(crate) mod layout;
pub(crate) mod misc;
pub(crate) mod utils;

use clap::{App, Arg};
use mlua::prelude::*;

use assets::defaults::LAYOUT;
use assets::{ANSI, HELP, PRINT};
use layout::Layout;

use std::env::var;
use std::fs::read_to_string;
use std::path::Path;

pub(crate) struct Arguments {
	pub ascii_distro: Option<String>,
}

pub(crate) trait Inject {
	fn prep(&mut self) {}
	fn inject(&self, _lua: &mut Lua) {}
}

fn main() {
	let app = App::new("freshfetch")
		.version("0.0.1")
		.author("Jack Johannesen")
		.about("A fresh take on neofetch.")
		.help(HELP)
		.arg(
			Arg::with_name("ascii_distro")
				.long("ascii_distro")
				.short("a")
				.takes_value(true)
				.value_name("ASCII_DISTRO"),
		)
		.arg(
			Arg::with_name("logo")
				.long("logo")
				.short("l")
				.takes_value(false),
		);

	let matches = app.get_matches();

	let args = Arguments {
		ascii_distro: match matches.value_of("ascii_distro") {
			Some(v) => Some(String::from(v)),
			None => None,
		},
	};

	let mut ctx = Lua::new();
	match ctx.load(PRINT).exec() {
		Ok(_) => (),
		Err(e) => {
			errors::handle(&format!("{}{}", errors::LUA, e));
			panic!();
		}
	}
	match ctx.load(ANSI).exec() {
		Ok(_) => (),
		Err(e) => {
			errors::handle(&format!("{}{}", errors::LUA, e));
			panic!();
		}
	}

	let mut layout = Layout::new(&args);
	layout.prep();
	layout.inject(&mut ctx);

	let layout_file = Path::new("/home/")
		.join(var("USER").unwrap_or(String::new()))
		.join(".config/freshfetch/layout.lua");

	if layout_file.exists() {
		match read_to_string(&layout_file) {
			Ok(v) => {
				match ctx.load(&v).exec() {
					Ok(_) => (),
					Err(e) => {
						errors::handle(&format!("{}{}", errors::LUA, e));
						panic!();
					}
				}
				match ctx.globals().get::<&str, String>("__freshfetch__") {
					Ok(v) => print!("{}", v),
					Err(e) => {
						errors::handle(&format!("{}{}", errors::LUA, e));
						panic!();
					}
				}
			}
			Err(e) => {
				errors::handle(&format!(
					"{}{file}{}{err}",
					errors::io::READ.0,
					errors::io::READ.1,
					file = layout_file.to_string_lossy(),
					err = e
				));
				panic!();
			}
		}
	} else {
		match ctx.load(LAYOUT).exec() {
			Ok(_) => (),
			Err(e) => {
				errors::handle(&format!("{}{}", errors::LUA, e));
				panic!();
			}
		}
		match ctx.globals().get::<&str, String>("__freshfetch__") {
			Ok(v) => print!("{}", v),
			Err(e) => {
				errors::handle(&format!("{}{}", errors::LUA, e));
				panic!();
			}
		}
	}
}
