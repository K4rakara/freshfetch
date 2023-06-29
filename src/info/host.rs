use crate::mlua;
use crate::regex;

use super::kernel;
use crate::errors;

use std::fs::read_to_string;

use mlua::prelude::*;
use regex::Regex;

use crate::Inject;
use kernel::Kernel;

#[derive(Clone, Debug)]
pub(crate) struct Host {
    pub model: String,
}

impl Host {
    pub fn new(k: &Kernel) -> Option<Self> {
        match k.name.as_str() {
            "Linux" => {
                let mut product_name =
                    match read_to_string("/sys/devices/virtual/dmi/id/product_name") {
                        Ok(product_name) => product_name,
                        Err(_) => return None,
                    };
                product_name = product_name
                    .replace('\n', "")
                    .replace("To Be Filled By O.E.M.", "")
                    .replace("Not Applicable", "")
                    .replace("System Product Name", "")
                    .replace("Undefined", "")
                    .replace("Default string", "")
                    .replace("Not Specified", "")
                    .replace("INVALID", "")
                    .replace('�', "");
                {
                    let regex = Regex::new(r#"(?i)To Be Filled.*?"#).unwrap();
                    product_name = String::from(regex.replace_all(&product_name, ""));
                }
                product_name = String::from(product_name.trim());
                if !product_name.is_empty() {
                    Some(Host {
                        model: product_name,
                    })
                } else {
                    None
                }
            }
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
