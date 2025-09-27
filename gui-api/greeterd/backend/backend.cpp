#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include "backend.h"
#include <QString>
#include <QDebug>
#include <string>
#include <vector>
#include <filesystem>

namespace fs = std::filesystem;

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    // Backend nesnelerini yarat
    Compositor* compositor       = compositor_new();
    HyprlandIPC* ipc             = ipc_new();
    LayoutManager* layoutManager = layout_manager_new("DefaultPanel");
    ThemeManager* themeManager   = theme_manager_new();
    Session* session             = session_new("DefaultSession", "/usr/bin/wayland-session");
    User* userManager            = user_new("user", "login_service", 2 /*TwoFactorMethod::None*/, nullptr);

    // Rust'tan panel name çek
    std::string panelName = "DefaultPanel";
    if (layoutManager) {
        const char* name = layout_manager_get_panel_name(layoutManager);
        if (name) {
            panelName = name;
            string_free((char*)name); // Bellek sızıntısı olmaması için free
        }
    }
    qDebug() << "Panel name from Rust:" << QString::fromStdString(panelName);

    // Aranacak dizinler
    std::vector<fs::path> searchDirs = {
        fs::path(getenv("HOME")) / ".config/hyprdm/quickshell",
        fs::path(getenv("HOME")) / ".local/share/quickshell",
        "/usr/share/hyprdm/quickshell"
    };

    fs::path qmlFilePath;
    for (auto& dir : searchDirs) {
        fs::path candidate = dir / panelName / "main.qml";
        if (fs::exists(candidate)) {
            qmlFilePath = candidate;
            break;
        }
    }

    if (qmlFilePath.empty()) {
        qWarning() << "main.qml not found for panel:" << QString::fromStdString(panelName);
        return -1;
    }

    qDebug() << "Using QML file:" << QString::fromStdString(qmlFilePath.string());

    // QML engine
    QQmlApplicationEngine engine;

    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [&qmlFilePath](QObject *obj, const QUrl &objUrl) {
                         if (!obj && objUrl == QUrl::fromLocalFile(QString::fromStdString(qmlFilePath.string())))
                             QCoreApplication::exit(-1);
                     }, Qt::QueuedConnection);

    engine.load(QUrl::fromLocalFile(QString::fromStdString(qmlFilePath.string())));

    int ret = app.exec();

    // Uygulama kapanmadan önce backend nesnelerini serbest bırak
    if (compositor)       compositor_stop(compositor);
    if (layoutManager)    layout_manager_free(layoutManager);
    if (session)          session_free(session);
    if (userManager)      user_free(userManager);
    if (themeManager)     theme_manager_free(themeManager);
    if (ipc)              ipc_free(ipc);

    return ret;
}
