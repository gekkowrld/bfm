use crate::fs::file::Directory;
use crate::ui::file_container::box_display;
use crate::ui::files::{FileColumn, FilesUITree};

pub fn directory_information(directory: &Directory) -> FilesUITree {
    let children = directory.files.iter().map(box_display);

    let mut children_info: Vec<FileColumn> = Vec::new();

    for child in children {
        children_info.push(child);
    }

    FilesUITree {
        id: directory.path.to_str().unwrap().to_string(),
        files_container: children_info,
    }
}
