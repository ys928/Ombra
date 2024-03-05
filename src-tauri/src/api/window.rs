use tauri::Window;

#[tauri::command]
pub fn open_devtools(w: Window) {
    w.open_devtools();
}

#[tauri::command]
pub fn shadow_window(w: Window) {
    #[cfg(any(windows, target_os = "macos"))]
    window_shadows::set_shadow(&w, true).expect("Unsupported platform!");
}