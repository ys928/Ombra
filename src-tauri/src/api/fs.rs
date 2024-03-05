use log::error;
use serde::{Deserialize, Serialize};
use tauri::Window;

use crate::unit::{self, fs::FileInfo};
use lazy_static::lazy_static;
use notify::ReadDirectoryChangesWatcher;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct FilesChange {
    kind: String,
    files: Vec<String>,
}

lazy_static! {
    static ref WATCHER_FILES: Mutex<HashMap<String, ReadDirectoryChangesWatcher>> =
        Mutex::new(HashMap::new());
}

#[tauri::command]
pub fn is_dir(path: &str) -> bool {
    let p = std::path::Path::new(path);
    if p.is_dir() {
        return true;
    }
    return false;
}

#[tauri::command(async)]
pub async fn walk_dir(path: String) -> Vec<FileInfo> {
    let mut files_list = Vec::new();
    let dir_entry = std::fs::read_dir(path.clone());
    if dir_entry.is_err(){
        error!("Error reading directory {}，error：{}", &path,dir_entry.unwrap_err());
        return files_list;
    }
    for entry in dir_entry.unwrap() {
        if entry.is_err() {
            continue;
        }
        let entry = entry.unwrap();

        let fi = unit::fs::get_file_info(entry.path().as_path());
        if fi.is_err() {
            continue;
        }
        files_list.push(fi.unwrap());
    }
    files_list
}

#[tauri::command]
pub fn watch_dir(w: Window, path: String) {
    let mut watcher_files = WATCHER_FILES.lock().unwrap();

    let watcher = notify::recommended_watcher(move |res: Result<notify::Event, _>| {
        if res.is_err() {
            return;
        }
        let res = res.unwrap();
        match res.kind {
            notify::EventKind::Create(_) => {
                let mut files = Vec::new();
                for f in res.paths {
                    files.push(f.to_string_lossy().to_string());
                }
                let _ = w.emit(
                    "file_watch",
                    FilesChange {
                        kind: "create".to_string(),
                        files: files,
                    },
                );
            }
            notify::EventKind::Modify(_) => {
                let mut files = Vec::new();
                for f in res.paths {
                    files.push(f.to_string_lossy().to_string());
                }
                let _ = w.emit(
                    "file_watch",
                    FilesChange {
                        kind: "modify".to_string(),
                        files: files,
                    },
                );
            }
            notify::EventKind::Remove(_) => {
                let mut files = Vec::new();
                for f in res.paths {
                    files.push(f.to_string_lossy().to_string());
                }
                let _ = w.emit(
                    "file_watch",
                    FilesChange {
                        kind: "remove".to_string(),
                        files: files,
                    },
                );
            }
            _ => {}
        }
    })
    .unwrap();

    watcher_files.insert(path, watcher);
}

#[tauri::command]
pub fn unwatch_dir(path: &str) {
    let mut watcher_files = WATCHER_FILES.lock().unwrap();
    watcher_files.remove(path);
}
