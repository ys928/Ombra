use std::path::PathBuf;

pub fn ensure_dir(dir: &str) -> bool {
    let p = std::path::Path::new(dir);
    if !p.exists() || !p.is_dir() {
        if let Err(_) = std::fs::create_dir_all(p) {
            return false;
        }
    }
    true
}

pub fn sys_time_to_seconds(time: std::time::SystemTime) -> u64 {
    let duration_since_epoch = time
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    return duration_since_epoch.as_secs();
}

pub fn get_data_dir(dir: Option<&str>) -> PathBuf {
    let data_path = dirs::data_dir().unwrap();
    let data_path = std::path::Path::new(&data_path);
    let path;
    if let None = dir {
        path = data_path.join("ombra");
    } else {
        path = data_path.join("ombra").join(dir.unwrap());
    }
    ensure_dir(path.to_str().unwrap());
    return path;
}