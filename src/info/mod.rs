use crate::regex;
use crate::sysinfo;
use crate::mlua;

use crate::errors;
use crate::assets;
use crate::assets::defaults;
pub(crate) mod kernel;
pub(crate) mod context;
pub(crate) mod distro;
pub(crate) mod uptime;
pub(crate) mod package_managers;
pub(crate) mod shell;
pub(crate) mod resolution;
pub(crate) mod wm;
pub(crate) mod de;
pub(crate) mod utils;
pub(crate) mod cpu;
pub(crate) mod gpu;
pub(crate) mod memory;
pub(crate) mod motherboard;
pub(crate) mod host;

use std::fs;
use std::path::Path;

use regex::Regex;
use sysinfo::SystemExt;
use mlua::prelude::*;

use crate::Inject;
use assets::{ ANSI, PRINT };
use defaults::INFO;
use utils::get_system;
use kernel::Kernel;
use context::Context;
use distro::Distro;
use uptime::Uptime;
use package_managers::PackageManagers;
use shell::Shell;
use resolution::Resolution;
use wm::Wm;
use de::De;
use cpu::Cpu;
use gpu::Gpus;
use memory::Memory;
use motherboard::Motherboard;
use host::Host;

pub(crate) struct Info {
	ctx: Lua,
	rendered: String,
	width: i32,
	height: i32,
	pub context: Option<Context>,
	pub distro: Distro,
	pub kernel: Kernel,
	pub uptime: Uptime,
	pub package_managers: PackageManagers,
	pub shell: Shell,
	pub resolution: Option<Resolution>,
	pub de: Option<De>,
	pub wm: Option<Wm>,
	pub cpu: Option<Cpu>,
	pub gpu: Option<Gpus>,
	pub memory: Memory,
  pub motherboard: Option<Motherboard>,
	pub host: Option<Host>,
}

impl Info {
	pub fn new() -> Self {
		get_system().refresh_all();
		let kernel = Kernel::new();
		let context = Context::new();
		let distro = Distro::new(&kernel);
		let uptime = Uptime::new(&kernel);
		let package_managers = PackageManagers::new(&kernel);
		let shell = Shell::new(&kernel);
		let resolution = Resolution::new(&kernel);
		let de = De::new(&kernel, &distro);
		let wm = Wm::new(&kernel);
		let cpu = Cpu::new(&kernel);
		let gpu = Gpus::new(&kernel);
		let memory = Memory::new();
    let motherboard = Motherboard::new(&kernel);
		let host = Host::new(&kernel);
    Info {
			ctx: Lua::new(),
			rendered: String::new(),
			width: 0,
			height: 0,
			context: context,
			distro: distro,
			kernel: kernel,
			uptime: uptime,
			package_managers: package_managers,
			shell: shell,
			resolution: resolution,
			de: de,
			wm: wm,
			cpu: cpu,
			gpu: gpu,
			memory: memory,
      motherboard,
			host,
		}
	}
	pub fn render(&mut self) {
		match self.ctx.load(PRINT).exec() {
			Ok(_) => (),
			Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
		}
		match self.ctx.load(ANSI).exec() {
			Ok(_) => (),
			Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
		}
		let info = Path::new("/home/")
			.join(self.context.clone()
				.unwrap_or(Context {
					user: String::new(),
					host: String::new(),
				})
				.user)
			.join(".config/freshfetch/info.lua");
		if info.exists() {
			match fs::read_to_string(&info) {
				Ok(file) => {
					match self.ctx.load(&file).exec() {
						Ok(_) => (),
						Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
					}
					match self.ctx.globals().get::<&str, String>("__freshfetch__") {
						Ok(v) => self.rendered = v,
						Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
					}
				}
				Err(e) => {
					errors::handle(&format!("{}{file:?}{}{err}",
						errors::io::READ.0,
						errors::io::READ.1,
						file = info,
						err = e));
					panic!();
				}
			}
		} else {
			match self.ctx.load(INFO).exec() {
				Ok(_) => (),
				Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
			}
			match self.ctx.globals().get::<&str, String>("__freshfetch__") {
				Ok(v) => self.rendered = v,
				Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
			}
		}
	}
}

impl Inject for Info {
	fn prep(&mut self) {
		match &self.context { Some(v) => v.inject(&mut self.ctx), None => (), }
		self.kernel.inject(&mut self.ctx);
		self.distro.inject(&mut self.ctx);
		self.uptime.inject(&mut self.ctx);
		self.package_managers.inject(&mut self.ctx);
		self.shell.inject(&mut self.ctx);
		match &self.resolution { Some(v) => v.inject(&mut self.ctx), None => (), }
		match &self.wm { Some(v) => v.inject(&mut self.ctx), None => (), }
		match &self.de { Some(v) => v.inject(&mut self.ctx), None => (), }
		match &self.cpu { Some(v) => v.inject(&mut self.ctx), None => (), }
		match &self.gpu { Some(v) => v.inject(&mut self.ctx), None => (), }
		self.memory.inject(&mut self.ctx);
    match &self.motherboard { Some(v) => v.inject(&mut self.ctx), None => (), }
		match &self.host { Some(v) => v.inject(&mut self.ctx), None => (), }
		self.render();
		{
			let plaintext = {
				let regex = Regex::new(r#"(?i)\[(?:[\d;]*\d+[a-z])"#).unwrap();
				String::from(regex.replace_all(&self.rendered, ""))
			};

			let mut w = 0usize;
			let mut h = 0usize;
			
			for line in plaintext.split("\n").collect::<Vec<&str>>() {
				{
					let len = line.chars().collect::<Vec<char>>().len();
					if len > w { w = len; }
				}
				h += 1;
			}

			self.width = w as i32;
			self.height = h as i32;
		}
	}
	fn inject(&self, lua: &mut Lua)  {
		let globals = lua.globals();

		match globals.set("info", self.rendered.as_str()) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
		match globals.set("infoWidth", self.width) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
		match globals.set("infoHeight", self.height) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
	}
}
