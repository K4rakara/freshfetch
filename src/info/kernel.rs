use crate::mlua;
use crate::uname;

use crate::errors;

use mlua::prelude::*;
use uname::uname;

use crate::Inject;

pub(crate) struct Kernel {
    pub name: String,
    pub version: String,
    pub architecture: String,
}

impl Kernel {
    pub fn new() -> Self {
        // TODO: Look into how `crate::uname::uname()` works and consider
        // switching this to a `match { Ok(v) => v Err(e) => { ... } }`.
        let uname = uname().expect("Failed to run `crate::uname::uname()`.");
        let name;
        match uname.sysname.as_str() {
            "Darwin" => {
                name = String::from("Darwin");
            }
            "SunOS" => {
                name = String::from("Solaris");
            }
            "Haiku" => {
                name = String::from("Haiku");
            }
            "MINIX" => {
                name = String::from("MINIX");
            }
            "AIX" => {
                name = String::from("AIX");
            }
            "FreeMiNT" => {
                name = String::from("FreeMiNT");
            }
            "Linux" => {
                name = String::from("Linux");
            }
            "DragonFly" => {
                name = String::from("BSD");
            }
            "Bitrig" => {
                name = String::from("BSD");
            }
            other => {
                if other.starts_with("GNU") {
                    name = String::from("Linux");
                } else if other.ends_with("BSD") {
                    name = String::from("BSD");
                } else if other.starts_with("CYGWIN")
                    || other.starts_with("MSYS")
                    || other.starts_with("MINGW")
                {
                    name = String::from("Windows");
                } else {
                    errors::handle(&format!("Unexpected OS \"{os}\". Create a pull request or issue at https://github.com/K4rakara/freshfetch to add support for your OS.",
						os = other));
                    panic!();
                }
            }
        }
        Kernel {
            name,
            version: uname.release,
            architecture: uname.machine,
        }
    }
}

impl Inject for Kernel {
    fn inject(&self, lua: &mut Lua) {
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
}
