#pragma once
#include <string>
#include <vector>
#include <filesystem>

namespace hdm_api {

enum class ThemeType {
    Gtk3,
    Gtk4,
    Qt5,
    Qt6,
    HyprSensivityObjective
};

struct Theme {
    std::string name;
    std::filesystem::path path;
    ThemeType kind;
};

struct HDMConfig {
    std::string theme;
    std::string two_factor_secret;
};

class ThemeManager {
public:
    ThemeManager(const HDMConfig& config, const std::string& config_path);

    void loadAndApplyThemes();
    void applyTheme(const std::string& name);
    std::vector<std::string> availableThemesForPlatform(ThemeType platform);

private:
    void applyGtkTheme(const std::string& themeName, int version);
    void applyQtTheme(const Theme& theme);
    void applyHypersensivityTheme(const std::filesystem::path& themePath);

    std::vector<Theme> themes;
    HDMConfig config;
    std::string config_path;
};

} // namespace hdm_api
