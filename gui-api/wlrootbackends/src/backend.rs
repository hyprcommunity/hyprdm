// src/lib.rs
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use hdm_api::{ThemeManager, Compositor, HyprlandIPC, Session};

// Global manager’lar
static THEME_MANAGER: Lazy<Mutex<ThemeManager>> = Lazy::new(|| {
    Mutex::new(ThemeManager::new(Default::default(), "/home/user/.config/hdm_config.json"))
});

static COMPOSITOR: Lazy<Mutex<Compositor>> = Lazy::new(|| {
    Mutex::new(Compositor::new().unwrap())
});

/// Load all themes
#[no_mangle]
pub extern "C" fn rust_load_themes() {
    let mut tm = THEME_MANAGER.lock().unwrap();
    tm.load_and_apply_themes().ok();
}

/// Set theme by name
#[no_mangle]
pub extern "C" fn rust_set_theme(name: *const c_char) -> bool {
    if name.is_null() { return false; }
    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() { Ok(s) => s, Err(_) => return false };
    let mut tm = THEME_MANAGER.lock().unwrap();
    tm.set_theme(name_str).is_ok()
}

/// Apply current layout
#[no_mangle]
pub extern "C" fn rust_apply_layout() {
    let comp = COMPOSITOR.lock().unwrap();
    comp.layout_manager.apply();
}

/// Return available sessions as `;` separated string
#[no_mangle]
pub extern "C" fn rust_available_sessions() -> *mut c_char {
    let sessions = Session::available_sessions();
    let names: Vec<String> = sessions.into_iter().map(|s| s.name).collect();
    CString::new(names.join(";")).unwrap().into_raw()
}

/// Switch to a session by name
#[no_mangle]
pub extern "C" fn rust_switch_session(new_session: *const c_char) -> bool {
    if new_session.is_null() { return false; }
    let c_str = unsafe { CStr::from_ptr(new_session) };
    let session_str = match c_str.to_str() { Ok(s) => s, Err(_) => return false };
    if let Some(mut s) = Session::available_sessions().into_iter().find(|x| x.name == session_str) {
        s.start().is_ok()
    } else {
        false
    }
}

/// Send command to Hyprland IPC
#[no_mangle]
pub extern "C" fn rust_send_ipc_command(cmd: *const c_char) {
    if cmd.is_null() { return; }
    let c_str = unsafe { CStr::from_ptr(cmd) };
    if let Ok(cmd_str) = c_str.to_str() {
        HyprlandIPC::send_command(cmd_str);
    }
}

/// Get active window status
#[no_mangle]
pub extern "C" fn rust_active_window() -> *mut c_char {
    match HyprlandIPC::get_status() {
        Ok(status) => CString::new(status).unwrap().into_raw(),
        Err(_) => CString::new("").unwrap().into_raw()
    }
}

/// Free a string allocated by Rust for C
#[no_mangle]
pub extern "C" fn rust_free_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { CString::from_raw(s) }; // drop
}
