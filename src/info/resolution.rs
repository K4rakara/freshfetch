use crate::clml_rs;
use crate::x11rb;

use crate::errors;

use clml_rs::{ CLML };
use x11rb::{ connect, connection::{ Connection } };

use crate::{ Inject };

#[derive(Clone, Debug)]
pub(crate) struct Resolution {
	pub width: u16,
	pub height: u16,
}

impl Resolution {
	pub fn new() -> Option<Self> {
		match connect(None) {
			Ok((conn, screen_n)) => {
				let screen = &conn.setup().roots[screen_n];
				Some(Resolution {
					width: screen.width_in_pixels,
					height: screen.height_in_pixels,
				})
			}
			Err(_) => None,
		}
	}
}

impl Inject for Resolution {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject clml values.
		clml
			.env("resolution.width", &format!("{}", self.width))
			.env("resolution.height", &format!("{}", self.height));

		// Inject Bash values.
		clml
			.bash_env("resolution_width", &format!("{}", self.width))
			.bash_env("resolution_width", &format!("{}", self.height));

		// Inject Lua values.
		{
			let lua = &clml.lua_env;
			let globals = lua.globals();

			match lua.create_table() {
				Ok(t) => {
					match t.set("width", self.width) {
						Ok(_) => (),
						Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
					}
					match t.set("height", self.height) {
						Ok(_) => (),
						Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
					}
					match globals.set("resolution", t) {
						Ok(_) => (),
						Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
					}
				}
				Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
			}
		}
		
		Ok(())
	}
}