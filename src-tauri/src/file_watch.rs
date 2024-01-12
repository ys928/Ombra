use std::{path::Path, sync::Mutex};

use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tauri::Window;

static WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None);

#[derive(Serialize, Deserialize, Clone)]
struct FileChange {
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
                        FileChange {
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
                        FileChange {
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
                        FileChange {
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
