use anyhow::{Context, Error, Result};
use std::path::PathBuf;

pub fn read_file(path: &PathBuf) -> Result<String, Error> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file: {}", path.display()));
    content
}
