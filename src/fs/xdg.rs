use dirs::{self, home_dir};
use std::path::PathBuf;

pub fn home() -> Option<PathBuf> {
    home_dir()
}

pub fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|mut p| {
        p.push("bfm");
        p
    })
}

pub fn config_file() -> Option<PathBuf> {
    config_dir().map(|mut p| {
        p.push("config.toml");
        p
    })
}
