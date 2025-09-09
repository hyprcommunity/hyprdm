#pragma once
#include <string>
#include <memory>
#include <atomic>
#include <thread>
#include "ipc.h"

namespace hdm_api {

class Compositor {
public:
    Compositor();

    bool isRunning() const;
    void runWithIPC(std::shared_ptr<HyprlandIPC> ipc = nullptr);
    void stop();

    // Config ve layout manager
    std::string config_path;
    struct LayoutManager layout_manager;
};

} // namespace hdm_api
