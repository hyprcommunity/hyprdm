#include "adapters.hpp"
#include <QDebug>

// =============== CompositorAdapter ===============
CompositorAdapter::CompositorAdapter(Compositor* raw, QObject* parent)
    : QObject(parent), m_comp(raw ? CompositorPtr(raw) : CompositorPtr(compositor_new())) {}

bool CompositorAdapter::runWithIpc(QObject* ipcAdapterObj) {
    if (!m_comp) return false;
    auto ipcAdapter = qobject_cast<IpcAdapter*>(ipcAdapterObj);
    HyprlandIPC* rawIpc = ipcAdapter ? ipcAdapter->raw() : nullptr;
    int ok = compositor_run_with_ipc(m_comp.get(), rawIpc);
    return ok != 0;
}

void CompositorAdapter::stop() {
    if (m_comp) compositor_stop(m_comp.get());
}

// =============== IpcAdapter ===============
IpcAdapter::IpcAdapter(HyprlandIPC* raw, QObject* parent)
    : QObject(parent), m_ipc(raw ? IpcPtr(raw) : IpcPtr(ipc_new())) {}

bool IpcAdapter::sendCommand(const QString& cmd) {
    if (!m_ipc) return false;
    return ipc_send_command(m_ipc.get(), cmd.toUtf8().constData()) != 0;
}

QString IpcAdapter::status() {
    if (!m_ipc) return {};
    char* s = ipc_get_status(m_ipc.get());
    if (!s) return {};
    QString out = QString::fromUtf8(s);
    string_free(s);
    return out;
}

// =============== LayoutAdapter ===============
LayoutAdapter::LayoutAdapter(LayoutManager* raw, QObject* parent)
    : QObject(parent), m_lm(raw ? LayoutMgrPtr(raw) : LayoutMgrPtr(layout_manager_new("DefaultPanel"))) {}

QString LayoutAdapter::panelName() const {
    if (!m_lm) return {};
    const char* n = layout_manager_get_panel_name(m_lm.get());
    if (!n) return {};
    QString out = QString::fromUtf8(n);
    // n, FFI’de into_raw ile döndü; serbest bırak:
    string_free(const_cast<char*>(n));
    return out;
}

void LayoutAdapter::apply(uint w, uint h, uint x, uint y) {
    if (!m_lm) return;
    layout_manager_apply(m_lm.get(), w, h, x, y);
}

QVariantMap LayoutAdapter::panelRect(uint screenW, uint screenH) {
    QVariantMap m;
    if (!m_lm) return m;
    PanelRect r{};
    int ok = layout_manager_get_panel_rect(m_lm.get(), &r, screenW, screenH);
    if (ok != 0) return m;
    m["x"] = r.x; m["y"] = r.y; m["width"] = r.width; m["height"] = r.height;
    return m;
}

// =============== SessionAdapter ===============
SessionAdapter::SessionAdapter(Session* raw, QObject* parent)
    : QObject(parent), m_session(raw ? SessionPtr(raw) : SessionPtr(session_new("DefaultSession", "/usr/share/wayland-sessions"))),
      m_baseExecDir("/usr/share/wayland-sessions") {}

bool SessionAdapter::restart() {
    if (!m_session) return false;
    return session_restart(m_session.get()) != 0;
}

bool SessionAdapter::start() {
    if (!m_session) return false;
    return session_start(m_session.get()) != 0;
}

bool SessionAdapter::stop() {
    if (!m_session) return false;
    return session_stop(m_session.get()) != 0;
}

bool SessionAdapter::switchAndStart(const QString& sessionName, const QString& customExecPath) {
    if (!m_session) return false;
    QString execPath = customExecPath;
    if (execPath.isEmpty()) {
        execPath = m_baseExecDir.endsWith("/") ? (m_baseExecDir + sessionName)
                                               : (m_baseExecDir + "/" + sessionName);
    }
    int okSwitch = session_switch(m_session.get(),
                                  sessionName.toUtf8().constData(),
                                  execPath.toUtf8().constData());
    if (okSwitch == 0) return false;
    return session_start(m_session.get()) != 0;
}

// =============== ThemeAdapter ===============
ThemeAdapter::ThemeAdapter(ThemeManager* raw, QObject* parent)
    : QObject(parent), m_tm(raw ? ThemeMgrPtr(raw) : ThemeMgrPtr(theme_manager_new())) {}

bool ThemeAdapter::setTheme(const QString& name) {
    if (!m_tm) return false;
    return theme_manager_set_theme(m_tm.get(), name.toUtf8().constData()) != 0;
}
bool ThemeAdapter::loadAndApply() {
    if (!m_tm) return false;
    return theme_manager_load_and_apply(m_tm.get()) != 0;
}

// =============== UnidataAdapter ===============
UnidataAdapter::UnidataAdapter(UnidataGenerator* raw, QObject* parent)
    : QObject(parent), m_ud(raw ? UnidataPtr(raw) : UnidataPtr(unidata_new())) {}

void UnidataAdapter::addTargetDir(int platform, const QString& dir) {
    if (!m_ud) return;
    unidata_add_target_dir(m_ud.get(), platform, dir.toUtf8().constData());
}
bool UnidataAdapter::writeScrub() {
    if (!m_ud) return false;
    return unidata_write_scrub(m_ud.get()) != 0;
}

// =============== UserAdapter ===============
UserAdapter::UserAdapter(User* raw, const QString& username, int twofactorMethod, QObject* parent)
    : QObject(parent),
      m_user(raw ? UserPtr(raw) : nullptr),
      m_username(username),
      m_twofactorMethod(twofactorMethod)
{
    // Eğer raw gelmediyse, yeni User oluştur
    if (!m_user) {
        if (!m_username.isEmpty()) {
            m_user.reset(user_new(m_username.toUtf8().constData(), "system-login", twofactorMethod, nullptr));
        } else {
            // fallback olarak sistem kullanıcısını al
            m_user.reset(user_new("", "system-login", twofactorMethod, nullptr));
        }
    }

    // Rust tarafındaki username’i QML tarafına senkronla
    if (m_user) {
        const char* uname = user_get_username(m_user.get());
        if (uname && *uname) {
            m_username = QString::fromUtf8(uname);
            string_free((char*)uname);
        }
    }
}

bool UserAdapter::authenticate(const QString& password) {
    if (!m_user) return false;
    return user_authenticate(m_user.get(), password.toUtf8().constData()) != 0;
}

bool UserAdapter::verifyTwoFactor(const QString& code) { 
    if (!m_user) return m_twofactorMethod == 0;
    if (m_twofactorMethod == 0) return true;
    return user_verify_2fa(m_user.get(), code.toUtf8().constData()) != 0;
}
