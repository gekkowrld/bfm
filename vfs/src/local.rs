use std::fs::File;
use std::io::{BufReader, Read, Result};

use crate::FileInformation;

pub fn read_file(filename: &str) -> Result<FileInformation> {
    let file = File::open(filename)?;
    let mut content = String::new();
    let mut buf_reader = BufReader::new(&file);
    buf_reader.read_to_string(&mut content)?;
    Ok(FileInformation { file, content })
}

pub fn list_files(path: &str) -> Result<Vec<String>> {
    let mut files = vec![];
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path = match path.to_str() {
            Some(p) => p.to_string(),
            None => continue,
        };
        files.push(path);
    }
    Ok(files)
}
