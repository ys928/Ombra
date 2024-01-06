#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::api::dialog;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Window};
use walkdir::WalkDir;
mod tools;
mod file_watch;

#[derive(Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub time: u64,
    pub ftype: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskProgress {
    status: String, //状态
    data: String,   //传送数据
}
#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

mod file_catch;
mod winsys;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let start_file_catch = CustomMenuItem::new("file_catch".to_string(), "文件缓存");
    let tray_menu = SystemTrayMenu::new()
        .add_item(start_file_catch)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "file_catch" => {
                    let window = app.get_window("MainWindow").unwrap();
                    walk_all_files(window);
                }
                _ => {}
            },
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("MainWindow");
                if window.is_none() {
                    return;
                }
                let w = window.unwrap();
                w.show().unwrap();
                w.set_focus().unwrap();
            }
            _ => {}
        })
        .setup(move |app| {
            let window = app.get_window("MainWindow").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            window_shadows::set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .invoke_handler(tauri::generate_handler![
            winsys::open_web_url,
            winsys::get_all_app,
            winsys::cmd_exec,
            winsys::get_explorer_show_path,
            file_watch::watch_dir,
            file_watch::unwatch_dir,
            walk_all_files,
            search_file,
            get_file_catch_info,
            shadow_window,
            dir_or_file,
            walk_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn shadow_window(w: Window) {
    #[cfg(any(windows, target_os = "macos"))]
    window_shadows::set_shadow(&w, true).expect("Unsupported platform!");
}
static IS_IN_WALKDIR: Mutex<bool> = Mutex::new(false);

#[tauri::command]
fn walk_all_files(w: Window) {
    let mut is_in_walkdir = IS_IN_WALKDIR.lock().unwrap();
    if *is_in_walkdir {
        dialog::message(Some(&w), "提示", "请勿重复操作！");
        return;
    }
    *is_in_walkdir = true;
    //单开一个线程遍历所有文件
    std::thread::spawn(move || {
        let drives = winsys::get_logical_drives().unwrap();

        file_catch::init(true); //重置缓存文件

        let (se, re) = std::sync::mpsc::channel();

        for d in drives {
            let t_se = se.clone();
            //每个盘都单开一个线程遍历，提高速度
            std::thread::spawn(move || {
                for entry in WalkDir::new(d).into_iter().filter_map(|e| e.ok()) {
                    let meta = entry.metadata().unwrap();
                    let name = entry.file_name().to_string_lossy().to_string();
                    let time = meta.modified().unwrap();
                    let parent = entry.path().parent();
                    let path;
                    if let None = parent {
                        path = "".to_string();
                    } else {
                        path = parent.unwrap().to_string_lossy().to_string();
                    }
                    let ftype;
                    if meta.is_dir() {
                        ftype = 2;
                    } else if meta.is_file() {
                        ftype = 1;
                    } else {
                        ftype = 0;
                    }
                    if ftype == 0 {
                        continue;
                    }
                    t_se.send(FileInfo {
                        name: name,
                        path: path,
                        time: tools::sys_time_to_seconds(time),
                        ftype: ftype,
                    })
                    .unwrap();
                }
            });
        }
        drop(se);
        let mut files: Vec<FileInfo> = Vec::new();
        for file in re {
            files.push(file);

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
        file_catch::insert_files(files);
        let _ = w.emit(
            "walk_files_process",
            TaskProgress {
                status: "end".to_string(),
                data: all_file_num.to_string(),
            },
        );
        let mut is_in_walkdir = IS_IN_WALKDIR.lock().unwrap();
        *is_in_walkdir = false;
    });
}

#[tauri::command]
fn search_file(w: Window, name: String, mode: String, limit: i32, offset: i32) {
    std::thread::spawn(move || {
        if mode == "normal" {
            let ret = file_catch::search_file(&name, limit, offset);
            let _ = w.emit("search_file_result", ret);
        } else if mode == "regex" {
            let ret = file_catch::search_file_as_regex(&name, limit, offset);
            let _ = w.emit("search_file_result", ret);
        } else if mode == "whole_word" {
            let ret = file_catch::search_file_as_whole_word(&name, limit, offset);
            let _ = w.emit("search_file_result", ret);
        }
    });
}

#[derive(Serialize, Deserialize)]
struct FileCatchInfo {
    is_exist: bool,
    time: u64,
    file_num: i32,
}

#[tauri::command]
fn get_file_catch_info() -> FileCatchInfo {
    let path = tools::get_data_dir(None);
    let fc = std::path::Path::new(&path).join("file_catch.db");
    if fc.exists() {
        let f = fc.metadata().unwrap();
        let time = f.modified().unwrap();
        return FileCatchInfo {
            is_exist: true,
            time: tools::sys_time_to_seconds(time),
            file_num: file_catch::get_file_num(),
        };
    } else {
        return FileCatchInfo {
            is_exist: false,
            time: 0,
            file_num: 0,
        };
    }
}

#[tauri::command]
fn dir_or_file(path: &str) -> String {
    let p = std::path::Path::new(path);
    if p.is_dir() {
        return "dir".to_string();
    } else if p.is_file() {
        return "file".to_string();
    }
    return "error".to_string();
}

#[tauri::command]
fn walk_dir(w: Window, path: String, level: usize) {
    std::thread::spawn(move || {
        let mut files_list = Vec::new();
        let walk;
        if level == 0 {
            walk = WalkDir::new(path);
        } else {
            walk = WalkDir::new(path).max_depth(level);
        }
        let mut f = true;
        for entry in walk.into_iter().filter_map(|e| e.ok()) {
            if f {
                f = false;
                continue;
            }
            let meta = entry.metadata().unwrap();
            let name = entry.file_name().to_string_lossy().to_string();
            let time = meta.modified().unwrap();
            let parent = entry.path().parent();
            let path;
            if let None = parent {
                path = "".to_string();
            } else {
                path = parent.unwrap().to_string_lossy().to_string();
            }
            let ftype;
            if meta.is_dir() {
                ftype = 2;
            } else if meta.is_file() {
                ftype = 1;
            } else {
                ftype = 0;
            }
            if ftype == 0 {
                continue;
            }
            files_list.push(FileInfo {
                name: name,
                path: path,
                time: tools::sys_time_to_seconds(time),
                ftype: ftype,
            });
        }
        let _ = w.emit("walk_dir_result", files_list);
    });
}
