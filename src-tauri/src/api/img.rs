use tauri::Window;

use crate::unit;
#[tauri::command]
pub fn img_compress(w: Window, img_path: String, save_path: String, quality: u32) {
    std::thread::spawn(move || {
        if img_path.ends_with(".jpg") || img_path.ends_with(".jpeg") {
            let ret = unit::img::compress_jpg(&img_path, &save_path, quality);
            w.emit("img_compress_result", ret).unwrap();
        } else if img_path.ends_with(".png") {
            let ret = unit::img::compress_png(&img_path, &save_path, quality);
            w.emit("img_compress_result", ret).unwrap();
        } else {
            w.emit("img_compress_result", false).unwrap();
        }
    });
}

#[tauri::command]
pub fn img_convert(w: Window, img_path: String, save_path: String, format: String) {
    std::thread::spawn(move || {
        let ret = unit::img::convert(&img_path, &save_path, &format);
        w.emit("img_convert_result", ret).unwrap();
    });
}
