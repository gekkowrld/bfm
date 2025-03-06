use std::time::SystemTime;

use crate::fs::file::File;
use crate::ui::files::FileColumn;
use crate::ui::files::FileColumnInformation;
use human_repr::HumanCount;

pub fn box_display(file_info: &File) -> FileColumn {
    let file_name: String = match file_info.path.file_name() {
        Some(name) => name.to_str().unwrap_or("Unnamed File").to_string(),
        None => "Unnamed File".to_string(),
    };

    let diff = timediff::TimeDiff::to_diff_duration(
        SystemTime::now()
            .duration_since(file_info.file.metadata.modified().unwrap())
            .unwrap(),
    )
    .parse()
    .unwrap();

    //icon(file_info.path.is_dir());

    FileColumn {
        id: file_info.path.to_str().unwrap().to_string(),
        information: FileColumnInformation {
            filename: file_name,
            file_type: file_type(file_info),
            file_size: file_info
                .file
                .metadata
                .len()
                .human_count_bytes()
                .to_string(),
            modified: diff,
            path: file_info.path.clone(),
        },
    }
}

fn file_type(file_info: &File) -> String {
    if file_info.path.is_dir() {
        "Directory".to_string()
    } else {
        "File".to_string()
    }
}
