#include "backend_wrapper.h"

GreeterBackend::GreeterBackend(QObject* parent)
    : QObject(parent)
{
    compositor    = compositor_new();
    ipc           = ipc_new();
    layoutManager = layout_manager_new("DefaultPanel");
    themeManager  = theme_manager_new();
    session       = session_new("DefaultSession", "/usr/bin/wayland-session");
    userManager   = user_new("user", "login_service", 2, nullptr); // 2 = TwoFactorMethod::None
}

GreeterBackend::~GreeterBackend()
{
    if (compositor) {
        compositor_stop(compositor);
        compositor_free(compositor);
    }

    if (ipc) {
        ipc_free(ipc);
    }

    if (layoutManager) {
        layout_manager_free(layoutManager);
    }

    if (themeManager) {
        theme_manager_free(themeManager);
    }

    if (session) {
        session_free(session);
    }

    if (userManager) {
        user_free(userManager);
    }
}

bool GreeterBackend::authenticate(const QString& password) {
    if (!userManager) return false;
    return user_authenticate(userManager, password.toUtf8().constData()) == 1;
}

bool GreeterBackend::startSession() {
    if (!session) return false;
    return session_start(session) == 0;
}
