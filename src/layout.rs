use crate::clml_rs;

use crate::misc;
use crate::art;
use crate::info;

use clml_rs::{ CLML };

use crate::{ Inject, Arguments };
use misc::{ Terminal };
use art::{ Art };
use info::{ Info };

pub(crate) struct Layout {
	pub art: Art,
	pub info: Info,
	pub terminal: Terminal,
}

impl Layout {
	pub fn new(args: &Arguments) -> Self {
		Layout {
			art: Art::new(&args.ascii_distro.clone().unwrap_or(String::from("manjaro"))),
			info: Info::new(),
			terminal: Terminal::new(),
		}
	}
}

impl Inject for Layout {
	fn prep(&mut self) -> Result<(), ()> {
		self.info.prep()?;
		self.art.prep()?;
		self.terminal.prep()?;
		Ok(())
	}
	fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
		self.art.inject(clml)?;
		self.terminal.inject(clml)?;
		self.info.inject(clml)?;
		Ok(())
	}
}
