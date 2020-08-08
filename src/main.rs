pub(crate) extern crate clml_rs;
pub(crate) extern crate cmd_lib;
pub(crate) extern crate cpuid;
pub(crate) extern crate mlua;
pub(crate) extern crate term_size;
pub(crate) extern crate uname;

pub(crate) mod art;
pub(crate) mod assets;
pub(crate) mod info;
pub(crate) mod layout;
pub(crate) mod misc;

use clml_rs::{ clml, CLML };
use layout::{ Layout };

pub(crate) trait Inject {
	fn prep(&mut self) -> Result<(), ()> { Ok(()) }
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> { Ok(()) }
}

fn main() {
	let mut ctx = CLML::new();
	let mut layout = Layout::new();
	layout.prep();
	layout.inject(&mut ctx);
	print!("{}", ctx.parse(include_str!("./assets/defaults/layout.clml")).unwrap());
}
