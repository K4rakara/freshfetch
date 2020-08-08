use crate::clml_rs;
use crate::uname;
use crate::cmd_lib;
use crate::cpuid;

use std::fs;
use std::env;
use std::path::{ Path };
use std::process::{ Command, Stdio, exit };

use clml_rs::{ CLML };
use uname::{ uname, Info as UnameInfo };
use cmd_lib::{ run_cmd, run_fun };
use cpuid::{ identify as cpu_identify };
use crate::{ Inject };

pub(crate) struct OS ( pub String, pub String, pub String );
pub(crate) fn get_os() -> OS {
	let uname = uname().expect("Failed to run `crate::uname::uname()`.");
	let os_name;
	match uname.sysname.as_str() {
		"Darwin" => { os_name = String::from("Darwin"); }
		"SunOS" => { os_name = String::from("Solaris"); }
		"Haiku" => { os_name = String::from("Haiku"); }
		"MINIX" => { os_name = String::from("MINIX"); }
		"AIX" => { os_name = String::from("AIX"); }
		"FreeMiNT" => { os_name = String::from("FreeMiNT"); }
		"Linux" => { os_name = String::from("Linux"); }
		"DragonFly" => { os_name = String::from("BSD"); }
		"Bitrig" => { os_name = String::from("BSD"); }
		other => {
			if other.starts_with("GNU") { os_name = String::from("Linux"); }
			else if other.ends_with("BSD") { os_name = String::from("BSD"); }
			else if other.starts_with("CYGWIN") || other.starts_with("MSYS") || other.starts_with("MINGW") { os_name = String::from("Windows"); }
			else {
				println!("Unexpected OS \"{os}\". Create a pull request or issue at https://github.com/K4rakara/freshfetch to add support for your OS.",
					os = other);
				exit(1);
			}
		}
	}
	let os_version = uname.release;
	let os_architecture = uname.machine;
	OS(os_name, os_version, os_architecture)
}

#[derive(Clone, Debug)]
pub(crate) struct Distro {
	pub long_name: String,
	pub short_name: String,
	pub architecture: String,
}

impl Distro {
	pub fn new() -> Self {
		// Get the os details.s
		let os = get_os();
		// Create fallback values.
		let mut long_name = String::new();
		let mut short_name = String::new();
		match os.0.as_str() {
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
							.arg(r#"for file in /etc/lsb-release /usr/lib/os-release /etc/os-release /etc/openwrt_release; do source $file && break; done; echo ${PRETTY_NAME:-${DISTRIB_DESCRIPTION}} ${VERSION_ID:-${DISTRIB_RELEASE}}; echo ${PRETTY_NAME:-${DISTRIB_DESCRIPTION:-${DISTRIB_ID:-${TAILS_PRODUCT_NAME}}}};"#)
							.stdout(Stdio::piped())
							.output();
						if try_release.is_ok() {
							let release = String::from_utf8(try_release.unwrap().stdout).expect("");
							let lines: Vec<&str> = release.split("\n").collect();
						 	if lines.len() == 1 {
								(String::from(lines[0]), os.0.clone())
							} else if lines.len() >= 2 {
								(String::from(lines[0]), String::from(lines[1]))
							} else {
								(os.0.clone(), os.0.clone())
							}
						} else {
							(os.0.clone(), os.0.clone())
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
			architecture: os.2.clone(),
		}
	}
}

impl Inject for Distro {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject env values.
		clml
			.env("distro.fullname", &self.long_name)
			.env("distro.shortname", &self.short_name)
			.env("distro.architecture", &self.architecture);
		
		// Inject Bash values.
		clml
			.bash_env("distro_fullname", &self.long_name)
			.bash_env("distro_shortname", &self.short_name)
			.bash_env("distro_architecture", &self.architecture);
		
		// Inject Lua values.
		{
			let lua = &clml.lua_env;
			let globals = lua.globals();

			match lua.create_table() {
				Ok(t) => {
					match t.set("fullname", self.long_name.as_str()) {
						Ok(_) => (),
						Err(e) => panic!(format!("The following Lua error occured:\n{}", e)),
					}
					match t.set("shortname", self.short_name.as_str()) {
						Ok(_) => (),
						Err(e) => panic!(format!("The following Lua error occured:\n{}", e)),
					}
					match t.set("architecture", self.architecture.as_str()) {
						Ok(_) => (),
						Err(e) => panic!(format!("The following Lua error occured:\n{}", e)),
					}
					match globals.set("distro", t) {
						Ok(_) => (),
						Err(e) => panic!(format!("The following Lua error occured:\n{}", e)),
					}
				}
				Err(e) => panic!(format!("The following Lua error occured:\n{}", e)),
			}
		}	

		Ok(())
	}
}

pub(crate) struct Info {
	ctx: CLML,
	distro: Distro,
	rendered: String,
}

impl Info {
	pub fn new() -> Self {
		Info {
			ctx: CLML::new(),
			distro: Distro::new(),
			rendered: String::new(),
		}
	}
	pub fn render(&mut self) -> Result<(), ()> {
		self.rendered = self.ctx
			.parse(include_str!("./assets/defaults/info_wip.clml"))
			.or(Err(()))?;
		Ok(())
	}
}

impl Inject for Info {
	fn prep(&mut self) -> Result<(), ()> {
		self.distro.inject(&mut self.ctx)?;
		self.render()?;
		Ok(())
	}
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject env values.
		clml.env("info", &format!("{}", self.rendered));

		// Inject bash values.
		clml.bash_env("info", &format!("{}", self.rendered));

		// Inject Lua values.
		{
			let lua = &clml.lua_env;
			let globals = lua.globals();

			match globals.set("info", self.rendered.as_str()) {
				Ok(_) => (),
				Err(e) => panic!(format!("The following Lua error occured: {}", e)),
			}
		}

		Ok(())
	}
}
