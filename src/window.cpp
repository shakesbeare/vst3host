#include "window.h"
#include "GLFW/glfw3.h"
#include <stdexcept>

Window::Window(int id, char* title)
    : id { id }, handle { glfwCreateWindow(800, 600, title, NULL, NULL) }{
        if (!handle) {
            throw std::runtime_error("Failed to initialize window");
        }
    }

Window::Window(Window&& a) noexcept
: id { a.id }, handle { a.handle }
{
    a.handle = nullptr;
}

Window& Window::operator=(Window&& a) noexcept {
    if (this != &a) {
        glfwDestroyWindow(handle);
        handle = a.handle;
        id = a.id;
        a.handle = nullptr;
    }

    return *this;
}

Window::~Window() {
    glfwDestroyWindow(handle);
}

GLFWwindow* Window::get_raw_window_pointer() {
    return handle;
}

int Window::get_id() {
    return id;
}

WindowManager::WindowManager() 
    : next_id { 0 }
{}
void WindowManager::new_window(char* title) {
    Window w = Window { next_id, title };
    windows.push_back(std::move(w));
    next_id += 1;
}

Window& WindowManager::get_window(int id) {
    for (auto& window : windows) {
        if (window.get_id() == id) 
            return window;
    }
    throw std::out_of_range("Window not found");
}

void WindowManager::remove_window(int id) {
    auto it = std::ranges::find_if(windows.begin(), windows.end(), [id](Window& w) {
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
        GLFWwindow* w = window.get_raw_window_pointer();
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
        Window& window = get_window(id);
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
