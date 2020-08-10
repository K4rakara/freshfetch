use crate::clml_rs;
use crate::term_size;

use crate::errors;

use clml_rs::{ CLML };

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
						Err(e) => errors::handle(&format!("{}{err}", errors::LUA, err =e)),
					}
					match t.set("height", self.height) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{err}", errors::LUA, err =e)),
					}
					match globals.set("terminal", t) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{err}", errors::LUA, err =e)),
					}
				}
				Err(e) => errors::handle(&format!("{}{err}", errors::LUA, err =e)),
			}
		}

		Ok(())
	}
}

