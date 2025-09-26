#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include "include/backend.h"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    // Backend nesnelerini yarat
    Compositor* compositor = compositor_new();
    HyprlandIPC* ipc = ipc_new();
    LayoutManager* layoutManager = layout_manager_new("DefaultPanel");
    ThemeManager* themeManager = theme_manager_new();
    Session* session = session_new("DefaultSession", "/usr/bin/wayland-session");
    User* userManager = user_new("user", "login_service", 2 /*TwoFactorMethod::None*/, nullptr);

    // QML engine
    QQmlApplicationEngine engine;

    // Backend nesnelerini QML tarafına aktar
    engine.rootContext()->setContextProperty("CompositorBackend", reinterpret_cast<quintptr>(compositor));
    engine.rootContext()->setContextProperty("IPCBackend", reinterpret_cast<quintptr>(ipc));
    engine.rootContext()->setContextProperty("LayoutManagerBackend", reinterpret_cast<quintptr>(layoutManager));
    engine.rootContext()->setContextProperty("ThemeManagerBackend", reinterpret_cast<quintptr>(themeManager));
    engine.rootContext()->setContextProperty("SessionBackend", reinterpret_cast<quintptr>(session));
    engine.rootContext()->setContextProperty("UserBackend", reinterpret_cast<quintptr>(userManager));

    // QML yükle
    const QUrl url(QStringLiteral("qrc:/main.qml"));
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl &objUrl) {
                         if (!obj && url == objUrl)
                             QCoreApplication::exit(-1);
                     }, Qt::QueuedConnection);
    engine.load(url);

    int ret = app.exec();

    // Uygulama kapanmadan önce backend nesnelerini serbest bırak
    if (compositor) compositor_stop(compositor);
    if (layoutManager) layout_manager_free(layoutManager);
    // Session, User, ThemeManager için backend tarafında free fonksiyonları olmalı

    return ret;
}
