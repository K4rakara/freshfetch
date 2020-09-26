use std::env;
use std::path::{ Path, PathBuf };

/// Mimics the functionality of the Linux `which` command.
///
/// Based on https://stackoverflow.com/a/37499032.
pub fn which<P: AsRef<Path>>(p: P) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|path| {
            let full_path = path.join(&p);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}

