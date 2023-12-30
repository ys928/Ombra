use std::{
    ffi::{OsStr, OsString},
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        process::CommandExt,
    },
    process::Command,
};

use serde::{Deserialize, Serialize};
use tauri::Window;
use winapi::ctypes::wchar_t;

use crate::tools::{self};

#[tauri::command]
pub fn open_web_url(url: &str) {
    let _ = webbrowser::open(url);
}

#[tauri::command]
pub fn cmd_exec(args: Vec<&str>) {
    let mut binding = Command::new("cmd");
    let binding = binding.creation_flags(0x08000000).arg("/c");
    for arg in args.iter() {
        binding.arg(arg);
    }
    let _ = binding.spawn();
}

fn get_icon_to_path(path: &str, save_path: &str) -> bool {
    //不是exe文件结尾，直接跳过。
    if !(path.ends_with(".exe") || path.ends_with(".EXE")) {
        return false;
    }
    let file_map = pelite::FileMap::open(path);
    if let Err(_e) = file_map {
        return false;
    }
    let file_map = file_map.unwrap();

    let file = pelite::PeFile::from_bytes(file_map.as_ref());
    if let Err(_e) = file {
        return false;
    }
    let file = file.unwrap();

    let mut icons = file.resources().unwrap().icons();

    let res = icons.next();
    if let None = res {
        return false;
    }
    let res = res.unwrap();
    if let Err(_e) = res {
        return false;
    }
    //只取第一组图标
    let (_, group) = res.unwrap();
    let mut f = false;
    for entry in group.entries() {
        // Fetch the image data for this entry
        match group.image(entry.nId) {
            Ok(image) => {
                // Check if the image data starts with the PNG magic bytes
                if image.starts_with(b"\x89PNG") {
                    std::fs::write(save_path, image).unwrap();
                    f = true;
                }
            }
            Err(_) => {}
        }
    }
    if f {
        return true;
    }
    let mut f = std::fs::File::create(save_path).unwrap();
    match group.write(&mut f) {
        Err(_) => false,
        Ok(_) => true,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstallApp {
    name: String,
    path: String,
    icon: String,
}

#[tauri::command]
pub fn get_all_app(w: Window) {
    std::thread::spawn(move || {
        let ico_path = tools::get_data_dir(Some("icons"));

        let mut paths: Vec<InstallApp> = Vec::new();
        let p = r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs";
        for entry in walkdir::WalkDir::new(p).into_iter().filter_map(|e| e.ok()) {
            if !entry.path().to_string_lossy().ends_with(".lnk") {
                continue;
            }
            //获取lnk对应的文件完整路径
            let full_path = lnk_to_target_path(entry.path().to_str().unwrap());
            if !(full_path.ends_with(".exe") || full_path.ends_with(".EXE")) {
                continue;
            }
            //获取lnk文件名
            let lnk_name = entry.file_name().to_string_lossy().to_string();
            //根据lnk名得到图片名，不以任何后缀结尾
            let lnk_name = lnk_name.replace(".lnk", "");
            //构造icon保存路径
            let icon_save_path = std::path::Path::new(&ico_path);
            let icon_save_path = icon_save_path.join(&lnk_name);
            if !icon_save_path.exists() {
                let ret = get_icon_to_path(full_path.as_str(), icon_save_path.to_str().unwrap());
                if !ret {
                    continue;
                }
            }

            paths.push(InstallApp {
                path: full_path,
                name: lnk_name,
                icon: icon_save_path.to_string_lossy().to_string(),
            });
        }
        let set: std::collections::HashSet<_> = paths.into_iter().collect();
        let deduplicated_vec: Vec<_> = set.into_iter().collect();
        let _ = w.emit("get_all_app_result", &deduplicated_vec);
    });
}

pub fn get_logical_drives() -> Result<Vec<String>, std::io::Error> {
    unsafe {
        let bitmask = winapi::um::fileapi::GetLogicalDrives();
        let mut drives = Vec::new();
        for i in 0..26 {
            if (bitmask & (1 << i)) != 0 {
                let drive_letter = (b'A' + i as u8) as wchar_t;
                let mut root_path: [wchar_t; 3] = [0; 3];
                root_path[0] = drive_letter;
                root_path[1] = ':' as wchar_t;
                let drive_type = winapi::um::fileapi::GetDriveTypeW(root_path.as_ptr());
                if drive_type == winapi::um::winbase::DRIVE_FIXED
                    || drive_type == winapi::um::winbase::DRIVE_REMOVABLE
                {
                    root_path[2] = '\\' as wchar_t;
                    let os_string = OsString::from_wide(&root_path[..]);
                    if let Some(drive_str) = os_string.to_str() {
                        drives.push(drive_str.to_string());
                    }
                }
            }
        }
        Ok(drives)
    }
}

fn lnk_to_target_path(lnk_path: &str) -> String {
    unsafe {
        let os_str = OsStr::new(lnk_path);
        let wide_str: Vec<u16> = os_str.encode_wide().chain(Some(0)).collect();

        let lib = libloading::Library::new("winsys.dll").unwrap();
        let lnk_target_path: libloading::Symbol<unsafe extern "C" fn(*const u16) -> *mut u16> =
            lib.get(b"lnk_target_path").unwrap();
        let free_memory: libloading::Symbol<unsafe extern "C" fn(*mut u16)> =
            lib.get(b"free_memory").unwrap();
        let path = lnk_target_path(wide_str.as_ptr());
        // 将原始指针转换为引用
        let reference: &[u16] = std::slice::from_raw_parts(path, 260);
        let os_stirng = OsString::from_wide(reference);
        let full_path = os_stirng.to_string_lossy().to_string();
        free_memory(path);
        let pos = full_path.find('\0').unwrap();
        let (a, _) = full_path.split_at(pos);
        return a.to_string();
    }
}

#[tauri::command]
pub fn get_explorer_show_path() -> String {
    unsafe {
        let lib = libloading::Library::new("winsys.dll").unwrap();
        let get_explorer_path: libloading::Symbol<unsafe extern "C" fn() -> *mut u16> =
            lib.get(b"get_explorer_path").unwrap();
        let free_memory: libloading::Symbol<unsafe extern "C" fn(*mut u16)> =
            lib.get(b"free_memory").unwrap();
        let path = get_explorer_path();
        if path == std::ptr::null_mut() {
            return "none".to_string();
        }
        // 将原始指针转换为引用
        let reference: &[u16] = std::slice::from_raw_parts(path, 260);
        let os_stirng = OsString::from_wide(reference);
        let full_path = os_stirng.to_string_lossy().to_string();
        free_memory(path);
        let pos = full_path.find('\0').unwrap();
        let (a, _) = full_path.split_at(pos);
        return a.to_string();
    }
}
