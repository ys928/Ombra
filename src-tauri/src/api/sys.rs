use crate::unit::{self, win::AppInfo};
use log::debug;

#[tauri::command]
pub fn cli_exec(args: Vec<&str>) {
    #[cfg(target_os = "windows")]
    unit::win::cmd_exec(args);
}

#[tauri::command]
pub fn open_web_url(url: &str) {
    let _ = webbrowser::open(url);
}

#[tauri::command(async)]
pub async fn get_all_app() -> Vec<AppInfo> {
    #[cfg(target_os = "windows")]
    let app_list = unit::win::get_all_app();

    debug!("get_all_app_result {}", app_list.len());

    return app_list;
}

#[tauri::command]
pub fn explorer_select_path(path: &str) {
    #[cfg(target_os = "windows")]
    unit::win::explorer_select_path(path);
}

#[tauri::command]
pub fn get_root_dirs() -> Vec<String> {
    #[cfg(target_os = "windows")]
    unit::win::get_logical_drives()
}

#[tauri::command]
pub fn get_explorer_show_path() -> String {
    #[cfg(target_os = "windows")]
    unit::win::get_explorer_show_path()
}

#[tauri::command]
pub fn default_open_file(path: &str) {
    #[cfg(target_os = "windows")]
    unit::win::default_open_file(path);
}

#[tauri::command]
pub fn get_associated_icon(file_path: &str, save_path: &str) {
    #[cfg(target_os = "windows")]
    unit::win::get_associated_icon(file_path, save_path);
}

#[tauri::command]
pub fn is_auto_start() -> bool {
    let current_exe = std::env::current_exe().unwrap();
    let auto_start = auto_launch::AutoLaunchBuilder::new()
        .set_app_name("ombra")
        .set_app_path(&current_exe.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap();
    return auto_start.is_enabled().unwrap();
}

#[tauri::command]
pub fn auto_start(start: bool) -> bool {
    let current_exe = std::env::current_exe().unwrap();
    let auto_start = auto_launch::AutoLaunchBuilder::new()
        .set_app_name("ombra")
        .set_app_path(&current_exe.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap();
    if start {
        if auto_start.enable().is_err() {
            return false;
        }
    } else {
        if auto_start.disable().is_err() {
            return false;
        };
    }
    return true;
}
