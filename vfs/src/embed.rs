use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../assets"]
#[include = "logo/bfm.svg"]
pub struct Assets;

impl Assets {
    pub fn get_logo() -> Vec<u8> {
        Self::get("logo/bfm.svg").unwrap().data.to_vec()
    }
}
