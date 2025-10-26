#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include "backend.h"
#include <QString>
#include <QDebug>
#include <string>
#include <vector>
#include <filesystem>
#include <fstream>
#include <sstream>

namespace fs = std::filesystem;

// --------------------------------------------------------------------
//  /etc/hyprdm/hyprdm.conf dosyasından default_session değerini okur
// --------------------------------------------------------------------
std::string getPanelNameFromConfig() {
    const std::string configPath = "/etc/hyprdm/hyprdm.conf";
    std::ifstream file(configPath);
    if (!file.is_open()) {
        qWarning() << "Config file not found at"
                   << QString::fromStdString(configPath)
                   << ", falling back to DefaultPanel";
        return "DefaultPanel";
    }

    std::string line;
    while (std::getline(file, line)) {
        if (line.empty() || line[0] == '#')
            continue;

        const std::string key = "default_session=";
        if (line.rfind(key, 0) == 0) { // satır 'default_session=' ile başlıyorsa
            std::string val = line.substr(key.size());
            if (!val.empty()) {
                while (!val.empty() && isspace(val.back()))
                    val.pop_back();
                return val;
            }
        }
    }

    return "DefaultPanel";
}

// --------------------------------------------------------------------
//                              MAIN
// --------------------------------------------------------------------
int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    // Backend nesnelerini yarat
    Compositor* compositor       = compositor_new();
    HyprlandIPC* ipc             = ipc_new();
    LayoutManager* layoutManager = layout_manager_new("DefaultPanel");
    ThemeManager* themeManager   = theme_manager_new();
    Session* session             = session_new("DefaultSession", "/usr/lib/wayland-session");
    User* userManager            = user_new("user", "login_service", 2 /*TwoFactorMethod::None*/, nullptr);

    // Panel adını config dosyasından al
    std::string panelName = getPanelNameFromConfig();
    qDebug() << "Panel name from config:" << QString::fromStdString(panelName);

    // Rust'tan panel adı override ediyorsa al
    if (layoutManager) {
        const char* name = layout_manager_get_panel_name(layoutManager);
        if (name) {
            panelName = name;
            string_free((char*)name);
        }
    }

    qDebug() << "Final panel name:" << QString::fromStdString(panelName);

    // Aranacak dizinler
    std::vector<fs::path> searchDirs = {
        fs::path(getenv("HOME")) / ".config/hyprdm/quickshell",
        fs::path(getenv("HOME")) / ".local/share/quickshell",
        fs::path(getenv("HOME")) / "hyprdm/gui-api/quickshell",
    };

    // QML dosyasını bul
    fs::path qmlFilePath;
    for (auto& dir : searchDirs) {
        fs::path candidate = dir / panelName / "main.qml";
        if (fs::exists(candidate)) {
            qmlFilePath = candidate;
            break;
        }
    }

    if (qmlFilePath.empty()) {
        qWarning() << "main.qml not found for panel:"
                   << QString::fromStdString(panelName);
        return -1;
    }

    qDebug() << "Using QML file:"
             << QString::fromStdString(qmlFilePath.string());

    // QML engine
    QQmlApplicationEngine engine;
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [&qmlFilePath](QObject *obj, const QUrl &objUrl) {
                         if (!obj && objUrl == QUrl::fromLocalFile(
                                 QString::fromStdString(qmlFilePath.string())))
                             QCoreApplication::exit(-1);
                     },
                     Qt::QueuedConnection);

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
