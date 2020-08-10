use crate::clml_rs;

use crate::errors;

use clml_rs::{ CLML };

use crate::{ Inject };

pub(crate) struct User ( pub String );

impl User {
	pub fn new() -> Self {
		User(String::from(users::get_current_username()
			.expect("Failed to get current username!")
			.to_string_lossy()))
	}
}

impl Inject for User {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject CLML value.
		clml.env("user", self.0.as_str());

		// Inject Bash value.
		clml.bash_env("user", self.0.as_str());

		// Inject Lua value.
		match clml.lua_env.globals().set("user", self.0.as_str()) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}

		Ok(())
	}
}
