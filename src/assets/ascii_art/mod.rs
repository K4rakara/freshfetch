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
	( "aix",				Check::StartsWith,	Some("AIX"),						include_str!("./large/a/.aix.clml"),				false,	),
	( "hash",				Check::StartsWith,	Some("Hash"),						include_str!("./large/.hash.clml"),					false,	),
	( "alpine",				Check::Is,			Some("Alpine"),						include_str!("./large/a/.alpine.clml"),				false,	),
	( "alpine_small",		Check::Is,			None,								include_str!("./small/a/.alpine.clml"),				false,	),
	( "alter",				Check::StartsWith,	Some("Alter"),						include_str!("./large/a/.alter.clml"),				false,	),
	( "amazon",				Check::StartsWith,	Some("Amazon"),						include_str!("./large/a/.amazon.clml"),				false,  ),
	( "anarchy",			Check::Is,			None,								include_str!("./large/a/.anarchy.clml"),			false,  ),
	( "android",			Check::StartsWith,	Some("Android"),					include_str!("./large/a/.android.clml"),			false,	),
	( "android_small",		Check::Is,			None,								include_str!("./small/a/.android.clml"),			false,	),
	( "antergos",			Check::StartsWith,	Some("Antergos"),					include_str!("./large/a/.antergos.clml"),			false,	),
	( "antix",				Check::StartsWith,	Some("antiX"),						include_str!("./large/a/.antix.clml"),				false,	),
	( "aosc_retro",			Check::StartsWith,	Some("AOSC OS/Retro"),				include_str!("./large/a/.aosc_os_retro.clml"),		false,	),
	( "aosc",				Check::StartsWith,	Some("AOSC OS"),					include_str!("./large/a/.aosc_os.clml"),			false,	),
	( "apricity",			Check::StartsWith,	Some("Apricity"),					include_str!("./large/a/.apricity.clml"),			false,	),
	( "arco",				Check::StartsWith,	Some("ArcoLinux"),					include_str!("./large/a/.arco.clml"),				false,	),
	( "arco_small",			Check::Is,			None,								include_str!("./small/a/.arco.clml"),				false,	),
	( "arch_old",			Check::Is,			None,								include_str!("./old/a/.arch.clml"),					false,	),
	( "arch_small",			Check::Is,			None,								include_str!("./small/a/.arch.clml"),				false,	),
	( "archbox",			Check::StartsWith,	Some("ArchBox"),					include_str!("./large/a/.archbox.clml"),			false,	),
	( "archlabs",			Check::StartsWith,	Some("ARCHlabs"),					include_str!("./large/a/.archlabs.clml"),			false,	),
	( "archstrike",			Check::StartsWith,	Some("ArchStrike"),					include_str!("./large/a/.archstrike.clml"),			false,	),
	( "arch",				Check::StartsWith,	Some("Arch"),						include_str!("./large/a/.arch.clml"),				false,	),
	( "artix",				Check::StartsWith,	Some("Artix"),						include_str!("./large/a/.artix.clml"),				false,	),
	( "artix_small",		Check::Is,			None,								include_str!("./small/a/.artix.clml"),				false,	),
	( "arya",				Check::StartsWith,	Some("Arya"),						include_str!("./large/a/.arya.clml"),				false,	),
	( "bedrock",			Check::StartsWith,	Some("Bedrock"),					include_str!("./large/b/.bedrock.clml"),			false,	),
	( "bitrig",				Check::StartsWith,	Some("Bitrig"),						include_str!("./large/b/.bitrig.clml"),				false,	),
	( "blackarch",			Check::StartsWith,	Some("BlackArch"),					include_str!("./large/b/.blackarch.clml"),			false,	),
	( "blag",				Check::StartsWith,  Some("BLAG"),						include_str!("./large/b/.blag.clml"),				false,	),
	( "blankon",			Check::StartsWith,	Some("BlankOn"),					include_str!("./large/b/.blankon.clml"),			false,	),
	( "bonsai",				Check::StartsWith,  Some("Bonsai"),						include_str!("./large/b/.bonsai.clml"),				false,	),
	( "bsd",				Check::Is,			Some("BSD"),						include_str!("./large/b/.bsd.clml"),				false,	),
	( "bunsenlabs",			Check::StartsWith,	Some("BunsenLabs"),					include_str!("./large/b/.bunsenlabs.clml"),			false,	),
	( "calculate",			Check::StartsWith,	Some("Calculate"),					include_str!("./large/c/.calculate.clml"),			false,	),
	( "carbs",				Check::StartsWith,	Some("Carbs"),						include_str!("./large/c/.carbs.clml"),				false,	),
	( "centos",				Check::StartsWith,	Some("CentOS"),						include_str!("./large/c/.centos.clml"),				false,	),
	( "centos_small",		Check::Is,			None,								include_str!("./small/c/.centos.clml"),				false,	),
	( "chakra",				Check::StartsWith,	Some("Chakra"),						include_str!("./large/c/.chakra.clml"),				false,	),
	( "chaletos",			Check::StartsWith,	Some("ChaletOS"),					include_str!("./large/c/.chaletos.clml"),			false,	),
	( "chapeau",			Check::StartsWith,	Some("Chapeau"),					include_str!("./large/c/.chapeau.clml"),			false,	),
	( "chrome",				Check::StartsWith,	Some("Chrom"),						include_str!("./large/c/.chrome.clml"),				false,	),
	( "cleanjaro",			Check::StartsWith,	Some("Cleanjaro"),					include_str!("./large/c/.cleanjaro.clml"),			false,	),
	( "cleanjaro_small",	Check::Is,			None,								include_str!("./small/c/.cleanjaro.clml"),			false,	),
	( "clearos",			Check::StartsWith,	Some("ClearOS"),					include_str!("./large/c/.clearos.clml"),			false,	),
	( "clear_linux",		Check::StartsWith,	Some("Clear Linux OS"),				include_str!("./large/c/.clear_linux.clml"),		false,	),
	( "",					Check::StartsWith,	Some("Clear_Linux"),				"@clear_linux",										false,	),
	( "clover",				Check::StartsWith,	Some("Clover"),						include_str!("./large/c/.clover.clml"),				false,	),
	( "condres",			Check::StartsWith,	Some("Condres"),					include_str!("./large/c/.condres.clml"),			false,	),
	( "container_linux",	Check::StartsWith,	Some("Container Linux by CoreOS"),	include_str!("./large/c/.container_linux.clml"),	false,	),
	( "",					Check::StartsWith,	Some("Container_Linux"),			"@container_linux",									false,	),
	( "crux",				Check::StartsWith,	Some("CRUX"),						include_str!("./large/c/.crux.clml"),				false,	),
	( "crux_small",			Check::Is,			None,								include_str!("./small/c/.crux.clml"),				false,	),
	( "cucumber",			Check::Contains,	Some("Cucumber"),					include_str!("./large/c/.cucumber.clml"),			false,	),
	( "debian",				Check::StartsWith,	Some("Debian"),						include_str!("./large/d/.debian.clml"),				false,	),
	( "debian_small",		Check::Is,			None,								include_str!("./small/d/.debian.clml"),				false,	),
	( "deepin",				Check::StartsWith,	Some("Deepin"),						include_str!("./large/d/.deepin.clml"),				false,	),
	( "desaos",				Check::Is,			Some("DesaOS"),						include_str!("./large/d/.desaos.clml"),				false,	),
	( "devuan",				Check::StartsWith,	Some("Devuan"),						include_str!("./large/d/.devuan.clml"),				false,	),
	( "dracos",				Check::StartsWith,	Some("DracOS"),						include_str!("./large/d/.dracos.clml"),				false,	),
	( "dragonfly",			Check::StartsWith,	Some("DragonFly"),					include_str!("./large/d/.dragonfly.clml"),			false,	),
	( "dragonfly_small",	Check::Is,			None,								include_str!("./small/d/.dragonfly.clml"),			false,	),
	( "dragonfly_old",		Check::Is,			None,								include_str!("./old/d/.dragonfly.clml"),			false,	),
	( "drauger",			Check::StartsWith,	Some("Drauger"),					include_str!("./large/d/.drauger.clml"),			false,	),
	( "elementary",			Check::StartsWith,	Some("Elementary"),					include_str!("./large/e/.elementary.clml"),			false,	),
	( "elementary_small",	Check::Is,			None,								include_str!("./small/e/.elementary.clml"),			false,	),
	( "endeavouros",		Check::StartsWith,	Some("EndeavourOS"),				include_str!("./large/e/.endeavouros.clml"),		false,	),
	( "endless",			Check::StartsWith,	Some("Endless"),					include_str!("./large/e/.endless.clml"),			false,	),
	( "eurolinux",			Check::StartsWith,	Some("EuroLinux"),					include_str!("./large/e/.eurolinux.clml"),			false,	),
	( "exhuerbo",			Check::StartsWith,	Some("Exherbo"),					include_str!("./large/e/.exherbo.clml"),			false,	),
	( "fedora",				Check::StartsWith,	Some("Fedora"),						include_str!("./large/f/.fedora.clml"),				false,	),
	( "fedora_small",		Check::Is,			None,								include_str!("./small/f/.fedora.clml"),				false,	),
	( "feren",				Check::StartsWith,	Some("Feren"),						include_str!("./large/f/.feren.clml"),				false,	),
	( "freebsd",			Check::StartsWith,	Some("FreeBSD"),					include_str!("./large/f/.freebsd.clml"),			false,	),
	( "freebsd_small",		Check::Is,			None,								include_str!("./small/f/.freebsd.clml"),			false,	),
	( "freemint",			Check::StartsWith,	Some("FreeMiNT"),					include_str!("./large/f/.freemint.clml"),			false,	),
	( "frugalware",			Check::StartsWith,	Some("Frugalware"),					include_str!("./large/f/.frugalware.clml"),			false,	),
	( "funtoo",				Check::StartsWith,	Some("Funtoo"),						include_str!("./large/f/.funtoo.clml"),				false,	),
	// Continue here.
	( "",					Check::StartsWith,	Some("RFRemix"),					"@fedora",											false,	),
	( "linux",				Check::Is,			None,								include_str!("./large/.linux.clml"),				false,	),
	( "linux_classic",		Check::Is,			None,								include_str!("./large/.linux_classic.clml"),		false,	),
	( "manjaro",			Check::Is,			None,								include_str!("./large/.manjaro.clml"),				false,	),
	( "manjaro_small",		Check::Is,			None,								include_str!("./small/.manjaro.clml"),				false,  ),
	( "windows10",			Check::Is,			None,								include_str!("./large/.windows10.clml"),			false,	),
	( "windows",			Check::Is,			None,								include_str!("./large/.windows.clml"),				false,	),
	( "xferience",			Check::Contains,	Some("XFerience"),					include_str!("./large/x/.xferience.clml"),			false,	),
];

pub(crate) fn get(of: &str) -> (&'static str, bool) {
	for art in ASCII_ART.iter() {
		let is_alias = art.3.starts_with("@");
		let get_tuple = || -> (&'static str, bool) {
			if !is_alias { (art.3, art.4) }
			else { get_cli(&art.3[1..]) }
		};
		match art.1 {
			Check::Is => {
				if art.2.is_none() {
					if of == art.0 { return get_tuple(); }
				} else {
					if of == art.2.clone().unwrap() { return get_tuple(); }
				}
			}
			Check::Contains => {
				if art.2.is_none() {
					if of.contains(art.0) { return get_tuple(); }
				} else {
					if of.contains(&art.2.clone().unwrap()) { return get_tuple(); }
				}
			}
			Check::StartsWith => {
				if art.2.is_none() {
					if of.starts_with(art.0) { return get_tuple(); }
				} else {
					if of.starts_with(&art.2.clone().unwrap()) { return get_tuple(); }
				}
			}
			Check::EndsWith => {
				if art.2.is_none() {
					if of.ends_with(art.0) { return get_tuple(); }
				} else {
					if of.ends_with(&art.2.clone().unwrap()) { return get_tuple(); }
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
