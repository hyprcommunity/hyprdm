#pragma once

#include <string>
#include <vector>

extern "C" {
    typedef struct Compositor Compositor;
    typedef struct HyprlandIPC HyprlandIPC;
    typedef struct LayoutManager LayoutManager;
    typedef struct Session Session;
    typedef struct ThemeManager ThemeManager;
    typedef struct UnidataGenerator UnidataGenerator;
    typedef struct User User;

    struct PanelRect {
        unsigned int x;
        unsigned int y;
        unsigned int width;
        unsigned int height;
    };

    enum Layout {
        TILING = 0,
        FLOATING = 1
    };

    // Compositor
    Compositor* compositor_new();
    int compositor_run_with_ipc(Compositor* c, HyprlandIPC* ipc);
    void compositor_stop(Compositor* c);

    // IPC
    HyprlandIPC* ipc_new();
    int ipc_send_command(HyprlandIPC* ipc, const char* cmd);
    char* ipc_get_status(HyprlandIPC* ipc);
    void ipc_string_free(char* s);

    // LayoutManager
    LayoutManager* layout_manager_new(const char* panel_name);
    void layout_manager_add_panel(LayoutManager* lm, const char* name, int layout);
    void layout_manager_apply(LayoutManager* lm, unsigned int width, unsigned int height, unsigned int x, unsigned int y);
    int layout_manager_get_panel_rect(LayoutManager* lm, struct PanelRect* out);
    void layout_manager_free(LayoutManager* lm);

    // Session
    Session* session_new(const char* name, const char* exec);
    int session_start(Session* s);
    int session_stop(Session* s);
    int session_restart(Session* s);
    int session_switch(Session* s, const char* new_name, const char* new_exec);

    // ThemeManager
    ThemeManager* theme_manager_new();
    int theme_manager_set_theme(ThemeManager* tm, const char* name);
    int theme_manager_load_and_apply(ThemeManager* tm);

    // Unidata
    UnidataGenerator* unidata_generator_new(const char* scrub_path);
    void unidata_add_target_dir(UnidataGenerator* ud, int platform, const char* dir_path);
    int unidata_write_scrub(UnidataGenerator* ud);

    // UserManager
    User* user_new(const char* username, const char* pam_service, int method, const char* secret);
    int user_authenticate(User* u, const char* password);
    int user_verify_2fa(User* u, const char* code);
}
