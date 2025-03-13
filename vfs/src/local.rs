use std::fs::File;
use std::io::{BufReader, Read, Result};

use crate::FileInformation;
use crate::{DirectoryInformation, FileInfo};

pub fn read_file(filename: &str) -> Result<FileInformation> {
    let file = File::open(filename)?;
    let mut content = vec![];
    let mut buf_reader = BufReader::new(&file);

    buf_reader.read_to_end(&mut content)?;
    let file_info = FileInformation {
        file: FileInfo {
            name: filename.to_string(),
            size: file.metadata()?.len(),
            is_dir: file.metadata()?.is_dir(),
            is_symlink: file.metadata()?.file_type().is_symlink(),
            is_ftp: false,
        },
        content,
    };
    Ok(file_info)
}

pub fn list_files(path: &str) -> Result<DirectoryInformation> {
    let mut files = vec![];
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        files.push(FileInfo {
            name: path.to_str().unwrap().to_string(),
            size: path.metadata()?.len(),
            is_dir: path.is_dir(),
            is_symlink: path.is_symlink(),
            is_ftp: false,
        });
    }
    Ok(DirectoryInformation {
        files,
        name: path.to_string(),
    })
}
