use crate::clml_rs;
use crate::cmd_lib;
use crate::regex;

use crate::errors;
use super::kernel;

use clml_rs::{ CLML };
use cmd_lib::{ run_fun };
use regex::{ Regex };

use crate::{ Inject };
use kernel::{ Kernel };

#[derive(Clone, Debug)]
pub(crate) struct Gpu {
    pub brand: String,
    pub name: String,
}

impl Gpu {
    #[inline]
    pub fn new(name: String, brand: String) -> Self {
        Gpu {
            name: name,
            brand: brand,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Gpus ( pub Vec<Gpu> );

impl Gpus {
	pub fn new(k: &Kernel) -> Option<Self> {
		match k.name.as_str() {
			"Linux" => {
				// TODO: Make a rust binding to whatever `lspci` uses, and use
				// that instead.
				let mut gpus = match run_fun!( lspci -mm ) {
					Ok(lspci) => {
						let mut to_return = Vec::new();
						let regex = Regex::new(r#"(?i)"(.*?(?:Display|3D|VGA).*?)" "(.*?\[.*?\])" "(?:.*?\[(.*?)\])""#).unwrap();
						let lspci_lines = lspci.split("\n").collect::<Vec<&str>>();
						for line in lspci_lines.iter() {
							let captures = regex.captures(&line);
							match captures {
								Some(captures) => {
									to_return.push((
										String::from(captures.get(1).unwrap().as_str()),
										String::from(captures.get(2).unwrap().as_str()),
										String::from(captures.get(3).unwrap().as_str()),
									));
								}
								None => (),
							}
						}
						to_return
					}
					Err(e) => {
						errors::handle(&format!("{}{cmd}{}{err}",
							errors::CMD.0,
							errors::CMD.1,
							cmd = "lspci -mm",
							err = e));
						panic!();
					}
				};

                dbg!(&gpus);

				// Fix Intel integrated graphics crap
				{
                    if gpus.len() >= 2 {
                        if gpus[0].1.to_lowercase().contains("intel")
                        && gpus[1].1.to_lowercase().contains("intel") {
                            gpus.pop();
                        }
                    }
				}

                let mut to_return: Vec<Gpu> = Vec::new(); 

                for gpu in gpus.iter_mut() {
                    if gpu.1.to_lowercase().contains("advanced") {
                        let mut brand = gpu.1.clone();
                        {
                            let regex = Regex::new(r#".*?AMD.*?ATI.*?"#).unwrap();
                            brand = String::from(regex.replace_all(&brand, "AMD ATI"));
                        }
                        to_return.push(
                            Gpu::new(
                                gpu.2.clone(),
                                brand
                                    .replace("[", "")
                                    .replace("]", "")
                                    .replace("OEM", "")
                                    .replace("Advanced Micro Devices, Inc.", "")));
                    } else if gpu.1.to_lowercase().contains("nvidea") {
                        to_return.push(
                            Gpu::new(
                                gpu.2.clone(),
                                gpu.1
                                    .clone()
                                    .replace("[", "")
                                    .replace("]", "")));
                    } else if gpu.1.to_lowercase().contains("intel") {
                        let mut brand = gpu.1.clone();
                        brand = {
                            let regex = Regex::new(".*?Intel").unwrap();
                            String::from(regex.replace(&brand, "Intel"))
                        };
                        brand = brand.replace("(R)", "").replace("Corporation", "");
                        brand = {
                            let regex = Regex::new(r#" \(.*?"#).unwrap();
                            String::from(regex.replace_all(&brand, ""))
                        };
                        brand = brand
                            .replace("Integrated Graphics Controller", "");
                        brand = {
                            let regex = Regex::new(r#".*?Xeon.*?"#).unwrap();
                            String::from(regex.replace(&brand, "Intel HD Graphics"))
                        };
                        brand = String::from(brand.trim());
                        if brand == "" { brand = String::from("Intel HD Graphics"); }
                        to_return.push(
                            Gpu::new(
                                gpu.2.clone(),
                                brand));
                    }
                }

                if to_return.len() >= 1 {
                    Some(Gpus(to_return))
                } else {
                    None
                }
			}
			_ => None,
		}
	}
}

impl Inject for Gpus {
    fn inject(&self, clml: &mut CLML) -> Result<(), ()> {
        // Inject CLML values.
        clml.env("gpus.len", &format!("{}", self.0.len()));
        for (i, gpu) in self.0.iter().enumerate() {
            clml
                .env(&format!("gpus.{}.name", i), gpu.name.as_str())
                .env(&format!("gpus.{}.brand", i), gpu.brand.as_str());
        }

        // Inject Bash values.
        {
            let mut to_return = String::from("(");
            for i in 0..(self.0.len() - 1) {
                if i != 0 { to_return.push(' '); }
                to_return = format!("{}gpus_{}",
                    to_return,
                    i);
            }
            to_return = String::from({
                let mut split = to_return.split("").collect::<Vec<&str>>();
                split.pop();
                split.join("")
            });
            to_return.push(')');
            clml.bash_env("gpus", to_return.as_str());
        }
        for (i, gpu) in self.0.iter().enumerate() {
            clml
                .bash_env(&format!("gpus_{}_name", i), gpu.name.as_str())
                .bash_env(&format!("gpus_{}_brand", i), gpu.brand.as_str());
        }

        // Inject lua values.
        {
            let lua = &clml.lua_env;
            let globals = lua.globals();

            match lua.create_table() {
                Ok(a) => {
                    for (i, gpu) in self.0.iter().enumerate() {
                        match lua.create_table() {
                            Ok(t) => {
                                match t.set("name", gpu.name.as_str()) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        errors::handle(&format!("{}{}", errors::LUA, e));
                                        panic!();
                                    }
                                }
                                match t.set("brand", gpu.brand.as_str()) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        errors::handle(&format!("{}{}", errors::LUA, e));
                                        panic!();
                                    }
                                }
                                match a.raw_set((i + 1) as i64, t) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        errors::handle(&format!("{}{}", errors::LUA, e));
                                        panic!();
                                    }
                                }
                            }
                            Err(e) => {
                                errors::handle(&format!("{}{}", errors::LUA, e));
                                panic!();
                            }
                        }
                    }
                    match globals.set("gpus", a) {
                        Ok(_) => (),
                        Err(e) => {
                            errors::handle(&format!("{}{}", errors::LUA, e));
                            panic!();
                        }
                    }
                }
                Err(e) => {
                    errors::handle(&format!("{}{}", errors::LUA, e));
                    panic!();
                }
            }
        }

        Ok(())
    }
}


