use crate::mlua;

use crate::errors;

use mlua::prelude::*;

use crate::{ Inject };

use std::fs::{ read_to_string };
use std::env::{ var };

#[derive(Clone, Debug)]
pub(crate) struct Context {
	pub user: String,
	pub host: String,
}

impl Context {
	pub fn new() -> Option<Self> {
		Some(Context {
			user: match var("USER") {
				Ok(v) => v,
				Err(_) => return None,
			},
			host: match read_to_string("/etc/hostname") {
				Ok(v) => v,
				Err(_) => return None,
			}
		})
	} 
}

impl Inject for Context {
	fn inject(&self, lua: &mut Lua) {
		let globals = lua.globals();
		match lua.create_table() {
			Ok(t) => {
				match t.set("user", self.user.as_str()) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!() }
				}
			}
			Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!() }
		}
	}
}
