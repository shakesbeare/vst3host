#ifndef EDITORHOST_WINDOW
#define EDITORHOST_WINDOW

#include "GLFW/glfw3.h"
#include <vector>

class Window {
public:
    Window(int id, char* title);
    Window(const Window&) = delete;
    Window& operator=(const Window&) = delete;

	Window(Window&& a) noexcept;
    Window& operator=(Window&& a) noexcept;
    ~Window();

    GLFWwindow* get_raw_window_pointer();
    int get_id();
private:
    GLFWwindow* handle {};
    int id {};
};

class WindowManager {
    public:
        WindowManager();

        void new_window(char* title);
        Window& get_window(int id);
        void remove_window(int id);
        void update_windows();
        bool has_active_windows();
        int num_windows();

    private:
        std::vector<Window> windows;
        int next_id {};
};
#endif
