use crate::mlua;
use crate::regex;

use crate::errors;
use crate::utils;
use super::kernel;

use std::env::{ var, vars };
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

use regex::Regex;
use mlua::prelude::*;

use crate::Inject;
use utils::which::which;
use kernel::Kernel;

#[derive(Clone, Debug)]
pub(crate) struct Resolution {
	pub width: u16,
	pub height: u16,
    pub refresh: Option<f32>,
}

impl Resolution {
    pub fn new(k: &Kernel) -> Option<Self> {
        match k.name.as_str() {
            "Linux" => {
                if which("xrandr").is_some()
                && var("DISPLAY").is_ok()
                && var("WAYLAND_DISPLAY").is_err() {
                    let mut to_return = Resolution {
                        width: 0,
                        height: 0,
                        refresh: None,
                    };

                    // Get output of `xrandr --nograb --current`.
                    let xrandr_string = {
                        let try_xrandr = Command::new("sh")
                            .arg("-c")
                            .arg("xrandr --nograb --current")
                            .envs(&mut vars())
                            .output();
                        match try_xrandr {
                            Ok(xrandr) => match String::from_utf8(xrandr.stdout) {
                                Ok(xrandr) => xrandr,
                                Err(_) => return None,
                            },
                            Err(e) => {
                                errors::handle(&format!("{}{cmd}{}{err}\nNOTE: xrandr was found in the path, so this should have succeeded.\n",
                                    errors::CMD.0,
                                    errors::CMD.1,
                                    cmd = "xrandr --nograb --current",
                                    err = e));
                                return None;
                            } 
                        }
                    };

                    // Split the output into lines.
                    let xrandr_lines = xrandr_string
                        .split("\n")
                        .collect::<Vec<&str>>();
                    
                    // Get data from lines.
                    {
                        let regex = Regex::new(r#"\s+(?:(\d+)x(\d+))\s+((?:\d+)\.(?:\d+)\*)"#).unwrap();
                        for line in xrandr_lines.iter() {
                            match regex.captures(&line) {
                                Some(caps) => {
                                    match caps.get(1) {
                                        Some(cap) => match cap.as_str().parse::<u16>() {
                                            Ok(width) => to_return.width = width,
                                            // `unreachable!()` used here b/c
                                            // only digit characters should
                                            // be here.
                                            Err(_) => unreachable!(),
                                        }
                                        // `unreachable!()` used here because
                                        // its a required match.
                                        None => unreachable!(),
                                    }
                                    match caps.get(2) {
                                        Some(cap) => match cap.as_str().parse::<u16>() {
                                            Ok(height) => to_return.height = height,
                                            // Same reason as above.
                                            Err(_) => unreachable!(),
                                        }
                                        // Same reason as above.
                                        None => unreachable!(),
                                    }
                                    match caps.get(3) {
                                        Some(cap) => {
                                            let mut v = String::from(cap.as_str());
                                            v = v.replace("*", "");
                                            match v.parse::<f32>() {
                                                Ok(refresh) => to_return.refresh = Some(refresh),
                                                // Same reason as above.
                                                Err(_) => unreachable!(),
                                            }
                                        }
                                        // Same reason as above.
                                        None => unreachable!(),
                                    }
                                    return Some(to_return);
                                }
                                None => (),
                            }
                        }
                    }
                } else if which("xwininfo").is_some()
                && var("DISPLAY").is_ok()
                && var("WAYLAND_DISPLAY").is_err() {
                    let mut to_return = Resolution {
                        width: 0,
                        height: 0,
                        refresh: None
                    };

                    // Get output of `xwininfo -root`.
                    let xwininfo_string = {
                        let try_xwininfo = Command::new("sh")
                            .arg("-c")
                            .arg("xwininfo -root")
                            .envs(&mut vars())
                            .output();
                        match try_xwininfo {
                            Ok(xwininfo) => match String::from_utf8(xwininfo.stdout) {
                                Ok(xwininfo) => xwininfo,
                                Err(_) => return None, 
                            },
                            Err(e) => {
                                errors::handle(&format!("{}{cmd}{}{err}\nNOTE: xwininfo was found in the path, so this should have succeeded.\n",
                                    errors::CMD.0,
                                    errors::CMD.1,
                                    cmd = "xwininfo -root",
                                    err = e));
                                return None;
                            }
                        }
                    };

                    // Split into lines.
                    let xwininfo_lines = xwininfo_string
                        .split("\n")
                        .collect::<Vec<&str>>();

                    let width_regex = Regex::new(r#"\s+Width: (\d+)"#).unwrap();
                    let mut width_regex_captured = false;
                    let height_regex = Regex::new(r#"\s+Height: (\d+)"#).unwrap();
                    let mut height_regex_captured = false;

                    for line in xwininfo_lines.iter() {
                        match width_regex.captures(&line) {
                            Some(caps) => match caps.get(1) {
                                Some(cap) => match cap.as_str().parse::<u16>() {
                                    Ok(width) => {
                                        to_return.width = width;
                                        width_regex_captured = true;
                                    }
                                    Err(_) => unreachable!(),
                                }
                                None => unreachable!(),
                            }
                            None => (),
                        }
                        match height_regex.captures(&line) {
                            Some(caps) => match caps.get(1) {
                                Some(cap) => match cap.as_str().parse::<u16>() {
                                    Ok(height) => {
                                        to_return.height = height;
                                        height_regex_captured = true;
                                    }
                                    Err(_) => unreachable!(),
                                }
                                None => unreachable!(),
                            }
                            None => (),
                        }
                    }

                    if width_regex_captured
                    && height_regex_captured {
                        return Some(to_return);
                    }
                } else if Path::new("/sys/class/drm/").is_dir() {
                    if let Ok(entries) = Path::new("/sys/class/drm/").read_dir() {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                if entry.path().join("modes").is_file() {
                                    let modes_string = match read_to_string(entry.path().join("modes")) {
                                        Ok(modes) => modes,
                                        Err(_) => return None,
                                    };

                                    let modes_lines = modes_string
                                        .split("\n")
                                        .collect::<Vec<&str>>();

                                    for line in modes_lines.iter() {
                                        let line_split = line
                                            .split("x")
                                            .collect::<Vec<&str>>();
                                        let width = line_split.get(0);
                                        let height = line_split.get(1);
                                        if width.is_some()
                                        && height.is_some() {
                                            return Some(Resolution {
                                                width: width.unwrap().parse::<u16>().unwrap(),
                                                height: height.unwrap().parse::<u16>().unwrap(),
                                                refresh: None,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                None
            }

            _ => None,
        }
    }
}

impl Inject for Resolution {
	fn inject(&self, lua: &mut Lua) {
		let globals = lua.globals();

		match lua.create_table() {
			Ok(t) => {
				match t.set("width", self.width) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
				match t.set("height", self.height) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
                match self.refresh {
                    Some(refresh) => match t.set("refresh", refresh) {
                        Ok(_) => (),
                        Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
                    }
                    None => (),
                }
				match globals.set("resolution", t) {
					Ok(_) => (),
					Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
				}
			}
			Err(e) => { errors::handle(&format!("{}{}", errors::LUA, e)); panic!(); }
		}
	}
}

