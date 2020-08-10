use crate::clml_rs;
use crate::regex;

use crate::assets::ascii_art;
use crate::errors;

use clml_rs::{ clml, CLML };
use regex::{ Regex };

use crate::{ Inject };

pub(crate) struct Art {
	inner: String,
	width: i32,
	height: i32,
}

impl Art {
	pub fn new(of: &str) -> Self {
		let mut to_return = Art {
			inner: String::from({
				let to_return;
				let got = ascii_art::get(of);
				if got.1 { to_return = clml(got.0); }
				else { to_return = String::from(got.0); }
				to_return
			}),
			width: 0,
			height: 0,
		};

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
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		// Inject clml values.
		clml
			.env("art", self.inner.as_str())
			.env("art.width", &format!("{}", self.width))
			.env("art.height", &format!("{}", self.height));

		// Inject Bash values.
		clml
			.bash_env("art", self.inner.as_str())
			.bash_env("art_width", &format!("{}", self.width))
			.bash_env("art_height", &format!("{}", self.height));

		// Inject Lua values.
		{
			let lua = &clml.lua_env;
			let globals = lua.globals();

			//k_err!((globals.set("art", self.inner.as_str())), (format!("")));
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

		Ok(())
	}
}
