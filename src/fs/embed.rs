use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "icons/*.svg"]
#[include = "themes/*.toml"]
#[include = "themes/*.lua"]
pub struct Assets;

impl Assets {
    pub fn get_icon(name: &str) -> Vec<u8> {
        Self::get(format!("icons/{name}.svg").as_str())
            .unwrap()
            .data
            .to_vec()
    }

    pub fn get_theme(name: &str) -> Vec<u8> {
        Self::get(format!("themes/{name}.toml").as_str())
            .unwrap()
            .data
            .to_vec()
    }

    pub fn get_lua(name: &str) -> Vec<u8> {
        Self::get(format!("themes/{name}.lua").as_str())
            .unwrap()
            .data
            .to_vec()
    }
}
