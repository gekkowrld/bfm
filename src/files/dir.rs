use crate::files::file::File;
use std::{fs::DirEntry, path::PathBuf};
use url::Url;

#[derive(Debug)]
pub struct Directory {
    pub files: Vec<File>,
    pub path: Url,
}

impl Directory {
    pub fn new(path: &PathBuf) -> Directory {
        Directory {
            files: Vec::new(),
            path: Url::from_file_path(path).unwrap(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }
}

pub fn directory_content(directory: PathBuf) -> Directory {
    let mut dir = Directory::new(&directory);
    let entries = std::fs::read_dir(directory).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_url = if path.is_dir() {
            Url::from_directory_path(&path).unwrap()
        } else {
            Url::from_file_path(&path).unwrap()
        };
        let _file = File::new(file_id(&entry), file_url, path.clone());

        let file = match _file {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error: {} On File {:#?}", err, path);
                continue;
            }
        };
        dir.add_file(file);
    }
    dir
}

fn file_id(file_info: &DirEntry) -> String {
    file_info.file_name().to_str().unwrap().to_string()
}
