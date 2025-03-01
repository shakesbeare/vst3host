#include "window.h"
#include "GLFW/glfw3.h"
#include "GLFW/glfw3native.h"
#include <print>
#include <stdexcept>

Steinberg::tresult PLUGIN_API WindowController::resizeView(Steinberg::IPlugView* view, Steinberg::ViewRect* newSize) {
    glfwSetWindowSize(ptr, newSize->getWidth(), newSize->getHeight());
    return 0;
}

WindowController::WindowController(int id, char* title)
    : id { id }, ptr { glfwCreateWindow(800, 600, title, NULL, NULL) } {
    if (!ptr) {
        throw std::runtime_error("Failed to initialize window");
    }
}

WindowController::WindowController(int id, char* title, int width, int height)
    : id { id }, ptr { glfwCreateWindow(width, height, title, NULL, NULL) } {
    if (!ptr) {
        throw std::runtime_error("Failed to initialize window");
    }
}

WindowController::WindowController(WindowController&& a) noexcept
: id { a.id }, ptr { a.ptr }
{
    a.ptr = nullptr;
}

WindowController& WindowController::operator=(WindowController&& a) noexcept {
    if (this != &a) {
        glfwDestroyWindow(ptr);
        ptr = a.ptr;
        id = a.id;
        a.ptr = nullptr;
    }

    return *this;
}

WindowController::~WindowController() {
    glfwDestroyWindow(ptr);
}

GLFWwindow* WindowController::get_window_ptr() {
    return ptr;
}

NativeWinHandle WindowController::get_native_ptr() {
#ifdef _WIN32
    RawWinHandle raw = { .hwnd=glfwGetWin32Window(ptr) };
    WinHandleTag tag = WinHandleTag::Win32;
    return {.tag=tag, .handle=raw };
#elif LINUX_WAYLAND
    RawWinHandle raw = { .wl_surface=glfwGetWaylandWindow(ptr) };
    WinHandleTag tag = WinHandleTag::Wayland;
    return { .tag=tag, .handle=raw };
#elif LINUX_X11
    RawWinHandle raw = { .wl_surface=glfwGetX11Window(ptr) };
    WinHandleTag tag = WinHandleTag::X11;
    return { .tag=tag, .handle=raw };
#elif __APPLE__
    RawWinHandle raw = { .wl_surface=glfwGetCocoaWindow(ptr) };
    WinHandleTag tag = WinHandleTag::Cocoa;
    return { .tag=tag, .handle=raw };
#else
#error "Unsupported Operating System"
#endif
}

int WindowController::get_id() {
    return id;
}

WindowManager::WindowManager() 
    : next_id { 0 } {}

void WindowManager::new_window(char* title) {
    WindowController w = WindowController { next_id, title };
    windows.push_back(std::move(w));
    next_id += 1;
}

void WindowManager::new_window(char* title, int width, int height) {
    WindowController w = WindowController { next_id, title, width, height };
    windows.push_back(std::move(w));
    next_id += 1;
}

WindowController& WindowManager::get_window(int id) {
    for (auto& window : windows) {
        if (window.get_id() == id) 
            return window;
    }
    throw std::out_of_range("Window not found");
}

void WindowManager::remove_window(int id) {
    auto it = std::ranges::find_if(windows.begin(), windows.end(), [id](WindowController& w) {
        return w.get_id() == id;
    });
    
    if (it != windows.end()) {
        windows.erase(it);
    } else {
        throw std::runtime_error("Attempted to delete window which did not exist");
    }
}

void WindowManager::update_windows() {
    std::vector<int> marked_remove;
    for (auto& window : windows) {
        GLFWwindow* w = window.get_window_ptr();
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

    for (auto& id : marked_remove) {
        WindowController& window = get_window(id);
        remove_window(id);
    }

    glfwPollEvents();
}

bool WindowManager::has_active_windows() {
    return windows.size() > 0;
}

int WindowManager::num_windows() {
    return windows.size();
}
