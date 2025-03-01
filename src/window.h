#ifndef EDITORHOST_WINDOW
#define EDITORHOST_WINDOW

#include "GLFW/glfw3.h"
#include <memory>
#include <vector>

#include "pluginterfaces/gui/iplugview.h"
#include "pluginterfaces/base/funknown.h"

#ifdef _WIN32
#define GLFW_EXPOSE_NATIVE_WIN32
#include <Windows.h>
#endif

#ifdef __APPLE__
#define GLFW_EXPOSE_NATIVE_COCOA
#include <Cocoa/Cocoa.h>
#endif 

#ifdef LINUX_WAYLAND
#define GLFW_EXPOSE_NATIVE_WAYLAND
#include <wayland-client.h>
#endif

#ifdef LINUX_X11
#define GLFW_EXPOSE_NATIVE_X11
#include <X11/Xlib.h>
#endif

#ifndef _WIN32
typedef std::nullptr_t HWND;
#endif

#ifndef __APPLE__
typedef std::nullptr_t NSWindow;
#endif

#ifndef LINUX_WAYLAND
struct wl_surface {};
#endif

#ifndef LINUX_X11
typedef std::nullptr_t Window;
#endif

union RawWinHandle { 
    HWND hwnd;
    NSWindow* ns_window;
    struct wl_surface* wl_surface;
    Window x_window;
};

// Enumerates the supported types of window handles
enum WinHandleTag {
    // Windows
    Win32,
    // MacOS
    Cocoa,
    // Linux with X11
    X11,
    // Linux with Wayland
    Wayland
};

struct NativeWinHandle {
    WinHandleTag tag {};
    RawWinHandle handle {};
};

class WindowController : public Steinberg::IPlugFrame {
public:
    WindowController(int id, char* title);
    WindowController(int id, char* title, int width, int height);
    WindowController(const WindowController&) = delete;
    WindowController& operator=(const WindowController&) = delete;

	WindowController(WindowController&& a) noexcept;
    WindowController& operator=(WindowController&& a) noexcept;
    virtual ~WindowController();

    GLFWwindow* get_window_ptr();
    NativeWinHandle get_native_ptr();
    int get_id();
    virtual Steinberg::tresult PLUGIN_API resizeView(Steinberg::IPlugView* view, Steinberg::ViewRect* newSize) override;

    Steinberg::tresult PLUGIN_API queryInterface(const Steinberg::TUID _iid, void** obj) override {
        if (Steinberg::FUnknownPrivate::iidEqual(_iid, IPlugFrame::iid) ||
                Steinberg::FUnknownPrivate::iidEqual(_iid, FUnknown::iid)) {
            *obj = this;
            addRef();
            return Steinberg::kResultTrue;
        }

        return Steinberg::kNoInterface;
    }

    Steinberg::uint32 PLUGIN_API addRef() override { return 1000; }
    Steinberg::uint32 PLUGIN_API release() override { return 1000; }
private:
    int id {};
    GLFWwindow* ptr {};
};

class WindowManager {
    public:
        WindowManager();
        virtual ~WindowManager() noexcept = default;

        void new_window(char* title);
        void new_window(char* title, int width, int height);
        WindowController& get_window(int id);
        void remove_window(int id);
        void update_windows();
        bool has_active_windows();
        int num_windows();

    private:
        std::vector<WindowController> windows;
        int next_id {};
};
#endif
