use crate::sysinfo;

use std::sync::{ Mutex, MutexGuard };

use sysinfo::{ System, SystemExt, ProcessExt };

lazy_static! {
	static ref SYSTEM: Mutex<System> = Mutex::new(System::new());
}

pub(crate) fn get_system() -> MutexGuard<'static, System> { SYSTEM.lock().unwrap() }

#[derive(Clone, Debug)]
pub(crate) struct Grep {
	pub max: Option<usize>,
	pub search: Option<String>,
	pub searches: Option<Vec<String>>,
	pub only_matching: Option<bool>,
}

fn grep(through: Vec<String>, conf: Grep) -> Vec<String> {
	let mut conf = conf.clone();
	if conf.searches.is_none() && conf.search.is_some() { conf.searches = Some(vec![conf.search.clone().unwrap()]); }
	if conf.searches.is_none() { return vec![]; }
	else {
		let mut to_return = Vec::new();
		let mut i = 0usize;
		for value in through.iter() {
			for search in conf.searches.clone().unwrap().iter() {
				if value.to_lowercase().contains(search) {
					if conf.only_matching.unwrap_or(false) {
						let mut range = (value.to_lowercase().rfind(search).unwrap(), 0);
						range.1 = range.0 + search.chars().collect::<Vec<char>>().len();
						to_return.push(String::from(&value[range.0..range.1]));
					} else {
						to_return.push(value.clone());
					}
					i += 1;
				}
			}
			if Some(i) >= conf.max { return to_return; }
		}
		to_return
	}
}

#[derive(Clone, Debug)]
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
	pub fn grep(&self, conf: Grep) -> Vec<String> {
		grep(self.0.clone(), conf)
	}
	pub fn contains(&self, search: &str) -> bool {
		!self.grep(Grep {
			max: None,
			search: Some(String::from(search)),
			searches: None,
			only_matching: None,
		}).is_empty()
	}
}
