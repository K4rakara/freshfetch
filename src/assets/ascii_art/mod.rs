/// ASCII art of various distros.
static ASCII_ART: &[(&'static str, &'static str)] = &[
	( "arch",			include_str!("./arch.clml"),		),
	( "tux_classic",	include_str!("./tux_classic.clml"),	),
];

pub(crate) fn get_ascii_art(of: &str) -> &'static str {
	for distro in ASCII_ART.iter() { if distro.0 == of { return distro.1 } }
	include_str!("./tux_classic.clml")
}
