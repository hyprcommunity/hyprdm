#include "include/backend.h"
#include <QDebug>
#include <QStringList>

// Rust tarafında string’i free etmek için helper
extern "C" void rust_free_string(char* s);

HypromanAPI::HypromanAPI(QObject *parent) : QObject(parent) {
    qDebug() << "HypromanAPI initialized";
}

void HypromanAPI::loadThemes() {
    qDebug() << "Calling Rust backend: loadThemes";
    rust_load_themes();
}

bool HypromanAPI::setTheme(const QString &name) {
    qDebug() << "Calling Rust backend: setTheme" << name;
    return rust_set_theme(name.toStdString().c_str());
}

void HypromanAPI::applyLayout() {
    qDebug() << "Calling Rust backend: applyLayout";
    rust_apply_layout();
}

QStringList HypromanAPI::availableSessions() {
    qDebug() << "Calling Rust backend: availableSessions";
    char* raw = rust_available_sessions();
    if (!raw) return {};

    QStringList list = QString(raw).split(";", Qt::SkipEmptyParts);

    rust_free_string(raw); // Hafıza sızıntısını önle
    return list;
}

bool HypromanAPI::switchSession(const QString &newSession) {
    qDebug() << "Calling Rust backend: switchSession" << newSession;
    return rust_switch_session(newSession.toStdString().c_str());
}

void HypromanAPI::sendIPCCommand(const QString &cmd) {
    qDebug() << "Calling Rust backend: sendIPCCommand" << cmd;
    rust_send_ipc_command(cmd.toStdString().c_str());
}

QString HypromanAPI::activeWindow() {
    qDebug() << "Calling Rust backend: activeWindow";
    char* raw = rust_active_window();
    if (!raw) return "";

    QString result = QString(raw);
    rust_free_string(raw); // Hafıza sızıntısını önle
    return result;
}
