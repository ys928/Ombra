use std::io::Read;

use log::debug;
use tauri::Window;

use crate::unit;

#[tauri::command]
pub fn save_icon_to_file(w: Window, url: String, save_path: String) {
    std::thread::spawn(move || {
        let ret = unit::web::save_web_icon(&url, &save_path);
        w.emit("save_icon_to_file_result", ret).unwrap();
    });
}

#[tauri::command]
pub fn download_file(w: Window, url: String, file: String) {
    std::thread::spawn(move || {
        let resp = reqwest::blocking::get(url);
        if resp.is_err() {
            debug!("{:?}", resp.err());
            w.emit("download_file_result", false).unwrap();
            return;
        }
        let mut resp = resp.unwrap();
        if !resp.status().is_success() {
            return;
        }
        let mut buf = Vec::new();
        resp.read_to_end(&mut buf).unwrap();
        std::fs::write(file, buf).unwrap();
        w.emit("download_file_result", true).unwrap();
    });
}
