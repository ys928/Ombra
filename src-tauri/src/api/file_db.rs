use std::{
    collections::LinkedList,
    sync::atomic::{AtomicBool, Ordering},
};

use log::debug;
use serde::{Deserialize, Serialize};
use tauri::Window;
use walkdir::WalkDir;

use crate::unit::{self, fs::FileInfo};

static IS_IN_WALKDIR: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskProgress {
    status: String, //状态
    data: String,   //传送数据
}

#[tauri::command(async)]
pub async fn walk_all_files(w: Window) {
    debug!("enter walk_all_files");
    if IS_IN_WALKDIR.load(Ordering::Relaxed) {
        debug!("Repeat call walk_all_files");
        return;
    }

    IS_IN_WALKDIR.store(true, Ordering::Relaxed);

    let drives = crate::api::sys::get_root_dirs();

    unit::file_db::init(); //重置缓存文件

    let (se, re) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        for d in drives {
            for entry in WalkDir::new(d).into_iter().filter_map(|e| e.ok()) {
                se.send(entry).unwrap();
            }
        }
    });

    let mut files: LinkedList<unit::fs::FileInfo> = LinkedList::new();
    for file in re {
        let fi = unit::fs::get_file_info(file.path());
        if fi.is_err() {
            continue;
        }
        files.push_back(fi.unwrap());

        if files.len() % 10000 != 0 {
            continue;
        }
        let _ = w.emit(
            "walk_files_process",
            TaskProgress {
                status: "walking".to_string(),
                data: files.len().to_string(),
            },
        );
    }
    //发送最后的结果
    let _ = w.emit(
        "walk_files_process",
        TaskProgress {
            status: "begin_save".to_string(),
            data: files.len().to_string(),
        },
    );
    //插入数据库中
    let all_file_num = files.len();
    unit::file_db::insert_files(files);
    let _ = w.emit(
        "walk_files_process",
        TaskProgress {
            status: "end".to_string(),
            data: all_file_num.to_string(),
        },
    );
    IS_IN_WALKDIR.store(false, Ordering::Relaxed);
    //遍历完成后，启动监视
    unit::fs::watch_all_files();
    debug!("leave walk_all_files");
}

#[tauri::command(async)]
pub async fn search_file(
    name: String,
    ext: String,
    mode: String,
    limit: i32,
    offset: i32,
) -> Vec<FileInfo> {
    let mut ret = Vec::new();
    if mode == "normal" {
        ret = unit::file_db::search_file(&name, &ext, limit, offset);
    } else if mode == "exact" {
        ret = unit::file_db::search_file_as_exact(&name, &ext, limit, offset);
    }
    ret
}

#[tauri::command]
pub fn get_file_catch_info() -> i32 {
    let fc = crate::unit::file_db::get_db_path();
    if fc.exists() {
        return crate::unit::file_db::get_file_num();
    } else {
        return 0;
    }
}
