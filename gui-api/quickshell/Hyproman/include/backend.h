#ifndef BACKEND_H
#define BACKEND_H

#include <QObject>
#include <QString>
#include <QStringList>

extern "C" {
    // Rust backend fonksiyonları
    void rust_load_themes();
    bool rust_set_theme(const char *name);
    void rust_apply_layout();
    QStringList rust_available_sessions();
    bool rust_switch_session(const char *newSession);
    void rust_send_ipc_command(const char *cmd);
    QString rust_active_window();
}

// C++ / QML wrapper
namespace backend {

class API : public QObject {
    Q_OBJECT
public:
    explicit API(QObject *parent = nullptr);

    Q_INVOKABLE void loadThemes();
    Q_INVOKABLE bool setTheme(const QString &name);
    Q_INVOKABLE void applyLayout();
    Q_INVOKABLE QStringList availableSessions();
    Q_INVOKABLE bool switchSession(const QString &newSession);
    Q_INVOKABLE void sendIPCCommand(const QString &cmd);
    Q_INVOKABLE QString activeWindow();
};

} // namespace backend

#endif // BACKEND_H
