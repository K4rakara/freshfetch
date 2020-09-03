use crate::clml_rs;
use crate::cmd_lib;
use crate::mlua;

use crate::errors;
use super::kernel;
use super::distro;

use std::env;

use mlua::prelude::*;
use clml_rs::{ CLML };
use cmd_lib::{ run_fun };

use crate::{ Inject };
use kernel::{ Kernel };
use distro::{ Distro };

pub(crate) struct De ( pub String, pub String, );

impl De {
	pub fn new(k: &Kernel, d: &Distro) -> Option<Self> {
		let to_return = match k.name.as_str() {
			"Mac OS X"|"macOS" => Some(De(String::from("Aqua"), String::new())),
			_ => {
				if d.short_name.starts_with("Windows") {
					if d.short_name.starts_with("Windows 8") || d.short_name.starts_with("Windows 10") {
						Some(De(String::from("Modern UI/Metro"), String::new()))
					} else {
						Some(De(String::from("Aero"), String::new()))
					}
				} else {
					if if let Ok(desktop_session) = env::var("DESKTOP_SESSION") { desktop_session == "regolith" } else { false } {
						Some(De(String::from("Regolith"), String::new()))
					} else if let Ok(mut current_desktop) = env::var("XDG_CURRENT_DESKTOP") {
						current_desktop = current_desktop.replace("X-", "");
						// The following is from neofetch, and I have
						// literally no idea what it does (~line 1718):
						// ```bash
						// de=${de/Budgie:GNOME/Budgie}
						// de=${de/:Unity7:ubuntu}
						// ```
						// Unless somebody opens a PR with whatever
						// that is in Rust, I'm just gonna pretend that
						// code doesn't exist lol.
						Some(De(current_desktop, String::new()))
					} else if env::var("GNOME_DESKTOP_SESSION_ID").is_ok() {
						Some(De(String::from("GNOME"), String::new()))
					} else if env::var("MATE_DESKTOP_SESSION_ID").is_ok() {
						Some(De(String::from("MATE"), String::new()))
					} else if env::var("TDE_FULL_SESSION").is_ok() {
						Some(De(String::from("Trinity"), String::new()))
					} else {
						None
					}
				}
			}
		};
		if let Some(mut to_return) = to_return {
			if env::var("KDE_SESSION_VERSION")
				.unwrap_or(String::from("0"))
				.parse::<i32>()
				.ok()
				.unwrap_or(0) >= 4 {
				to_return.0 = to_return.0.replace("KDE", "Plasma");
			}
			// Get version number.
			{
				// In neofetch, this uses a Bash switch statement, but because 
				// Bash switch statements let you do patterns, we can't use a 
				// switch statement here.
				if to_return.0.starts_with("Plasma") {
					to_return.1 = run_fun!( plasmashell --version )
						.ok()
						.unwrap_or(String::new())
						.replace("plasmashell ", "")
						.replace("\n", "");
				} else if to_return.0.starts_with("MATE") {
					to_return.1 = run_fun!( mate-session --version )
						.ok()
						.unwrap_or(String::new());
				} else if to_return.0.starts_with("Xfce") {
					to_return.1 = run_fun!( xfce4-session --version )
						.ok()
						.unwrap_or(String::new());
				} else if to_return.0.starts_with("GNOME") {
					to_return.1 = run_fun!( gnome-shell --version )
						.ok()
						.unwrap_or(String::new());
				} else if to_return.0.starts_with("Cinnamon") {
					to_return.1 = run_fun!( cinnamon --version )
						.ok()
						.unwrap_or(String::new());
				} else if to_return.0.starts_with("Budgie") {
					to_return.1 = run_fun!( budgie-desktop --version )
						.ok()
						.unwrap_or(String::new());
				} else if to_return.0.starts_with("LXQt") {
					to_return.1 = run_fun!( lxqt-session --version )
						.ok()
						.unwrap_or(String::new());
				} else if to_return.0.starts_with("Unity") {
					to_return.1 = run_fun!( $(unity --version) )
						.ok()
						.unwrap_or(String::new());
				}
			}
			Some(to_return)
		} else {
			to_return
		}
	}
}

impl Inject for De {
	fn inject(&self, lua: &mut Lua) {
		let globals = lua.globals();

		match lua.create_table() {
			Ok(t) => {
				match t.set("name", self.0.as_str()) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{err}", errors::LUA, err = e)); panic!(); }
				}
				match t.set("version", self.1.as_str()) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{err}", errors::LUA, err = e)); panic!(); }
				}
				match globals.set("de", t) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{err}", errors::LUA, err = e)); panic!(); }
				}
			}
			Err(e) => { errors::handle(&format!("{}{err}", errors::LUA, err = e)); panic!(); }
		}
	}
}
