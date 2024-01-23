use std::{path::{Path, PathBuf}, sync::Mutex, time::Duration};

use log::debug;
use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tauri::Window;

use crate::{file_catch, winsys};

static WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None);

//所有更改的文件
static CHANGE_FILES: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

#[derive(Serialize, Deserialize, Clone)]
struct FilesChange {
    kind: String,
    files: Vec<String>,
}

#[tauri::command]
pub fn watch_dir(w: Window, path: &str) {
    let mut watcher = WATCHER.lock().unwrap();
    if watcher.is_none() {
        let tmp_watcher = notify::recommended_watcher(move |res: Result<notify::Event, _>| {
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
        *watcher = Some(tmp_watcher);
    }

    watcher
        .as_mut()
        .unwrap()
        .watch(Path::new(path), RecursiveMode::Recursive)
        .unwrap();
}

#[tauri::command]
pub fn unwatch_dir(path: &str) {
    let mut watcher = WATCHER.lock().unwrap();
    if watcher.is_none() {
        return;
    }
    let _ = watcher.as_mut().unwrap().unwatch(Path::new(path));
}

static FILE_SYSTEM_WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None);

pub fn watch_all_files() {
    debug!("enter watch_all_files");
    let drives = winsys::get_logical_drives().unwrap();

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
            file_catch::update_file(&(*cf));
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
        let res=res.unwrap();
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
