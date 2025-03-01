#include <print>
#include "GLFW/glfw3.h"
#include "window.h"

static void error_callback(int error, const char* description) {
    std::println("GLFW Error: {}", description);
}

int main() {
    glfwSetErrorCallback(error_callback);
    if (!glfwInit()) {
        std::println("Failed to initialize GLFW");
    }

    WindowManager wm = WindowManager();
    wm.new_window((char*)"Window 1");
    wm.new_window((char*)"Window 2");
    wm.new_window((char*)"Window 3");

    while (wm.has_active_windows()) {
        wm.update_windows();
    }

    std::println("No remaining windows, exiting");
    return 0;
}
