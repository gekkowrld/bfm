use std::fs::DirEntry;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

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

    pub fn file_content(&mut self) -> Result<String, std::io::Error> {
        extract_content(&self.file)
    }
}

pub fn file_content(file_path: PathBuf) -> Result<String, std::io::Error> {
    let file = std::fs::File::open(&file_path)?;
    extract_content(&file)
}

fn extract_content(file: &std::fs::File) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
pub struct Directory {
    pub files: Vec<File>,
    pub path: PathBuf,
}

impl Directory {
    pub fn new(path: &Path) -> Directory {
        Directory {
            files: Vec::new(),
            path: path.to_path_buf(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }
}

pub fn directory_content(directory: PathBuf) -> std::io::Result<Directory> {
    let mut dir = Directory::new(&directory);
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
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
    Ok(dir)
}

fn file_id(file_info: &DirEntry) -> String {
    file_info.file_name().to_str().unwrap().to_string()
}

pub fn path_to_string(path: &Path) -> String {
    path.to_str().unwrap().to_string()
}
