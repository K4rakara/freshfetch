/// ASCII art of various distros.
static ASCII_ART: &[(&'static str, &'static str, bool)] = &[
	( "arch",					include_str!("./large/simple/.arch.clml"),		false,	),
	( "arch_simple",			"@arch",										false,	),
	( "arch_complex",			include_str!("./large/complex/.arch.clml"),		false,	),
	( "arch_large_simple",		"@arch",										false,	),
	( "arch_large_complex",		"@arch_complex",								false,	),
	( "arch_small_simple",		include_str!("./small/simple/.arch.clml"),		false,	),
	( "arch_small_complex",		include_str!("./small/complex/.arch.clml"),		false,	),
	( "linux",					include_str!("./large/simple/.linux.clml"),		false,	),
	( "linux_simple",			"@linux",										false,	),
	( "linux_complex",			include_str!("./large/complex/.linux.clml"),	false,	),
	( "linux_large",			"@linux",										false,	),
	( "linux_large_simple",		"@linux",										false,	),
	( "linux_large_complex",	"@linux_complex",								false,	),
	( "manjaro",				include_str!("./large/simple/.manjaro.clml"),	false,	),
	( "manjaro_simple",			"@manjaro",										false,	),
	( "manjaro_complex",		include_str!("./large/complex/.manjaro.clml"),	false,	),
	( "manjaro_large_simple",	"@manjaro",										false,	),
	( "manjaro_large_complex",	"@manjaro_complex",								false,	),
	( "manjaro_small",			include_str!("./small/simple/.manjaro.clml"),	false,  ),
	( "manjaro_small_simple",	"@manjaro_small",								false,  ),
	( "manjaro_small_complex",	include_str!("./small/complex/.manjaro.clml"),	false,  ),
];

pub(crate) fn get_ascii_art(of: &str) -> &'static str {
	for distro in ASCII_ART.iter() {
		if distro.0 == of {
			if distro.1.starts_with("@") {
				return get_ascii_art(&distro.1[1..]);
			} else {
				return distro.1;
			}
		}
	}
	include_str!("./linux_classic.clml")
}
