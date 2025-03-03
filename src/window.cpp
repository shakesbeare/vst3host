#include "window.h"
#include "GLFW/glfw3.h"
#include "GLFW/glfw3native.h"
#include <print>
#include <stdexcept>

namespace Host {
    Steinberg::tresult PLUGIN_API WindowController::resizeView(
        Steinberg::IPlugView *view, Steinberg::ViewRect *newSize) {
        glfwSetWindowSize(m_ptr, newSize->getWidth(), newSize->getHeight());
        return 0;
    }

    Host::WindowController::WindowController(int id, char *title)
        : m_id{id}, m_ptr{glfwCreateWindow(800, 600, title, NULL, NULL)} {
        if (!m_ptr) {
            throw std::runtime_error("Failed to initialize window");
        }
    }

    WindowController::WindowController(int id, char *title, int width,
                                       int height)
        : m_id{id}, m_ptr{glfwCreateWindow(width, height, title, NULL, NULL)} {
        if (!m_ptr) {
            throw std::runtime_error("Failed to initialize window");
        }
    }

    WindowController::WindowController(WindowController &&a) noexcept
        : m_id{a.m_id}, m_ptr{a.m_ptr} {
        a.m_ptr = nullptr;
    }

    WindowController &
    WindowController::operator=(WindowController &&a) noexcept {
        if (this != &a) {
            glfwDestroyWindow(m_ptr);
            m_ptr = a.m_ptr;
            m_id = a.m_id;
            a.m_ptr = nullptr;
        }

        return *this;
    }

    WindowController::~WindowController() { glfwDestroyWindow(m_ptr); }

    GLFWwindow *WindowController::getWindowPtr() { return m_ptr; }

    NativeWinHandle WindowController::getNativePtr() {
#ifdef _WIN32
        RawWinHandle raw = {.hwnd = glfwGetWin32Window(m_ptr)};
        WinHandleTag tag = WinHandleTag::Win32;
        return {.m_tag = tag, .m_handle = raw};
#elif LINUX_WAYLAND
        RawWinHandle raw = {.wl_surface = glfwGetWaylandWindow(ptr)};
        WinHandleTag tag = WinHandleTag::Wayland;
        return {.tag = tag, .handle = raw};
#elif LINUX_X11
        RawWinHandle raw = {.wl_surface = glfwGetX11Window(ptr)};
        WinHandleTag tag = WinHandleTag::X11;
        return {.tag = tag, .handle = raw};
#elif __APPLE__
        RawWinHandle raw = {.wl_surface = glfwGetCocoaWindow(ptr)};
        WinHandleTag tag = WinHandleTag::Cocoa;
        return {.tag = tag, .handle = raw};
#else
#error "Unsupported Operating System"
#endif
    }

    int WindowController::get_id() { return m_id; }

    WindowManager::WindowManager() : m_nextId{0} {}

    int WindowManager::newWindow(char *title) {
        int id = m_nextId;
        m_nextId += 1;
        WindowController w = WindowController{id, title};
        m_windows.push_back(std::move(w));
        return id;
    }

    int WindowManager::newWindow(char *title, int width, int height) {
        int id = m_nextId;
        m_nextId += 1;
        WindowController w = WindowController{id, title, width, height};
        m_windows.push_back(std::move(w));
        return id;
    }

    WindowController &WindowManager::getWindow(int id) {
        for (auto &window : m_windows) {
            if (window.get_id() == id)
                return window;
        }
        throw std::out_of_range("Window not found");
    }

    void WindowManager::removeWindow(int id) {
        auto it = std::ranges::find_if(
            m_windows.begin(), m_windows.end(),
            [id](WindowController &w) { return w.get_id() == id; });

        if (it != m_windows.end()) {
            m_windows.erase(it);
        } else {
            throw std::runtime_error(
                "Attempted to delete window which did not exist");
        }
    }

    void WindowManager::removeAllWindows() {
        m_windows.clear();
    }

    void WindowManager::updateWindows() {
        std::vector<int> marked_remove;
        for (auto &window : m_windows) {
            GLFWwindow *w = window.getWindowPtr();
            if (!w)
                throw std::runtime_error("Tried to update a null window");
            glfwMakeContextCurrent(w);
            if (glfwWindowShouldClose(w)) {
                marked_remove.push_back(window.get_id());
                continue;
            }
            glClear(GL_COLOR_BUFFER_BIT);
            glfwSwapBuffers(w);
        }

        for (auto &id : marked_remove) {
            WindowController &window = getWindow(id);
            removeWindow(id);
        }

        glfwPollEvents();
    }

    bool WindowManager::hasActiveWindows() { return m_windows.size() > 0; }

    int WindowManager::numWindows() { return m_windows.size(); }
} // namespace Host
