use crate::chrono;
use crate::clml_rs;
use crate::sysinfo;

use crate::errors;
use super::kernel;
use super::utils;

use std::path::{ Path };

use chrono::{ Utc, DateTime, Datelike, Timelike, TimeZone };
use clml_rs::{ CLML };
use sysinfo::{ SystemExt };

use crate::{ Inject };
use kernel::{ Kernel };
use utils::{ get_system };

pub(crate) struct Uptime ( pub DateTime<Utc> );

impl Uptime {
	pub fn new(k: &Kernel) -> Self {
		let uptime_seconds;
		match k.name.as_str() {
			"Linux"|"Windows"|"MINIX" => {
				// Since `crate::sysinfo::SystemExt::get_uptime()` gets uptime
				// from /proc/uptime, we should check that it exists and have a
				// fallback.
				if Path::new("/proc/uptime").exists() {
					uptime_seconds = get_system().get_uptime() as i64;
				} else {
					// `crate::sysinfo::SystemExt::get_boot_time()` doesn't
					// appear to rely on /proc/uptime, so we should be able to 
					// use it here.
					let boot_time = get_system().get_boot_time() as i64;
					let now_time = Utc::now().timestamp();
					uptime_seconds = boot_time - now_time;
				}
			}
			// Unknown OSes should have already exit(1)'d by now, this is just
			// to satisfy the compiler.
			_ => { uptime_seconds = 0; }
		}
		Uptime(Utc.timestamp(uptime_seconds, 0))
	}
}

impl Inject for Uptime {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject clml values.
		clml
			.env("uptime.days", format!("{}", self.0.day() - 1).as_str())
			.env("uptime.hours", format!("{}", self.0.hour()).as_str())
			.env("uptime.minutes", format!("{}", self.0.minute()).as_str())
			.env("uptime.seconds", format!("{}", self.0.second()).as_str());
		
		// Inject bash values.
		clml
			.bash_env("uptime_days", format!("{}", self.0.day() - 1).as_str())
			.bash_env("uptime_hours", format!("{}", self.0.hour()).as_str())
			.bash_env("uptime_minutes", format!("{}", self.0.minute()).as_str())
			.bash_env("uptime_seconds", format!("{}", self.0.second()).as_str());

		// Inject Lua values.
		{
			let lua = &clml.lua_env;
			let globals = lua.globals();

			match lua.create_table() {
				Ok(t) => {
					match t.set("days", self.0.day() - 1) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
					match t.set("hours", self.0.hour()) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
					match t.set("minutes", self.0.minute()) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
					match t.set("seconds", self.0.second()) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
					match globals.set("uptime", t) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
				}
				Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
			}
		}

		Ok(())
	}
}
