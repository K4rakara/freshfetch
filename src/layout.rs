use crate::mlua;

use crate::misc;
use crate::art;
use crate::info;

use mlua::prelude::*;

use crate::{ Inject, Arguments };
use misc::Terminal;
use art::Art;
use info::Info;

pub(crate) struct Layout {
	pub art: Art,
	pub info: Info,
	pub terminal: Terminal,
}

impl Layout {
	pub fn new(args: &Arguments) -> Self {
		let mut info = Info::new();
		let art = Art::new(&mut info, &args);
		let terminal = Terminal::new();
		Layout {
			art: art,
			info: info,
			terminal: terminal,
		}
	}
}

impl Inject for Layout {
	fn prep(&mut self) {
		self.info.prep();
		self.art.prep();
		self.terminal.prep();
	}
	fn inject(&self, lua: &mut Lua) {
		self.art.inject(lua);
		self.terminal.inject(lua);
		self.info.inject(lua);
	}
}
