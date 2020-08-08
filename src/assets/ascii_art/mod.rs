/// ASCII art of various distros.
static ASCII_ART: &[(&'static str, &'static str, bool)] = &[
	( "arch",			include_str!("./.arch.clml"),			false,	),
	( "linux",			include_str!("./.linux_classic.clml"),	false,	),
	( "linux_classic",	include_str!("./.linux_classic.clml"),	false,	),
	( "linux_256",		include_str!("./.linux_256.clml"),		false,	),
];

pub(crate) fn get_ascii_art(of: &str) -> &'static str {
	for distro in ASCII_ART.iter() { if distro.0 == of { return distro.1 } }
	include_str!("./linux_classic.clml")
}
