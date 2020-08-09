use crate::clml_rs;
use crate::uname;
use crate::cmd_lib;
use crate::cpuid;
use crate::chrono;
use crate::users;

use crate::errors;

use std::fs;
use std::env;
use std::path::{ Path };
use std::process::{ Command, Stdio, exit };

use clml_rs::{ CLML };
use uname::{ uname, Info as UnameInfo };
use cmd_lib::{ run_cmd, run_fun };
use cpuid::{ identify as cpu_identify };
use chrono::{ Utc, DateTime, Datelike, Timelike, TimeZone, };

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

pub(crate) struct Host ( pub String );

impl Host {
	pub fn new() -> Self {
		let hostname = &fs::read_to_string("/etc/hostname")
			.expect("Failed to read \"/etc/hostname\"!");
		Host(String::from(&hostname[0..(hostname.len() - 1)]))
	}
}

impl Inject for Host {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject CLML value.
		clml.env("host", self.0.as_str());

		// Inject Bash value.
		clml.bash_env("host", self.0.as_str());

		// Inject Lua value.
		match clml.lua_env.globals().set("host", self.0.as_str()) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}

		Ok(())
	}
}

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

		Ok(())
	}
}

pub(crate) struct Kernel {
	pub name: String,
	pub version: String,
	pub architecture: String,
}

impl Kernel {
	pub fn new() -> Self {
		let os = get_os();
		Kernel {
			name: os.0,
			version: os.1,
			architecture: os.2,
		}
	}
}

impl Inject for Kernel {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject env values.
		clml
			.env("kernel.name", self.name.as_str())
			.env("kernel.version", self.version.as_str())
			.env("kernel.architecture", self.architecture.as_str());
		
		// Inject bash values.
		clml
			.bash_env("kernel_name", self.name.as_str())
			.bash_env("kernel_version", self.version.as_str())
			.bash_env("kernel_architecture", self.architecture.as_str());

		// Inject Lua values.
		{
			let lua = &clml.lua_env;
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
					match t.set("architecture", self.architecture.as_str()) {
						Ok(_) => (),
						Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
					}
					match globals.set("kernel", t) {
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
							uptime_seconds = uptime_seconds_string.parse::<i64>().expect("Failed to convert /proc/uptime to a u64.");
						}
						Err(e) => panic!(format!("An I/O error occured while reading {}:\n{}", "/proc/uptime", e)),
					}
				} else {
					let boot_time = {
						let try_boot_time_output = Command::new("sh")
							.arg("-c")
							.arg(r#"printf "$(date -d"$(uptime -s)" +%s)""#)
							.output();
						match try_boot_time_output {
							Ok(boot_time_output) => {
								let boot_time_string = String::from_utf8(boot_time_output.stdout)
									.expect("`boot_time_output.stdout` contained invalid UTF8!");
								boot_time_string.parse::<i64>()
									.expect("Failed to convert `boot_time_string` to `u64`!")
							}
							Err(e) => panic!(format!(""))
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
		// Inject env values.
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

	pub fn get_main(&self) -> String {
		for package_manager in self.0.iter() {
			match package_manager.name.as_str() {
				"snap"|"flatpak"|"npm" => (),
				_ => {
					return package_manager.name.clone();
				}
			}
		}
		String::new()
	}
}

impl Inject for PackageManagers {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject env values.
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

pub(crate) struct Shell {
	pub name: String,
	pub version: String,
}

impl Shell {
	pub fn new(k: &Kernel) -> Self {
		let name;
		let version;
		match k.name.as_str() {
			"Linux"|"BSD"|"Windows" => {
				let shell_bin = String::from(
					Path::new(
						&match env::var("SHELL") {
							Ok(v) => v,
							Err(e) => panic!(format!("Failed to get $SHELL. Details:\n{}", e)),
						}
					)
					.file_name()
					.expect("$SHELL is invalid!")
					.to_string_lossy());
				name = shell_bin;
				match name.as_str() {
					"zsh" => version = {
						let try_output = Command::new("zsh")
							.arg("-c")
							.arg("printf $ZSH_VERSION")
							.output();
						match try_output {
							Ok(output) => {
								String::from_utf8(output.stdout)
									.expect("The output of \"zsh -c printf $ZSH_VERSION\" contained invalid UTF8.")
							}
							Err(e) => panic!("Failed to get ZSH_VERSION."),
						}
					},
					_ => version = String::new(),
				}
			}
			_ => { name = String::new(); version = String::new(); }
		}
		Shell {
			name: name,
			version: version,
		}
	}
}

impl Inject for Shell {
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject env values.
		clml
			.env("shell.name", self.name.as_str())
			.env("shell.version", self.version.as_str());

		// Inject bash values.
		clml
			.bash_env("shell_name", self.name.as_str())
			.bash_env("shell_version", self.version.as_str());

		// Inject Lua values.
		{
			let lua = &clml.lua_env;
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
		
		Ok(())
	}
}

pub(crate) struct Info {
	ctx: CLML,
	user: User,
	host: Host,
	distro: Distro,
	kernel: Kernel,
	uptime: Uptime,
	package_managers: PackageManagers,
	shell: Shell,
	rendered: String,
}

impl Info {
	pub fn new() -> Self {
		let kernel = Kernel::new();
		let uptime = Uptime::new(&kernel);
		let package_managers = PackageManagers::new(&kernel);
		let shell = Shell::new(&kernel);
		Info {
			ctx: CLML::new(),
			user: User::new(),
			host: Host::new(),
			distro: Distro::new(),
			kernel: kernel,
			uptime: uptime,
			package_managers: package_managers,
			shell: shell,
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
		self.user.inject(&mut self.ctx)?;
		self.host.inject(&mut self.ctx)?;
		self.kernel.inject(&mut self.ctx)?;
		self.distro.inject(&mut self.ctx)?;
		self.uptime.inject(&mut self.ctx)?;
		self.package_managers.inject(&mut self.ctx)?;
		self.shell.inject(&mut self.ctx)?;
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
				Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
			}
		}

		Ok(())
	}
}
