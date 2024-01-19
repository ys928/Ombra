#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::debug;
use pinyin::ToPinyin;
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::api::dialog;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Window};
use walkdir::WalkDir;

use log4rs::append::rolling_file::{
    policy::compound::{roll::delete::DeleteRoller, trigger::size::SizeTrigger, CompoundPolicy},
    RollingFileAppender,
};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::tools::FileInfo;

mod file_watch;
mod tools;

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
    let log_file_path = tools::get_data_dir(None).join("ombra.log");
    let requests = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build(
            log_file_path,
            Box::new(CompoundPolicy::new(
                Box::new(SizeTrigger::new(1024 * 1024)),
                Box::new(DeleteRoller::new()),
            )),
        )
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(requests)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();

    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let update = CustomMenuItem::new("update".to_string(), "更新");
    let tray_menu = SystemTrayMenu::new().add_item(update).add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    let w = app.get_window("MainWindow").unwrap();
                    let _ = w.emit("exit_app", "");
                    std::process::exit(0);
                }
                "update" => {
                    winsys::open_web_url("https://github.com/ys928/Ombra/releases");
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
            window.center().unwrap();
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
            winsys::explorer_select_path,
            winsys::default_open_file,
            file_watch::watch_dir,
            file_watch::unwatch_dir,
            file_catch::get_file_catch_info,
            walk_all_files,
            search_file,
            shadow_window,
            dir_or_file,
            walk_dir,
            to_pinyin,
            open_devtools
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn shadow_window(w: Window) {
    #[cfg(any(windows, target_os = "macos"))]
    window_shadows::set_shadow(&w, true).expect("Unsupported platform!");
}
static IS_IN_WALKDIR: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn walk_all_files(w: Window) {
    // //测试监视
    // file_catch::init(false);
    // file_watch::watch_all_files();
    // return;
    debug!("enter walk_all_files");
    if IS_IN_WALKDIR.load(Ordering::Relaxed) {
        dialog::message(Some(&w), "提示", "请勿重复操作！");
        return;
    }

    IS_IN_WALKDIR.store(true, Ordering::Relaxed);

    //单开一个线程遍历所有文件
    std::thread::spawn(move || {
        let drives = winsys::get_logical_drives().unwrap();

        file_catch::init(); //重置缓存文件

        let (se, re) = std::sync::mpsc::channel();

        for d in drives {
            let t_se = se.clone();
            //每个盘都单开一个线程遍历，提高速度
            std::thread::spawn(move || {
                for entry in WalkDir::new(d).into_iter().filter_map(|e| e.ok()) {
                    let fi = tools::get_file_info(entry.path());
                    if fi.is_none() {
                        continue;
                    }
                    t_se.send(fi.unwrap()).unwrap();
                }
            });
        }
        drop(se);
        let mut files: LinkedList<FileInfo> = LinkedList::new();
        for file in re {
            files.push_back(file);

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
        IS_IN_WALKDIR.store(false, Ordering::Relaxed);
        //遍历完成后，启动监视
        file_watch::watch_all_files();
    });
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
            let fi = tools::get_file_info(entry.path());
            if fi.is_none() {
                continue;
            }
            files_list.push(fi);
        }
        let _ = w.emit("walk_dir_result", files_list);
    });
}

#[tauri::command]
fn to_pinyin(hans: &str) -> Vec<String> {
    let mut ret = Vec::new();
    for pinyin in hans.to_pinyin() {
        if let Some(pinyin) = pinyin {
            ret.push(pinyin.plain().to_string());
        }
    }
    return ret;
}

#[tauri::command]
fn search_file(w: Window, name: String, ext: String, mode: String, limit: i32, offset: i32) {
    std::thread::spawn(move || {
        if mode == "normal" {
            let ret = file_catch::search_file(&name, &ext, limit, offset);
            let _ = w.emit("search_file_result", ret);
        } else if mode == "exact" {
            let ret = file_catch::search_file_as_exact(&name, &ext, limit, offset);
            let _ = w.emit("search_file_result", ret);
        }
    });
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
fn open_devtools(w:Window){
    w.open_devtools();
}