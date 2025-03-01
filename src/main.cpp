#include <print>
#include "GLFW/glfw3.h"
#include "window.h"
#include "public.sdk/source/vst/hosting/module.h"

static void error_callback(int error, const char* description) {
    std::println("GLFW Error: {}", description);
}

int main() {
    glfwSetErrorCallback(error_callback);
    if (!glfwInit()) {
    }

    std::string error;
    auto mod = VST3::Hosting::Module::create("C:/Program Files/Common Files/VST3/OTT.vst3", error);
    if (!mod) {
        std::println("Failed to load module");
        std::println("{}", error);
        return -1;
    }

    WindowManager wm = WindowManager();
    wm.new_window((char*)"Window 1");

    while (wm.has_active_windows()) {
        wm.update_windows();
    }

    return 0;
}
