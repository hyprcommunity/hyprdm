// -------------------- FFI FULL --------------------
use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::sync::Arc;
use std::ptr;
use std::path::Path;

use hdm_api::compositor::Compositor;
use hdm_api::ipc::HyprlandIPC;
use hdm_api::layout_manager::{LayoutManager, Layout, Panel};
use hdm_api::session_manager::Session;
use hdm_api::theme_manager::ThemeManager;
use hdm_api::unidata::{UnidataGenerator, PlatformType};
use hdm_api::user_manager::{User, TwoFactorMethod};

// -------------------- Compositor FFI --------------------
#[no_mangle]
pub extern "C" fn compositor_new() -> *mut Compositor {
    match Compositor::new() {
        Ok(c) => Box::into_raw(Box::new(c)),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn compositor_run_with_ipc(c: *mut Compositor, ipc: *mut HyprlandIPC) -> c_int {
    if c.is_null() { return -1; }
    let compositor = unsafe { &mut *c };
    let ipc_arc = if ipc.is_null() {
        None
    } else {
        Some(unsafe { Arc::from_raw(ipc) })
    };
    let result = compositor.run_with_ipc(ipc_arc).is_ok() as c_int;
    result
}

#[no_mangle]
pub extern "C" fn compositor_stop(c: *mut Compositor) {
    if !c.is_null() {
        let compositor = unsafe { &mut *c };
        compositor.stop();
    }
}

// -------------------- IPC FFI --------------------
#[no_mangle]
pub extern "C" fn ipc_new() -> *mut HyprlandIPC {
    Box::into_raw(Box::new(HyprlandIPC))
}

#[no_mangle]
pub extern "C" fn ipc_send_command(ipc: *mut HyprlandIPC, cmd: *const c_char) -> c_int {
    if ipc.is_null() { return -1; }
    let ipc_ref = unsafe { &mut *ipc };
    let c_str = unsafe { CStr::from_ptr(cmd) };
    let s = c_str.to_str().unwrap_or("");
    ipc_ref.send_command(s).is_ok() as c_int
}

#[no_mangle]
pub extern "C" fn ipc_get_status(ipc: *mut HyprlandIPC) -> *mut c_char {
    if ipc.is_null() { return ptr::null_mut(); }
    let ipc_ref = unsafe { &mut *ipc };
    match ipc_ref.get_status() {
        Ok(s) => CString::new(s).unwrap().into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

// -------------------- LayoutManager FFI --------------------

#[no_mangle]
pub extern "C" fn layout_manager_new(panel_name: *const c_char) -> *mut LayoutManager {
    if panel_name.is_null() {
        return std::ptr::null_mut();
    }
    let name_str = unsafe { CStr::from_ptr(panel_name).to_string_lossy().to_string() };
    let lm = LayoutManager {
        panel: Panel {
            name: name_str,
            layout: Layout::Tiling,
        },
        global_layout: Layout::Tiling,
    };
    Box::into_raw(Box::new(lm))
}

#[repr(C)]
pub struct PanelRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[no_mangle]
pub extern "C" fn layout_manager_apply(
    lm: *mut LayoutManager,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
) {
    if lm.is_null() { return; }
    let lm_ref = unsafe { &mut *lm };
    lm_ref.apply(width, height, x, y);
}

#[no_mangle]
pub extern "C" fn layout_manager_get_panel_rect(
    lm: *mut LayoutManager,
    out: *mut PanelRect,
    screen_width: u32,
    screen_height: u32,
) -> c_int {
    if lm.is_null() || out.is_null() { return -1; }
    let lm_ref = unsafe { &mut *lm };
    let rect = match lm_ref.panel.layout {
        Layout::Tiling => PanelRect { x:0, y:0, width:screen_width, height:screen_height },
        Layout::Floating => PanelRect { x:screen_width/4, y:screen_height/4, width:screen_width/2, height:screen_height/2 },
    };
    unsafe { *out = rect; }
    0
}

// -------------------- Panel Name Getter --------------------
#[no_mangle]
pub extern "C" fn layout_manager_get_panel_name(lm: *const LayoutManager) -> *const c_char {
    if lm.is_null() { return ptr::null(); }
    let lm_ref = unsafe { &*lm };
    match CString::new(lm_ref.panel.name.clone()) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null(),
    }
}
#[no_mangle]
pub extern "C" fn string_free(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { let _ = CString::from_raw(s); } 
}

// -------------------- Session FFI --------------------
#[no_mangle]
pub extern "C" fn session_new(name: *const c_char, exec: *const c_char) -> *mut Session {
    let name_str = unsafe { CStr::from_ptr(name).to_string_lossy() }.to_string();
    let exec_str = unsafe { CStr::from_ptr(exec).to_string_lossy() }.to_string();
    Box::into_raw(Box::new(Session::new(&name_str, &exec_str)))
}

#[no_mangle]
pub extern "C" fn session_start(s: *mut Session) -> c_int {
    if s.is_null() { return -1; }
    let session = unsafe { &mut *s };
    session.start().is_ok() as c_int
}

#[no_mangle]
pub extern "C" fn session_stop(s: *mut Session) -> c_int {
    if s.is_null() { return -1; }
    let session = unsafe { &mut *s };
    session.stop().is_ok() as c_int
}

#[no_mangle]
pub extern "C" fn session_restart(s: *mut Session) -> c_int {
    if s.is_null() { return -1; }
    let s_ref = unsafe { &mut *s };
    s_ref.restart().is_ok() as c_int
}

#[no_mangle]
pub extern "C" fn session_switch(s: *mut Session, new_name: *const c_char, new_exec: *const c_char) -> c_int {
    if s.is_null() { return -1; }
    let s_ref = unsafe { &mut *s };
    let name_str = unsafe { CStr::from_ptr(new_name).to_string_lossy().to_string() };
    let exec_str = unsafe { CStr::from_ptr(new_exec).to_string_lossy().to_string() };
    s_ref.switch_session(&exec_str, &name_str).is_ok() as c_int
}

// -------------------- ThemeManager FFI --------------------
#[no_mangle]
pub extern "C" fn theme_manager_new() -> *mut ThemeManager {
    let dummy_config = config::HDMConfig {
        theme: "".to_string(),
        tiling: true,
        default_session: "Hyprland".to_string(),
        autologin: false,
        two_factor_enabled: false,
        two_factor_method: None,
        two_factor_secret: None,
        systemctl_usedefine: false,
    };
    Box::into_raw(Box::new(ThemeManager::new(dummy_config, Default::default())))
}

#[no_mangle]
pub extern "C" fn theme_manager_set_theme(tm: *mut ThemeManager, name: *const c_char) -> c_int {
    if tm.is_null() { return -1; }
    let tm_ref = unsafe { &mut *tm };
    let name_str = unsafe { CStr::from_ptr(name).to_string_lossy() };
    tm_ref.set_theme(&name_str).is_ok() as c_int
}

#[no_mangle]
pub extern "C" fn theme_manager_load_and_apply(tm: *mut ThemeManager) -> c_int {
    if tm.is_null() { return -1; }
    let tm_ref = unsafe { &mut *tm };
    tm_ref.load_and_apply_themes().is_ok() as c_int
}

// -------------------- Unidata FFI --------------------
#[no_mangle]
pub extern "C" fn unidata_add_target_dir(ud: *mut UnidataGenerator, platform: c_int, dir_path: *const c_char) {
    if ud.is_null() { return; }
    let ud_ref = unsafe { &mut *ud };
    let dir_str = unsafe { CStr::from_ptr(dir_path).to_string_lossy().to_string() };
    let platform_enum = match platform {
        0 => PlatformType::Gtk3,
        1 => PlatformType::Gtk4,
        2 => PlatformType::Qt5,
        3 => PlatformType::Qt6,
        4 => PlatformType::HyprSensivityObjective,
        _ => PlatformType::Gtk3,
    };
    ud_ref.add_target_dir(platform_enum, dir_str);
}

#[no_mangle]
pub extern "C" fn unidata_write_scrub(ud: *mut UnidataGenerator) -> c_int {
    if ud.is_null() { return -1; }
    let ud_ref = unsafe { &mut *ud };
    ud_ref.write_scrub().is_ok() as c_int
}
#[no_mangle]
pub extern "C" fn unidata_new(scrub_path: *const c_char) -> *mut UnidataGenerator {
    if scrub_path.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(scrub_path) };
    let path_str = c_str.to_string_lossy().to_string();
    let ud = UnidataGenerator::new(&path_str);
    Box::into_raw(Box::new(ud))
}

/// -------------------- UserManager FFI --------------------
#[no_mangle]
pub extern "C" fn user_new(
    username: *const c_char,
    pam_service: *const c_char,
    method: i32,
    secret: *const c_char,
) -> *mut User {
    // username
    let username_str = unsafe {
        if username.is_null() {
            "".to_string()
        } else {
            CStr::from_ptr(username).to_string_lossy().to_string()
        }
    };

    let pam_service_str = unsafe {
        if pam_service.is_null() {
            "".to_string()
        } else {
            CStr::from_ptr(pam_service).to_string_lossy().to_string()
        }
    };

    let secret_str = unsafe {
        if secret.is_null() {
            None
        } else {
            Some(CStr::from_ptr(secret).to_string_lossy().to_string())
        }
    };

    // twofactor method
    let method_enum = match method {
        1 => TwoFactorMethod::TOTP,
        2 => TwoFactorMethod::HOTP { counter: 0 },
        _ => TwoFactorMethod::None,
    };

    // oluştur ve raw pointer olarak döndür
    let user = User::new(&username_str, &pam_service_str, method_enum, secret_str);
    Box::into_raw(Box::new(user))
}

#[no_mangle]
pub extern "C" fn user_set_username(u: *mut User, username: *const c_char) {
    if u.is_null() || username.is_null() {
        return;
    }
    let u_ref = unsafe { &mut *u };
    let name = unsafe { CStr::from_ptr(username) }.to_string_lossy().to_string();
    u_ref.username = name;
}

#[no_mangle]
pub extern "C" fn user_get_username(u: *const User) -> *mut c_char {
    if u.is_null() {
        return ptr::null_mut();
    }
    let u_ref = unsafe { &*u };
    match CString::new(u_ref.username.clone()) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(), // invalid UTF-8 safety fallback
    }
}

#[no_mangle]
pub extern "C" fn user_authenticate(u: *mut User, password: *const c_char) -> i32 {
    if u.is_null() || password.is_null() {
        return 0;
    }
    let u_ref = unsafe { &mut *u };
    let password_str = unsafe { CStr::from_ptr(password) }.to_string_lossy().to_string();
    if u_ref.authenticate(&password_str) { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn user_verify_2fa(u: *mut User, code: *const c_char) -> i32 {
    if u.is_null() || code.is_null() {
        return 0;
    }
    let u_ref = unsafe { &mut *u };
    let code_str = unsafe { CStr::from_ptr(code) }.to_string_lossy().to_string();
    let config_path = Path::new("/etc/hyprdm/hyprdm.conf");
    if u_ref.verify_2fa(&code_str, config_path) { 1 } else { 0 }
}

// -------------------- Free / Drop functions --------------------
#[no_mangle]
pub extern "C" fn compositor_free(c: *mut Compositor) {
    if !c.is_null() {
        unsafe { let _ = Box::from_raw(c); }
    }
}

#[no_mangle]
pub extern "C" fn ipc_free(ipc: *mut HyprlandIPC) {
    if !ipc.is_null() {
        unsafe { let _ = Box::from_raw(ipc); }
    }
}

#[no_mangle]
pub extern "C" fn layout_manager_free(lm: *mut LayoutManager) {
    if !lm.is_null() {
        unsafe { let _ = Box::from_raw(lm); }
    }
}

#[no_mangle]
pub extern "C" fn session_free(s: *mut Session) {
    if !s.is_null() {
        unsafe { let _ = Box::from_raw(s); }
    }
}

#[no_mangle]
pub extern "C" fn theme_manager_free(tm: *mut ThemeManager) {
    if !tm.is_null() {
        unsafe { let _ = Box::from_raw(tm); }
    }
}

#[no_mangle]
pub extern "C" fn user_free(u: *mut User) {
    if !u.is_null() {
        unsafe { let _ = Box::from_raw(u); }
    }
}

#[no_mangle]
pub extern "C" fn unidata_free(ud: *mut UnidataGenerator) {
    if !ud.is_null() {
        unsafe { let _ = Box::from_raw(ud); }
    }
}
