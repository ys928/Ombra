use std::{
    ffi::OsString,
    fs,
    mem::{size_of, ManuallyDrop},
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
        Graphics::Gdi::{
            DeleteObject, GetBitmapBits, GetObjectW, BITMAP, BITMAPINFOHEADER, HBITMAP,
        },
        Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives},
        System::{
            Com::{
                CoCreateInstance, CoInitialize, CoInitializeEx, CoTaskMemFree, CoUninitialize,
                CreateBindCtx, IServiceProvider,
                StructuredStorage::{PropVariantClear, PropVariantToString},
                CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
            },
            Variant::{VARIANT, VARIANT_0, VARIANT_0_0, VARIANT_0_0_0, VT_I4},
            WindowsProgramming,
        },
        UI::{
            Shell::{
                BHID_EnumItems, BHID_PropertyStore, ExtractAssociatedIconW, ExtractIconExW,
                IEnumShellItems, IFolderView, ILCreateFromPathW, IPersistFolder2, IShellBrowser,
                IShellItem, IShellWindows, IWebBrowser2,
                PropertiesSystem::{IPropertyStore, PSGetNameFromPropertyKey, PROPERTYKEY},
                SHCreateItemFromParsingName, SHGetNameFromIDList, SHOpenFolderAndSelectItems,
                ShellExecuteW, ShellWindows, SIGDN_FILESYSPATH,
            },
            WindowsAndMessaging::{
                DestroyIcon, GetClassNameW, GetForegroundWindow, GetIconInfo, HICON, ICONINFO, SW_SHOWNORMAL
            },
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

fn get_icon_from_pe(path: &str, save_path: &str) -> bool {
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
                let mut target = String::new();
                for i in 0..count {
                    let mut pk = PROPERTYKEY::default();
                    let ret = store.GetAt(i, &mut pk);
                    if ret.is_err() {
                        continue;
                    }
                    let pk_name = PSGetNameFromPropertyKey(&pk).unwrap();
                    let k = String::from_utf16_lossy(pk_name.as_wide());
                    let mut value = store.GetValue(&pk).unwrap();
                    let mut arr = [0; 1024];
                    let _ = PropVariantToString(&value, &mut arr);
                    let _ = PropVariantClear(&mut value);
                    let pos = arr.iter().position(|c| *c == 0).unwrap();
                    let v = String::from_utf16_lossy(&arr[0..pos]);
                    // println!("{}={}", &k, &v);
                    // if app.name == "放大镜" {
                    //     println!("{}={}", &k, &v);
                    // }
                    if k == "System.ItemNameDisplay" {
                        app.name = v;
                    } else if k == "System.AppUserModel.ID" {
                        app.start = "shell:appsFolder\\".to_string() + &v;
                    } else if k == "System.Link.TargetParsingPath" {
                        target = v;
                    } else if k == "System.AppUserModel.PackageInstallPath" {
                        pack = v;
                    } else if k == "System.Tile.Square150x150LogoPath" {
                        if icon.len() == 0 {
                            icon = v;
                        }
                    } else if k == "System.Tile.SmallLogoPath" {
                        icon = v;
                    }
                }
                //跳过卸载软件
                if target.to_lowercase().contains("uninstall") {
                    continue;
                }
                //跳过非可执行文件
                if target.len() > 0
                    && !(target.ends_with(".exe")
                        || target.ends_with(".msc")
                        || target.ends_with(".bat"))
                {
                    continue;
                }
                //获取包图标
                if icon.len() > 0 && pack.len() > 0 {
                    let icon = std::path::Path::new(&pack).join(&icon);
                    app.icon = match_icon_path(icon.as_path());
                }
                //exe程序优先
                if target.ends_with(".exe") {
                    app.start = target.clone();
                    let icon_save_path = std::path::Path::new(&ico_path);
                    let icon_save_path = icon_save_path.join(&app.name);
                    //图标不存在, 并且没有缓存
                    if !std::path::Path::new(&app.icon).exists() && !icon_save_path.exists() {
                        //先尝试关联文件获取、失败则继续尝试pe文件中获取
                        let ret = get_associated_icon(&target, icon_save_path.to_str().unwrap())
                            || get_icon_from_pe(&target, icon_save_path.to_str().unwrap());
                        if ret {
                            app.icon = icon_save_path.to_string_lossy().to_string();
                        } else {
                            app.icon.clear();
                        }
                    } else {
                        app.icon = icon_save_path.to_string_lossy().to_string();
                    }
                }else if target.ends_with(".msc") {
                    app.start = target.clone();
                    let icon_save_path = std::path::Path::new(&ico_path);
                    let icon_save_path = icon_save_path.join(&app.name);
                    if !icon_save_path.exists() {
                        msc_icon(&target, icon_save_path.to_str().unwrap());
                    }
                    app.icon=icon_save_path.to_string_lossy().to_string();
                }
                if !std::path::Path::new(&app.icon).exists() {
                    println!("{}:{}:{}", app.name, app.icon,target);
                }
                app_list.push(app);
            }
            CoUninitialize();
            let _ = w.emit("get_all_app_result", &app_list);
            debug!("send event:get_all_app_result {}", app_list.len());
        };
    });
}

