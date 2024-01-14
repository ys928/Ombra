use std::{
    ffi::OsString,
    os::windows::{ffi::OsStringExt, process::CommandExt},
    process::Command,
};

use serde::{Deserialize, Serialize};
use tauri::Window;

use crate::tools::{self};
use windows::{
    core::{w, Interface, PCWSTR},
    Win32::{
        Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives},
        System::{
            Com::{
                CoInitialize, CoUninitialize, CreateBindCtx,
                StructuredStorage::{PropVariantClear, PropVariantToString},
            },
            WindowsProgramming,
        },
        UI::Shell::{
            BHID_EnumItems, BHID_PropertyStore, IEnumShellItems, IShellItem,
            PropertiesSystem::{IPropertyStore, PSGetNameFromPropertyKey, PROPERTYKEY},
            SHCreateItemFromParsingName,
        },
    },
};

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

#[derive(Serialize, Deserialize, Default)]
struct AppInfo {
    name: String,
    icon: String,
    start: String,
}

#[tauri::command]
pub fn get_all_app(w: Window) {
    std::thread::spawn(move || {
        unsafe {
            let _ = CoInitialize(Some(std::ptr::null()));
            let pbc = CreateBindCtx(0).unwrap();
            let folder: IShellItem =
                SHCreateItemFromParsingName(w!("shell:appsFolder"), &pbc).unwrap();
            let shell_items: IEnumShellItems = folder.BindToHandler(&pbc, &BHID_EnumItems).unwrap();
            let mut app_list = Vec::new();
            let ico_path = tools::get_data_dir(Some("icons"));
            loop {
                let mut data: u32 = 0;
                let mut item = vec![Some(IShellItem::from_raw(std::ptr::null_mut()))];
                let result = shell_items.Next(&mut item, Some(&mut data));
                if result.is_err() {
                    break;
                }
                let item = &item[0];
                if item.is_none() {
                    break;
                }
                let item = item.clone().unwrap();
                let store: IPropertyStore = item.BindToHandler(&pbc, &BHID_PropertyStore).unwrap();
                let count = store.GetCount().unwrap();
                let mut app = AppInfo::default();
                let mut icon = String::new();
                let mut pack = String::new();
                let mut exe = String::new();
                for i in 0..count {
                    let mut pk = PROPERTYKEY::default();
                    let ret = store.GetAt(i, &mut pk);
                    if ret.is_err() {
                        continue;
                    }
                    let pk_name = PSGetNameFromPropertyKey(&pk).unwrap();
                    let k = OsString::from_wide(pk_name.as_wide())
                        .to_string_lossy()
                        .to_string();
                    let mut value = store.GetValue(&pk).unwrap();
                    let mut arr = [0; 1024];
                    let _ = PropVariantToString(&value, &mut arr);
                    let _ = PropVariantClear(&mut value);
                    let pos = arr.iter().position(|c| *c == 0).unwrap();
                    let v = OsString::from_wide(&arr[0..pos])
                        .to_string_lossy()
                        .to_string();
                    // println!("{}={}", &k, &v);
                    if k == "System.ItemNameDisplay" {
                        app.name = v;
                    } else if k == "System.AppUserModel.ID" {
                        app.start = "shell:appsFolder\\".to_string() + &v;
                    } else if k == "System.Link.TargetParsingPath" {
                        exe = v;
                    } else if k == "System.AppUserModel.PackageInstallPath" {
                        pack = v;
                    } else if k == "System.Tile.Square150x150LogoPath" {
                        icon = v;
                    } else if k == "System.Tile.SmallLogoPath" {
                        icon = v;
                    }
                }
                if exe.len() > 0
                    && !(exe.ends_with(".exe") || exe.ends_with(".msc") || exe.ends_with(".bat"))
                {
                    continue;
                }
                if exe.ends_with(".exe") && !std::path::Path::new(&icon).exists() {
                    let icon_save_path = std::path::Path::new(&ico_path);
                    let icon_save_path = icon_save_path.join(&app.name);
                    if !icon_save_path.exists() {
                        get_icon_to_path(&exe, icon_save_path.to_str().unwrap());
                    }
                    app.icon = icon_save_path.to_string_lossy().to_string();
                } else if icon.len() > 0 && pack.len() > 0 {
                    app.icon = std::path::Path::new(&pack)
                        .join(icon)
                        .to_string_lossy()
                        .to_string();
                }
                app_list.push(app);
            }
            CoUninitialize();
            let _ = w.emit("get_all_app_result", &app_list);
        };
    });
}

pub fn get_logical_drives() -> Result<Vec<String>, std::io::Error> {
    unsafe {
        let bitmask = GetLogicalDrives();
        let mut drives = Vec::new();
        for i in 0..26 {
            if (bitmask & (1 << i)) != 0 {
                let drive_letter = (b'A' + i as u8) as u16;
                let mut root_path: [u16; 3] = [0; 3];
                root_path[0] = drive_letter;
                root_path[1] = ':' as u16;
                let drive = PCWSTR::from_raw(root_path.as_ptr());
                let drive_type = GetDriveTypeW(drive);
                if drive_type == WindowsProgramming::DRIVE_FIXED
                    || drive_type == WindowsProgramming::DRIVE_REMOVABLE
                {
                    root_path[2] = '\\' as u16;
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
