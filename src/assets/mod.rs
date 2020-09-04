pub(crate) mod ascii_art;
pub(crate) mod defaults;

pub(crate) static ANSI: &'static str = include_str!("./ansi.lua");
pub(crate) static PRINT: &'static str = include_str!("./print.lua");
pub(crate) static HELP: &'static str = include_str!("./.help.clml");