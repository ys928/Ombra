use std::{path::Path, sync::Mutex};

use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};
use tauri::Window;

static WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None);

#[tauri::command]
pub fn watch_dir(w: Window, path: &str) {
    let mut watcher = WATCHER.lock().unwrap();
    if watcher.is_none() {
        let tmp_watcher = notify::recommended_watcher(move |res| match res {
            Ok(_e) => {
                let _ = w.emit("file_change", "change");
            }
            Err(_e) => {}
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
pub fn unwatch_dir(w: Window, path: &str) {
    let mut watcher = WATCHER.lock().unwrap();
    if watcher.is_none() {
        return;
    }
    watcher.as_mut().unwrap().unwatch(Path::new(path)).unwrap();
}
