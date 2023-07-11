use crate::mlua;

use crate::art;
use crate::info;
use crate::misc;

use mlua::prelude::*;

use crate::{Arguments, Inject};
use art::Art;
use info::Info;
use misc::Terminal;

pub(crate) struct Layout {
    pub art: Art,
    pub info: Info,
    pub terminal: Terminal,
}

impl Layout {
    pub fn new(args: &Arguments) -> Self {
        let mut info = Info::new();
        let art = Art::new(&mut info, args);
        let terminal = Terminal::new();
        Layout {
            art,
            info,
            terminal,
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
