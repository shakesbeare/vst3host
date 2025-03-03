#pragma once
#include "GLFW/glfw3.h"
#include <memory>
#include <string>
#include <vector>

#include "pluginterfaces/base/funknown.h"
#include "pluginterfaces/gui/iplugview.h"

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

namespace Host {
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
        NSWindow *ns_window;
        struct wl_surface *wl_surface;
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

    class NativeWinHandle {
    public:
        WinHandleTag m_tag {};
        RawWinHandle m_handle {};

        void *asPtr() {
#ifdef _WIN32
            return (void *)m_handle.hwnd;
#elif LINUX_WAYLAND
            return (void *)handle.wl_surface;
#elif LINUX_X11
            return (void *)handle.x_window;
#elif __APPLE__
            return (void *)handle.ns_window;
#endif
        }

        char *windowType() {
#ifdef _WIN32
            return (char *)"HWND";
#elif LINUX_WAYLAND
            return (char *)"WL_Surface";
#elif LINUX_X11
            return (char *)"XWindow";
#elif __APPLE__
            return (char *)"NSWindow";
#endif
        }
    };

    class WindowController : public Steinberg::IPlugFrame {
    public:
        WindowController(int id, char *title);
        WindowController(int id, char *title, int width, int height);
        WindowController(const WindowController &) = delete;
        WindowController &operator=(const WindowController &) = delete;

        WindowController(WindowController &&a) noexcept;
        WindowController &operator=(WindowController &&a) noexcept;
        virtual ~WindowController();

        GLFWwindow *getWindowPtr();
        NativeWinHandle getNativePtr();
        int get_id();
        virtual Steinberg::tresult PLUGIN_API resizeView(
            Steinberg::IPlugView *view, Steinberg::ViewRect *newSize) override;

        Steinberg::tresult PLUGIN_API queryInterface(const Steinberg::TUID _iid,
                                                     void **obj) override {
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
        int m_id {};
        GLFWwindow *m_ptr {};
    };

    class WindowManager {
    public:
        WindowManager();
        virtual ~WindowManager() noexcept = default;

        int newWindow(char *title);
        int newWindow(char *title, int width, int height);
        WindowController &getWindow(int id);
        void removeWindow(int id);
        void removeAllWindows();
        void updateWindows();
        bool hasActiveWindows();
        int numWindows();

    private:
        std::vector<WindowController> m_windows;
        int m_nextId {};
    };
} // namespace Host
