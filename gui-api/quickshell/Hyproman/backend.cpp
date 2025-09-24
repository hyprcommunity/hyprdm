#include "include/backend.h"
#include <string>
#include <vector>

// Tüm fonksiyonlar doğrudan Rust FFI fonksiyonlarını çağırır.
// Örnek olarak wrapper işlevleri de C++ tarafında oluşturulabilir.

extern "C" {

// Compositor
Compositor* compositor_new() { return ::compositor_new(); }
int compositor_run_with_ipc(Compositor* c, HyprlandIPC* ipc) { return ::compositor_run_with_ipc(c, ipc); }
void compositor_stop(Compositor* c) { ::compositor_stop(c); }

// IPC
HyprlandIPC* ipc_new() { return ::ipc_new(); }
int ipc_send_command(HyprlandIPC* ipc, const char* cmd) { return ::ipc_send_command(ipc, cmd); }
char* ipc_get_status(HyprlandIPC* ipc) { return ::ipc_get_status(ipc); }
void ipc_string_free(char* s) { ::ipc_string_free(s); }

// LayoutManager
LayoutManager* layout_manager_new() { return ::layout_manager_new(); }
void layout_manager_add_panel(LayoutManager* lm, const char* name, int layout) { ::layout_manager_add_panel(lm, name, layout); }
void layout_manager_apply(LayoutManager* lm) { ::layout_manager_apply(lm); }
int layout_manager_get_tiling_panels(LayoutManager* lm, PanelRect* out, size_t max) { return ::layout_manager_get_tiling_panels(lm, out, max); }

// Session
Session* session_new(const char* name, const char* exec) { return ::session_new(name, exec); }
int session_start(Session* s) { return ::session_start(s); }
int session_stop(Session* s) { return ::session_stop(s); }
int session_restart(Session* s) { return ::session_restart(s); }
int session_switch(Session* s, const char* new_name, const char* new_exec) { return ::session_switch(s, new_name, new_exec); }

// ThemeManager
ThemeManager* theme_manager_new() { return ::theme_manager_new(); }
int theme_manager_set_theme(ThemeManager* tm, const char* name) { return ::theme_manager_set_theme(tm, name); }
int theme_manager_load_and_apply(ThemeManager* tm) { return ::theme_manager_load_and_apply(tm); }

// Unidata
UnidataGenerator* unidata_generator_new(const char* scrub_path) { return ::unidata_generator_new(scrub_path); }
void unidata_add_target_dir(UnidataGenerator* ud, int platform, const char* dir_path) { ::unidata_add_target_dir(ud, platform, dir_path); }
int unidata_write_scrub(UnidataGenerator* ud) { return ::unidata_write_scrub(ud); }

// UserManager
User* user_new(const char* username, const char* pam_service, int method, const char* secret) { return ::user_new(username, pam_service, method, secret); }
int user_authenticate(User* u, const char* password) { return ::user_authenticate(u, password); }
int user_verify_2fa(User* u, const char* code) { return ::user_verify_2fa(u, code); }

} // extern "C"
