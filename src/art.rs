use crate::clml_rs;
use crate::regex;
use crate::mlua;

use crate::assets::ascii_art;
use crate::errors;
use crate::info;
use info::distro;

use std::fs;
use std::env;
use std::path::{ Path };

use mlua::prelude::*;
use clml_rs::{ clml, CLML };
use regex::{ Regex };

use crate::{ Inject, Arguments };
use info::{ Info };
use distro::{ DistroColors };

pub(crate) struct Art {
	inner: String,
	width: i32,
	height: i32,
}

impl Art {
	pub fn new(info: &mut Info, arguments: &Arguments) -> Self {
		let mut to_return = Art {
			inner: String::new(),
			width: 0,
			height: 0,
		};

		// Get inner & distro colors.
		{
			match arguments.ascii_distro.clone() {
				None => {
					let art = Path::new("/home/")
						.join(env::var("USER").unwrap_or(String::new()))
						.join(".config/freshfetch/art.clml");
					if art.exists() {
						match fs::read_to_string(art) {
							Ok(file) => to_return.inner = clml(&file),
							Err(e) => {
								errors::handle(&format!("{}{file}{}{err}",
									errors::io::READ.0,
									errors::io::READ.1,
									file = "~/.config/freshfetch/art.clml",
									err = e));
								panic!();
							}
						}
					} else {
						let got = ascii_art::get(&info.distro.short_name);
						to_return.inner = String::from(got.0);
						info.distro.colors = DistroColors::from(got.1);
					}
				}
				Some(a) => {
					let got = ascii_art::get(&a);
					to_return.inner = String::from(got.0);
					info.distro.colors = DistroColors::from(got.1);
				}
			}
		}

		// Get width and height
		{
			let plaintext = {
				let regex = Regex::new(r#"(?i)\[(?:[\d;]*\d+[a-z])"#).unwrap();
				String::from(regex.replace_all(&to_return.inner, ""))
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

			to_return.width = w as i32;
			to_return.height = h as i32;
		}

		to_return
	}
}

impl Inject for Art {
	fn inject(&self, lua: &mut Lua) {
		let globals = lua.globals();

		match globals.set("art", self.inner.as_str()) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
		match globals.set("artWidth", self.width) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
		match globals.set("artHeight", self.height) {
			Ok(_) => (),
			Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
		}
	}
}
