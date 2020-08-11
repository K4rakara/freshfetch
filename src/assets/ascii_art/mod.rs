enum Check {
	Is,
	StartsWith,
	EndsWith,
	Contains,
}

/// ASCII art of various distros.
/// 
/// `_.0`: `&'static str` -- The CLI name of this art.
/// `_.1`: `Check` -- The type of comparison to make.
/// `_.2`: `Option<&'static str>` -- The `shortname` of this art, if it is different than the CLI name.
/// `_.3`: `&'static str` -- The actual string of the art.
/// `_.4`: `bool` -- if this art needs to be parsed at runtime or not.
static ASCII_ART: &[(&'static str, Check, Option<&'static str>, &'static str, bool)] = &[
	( "aix",			Check::StartsWith,	Some("AIX"),			include_str!("./large/a/.aix.clml"),				false,	),
	( "hash",			Check::StartsWith,	Some("Hash"),			include_str!("./large/.hash.clml"),					false,	),
	( "alpine",			Check::Is,			Some("Alpine"),			include_str!("./large/a/.alpine.clml"),				false,	),
	( "alpine_small",	Check::Is,			None,					include_str!("./small/a/.alpine.clml"),				false,	),
	( "alter",			Check::StartsWith,	Some("Alter"),			include_str!("./large/a/.alter.clml"),				false,	),
	( "amazon",			Check::StartsWith,	Some("Amazon"),			include_str!("./large/a/.amazon.clml"),				false,  ),
	( "anarchy",		Check::Is,			None,					include_str!("./large/a/.anarchy.clml"),			false,  ),
	( "android",		Check::StartsWith,	Some("Android"),		include_str!("./large/a/.android.clml"),			false,	),
	( "android_small",	Check::Is,			None,					include_str!("./small/a/.android.clml"),			false,	),
	( "antergos",		Check::StartsWith,	Some("Antergos"),		include_str!("./large/a/.antergos.clml"),			false,	),
	( "antix",			Check::StartsWith,	Some("antiX"),			include_str!("./large/a/.antix.clml"),				false,	),
	( "aosc_retro",		Check::StartsWith,	Some("AOSC OS/Retro"),	include_str!("./large/a/.aosc_os_retro.clml"),		false,	),
	( "aosc",			Check::StartsWith,	Some("AOSC OS"),		include_str!("./large/a/.aosc_os.clml"),			false,	),
	( "apricity",		Check::StartsWith,	Some("Apricity"),		include_str!("./large/a/.apricity.clml"),			false,	),
	( "arco",			Check::StartsWith,	Some("ArcoLinux"),		include_str!("./large/a/.arco.clml"),				false,	),
	( "arco_small",		Check::Is,			None,					include_str!("./small/a/.arco.clml"),				false,	),
	( "arch_old",		Check::Is,			None,					include_str!("./old/a/.arch.clml"),					false,	),
	( "arch_small",		Check::Is,			None,					include_str!("./small/a/.arch.clml"),				false,	),
	( "archbox",		Check::StartsWith,	Some("ArchBox"),		include_str!("./large/a/.archbox.clml"),			false,	),
	( "archlabs",		Check::StartsWith,	Some("ARCHlabs"),		include_str!("./large/a/.archlabs.clml"),			false,	),
	( "archstrike",		Check::StartsWith,	Some("ArchStrike"),		include_str!("./large/a/.archstrike.clml"),			false,	),
	( "arch",			Check::StartsWith,	Some("Arch"),			include_str!("./large/a/.arch.clml"),				false,	),
	( "artix",			Check::StartsWith,	Some("Artix"),			include_str!("./large/a/.artix.clml"),				false,	),
	( "artix_small",	Check::Is,			None,					include_str!("./small/a/.artix.clml"),				false,	),
	( "arya",			Check::StartsWith,	Some("Arya"),			include_str!("./large/a/.arya.clml"),				false,	),
	( "bedrock",		Check::StartsWith,	Some("Bedrock"),		include_str!("./large/b/.bedrock.clml"),			false,	),
	( "bitrig",			Check::StartsWith,	Some("Bitrig"),			include_str!("./large/b/.bitrig.clml"),				false,	),
	( "blackarch",		Check::StartsWith,	Some("BlackArch"),		include_str!("./large/b/.blackarch.clml"),			false,	),
	( "blag",			Check::StartsWith,  Some("BLAG"),			include_str!("./large/b/.blag.clml"),				false,	),
	( "blankon",		Check::StartsWith,	Some("BlankOn"),		include_str!("./large/b/.blankon.clml"),			false,	),
	( "bonsai",			Check::StartsWith,  Some("Bonsai"),			include_str!("./large/b/.bonsai.clml"),				false,	),
	( "bsd",			Check::Is,			Some("BSD"),			include_str!("./large/b/.bsd.clml"),				false,	),
	( "bunsenlabs",		Check::StartsWith,	Some("BunsenLabs"),		include_str!("./large/b/.bunsenlabs.clml"),			false,	),
	( "calculate",		Check::StartsWith,	Some("Calculate"),		include_str!("./large/c/.calculate.clml"),			false,	),
	( "carbs",			Check::StartsWith,	Some("Carbs"),			include_str!("./large/c/.carbs.clml"),				false,	),
	// Continue here.
	( "linux",			Check::Is,			None,					include_str!("./large/.linux.clml"),				false,	),
	( "linux_classic",	Check::Is,			None,					include_str!("./large/.linux_classic.clml"),		false,	),
	( "manjaro",		Check::Is,			None,					include_str!("./large/.manjaro.clml"),				false,	),
	( "manjaro_small",	Check::Is,			None,					include_str!("./small/.manjaro.clml"),				false,  ),
	( "windows10",		Check::Is,			None,					include_str!("./large/.windows10.clml"),			false,	),
	( "windows",		Check::Is,			None,					include_str!("./large/.windows.clml"),				false,	),
	( "xferience",		Check::Contains,	Some("XFerience"),		include_str!("./large/x/.xferience.clml"),			false,	),
];

pub(crate) fn get(of: &str) -> (&'static str, bool) {
	for art in ASCII_ART.iter() {
		match art.1 {
			Check::Is => {
				if art.2.is_none() {
					if of == art.0 { return (art.3, art.4); }
				} else {
					if of == art.2.clone().unwrap() { return (art.3, art.4); }
				}
			}
			Check::Contains => {
				if art.2.is_none() {
					if of.contains(art.0) { return (art.3, art.4); }
				} else {
					if of.contains(&art.2.clone().unwrap()) { return (art.3, art.4); }
				}
			}
			Check::StartsWith => {
				if art.2.is_none() {
					if of.starts_with(art.0) { return (art.3, art.4); }
				} else {
					if of.starts_with(&art.2.clone().unwrap()) { return (art.3, art.4); }
				}
			}
			Check::EndsWith => {
				if art.2.is_none() {
					if of.ends_with(art.0) { return (art.3, art.4); }
				} else {
					if of.ends_with(&art.2.clone().unwrap()) { return (art.3, art.4); }
				}
			}
		}
	}
	return get("linux");
}

pub(crate) fn get_cli(of: &str) -> (&'static str, bool) {
	for art in ASCII_ART.iter() {
		if art.0 == of {
			return (art.3, art.4);
		}
	}
	return get_cli("linux");
}
