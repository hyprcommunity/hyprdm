#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QString>
#include <QDebug>
#include <string>
#include <vector>
#include <filesystem>
#include <fstream>
#include <cstdlib>

#include "adapters.hpp" // tüm adapter sınıfları burada
#include "backend.h"    // FFI header (senin rust tarafı için)

// ----------------- Yardımcı fonksiyonlar -----------------
namespace fs = std::filesystem;

static std::string getEnv(const char* key) {
    const char* val = std::getenv(key);
    return val ? std::string(val) : std::string();
}

static std::string getPanelNameFromConfig() {
    const std::string configPath = "/etc/hyprdm/hyprdm.conf";
    std::ifstream file(configPath);
    if (!file.is_open()) {
        qWarning() << "Config file not found at"
                   << QString::fromStdString(configPath)
                   << ", using DefaultPanel.";
        return "DefaultPanel";
    }

    std::string line;
    while (std::getline(file, line)) {
        if (line.empty() || line[0] == '#') continue;
        const std::string key = "default_session=";
        if (line.rfind(key, 0) == 0) {
            std::string val = line.substr(key.size());
            while (!val.empty() && isspace(static_cast<unsigned char>(val.back())))
                val.pop_back();
            if (!val.empty()) return val;
        }
    }

    return "DefaultPanel";
}

// ----------------- Ana fonksiyon -----------------
int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    // 1️⃣ Panel adını config’ten al
    std::string panelName = getPanelNameFromConfig();
    qDebug() << "Panel from config:" << QString::fromStdString(panelName);

    // 2️⃣ FFI backend objeleri oluştur
    Compositor*    compositor     = compositor_new();
    HyprlandIPC*   ipc            = ipc_new();
    LayoutManager* layoutManager  = layout_manager_new(panelName.c_str());
    ThemeManager*  themeManager   = theme_manager_new();
    Session*       session        = session_new("DefaultSession", "/usr/share/wayland-sessions");
    User*          userManager    = user_new("", "system-login", 0 /*TwoFactorMethod::None*/, nullptr);
    if (layoutManager) {
        const char* name = layout_manager_get_panel_name(layoutManager);
        if (name) {
            panelName = name;
            string_free((char*)name);
        }
    }
    qDebug() << "Final panel:" << QString::fromStdString(panelName);

    // 4️⃣ QML dosyasını bul
    std::vector<fs::path> searchDirs = {
        fs::path(getEnv("HOME")) / ".config/hyprdm/quickshell",
        fs::path(getEnv("HOME")) / ".local/share/quickshell",
        fs::path("/usr/share/hyprdm/quickshell")
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
        qWarning() << "main.qml not found for panel:"
                   << QString::fromStdString(panelName);
        return -1;
    }

    qDebug() << "Using QML file:"
             << QString::fromStdString(qmlFilePath.string());

    auto compositorAdapter = new CompositorAdapter(compositor);
    auto ipcAdapter        = new IpcAdapter(ipc);
    auto layoutAdapter     = new LayoutAdapter(layoutManager);
    auto themeAdapter      = new ThemeAdapter(themeManager);
    auto sessionAdapter    = new SessionAdapter(session);
    auto userAdapter       = new UserAdapter(nullptr, "", 0);

    // 6️⃣ QML motoru kur
    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("CompositorBackend", compositorAdapter);
    engine.rootContext()->setContextProperty("IpcBackend", ipcAdapter);
    engine.rootContext()->setContextProperty("LayoutBackend", layoutAdapter);
    engine.rootContext()->setContextProperty("ThemeBackend", themeAdapter);
    engine.rootContext()->setContextProperty("SessionBackend", sessionAdapter);
    engine.rootContext()->setContextProperty("UserBackend", userAdapter);

    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [&qmlFilePath](QObject *obj, const QUrl &objUrl) {
                         if (!obj && objUrl == QUrl::fromLocalFile(
                                 QString::fromStdString(qmlFilePath.string())))
                             QCoreApplication::exit(-1);
                     },
                     Qt::QueuedConnection);

    engine.load(QUrl::fromLocalFile(QString::fromStdString(qmlFilePath.string())));

    if (engine.rootObjects().isEmpty())
        return -1;

    // 7️⃣ Olay döngüsü
    int ret = app.exec();

    // 8️⃣ Serbest bırak (adapter’lar unique_ptr ile kendisi temizler)
    qDebug() << "Shutting down HyprDM backend...";

    compositor_stop(compositor);
    layout_manager_free(layoutManager);
    session_free(session);
    user_free(userManager);
    theme_manager_free(themeManager);
    ipc_free(ipc)

    return ret;
}