fn match_icon_path(icon: &std::path::Path) -> String {
    let parent = icon.parent().unwrap();
    let name_ext = icon.file_name().unwrap().to_str().unwrap();
    let name = icon.file_stem().unwrap().to_str().unwrap();
    let ext = icon.extension().unwrap().to_str().unwrap();
    let mut image_size = 0;
    let r_size = regex::Regex::new(r"(\d+)").unwrap();

    let mut ret_image_path = String::new();
    // 获取目录下的所有文件，找到图标尺寸最大的图标
    let entries = fs::read_dir(parent);
    if entries.is_err() {
        return ret_image_path;
    }
    let entries = entries.unwrap();
    for entry in entries {
        if entry.is_err() {
            continue;
        }
        let entry = entry.unwrap();
        let file_path = entry.path();
        //跳过对比图
        if file_path.to_str().unwrap().contains("contrast") {
            continue;
        }
        if file_path.is_dir() {
            let dir_name = file_path.file_name().unwrap().to_str().unwrap();
            let p = parent.join(dir_name).join(name_ext);
            let ret_img_path = match_icon_path(p.as_path());
            if ret_img_path.len() == 0 {
                continue;
            }
            let p = std::path::Path::new(&ret_img_path);
            let file_name = p.file_name().unwrap().to_str().unwrap();
            let cap = r_size.captures(&file_name);
            if cap.is_none() {
                continue;
            }
            let cap = cap.unwrap();
            let s = cap.get(0).map_or("0", |s| s.as_str());
            let size: u32 = s.parse().unwrap();
            let file_ext = p.extension().unwrap().to_str().unwrap();
            if size >= image_size && file_name.starts_with(name) && ext == file_ext {
                image_size = size;
                ret_image_path = ret_img_path;
            }
        } else {
            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
            let cap = r_size.captures(&file_name);
            if cap.is_none() {
                continue;
            }
            let cap = cap.unwrap();
            let s = cap.get(0).map_or("0", |s| s.as_str());
            let size: u32 = s.parse().unwrap();
            let file_ext = file_path.extension().unwrap().to_str().unwrap();
            if size >= image_size && file_name.starts_with(name) && ext == file_ext {
                image_size = size;
                ret_image_path = file_path.to_string_lossy().to_string();
            }
        }
    }
    return ret_image_path;
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

#[tauri::command]
pub fn explorer_select_path(path: &str) {
    debug!("enter explorer_select_path");
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
        let mut p: Vec<u16> = path.encode_utf16().collect();
        p.push(0);
        let p = PCWSTR::from_raw(p.as_mut_ptr());
        let it = ILCreateFromPathW(p);
        let _ = SHOpenFolderAndSelectItems(it, None, 0);
        CoUninitialize();
    }
}

