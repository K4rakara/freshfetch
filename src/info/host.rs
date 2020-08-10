use crate::clml_rs;

use crate::errors;

use std::fs;

use clml_rs::{ CLML };

use crate::{ Inject };

pub(crate) struct Host ( pub String );

impl Host {
	pub fn new() -> Self {
		let hostname = &fs::read_to_string("/etc/hostname")
			.expect("Failed to read \"/etc/hostname\"!");
		Host(String::from(&hostname[0..(hostname.len() - 1)]))
	}
}

impl Inject for Host {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject CLML value.
		clml.env("host", self.0.as_str());

		// Inject Bash value.
		clml.bash_env("host", self.0.as_str());

		// Inject Lua value.
		match clml.lua_env.globals().set("host", self.0.as_str()) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}

		Ok(())
	}
}
