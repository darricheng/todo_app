use std::fs::canonicalize;
use std::path::PathBuf;

pub fn generate_path() -> PathBuf {
    canonicalize("../to_do_state.json").unwrap()
}
