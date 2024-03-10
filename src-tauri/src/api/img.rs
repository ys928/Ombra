use crate::unit;
#[tauri::command(async)]
pub async fn img_compress(img_path: String, save_path: String, quality: u32) -> bool {
    if img_path.ends_with(".jpg") || img_path.ends_with(".jpeg") {
        let ret = unit::img::compress_jpg(&img_path, &save_path, quality);
        return ret;
    } else if img_path.ends_with(".png") {
        let ret = unit::img::compress_png(&img_path, &save_path, quality);
        return ret;
    } else {
        return false;
    }
}

#[tauri::command(async)]
pub async fn img_convert(img_path: String, save_path: String, format: String) -> bool {
    unit::img::convert(&img_path, &save_path, &format)
}
