#pragma once
#include <string>

namespace hdm_api {

class HyprlandIPC {
public:
    static void sendCommand(const std::string& cmd);
    static std::string getStatus();
};

} // namespace hdm_api
