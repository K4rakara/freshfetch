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
	( "",					Check::StartsWith,	Some("Ataraxia Linux"),				"@janus",											false,	),
	( "",					Check::StartsWith,	Some("Ataraxia"),					"@janus",											false,	),
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
	( "galliumos",			Check::StartsWith,	Some("GalliumOS"),					include_str!("./large/g/.galliumos.clml"),			false,	),
	( "garuda",				Check::StartsWith,	Some("Garuda"),						include_str!("./large/g/.garuda.clml"),				false,	),
	( "gentoo",				Check::StartsWith,	Some("Gentoo"),						include_str!("./large/g/.gentoo.clml"),				false,	),
	( "gentoo_small",		Check::Is,			None,								include_str!("./small/g/.gentoo.clml"),				false,	),
	( "gnewsense",			Check::StartsWith,	Some("gNewSense"),					include_str!("./large/g/.gnewsense.clml"),			false,	),
	( "gnome",				Check::StartsWith,	Some("GNOME"),						include_str!("./large/g/.gnome.clml"),				false,	),
	( "gnu",				Check::Is,			Some("GNU"),						include_str!("./large/g/.gnu.clml"),				false,	),
	( "gobo",				Check::StartsWith,	Some("GoboLinux"),					include_str!("./large/g/.gobo.clml"),				false,	),
	( "grombyang",			Check::StartsWith,	Some("Grombyang"),					include_str!("./large/g/.grombyang.clml"),			false,	),
	( "haiku",				Check::StartsWith,	Some("Haiku"),						include_str!("./large/h/.haiku.clml"),				false,	),
	( "haiku_small",		Check::Is,			None,								include_str!("./small/h/.haiku.clml"),				false,	),
	( "huayra",				Check::StartsWith,	Some("Huayra"),						include_str!("./large/h/.huayra.clml"),				false,	),
	( "hyperbola",			Check::StartsWith,	Some("Hyperbola"),					include_str!("./large/h/.hyperbola.clml"),			false,	),
	( "hyperbola_small",	Check::Is,			None,								include_str!("./small/h/.hyperbola.clml"),			false,	),
	( "janus",				Check::StartsWith,	Some("janusLinux"),					include_str!("./large/j/.janus.clml"),				false,	),
	( "",					Check::StartsWith,	Some("janus"),						"@janus",											false,	),
	( "kali",				Check::StartsWith,	Some("Kali"),						include_str!("./large/k/.kali.clml"),				false,	),
	( "kaos",				Check::StartsWith,	Some("KaOS"),						include_str!("./large/k/.kaos.clml"),				false,	),
	( "kde",				Check::StartsWith,	Some("KDE"),						include_str!("./large/k/.kde.clml"),				false,	),
	( "kibojoe",			Check::StartsWith,	Some("Kibojoe"),					include_str!("./large/k/.kibojoe.clml"),			false,	),
	( "kogaion",			Check::StartsWith,	Some("Kogaion"),					include_str!("./large/k/.kogaion.clml"),			false,	),
	( "korora",				Check::StartsWith,	Some("Korora"),						include_str!("./large/k/.korora.clml"),				false,	),
	( "kslinux",			Check::StartsWith,	Some("KSLinux"),					include_str!("./large/k/.kslinux.clml"),			false,	),
	( "kubuntu",			Check::StartsWith,	Some("Kubuntu"),					include_str!("./large/k/.kubuntu.clml"),			false,	),
	( "lede",				Check::StartsWith,	Some("LEDE"),						include_str!("./large/l/.lede.clml"),				false,	),
	( "libreelec",			Check::StartsWith,	Some("LibreELEC"),					include_str!("./large/l/.libreelec.clml"),			false,	),
	( "linux",				Check::Is,			Some("Linux"),						include_str!("./large/l/.linux.clml"),				false,	),
	( "linux_classic",		Check::Is,			None,								include_str!("./large/l/.linux_classic.clml"),		false,	),
	( "linuxlite",			Check::StartsWith,	Some("Linux Lite"),					include_str!("./large/l/.linuxlite.clml"),			false,	),
	( "linuxlite_small",	Check::Is,			None,								include_str!("./small/l/.linuxlite.clml"),			false,	),
	( "",					Check::StartsWith,	Some("Linux_Lite"),					"@linuxlite",										false,	),
	( "lubuntu",			Check::StartsWith,	Some("Lubuntu"),					include_str!("./large/l/.lubuntu.clml"),			false,	),
	( "lunar",				Check::StartsWith,	Some("Lunar"),						include_str!("./large/l/.lunar.clml"),				false,	),
	( "mac",				Check::StartsWith,	Some("mac"),						include_str!("./large/m/.mac.clml"),				false,	),
	( "mac_small",			Check::Is,			None,								include_str!("./small/m/.mac.clml"),				false,	),
	( "mac_modern",			Check::Is,			None,								include_str!("./large/m/.mac_modern.clml"),			false,	),
	( "",					Check::StartsWith,	Some("Darwin"),						"@mac",												false,	),
	( "mageia",				Check::StartsWith,	Some("Mageia"),						include_str!("./large/m/.mageia.clml"),				false,	),
	( "mageia_small",		Check::Is,			None,								include_str!("./small/m/.mageia.clml"),				false,	),
	( "magpieos",			Check::StartsWith,	Some("MagpieOS"),					include_str!("./large/m/.magpieos.clml"),			false,	),
	( "mandriva",			Check::StartsWith,	Some("Mandriva"),					include_str!("./large/m/.mandriva.clml"),			false,	),
	( "manjaro",			Check::Is,			None,								include_str!("./large/m/.manjaro.clml"),			false,	),
	( "manjaro_small",		Check::Is,			None,								include_str!("./small/m/.manjaro.clml"),			false,  ),
	( "maui",				Check::StartsWith,	Some("Maui"),						include_str!("./large/m/.maui.clml"),				false,	),
	( "mer",				Check::StartsWith,	Some("Mer"),						include_str!("./large/m/.mer.clml"),				false,	),
	( "minux",				Check::StartsWith,	Some("Minux"),						include_str!("./large/m/.minux.clml"),				false,	),
	( "mint",				Check::StartsWith,	Some("Linux Mint"),					include_str!("./large/m/.mint.clml"),				false,	),
	( "mint_small",			Check::Is,			None,								include_str!("./small/m/.mint.clml"),				false,	),
	( "mint_old",			Check::Is,			None,								include_str!("./old/m/.mint.clml"),					false,	),
	( "",					Check::StartsWith,	Some("LinuxMint"),					"@mint",											false,	),
	( "",					Check::StartsWith,	Some("mint"),						"@mint",											false,	),
	( "mx",					Check::StartsWith,	Some("MX"),							include_str!("./large/m/.mx.clml"),					false,	),
	( "mx_small",			Check::Is,			None,								include_str!("./small/m/.mx.clml"),					false,	),
	( "namib",				Check::StartsWith,	Some("Namib"),						include_str!("./large/n/.namib.clml"),				false,	),
	( "neptune",			Check::StartsWith,	Some("Neptune"),					include_str!("./large/n/.neptune.clml"),			false,	),
	( "netbsd",				Check::StartsWith,	Some("NetBSD"),						include_str!("./large/n/.netbsd.clml"),				false,	),
	( "netbsd_small",		Check::Is,			None,								include_str!("./small/n/.netbsd.clml"),				false,	),
	( "netrunner",			Check::StartsWith,	Some("Netrunner"),					include_str!("./large/n/.netrunner.clml"),			false,	),
	( "nitrux",				Check::StartsWith,	Some("Nitrux"),						include_str!("./large/n/.nitrux.clml"),				false,	),
	( "nixos",				Check::StartsWith,	Some("NixOS"),						include_str!("./large/n/.nixos.clml"),				false,	),
	( "nixos_small",		Check::Is,			None,								include_str!("./small/n/.nixos.clml"),				false,	),
	( "nurunner",			Check::StartsWith,	Some("Nurunner"),					include_str!("./large/n/.nurunner.clml"),			false,	),
	( "nutyx",				Check::StartsWith,	Some("NuTyX"),						include_str!("./large/n/.nutyx.clml"),				false,	),
	( "obrevenge",			Check::StartsWith,	Some("OBRevenge"),					include_str!("./large/o/.obrevenge.clml"),			false,	),
	( "openbsd",			Check::StartsWith,	Some("OpenBSD"),					include_str!("./large/o/.openbsd.clml"),			false,	),
	( "openbsd_small",		Check::Is,			None,								include_str!("./small/o/.openbsd.clml"),			false,	),
	( "openeuler",			Check::StartsWith,	Some("OpenEuler"),					include_str!("./large/o/.openeuler.clml"),			false,	),
	( "openindiana",		Check::StartsWith,	Some("OpenIndiana"),				include_str!("./large/o/.openindiana.clml"),		false,	),
	( "openmamba",			Check::StartsWith,	None,								include_str!("./large/o/.openmamba.clml"),			false,	),
	( "openmandriva",		Check::StartsWith,	Some("OpenMandrivia"),				include_str!("./large/o/.openmandriva.clml"),		false,	),
	( "openstage",			Check::StartsWith,	Some("OpenStage"),					include_str!("./large/o/.openstage.clml"),			false,	),
	( "openwrt",			Check::StartsWith,	Some("OpenWrt"),					include_str!("./large/o/.openwrt.clml"),			false,	),
	( "osmc",				Check::StartsWith,	Some("Open Source Media Center"),	include_str!("./large/o/.osmc.clml"),				false,	),
	( "oracle",				Check::StartsWith,	Some("Oracle"),						include_str!("./large/o/.oracle.clml"),				false,	),
	( "oselbrus",			Check::StartsWith,	Some("OS Elbrus"),					include_str!("./large/o/.oselbrus.clml"),			false,	),
	( "pacbsd",				Check::StartsWith,	Some("PacBSD"),						include_str!("./large/p/.pacbsd.clml"),				false,	),
	( "parabola",			Check::StartsWith,	Some("Parabola"),					include_str!("./large/p/.parabola.clml"),			false,	),
	( "parabola_small",		Check::Is,			None,								include_str!("./small/p/.parabola.clml"),			false,	),
	( "pardus",				Check::StartsWith,	Some("Pardus"),						include_str!("./large/p/.pardus.clml"),				false,	),
	( "parrot",				Check::StartsWith,	Some("Parrot"),						include_str!("./large/p/.parrot.clml"),				false,	),
	( "parsix",				Check::StartsWith,	Some("Parsix"),						include_str!("./large/p/.parsix.clml"),				false,	),
	( "pcbsd",				Check::StartsWith,	Some("PCBSD"),						include_str!("./large/p/.pcbsd.clml"),				false,	),
	( "pclinuxos",			Check::StartsWith,	Some("PCLinuxOS"),					include_str!("./large/p/.pclinuxos.clml"),			false,	),
	( "pentoo",				Check::StartsWith,	Some("Pentoo"),						include_str!("./large/p/.pentoo.clml"),				false,	),
	( "peppermint",			Check::StartsWith,	Some("Peppermint"),					include_str!("./large/p/.peppermint.clml"),			false,	),
	( "popos",				Check::StartsWith,	Some("Pop!_OS"),					include_str!("./large/p/.popos.clml"),				false,	),
	( "popos_small",		Check::Is,			None,								include_str!("./small/p/.popos.clml"),				false,	),
	( "",					Check::StartsWith,	Some("popos"),						"@popos",											false,	),
	( "",					Check::StartsWith,	Some("pop_os"),						"@popos",											false,	),
	( "porteus",			Check::StartsWith,	Some("Porteus"),					include_str!("./large/p/.porteus.clml"),			false,	),
	( "postmarketos",		Check::StartsWith,	Some("PostMarketOS"),				include_str!("./large/p/.postmarketos.clml"),		false,	),
	( "postmarketos_small",	Check::Is,			None,								include_str!("./small/p/.postmarketos.clml"),		false,	),
	( "proxmox",			Check::StartsWith,	Some("Proxmox"),					include_str!("./large/p/.proxmox.clml"),			false,	),
	( "puppy",				Check::StartsWith,	Some("Puppy"),						include_str!("./large/p/.puppy.clml"),				false,	),
	( "",					Check::StartsWith,	Some("Precise Puppy"),				"@puppy",											false,	),
	( "",					Check::StartsWith,	Some("Quirky Werewolf"),			"@puppy",											false,	),
	( "pureos",				Check::StartsWith,	Some("PureOS"),						include_str!("./large/p/.pureos.clml"),				false,	),
	( "pureos_small",		Check::Is,			None,								include_str!("./small/p/.pureos.clml"),				false,	),
	( "qubes",				Check::StartsWith,	Some("Qubes"),						include_str!("./large/q/.qubes.clml"),				false,	),
	( "radix",				Check::StartsWith,	Some("Radix"),						include_str!("./large/r/.radix.clml"),				false,	),
	( "raspbian",			Check::StartsWith,	Some("Raspbian"),					include_str!("./large/r/.raspbian.clml"),			false,	),
	( "raspbian_small",		Check::Is,			None,								include_str!("./small/r/.raspbian.clml"),			false,	),
	( "reborn",				Check::StartsWith,	Some("Reborn"),						include_str!("./large/r/.reborn.clml"),				false,	),
	( "redstar",			Check::StartsWith,	Some("Red Star"),					include_str!("./large/r/.redstar.clml"),			false,	),
	( "",					Check::StartsWith,	Some("RedStar"),					"@redstar",											false,	),
	( "redcore",			Check::StartsWith,	Some("Redcore"),					include_str!("./large/r/.redcore.clml"),			false,	),
	( "redhat",				Check::StartsWith,	Some("Redhat"),						include_str!("./large/r/.redhat.clml"),				false,	),
	( "redhat_old",			Check::Is,			None,								include_str!("./old/r/.redhat.clml"),				false,	),
	( "",					Check::StartsWith,	Some("Red Hat"),					"@redhat",											false,	),
	( "",					Check::StartsWith,	Some("rhel"),						"@redhat",											false,	),
	( "refracteddevuan",	Check::StartsWith,	Some("Refracted Devuan"),			include_str!("./large/r/.refracteddevuan.clml"),	false,	),
	( "",					Check::StartsWith,	Some("Refracted_Devuan"),			"@refracteddevuan",									false,	),
	( "regata",				Check::StartsWith,	Some("Regata"),						include_str!("./large/r/.regata.clml"),				false,	),
	( "regolith",			Check::StartsWith,	Some("Regolith"),					include_str!("./large/r/.regolith.clml"),			false,	),
	( "",					Check::StartsWith,	Some("RFRemix"),					"@fedora",											false,	),
	( "rosa",				Check::StartsWith,	Some("Rosa"),						include_str!("./large/r/.rosa.clml"),				false,	),
	// Continue here.
	( "",					Check::StartsWith,	Some("TrueOS"),						"@pcbsd",											false,	),
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
