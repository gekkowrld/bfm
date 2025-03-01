use crate::fs::xdg;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub column_width: ColumnWidth,
    last_path: Option<String>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct ColumnWidth {
    pub name: f32,
    pub type_: f32,
    pub size: f32,
}

impl Config {
    pub fn new() -> Self {
        // Check if a config file exists, if not create one
        // If it does exist, read the file and set the values
        // If the file is empty, set the default values

        let file_content = std::fs::read_to_string(xdg::config_file().unwrap());

        match file_content {
            Ok(content) => {
                let config: Config = toml::from_str(&content).unwrap();
                config
            }
            Err(_) => Self::create_new_conf(),
        }
    }

    fn create_new_conf() -> Config {
        let config: Config = Self::default_config();
        let config_string = toml::to_string(&config).unwrap();

        // Create parent directory if it doesn't exist
        std::fs::create_dir_all(xdg::config_dir().unwrap()).unwrap();

        std::fs::write(xdg::config_file().unwrap(), config_string).unwrap();

        config
    }

    pub fn default_config() -> Self {
        Self {
            column_width: ColumnWidth {
                name: 50.0,
                type_: 50.0,
                size: 50.0,
            },
            last_path: None,
        }
    }

    pub fn set_column_width(&mut self, width: &ColumnWidth) {
        self.column_width = ColumnWidth {
            name: width.name,
            type_: width.type_,
            size: width.size,
        };
        let config_string = toml::to_string(&self).unwrap();
        std::fs::write(xdg::config_file().unwrap(), config_string).unwrap();
    }

    pub fn set_last_path(&mut self, path: &str) {
        self.last_path = Some(path.to_string());
        let config_string = toml::to_string(&self).unwrap();
        std::fs::write(xdg::config_file().unwrap(), config_string).unwrap();
    }

    pub fn get_column_width(&self) -> ColumnWidth {
        ColumnWidth {
            name: self.column_width.name,
            type_: self.column_width.type_,
            size: self.column_width.size,
        }
    }

    pub fn get_last_path(&self) -> Option<String> {
        self.last_path.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default_config();
        assert_eq!(config.column_width.name, 50.0);
        assert_eq!(config.column_width.type_, 50.0);
        assert_eq!(config.column_width.size, 50.0);
        assert_eq!(config.last_path, None);
    }

    #[test]
    fn test_set_column_width() {
        let mut config = Config::default_config();
        let width = ColumnWidth {
            name: 100.0,
            type_: 100.0,
            size: 100.0,
        };
        config.set_column_width(&width);
        assert_eq!(config.column_width.name, 100.0);
        assert_eq!(config.column_width.type_, 100.0);
        assert_eq!(config.column_width.size, 100.0);
    }

    #[test]
    fn test_set_last_path() {
        let mut config = Config::default_config();
        config.set_last_path("/home/user");
        assert_eq!(config.last_path, Some("/home/user".to_string()));
    }

    #[test]
    fn test_get_column_width() {
        let config = Config::default_config();
        let width = config.get_column_width();
        assert_eq!(width.name, 50.0);
        assert_eq!(width.type_, 50.0);
        assert_eq!(width.size, 50.0);
    }

    #[test]
    fn test_get_last_path() {
        let config = Config::default_config();
        let path = config.get_last_path();
        assert_eq!(path, None);
    }
}
