use crate::mlua;

use crate::errors;
use super::kernel;
use super::utils;

use std::fs;
use std::env;
use std::process::Command;

use mlua::prelude::*;

use crate::Inject;
use kernel::Kernel;
use utils::{ PsAux, Grep };

pub(crate) struct Wm ( pub String );

impl Wm {
	pub fn new(k: &Kernel) -> Option<Self> {
		if env::var("WAYLAND_DISPLAY").is_ok() {
			let res = PsAux::new().grep(Grep {
				max: Some(1usize),
				search: None,
				searches: Some(vec![
					String::from("arcan"),
					String::from("arcan"),
					String::from("asc"),
					String::from("clayland"),
					String::from("dwc"),
					String::from("fireplace"),
					String::from("gnome-shell"),
					String::from("greenfield"),
					String::from("grefsen"),
					String::from("kwin"),
					String::from("lipstick"),
					String::from("maynard"),
					String::from("mazecompositor"),
					String::from("motorcar"),
					String::from("orbital"),
					String::from("orbment"),
					String::from("perceptia"),
					String::from("rustland"),
					String::from("sway"),
					String::from("ulubis"),
					String::from("velox"),
					String::from("wavy"),
					String::from("way-cooler"),
					String::from("wayfire"),
					String::from("wayhouse"),
					String::from("westeros"),
					String::from("westford"),
					String::from("weston"),
				]),
				only_matching: Some(true),
			});
			if !res.is_empty() {
				Some(Wm(res[0].clone()))
			} else {
				None
			}
		} else if env::var("DISPLAY").is_ok() && k.name != "macOS" && k.name != "Mac OS X" && k.name != "FreeMiNT" {
			// TODO: Port this to rust using `x11rb` or a similar lib.
			let try_output = Command::new("bash")
				.arg("-c")
				.arg(r#"id=$(xprop -root -notype _NET_SUPPORTING_WM_CHECK) && id=${id##* } && wm=$(xprop -id "$id" -notype -len 100 -f _NET_WM_NAME 8t) && wm=${wm/*WM_NAME = } && wm=${wm/\"} && wm=${wm/\"*} && printf $wm"#)
				.output();
			match try_output {
				Ok(output) => {
					let stdout = match String::from_utf8(output.stdout.clone()) {
						Ok(v) => v,
						Err(e) => {
							errors::handle(&format!("{}{v:?}{}String{}{err}",
								errors::PARSE.0,
								errors::PARSE.1,
								errors::PARSE.2,
								v = output.stdout,
								err = e));
							panic!();
						}
					};
					if stdout != "" {
						Some(Wm(stdout))
					} else {
						None
					}
				}
				Err(e) => {
					errors::handle(&format!("{}{cmd}{}{err}",
						errors::CMD.0,
						errors::CMD.1,
						cmd = "...",
						err = e));
					panic!();
				}
			}
		} else {
			match k.name.as_str() {
				"Mac OS X"|"macOS" => {
					let res = PsAux::new().grep(Grep {
						max: Some(1usize),
						search: None,
						searches: Some(vec![
							String::from("spectacle"),
							String::from("amethyst"),
							String::from("kwm"),
							String::from("chunkwm"),
							String::from("abai"),
							String::from("rectangle"),
						]),
						only_matching: Some(true),
					});
					if !res.is_empty() {
						Some(Wm(res[0].clone()))
					} else {
						Some(Wm(String::from("Quartz Compositor")))
					}
				}
				"Windows" => {
					let mut res = PsAux::new().grep(Grep {
						max: Some(1usize),
						search: None,
						searches: Some(vec![
							String::from("bugn"),
							String::from("Windawesome"),
							String::from("blackbox"),
							String::from("emerge"),
							String::from("litestep"),
						]),
						only_matching: Some(true),
					});
					if !res.is_empty() {
						if res[0] == "blackbox" { res[0] = String::from("bbLean (Blackbox)"); }
						Some(Wm(format!("{}, Explorer", res[0].clone())))
					} else {
						Some(Wm(String::from("Explorer")))
					}
				}
				"FreeMiNT" => {
					match fs::read_dir("/proc/") {
						Ok(dir) => {
							for try_file in dir {
								match try_file {
									Ok(file) => {
										match file.path().file_name() {
											Some(v) => {
												let name = v.to_string_lossy();
												if name.contains("xaaes") || name.contains("xaloader") {
													return Some(Wm(String::from("XaAES")));
												} else if name.contains("myaes") {
													return Some(Wm(String::from("MyAES")));
												} else if name.contains("naes") {
													return Some(Wm(String::from("N.AES")));
												} else if name.contains("geneva") {
													return Some(Wm(String::from("Geneva")));
												}
											}
											None => (),
										}
									}
									Err(_) => (),
								}
							}
							Some(Wm(String::from("Atari AES")))
						}
						Err(_) => Some(Wm(String::from("Atari AES"))),
					}
				}
				_ => None,
			}
		}
	}
}

impl Inject for Wm {
	fn inject(&self, lua: &mut Lua) {
		match lua.globals().set("wm", self.0.as_str()) {
			Ok(_) => (),
			Err(e) => { errors::handle(&format!("{}{err}", errors::LUA, err = e)); panic!(); }
		}
	}
}
