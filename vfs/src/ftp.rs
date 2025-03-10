use std::{io::Result, path::Path};
use suppaftp::{FtpStream, types::FileType};

use crate::{DirectoryInformation, FileInformation};

pub fn connect(address: &str, username: &str, password: &str) -> Option<FtpStream> {
    let mut stream: FtpStream = FtpStream::connect(addres_port(address).as_str()).ok()?;
    stream.login(username, password).ok()?;

    stream.transfer_type(FileType::Binary).ok()?;

    Some(stream)
}

fn addres_port(address: &str) -> String {
    let mut parts = address.split(':');
    let address = parts.next().unwrap();
    let port = parts.next().unwrap_or("21");

    format!("{}:{}", address, port)
}

pub fn list_files(stream: &mut FtpStream, path: &str) -> Result<DirectoryInformation> {
    let list = stream.list(Some(path));

    let list = match list {
        Ok(list) => list,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to list files",
            ));
        }
    };

    Ok(DirectoryInformation::new(
        path.to_string(),
        stream_list_to_file_info(path, list),
    ))
}

fn stream_list_to_file_info(parent: &str, list: Vec<String>) -> Vec<crate::FileInfo> {
    list.iter()
        .map(|file| stream_file_to_file_info(parent, file.to_string()))
        .collect()
}

fn stream_file_to_file_info(parent: &str, file: String) -> crate::FileInfo {
    // The format of the file is:
    // drwxr-xr-x  2 user group 4096 Mar  7  2019 file
    // 'Permissions', 'Links', 'Owner', 'Group', 'Size', 'Month', 'Day', 'Time', 'Name'
    // We have to split it to get the relevant information

    let parts: Vec<&str> = file.split_whitespace().collect();
    let name = parts[8].to_string();
    let is_dir = parts[0].starts_with('d');
    let size = parts[4].parse().unwrap_or(0);

    let name = Path::new(parent).join(name).to_str().unwrap().to_string();

    crate::FileInfo {
        name,
        size,
        is_dir,
        is_symlink: false,
        is_ftp: true,
    }
}

pub fn read_file(stream: &mut FtpStream, filename: &str) -> Result<FileInformation> {
    let buffer = stream.retr_as_buffer(filename).unwrap();
    let content = String::from_utf8(buffer.into_inner()).ok().unwrap();

    Ok(FileInformation {
        content,
        file: crate::FileInfo {
            name: filename.to_string(),
            size: 0,
            is_dir: false,
            is_symlink: false,
            is_ftp: true,
        },
    })
}
