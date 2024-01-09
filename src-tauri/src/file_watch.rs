use std::{path::Path, sync::Mutex};

use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};
use tauri::Window;

static WATCHER: Mutex<Option<ReadDirectoryChangesWatcher>> = Mutex::new(None);

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
                    let _ = w.emit("file_watch", "create");
                }
                notify::EventKind::Modify(_) => {
                    let _ = w.emit("file_watch", "modify");
                }
                notify::EventKind::Remove(_) => {
                    let _ = w.emit("file_watch", "remove");
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
    watcher.as_mut().unwrap().unwatch(Path::new(path)).unwrap();
}
