use crate::chrono;
use crate::clml_rs;
use crate::cmd_lib;

use crate::errors;
use super::kernel;

use std::fs;
use std::path::{ Path };

use chrono::{ Utc, DateTime, Datelike, Timelike, TimeZone };
use clml_rs::{ CLML };
use cmd_lib::{ run_fun };

use crate::{ Inject };
use kernel::{ Kernel };

pub(crate) struct Uptime ( pub DateTime<Utc> );

impl Uptime {
	pub fn new(k: &Kernel) -> Self {
		let uptime_seconds;
		match k.name.as_str() {
			"Linux"|"Windows"|"MINIX" => {
				if Path::new("/proc/uptime").exists() {
					match fs::read_to_string("/proc/uptime") {
						Ok(uptime_string) => {
							let uptime_seconds_string = String::from(uptime_string
								.split(".")
								.next()
								.unwrap());
							uptime_seconds = match uptime_seconds_string.parse::<i64>() {
								Ok(v) => v,
								Err(e) => {
									errors::handle(&format!("{}{v}{}i64{}{err}",
										errors::PARSE.0,
										errors::PARSE.1,
										errors::PARSE.2,
										v = uptime_seconds_string,
										err = e));
									panic!();
								}
							}
						}
						Err(e) => {
							errors::handle(&format!("{}{file}{}{err}",
								errors::io::READ.0,
								errors::io::READ.1,
								file = "/proc/uptime",
								err = e));
							panic!();
						}
					}
				} else {
					let boot_time = {
						let try_boot_time_string = run_fun!( printf "$(date -d"$(uptime -s)" +%s)" );
						match try_boot_time_string {
							Ok(boot_time_string) => {
								match boot_time_string.parse::<i64>() {
									Ok(v) => v,
									Err(e) => {
										errors::handle(&format!("{}{v}{}i64{}{err}",
											errors::PARSE.0,
											errors::PARSE.1,
											errors::PARSE.2,
											v = boot_time_string,
											err = e));
										panic!();
									}
								}
							}
							Err(e) => {
								errors::handle(&format!("{}{cmd}{}{err}",
									errors::CMD.0,
									errors::CMD.1,
									cmd = r#"printf "$(date -d"$(uptime -s)" +%s)"#,
									err = e));
								panic!();
							}
						}
					};
					let now_time = Utc::now().timestamp();
					uptime_seconds = boot_time - now_time;
				}
			}
			// Unknown OS'es should have already exit(1)'d by now, this is just
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
