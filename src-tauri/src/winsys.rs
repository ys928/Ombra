use std::{
    ffi::OsString,
    mem::ManuallyDrop,
    os::windows::{ffi::OsStringExt, process::CommandExt},
    process::Command,
};

use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tauri::Window;

use crate::tools::{self};
use windows::{
    core::{w, ComInterface, Interface, PCWSTR},
    Win32::{
        Foundation::S_OK,
        Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives},
        System::{
            Com::{
                CoCreateInstance, CoInitialize, CoTaskMemFree, CoUninitialize, CreateBindCtx,
                IServiceProvider,
                StructuredStorage::{PropVariantClear, PropVariantToString},
                CLSCTX_LOCAL_SERVER,
            },
            Variant::{VARIANT, VARIANT_0, VARIANT_0_0, VARIANT_0_0_0, VT_I4},
            WindowsProgramming,
        },
        UI::{
            Shell::{
                BHID_EnumItems, BHID_PropertyStore, IEnumShellItems, IFolderView, IPersistFolder2,
                IShellBrowser, IShellItem, IShellWindows, IWebBrowser2,
                PropertiesSystem::{IPropertyStore, PSGetNameFromPropertyKey, PROPERTYKEY},
                SHCreateItemFromParsingName, SHGetNameFromIDList, ShellWindows, SIGDN_FILESYSPATH,
            },
            WindowsAndMessaging::{GetClassNameW, GetForegroundWindow},
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
    debug!("enter get_all_app");
    std::thread::spawn(move || {
        unsafe {
            let _ = CoInitialize(Some(std::ptr::null()));
            let pbc = CreateBindCtx(0);
            if pbc.is_err() {
                error!("call CreateBindCtx failed");
                return;
            }
            let pbc = pbc.unwrap();
            let folder = SHCreateItemFromParsingName(w!("shell:appsFolder"), &pbc);
            if folder.is_err() {
                error!("call SHCreateItemFromParsingName failed");
                return;
            }
            let folder: IShellItem = folder.unwrap();
            let shell_items = folder.BindToHandler(&pbc, &BHID_EnumItems);
            if shell_items.is_err() {
                error!("call BindToHandler to get folder failed");
                return;
            }
            let shell_items: IEnumShellItems = shell_items.unwrap();
            let mut app_list = Vec::new();
            let ico_path = tools::get_data_dir(Some("icons"));
            loop {
                let mut data: u32 = 0;
                let mut item = vec![None];
                let result = shell_items.Next(&mut item, Some(&mut data));
                if result.is_err() {
                    break;
                }
                let item = &item[0];
                if item.is_none() {
                    break;
                }
                let item = item.clone();
                if item.is_none() {
                    info!("clone item failed");
                    continue;
                }
                let item = item.unwrap();
                let store = item.BindToHandler(&pbc, &BHID_PropertyStore);
                if store.is_err() {
                    info!("call BindToHandler to get item failed");
                    continue;
                }
                let store: IPropertyStore = store.unwrap();
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
            debug!("send event:get_all_app_result {}", app_list.len());
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
    debug!("enter get_explorer_show_path");
    let mut folder_cur_path = "none".to_string();
    unsafe {
        let foreground_window: windows::Win32::Foundation::HWND = GetForegroundWindow();
        let mut class_name = [0; 260];
        GetClassNameW(foreground_window, &mut class_name);
        let pos = class_name.iter().position(|c| *c == 0).unwrap();
        let class_name = String::from_utf16_lossy(&class_name[0..pos]);
        if class_name != "CabinetWClass" {
            debug!("not explorer window：{}", class_name);
            return folder_cur_path;
        }
        debug!("find explorer windows");
        let _ = CoInitialize(None);
        let psh_windows = CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER);
        if psh_windows.is_err() {
            CoUninitialize();
            return folder_cur_path;
        }
        let psh_windows: IShellWindows = psh_windows.unwrap();
        let count = psh_windows.Count();
        if count.is_err() {
            CoUninitialize();
            return folder_cur_path;
        }
        let count: i32 = count.unwrap();
        for i in 0..count {
            let i = VARIANT {
                Anonymous: VARIANT_0 {
                    Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                        vt: VT_I4,
                        wReserved1: 0,
                        wReserved2: 0,
                        wReserved3: 0,
                        Anonymous: VARIANT_0_0_0 { lVal: i },
                    }),
                },
            };
            let disp = psh_windows.Item(i);
            if disp.is_err() {
                continue;
            }
            let disp = disp.unwrap();
            let mut p_app = std::ptr::null_mut();
            let ret = disp.query(&IWebBrowser2::IID, &mut p_app);
            if ret != S_OK {
                continue;
            }
            let p_app = IWebBrowser2::from_raw(p_app);
            let win_hwnd = p_app.HWND();
            if win_hwnd.is_err() {
                continue;
            }
            let win_hwnd = win_hwnd.unwrap();
            if win_hwnd.0 != foreground_window.0 {
                continue;
            }
            let mut psp = std::ptr::null_mut();
            let ret = p_app.query(&IServiceProvider::IID, &mut psp);
            if ret != S_OK {
                continue;
            }
            let psp = IServiceProvider::from_raw(psp);
            let browser = psp.QueryService(&IShellBrowser::IID);
            if browser.is_err() {
                continue;
            }
            let browser: IShellBrowser = browser.unwrap();
            let shell_view = browser.QueryActiveShellView();
            if shell_view.is_err() {
                continue;
            }
            let shell_view = shell_view.unwrap();
            let mut p_folder_view = std::ptr::null_mut();
            let ret = shell_view.query(&IFolderView::IID, &mut p_folder_view);
            if ret != S_OK {
                continue;
            }
            let p_folder_view = IFolderView::from_raw(p_folder_view);
            let folder = p_folder_view.GetFolder();
            if folder.is_err() {
                continue;
            }
            let folder: IPersistFolder2 = folder.unwrap();
            let pidl = folder.GetCurFolder();
            if pidl.is_err() {
                continue;
            }
            let pidl = pidl.unwrap();
            let path = SHGetNameFromIDList(pidl, SIGDN_FILESYSPATH);
            if path.is_err() {
                continue;
            }
            let path = path.unwrap();
            let path = path.as_wide();
            folder_cur_path = String::from_utf16_lossy(path);
            CoTaskMemFree(Some(path.as_ptr() as *const std::ffi::c_void));
            break;
        }
        CoUninitialize();
    };
    return folder_cur_path;
}
