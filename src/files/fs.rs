use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug)]
pub struct File {
    pub id: String,
    pub path: PathBuf,
    pub file: std::fs::File,
}

impl File {
    pub fn new(id: String, path: PathBuf) -> Result<File, std::io::Error> {
        let file = std::fs::File::open(&path)?;
        Ok(File { id, path, file })
    }
}

pub fn file_content(path: PathBuf) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

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
        let _file = File::new(file_id(&entry), path.clone());

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

pub fn path_to_string(path: &Path) -> String {
    path.to_str().unwrap().to_string()
}
