use crate::mlua;
use crate::regex;

use crate::errors;
use super::kernel;

use std::fs::{ read_to_string };
use std::path::{ Path };
use std::process::{ Command };

use regex::{ Regex };
use mlua::prelude::*;

use crate::{ Inject };
use kernel::{ Kernel };

#[derive(Clone, Debug)]
pub struct Motherboard {
    pub name: String,
    pub vendor: String,
    pub revision: String,
}

impl Motherboard {
    pub(crate) fn new(k: &Kernel) -> Option<Self> {
        match k.name.as_str() {
            "Linux" => {
                let sys_devices_virtual_dmi_id = Path::new("/sys/devices/virtual/dmi/id");
                // Android
                if Path::new("/system/app").is_dir()
                && Path::new("/system/priv-app").is_dir() {
                    // TODO: If you know how to get the motherboard info (not
                    // the phone name) with getprop, lmk or make a PR.
                    None
                // Standard
                } else if sys_devices_virtual_dmi_id.exists() && (
                   sys_devices_virtual_dmi_id.join("board_name").is_file()
                || sys_devices_virtual_dmi_id.join("board_vendor").is_file()
                || sys_devices_virtual_dmi_id.join("board_version").is_file()) {
                    Some(Motherboard {
                        name: read_to_string(sys_devices_virtual_dmi_id.join("board_name"))
                            .unwrap_or(String::new())
                            .replace("\n", " ")
                            .trim()
                            .to_string(),
                        vendor: read_to_string(sys_devices_virtual_dmi_id.join("board_vendor"))
                            .unwrap_or(String::new())
                            .replace("\n", " ")
                            .trim()
                            .to_string(),
                        revision: read_to_string(sys_devices_virtual_dmi_id.join("board_version"))
                            .unwrap_or(String::new())
                            .replace("\n", " ")
                            .trim()
                            .to_string(),
                    })
                } else {
                    // TODO: Fallback? I only have 2 computers and the previous
                    // code works on both, but thats because they're both Arch
                    // Linux. Idk about stuff like OpenBSD or whatever.
                    None
                }
            }
            "Mac OS X"|"macOS" => {
                // TODO: It looks to me like something from the output of
                // `sysctl` can be used to get info of this nature. Not sure
                // personally, and I don't own a Mac to test on.
                None
            }
            "BSD"|"MINIX" => {
                // TODO: Idk BSD or MINUX, but I think this would be something
                // with `sysctl`.
                None
            }
            "Windows" /*(ew)*/ => {
                // TODO: Get someone to test this.
                let try_wmic = {
                    let try_output = Command::new("wmic")
                        .arg("baseboard")
                        .arg("get")
                        .arg("product,manufacturer")
                        .output();
                    match try_output {
                        Ok(output) => match String::from_utf8(output.stdout) {
                            Ok(stdout) => Some(stdout),
                            Err(_) => None,
                        }
                        Err(_) => None,
                    }
                };
                match try_wmic {
                    Some(wmic) => {
                        let try_name_n_vendor = {
                            let lines = wmic.split("\n").collect::<Vec<&str>>();
                            if lines.len() >= 2 {
                                let regex = Regex::new(r#"(\S+)\s+(\S+)"#).unwrap();
                                match regex.captures(&lines[1]) {
                                    Some(caps) => Some((
                                        String::from(caps.get(1).unwrap().as_str()),
                                        String::from(caps.get(2).unwrap().as_str()),
                                    )),
                                    None => None,
                                }
                            } else {
                                None
                            }
                        };
                        match try_name_n_vendor {
                            Some((name, vendor)) => Some(Motherboard {
                                name,
                                vendor,
                                revision: String::new(),
                            }),
                            None => None
                        }
                    }
                    None => None,
                }
            }
            _ => None,
        }
    }
}

impl Inject for Motherboard {
    fn inject(&self, lua: &mut Lua) {
        match lua.create_table() {
            Ok(t) => {
                match t.set("name", self.name.clone()) {
                    Ok(_) => (),
                    Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!() }
                }
                match t.set("vendor", self.vendor.clone()) {
                    Ok(_) => (),
                    Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!() }
                }
                match t.set("revision", self.revision.clone()) {
                    Ok(_) => (),
                    Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!() }
                }
                match lua.globals().set("motherboard", t) {
                    Ok(_) => (),
                    Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!() }
                }
            }
            Err(e) => {
                errors::handle(&format!("{}{}", errors::LUA, e));
                panic!();
            }
        }
    }
}

