#pragma once
#include <QObject>
#include <QString>
#include <QVariant>
#include <QVariantMap>
#include <QVector>
#include <QPointer>
#include <memory>
#include <functional>

// ===================== FFI DECLARATIONS (C) =====================
extern "C" {
    // Types
    struct Compositor;
    struct HyprlandIPC;
    struct LayoutManager;
    struct Session;
    struct ThemeManager;
    struct UnidataGenerator;
    struct User;

    // Compositor
    Compositor* compositor_new();
    int         compositor_run_with_ipc(Compositor*, HyprlandIPC*); // c_int
    void        compositor_stop(Compositor*);
    void        compositor_free(Compositor*);

    // IPC
    HyprlandIPC* ipc_new();
    int          ipc_send_command(HyprlandIPC*, const char*); // c_int
    char*        ipc_get_status(HyprlandIPC*);                // must be free()'d? (FFI'de raw char*, biz CString::into_raw kull.
    void         ipc_free(HyprlandIPC*);

    // LayoutManager
    LayoutManager* layout_manager_new(const char* panel_name);
    void           layout_manager_free(LayoutManager*);
    const char*    layout_manager_get_panel_name(const LayoutManager*);
    void           layout_manager_apply(LayoutManager*, unsigned width, unsigned height, unsigned x, unsigned y);
    struct PanelRect { unsigned x, y, width, height; };
    int            layout_manager_get_panel_rect(LayoutManager*, PanelRect* out, unsigned screen_w, unsigned screen_h);

    // Session
    Session* session_new(const char* name, const char* exec_dir);
    int      session_start(Session*);
    int      session_stop(Session*);
    int      session_restart(Session*);
    int      session_switch(Session*, const char* new_name, const char* new_exec);
    void     session_free(Session*);

    // ThemeManager
    ThemeManager* theme_manager_new();
    int           theme_manager_set_theme(ThemeManager*, const char* name);
    int           theme_manager_load_and_apply(ThemeManager*);
    void          theme_manager_free(ThemeManager*);

    // Unidata
    UnidataGenerator* unidata_new();                // yoksa opsiyonel, gerekirse eklersin
    void              unidata_add_target_dir(UnidataGenerator*, int platform, const char* dir);
    int               unidata_write_scrub(UnidataGenerator*);
    void              unidata_free(UnidataGenerator*);

    // User
    User* user_new(const char* username, const char* pam_service, int method, const char* secret);
    const char* user_get_username(const User*);
    int   user_authenticate(User*, const char* password); // 0/1
    int   user_verify_2fa(User*, const char* code);       // 0/1
    void  user_free(User*);

    // misc
    void  string_free(char*); // CString::into_raw ile dönen char* için
}

// ===================== UTIL: Smart deleters =====================
template <typename T, void(*FreeFn)(T*)>
struct FfiDeleter {
    void operator()(T* p) const noexcept { if (p) FreeFn(p); }
};

using CompositorPtr   = std::unique_ptr<Compositor,   FfiDeleter<Compositor,   compositor_free>>;
using IpcPtr          = std::unique_ptr<HyprlandIPC,  FfiDeleter<HyprlandIPC,  ipc_free>>;
using LayoutMgrPtr    = std::unique_ptr<LayoutManager,FfiDeleter<LayoutManager,layout_manager_free>>;
using SessionPtr      = std::unique_ptr<Session,      FfiDeleter<Session,      session_free>>;
using ThemeMgrPtr     = std::unique_ptr<ThemeManager, FfiDeleter<ThemeManager, theme_manager_free>>;
using UnidataPtr      = std::unique_ptr<UnidataGenerator, FfiDeleter<UnidataGenerator, unidata_free>>;
using UserPtr         = std::unique_ptr<User,         FfiDeleter<User,         user_free>>;

// ===================== ADAPTERS (QObjects) =====================

class CompositorAdapter : public QObject {
    Q_OBJECT
public:
    explicit CompositorAdapter(Compositor* raw = nullptr, QObject* parent=nullptr);
    Q_INVOKABLE bool runWithIpc(QObject* ipcAdapter); // alıcı: IpcAdapter*
    Q_INVOKABLE void stop();

private:
    CompositorPtr m_comp;
};

class IpcAdapter : public QObject {
    Q_OBJECT
public:
    explicit IpcAdapter(HyprlandIPC* raw = nullptr, QObject* parent=nullptr);
    HyprlandIPC* raw() const { return m_ipc.get(); }

    Q_INVOKABLE bool sendCommand(const QString& cmd);
    Q_INVOKABLE QString status();

private:
    IpcPtr m_ipc;
};

class LayoutAdapter : public QObject {
    Q_OBJECT
    Q_PROPERTY(QString panelName READ panelName NOTIFY panelNameChanged)
public:
    explicit LayoutAdapter(LayoutManager* raw=nullptr, QObject* parent=nullptr);

    QString panelName() const;

    Q_INVOKABLE void apply(uint width, uint height, uint x, uint y);
    Q_INVOKABLE QVariantMap panelRect(uint screenW, uint screenH); // {x,y,width,height}

signals:
    void panelNameChanged();

private:
    LayoutMgrPtr m_lm;
};

class SessionAdapter : public QObject {
    Q_OBJECT
    Q_PROPERTY(QString baseExecDir READ baseExecDir WRITE setBaseExecDir NOTIFY baseExecDirChanged)
public:
    explicit SessionAdapter(Session* raw=nullptr, QObject* parent=nullptr);

    QString baseExecDir() const { return m_baseExecDir; }
    void setBaseExecDir(const QString& d) { if (m_baseExecDir==d) return; m_baseExecDir=d; emit baseExecDirChanged(); }

    Q_INVOKABLE bool restart();
    Q_INVOKABLE bool start();
    Q_INVOKABLE bool stop();
    Q_INVOKABLE bool switchAndStart(const QString& sessionName, const QString& customExecPath = QString());

signals:
    void baseExecDirChanged();

private:
    SessionPtr m_session;
    QString    m_baseExecDir; // /usr/share/wayland-sessions
};

class ThemeAdapter : public QObject {
    Q_OBJECT
public:
    explicit ThemeAdapter(ThemeManager* raw=nullptr, QObject* parent=nullptr);

    Q_INVOKABLE bool setTheme(const QString& name);
    Q_INVOKABLE bool loadAndApply();

private:
    ThemeMgrPtr m_tm;
};

class UnidataAdapter : public QObject {
    Q_OBJECT
public:
    explicit UnidataAdapter(UnidataGenerator* raw=nullptr, QObject* parent=nullptr);

    // platform: 0=Gtk3, 1=Gtk4, 2=Qt5, 3=Qt6, 4=HyprSensivityObjective
    Q_INVOKABLE void addTargetDir(int platform, const QString& dir);
    Q_INVOKABLE bool writeScrub();

private:
    UnidataPtr m_ud;
};

class UserAdapter : public QObject {
    Q_OBJECT
    Q_PROPERTY(QString username READ username CONSTANT)
    Q_PROPERTY(int     twofactor_method READ twofactorMethod CONSTANT) // 0=None,1=TOTP,2=HOTP
public:
    explicit UserAdapter(User* raw, const QString& username, int twofactorMethod, QObject* parent=nullptr);

    QString username() const { return m_username; }
    int     twofactorMethod() const { return m_twofactorMethod; }

    Q_INVOKABLE bool authenticate(const QString& password);
    Q_INVOKABLE bool verifyTwoFactor(const QString& code);

private:
    UserPtr  m_user;
    QString  m_username;
    int      m_twofactorMethod = 0;
};
