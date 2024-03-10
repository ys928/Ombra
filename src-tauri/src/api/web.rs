use std::io::Read;

use log::debug;

use crate::unit;

#[tauri::command(async)]
pub async fn web_icon_to_file(url: String, save_path: String) -> bool {
   unit::web::save_web_icon(&url, &save_path).await
}

#[tauri::command(async)]
pub async fn download_file(url: String, file: String) -> bool {
    let resp = reqwest::blocking::get(url);
    if resp.is_err() {
        debug!("{:?}", resp.err());
        return false;
    }
    let mut resp = resp.unwrap();
    if !resp.status().is_success() {
        return false;
    }
    let mut buf = Vec::new();
    resp.read_to_end(&mut buf).unwrap();
    std::fs::write(file, buf).unwrap();
    return true;
}