#[tauri::command]
pub fn default_open_file(path: &str) {
    debug!("enter default_open_file");
    unsafe {
        let mut p: Vec<u16> = path.encode_utf16().collect();
        p.push(0);
        let p = PCWSTR::from_raw(p.as_mut_ptr());
        ShellExecuteW(None, w!("open"), p, None, None, SW_SHOWNORMAL);
    }
}

#[derive(Default)]
#[repr(C)]
pub struct ICONHEADER {
    id_reserved: u16, // must be 0
    id_type: u16,     // 1 = ICON, 2 = CURSOR
    id_count: u16,    // number of images (and ICONDIRs)
                      // ICONDIR [1...n]
                      // ICONIMAGE [1...n]
}

#[derive(Default)]
#[repr(C)]
pub struct ICONDIR {
    b_width: u8,
    b_height: u8,
    b_color_count: u8,
    b_reserved: u8,
    w_planes: u16,    // for cursors, this field = wXHotSpot
    w_bit_count: u16, // for cursors, this field = wYHotSpot
    dw_bytes_in_res: u32,
    dw_image_offset: u32, // file-offset to the start of ICONIMAGE
}
#[tauri::command]
pub fn get_associated_icon(file_path: &str, save_path: &str) -> bool {
    let file_path: Vec<u16> = file_path.encode_utf16().collect();
    let mut path: [u16; 128] = [0; 128];
    for i in 0..file_path.len() {
        path[i] = file_path[i];
    }
    unsafe {
        //获取HICON
        let mut pindex = 0;
        let hicon = ExtractAssociatedIconW(None, &mut path, &mut pindex);
        if hicon.0 == 0 {
            return false;
        }
        hicon_to_file(hicon, save_path);
    }
    return true;
}

fn num_bitmap_bytes(p_bitmap: &BITMAP) -> i32 {
    let mut n_width_bytes = p_bitmap.bmWidthBytes;
    let res = n_width_bytes % 4;
    if res != 0 {
        n_width_bytes = n_width_bytes + 4 - res;
    }
    return n_width_bytes * p_bitmap.bmHeight;
}
fn write_icon_data(icon_data: &mut Vec<u8>, hbitmap: HBITMAP) {
    unsafe {
        let mut bmp = BITMAP::default();

        GetObjectW(
            hbitmap,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut BITMAP as *mut std::ffi::c_void),
        );

        let n_bitmap_bytes = num_bitmap_bytes(&bmp);
        let mut data = Vec::new();
        data.resize(n_bitmap_bytes as usize, 0);
        GetBitmapBits(
            hbitmap,
            n_bitmap_bytes,
            data.as_mut_ptr() as *mut std::ffi::c_void,
        );

        for i in (0..bmp.bmHeight).rev() {
            // Write the bitmap scanline
            let line_data: &[u8] = std::slice::from_raw_parts(
                (data.as_ptr() as *const _ as *const u8)
                    .wrapping_add((i * bmp.bmWidthBytes) as usize),
                bmp.bmWidthBytes as usize,
            );
            line_data.iter().for_each(|n| icon_data.push(*n));
            // extend to a 32bit boundary (in the file) if necessary
            if bmp.bmWidthBytes % 4 != 0 {
                let padding: u32 = 0;
                let padding: &[u8] =
                    std::slice::from_raw_parts(&padding as *const _ as *const u8, 4);
                padding.iter().for_each(|n| icon_data.push(*n));
            }
        }
    }
}

