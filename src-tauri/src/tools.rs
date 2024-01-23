use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub ext: String,
    pub time: u64,
    pub isdir: bool,
}

pub fn ensure_dir(dir: &str) -> bool {
    let p = std::path::Path::new(dir);
    if !p.exists() || !p.is_dir() {
        if let Err(_) = std::fs::create_dir_all(p) {
            return false;
        }
    }
    true
}

pub fn sys_time_to_seconds(time: std::time::SystemTime) -> u64 {
    let duration_since_epoch = time
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    return duration_since_epoch.as_secs();
}

pub fn get_data_dir(dir: Option<&str>) -> PathBuf {
    let data_path = dirs::data_dir().unwrap();
    let data_path = std::path::Path::new(&data_path);
    let path;
    if let None = dir {
        path = data_path.join("ombra");
    } else {
        path = data_path.join("ombra").join(dir.unwrap());
    }
    ensure_dir(path.to_str().unwrap());
    return path;
}

pub fn get_file_info(path: &std::path::Path) -> Result<FileInfo,std::io::Error> {
    let meta= path.metadata()?;

    let isdir = meta.is_dir();
    let time = meta.modified().unwrap();

    let name;
    let ext;
    if isdir {
        name = path
            .file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string();
        ext = String::new();
    } else {
        name = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        ext = path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
    }

    let parent_path = path
        .parent()
        .unwrap_or(&std::path::PathBuf::new())
        .to_string_lossy()
        .to_string();

    Ok(FileInfo {
        name: name,
        path: parent_path,
        ext: ext,
        time: sys_time_to_seconds(time),
        isdir: isdir,
    })
}
