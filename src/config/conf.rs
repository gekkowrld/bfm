pub struct Config {
    pub column_width: ColumnWidth,
}

#[derive(Default)]
pub struct ColumnWidth {
    pub name: f32,
    pub type_: f32,
    pub size: f32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            column_width: ColumnWidth {
                name: 1.0,
                type_: 1.0,
                size: 1.0,
            },
        }
    }

    pub fn set_column_width(&mut self, width: &ColumnWidth) {
        unsafe {
            std::env::set_var("BFM_COLUMN_WIDTH_NAME", width.name.to_string());
            std::env::set_var("BFM_COLUMN_WIDTH_SIZE", width.size.to_string());
            std::env::set_var("BFM_COLUMN_WIDTH_TYPE", width.type_.to_string());
        }
    }

    pub fn get_column_width(&self) -> ColumnWidth {
        let name: f32 = std::env::var("BFM_COLUMN_WIDTH_NAME")
            .unwrap_or("1.0".to_string())
            .parse()
            .unwrap();

        let size: f32 = std::env::var("BFM_COLUMN_WIDTH_SIZE")
            .unwrap_or("1.0".to_string())
            .parse()
            .unwrap();

        let type_: f32 = std::env::var("BFM_COLUMN_WIDTH_TYPE")
            .unwrap_or("1.0".to_string())
            .parse()
            .unwrap();

        ColumnWidth { name, size, type_ }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
