use crate::clml_rs;

use crate::errors;
use super::kernel;

use std::process::{ Command };

use clml_rs::{ CLML };

use crate::{ Inject };
use kernel::{ Kernel };

pub(crate) struct PackageManager {
	pub name: String,
	pub packages: i32,
}

impl PackageManager {
	pub fn new(name: &str, packages: i32) -> Self { PackageManager { name: String::from(name), packages: packages } }
}

pub(crate) struct PackageManagers ( Vec<PackageManager> );

impl PackageManagers {
	pub fn new(k: &Kernel) -> Self {
		let mut to_return = Vec::new();
		
		let has = |package_manager: &str| -> bool {
			let try_output = Command::new("sh")
				.arg("-c")
				.arg(format!(r#"type -p "{}""#, package_manager))
				.output();
			match try_output {
				Ok(output) => output.status.success(),
				Err(_) => false,
			}
		};
		let mut add = |package_manager: &str, command: &str| {
			to_return.push(PackageManager::new(package_manager, {
				let try_output = Command::new("sh")
					.arg("-c")
					.arg(&format!(r#"{}"#, command))
					.output();
				match try_output {
					Ok(output) => {
						let stdout_string = String::from_utf8(output.stdout)
							.expect(&format!("The output of \"{}\" contained invalid UTF8.", command));
						let stdout_lines: Vec<&str> = stdout_string
							.split("\n")
							.collect();
						// 1 is subtracted because of the trailing
						// newline that commands have.
						stdout_lines.len() as i32 - 1
					}
					Err(e) => panic!(format!("Failed to run \"{cmd}\" Details:\n{err}",
						cmd = command,
						err = e)),
				}
			}))
		};

		match k.name.as_str() {
			"Linux"|"BSD"|"iPhone OS"|"Solaris" => {
				if has("kiss") { add("kiss", "kiss l"); }
				if has("pacman-key") { add("pacman", "pacman -Qq --color never"); }
				if has("dpkg") { add("dpkg", "dpkg-query -f '.\n' -W"); }
				if has("rpm") { add("rpm", "rpm -qa"); }
				if has("xbps-query") { add("xbps-query", "xbps-query -l"); }
				if has("apk") { add("apk", "apk info"); }
				if has("opkg") { add("opkg", "opkg list-installed"); }
				if has("pacman-g2") { add("pacman-g2", "pacman-g2 -Q"); }
				if has("lvu") { add("lvu", "lvu installed"); }
				if has("tce-status") { add("tce-status", "tce-status -i"); }
				if has("pkg-info") { add("pkg-info", "pkg_info"); }
				if has("tazpkg") { add("tazpkg", "tazpkg list"); }
				if has("sorcery") { add("sorcery", "gaze installed"); }
				if has("alps") { add("alps", "alps showinstalled"); }
				if has("butch") { add("butch", "butch list"); }
				if has("mine") { add("mine", "mine -q"); }
				
				if has("flatpak") { add("flatpak", "flatpak list"); }
				if has("snap") {
					let daemon_running = {
						let try_output = Command::new("sh")
							.arg("-c")
							.arg(r#"ps aux | grep -qFm 1 snapd"#)
							.output();
						match try_output {
							Ok(output) => output.status.success(),
							Err(_) => false,
						}
					};
					if daemon_running { add("snap", "snap list"); }
				}

				if has("npm") { add("npm", "ls $(npm root -g) --color=none"); }
			}
			_ => {}
		}

		PackageManagers(to_return)
	}
}

impl Inject for PackageManagers {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject clml values.
		{
			let prefix = String::from("packageManagers");
			for (i, package_manager) in self.0.iter().enumerate() {
				{
					let prefix = format!("{}.{}", prefix, i);
					clml
						.env(&format!("{}.{}", prefix, "name"), package_manager.name.as_str())
						.env(&format!("{}.{}", prefix, "packages"), &format!("{}", package_manager.packages));
				}
				{
					let prefix = format!("{}.{}", prefix, package_manager.name);
					clml
						.env(&prefix, &format!("{}", package_manager.packages));
				}
			}
		}

		// Inject bash values.
		{
			let mut to_return = String::from("(");
			for (i, package_manager) in self.0.iter().enumerate() {
				if i != 0 { to_return = format!("{} ", to_return); }
				to_return = format!("{}{}",
					to_return,
					format!("\"{}:{}\"", package_manager.name, package_manager.packages));
			}
			to_return = format!("{})", to_return);
			clml.bash_env("package_managers", to_return.as_str());
		}

		// Inject Lua values.
		{
			let lua = &clml.lua_env;
			let globals = lua.globals();

			match lua.create_table() {
				Ok(t) => {			
					for (i, package_manager) in self.0.iter().enumerate() {
						match lua.create_table() {
							Ok(t2) => {
								match t2.set("name", package_manager.name.as_str()) {
									Ok(_) => (),
									Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
								}
								match t2.set("packages", package_manager.packages) {
									Ok(_) => (),
									Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
								}
								match t.raw_insert(i as i64 + 1, t2) {
									Ok(_) => (),
									Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
								}
							}
							Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
						}
					}
					match globals.set("packageManagers", t) {
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
