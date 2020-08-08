use crate::clml_rs;

use crate::assets::ascii_art;

use clml_rs::{ CLML };

use crate::{ Inject };
use ascii_art::{ get_ascii_art };

pub(crate) struct Art ( &'static str );

impl Art {
	pub fn new(of: &str) -> Self {
		Art(get_ascii_art(of))
	}
}

impl Inject for Art {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject env values.
		clml.env("art", &format!("{}", self.0));

		// Inject bash values.
		clml.bash_env("art", &format!("{}", self.0));

		// Inject Lua values.
		{
			let lua = &clml.lua_env;
			let globals = lua.globals();

			globals.set("art", self.0).or(Err(()))?;
		}

		Ok(())
	}
}
