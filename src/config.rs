use crate::{Error, Result};
use std::{collections::HashSet, fs, path::Path, sync::OnceLock};

static INSTANCE: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub wordlist: HashSet<Vec<u8>>,
}

pub fn init(wordlist_file: &Path) -> Result<()> {
    let content = fs::read(wordlist_file).expect("failed to read the wordlist file");
    let wordlist = content
        .split(|b| *b == b'\n')
        .map(|line| line.trim_ascii().to_ascii_lowercase().to_vec())
        .filter(|line| !line.is_empty())
        .collect();
    INSTANCE
        .set(Config { wordlist })
        .map_err(|_| Error::ConfigAlreadyInitialized)
}

pub fn config() -> &'static Config {
    INSTANCE
        .get()
        .expect("Fatal - config() called before init_config()")
}
