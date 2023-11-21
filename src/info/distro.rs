use crate::cmd_lib;
use crate::mlua;

use crate::errors;
use super::kernel;

use std::fs;
use std::env;
use std::path::Path;
use std::process::{ Command, Stdio };

use mlua::prelude::*;
use cmd_lib::run_fun;

use crate::Inject;
use kernel::Kernel;

#[derive(Clone, Debug)]
pub(crate) struct Distro {
	pub long_name: String,
	pub short_name: String,
	pub architecture: String,
	pub colors: DistroColors,
}

impl Distro {
	pub fn new(k: &Kernel) -> Self {
		// Create fallback values.
		let mut long_name = String::new();
		let mut short_name = String::new();
		match k.name.as_str() {
			"Linux"|"BSD"|"MINIX" => {
				// Bedrock Linux
				if Path::new("/bedrock/etc/bedrock-release").exists()
				&& env::var("PATH").unwrap_or(String::new()).contains("/bedrock/cross/") {
					long_name = fs::read_to_string("/bedrock/etc/bedrock-release")
						.unwrap_or(String::from("Bedrock Linux"));
					short_name = String::from("Bedrock Linux");
				}
				// Red Star OS
				else if Path::new("/etc/redstar-release").exists() {
					long_name = {
						// TODO: Rework this into rust.
						let to_return;
						let release = run_fun!(printf "Red Star OS $(awk -F'[^0-9*]' '$0=$2' /etc/redstar-release)");
						if release.is_err() { to_return = String::from("Red Star OS"); }
						else { to_return = release.unwrap(); }
						to_return
					};
					short_name = String::from("Red Star OS");
				}
				// Generic
				else if Path::new("/etc/os-release").exists()
				|| Path::new("/usr/share/os-release").exists()
				|| Path::new("/etc/openwrt_release").exists()
				|| Path::new("/etc/lsb-release").exists() {
					let (long, short) = {
						// TODO: Rework this into pure rust.
						let try_release = Command::new("sh")
							.arg("-c")
							.arg(r#"for file in /etc/lsb-release /usr/lib/os-release /etc/os-release /etc/openwrt_release; do [[ -f $file ]] && source $file && break; done; echo ${PRETTY_NAME:-${DISTRIB_DESCRIPTION}} ${VERSION_ID:-${DISTRIB_RELEASE}}; echo ${PRETTY_NAME:-${DISTRIB_DESCRIPTION:-${DISTRIB_ID:-${TAILS_PRODUCT_NAME}}}};"#)
							.stdout(Stdio::piped())
							.output();
						if try_release.is_ok() {
							let release = String::from_utf8(try_release.unwrap().stdout).expect("");
							let lines: Vec<&str> = release.split("\n").collect();
						 	if lines.len() == 1 {
								(String::from(lines[0]), k.name.clone())
							} else if lines.len() >= 2 {
								(String::from(lines[0]), String::from(lines[1]))
							} else {
								(k.name.clone(), k.name.clone())
							}
						} else {
							(k.name.clone(), k.name.clone())
						}
					};
					long_name = long;
					short_name = short;
				}
			}
			_ => {} // Do nothing, unknown OS'es should have already exited by now.
		}
		Distro {
			long_name: long_name,
			short_name: short_name,
			architecture: k.architecture.clone(),
			colors: DistroColors::new(),
		}
	}
}

impl Inject for Distro {
	fn inject(&self, lua: &mut Lua) {
		self.colors.inject(lua);
		let globals = lua.globals();
		match lua.create_table() {
				Ok(t) => {
					match t.set("fullname", self.long_name.as_str()) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
					match t.set("shortname", self.short_name.as_str()) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
					match t.set("architecture", self.architecture.as_str()) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
					match globals.set("distro", t) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
				}
				Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
	}
}

#[derive(Clone, Debug)]
pub(crate) struct DistroColors ( pub String, pub String, pub String, pub String );

impl DistroColors {
	pub fn new() -> Self {
		DistroColors (
			String::from("\u{001b}[38;5;7m"),
			String::from("\u{001b}[38;5;7m"),
			String::from("\u{001b}[38;5;7m"),
			String::from("\u{001b}[38;5;7m"),
		)
	}
}

impl Inject for DistroColors {
	fn inject(&self, lua: &mut Lua) {
		let globals = lua.globals();

		match lua.create_table() {
			Ok(t) => {
				match t.raw_insert(1, self.0.as_str()) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
				match t.raw_insert(2, self.1.as_str()) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
				match t.raw_insert(3, self.2.as_str()) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
				match t.raw_insert(4, self.3.as_str()) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
				match globals.set("distroColors", t) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
			}
			Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
		}
	}
}

impl From<[Option<&'static str>; 4]> for DistroColors {
	fn from(v: [Option<&'static str>; 4]) -> Self {
		let mut _1 = String::from(v[0].unwrap_or("white"));
		let mut _2 = String::from(v[1]
			.unwrap_or(v[0]
				.unwrap_or("\u{001b}[38;5;7m")));
		let mut _3 = String::from(v[2]
			.unwrap_or(v[1]
				.unwrap_or(v[0]
					.unwrap_or("\u{001b}[38;5;7m"))));
		let mut _4 = String::from(
			v[3].unwrap_or(
				v[2].unwrap_or(v[1]
					.unwrap_or(v[0]
						.unwrap_or("\u{001b}[38;5;7m")))));
		if _2 == "\u{001b}[38;5;7m" { _2 = _1.clone(); }
		DistroColors ( _1, _2, _3, _4 )
	}
}
