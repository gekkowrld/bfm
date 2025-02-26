use url;

#[derive(Debug)]
pub struct File {
    pub id: String,
    pub url: url::Url,
    pub is_dir: bool,
    pub file: std::fs::File,
}

impl File {
    pub fn new(
        id: String,
        url: url::Url,
        path: std::path::PathBuf,
    ) -> Result<File, std::io::Error> {
        let file = std::fs::File::open(&path)?;
        Ok(File {
            id,
            url,
            is_dir: path.is_dir(),
            file,
        })
    }
}
