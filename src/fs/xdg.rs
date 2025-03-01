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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home() {
        assert!(home().is_some());
    }

    #[test]
    fn test_config_dir() {
        assert!(config_dir().is_some());
    }

    #[test]
    fn test_config_file() {
        assert!(config_file().is_some());
    }
}