fn msc_icon(path: &str, save_path: &str) {
    if !path.ends_with(".msc") {
        return;
    }
    let path = std::path::Path::new(path);
    if !path.exists() {
        return;
    }
    let data = std::fs::read_to_string(path).unwrap();
    let names_element = xmltree::Element::parse(data.as_bytes()).unwrap();

    let name = names_element
        .get_child("VisualAttributes")
        .expect("Can't find name element")
        .get_child("Icon")
        .unwrap();
    let index: i32 = name.attributes.get("Index").unwrap().parse().unwrap();
    let file: &String = name.attributes.get("File").unwrap();
    let r = regex::Regex::new("%(.*?)%").unwrap();
    let cap = r.captures(file);
    let file_path;
    if cap.is_none(){
        file_path=file.clone();
    }else{
        let ret = cap.unwrap().get(1).map_or("", |m| m.as_str());
        if ret.len() == 0 {
            return;
        }
        let sysdir = std::env::var_os(ret).unwrap().to_string_lossy().to_string();
        file_path = r.replace(file, &sysdir).to_string();
    }
    let mut file_path: Vec<u16> = file_path.encode_utf16().collect();
    file_path.push(0);
    let file_path = PCWSTR::from_raw(file_path.as_ptr());
    let mut p_hicon = HICON::default();
    unsafe {
        ExtractIconExW(file_path, index, Some(&mut p_hicon), None, 1);
        if p_hicon.0 == 0 {
            debug!("ExtractIconExW failed");
            return;
        }
        hicon_to_file(p_hicon, save_path);
        DestroyIcon(p_hicon).unwrap();
    };
}

fn hicon_to_file(hicon: HICON, save_path: &str) {
    unsafe {
        //根据HICON获取bmp图像数据
        let mut info = ICONINFO::default();
        let _ = GetIconInfo(hicon, &mut info);
        let mut bmp_color = BITMAP::default();
        let mut bmp_mask = BITMAP::default();

        GetObjectW(
            info.hbmColor,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp_color as *mut BITMAP as *mut std::ffi::c_void),
        );

        GetObjectW(
            info.hbmMask,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp_mask as *mut BITMAP as *mut std::ffi::c_void),
        );

        //得到BMP图片数据
        let n_image_bytes = (num_bitmap_bytes(&bmp_color) + num_bitmap_bytes(&bmp_mask)) as u32;
        let mut bi_header = BITMAPINFOHEADER::default();
        bi_header.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bi_header.biWidth = bmp_color.bmWidth;
        bi_header.biHeight = bmp_color.bmHeight * 2;
        bi_header.biPlanes = bmp_color.bmPlanes;
        bi_header.biBitCount = bmp_color.bmBitsPixel;
        bi_header.biSizeImage = n_image_bytes;
        //得到ICONDIR数据
        let mut icon_dir = ICONDIR::default();
        icon_dir.b_width = bmp_color.bmWidth as u8;
        icon_dir.b_height = bmp_color.bmHeight as u8;
        icon_dir.b_color_count = 0;
        icon_dir.w_planes = bmp_color.bmPlanes;
        icon_dir.w_bit_count = bmp_color.bmBitsPixel;
        icon_dir.dw_bytes_in_res = size_of::<BITMAPINFOHEADER>() as u32 + n_image_bytes;
        icon_dir.dw_image_offset = (size_of::<ICONHEADER>() + size_of::<ICONDIR>()) as u32;

        //写入icon头
        let mut icon_header = ICONHEADER::default();
        icon_header.id_reserved = 0;
        icon_header.id_type = 1; //type 1=ICON
        icon_header.id_count = 1; //number of icon dir
        let icon_header: &[u8] = std::slice::from_raw_parts(
            &icon_header as *const _ as *const u8,
            size_of::<ICONHEADER>(),
        );

        let mut icon_data = Vec::new();
        icon_header.iter().for_each(|n| icon_data.push(*n));

        //写入icon dir
        let icon_dir: &[u8] =
            std::slice::from_raw_parts(&icon_dir as *const _ as *const u8, size_of::<ICONDIR>());
        icon_dir.iter().for_each(|n| icon_data.push(*n));

        //写入bmp头
        let bi_header: &[u8] = std::slice::from_raw_parts(
            &bi_header as *const _ as *const u8,
            size_of::<BITMAPINFOHEADER>(),
        );
        bi_header.iter().for_each(|n| icon_data.push(*n));

        //写入图片数据
        write_icon_data(&mut icon_data, info.hbmColor);
        write_icon_data(&mut icon_data, info.hbmMask);

        DeleteObject(info.hbmColor);
        DeleteObject(info.hbmMask);

        std::fs::write(save_path, icon_data).unwrap();
    }
}
