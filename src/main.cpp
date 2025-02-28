#include <print>
#include <vector>
#include "GLFW/glfw3.h"

static void error_callback(int error, const char* description) {
    std::println("GLFW Error: {}", description);
}

class Window {
public:
    Window(int id)
    : id { id }, handle { glfwCreateWindow(800, 600, "My Cool Window", NULL, NULL) }{
        if (!handle) {
            throw std::runtime_error("Failed to initialize window");
        }
    }

    Window(const Window&) = delete;
    Window& operator=(const Window&) = delete;

	Window(Window&& a) noexcept
		: id { a.id }, handle { a.handle }
	{
        a.handle = nullptr;
    }

    Window& operator=(Window&& a) noexcept {
        if (this != &a) {
            glfwDestroyWindow(handle);
            handle = a.handle;
            id = a.id;
            a.handle = nullptr;
        }

        return *this;
    }

    ~Window() {
        glfwDestroyWindow(handle);
    }

    GLFWwindow* get_raw_window_pointer() {
        return handle;
    }

    int get_id() {
        return id;
    }
private:
    GLFWwindow* handle {};
    int id {};
};

class WindowManager {
    public:
        WindowManager() 
            : next_id { 0 }
        {
        }
        void new_window() {
            Window w = Window { next_id };
            windows.push_back(std::move(w));
            next_id += 1;
        }

        Window& get_window(int id) {
            for (auto& window : windows) {
                if (window.get_id() == id) 
                    return window;
            }
            throw std::out_of_range("Window not found");
        }

        void remove_window(int id) {
            auto it = std::find_if(windows.begin(), windows.end(), [id](Window& w) {
                return w.get_id() == id;
            });
            
            if (it != windows.end()) {
                windows.erase(it);
            } else {
                throw std::runtime_error("Attempted to delete window which did not exist");
            }
        }

        void update_windows() {
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

        bool has_active_windows() {
            return windows.size() > 0;
        }

        int num_windows() {
            return windows.size();
        }

    private:
        std::vector<Window> windows;
        int next_id {};
};

int main() {
    glfwSetErrorCallback(error_callback);
    if (!glfwInit()) {
        std::println("Failed to initialize GLFW");
    }

    WindowManager wm = WindowManager ();
    wm.new_window();
    wm.new_window();
    wm.new_window();

    while (wm.has_active_windows()) {
        wm.update_windows();
    }

    std::println("No remaining windows, exiting");

    return 0;
}
