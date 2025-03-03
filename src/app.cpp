#include "GLFW/glfw3.h"
#include <print>

#include "app.h"
#include "window.h"

namespace Host {
    static void error_callback(int error, const char *description) {
        std::println("GLFW Error: {}", description);
    }

    App::App() : m_windowManager{WindowManager()} {
        glfwSetErrorCallback(error_callback);
        if (!glfwInit()) {
        }
        glfwWindowHint(GLFW_RESIZABLE, GL_FALSE);
    }

    App::~App() {}

    void App::run() {

        std::filesystem::path ott =
            "C:/Program Files/Common Files/VST3/OTT.vst3";
        auto plug = registerPlugin(ott);
        createViewAndShow(plug.getName());

        while (m_windowManager.hasActiveWindows()) {
            m_windowManager.updateWindows();
        }
    }

    void App::exit() { m_windowManager.removeAllWindows(); }

    WindowController &App::requestWindow(const std::string &title, int width,
                                         int height) {
        int id = m_windowManager.newWindow(title, width, height);
        return m_windowManager.getWindow(id);
    }

    // TODO: find an open space instead of slamming it onto the end
    Plugin::Plugin &App::registerPlugin(std::filesystem::path path) {
        auto plugin = Plugin::Plugin(path);
        m_plugins.push_back(plugin);
        return m_plugins.back();
    }

    int64_t App::findPlugByName(const std::string &plugName) {
        auto predicate = [plugName](Plugin::Plugin plug) {
            return plug.getName() == plugName;
        };
        auto it = std::find_if(m_plugins.begin(), m_plugins.end(), predicate);
        if (it != m_plugins.end()) {
            return std::distance(m_plugins.begin(), it);
        } else {
            return -1;
        }
    }

    void App::createViewAndShow(const std::string &plugName) {
        int64_t index = findPlugByName(plugName);
        if (index == -1) {
            std::println("Failed to find plugin {}", plugName);
            throw std::runtime_error("Plugin does not exist");
        }

        Plugin::Plugin plug = m_plugins[index];
        auto plugProvider = plug.getPlugProvider();
        auto editController = plugProvider->getController();
        if (!editController) {
            throw std::runtime_error("No EditController found");
        }
        editController->release(); // plug_provider does an addRef, this is
                                   // important, I guess
        editController->setComponentHandler(&Host::gComponentHandler);
        auto view =
            editController->createView(Steinberg::Vst::ViewType::kEditor);

        Steinberg::ViewRect plugViewSize;
        auto result = view->getSize(&plugViewSize);
        if (result != Steinberg::kResultTrue) {
            throw std::runtime_error("Could not get editor view size");
        }

        WindowController &win =
            requestWindow(plug.getName().c_str(), plugViewSize.getWidth(),
                          plugViewSize.getHeight());
        auto handle = win.getNativePtr();
        view->setFrame(&win);
        if (view->attached(handle.asPtr(), handle.windowType()) !=
            Steinberg::kResultTrue) {
            throw std::runtime_error("Attaching PlugView failed");
        }
    }
} // namespace Host
