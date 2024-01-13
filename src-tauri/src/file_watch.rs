use std::{path::Path, sync::Mutex, time::Duration};

use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tauri::Window;

use crate::file_catch;

static WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None);

//所有更改的文件
static CHANGE_FILES: Mutex<Vec<FileChange>> = Mutex::new(Vec::new());

#[derive(Serialize, Deserialize, Clone)]
struct FilesChange {
    kind: String,
    files: Vec<String>,
}

pub struct FileChange {
    pub kind: String,
    pub path: String,
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

pub fn watch_dir_to_file_catch(path: Vec<String>) {
    let mut fs_watcher=FILE_SYSTEM_WATCHER.lock().unwrap();
    //确保是第一次监视，且只在第一次调用时启动监视线程
    if fs_watcher.is_some(){
        (*fs_watcher)=None;
    }else{
        std::thread::spawn(|| loop {
            std::thread::sleep(Duration::from_secs(8));
            let mut cf = CHANGE_FILES.lock().unwrap();
            file_catch::update_file(&(*cf));
            (*cf).clear();
        });
    }
    let mut tmp_watcher = notify::recommended_watcher(move |res: Result<notify::Event, _>| {
        if res.is_err() {
            return;
        }
        let res = res.unwrap();
        match res.kind {
            notify::EventKind::Create(_) => {
                let mut cf = CHANGE_FILES.lock().unwrap();
                for p in res.paths {
                    let mut test = true;
                    for f in (*cf).iter() {
                        if f.path == p.to_str().unwrap() && f.kind == "create" {
                            test = false;
                            break;
                        }
                    }
                    if test {
                        (*cf).push(FileChange {
                            kind: "create".to_string(),
                            path: p.to_string_lossy().to_string(),
                        });
                    }
                }
            }
            notify::EventKind::Modify(_) => {
                let mut cf = CHANGE_FILES.lock().unwrap();
                for p in res.paths {
                    let mut test = true;
                    for f in (*cf).iter() {
                        if f.path == p.to_str().unwrap() && f.kind == "modify" {
                            test = false;
                            break;
                        }
                    }
                    if test {
                        (*cf).push(FileChange {
                            kind: "modify".to_string(),
                            path: p.to_string_lossy().to_string(),
                        });
                    }
                }
            }
            notify::EventKind::Remove(_) => {
                let mut cf = CHANGE_FILES.lock().unwrap();
                for p in res.paths {
                    let mut test = true;
                    for f in (*cf).iter() {
                        if f.path == p.to_str().unwrap() && f.kind == "remove" {
                            test = false;
                            break;
                        }
                    }
                    if test {
                        (*cf).push(FileChange {
                            kind: "remove".to_string(),
                            path: p.to_string_lossy().to_string(),
                        });
                    }
                }
            }
            _ => {}
        }
    })
    .unwrap();
    for p in path {
        tmp_watcher
            .watch(Path::new(&p), RecursiveMode::Recursive)
            .unwrap();
    }

    (*fs_watcher)=Some(tmp_watcher);
}
