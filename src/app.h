#pragma once

#include <filesystem>
#include <optional>

#include "window.h"
#include "plugin.h"
#include "component_handler.h"


namespace Host {
    static ComponentHandler gComponentHandler;

    class App {
    public:
        App();
        ~App();

        WindowController& requestWindow(const std::string& title, int width, int height);
        void run();
        void exit();

    private:
        std::vector<Plugin::Plugin> m_plugins;
        WindowManager m_windowManager {};
        Plugin::Plugin& registerPlugin(std::filesystem::path path);
        void createViewAndShow(const std::string& plugName);
        int64_t findPlugByName(const std::string& plugName);
    };
} // namespace Host
