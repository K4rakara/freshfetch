use crate::errors;
use crate::mlua;
use crate::Inject;

use super::kernel::Kernel;
use mlua::prelude::*;

use std::fs::read_to_string;

#[derive(Clone, Debug)]
pub(crate) struct Host {
    pub model: String,
}

impl Host {
    pub fn new(k: &Kernel) -> Option<Self> {
        match &*k.name {
            "Linux" => Some(Host {
                model: read_to_string("/sys/devices/virtual/dmi/id/product_name").unwrap(),
            }),
            _ => None,
        }
    }
}

impl Inject for Host {
    fn inject(&self, lua: &mut Lua) {
        let globals = lua.globals();
        match lua.create_table() {
            Ok(t) => {
                match t.set("model", self.model.as_str()) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
                }
                match globals.set("host", t) {
                    Ok(_) => (),
                    Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
                }
            }
            Err(e) => errors::handle(&format!("{}{}", errors::LUA, e)),
        }
    }
}
