#pragma once
#include <QObject>
#include <QString>
#include "backend.h"

class GreeterBackend : public QObject {
    Q_OBJECT
public:
    explicit GreeterBackend(QObject* parent = nullptr);
    ~GreeterBackend();

    Q_INVOKABLE bool authenticate(const QString& password);
    Q_INVOKABLE bool startSession();

private:
    Compositor* compositor;
    HyprlandIPC* ipc;
    LayoutManager* layoutManager;
    ThemeManager* themeManager;
    Session* session;
    User* userManager;
};
