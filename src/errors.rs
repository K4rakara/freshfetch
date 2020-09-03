use std::process::{ exit };

pub(crate) static LUA: &'static str = "A Lua error occurred. Details:\n";
pub(crate) static CMD: (&'static str, &'static str) =
	( "An error occurred while executing \"", "\". Details:\n" );
pub(crate) static PARSE: (&'static str, &'static str, &'static str) =
	( "An error occurred while parsing \"", "\" into a \"", "\". Details:\n" );

pub(crate) mod io {
	pub(crate) static READ: (&'static str, &'static str) =
		( "An I/O error occurred while trying to read from \"", "\". Details:\n" );
}

pub(crate) fn handle(msg: &str) {
	println!("{header}{body}",
		header = "\u{001b}[38;5;1mError.\u{001b}[0m\n",
		body = msg);
	exit(1);
}
