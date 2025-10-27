#pragma once

#include <cstddef>

extern "C" {

// Forward declaration
typedef struct Compositor Compositor;
typedef struct HyprlandIPC HyprlandIPC;
typedef struct LayoutManager LayoutManager;
typedef struct Session Session;
typedef struct ThemeManager ThemeManager;
typedef struct UnidataGenerator UnidataGenerator;
typedef struct User User;


// -------------------- Layout --------------------
enum Layout {
    TILING = 0,
    FLOATING = 1
};

// -------------------- Compositor --------------------
Compositor* compositor_new();
int compositor_run_with_ipc(Compositor* c, HyprlandIPC* ipc);
void compositor_stop(Compositor* c);
void compositor_free(Compositor* c);

// -------------------- IPC --------------------
HyprlandIPC* ipc_new();
int ipc_send_command(HyprlandIPC* ipc, const char* cmd);
char* ipc_get_status(HyprlandIPC* ipc);
void ipc_string_free(char* s);
void ipc_free(HyprlandIPC* ipc);

// -------------------- LayoutManager --------------------
LayoutManager* layout_manager_new(const char* panel_name);
void layout_manager_add_panel(LayoutManager* lm, const char* name, int layout);
void layout_manager_apply(LayoutManager* lm, unsigned int width, unsigned int height, unsigned int x, unsigned int y);
int layout_manager_get_panel_rect(LayoutManager* lm, struct PanelRect* out, unsigned int screen_width, unsigned int screen_height);
void layout_manager_free(LayoutManager* lm);

// Rust panel name getter
const char* layout_manager_get_panel_name(const LayoutManager* lm);
void string_free(char* s);

// -------------------- Session --------------------
Session* session_new(const char* name, const char* exec);
int session_start(Session* s);
int session_stop(Session* s);
int session_restart(Session* s);
int session_switch(Session* s, const char* new_name, const char* new_exec);
void session_free(Session* s);

// -------------------- ThemeManager --------------------
ThemeManager* theme_manager_new();
int theme_manager_set_theme(ThemeManager* tm, const char* name);
int theme_manager_load_and_apply(ThemeManager* tm);
void theme_manager_free(ThemeManager* tm);

// -------------------- Unidata --------------------
UnidataGenerator* unidata_generator_new(const char* scrub_path);
void unidata_add_target_dir(UnidataGenerator* ud, int platform, const char* dir_path);
int unidata_write_scrub(UnidataGenerator* ud);
void unidata_free(UnidataGenerator* ud);

// -------------------- User --------------------
User* user_new(const char* username, const char* pam_service, int method, const char* secret);
int user_authenticate(User* u, const char* password);
int user_verify_2fa(User* u, const char* code);
void user_free(User* u);

} // extern "C"
