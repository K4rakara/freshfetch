use crate::mlua;
use crate::regex;

use super::kernel;
use crate::errors;

use std::fs;
use std::path::Path;

use mlua::prelude::*;
use regex::Regex;

use crate::Inject;
use kernel::Kernel;

#[derive(Debug)]
pub(crate) struct Cpu {
    /// The name of the CPU.
    pub name: String,
    /// The name of the CPU, without any information cut off.
    pub full_name: String,
    /// The frequency of the CPU, in MHz.
    pub freq: f32,
    /// The number of cores in the CPU.
    pub cores: i32,
}

impl Cpu {
    pub fn new(k: &Kernel) -> Option<Self> {
        let mut name: Option<String> = None;
        let mut freq: Option<f32> = None;
        let mut cores: Option<i32> = None;
        match k.name.as_str() {
            "Linux" | "MINIX" | "Windows" => {
                // TODO: Neofetch has some code to handle oddball CPU
                // architectures here. Idk if rust has support for those, but
                // porting that functionality wouldn't do much harm.

                match fs::read_to_string("/proc/cpuinfo") {
                    Ok(cpu_info) => {
                        let cpu_info_lines: Vec<&str> = cpu_info.split('\n').collect();

                        // Get CPU name.
                        name = {
                            let mut to_return = None;
                            let mut skip = false;
                            for line in cpu_info_lines.iter() {
                                if !skip && line.starts_with("model name")
                                    || line.starts_with("Hardware")
                                    || line.starts_with("Processor")
                                    || line.starts_with("cpu model")
                                    || line.starts_with("chip type")
                                    || line.starts_with("cpu type")
                                {
                                    let split: Vec<&str> = line.split(": ").collect();
                                    to_return = Some(String::from(split[1]));
                                    skip = true;
                                }
                            }
                            to_return
                        };

                        // Get CPU frequency.
                        freq = {
                            if Path::new("/sys/devices/system/cpu/cpu0/cpufreq/").exists() {
                                let mut to_return = None;
                                let to_check = [
                                    "/sys/devices/system/cpu/cpu0/cpufreq/bios_limit",
                                    "/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq",
                                    "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq",
                                ];
                                for file in to_check.iter() {
                                    if to_return.is_none() {
                                        if let Ok(mut bios_limit) = fs::read_to_string(file) {
                                            bios_limit = bios_limit.replace(['\n', '\t'], "");
                                            match bios_limit.parse::<f32>() {
                                                Ok(freq) => to_return = Some(freq / 1000.0),
                                                Err(e) => {
                                                    errors::handle(
                                                        &format!("{}{v}{}{type}{}{err}",
                                                                       errors::PARSE.0,
                                                                       errors::PARSE.1,
                                                                       errors::PARSE.2,
                                                                       v = bios_limit,
                                                                       type = "f32",
                                                                       err = e),
                                                    );
                                                    panic!();
                                                }
                                            }
                                        }
                                    }
                                }
                                to_return
                            } else {
                                let mut to_return = None;
                                let mut skip = false;
                                for line in cpu_info_lines.iter() {
                                    if !skip && line.starts_with("cpu MHz")
                                        || line.starts_with("clock")
                                    {
                                        let split: Vec<&str> = line.split(": ").collect();
                                        let to_parse = String::from(split[1]).replace("MHz", "");
                                        to_return = match to_parse.parse::<f32>() {
                                            Ok(freq) => Some(freq / 1000.0),
                                            Err(e) => {
                                                errors::handle(&format!("{}{v}{}{type}{}{err}",
														errors::PARSE.0,
														errors::PARSE.1,
														errors::PARSE.2,
														v = to_parse,
														type = "f32",
														err = e));
                                                panic!();
                                            }
                                        };
                                        skip = true;
                                    }
                                }
                                to_return
                            }
                        };

                        // Get CPU cores.
                        cores = {
                            let mut to_return = 0;
                            for line in cpu_info_lines.iter() {
                                if line.starts_with("processor") {
                                    to_return += 1;
                                }
                            }
                            Some(to_return)
                        };
                    }
                    Err(e) => {
                        errors::handle(&format!(
                            "{}{file}{}{err}",
                            errors::io::READ.0,
                            errors::io::READ.1,
                            file = "/proc/cpuinfo",
                            err = e
                        ));
                        panic!();
                    }
                }
            }
            _ => (),
        }
        if let (Some(full_name), Some(freq), Some(cores)) = (name, freq, cores) {
            Some(Cpu {
                name: {
                    let mut to_return = full_name
                        .replace("(tm)", "")
                        .replace("(TM)", "")
                        .replace("(R)", "")
                        .replace("(r)", "")
                        .replace("CPU", "")
                        .replace("Processor", "")
                        .replace("Dual-Core", "")
                        .replace("Quad-Core", "")
                        .replace("Six-Core", "")
                        .replace("Eight-Core", "")
                        .replace("Quad-Core", "");
                    {
                        let regex = Regex::new(r#"(?i)\d\d?-Core"#).unwrap();
                        to_return = String::from(regex.replace_all(&to_return, ""));
                    }
                    {
                        let regex = Regex::new(r#"(?i), .*? Compute Cores"#).unwrap();
                        to_return = String::from(regex.replace_all(&to_return, ""));
                    }
                    to_return = to_return.replace("Cores ", " ");
                    {
                        let regex = Regex::new(r#"(?i)\("AuthenticAMD".*?\)"#).unwrap();
                        to_return = String::from(regex.replace_all(&to_return, ""));
                    }
                    {
                        let regex = Regex::new(r#"(?i)with Radeon .*? Graphics"#).unwrap();
                        to_return = String::from(regex.replace_all(&to_return, ""));
                    }
                    to_return = to_return
                        .replace(", altivec supported", "")
                        .replace("Technologies, Inc", "")
                        .replace("Core2", "Core 2");
                    {
                        let regex = Regex::new(r#"FPU.*?"#).unwrap();
                        to_return = String::from(regex.replace_all(&to_return, ""));
                    }
                    {
                        let regex = Regex::new(r#"Chip Revision.*?"#).unwrap();
                        to_return = String::from(regex.replace_all(&to_return, ""));
                    }
                    to_return = String::from(to_return.trim());
                    to_return
                },
                full_name,
                freq,
                cores,
            })
        } else {
            None
        }
    }
}

impl Inject for Cpu {
    fn inject(&self, lua: &mut Lua) {
        let globals = lua.globals();

        match lua.create_table() {
            Ok(t) => {
                match t.set("name", self.name.as_str()) {
                    Ok(_) => (),
                    Err(e) => {
                        errors::handle(&format!("{}{}", errors::LUA, e));
                        panic!();
                    }
                }
                match t.set("fullName", self.full_name.as_str()) {
                    Ok(_) => (),
                    Err(e) => {
                        errors::handle(&format!("{}{}", errors::LUA, e));
                        panic!();
                    }
                }
                match t.set("cores", self.cores) {
                    Ok(_) => (),
                    Err(e) => {
                        errors::handle(&format!("{}{}", errors::LUA, e));
                        panic!();
                    }
                }
                match t.set("freq", self.freq) {
                    Ok(_) => (),
                    Err(e) => {
                        errors::handle(&format!("{}{}", errors::LUA, e));
                        panic!();
                    }
                }
                match globals.set("cpu", t) {
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
}
