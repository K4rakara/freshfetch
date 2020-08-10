use crate::sysinfo;

use std::sync::{ Mutex, MutexGuard };

use sysinfo::{ System, SystemExt, ProcessExt };

lazy_static! {
	static ref SYSTEM: Mutex<System> = Mutex::new(System::new());
}

pub(crate) fn get_system() -> MutexGuard<'static, System> { SYSTEM.lock().unwrap() }

pub(crate) struct PsAux ( Vec<String> );

impl PsAux {
	pub fn new() -> Self {
		PsAux({
			let mut to_return: Vec<String> = Vec::new();
			let system = get_system();
			for (_, proc) in system.get_processes() { to_return.push(String::from(proc.name())); }
			to_return
		})
	}
	pub fn grep(&self, search: &str) -> Vec<String> {
		let mut to_return: Vec<String> = Vec::new();
		for proc in self.0.iter() { if proc.contains(search) { to_return.push(proc.clone()) } }
		to_return
	}
}
