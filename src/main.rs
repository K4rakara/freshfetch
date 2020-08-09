pub(crate) extern crate clml_rs;
pub(crate) extern crate cmd_lib;
pub(crate) extern crate chrono;
pub(crate) extern crate cpuid;
pub(crate) extern crate mlua;
pub(crate) extern crate term_size;
pub(crate) extern crate uname;
pub(crate) extern crate users;
pub(crate) extern crate regex;
pub(crate) extern crate clap;

pub(crate) mod art;
pub(crate) mod assets;
pub(crate) mod info;
pub(crate) mod layout;
pub(crate) mod misc;

use clap::{ App, Arg };
use clml_rs::{ clml, CLML };

use layout::{ Layout };

pub(crate) struct Arguments {
	pub ascii_distro: Option<String>,
}

pub(crate) trait Inject {
	fn prep(&mut self) -> Result<(), ()> { Ok(()) }
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> { Ok(()) }
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

	let mut ctx = CLML::new();
	let mut layout = Layout::new(&args);
	layout.prep();
	layout.inject(&mut ctx);
	print!("{}", ctx.parse(include_str!("./assets/defaults/layout.clml")).unwrap());
}
