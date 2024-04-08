#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, LogicalPosition, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

use log4rs::append::rolling_file::{
    policy::compound::{roll::delete::DeleteRoller, trigger::size::SizeTrigger, CompoundPolicy},
    RollingFileAppender,
};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::unit::fs::FileInfo;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

mod api;
mod unit;

fn main() {
    let log_file_path = unit::fs::data_dir(None).join("ombra.log");
    let requests = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] {f}:{L} => {m}{n}",
        )))
        .build(
            &log_file_path,
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

    let update = CustomMenuItem::new("update".to_string(), "更新");
    let logfile = CustomMenuItem::new("log".to_string(), "日志");
    let relaunch = CustomMenuItem::new("relaunch".to_string(), "重启");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new()
        .add_item(logfile)
        .add_item(update)
        .add_item(relaunch)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .system_tray(system_tray)
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    app.exit(0);
                }
                "update" => {
                    api::sys::open_web_url("https://github.com/ys928/Ombra/releases");
                }
                "log" => {
                    api::sys::explorer_select_path(log_file_path.to_str().unwrap());
                }
                "relaunch" => {
                    app.restart();
                }
                _ => {}
            },
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                app.emit_all("click_tray", ()).unwrap();
            }
            _ => {}
        })
        .setup(move |app| {
            let window = app.get_window("MainWindow").unwrap();
            window
                .set_position(LogicalPosition { x: 350, y: 200 })
                .unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            window_shadows::set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .invoke_handler(tauri::generate_handler![
            api::sys::cli_exec,
            api::sys::open_web_url,
            api::sys::get_all_app,
            api::sys::explorer_select_path,
            api::sys::get_explorer_show_path,
            api::sys::default_open_file,
            api::sys::get_associated_icon,
            api::sys::auto_start,
            api::sys::is_auto_start,
            api::web::web_icon_to_file,
            api::web::download_file,
            api::img::img_compress,
            api::img::img_convert,
            api::file_db::get_file_catch_info,
            api::text::to_pinyin,
            api::window::open_devtools,
            api::window::shadow_window,
            api::fs::is_dir,
            api::fs::walk_dir,
            api::fs::watch_dir,
            api::fs::unwatch_dir,
            api::file_db::walk_all_files,
            api::file_db::search_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn create_window() {
//     use std::num::NonZeroU32;
//     use std::rc::Rc;
//     use winit::event::{Event, WindowEvent};
//     use winit::event_loop::{ControlFlow, EventLoop};
//     use winit::window::WindowBuilder;

//     let event_loop = EventLoop::new().unwrap();
//     let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
//     let context = softbuffer::Context::new(window.clone()).unwrap();
//     let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

//     event_loop
//         .run(move |event, elwt| {
//             elwt.set_control_flow(ControlFlow::Wait);

//             match event {
//                 Event::WindowEvent {
//                     window_id,
//                     event: WindowEvent::RedrawRequested,
//                 } if window_id == window.id() => {
//                     let (width, height) = {
//                         let size = window.inner_size();
//                         (size.width, size.height)
//                     };
//                     surface
//                         .resize(
//                             NonZeroU32::new(width).unwrap(),
//                             NonZeroU32::new(height).unwrap(),
//                         )
//                         .unwrap();

//                     let mut buffer = surface.buffer_mut().unwrap();
//                     for index in 0..(width * height) {
//                         let y = index / width;
//                         let x = index % width;
//                         let red = x % 255;
//                         let green = y % 255;
//                         let blue = (x * y) % 255;

//                         buffer[index as usize] = blue | (green << 8) | (red << 16);
//                     }

//                     buffer.present().unwrap();
//                 }
//                 Event::WindowEvent {
//                     event: WindowEvent::CloseRequested,
//                     window_id,
//                 } if window_id == window.id() => {
//                     elwt.exit();
//                 }
//                 _ => {}
//             }
//         })
//         .unwrap();
// }
