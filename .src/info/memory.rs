use crate::mlua;
use crate::sysinfo;

use super::utils;
use crate::errors;

use mlua::prelude::*;
use sysinfo::{ SystemExt };

use crate::{ Inject };
use utils::{ get_system };

#[derive(Clone, Debug)]
pub(crate) struct Memory {
	pub max: u64,
	pub used: u64,
}

impl Memory {
	pub fn new() -> Self {
		let system = get_system();
		Memory {
			max: system.get_total_memory(),
			used: system.get_used_memory(),
		}	
	}
}

impl Inject for Memory {
	fn inject(&self, lua: &mut Lua) {
		let globals = lua.globals();
		match lua.create_table() {
			Ok(t) => {
				match t.set("max", self.max) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
				match t.set("used", self.used) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
				match globals.set("memory", t) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
			}
			Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
		}
	}
}