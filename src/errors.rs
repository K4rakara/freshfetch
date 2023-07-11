use std::process::exit;

pub(crate) static LUA: &str = "A Lua error occurred. Details:\n";
pub(crate) static CMD: (&str, &str) = ("An error occurred while executing \"", "\". Details:\n");
pub(crate) static PARSE: (&str, &str, &str) = (
    "An error occurred while parsing \"",
    "\" into a \"",
    "\". Details:\n",
);

pub(crate) mod io {
    pub(crate) static READ: (&str, &str) = (
        "An I/O error occurred while trying to read from \"",
        "\". Details:\n",
    );
}

pub(crate) fn handle(msg: &str) {
    println!("\u{001b}[38;5;1mError.\u{001b}[0m\n{msg}",);
    exit(1);
}
