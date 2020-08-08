use crate::clml_rs;
use crate::mlua;
use crate::term_size;

use clml_rs::{ CLML };
use mlua::prelude::*;

use crate::Inject;

pub(crate) struct Terminal {
	pub width: i32,
	pub height: i32,
}

impl Terminal {
	pub fn new() -> Self {
		let (w, h) = term_size::dimensions().expect("Failed to get terminal dimensions.");
		Terminal {
			width: w as i32,
			height: h as i32,
		}
	}
}

impl Inject for Terminal {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject env values.
		clml
			.env("terminal.width", &format!("{}", self.width))
			.env("terminal.height", &format!("{}", self.height));

		// Inject bash values.
		clml
			.bash_env("terminal_width", &format!("{}", self.width))
			.bash_env("terminal_height", &format!("{}", self.height));

		// Inject Lua values.
		{
			let lua = &clml.lua_env;
			let globals = lua.globals();
		
			match lua.create_table() {
				Ok(t) => {
					match t.set("width", self.width) {
						Ok(_) => (),
						Err(e) => panic!(format!("The following Lua error occured: {}", e)),
					}
					match t.set("height", self.height) {
						Ok(_) => (),
						Err(e) => panic!(format!("The following Lua error occured: {}", e)),
					}
					match globals.set("terminal", t) {
						Ok(_) => (),
						Err(e) => panic!(format!("The following Lua error occured: {}", e)),
					}
				}
				Err(e) => panic!(format!("The following Lua error occured: {}", e)),
			}
		}

		Ok(())
	}
}

