#[macro_use] pub(crate) extern crate lazy_static;
pub(crate) extern crate chrono;
pub(crate) extern crate clap;
pub(crate) extern crate cmd_lib;
pub(crate) extern crate cpuid;
pub(crate) extern crate mlua;
pub(crate) extern crate regex;
pub(crate) extern crate sysinfo;
pub(crate) extern crate term_size;
pub(crate) extern crate uname;
pub(crate) extern crate users;
pub(crate) extern crate x11rb;

pub(crate) mod art;
pub(crate) mod assets;
pub(crate) mod errors;
pub(crate) mod info;
pub(crate) mod layout;
pub(crate) mod misc;

use mlua::prelude::*;
use clap::{ App, Arg };

use layout::{ Layout };
use assets::{ ANSI, PRINT };
use assets::defaults::{ LAYOUT };

use std::env::{ var };
use std::path::{ Path };

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
		.help("")
		.arg(Arg::with_name("ascii_distro")
			.long("ascii_distro")
			.short("a")
			.takes_value(true)
			.value_name("ASCII_DISTRO"));

	let matches = app.get_matches();

	let args = Arguments {
		ascii_distro: match matches.value_of("ascii_distro") {
			Some(v) => { Some(String::from(v)) }
			None => None,
		},
	};

	let mut ctx = Lua::new();
	match ctx.load(PRINT).exec() {
		Ok(_) => (),
		Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
	}
	match ctx.load(ANSI).exec() {
		Ok(_) => (),
		Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
	}

	let mut layout = Layout::new(&args);
	layout.prep();
	layout.inject(&mut ctx);

	let layout_file = Path::new("/home/")
		.join(var("USER").unwrap_or(String::new()))
		.join(".config/freshfetch/layout.lua");

	if layout_file.exists() {

	} else {
		match ctx.load(LAYOUT).exec() {
			Ok(_) => (),
			Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
		}
		match ctx.globals().get::<&str, String>("__freshfetch__") {
			Ok(v) => print!("{}", v),
			Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
		}
	}
}
