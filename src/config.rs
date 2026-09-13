use crate::{Error, Result};
use std::{collections::HashSet, fs, path::Path, sync::OnceLock};

static INSTANCE: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub wordlist: HashSet<Box<[u8]>>,
}

pub fn init(wordlist_file: &Path) -> Result<()> {
    let content = fs::read(wordlist_file).map_err(|source| Error::FileRead {
        path: wordlist_file.to_path_buf(),
        source,
    })?;

    let wordlist = content
        .split(|b| *b == b'\n')
        .map(|line| line.trim_ascii().to_ascii_lowercase().into_boxed_slice())
        .filter(|line| !line.is_empty())
        .collect();
    INSTANCE
        .set(Config { wordlist })
        .map_err(|_| Error::ConfigAlreadyInitialized)
}

pub fn config() -> &'static Config {
    INSTANCE
        .get()
        .expect("config::init() must be called before config()")
}

#[cfg(test)]
pub(crate) fn init_for_tests() {
    let wordlist = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/words");
    match init(&wordlist) {
        Ok(()) | Err(Error::ConfigAlreadyInitialized) => {}
        Err(err) => panic!("test wordlist should load: {err}"),
    }
}
