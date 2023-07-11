pub(crate) mod ascii_art;
pub(crate) mod defaults;

pub(crate) static ANSI: &str = include_str!("./ansi.lua");
pub(crate) static PRINT: &str = include_str!("./print.lua");
pub(crate) static HELP: &str = include_str!("./.help.clml");
