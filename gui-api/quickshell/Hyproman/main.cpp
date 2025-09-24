#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include "backend.h"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    // Rust backend nesnelerini yarat
    Compositor* compositor = compositor_new();
    HyprlandIPC* ipc = ipc_new();
    LayoutManager* lm = layout_manager_new();
    ThemeManager* tm = theme_manager_new();

    // QML engine
    QQmlApplicationEngine engine;

    // Backend nesnelerini QML tarafına aktar
    engine.rootContext()->setContextProperty("CompositorBackend", reinterpret_cast<quintptr>(compositor));
    engine.rootContext()->setContextProperty("IPCBackend", reinterpret_cast<quintptr>(ipc));
    engine.rootContext()->setContextProperty("LayoutManagerBackend", reinterpret_cast<quintptr>(lm));
    engine.rootContext()->setContextProperty("ThemeManagerBackend", reinterpret_cast<quintptr>(tm));

    const QUrl url(QStringLiteral("qrc:/main.qml"));
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl &objUrl) {
        if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);
    engine.load(url);

    int ret = app.exec();

    // Uygulama kapanmadan önce backend nesnelerini durdur
    compositor_stop(compositor);
    // IPC, LayoutManager, ThemeManager için Rust tarafında drop olacağından ekstra free gerekmez

    return ret;
}
