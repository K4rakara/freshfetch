use crate::mlua;

use super::kernel;
use crate::errors;

use std::env;
use std::path::Path;
use std::process::Command;

use mlua::prelude::*;

use crate::Inject;
use kernel::Kernel;

pub(crate) struct Shell {
	pub name: String,
	pub version: String,
}

impl Shell {
	pub fn new(k: &Kernel) -> Self {
		let name;
		let version;
		match k.name.as_str() {
			"Linux" | "BSD" | "Windows" => {
				let shell_bin = String::from(
					Path::new(&match env::var("SHELL") {
						Ok(v) => v,
						#[allow(non_fmt_panics)]
						Err(e) => panic!(format!("Failed to get $SHELL. Details:\n{}", e)),
					})
					.file_name()
					.expect("$SHELL is invalid!")
					.to_string_lossy(),
				);
				name = shell_bin;
				match name.as_str() {
					"zsh" => {
						version = {
							let try_output = Command::new("zsh")
								.arg("-c")
								.arg("printf $ZSH_VERSION")
								.output();
							match try_output {
							Ok(output) => {
								String::from_utf8(output.stdout)
									.expect("The output of \"zsh -c printf $ZSH_VERSION\" contained invalid UTF8.")
							}
							Err(_) => panic!("Failed to get ZSH_VERSION."),
						}
						}
					}
					_ => version = String::new(),
				}
			}
			_ => {
				name = String::new();
				version = String::new();
			}
		}
		Shell {
			name: name,
			version: version,
		}
	}
}

impl Inject for Shell {
	fn inject(&self, lua: &mut Lua) {
		let globals = lua.globals();

		match lua.create_table() {
			Ok(t) => {
				match t.set("name", self.name.as_str()) {
					Ok(_) => (),
					Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
				}
				match t.set("version", self.version.as_str()) {
					Ok(_) => (),
					Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
				}
				match globals.set("shell", t) {
					Ok(_) => (),
					Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
				}
			}
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
	}
}
