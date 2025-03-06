use rayon::prelude::*;
use std::fs::DirEntry;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct File {
    pub id: String,
    pub path: PathBuf,
    pub file: FileInformation,
}

#[derive(Debug, Clone)]
pub struct FileInformation {
    pub metadata: std::fs::Metadata,
}

impl File {
    pub fn new(id: String, path: PathBuf) -> Result<File, std::io::Error> {
        let file = std::fs::File::open(&path)?;
        let metadata = file.metadata()?;
        Ok(File {
            id,
            path,
            file: FileInformation { metadata },
        })
    }
}

pub fn file_content(file_path: &PathBuf) -> Result<String, std::io::Error> {
    let file = std::fs::File::open(file_path)?;
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

pub fn directory_content(directory: &PathBuf) -> std::io::Result<Directory> {
    let mut dir = Directory::new(directory);

    let entries = std::fs::read_dir(directory)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let files: Vec<File> = entries
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let file_result = File::new(file_id(entry), path.clone());
            match file_result {
                Ok(file) => Some(file),
                Err(err) => {
                    eprintln!("Error: {} On File {:#?}", err, path);
                    None
                }
            }
        })
        .collect();

    dir.files = files;
    Ok(dir)
}

fn file_id(file_info: &DirEntry) -> String {
    file_info.file_name().to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File as StdFile;
    use std::io::Write;

    fn create_temp_file(content: &str) {
        // Create temp dir
        let _ = std::fs::create_dir_all("/tmp/test_files");
        let file_path = PathBuf::from("/tmp/test_files/test.txt");
        let mut file = StdFile::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_file_content() {
        let file_path = PathBuf::from("/tmp/test_files/test.txt");
        create_temp_file("Hello, World!");
        let content = file_content(&file_path).unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_directory_content() {
        let dir_path = PathBuf::from("/tmp/test_files");
        let dir = directory_content(&dir_path).unwrap();
        assert_eq!(dir.files().len(), 1);
        assert_eq!(dir.files()[0].id, "test.txt");
    }
}
