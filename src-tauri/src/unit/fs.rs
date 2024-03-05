use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Mutex, time::Duration};

use log::debug;
use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};

use crate::api;

use lazy_static::lazy_static;

lazy_static! {
    //所有更改的文件
    static ref CHANGE_FILES: Mutex<Vec<PathBuf>> =Mutex::new(Vec::new());
}
static FILE_SYSTEM_WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None);

#[derive(Serialize, Deserialize, Clone)]
struct FilesChange {
    kind: String,
    files: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub ext: String,
    pub time: u64,
    pub isdir: bool,
}

pub fn get_file_info(path: &std::path::Path) -> Result<FileInfo, std::io::Error> {
    let meta = path.metadata()?;

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
        time: super::time::sys_time_to_seconds(time),
        isdir: isdir,
    })
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

pub fn data_dir(dir: Option<&str>) -> PathBuf {
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

pub fn watch_all_files() {
    debug!("enter watch_all_files");
    let drives = api::sys::get_root_dirs();

    let mut fs_watcher = FILE_SYSTEM_WATCHER.lock().unwrap();
    //确保是第一次监视
    if fs_watcher.is_some() {
        (*fs_watcher) = None;
    } else {
        //且只在第一次调用时启动监视线程
        std::thread::spawn(|| loop {
            std::thread::sleep(Duration::from_secs(3));
            let cf = CHANGE_FILES.lock();
            if cf.is_err() {
                continue;
            }
            let mut cf = cf.unwrap();
            crate::unit::file_db::update_file(&(*cf));
            (*cf).clear();
        });
    }
    let mut tmp_watcher = notify::recommended_watcher(move |res: Result<notify::Event, _>| {
        let cf = CHANGE_FILES.lock();
        if cf.is_err() {
            return;
        }
        let mut cf = cf.unwrap();
        if res.is_err() {
            return;
        }
        let res = res.unwrap();
        for p in res.paths {
            (*cf).push(p);
        }
    })
    .unwrap();
    for p in drives {
        tmp_watcher
            .watch(Path::new(&p), RecursiveMode::Recursive)
            .unwrap();
    }

    (*fs_watcher) = Some(tmp_watcher);
}
