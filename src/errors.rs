use crate::clml_rs;

use std::process::{ exit };

use clml_rs::{ clml };

pub(crate) static LUA: &'static str = "A Lua error occurred. Details:\n";

pub(crate) mod io {
	pub(crate) static READ: (&'static str, &'static str) =
		( "An I/O error occurred while trying to read from \"", "\". Details:\n" );
	pub(crate) static WRITE: (&'static str, &'static str) =
		( "An I/O error occurred while trying to write to \"", "\". Details:\n" );
}

pub(crate) fn handle(msg: &str) {
	println!("{header}{body}",
		header = clml("<red>Error.<reset>\n"),
		body = msg);
	exit(1);
}
